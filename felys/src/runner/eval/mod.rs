mod expr;
mod lit;
mod stmt;
mod pat;

use crate::ast::Program;
use crate::runner::environ::{Environ, Value};
use crate::runner::execute::{Evaluation, Signal};

impl Evaluation for Program {
    fn _eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        for stmt in &self.0 {
            stmt.eval(env)?.void()?
        }
        Ok(Value::Void)
    }
}