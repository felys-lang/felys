use crate::demiurge::fault::Fault;
use std::collections::{HashMap, HashSet, VecDeque};
use crate::utils::function::{Fragment, Function};
use crate::utils::ir::{Const, Instruction, Label, Terminator, Var};

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
    Phi,
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
                        Id::Phi => fragment.analyze_phis(*label, &mut meta),
                    }
                }
            }
        }
        Ok(meta)
    }
}

impl Fragment {
    fn analyze_phis(&self, label: Label, meta: &mut Meta) {
        for phi in self.phis.iter() {
            let mut new = Lattice::Top;
            for (pred, var) in phi.inputs.iter() {
                if meta.edges.contains(&(*pred, label)) {
                    let input = meta.get(*var);
                    new = new.meet(input);
                }
            }
            meta.update(phi.var, new);
        }
    }

    fn analyze(&self, label: Label, meta: &mut Meta) -> Result<(), Fault> {
        self.analyze_phis(label, meta);
        if meta.visited.insert(label) {
            for instruction in self.instructions.iter() {
                instruction.analyze(meta)?;
            }
            self.terminator.as_ref().unwrap().analyze(label, meta)?;
        }
        Ok(())
    }

    fn usage(&self, label: Label, map: &mut HashMap<Var, Vec<(Label, Id)>>) {
        for phi in self.phis.iter() {
            for (_, input) in phi.inputs.iter() {
                map.entry(*input).or_default().push((label, Id::Phi));
            }
        }
        for (i, instruction) in self.instructions.iter().enumerate() {
            instruction.usage(i, label, map);
        }
        if let Some(terminator) = self.terminator.as_ref() {
            terminator.usage(label, map);
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
            Instruction::Unary(var, op, inner) => {
                let new = match meta.get(*inner) {
                    Lattice::Top => Lattice::Top,
                    Lattice::Const(c) => Lattice::Const(c.unary(op)?),
                    Lattice::Bottom => Lattice::Bottom,
                };
                meta.update(*var, new);
            }
            Instruction::Field(dst, _, _)
            | Instruction::Unpack(dst, _, _)
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
            Instruction::Field(_, x, _)
            | Instruction::Unpack(_, x, _)
            | Instruction::Unary(_, _, x) => update(*x),
            Instruction::Binary(_, l, _, r) => {
                update(*l);
                update(*r);
            }
            Instruction::Index(_, src, x) => {
                update(*src);
                update(*x);
            }
            Instruction::Call(_, x, params) | Instruction::Method(_, x, _, params) => {
                update(*x);
                params.iter().for_each(|x| update(*x));
            }
            Instruction::List(_, params) | Instruction::Tuple(_, params) => {
                params.iter().for_each(|x| update(*x));
            }
            Instruction::Group(_, _) | Instruction::Function(_, _) | Instruction::Load(_, _) => {}
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
