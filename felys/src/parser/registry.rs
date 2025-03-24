use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use felysian::Cachable;

#[derive(Clone, Cachable)]
pub enum Cache {
    Expr(Option<Expr>),
    Stmt(Option<Stmt>),
}
