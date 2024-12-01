use crate::registry::{Control, Expression, Helper, Literal, Pattern, CR};
use ast::expr::{BinOp, Expr, UnaOp};
use packrat::Parser;

impl Expression for Parser<CR> {
    #[packrat::memoize]
    fn expr(&mut self) -> Option<Expr> {
        self.tuple()
    }

    #[packrat::memoize]
    fn tuple(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| {
            x.expect("(")?;
            let first = x.tuple()?;
            x.expect(",")?;
            let second = x.tuple()?;
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

    #[packrat::lecursion]
    fn disjunction(&mut self) -> Option<Expr> {
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

    #[packrat::lecursion]
    fn conjunction(&mut self) -> Option<Expr> {
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

    #[packrat::memoize]
    fn inversion(&mut self) -> Option<Expr> {
        if let Some(res) = self.alter(|x| {
            x.keyword("not")?;
            let rhs = x.inversion()?;
            Some(Expr::Unary(UnaOp::Not, rhs.into()))
        }) {
            return res;
        }
        self.equality()
    }

    #[packrat::lecursion]
    fn equality(&mut self) -> Option<Expr> {
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

    #[packrat::lecursion]
    fn comparison(&mut self) -> Option<Expr> {
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
        self.term()
    }

    #[packrat::lecursion]
    fn term(&mut self) -> Option<Expr> {
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

    #[packrat::lecursion]
    fn factor(&mut self) -> Option<Expr> {
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

    #[packrat::memoize]
    fn unary(&mut self) -> Option<Expr> {
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

    #[packrat::lecursion]
    fn evaluation(&mut self) -> Option<Expr> {
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

    #[packrat::memoize]
    fn primary(&mut self) -> Option<Expr> {
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
        if let Some(res) = self.alter(|x| {
            let body = x.ctrl()?;
            Some(Expr::Ctrl(body.into()))
        }) {
            return res;
        }
        None
    }
}