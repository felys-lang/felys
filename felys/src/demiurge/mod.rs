use crate::cyrene::Function;
use crate::philia093::Intern;
use std::collections::BTreeMap;

pub struct Demiurge {
    pub functions: BTreeMap<usize, Function>,
    pub main: Function,
    pub intern: Intern,
}
