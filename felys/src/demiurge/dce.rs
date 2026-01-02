use crate::ast::{BinOp, UnaOp};
use crate::cyrene::{Const, Fragment, Instruction, Label, Var};
use crate::demiurge::{Demiurge, Function};
use crate::error::Fault;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, Div, Mul, Rem, Sub};

impl Demiurge {
    pub fn dec(&self) -> Result<(), Fault> {
        for function in self.fns.values() {
            function.optimize()?;
        }
        self.main.optimize()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Lattice {
    Top,
    Const(Const),
    Bottom,
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

#[derive(Default)]
struct Context {
    values: HashMap<Var, Lattice>,
    edges: HashSet<(Label, Label)>,
    visited: HashSet<Label>,

    flow: VecDeque<(Label, Label)>,
    ssa: VecDeque<Var>,
}

impl Context {
    fn update(&mut self, var: Var, new: Lattice) {
        let old = self.values.entry(var).or_insert(Lattice::Top);
        if *old != new {
            *old = new;
            self.ssa.push_back(var);
        }
    }
}

impl Function {
    fn optimize(&self) -> Result<(), Fault> {
        let usage = self.usage();

        let mut ctx = Context::default();
        ctx.flow.push_back((Label::Entry, Label::Entry));
        for var in self.args.iter() {
            ctx.values.insert(*var, Lattice::Bottom);
        }

        while !ctx.flow.is_empty() || !ctx.ssa.is_empty() {
            while let Some((pred, label)) = ctx.flow.pop_front() {
                if ctx.edges.contains(&(pred, label)) {
                    continue;
                }
                ctx.edges.insert((pred, label));
                let fragment = self.get(label).unwrap();
                fragment.optimize(label, &mut ctx)?;
            }
            while let Some(var) = ctx.ssa.pop_front() {
                if let Some(users) = usage.get(&var) {
                    for (label, idx) in users {
                        if ctx.visited.contains(label) {
                            let instruction = self.loc(*label, *idx);
                            instruction.optimize(*label, &mut ctx)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn usage(&self) -> HashMap<Var, Vec<(Label, usize)>> {
        let mut map = HashMap::new();
        self.entry.usage(Label::Entry, &mut map);
        for (id, fragment) in &self.fragments {
            fragment.usage(Label::Id(*id), &mut map);
        }
        self.exit.usage(Label::Entry, &mut map);
        map
    }

    fn loc(&self, label: Label, index: usize) -> &Instruction {
        match label {
            Label::Entry => self.entry.instructions.get(index).unwrap(),
            Label::Id(id) => self
                .fragments
                .get(&id)
                .unwrap()
                .instructions
                .get(index)
                .unwrap(),
            Label::Exit => self.exit.instructions.get(index).unwrap(),
        }
    }
}

impl Fragment {
    fn optimize(&self, label: Label, ctx: &mut Context) -> Result<(), Fault> {
        for (var, inputs) in self.phis.iter() {
            let mut new = Lattice::Top;
            for (pred, var) in inputs {
                if ctx.edges.contains(&(*pred, label)) {
                    let input = ctx.values.get(var).unwrap_or(&Lattice::Top);
                    new = new.meet(input);
                }
            }
            ctx.update(*var, new);
        }
        if ctx.visited.insert(label) {
            for instruction in self.instructions.iter() {
                instruction.optimize(label, ctx)?;
            }
        }
        Ok(())
    }

    fn usage(&self, label: Label, map: &mut HashMap<Var, Vec<(Label, usize)>>) {
        for (i, instruction) in self.instructions.iter().enumerate() {
            let mut update = |var: Var| {
                map.entry(var).or_default().push((label, i));
            };
            match instruction {
                Instruction::Field(_, x, _) => update(*x),
                Instruction::Binary(_, l, _, r) => {
                    update(*l);
                    update(*r);
                }
                Instruction::Unary(_, _, x) => update(*x),
                Instruction::Branch(x, _, _) => update(*x),
                Instruction::Return(x) => update(*x),
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
}

impl Instruction {
    fn optimize(&self, label: Label, ctx: &mut Context) -> Result<(), Fault> {
        match self {
            Instruction::Load(var, c) => ctx.update(*var, Lattice::Const(c.clone())),
            Instruction::Binary(var, lhs, op, rhs) => {
                let l = ctx.values.get(lhs).unwrap_or(&Lattice::Top);
                let r = ctx.values.get(rhs).unwrap_or(&Lattice::Top);
                let new = match (l, r) {
                    (Lattice::Const(lc), Lattice::Const(rc)) => Lattice::Const(lc.binary(op, rc)?),
                    (Lattice::Bottom, _) | (_, Lattice::Bottom) => Lattice::Bottom,
                    _ => Lattice::Top,
                };
                ctx.update(*var, new);
            }
            Instruction::Unary(var, op, inner) => {
                let val = ctx.values.get(inner).unwrap_or(&Lattice::Top);
                let new = match val {
                    Lattice::Top => Lattice::Top,
                    Lattice::Const(c) => Lattice::Const(c.unary(op)?),
                    Lattice::Bottom => Lattice::Bottom,
                };
                ctx.update(*var, new);
            }
            Instruction::Branch(cond, yes, no) => {
                let val = ctx.values.get(cond).unwrap_or(&Lattice::Top);
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
            Instruction::Jump(next) => ctx.flow.push_back((label, *next)),
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
