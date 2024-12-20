mod expr;
mod lit;
mod ctrl;
mod stmt;
mod pat;

use crate::environ::{Environ, Value};
use crate::execute::{Evaluation, Signal};
use ast::Program;

impl Evaluation for Program {
    fn _eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        for stmt in &self.0 {
            stmt.eval(env)?.void()?
        }
        Ok(Value::Void)
    }
}