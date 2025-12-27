use crate::cyrene::{Function, Group};
use crate::philia093::Intern;
use std::collections::{BTreeMap, HashMap};

pub struct Demiurge {
    pub groups: HashMap<usize, Group>,
    pub fns: BTreeMap<usize, Function>,
    pub main: Function,
    pub intern: Intern,
}
