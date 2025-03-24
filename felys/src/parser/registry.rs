use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use felysian::Cachable;
use crate::ast::lit::Lit;
use crate::ast::pat::Pat;

#[derive(Clone, Cachable)]
pub enum Cache {
    Expr(Option<Expr>),
    Stmt(Option<Stmt>),
    Pat(Option<Pat>),
    Lit(Option<Lit>),
}
