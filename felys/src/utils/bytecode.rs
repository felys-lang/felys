use crate::utils::ast::{BinOp, UnaOp};
use crate::utils::ir::Pointer;

#[derive(Debug)]
pub enum Bytecode {
    Arg(Reg, Index),
    Field(Reg, Reg, usize),
    Unpack(Reg, Reg, Index),
    Pointer(Reg, Pointer, Index),
    Load(Reg, Index),
    Binary(Reg, Reg, BinOp, Reg),
    Unary(Reg, UnaOp, Reg),
    Call(Reg, Reg, Vec<Reg>),
    List(Reg, Vec<Reg>),
    Tuple(Reg, Vec<Reg>),
    Index(Reg, Reg, Reg),
    Method(Reg, Reg, usize, Vec<Reg>),
    Branch(Reg, Index, Index),
    Jump(Index),
    Return(Reg),
    Copy(Reg, Reg),
}

pub type Reg = u8;

pub type Index = u32;
