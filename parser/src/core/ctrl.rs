use crate::registry::{Control, Expression, Helper, Pattern, Statement, CR};
use ast::ctrl::{AssOp, Ctrl};
use packrat::Parser;

impl Control for Parser<CR> {
    #[packrat::memoize]
    fn ctrl(&mut self) -> Option<Ctrl> {
        if let Some(res) = self.alter(|x| {
            x.assign()
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.expect("{")?;
            let mut body = Vec::new();
            while let Some(stmt) = x.stmt() {
                body.push(stmt)
            }
            x.expect("}")?;
            Some(Ctrl::Block(body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("break")?;
            let body = x.expr();
            Some(Ctrl::Break(body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("continue")?;
            Some(Ctrl::Continue)
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("for")?;
            let pat = x.pat()?;
            x.keyword("in")?;
            let expr = x.expr()?;
            let body = x.ctrl()?;
            Some(Ctrl::For(pat, expr, body.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("match")?;
            let expr = x.expr()?;
            x.expect("{")?;
            let mut body = Vec::new();
            if let Some(pat) = x.pat() {
                x.expect("=>")?;
                let expr = x.expr()?;
                body.push((pat, expr));
                while x.expect(",").is_some() {
                    let pat = x.pat()?;
                    x.expect("=>")?;
                    let expr = x.expr()?;
                    body.push((pat, expr));
                }
            }
            x.expect("}")?;
            Some(Ctrl::Match(expr, body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("if")?;
            let expr = x.expr()?;
            let body = x.ctrl()?;
            let mut alter = None;
            if x.expect("else").is_some() {
                alter = Some(x.ctrl()?.into())
            }
            Some(Ctrl::If(expr, body.into(), alter))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("loop")?;
            let body = x.ctrl()?;
            Some(Ctrl::Loop(body.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("return")?;
            let body = x.expr();
            Some(Ctrl::Return(body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("while")?;
            let expr = x.expr()?;
            let body = x.ctrl()?;
            Some(Ctrl::While(expr, body.into()))
        }) {
            return res;
        }
        None
    }

    #[packrat::memoize]
    fn assign(&mut self) -> Option<Ctrl> {
        if let Some(res) = self.alter(|x| {
            let pat = x.pat()?;
            x.expect("=")?;
            let expr = x.expr()?;
            Some(Ctrl::Assign(pat, AssOp::Eq, expr))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let pat = x.pat()?;
            x.expect("+=")?;
            let expr = x.expr()?;
            Some(Ctrl::Assign(pat, AssOp::AddEq, expr))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let pat = x.pat()?;
            x.expect("-=")?;
            let expr = x.expr()?;
            Some(Ctrl::Assign(pat, AssOp::SubEq, expr))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let pat = x.pat()?;
            x.expect("*=")?;
            let expr = x.expr()?;
            Some(Ctrl::Assign(pat, AssOp::MulEq, expr))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let pat = x.pat()?;
            x.expect("/=")?;
            let expr = x.expr()?;
            Some(Ctrl::Assign(pat, AssOp::DivEq, expr))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let pat = x.pat()?;
            x.expect("%=")?;
            let expr = x.expr()?;
            Some(Ctrl::Assign(pat, AssOp::ModEq, expr))
        }) {
            return res;
        }
        None
    }
}