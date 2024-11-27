use crate::expr::Expr;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum Stmt {
    /// single semicolon: `;`
    Empty,
    /// expression: `1 + 1`
    Expr(Expr),
    /// expression with semicolon: `1 + 1;`
    Semi(Expr),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Empty => write!(f, ";"),
            Stmt::Expr(x) => writeln!(f, "{}", x),
            Stmt::Semi(x) => writeln!(f, "{};", x),
        }
    }
}