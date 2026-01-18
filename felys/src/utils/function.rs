use crate::utils::ir::{Instruction, Label, Terminator, Var};
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug)]
pub struct Function {
    pub args: Range<usize>,
    pub vars: usize,
    pub labels: usize,
    pub entry: Fragment,
    pub fragments: HashMap<usize, Fragment>,
    pub exit: Fragment,
}

impl Function {
    pub fn var(&mut self) -> Var {
        let var = self.vars;
        self.vars += 1;
        var
    }

    pub fn label(&mut self) -> Label {
        let label = self.labels;
        self.labels += 1;
        Label::Id(label)
    }

    pub fn add(&mut self, label: Label) -> &mut Fragment {
        let Label::Id(id) = label else { panic!() };
        self.fragments.entry(id).or_default()
    }

    pub fn get(&self, label: Label) -> Option<&Fragment> {
        match label {
            Label::Entry => Some(&self.entry),
            Label::Id(id) => self.fragments.get(&id),
            Label::Exit => Some(&self.exit),
        }
    }

    pub fn modify(&mut self, label: Label) -> Option<&mut Fragment> {
        match label {
            Label::Entry => Some(&mut self.entry),
            Label::Id(id) => self.fragments.get_mut(&id),
            Label::Exit => Some(&mut self.exit),
        }
    }
}

#[derive(Debug, Default)]
pub struct Fragment {
    pub phis: Vec<Phi>,
    pub predecessors: Vec<Label>,
    pub instructions: Vec<Instruction>,
    pub terminator: Option<Terminator>,
}

#[derive(Debug, Default)]
pub struct Phi {
    pub var: Var,
    pub inputs: Vec<(Label, Var)>,
}
