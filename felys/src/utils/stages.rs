use crate::philia093::Intern;
use crate::utils::ast::Root;
use crate::utils::bytecode::{Bytecode, Reg};
use crate::utils::function::Function;
use crate::utils::group::Group;
use crate::utils::ir::Const;
use crate::utils::stdlib::utils::Signature;
use std::collections::HashMap;

pub struct Cyrene {
    pub root: Root,
    pub intern: Intern,
}

pub struct Demiurge {
    pub gps: HashMap<usize, Group>,
    pub fns: HashMap<usize, Function>,
    pub main: Function,
}

pub struct Elysia {
    pub main: Callable,
    pub text: Vec<Callable>,
    pub rust: Vec<Signature>,
    pub data: Vec<Const>,
    pub groups: Vec<Group>,
}

#[derive(Debug)]
pub struct Callable {
    pub args: usize,
    pub registers: Reg,
    pub bytecodes: Vec<Bytecode>,
}
