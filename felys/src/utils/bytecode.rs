use crate::utils::ast::{BinOp, UnaOp};
use crate::utils::function::Pointer;

#[derive(Debug)]
pub enum Bytecode {
    Arg(Reg, Index),
    Field(Reg, Reg, Id),
    Unpack(Reg, Reg, Index),
    Pointer(Reg, Pointer, Index),
    Load(Reg, Index),
    Binary(Reg, Reg, BinOp, Reg),
    Unary(Reg, UnaOp, Reg),
    Call(Reg, Reg, Vec<Reg>),
    List(Reg, Vec<Reg>),
    Tuple(Reg, Vec<Reg>),
    Index(Reg, Reg, Reg),
    Method(Reg, Reg, Id, Vec<Reg>),
    Branch(Reg, Index, Index),
    Jump(Index),
    Return(Reg),
    Copy(Reg, Reg),
}

pub type Reg = u8;

pub type Index = u32;

pub type Id = u32;
