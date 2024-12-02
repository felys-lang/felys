use crate::format::Indenter;
use crate::Symbol;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum Lit {
    /// integer: `0xf`, `0o77`, `15`, `0b1111`
    Int(Int),
    /// decimal: `11.11`
    Float(Float),
    /// boolean: `true`, `false`
    Bool(Bool),
    /// string: `"elysia"`, `f"{1+1} = 2"`, `r"\t\r\n"`
    Str(Str),
}

impl Indenter for Lit {
    fn print(&self, _: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Lit::Int(x) => write!(f, "{}", x),
            Lit::Float(x) => write!(f, "{}", x),
            Lit::Bool(x) => write!(f, "{}", x),
            Lit::Str(x) => write!(f, "\"{}\"", x),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Int {
    Base16(Symbol),
    Base10(Symbol),
    Base8(Symbol),
    Base2(Symbol),
}

impl Display for Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Int::Base16(x) => write!(f, "{}", x),
            Int::Base10(x) => write!(f, "{}", x),
            Int::Base8(x) => write!(f, "{}", x),
            Int::Base2(x) => write!(f, "{}", x),
        }
    }
}

pub type Float = Symbol;

#[derive(Clone, Debug)]
pub enum Bool {
    True,
    False,
}

impl Display for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Bool::True => write!(f, "true"),
            Bool::False => write!(f, "false"),
        }
    }
}

pub type Str = Symbol;