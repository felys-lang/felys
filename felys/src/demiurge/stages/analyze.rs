use crate::cyrene::{Const, Fragment, Instruction, Label, Terminator, Var};
use crate::demiurge::context::{Meta, Lattice};
use crate::demiurge::Function;
use crate::error::Fault;
use std::collections::HashMap;

enum Id {
    Ins(usize),
    Term,
}

impl Function {
    pub fn analyze(&self) -> Result<Meta, Fault> {
        let mut usage = HashMap::new();
        for (label, fragment) in self.safe() {
            fragment.usage(label, &mut usage);
        }

        let mut ctx = Meta::new(self.vars);
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
    fn analyze(&self, label: Label, ctx: &mut Meta) -> Result<(), Fault> {
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
    fn analyze(&self, ctx: &mut Meta) -> Result<(), Fault> {
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
    fn analyze(&self, label: Label, ctx: &mut Meta) -> Result<(), Fault> {
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
