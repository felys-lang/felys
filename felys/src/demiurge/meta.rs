use crate::cyrene::{Const, Label, Var};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq)]
pub enum Lattice {
    Top,
    Const(Const),
    Bottom,
}

impl Lattice {
    pub fn meet(&self, other: &Self) -> Self {
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
    values: Vec<Lattice>,
    pub edges: HashSet<(Label, Label)>,
    pub visited: HashSet<Label>,

    pub flow: VecDeque<(Label, Label)>,
    pub ssa: VecDeque<Var>,
}

impl Meta {
    pub fn new(vars: usize) -> Self {
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

    pub fn update(&mut self, var: Var, new: Lattice) {
        let old = &mut self.values[var];
        if *old != new {
            *old = new;
            self.ssa.push_back(var);
        }
    }
}
