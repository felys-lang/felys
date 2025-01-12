use crate::ast::stmt::{Block, Stmt};
use crate::packrat::Parser;
use crate::parser::registry::{Expression, Statement, CR};

impl Statement for Parser<CR> {
    #[helper::memoize]
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

    fn block(&mut self) -> Option<Block> {
        if let Some(res) = self.alter(|x| {
            x.expect("{")?;
            let mut body = Vec::new();
            while let Some(stmt) = x.stmt() {
                body.push(stmt)
            }
            x.e("scope not closed").expect("}")?;
            Some(Block(body))
        }) {
            return res;
        }
        None
    }
}