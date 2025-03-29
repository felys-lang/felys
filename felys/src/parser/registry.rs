use crate::ast::common::{Ident, Path};
use crate::ast::expr::Expr;
use crate::ast::lit::Lit;
use crate::ast::pat::Pat;
use crate::ast::stmt::Stmt;
use felysium::Cachable;

#[derive(Clone, Cachable)]
pub enum Cache {
    Expr(Option<Expr>),
    Stmt(Option<Stmt>),
    Pat(Option<Pat>),
    Lit(Option<Lit>),
    Path(Option<Path>),
    Ident(Option<Ident>),
}
