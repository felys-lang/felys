use crate::ast::common::Block;
use crate::ast::stmt::Stmt;
use crate::parser::packrat::Parser;

impl Parser {
    #[felysium::memoize]
    pub fn stmt(&mut self) -> Option<Stmt> {
        if let Some(res) = self.alter(|x| {
            let expr = x.expr()?;
            x.expect(";")?;
            Some(Stmt::Semi(expr))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let expr = x.expr()?;
            Some(Stmt::Expr(expr))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.expect(";")?;
            Some(Stmt::Empty)
        }) {
            return res;
        }
        None
    }
}
