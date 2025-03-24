use crate::ast::expr::Expr;
use crate::parser::packrat::Parser;

impl Parser {
    pub fn expr(&mut self) -> Option<Expr> {
        None
    }
}
