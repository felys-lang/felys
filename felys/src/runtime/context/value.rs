use crate::ast::{Expr, Ident};
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    Float(f64),
    Int(isize),
    Str(String),
    Closure(Vec<Ident>, Rc<Expr>),
    Tuple(Vec<Value>),
    List(Vec<Value>),
    Void,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(x) => write!(f, "{}", x),
            Value::Float(x) => write!(f, "{}", x),
            Value::Int(x) => write!(f, "{}", x),
            Value::Str(x) => write!(f, "{}", x),
            Value::Closure(_, x) => write!(f, "<{:p}>", x),
            Value::Tuple(x) => {
                write!(f, "(")?;
                let mut x = x.iter();
                if let Some(first) = x.next() {
                    write!(f, "{}", first)?;
                }
                for val in x {
                    write!(f, ", {}", val)?
                }
                write!(f, ")")
            }
            Value::List(x) => {
                write!(f, "[")?;
                let mut x = x.iter();
                if let Some(first) = x.next() {
                    write!(f, "{}", first)?;
                }
                for val in x {
                    write!(f, ", {}", val)?
                }
                write!(f, "]")
            }
            Value::Void => write!(f, "<void>"),
        }
    }
}
