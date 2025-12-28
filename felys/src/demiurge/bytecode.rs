use crate::ast::{BinOp, UnaOp};

#[derive(Debug)]
pub enum Bytecode {
    Field(usize, usize, usize),
    Func(usize, usize),
    Load(usize, usize),
    Binary(usize, usize, BinOp, usize),
    Unary(usize, UnaOp, usize),
    Copy(usize, usize),
    Branch(usize, bool, usize),
    Jump(usize),
    Return(Option<usize>),
    Buffer,
    Push(usize),
    Call(usize, usize),
    List(usize),
    Tuple(usize),
    Index(usize, usize, usize),
    Method(usize, usize, usize),
    Group(usize, usize),
}
