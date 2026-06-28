use crate::ast::{Block, Root};
use crate::frontend::group::Group;
use crate::frontend::namespace::Namespace;
use crate::philia093::Interner;
use std::collections::HashMap;

pub struct I {
    pub root: Root,
    pub interner: Interner,
}

pub struct II {
    pub namespace: Namespace,
    pub groups: HashMap<usize, Group>,
    pub functions: HashMap<usize, (Vec<usize>, Block)>,
    pub main: (usize, Block),
    pub interner: Interner,
}
