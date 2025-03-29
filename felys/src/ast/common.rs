use crate::ast::stmt::Stmt;
use crate::ast::utils::Id;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Program(pub Vec<Stmt>);

#[derive(Clone, Debug)]
pub struct Block(pub Vec<Stmt>);

pub type Ident = Id;

#[derive(Clone, Debug)]
pub enum Path {
    /// namespace: `Elysia::sleep`
    Path(Rc<Path>, Ident),
    /// single identifier: `Elysia`, `elysia`
    Ident(Ident),
}
