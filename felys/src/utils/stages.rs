use crate::philia093::Intern;
use crate::utils::ast::{Block, Root};
use crate::utils::bytecode::{Bytecode, Reg};
use crate::utils::function::Const;
use crate::utils::group::Group;
use crate::utils::namespace::Namespace;
use std::collections::HashMap;

pub struct I {
    pub root: Root,
    pub intern: Intern,
}

pub struct II {
    pub namespace: Namespace,
    pub groups: HashMap<usize, Group>,
    pub functions: HashMap<usize, (Vec<usize>, Block)>,
    pub main: (usize, Block),
    pub intern: Intern,
}

pub struct III {
    pub main: Callable,
    pub text: Vec<Callable>,
    pub data: Vec<Const>,
    pub groups: Vec<Group>,
}

#[derive(Debug)]
pub struct Callable {
    pub registers: Reg,
    pub bytecodes: Vec<Bytecode>,
}
