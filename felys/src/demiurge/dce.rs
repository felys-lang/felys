use crate::cyrene::{Const, Fragment, Instruction, Label, Terminator, Var};
use crate::demiurge::context::{Context, Lattice, Renamer};
use crate::demiurge::{Demiurge, Function};
use crate::error::Fault;
use std::collections::HashMap;

impl Demiurge {
    pub fn dce(mut self) -> Result<Self, Fault> {
        for function in self.fns.values_mut() {
            function.dce()?;
        }
        self.main.dce()?;
        Ok(self)
    }
}

enum Id {
    Ins(usize),
    Term,
}

impl Function {
    fn dce(&mut self) -> Result<(), Fault> {
        let ctx = self.context()?;
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

    fn context(&self) -> Result<Context, Fault> {
        let mut usage = HashMap::new();
        self.entry.usage(Label::Entry, &mut usage);
        for (id, fragment) in &self.fragments {
            fragment.usage(Label::Id(*id), &mut usage);
        }
        self.exit.usage(Label::Entry, &mut usage);

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
            Instruction::Field(_, var, _) => *var = renamer.get(*var),
            Instruction::Call(_, var, params)
            | Instruction::Method(_, var, _, params)
            | Instruction::List(var, params)
            | Instruction::Tuple(var, params) => {
                *var = renamer.get(*var);
                params.iter_mut().for_each(|x| *x = renamer.get(*x));
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
            | Instruction::Call(dst, _, _)
            | Instruction::List(dst, _)
            | Instruction::Tuple(dst, _)
            | Instruction::Index(dst, _, _)
            | Instruction::Method(dst, _, _, _)
            | Instruction::Group(dst, _) => ctx.update(*dst, Lattice::Bottom),
        }
        Ok(())
    }

    fn usage(&self, i: usize, label: Label, map: &mut HashMap<Var, Vec<(Label, Id)>>) {
        let mut update = |var: Var| {
            map.entry(var).or_default().push((label, Id::Ins(i)));
        };
        match self {
            Instruction::Field(_, x, _) | Instruction::Unary(_, _, x) => update(*x),
            Instruction::Binary(_, l, _, r) => {
                update(*l);
                update(*r);
            }
            Instruction::Index(_, src, x) => {
                update(*src);
                update(*x);
            }
            Instruction::Call(_, x, params)
            | Instruction::Method(_, x, _, params)
            | Instruction::List(x, params)
            | Instruction::Tuple(x, params) => {
                update(*x);
                params.iter().for_each(|x| update(*x));
            }
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
