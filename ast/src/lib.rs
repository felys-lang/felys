use crate::format::Indenter;
use crate::stmt::Stmt;
use std::fmt::{Display, Formatter};

pub mod expr;
pub mod ctrl;
pub mod pat;
pub mod lit;
pub mod stmt;
mod format;

#[derive(Clone, Debug)]
pub struct Symbol(pub usize);

impl From<usize> for Symbol {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Symbol> for usize {
    fn from(value: Symbol) -> Self {
        value.0
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Program(pub Vec<Stmt>);

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for each in &self.0 {
            each.print(0, f)?;
            writeln!(f)?
        }
        Ok(())
    }
}