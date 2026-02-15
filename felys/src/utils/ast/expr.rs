use crate::utils::ast::utils::BufVec;
use crate::utils::ast::*;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Expr {
    Block(Block),
    Break(Option<Rc<Expr>>),
    Continue,
    For(Pat, Rc<Expr>, Block),
    If(Rc<Expr>, Block, Option<Rc<Expr>>),
    Loop(Block),
    Return(Rc<Expr>),
    While(Rc<Expr>, Block),
    Binary(Rc<Expr>, BinOp, Rc<Expr>),
    Call(Rc<Expr>, Option<BufVec<Expr, 1>>),
    Field(Rc<Expr>, usize),
    Method(Rc<Expr>, usize, Option<BufVec<Expr, 1>>),
    Index(Rc<Expr>, Rc<Expr>),
    Tuple(BufVec<Expr, 2>),
    List(Option<BufVec<Expr, 1>>),
    Lit(Lit),
    Paren(Rc<Expr>),
    Unary(UnaOp, Rc<Expr>),
    Path(usize, BufVec<usize, 1>),
}

#[derive(Clone, Copy, Debug)]
pub enum BinOp {
    Or,
    And,
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    At,
}

#[derive(Clone, Copy, Debug)]
pub enum UnaOp {
    Not,
    Pos,
    Neg,
}
