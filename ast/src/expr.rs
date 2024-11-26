use crate::ctrl::Ctrl;
use crate::lit::Lit;
use crate::pat::Ident;

#[derive(Clone)]
pub enum Expr {
    /// binary operation: `1 + 2`
    Binary(Box<Expr>, BinOp, Box<Expr>),
    /// closure: `|x| { x+1 }`, `|x| x+1`
    Closure(Vec<Ident>, Box<Expr>),
    /// function call: `func(1, 2)`
    Call(Box<Expr>, Vec<Expr>),
    /// field: `elysia.mei`, `elysia.0`
    Field(Box<Expr>, Ident),
    /// tuple: `(elysia, 11.11)`
    Tuple(Vec<Expr>),
    /// literals: `"elysia"`, `11.11`, `true`
    Lit(Lit),
    /// explicit precedence: `(1 + 2)`
    Paren(Box<Expr>),
    /// flow control in expression: `1 + if true { 1 } else { 2 }`
    Ctrl(Box<Ctrl>),
    /// unary operation: `-1`
    Unary(UnaOp, Box<Expr>),
}

#[derive(Clone)]
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
}

#[derive(Clone)]
pub enum UnaOp {
    Not,
    Pos,
    Neg,
}