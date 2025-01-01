use crate::ast::expr::Expr;
use crate::ast::format::{Indenter, INDENT};
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub enum Stmt {
    /// single semicolon: `;`
    Empty,
    /// expression: `1 + 1`
    Expr(Expr),
    /// expression with semicolon: `1 + 1;`
    Semi(Expr),
}

impl Indenter for Stmt {
    fn print(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..indent {
            write!(f, "{}", INDENT)?;
        }
        match self {
            Stmt::Empty => write!(f, ";"),
            Stmt::Expr(x) => x.print(indent, f),
            Stmt::Semi(x) => {
                x.print(indent, f)?;
                write!(f, ";")
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Block(pub Vec<Stmt>);

impl Indenter for Block {
    fn print(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        for each in &self.0 {
            each.print(indent + 1, f)?;
            writeln!(f)?
        }
        for _ in 0..indent {
            write!(f, "{}", INDENT)?;
        }
        write!(f, "}}")
    }
}