use crate::ast::expr::Expr;
use crate::ast::lit::{Bool, Float, Int, Lit, Str};
use crate::ast::pat::{Ident, Pat};
use crate::ast::stmt::{Block, Stmt};
use crate::ast::Program;
use helper::Cache;

#[derive(Clone, Cache)]
pub enum CR {
    Stmt(Option<Stmt>),
    Expr(Option<Expr>),
    Pat(Option<Pat>),
    Lit(Option<Lit>),
}


pub trait Base {
    type CR;
}

pub trait Entry: Base + Helper + Literal + Expression + Statement + Pattern {
    fn program(&mut self) -> Option<Program>;
}

pub trait Helper: Base {
    fn keyword(&mut self, s: &'static str) -> Option<&'static str>;
    fn eof(&mut self) -> Option<char>;
}

pub trait Literal: Base {
    fn lit(&mut self) -> Option<Lit>;
    fn int(&mut self) -> Option<Int>;
    fn float(&mut self) -> Option<Float>;
    fn bool(&mut self) -> Option<Bool>;
    fn str(&mut self) -> Option<Str>;
}

pub trait Expression: Base {
    fn expr(&mut self) -> Option<Expr>;
    fn assign(&mut self) -> Option<Expr>;
    fn tuple(&mut self) -> Option<Expr>;
    fn disjunction(&mut self) -> Option<Expr>;
    fn conjunction(&mut self) -> Option<Expr>;
    fn inversion(&mut self) -> Option<Expr>;
    fn equality(&mut self) -> Option<Expr>;
    fn comparison(&mut self) -> Option<Expr>;
    fn term(&mut self) -> Option<Expr>;
    fn factor(&mut self) -> Option<Expr>;
    fn unary(&mut self) -> Option<Expr>;
    fn evaluation(&mut self) -> Option<Expr>;
    fn primary(&mut self) -> Option<Expr>;
}

pub trait Statement: Base {
    fn stmt(&mut self) -> Option<Stmt>;
    fn block(&mut self) -> Option<Block>;
}

pub trait Pattern: Base {
    fn pat(&mut self) -> Option<Pat>;
    fn ident(&mut self) -> Option<Ident>;
}
