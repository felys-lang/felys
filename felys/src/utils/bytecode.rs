use crate::utils::ast::{BinOp, UnaOp};
use crate::utils::ir::Pointer;

#[derive(Debug)]
pub enum Bytecode {
    Arg(Reg, Idx),
    Field(Reg, Reg, usize),
    Unpack(Reg, Reg, usize),
    Pointer(Reg, Pointer, Idx),
    Load(Reg, Idx),
    Binary(Reg, Reg, BinOp, Reg),
    Unary(Reg, UnaOp, Reg),
    Call(Reg, Reg, Vec<Reg>),
    List(Reg, Vec<Reg>),
    Tuple(Reg, Vec<Reg>),
    Index(Reg, Reg, Reg),
    Method(Reg, Reg, usize, Vec<Reg>),
    Branch(Reg, Idx, Idx),
    Jump(Idx),
    Return(Reg),
    Copy(Reg, Reg),
}

pub type Reg = usize;

pub type Idx = usize;
