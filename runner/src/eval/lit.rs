use crate::environ::{Environ, Value};
use crate::execute::{Evaluation, Signal};
use ast::lit::Lit;

impl Evaluation for Lit {
    fn eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        match self {
            Lit::Int(_) => todo!(),
            Lit::Float(_) => todo!(),
            Lit::Bool(_) => todo!(),
            Lit::Str(_) => todo!(),
        }
    }
}