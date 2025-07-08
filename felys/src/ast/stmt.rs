use crate::ast::*;

#[derive(Clone, Debug)]
pub enum Stmt {
    /// single semicolon: `;`
    Empty,
    /// expression: `1 + 1`
    Expr(Expr),
    /// expression with semicolon: `1 + 1;`
    Semi(Expr),
}

#[derive(Clone, Debug)]
pub struct Block(pub Vec<Stmt>);

#[derive(Clone, Debug)]
pub struct Grammar(pub Vec<Stmt>);