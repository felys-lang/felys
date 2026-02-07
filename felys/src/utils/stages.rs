use crate::philia093::Intern;
use crate::utils::ast::{Block, Root};
use crate::utils::group::Group;
use std::collections::HashMap;

pub struct I {
    pub root: Root,
    pub intern: Intern,
}

pub struct II {
    pub groups: HashMap<usize, Group>,
    pub functions: HashMap<usize, (Vec<usize>, Block)>,
    pub main: (usize, Block),
    pub intern: Intern,
}
