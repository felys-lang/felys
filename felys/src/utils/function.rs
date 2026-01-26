use crate::utils::ir::{Instruction, Label, Terminator, Var};
use std::collections::{HashMap, HashSet};

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

    pub fn safe(&self) -> impl Iterator<Item=(Label, &Fragment)> {
        let fragments = self
            .fragments
            .iter()
            .map(|(id, frag)| (Label::Id(*id), frag));
        [(Label::Entry, &self.entry)]
            .into_iter()
            .chain(fragments)
            .chain([(Label::Exit, &self.exit)])
    }

    pub fn cautious(&mut self) -> impl Iterator<Item=(Label, &mut Fragment)> {
        let fragments = self
            .fragments
            .iter_mut()
            .map(|(id, frag)| (Label::Id(*id), frag));
        [(Label::Entry, &mut self.entry)]
            .into_iter()
            .chain(fragments)
            .chain([(Label::Exit, &mut self.exit)])
    }

    pub fn order(&self) -> impl Iterator<Item=Label> {
        let mut order = self.fragments.keys().cloned().collect::<Vec<_>>();
        order.sort_unstable();
        [Label::Entry]
            .into_iter()
            .chain(order.into_iter().map(Label::Id))
            .chain([Label::Exit])
    }

    pub fn rpo(&self) -> Vec<Label> {
        let mut order = Vec::new();
        let mut visited = HashSet::new();
        fn dfs(f: &Function, label: Label, visited: &mut HashSet<Label>, order: &mut Vec<Label>) {
            if !visited.insert(label) {
                return;
            }
            match f.get(label).unwrap().terminator.as_ref().unwrap() {
                Terminator::Branch(_, yes, no) => {
                    dfs(f, *yes, visited, order);
                    dfs(f, *no, visited, order);
                }
                Terminator::Jump(target) => {
                    dfs(f, *target, visited, order);
                }
                Terminator::Return(_) => {}
            }
            order.push(label);
        }
        dfs(self, Label::Entry, &mut visited, &mut order);
        order.reverse();
        order
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
