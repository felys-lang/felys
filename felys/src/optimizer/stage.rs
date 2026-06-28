use crate::frontend::cfg::function::Const;
use crate::frontend::group::Group;
use crate::optimizer::bytecode::{Bytecode, Reg};

pub struct III {
    pub main: Callable,
    pub text: Vec<Callable>,
    pub data: Vec<Const>,
    pub groups: Vec<Group>,
}

#[derive(Debug)]
pub struct Callable {
    pub args: Reg,
    pub registers: Reg,
    pub bytecodes: Vec<Bytecode>,
}
