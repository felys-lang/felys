use crate::utils::ir::{Instruction, Label, Terminator, Var};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Function {
    pub args: usize,
    pub vars: usize,
    pub entry: Fragment,
    pub fragments: HashMap<usize, Fragment>,
    pub exit: Fragment,
}

impl Function {
    pub fn new(args: usize) -> Self {
        Function {
            args,
            vars: 0,
            entry: Default::default(),
            fragments: Default::default(),
            exit: Default::default(),
        }
    }

    pub fn var(&mut self) -> Var {
        let id = self.vars;
        self.vars += 1;
        id
    }

    pub fn label(&mut self) -> Label {
        let id = self.fragments.len();
        self.fragments.insert(id, Fragment::default());
        Label::Id(id)
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

#[derive(Debug)]
pub struct Phi {
    pub var: Var,
    pub inputs: Vec<(Label, Var)>,
}
