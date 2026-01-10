use crate::cyrene::{Const, Fragment, Function, Instruction, Label, Terminator, Var};
use crate::demiurge::meta::{Lattice, Meta};
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

        let mut meta = Meta::new(self.vars);
        meta.flow.push_back((Label::Entry, Label::Entry));
        for var in self.args.clone() {
            meta.update(var, Lattice::Bottom);
        }

        while !meta.flow.is_empty() || !meta.ssa.is_empty() {
            while let Some((pred, label)) = meta.flow.pop_front() {
                if !meta.edges.insert((pred, label)) {
                    continue;
                }
                let fragment = self.get(label).unwrap();
                fragment.analyze(label, &mut meta)?;
            }
            while let Some(var) = meta.ssa.pop_front() {
                let Some(users) = usage.get(&var) else {
                    continue;
                };
                for (label, id) in users {
                    if !meta.visited.contains(label) {
                        continue;
                    }
                    let fragment = self.get(*label).unwrap();
                    match id {
                        Id::Ins(index) => fragment
                            .instructions
                            .get(*index)
                            .unwrap()
                            .analyze(&mut meta)?,
                        Id::Term => fragment
                            .terminator
                            .as_ref()
                            .unwrap()
                            .analyze(*label, &mut meta)?,
                    }
                }
            }
        }
        Ok(meta)
    }
}

impl Fragment {
    fn analyze(&self, label: Label, meta: &mut Meta) -> Result<(), Fault> {
        for (var, inputs) in self.phis.iter() {
            let mut new = Lattice::Top;
            for (pred, var) in inputs {
                if meta.edges.contains(&(*pred, label)) {
                    let input = meta.get(*var);
                    new = new.meet(input);
                }
            }
            meta.update(*var, new);
        }
        if meta.visited.insert(label) {
            for instruction in self.instructions.iter() {
                instruction.analyze(meta)?;
            }
            self.terminator.as_ref().unwrap().analyze(label, meta)?;
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
    fn analyze(&self, meta: &mut Meta) -> Result<(), Fault> {
        match self {
            Instruction::Load(var, c) => meta.update(*var, Lattice::Const(c.clone())),
            Instruction::Binary(var, lhs, op, rhs) => {
                let new = match (meta.get(*lhs), meta.get(*rhs)) {
                    (Lattice::Const(l), Lattice::Const(r)) => Lattice::Const(l.binary(op, r)?),
                    (Lattice::Bottom, _) | (_, Lattice::Bottom) => Lattice::Bottom,
                    _ => Lattice::Top,
                };
                meta.update(*var, new);
            }
            Instruction::Unary(var, op, inner) => {
                let new = match meta.get(*inner) {
                    Lattice::Top => Lattice::Top,
                    Lattice::Const(c) => Lattice::Const(c.unary(op)?),
                    Lattice::Bottom => Lattice::Bottom,
                };
                meta.update(*var, new);
            }
            Instruction::Field(dst, _, _)
            | Instruction::Function(dst, _)
            | Instruction::Call(dst, _, _)
            | Instruction::List(dst, _)
            | Instruction::Tuple(dst, _)
            | Instruction::Index(dst, _, _)
            | Instruction::Method(dst, _, _, _)
            | Instruction::Group(dst, _) => meta.update(*dst, Lattice::Bottom),
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
    fn analyze(&self, label: Label, meta: &mut Meta) -> Result<(), Fault> {
        match self {
            Terminator::Branch(cond, yes, no) => {
                let val = meta.get(*cond);
                match val {
                    Lattice::Const(Const::Bool(true)) => meta.flow.push_back((label, *yes)),
                    Lattice::Const(Const::Bool(false)) => meta.flow.push_back((label, *no)),
                    Lattice::Const(_) => return Err(Fault::InvalidOperation),
                    Lattice::Bottom => {
                        meta.flow.push_back((label, *yes));
                        meta.flow.push_back((label, *no));
                    }
                    Lattice::Top => {}
                }
            }
            Terminator::Jump(next) => meta.flow.push_back((label, *next)),
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
