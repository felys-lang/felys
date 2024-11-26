use crate::stmt::Stmt;

pub mod expr;
pub mod ctrl;
pub mod pat;
pub mod lit;
pub mod stmt;

pub type Symbol = usize;

pub struct Program(pub Vec<Stmt>);