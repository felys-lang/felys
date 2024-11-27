use crate::ctrl::Ctrl;
use crate::format::Indenter;
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

impl Indenter for Expr {
    fn print(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(lhs, op, rhs) => {
                lhs.print(indent, f)?;
                write!(f, " {} ", op)?;
                rhs.print(indent, f)
            }
            Expr::Closure(param, block) => {
                write!(f, "|")?;
                if let Some(first) = param.first() {
                    write!(f, "{}", first)?
                }
                for each in param.iter().skip(1) {
                    write!(f, ", {}", each)?
                }
                write!(f, "| ")?;
                block.print(indent, f)
            }
            Expr::Call(func, param) => {
                func.print(indent, f)?;
                write!(f, "(")?;
                if let Some(first) = param.first() {
                    first.print(indent, f)?
                }
                for each in param.iter().skip(1) {
                    write!(f, ",")?;
                    each.print(indent, f)?
                }
                write!(f, ")")
            }
            Expr::Field(root, field) => {
                root.print(indent, f)?;
                write!(f, ".{}", field)
            }
            Expr::Ident(x) => write!(f, "{}", x),
            Expr::Tuple(member) => {
                write!(f, "(")?;
                if let Some(first) = member.first() {
                    first.print(indent, f)?
                }
                for each in member.iter().skip(1) {
                    write!(f, ",")?;
                    each.print(indent, f)?
                }
                write!(f, ")")
            }
            Expr::Lit(x) => x.print(indent, f),
            Expr::Paren(expr) => {
                write!(f, "(")?;
                expr.print(indent, f)?;
                write!(f, ")")
            }
            Expr::Ctrl(ctrl) => ctrl.print(indent, f),
            Expr::Unary(op, expr) => {
                write!(f, " {}", op)?;
                expr.print(indent, f)
            }
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