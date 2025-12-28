use crate::ast::{BinOp, UnaOp};

#[derive(Debug)]
pub enum Bytecode {
    Field(Reg, Reg, usize),
    Func(Reg, Idx),
    Load(Reg, Idx),
    Binary(Reg, Reg, BinOp, Reg),
    Unary(Reg, UnaOp, Reg),
    Copy(Reg, Reg),
    Branch(Reg, bool, Idx),
    Jump(Idx),
    Return(Option<Reg>),
    Buffer,
    Push(Reg),
    Call(Reg, Reg),
    List(Reg),
    Tuple(Reg),
    Index(Reg, Reg, Reg),
    Method(Reg, Reg, usize),
    Group(Reg, Idx),
}

pub type Reg = usize;

pub type Idx = usize;
