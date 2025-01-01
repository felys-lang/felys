use crate::runner::environ::{Environ, Value};
use crate::ast::pat::Ident;

pub enum Signal {
    Error(&'static str),
    Return(Value),
    Break(Value),
    Continue,
}

pub trait Evaluation {
    fn _eval(&self, env: &mut Environ) -> Result<Value, Signal>;
    fn eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        if env.timer.try_recv().unwrap_or(false) {
            return Err(Signal::Error("timeout"));
        }
        self._eval(env)
    }
}

pub type Pairs = Vec<(Ident, Value)>;

pub trait Unpack {
    fn unpack(&self, env: &mut Environ, pairs: &mut Pairs, value: Value) -> Result<(), Signal>;
}