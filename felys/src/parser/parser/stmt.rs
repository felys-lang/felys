use crate::ast::stmt::{Block, Stmt};
use crate::parser::packrat::Parser;

impl Parser {
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

    pub fn block(&mut self) -> Option<Block> {
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
