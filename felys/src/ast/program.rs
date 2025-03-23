use crate::ast::stmt::Stmt;

#[derive(Clone, Debug)]
pub struct Program(pub Vec<Stmt>);
