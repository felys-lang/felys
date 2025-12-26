use crate::cyrene::Function;
use crate::philia093::Intern;
use std::collections::HashMap;

pub struct Demiurge {
    pub functions: HashMap<usize, Function>,
    pub main: Function,
    pub intern: Intern,
}
