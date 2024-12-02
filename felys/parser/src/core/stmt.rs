use crate::registry::{Control, Expression, Statement, CR};
use ast::expr::Expr;
use ast::stmt::{Block, Stmt};
use packrat::Parser;

impl Statement for Parser<CR> {
    #[packrat::memoize]
    fn stmt(&mut self) -> Option<Stmt> {
        if let Some(res) = self.alter(|x| {
            let ctrl = x.ctrl()?;
            x.expect(";")?;
            Some(Stmt::Semi(Expr::Ctrl(ctrl.into())))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let ctrl = x.ctrl()?;
            Some(Stmt::Expr(Expr::Ctrl(ctrl.into())))
        }) {
            return res;
        }
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

    fn block(&mut self) -> Option<Block> {
        if let Some(res) = self.alter(|x| {
            x.expect("{")?;
            let mut body = Vec::new();
            while let Some(stmt) = x.stmt() {
                body.push(stmt)
            }
            x.expect("}")?;
            Some(Block(body))
        }) {
            return res;
        }
        None
    }
}