use crate::expr::Expr;
use crate::pat::Pat;
use crate::stmt::Stmt;

pub enum Ctrl {
    /// assignment: `x = 42`
    Assign(Pat, AssOp, Expr),
    /// code block: `{ elysia }`
    Block(Vec<Stmt>),
    /// break the loop: `break elysia;`
    Break(Option<Expr>),
    /// skip to the next loop: `continue`
    Continue,
    /// for loop: `for x in array { block }`
    For(Pat, Expr, Box<Ctrl>),
    /// match: `match x { Elysia => 1, _ => 0 }`
    Match(Expr, Vec<(Pat, Expr)>),
    /// if statement with optional else: `if expr { block } else { block }`
    If(Expr, Box<Ctrl>, Option<Box<Ctrl>>),
    /// loop with not tests: `loop { block }`
    Loop(Box<Ctrl>),
    /// return value: `return elysia`
    Return(Option<Expr>),
    /// while loop: `while expr { block }`
    While(Expr, Box<Ctrl>),
}

pub enum AssOp {
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
    Eq,
}