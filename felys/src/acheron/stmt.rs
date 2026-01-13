use crate::acheron::*;

#[derive(Clone, Debug)]
pub enum Stmt {
    Empty,
    Expr(Expr),
    Semi(Expr),
    Assign(Pat, AssOp, Expr),
}

#[derive(Clone, Debug)]
pub enum AssOp {
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
    Eq,
}

#[derive(Clone, Debug)]
pub struct Block(pub Vec<Stmt>);
