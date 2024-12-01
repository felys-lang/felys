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

    fn scoped(&self, env: &mut Environ, prelude: Pairs) -> Result<Value, Signal> {
        env.warehouse.stack();
        for (ident, val) in prelude {
            env.warehouse.put(ident.into(), val)
        }
        let result = self.eval(env);
        env.warehouse.unstack();
        result
    }
}

pub type Pairs = Vec<(Ident, Value)>;

pub trait Unpack {
    fn unpack(&self, env: &mut Environ, pairs: &mut Pairs, value: Value) -> Result<(), Signal>;
}