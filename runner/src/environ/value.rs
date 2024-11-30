use crate::execute::Signal;
use ast::expr::Expr;
use ast::pat::Ident;

pub enum Value {
    Bool(bool),
    Float(f64),
    Int(isize),
    Str(String),
    Closure(Vec<Ident>, Expr),
    Tuple(Vec<Value>),
    Void,
}

impl Value {
    pub fn void(&self) -> Result<(), Signal> {
        if let Value::Void = self {
            Ok(())
        } else {
            Err(Signal::Error("expect a `void` type".to_string()))
        }
    }
}