use crate::environ::{Environ, Value};
use ast::pat::Ident;

pub enum Signal {
    Return(Value),
    Error(String),
    Break(Value),
    Continue,
}

pub trait Evaluation {
    fn eval(&self, env: &mut Environ) -> Result<Value, Signal>;
}

pub type Pairs = Vec<(Ident, Value)>;

pub trait Unpack {
    fn unpack(&self, env: &mut Environ, pairs: &mut Pairs, value: Value) -> Result<(), Signal>;
}