use ast::ctrl::Ctrl;
use ast::expr::Expr;
use ast::lit::{Bool, Float, Int, Lit, Str};
use ast::pat::{Ident, Pat};
use ast::stmt::Stmt;
use ast::Program;
use packrat::Cache;

#[derive(Clone, Cache)]
pub enum CR {
    Stmt(Option<Stmt>),
    Expr(Option<Expr>),
    Ctrl(Option<Ctrl>),
    Pat(Option<Pat>),
    Lit(Option<Lit>),
}


pub trait Base {
    type CR;
}

pub trait Entry: Base + Helper + Literal + Expression + Control + Statement + Pattern {
    fn program(&mut self) -> Option<Program>;
}

pub trait Helper: Base {
    fn keyword(&mut self, s: &'static str) -> Option<&'static str>;
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

pub trait Control: Base {
    fn ctrl(&mut self) -> Option<Ctrl>;
    fn assign(&mut self) -> Option<Ctrl>;
}

pub trait Statement: Base {
    fn stmt(&mut self) -> Option<Stmt>;
}

pub trait Pattern: Base {
    fn pat(&mut self) -> Option<Pat>;
    fn ident(&mut self) -> Option<Ident>;
}
