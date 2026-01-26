use crate::utils::ast::{BinOp, UnaOp};
use std::rc::Rc;

#[derive(Debug)]
pub enum Instruction {
    Arg(Var, usize),
    Field(Var, Var, usize),
    Unpack(Var, Var, usize),
    Pointer(Var, Pointer, usize),
    Load(Var, Const),
    Binary(Var, Var, BinOp, Var),
    Unary(Var, UnaOp, Var),
    Call(Var, Var, Vec<Var>),
    List(Var, Vec<Var>),
    Tuple(Var, Vec<Var>),
    Index(Var, Var, Var),
    Method(Var, Var, usize, Vec<Var>),
}

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum Pointer {
    Function,
    Group,
    Rust,
}

#[derive(Debug)]
pub enum Terminator {
    Branch(Var, Label, Label),
    Jump(Label),
    Return(Var),
}

pub type Var = usize;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Label {
    Entry,
    Id(usize),
    Exit,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Const {
    Int(isize),
    Float(u64),
    Bool(bool),
    Str(Rc<str>),
}
