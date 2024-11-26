use crate::registry::{Expression, Statement, CR};
use ast::stmt::Stmt;
use packrat::Parser;

impl Statement for Parser<CR> {
    #[packrat::memoize]
    fn stmt(&mut self) -> Option<Stmt> {
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