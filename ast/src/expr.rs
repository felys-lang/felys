use crate::ctrl::Ctrl;
use crate::lit::Lit;
use crate::pat::Ident;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum Expr {
    /// binary operation: `1 + 2`
    Binary(Box<Expr>, BinOp, Box<Expr>),
    /// closure: `|x| { x+1 }`, `|x| x+1`
    Closure(Vec<Ident>, Box<Expr>),
    /// function call: `func(1, 2)`
    Call(Box<Expr>, Vec<Expr>),
    /// field: `elysia.mei`
    Field(Box<Expr>, Ident),
    /// identifier: `elysia`
    Ident(Ident),
    /// tuple: `(elysia, 11.11)`
    Tuple(Vec<Expr>),
    /// literals: `"elysia"`, `11.11`, `true`
    Lit(Lit),
    /// explicit precedence: `(1 + 2)`
    Paren(Box<Expr>),
    /// flow control in expression: `1 + if true { 1 } else { 2 }`
    Ctrl(Box<Ctrl>),
    /// unary operation: `-1`
    Unary(UnaOp, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(l, op, r) => write!(f, "{} {} {}", l, op, r),
            Expr::Closure(_, _) => todo!(),
            Expr::Call(_, _) => todo!(),
            Expr::Field(p, m) => write!(f, "{}.{}", p, m),
            Expr::Ident(x) => write!(f, "{}", x),
            Expr::Tuple(x) => {
                write!(f, "(")?;
                if let Some(first) = x.first() {
                    write!(f, "{}", first)?
                }
                for each in x {
                    write!(f, ", {}", each)?
                }
                write!(f, ")")
            }
            Expr::Lit(x) => write!(f, "{}", x),
            Expr::Paren(x) => write!(f, "({})", x),
            Expr::Ctrl(x) => write!(f, "{}", x),
            Expr::Unary(op, x) => write!(f, "{}{}", op, x),
        }
    }
}

#[derive(Clone, Debug)]
pub enum BinOp {
    Or,
    And,
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Or => write!(f, "or"),
            BinOp::And => write!(f, "and"),
            BinOp::Gt => write!(f, ">"),
            BinOp::Ge => write!(f, ">="),
            BinOp::Lt => write!(f, "<"),
            BinOp::Le => write!(f, "<="),
            BinOp::Eq => write!(f, "=="),
            BinOp::Ne => write!(f, "!="),
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Mod => write!(f, "%"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum UnaOp {
    Not,
    Pos,
    Neg,
}

impl Display for UnaOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaOp::Not => write!(f, "not "),
            UnaOp::Pos => write!(f, "+"),
            UnaOp::Neg => write!(f, "-"),
        }
    }
}