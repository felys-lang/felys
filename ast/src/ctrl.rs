use crate::expr::Expr;
use crate::format::Indenter;
use crate::pat::Pat;
use crate::stmt::Block;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum Ctrl {
    /// assignment: `x = 42`
    Assign(Pat, AssOp, Expr),
    /// code block: `{ elysia }`
    Block(Block),
    /// break the loop: `break elysia;`
    Break(Option<Expr>),
    /// skip to the next loop: `continue`
    Continue,
    /// for loop: `for x in array { block }`
    For(Pat, Expr, Block),
    /// match: `match x { Elysia => 1, _ => 0 }`
    Match(Expr, Vec<(Pat, Expr)>),
    /// if statement with optional else: `if expr { block } else { block }`
    If(Expr, Block, Option<Expr>),
    /// loop with not tests: `loop { block }`
    Loop(Block),
    /// return value: `return elysia`
    Return(Option<Expr>),
    /// while loop: `while expr { block }`
    While(Expr, Block),
}

impl Indenter for Ctrl {
    fn print(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Ctrl::Assign(lhs, op, rhs) => {
                lhs.print(indent, f)?;
                write!(f, " {} ", op)?;
                rhs.print(indent, f)
            }
            Ctrl::Block(block) => block.print(indent, f),
            Ctrl::Break(x) => {
                write!(f, "break")?;
                if let Some(expr) = x {
                    write!(f, " ")?;
                    expr.print(indent, f)
                } else {
                    Ok(())
                }
            }
            Ctrl::Continue => write!(f, "continue"),
            Ctrl::For(pat, expr, block) => {
                write!(f, "for ")?;
                pat.print(indent, f)?;
                write!(f, " in ")?;
                expr.print(indent, f)?;
                write!(f, " ")?;
                block.print(indent, f)
            }
            Ctrl::Match(_, _) => todo!(),
            Ctrl::If(expr, block, then) => {
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
            Ctrl::Loop(block) => {
                write!(f, "loop ")?;
                block.print(indent, f)
            }
            Ctrl::Return(x) => {
                write!(f, "return")?;
                if let Some(expr) = x {
                    write!(f, " ")?;
                    expr.print(indent, f)
                } else {
                    Ok(())
                }
            }
            Ctrl::While(expr, block) => {
                write!(f, "while ")?;
                expr.print(indent, f)?;
                write!(f, " ")?;
                block.print(indent, f)
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