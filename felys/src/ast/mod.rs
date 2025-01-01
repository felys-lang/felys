pub mod expr;
pub mod pat;
pub mod lit;
pub mod stmt;
pub mod format;


use crate::ast::format::Indenter;
use crate::ast::stmt::Stmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Id(usize);

impl From<usize> for Id {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Id> for usize {
    fn from(value: Id) -> Self {
        value.0
    }
}

impl From<&Id> for usize {
    fn from(value: &Id) -> Self {
        value.0
    }
}

impl Display for Id {
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