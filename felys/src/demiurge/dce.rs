use crate::ast::{BinOp, UnaOp};
use crate::cyrene::{Const, Fragment, Instruction, Label, Terminator, Var};
use crate::demiurge::{Demiurge, Function};
use crate::error::Fault;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, Div, Mul, Rem, Sub};

impl Demiurge {
    pub fn dec(&mut self) -> Result<(), Fault> {
        for function in self.fns.values_mut() {
            function.dce()?;
        }
        self.main.dce()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Lattice {
    Top,
    Const(Const),
    Bottom,
}

enum Id {
    Ins(usize),
    Term,
}

impl Lattice {
    fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            (Lattice::Top, x) | (x, Lattice::Top) => x.clone(),
            (Lattice::Const(l), Lattice::Const(r)) => {
                if l == r {
                    Lattice::Const(l.clone())
                } else {
                    Lattice::Bottom
                }
            }
            (Lattice::Bottom, _) | (_, Lattice::Bottom) => Lattice::Bottom,
        }
    }
}

struct Renamer {
    map: HashMap<Var, Var>,
}

impl Renamer {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, from: Var, to: Var) {
        self.map.insert(from, to);
    }

    fn get(&self, var: Var) -> Var {
        let mut current = var;
        let mut visited = HashSet::new();
        while let Some(&next) = self.map.get(&current) {
            if !visited.insert(next) {
                break;
            }
            current = next;
        }
        current
    }
}

struct Context {
    values: Vec<Lattice>,
    edges: HashSet<(Label, Label)>,
    visited: HashSet<Label>,

    flow: VecDeque<(Label, Label)>,
    ssa: VecDeque<Var>,
}

impl Context {
    fn new(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            edges: HashSet::new(),
            visited: HashSet::new(),
            flow: VecDeque::new(),
            ssa: VecDeque::new(),
        }
    }

    fn get(&self, var: Var) -> &Lattice {
        self.values.get(var).unwrap_or(&Lattice::Top)
    }

    fn update(&mut self, var: Var, new: Lattice) {
        if var >= self.values.len() {
            self.values.resize(var + 1, Lattice::Top);
        }

        let old = &mut self.values[var];
        if *old != new {
            *old = new;
            self.ssa.push_back(var);
        }
    }
}

impl Function {
    fn dce(&mut self) -> Result<(), Fault> {
        let ctx = self.analyze()?;
        self.fragments
            .retain(|id, _| ctx.visited.contains(&Label::Id(*id)));

        let mut renamer = Renamer::new();
        let mut changed = true;
        while changed {
            changed = false;
            for (id, fragment) in self.fragments.iter_mut() {
                if fragment.rename(Label::Id(*id), &ctx, &mut renamer) {
                    changed = true;
                }
            }
        }

        self.entry.rewrite(&ctx, &renamer)?;
        for (_, fragment) in self.fragments.iter_mut() {
            fragment.rewrite(&ctx, &renamer)?;
        }
        self.exit.rewrite(&ctx, &renamer)
    }

    fn analyze(&self) -> Result<Context, Fault> {
        let usage = self.usage();

        let mut ctx = Context::new(self.vars);
        ctx.flow.push_back((Label::Entry, Label::Entry));
        for var in self.args.iter() {
            ctx.update(*var, Lattice::Bottom);
        }

        while !ctx.flow.is_empty() || !ctx.ssa.is_empty() {
            while let Some((pred, label)) = ctx.flow.pop_front() {
                if ctx.edges.contains(&(pred, label)) {
                    continue;
                }
                ctx.edges.insert((pred, label));
                let fragment = self.get(label).unwrap();
                fragment.analyze(label, &mut ctx)?;
            }
            while let Some(var) = ctx.ssa.pop_front() {
                let Some(users) = usage.get(&var) else {
                    continue;
                };
                for (label, id) in users {
                    if !ctx.visited.contains(label) {
                        continue;
                    }
                    let fragment = self.get(*label).unwrap();
                    match id {
                        Id::Ins(index) => fragment
                            .instructions
                            .get(*index)
                            .unwrap()
                            .analyze(&mut ctx)?,
                        Id::Term => fragment
                            .terminator
                            .as_ref()
                            .unwrap()
                            .analyze(*label, &mut ctx)?,
                    }
                }
            }
        }
        Ok(ctx)
    }

    fn usage(&self) -> HashMap<Var, Vec<(Label, Id)>> {
        let mut map = HashMap::new();
        self.entry.usage(Label::Entry, &mut map);
        for (id, fragment) in &self.fragments {
            fragment.usage(Label::Id(*id), &mut map);
        }
        self.exit.usage(Label::Entry, &mut map);
        map
    }
}

impl Fragment {
    fn rename(&mut self, label: Label, ctx: &Context, renamer: &mut Renamer) -> bool {
        let mut changed = false;
        for (_, inputs) in self.phis.iter_mut() {
            let len = inputs.len();
            inputs.retain(|(pred, _)| ctx.edges.contains(&(*pred, label)));
            if len != inputs.len() {
                changed = true;
            }
        }

        self.phis.retain(|(var, input)| {
            let mut trivial = true;
            let mut candidate = None;
            for (_, src) in input {
                let resolved = renamer.get(*src);
                if resolved == *var {
                    continue;
                }
                if let Some(c) = candidate {
                    if c != resolved {
                        trivial = false;
                        break;
                    }
                } else {
                    candidate = Some(resolved);
                }
            }

            if trivial && let Some(replacement) = candidate {
                renamer.insert(*var, replacement);
                changed = true;
                return false;
            }
            true
        });
        changed
    }

    fn rewrite(&mut self, ctx: &Context, renamer: &Renamer) -> Result<(), Fault> {
        for instruction in self.instructions.iter_mut() {
            instruction.rewrite(ctx, renamer)?;
        }
        self.terminator.as_mut().unwrap().rewrite(ctx, renamer)
    }

    fn analyze(&self, label: Label, ctx: &mut Context) -> Result<(), Fault> {
        for (var, inputs) in self.phis.iter() {
            let mut new = Lattice::Top;
            for (pred, var) in inputs {
                if ctx.edges.contains(&(*pred, label)) {
                    let input = ctx.get(*var);
                    new = new.meet(input);
                }
            }
            ctx.update(*var, new);
        }
        if ctx.visited.insert(label) {
            for instruction in self.instructions.iter() {
                instruction.analyze(ctx)?;
            }
            self.terminator.as_ref().unwrap().analyze(label, ctx)?;
        }
        Ok(())
    }

    fn usage(&self, label: Label, map: &mut HashMap<Var, Vec<(Label, Id)>>) {
        for (i, instruction) in self.instructions.iter().enumerate() {
            instruction.usage(i, label, map);
        }
        self.terminator.as_ref().unwrap().usage(label, map);
    }
}

impl Instruction {
    fn rewrite(&mut self, ctx: &Context, renamer: &Renamer) -> Result<(), Fault> {
        match self {
            Instruction::Binary(dst, lhs, _, rhs) => {
                if let Lattice::Const(c) = ctx.get(*dst) {
                    *self = Instruction::Load(*dst, c.clone());
                    return Ok(());
                }
                *lhs = renamer.get(*lhs);
                *rhs = renamer.get(*rhs);
            }
            Instruction::Unary(dst, _, inner) => {
                if let Lattice::Const(c) = ctx.get(*dst) {
                    *self = Instruction::Load(*dst, c.clone());
                    return Ok(());
                }
                *inner = renamer.get(*inner);
            }
            Instruction::Field(_, var, _)
            | Instruction::Push(var)
            | Instruction::Call(_, var)
            | Instruction::Method(_, var, _) => {
                *var = renamer.get(*var);
            }
            Instruction::Index(_, var, index) => {
                *var = renamer.get(*var);
                *index = renamer.get(*index);
            }
            _ => {}
        }
        Ok(())
    }

    fn analyze(&self, ctx: &mut Context) -> Result<(), Fault> {
        match self {
            Instruction::Load(var, c) => ctx.update(*var, Lattice::Const(c.clone())),
            Instruction::Binary(var, lhs, op, rhs) => {
                let new = match (ctx.get(*lhs), ctx.get(*rhs)) {
                    (Lattice::Const(l), Lattice::Const(r)) => Lattice::Const(l.binary(op, r)?),
                    (Lattice::Bottom, _) | (_, Lattice::Bottom) => Lattice::Bottom,
                    _ => Lattice::Top,
                };
                ctx.update(*var, new);
            }
            Instruction::Unary(var, op, inner) => {
                let new = match ctx.get(*inner) {
                    Lattice::Top => Lattice::Top,
                    Lattice::Const(c) => Lattice::Const(c.unary(op)?),
                    Lattice::Bottom => Lattice::Bottom,
                };
                ctx.update(*var, new);
            }
            Instruction::Field(dst, _, _)
            | Instruction::Func(dst, _)
            | Instruction::Call(dst, _)
            | Instruction::List(dst)
            | Instruction::Tuple(dst)
            | Instruction::Index(dst, _, _)
            | Instruction::Method(dst, _, _)
            | Instruction::Group(dst, _) => ctx.update(*dst, Lattice::Bottom),
            _ => {}
        }
        Ok(())
    }

    fn usage(&self, i: usize, label: Label, map: &mut HashMap<Var, Vec<(Label, Id)>>) {
        let mut update = |var: Var| {
            map.entry(var).or_default().push((label, Id::Ins(i)));
        };
        match self {
            Instruction::Field(_, x, _) => update(*x),
            Instruction::Binary(_, l, _, r) => {
                update(*l);
                update(*r);
            }
            Instruction::Unary(_, _, x) => update(*x),
            Instruction::Push(x) => update(*x),
            Instruction::Call(_, x) => update(*x),
            Instruction::Index(_, src, x) => {
                update(*src);
                update(*x);
            }
            Instruction::Method(_, x, _) => update(*x),
            _ => {}
        }
    }
}

impl Terminator {
    fn rewrite(&mut self, ctx: &Context, renamer: &Renamer) -> Result<(), Fault> {
        match self {
            Terminator::Branch(cond, yes, no) => {
                if let Lattice::Const(c) = ctx.get(*cond) {
                    let label = if c.bool()? { yes } else { no };
                    *self = Terminator::Jump(*label);
                    return Ok(());
                }
                *cond = renamer.get(*cond)
            }
            Terminator::Return(var) => *var = renamer.get(*var),
            _ => {}
        }
        Ok(())
    }

    fn analyze(&self, label: Label, ctx: &mut Context) -> Result<(), Fault> {
        match self {
            Terminator::Branch(cond, yes, no) => {
                let val = ctx.get(*cond);
                match val {
                    Lattice::Const(Const::Bool(true)) => ctx.flow.push_back((label, *yes)),
                    Lattice::Const(Const::Bool(false)) => ctx.flow.push_back((label, *no)),
                    Lattice::Const(_) => return Err(Fault::InvalidOperation),
                    Lattice::Bottom => {
                        ctx.flow.push_back((label, *yes));
                        ctx.flow.push_back((label, *no));
                    }
                    Lattice::Top => {}
                }
            }
            Terminator::Jump(next) => ctx.flow.push_back((label, *next)),
            _ => {}
        }
        Ok(())
    }

    fn usage(&self, label: Label, map: &mut HashMap<Var, Vec<(Label, Id)>>) {
        let mut update = |var: Var| {
            map.entry(var).or_default().push((label, Id::Term));
        };
        match self {
            Terminator::Branch(x, _, _) => update(*x),
            Terminator::Return(x) => update(*x),
            _ => {}
        }
    }
}

impl Const {
    fn binary(&self, op: &BinOp, rhs: &Const) -> Result<Const, Fault> {
        match op {
            BinOp::Or => self.or(rhs),
            BinOp::And => self.and(rhs),
            BinOp::Gt => self.gt(rhs),
            BinOp::Ge => self.ge(rhs),
            BinOp::Lt => self.lt(rhs),
            BinOp::Le => self.le(rhs),
            BinOp::Eq => self.eq(rhs),
            BinOp::Ne => self.ne(rhs),
            BinOp::Add => self.add(rhs),
            BinOp::Sub => self.sub(rhs),
            BinOp::Mul => self.mul(rhs),
            BinOp::Div => self.div(rhs),
            BinOp::Mod => self.rem(rhs),
            BinOp::Dot => self.dot(rhs),
        }
    }

    fn unary(&self, op: &UnaOp) -> Result<Const, Fault> {
        match op {
            UnaOp::Not => self.not(),
            UnaOp::Pos => self.pos(),
            UnaOp::Neg => self.neg(),
        }
    }

    fn or(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Bool(x) => *x || rhs.bool()?,
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(Const::Bool(value))
    }

    fn and(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Bool(x) => *x && rhs.bool()?,
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(Const::Bool(value))
    }

    fn gt(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x) > rhs.int()?,
            Const::Float(x) => f64::from_bits(*x) > rhs.float()?,
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(Const::Bool(value))
    }

    fn ge(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x) >= rhs.int()?,
            Const::Float(x) => f64::from_bits(*x) >= rhs.float()?,
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(Const::Bool(value))
    }

    fn lt(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x) < rhs.int()?,
            Const::Float(x) => f64::from_bits(*x) < rhs.float()?,
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(Const::Bool(value))
    }

    fn le(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x) <= rhs.int()?,
            Const::Float(x) => f64::from_bits(*x) <= rhs.float()?,
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(Const::Bool(value))
    }

    fn eq(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x) == rhs.int()?,
            Const::Float(x) => f64::from_bits(*x) == rhs.float()?,
            Const::Bool(x) => *x == rhs.bool()?,
            Const::Str(x) => x.as_ref() == rhs.str()?,
        };
        Ok(Const::Bool(value))
    }

    fn ne(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x) != rhs.int()?,
            Const::Float(x) => f64::from_bits(*x) != rhs.float()?,
            Const::Bool(x) => *x != rhs.bool()?,
            Const::Str(x) => x.as_ref() != rhs.str()?,
        };
        Ok(Const::Bool(value))
    }

    fn add(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x)
                .checked_add(rhs.int()?)
                .ok_or(Fault::InvalidOperation)?
                .into(),
            Const::Float(x) => f64::from_bits(*x).add(rhs.float()?).into(),
            Const::Str(x) => format!("{}{}", x, rhs.str()?).into(),
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(value)
    }

    fn sub(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => Const::from(
                (*x).checked_sub(rhs.int()?)
                    .ok_or(Fault::InvalidOperation)?,
            ),
            Const::Float(x) => f64::from_bits(*x).sub(rhs.float()?).into(),
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(value)
    }

    fn mul(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x)
                .checked_mul(rhs.int()?)
                .ok_or(Fault::InvalidOperation)?
                .into(),
            Const::Float(x) => f64::from_bits(*x).mul(rhs.float()?).into(),
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(value)
    }

    fn div(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x)
                .checked_div(rhs.int()?)
                .ok_or(Fault::InvalidOperation)?
                .into(),
            Const::Float(x) => f64::from_bits(*x).div(rhs.float()?).into(),
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(value)
    }

    fn rem(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x).rem(rhs.int()?).into(),
            Const::Float(x) => f64::from_bits(*x).rem(rhs.float()?).into(),
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(value)
    }

    fn dot(&self, _: &Const) -> Result<Const, Fault> {
        Err(Fault::InvalidOperation)
    }

    fn not(&self) -> Result<Const, Fault> {
        let value = match self {
            Const::Bool(x) => (!x).into(),
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(value)
    }

    fn pos(&self) -> Result<Const, Fault> {
        if matches!(self, Const::Int(_) | Const::Float(_)) {
            Ok(self.clone())
        } else {
            Err(Fault::InvalidOperation)
        }
    }

    fn neg(&self) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (-*x).into(),
            Const::Float(x) => (-f64::from_bits(*x)).into(),
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(value)
    }

    fn int(&self) -> Result<isize, Fault> {
        if let Const::Int(x) = self {
            Ok(*x)
        } else {
            Err(Fault::InvalidOperation)
        }
    }

    fn float(&self) -> Result<f64, Fault> {
        if let Const::Float(x) = self {
            Ok(f64::from_bits(*x))
        } else {
            Err(Fault::InvalidOperation)
        }
    }

    fn bool(&self) -> Result<bool, Fault> {
        if let Const::Bool(x) = self {
            Ok(*x)
        } else {
            Err(Fault::InvalidOperation)
        }
    }

    fn str(&self) -> Result<&str, Fault> {
        if let Const::Str(x) = self {
            Ok(x)
        } else {
            Err(Fault::InvalidOperation)
        }
    }
}

impl From<f64> for Const {
    fn from(x: f64) -> Const {
        Const::Float(x.to_bits())
    }
}

impl From<isize> for Const {
    fn from(x: isize) -> Const {
        Const::Int(x)
    }
}

impl From<bool> for Const {
    fn from(x: bool) -> Const {
        Const::Bool(x)
    }
}

impl From<String> for Const {
    fn from(x: String) -> Const {
        Const::Str(x.into())
    }
}
