use crate::ast::format::{Indenter, INDENT};
use crate::ast::lit::Lit;
use crate::ast::pat::{Ident, Pat};
use crate::ast::stmt::Block;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Expr {
    /// assignment: `x = 42`
    Assign(Pat, AssOp, Rc<Expr>),
    /// code block: `{ elysia }`
    Block(Block),
    /// break the loop: `break elysia;`
    Break(Option<Rc<Expr>>),
    /// skip to the next loop: `continue`
    Continue,
    /// for loop: `for x in array { block }`
    For(Pat, Rc<Expr>, Block),
    /// match: `match x { Elysia => 1, _ => 0 }`
    Match(Rc<Expr>, Vec<(Pat, Expr)>),
    /// if statement with optional else: `if expr { block } else { block }`
    If(Rc<Expr>, Block, Option<Rc<Expr>>),
    /// loop with not tests: `loop { block }`
    Loop(Block),
    /// return value: `return elysia`
    Return(Option<Rc<Expr>>),
    /// while loop: `while expr { block }`
    While(Rc<Expr>, Block),
    /// binary operation: `1 + 2`
    Binary(Rc<Expr>, BinOp, Rc<Expr>),
    /// closure: `|x| { x+1 }`, `|x| x+1`
    Func(Vec<Ident>, Rc<Expr>),
    /// function call: `func(1, 2)`
    Call(Rc<Expr>, Vec<Expr>),
    /// field: `elysia.mei`
    Field(Rc<Expr>, Ident),
    /// identifier: `elysia`
    Ident(Ident),
    /// tuple: `(elysia, 11.11)`
    Tuple(Vec<Expr>),
    /// literals: `"elysia"`, `11.11`, `true`
    Lit(Lit),
    /// explicit precedence: `(1 + 2)`
    Paren(Rc<Expr>),
    /// unary operation: `-1`
    Unary(UnaOp, Rc<Expr>),
}

impl Indenter for Expr {
    fn print(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Assign(lhs, op, rhs) => {
                lhs.print(indent, f)?;
                write!(f, " {} ", op)?;
                rhs.print(indent, f)
            }
            Expr::Block(block) => block.print(indent, f),
            Expr::Break(expr) => {
                write!(f, "break")?;
                if let Some(expr) = expr {
                    write!(f, " ")?;
                    expr.print(indent, f)
                } else {
                    Ok(())
                }
            }
            Expr::Continue => write!(f, "continue"),
            Expr::For(pat, expr, block) => {
                write!(f, "for ")?;
                pat.print(indent, f)?;
                write!(f, " in ")?;
                expr.print(indent, f)?;
                write!(f, " ")?;
                block.print(indent, f)
            }
            Expr::Match(expr, arms) => {
                write!(f, "match ")?;
                expr.print(indent, f)?;
                writeln!(f, " {{")?;
                if let Some((pat, expr)) = arms.first() {
                    for _ in 0..indent + 1 {
                        write!(f, "{}", INDENT)?;
                    }
                    pat.print(indent + 1, f)?;
                    write!(f, " => ")?;
                    expr.print(indent + 1, f)?
                }
                for (pat, expr) in arms.iter().skip(1) {
                    write!(f, ",")?;
                    writeln!(f)?;
                    for _ in 0..indent + 1 {
                        write!(f, "{}", INDENT)?;
                    }
                    pat.print(indent + 1, f)?;
                    write!(f, " => ")?;
                    expr.print(indent + 1, f)?;
                }
                writeln!(f)?;
                write!(f, "}}")
            }
            Expr::If(expr, block, then) => {
                write!(f, "if ")?;
                expr.print(indent, f)?;
                write!(f, " ")?;
                block.print(indent, f)?;
                if let Some(then) = then {
                    write!(f, " else ")?;
                    then.print(indent, f)
                } else {
                    Ok(())
                }
            }
            Expr::Loop(block) => {
                write!(f, "loop ")?;
                block.print(indent, f)
            }
            Expr::Return(expr) => {
                write!(f, "return")?;
                if let Some(expr) = expr {
                    write!(f, " ")?;
                    expr.print(indent, f)
                } else {
                    Ok(())
                }
            }
            Expr::While(expr, block) => {
                write!(f, "while ")?;
                expr.print(indent, f)?;
                write!(f, " ")?;
                block.print(indent, f)
            }
            Expr::Binary(lhs, op, rhs) => {
                lhs.print(indent, f)?;
                write!(f, " {} ", op)?;
                rhs.print(indent, f)
            }
            Expr::Func(params, expr) => {
                write!(f, "|")?;
                if let Some(first) = params.first() {
                    write!(f, "{}", first)?
                }
                for each in params.iter().skip(1) {
                    write!(f, ", {}", each)?
                }
                write!(f, "| ")?;
                expr.print(indent, f)
            }
            Expr::Call(func, args) => {
                func.print(indent, f)?;
                write!(f, "(")?;
                if let Some(first) = args.first() {
                    first.print(indent, f)?
                }
                for each in args.iter().skip(1) {
                    write!(f, ",")?;
                    each.print(indent, f)?
                }
                write!(f, ")")
            }
            Expr::Field(root, field) => {
                root.print(indent, f)?;
                write!(f, ".{}", field)
            }
            Expr::Ident(ident) => write!(f, "{}", ident),
            Expr::Tuple(tup) => {
                write!(f, "(")?;
                if let Some(first) = tup.first() {
                    first.print(indent, f)?
                }
                for each in tup.iter().skip(1) {
                    write!(f, ", ")?;
                    each.print(indent, f)?
                }
                write!(f, ")")
            }
            Expr::Lit(lit) => lit.print(indent, f),
            Expr::Paren(expr) => {
                write!(f, "(")?;
                expr.print(indent, f)?;
                write!(f, ")")
            }
            Expr::Unary(op, rhs) => {
                write!(f, " {}", op)?;
                rhs.print(indent, f)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum AssOp {
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
    Eq,
}

impl Display for AssOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssOp::AddEq => write!(f, "+="),
            AssOp::SubEq => write!(f, "-="),
            AssOp::MulEq => write!(f, "*="),
            AssOp::DivEq => write!(f, "/="),
            AssOp::ModEq => write!(f, "%="),
            AssOp::Eq => write!(f, "="),
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