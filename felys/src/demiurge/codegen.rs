use crate::cyrene::{Fragment, Group, Label};
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
    pub entry: Fragment,
    pub fragments: HashMap<usize, Fragment>,
    pub exit: Fragment,
}

impl Function {
    pub fn get(&self, label: Label) -> Option<&Fragment> {
        match label {
            Label::Entry => Some(&self.entry),
            Label::Id(id) => self.fragments.get(&id),
            Label::Exit => Some(&self.exit),
        }
    }
}
