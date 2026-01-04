use crate::cyrene::{Fragment, Group, Label};
use crate::error::Fault;
use crate::philia093::Intern;
use std::collections::HashMap;

pub struct Demiurge {
    pub groups: HashMap<usize, Group>,
    pub fns: HashMap<usize, Function>,
    pub main: Function,
    pub intern: Intern,
}

#[derive(Debug)]
pub struct Function {
    pub args: Vec<usize>,
    pub vars: usize,
    pub entry: Fragment,
    pub fragments: HashMap<usize, Fragment>,
    pub exit: Fragment,
}

impl Demiurge {
    pub fn optimize(mut self) -> Result<Self, Fault> {
        for function in self.fns.values_mut() {
            function.optimize()?;
        }
        self.main.optimize()?;
        Ok(self)
    }
}

impl Function {
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

    pub fn dangerous(&mut self) -> impl Iterator<Item=(Label, &mut Fragment)> {
        let fragments = self
            .fragments
            .iter_mut()
            .map(|(id, frag)| (Label::Id(*id), frag));
        [(Label::Entry, &mut self.entry)]
            .into_iter()
            .chain(fragments)
            .chain([(Label::Exit, &mut self.exit)])
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
