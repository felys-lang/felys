use crate::demiurge::fault::Fault;
use crate::utils::function::{Fragment, Function, Phi};
use crate::utils::ir::{Const, Instruction, Label, Terminator, Var};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq)]
pub enum Lattice {
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

pub struct Meta {
    pub visited: HashSet<Label>,
    values: Vec<Lattice>,
    edges: HashSet<(Label, Label)>,

    flow: VecDeque<(Label, Label)>,
    ssa: VecDeque<Var>,
}

impl Meta {
    fn new(vars: usize) -> Self {
        Self {
            values: vec![Lattice::Top; vars],
            edges: HashSet::new(),
            visited: HashSet::new(),
            flow: VecDeque::new(),
            ssa: VecDeque::new(),
        }
    }

    pub fn get(&self, var: Var) -> &Lattice {
        self.values.get(var).unwrap_or(&Lattice::Top)
    }

    fn update(&mut self, var: Var, new: Lattice) {
        let old = &mut self.values[var];
        if *old != new {
            *old = new;
            self.ssa.push_back(var);
        }
    }
}

enum Id {
    Phi(usize),
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
                        Id::Phi(index) => fragment
                            .phis
                            .get(*index)
                            .unwrap()
                            .analyze(*label, &mut meta)?,
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
        if !meta.visited.contains(&Label::Exit) {
            return Err(Fault::ExitBlockUnreachable);
        }
        Ok(meta)
    }
}

impl Fragment {
    fn analyze(&self, label: Label, meta: &mut Meta) -> Result<(), Fault> {
        for phi in self.phis.iter() {
            phi.analyze(label, meta)?;
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
        for (i, phi) in self.phis.iter().enumerate() {
            phi.usage(i, label, map);
        }
        for (i, instruction) in self.instructions.iter().enumerate() {
            instruction.usage(i, label, map);
        }
        if let Some(terminator) = self.terminator.as_ref() {
            terminator.usage(label, map);
        }
    }
}

impl Phi {
    fn analyze(&self, label: Label, meta: &mut Meta) -> Result<(), Fault> {
        let mut new = Lattice::Top;
        for (pred, var) in self.inputs.iter() {
            if meta.edges.contains(&(*pred, label)) {
                let input = meta.get(*var);
                new = new.meet(input);
            }
        }
        meta.update(self.var, new);
        Ok(())
    }

    fn usage(&self, i: usize, label: Label, map: &mut HashMap<Var, Vec<(Label, Id)>>) {
        for (_, input) in self.inputs.iter() {
            map.entry(*input).or_default().push((label, Id::Phi(i)));
        }
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
            Instruction::Unary(var, op, src) => {
                let new = match meta.get(*src) {
                    Lattice::Top => Lattice::Top,
                    Lattice::Const(c) => Lattice::Const(c.unary(op)?),
                    Lattice::Bottom => Lattice::Bottom,
                };
                meta.update(*var, new);
            }
            Instruction::Arg(dst, _)
            | Instruction::Field(dst, _, _)
            | Instruction::Unpack(dst, _, _)
            | Instruction::Call(dst, _, _)
            | Instruction::List(dst, _)
            | Instruction::Tuple(dst, _)
            | Instruction::Index(dst, _, _)
            | Instruction::Method(dst, _, _, _)
            | Instruction::Pointer(dst, _, _) => meta.update(*dst, Lattice::Bottom),
        }
        Ok(())
    }

    fn usage(&self, i: usize, label: Label, map: &mut HashMap<Var, Vec<(Label, Id)>>) {
        let mut update = |var: &Var| {
            map.entry(*var).or_default().push((label, Id::Ins(i)));
        };
        match self {
            Instruction::Field(_, src, _)
            | Instruction::Unpack(_, src, _)
            | Instruction::Unary(_, _, src) => update(src),
            Instruction::Binary(_, src, _, other) | Instruction::Index(_, src, other) => {
                update(src);
                update(other);
            }
            Instruction::Call(_, src, args) | Instruction::Method(_, src, _, args) => {
                update(src);
                args.iter().for_each(update);
            }
            Instruction::List(_, args) | Instruction::Tuple(_, args) => {
                args.iter().for_each(update);
            }
            Instruction::Arg(_, _) | Instruction::Pointer(_, _, _) | Instruction::Load(_, _) => {}
        }
    }
}

impl Terminator {
    fn analyze(&self, label: Label, meta: &mut Meta) -> Result<(), Fault> {
        match self {
            Terminator::Branch(cond, yes, no) => {
                let val = meta.get(*cond);
                match val {
                    Lattice::Const(c) => {
                        if c.bool()? {
                            meta.flow.push_back((label, *yes))
                        } else {
                            meta.flow.push_back((label, *no))
                        }
                    }
                    Lattice::Bottom => {
                        meta.flow.push_back((label, *yes));
                        meta.flow.push_back((label, *no));
                    }
                    Lattice::Top => {}
                }
            }
            Terminator::Jump(next) => meta.flow.push_back((label, *next)),
            Terminator::Return(_) => {}
        }
        Ok(())
    }

    fn usage(&self, label: Label, map: &mut HashMap<Var, Vec<(Label, Id)>>) {
        let mut update = |var: &Var| {
            map.entry(*var).or_default().push((label, Id::Term));
        };
        match self {
            Terminator::Branch(x, _, _) | Terminator::Return(x) => update(x),
            Terminator::Jump(_) => {}
        }
    }
}
