use crate::ast::*;
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
    /// match: `match x { Mei => 1, Kiana => 0 }`
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

#[derive(Clone, Debug)]
pub enum AssOp {
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
    Eq,
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

#[derive(Clone, Debug)]
pub enum UnaOp {
    Not,
    Pos,
    Neg,
}
