use crate::ast::expr::{AssOp, BinOp, Expr, UnaOp};
use crate::parser::Parser;
use std::rc::Rc;

impl Parser {
    #[felysium::memoize]
    pub fn expr(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| x.assign()) {
            return res;
        }
        if let Some(res) = self.alter(|x| x.tuple()) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let body = x.block()?;
            Some(Expr::Block(body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("break")?;
            let body = x.expr().map(Rc::new);
            Some(Expr::Break(body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("continue")?;
            Some(Expr::Continue)
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("for")?;
            let pat = x.pat()?;
            x.keyword("in")?;
            let expr = x.expr()?;
            let body = x.block()?;
            Some(Expr::For(pat, expr.into(), body))
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
            Some(Expr::Match(expr.into(), body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("if")?;
            let expr = x.expr()?;
            let body = x.block()?;
            let mut alter = None;
            if x.expect("else").is_some() {
                alter = Some(x.expr()?.into())
            }
            Some(Expr::If(expr.into(), body, alter))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("loop")?;
            let body = x.block()?;
            Some(Expr::Loop(body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("return")?;
            let body = x.expr().map(Rc::new);
            Some(Expr::Return(body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("while")?;
            let expr = x.expr()?;
            let body = x.block()?;
            Some(Expr::While(expr.into(), body))
        }) {
            return res;
        }
        None
    }

    #[felysium::memoize]
    pub fn assign(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| {
            let pat = x.pat()?;
            x.expect("=")?;
            x.lookahead(|c| c != '=')?;
            let expr = x.expr()?;
            Some(Expr::Assign(pat, AssOp::Eq, expr.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let pat = x.pat()?;
            x.expect("+=")?;
            let expr = x.expr()?;
            Some(Expr::Assign(pat, AssOp::AddEq, expr.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let pat = x.pat()?;
            x.expect("-=")?;
            let expr = x.expr()?;
            Some(Expr::Assign(pat, AssOp::SubEq, expr.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let pat = x.pat()?;
            x.expect("*=")?;
            let expr = x.expr()?;
            Some(Expr::Assign(pat, AssOp::MulEq, expr.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let pat = x.pat()?;
            x.expect("/=")?;
            let expr = x.expr()?;
            Some(Expr::Assign(pat, AssOp::DivEq, expr.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let pat = x.pat()?;
            x.expect("%=")?;
            let expr = x.expr()?;
            Some(Expr::Assign(pat, AssOp::ModEq, expr.into()))
        }) {
            return res;
        }
        None
    }

    #[felysium::memoize]
    pub fn tuple(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| {
            x.expect("(")?;
            let first = x.expr()?;
            x.expect(",")?;
            let second = x.expr()?;
            let mut body = vec![first, second];
            while x.expect(",").is_some() {
                let expr = x.expr()?;
                body.push(expr)
            }
            x.expect(")")?;
            Some(Expr::Tuple(body))
        }) {
            return res;
        }
        self.disjunction()
    }

    #[felysium::lecursion]
    pub fn disjunction(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| {
            let lhs = x.disjunction()?;
            x.keyword("or")?;
            let rhs = x.conjunction()?;
            Some(Expr::Binary(lhs.into(), BinOp::Or, rhs.into()))
        }) {
            return res;
        }
        self.conjunction()
    }

    #[felysium::lecursion]
    pub fn conjunction(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| {
            let lhs = x.conjunction()?;
            x.keyword("and")?;
            let rhs = x.inversion()?;
            Some(Expr::Binary(lhs.into(), BinOp::And, rhs.into()))
        }) {
            return res;
        }
        self.inversion()
    }

    #[felysium::memoize]
    pub fn inversion(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| {
            x.keyword("not")?;
            let rhs = x.inversion()?;
            Some(Expr::Unary(UnaOp::Not, rhs.into()))
        }) {
            return res;
        }
        self.equality()
    }

    #[felysium::lecursion]
    pub fn equality(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| {
            let lhs = x.equality()?;
            x.expect("==")?;
            let rhs = x.comparison()?;
            Some(Expr::Binary(lhs.into(), BinOp::Eq, rhs.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let lhs = x.equality()?;
            x.expect("!=")?;
            let rhs = x.comparison()?;
            Some(Expr::Binary(lhs.into(), BinOp::Ne, rhs.into()))
        }) {
            return res;
        }
        self.comparison()
    }

    #[felysium::lecursion]
    pub fn comparison(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| {
            let lhs = x.comparison()?;
            x.expect(">=")?;
            let rhs = x.term()?;
            Some(Expr::Binary(lhs.into(), BinOp::Ge, rhs.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let lhs = x.comparison()?;
            x.expect("<=")?;
            let rhs = x.term()?;
            Some(Expr::Binary(lhs.into(), BinOp::Le, rhs.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let lhs = x.comparison()?;
            x.expect(">")?;
            let rhs = x.term()?;
            Some(Expr::Binary(lhs.into(), BinOp::Gt, rhs.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let lhs = x.comparison()?;
            x.expect("<")?;
            let rhs = x.term()?;
            Some(Expr::Binary(lhs.into(), BinOp::Lt, rhs.into()))
        }) {
            return res;
        }
        self.term()
    }

    #[felysium::lecursion]
    pub fn term(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| {
            let lhs = x.term()?;
            x.expect("+")?;
            let rhs = x.factor()?;
            Some(Expr::Binary(lhs.into(), BinOp::Add, rhs.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let lhs = x.term()?;
            x.expect("-")?;
            let rhs = x.factor()?;
            Some(Expr::Binary(lhs.into(), BinOp::Sub, rhs.into()))
        }) {
            return res;
        }
        self.factor()
    }

    #[felysium::lecursion]
    pub fn factor(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| {
            let lhs = x.factor()?;
            x.expect("*")?;
            let rhs = x.unary()?;
            Some(Expr::Binary(lhs.into(), BinOp::Mul, rhs.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let lhs = x.factor()?;
            x.expect("/")?;
            let rhs = x.unary()?;
            Some(Expr::Binary(lhs.into(), BinOp::Div, rhs.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let lhs = x.factor()?;
            x.expect("%")?;
            let rhs = x.unary()?;
            Some(Expr::Binary(lhs.into(), BinOp::Mod, rhs.into()))
        }) {
            return res;
        }
        self.unary()
    }

    #[felysium::memoize]
    pub fn unary(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| {
            x.expect("+")?;
            let rhs = x.unary()?;
            Some(Expr::Unary(UnaOp::Pos, rhs.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.expect("-")?;
            let rhs = x.unary()?;
            Some(Expr::Unary(UnaOp::Neg, rhs.into()))
        }) {
            return res;
        }
        self.evaluation()
    }

    #[felysium::lecursion]
    pub fn evaluation(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| {
            let callable = x.evaluation()?;
            x.expect("(")?;
            let mut body = Vec::new();
            if let Some(expr) = x.expr() {
                body.push(expr);
                while x.expect(",").is_some() {
                    let expr = x.expr()?;
                    body.push(expr)
                }
            }
            x.expect(")")?;
            Some(Expr::Call(callable.into(), body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let parent = x.evaluation()?;
            x.expect(".")?;
            let member = x.ident()?;
            Some(Expr::Field(parent.into(), member))
        }) {
            return res;
        }
        self.primary()
    }

    #[felysium::memoize]
    pub fn primary(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| {
            let body = x.lit()?;
            Some(Expr::Lit(body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let body = x.ident()?;
            Some(Expr::Ident(body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.expect("(")?;
            let body = x.expr()?;
            x.expect(")")?;
            Some(Expr::Paren(body.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.expect("|")?;
            let mut body = Vec::new();
            if let Some(param) = x.ident() {
                body.push(param);
                while x.expect(",").is_some() {
                    let param = x.ident()?;
                    body.push(param)
                }
            }
            x.expect("|")?;
            let expr = x.expr()?;
            Some(Expr::Func(body, expr.into()))
        }) {
            return res;
        }
        None
    }
}
