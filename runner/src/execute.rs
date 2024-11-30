use crate::environ::{Environ, Value};

pub enum Signal {
    Return(Value),
    Error(String),
    Break(Value),
    Continue,
}

pub trait Evaluation {
    fn eval(&self, env: &mut Environ) -> Result<Value, Signal>;
}