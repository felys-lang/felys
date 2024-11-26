use crate::expr::Expr;

pub enum Stmt {
    /// single semicolon: `;`
    Empty,
    /// expression: `1 + 1`
    Expr(Expr),
    /// expression with semicolon: `1 + 1;`
    Semi(Expr),
}