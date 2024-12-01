mod execute;
mod environ;
mod eval;
mod unpack;

use crate::environ::{Environ, Value};
use crate::execute::{Evaluation, Signal};
use ast::Program;
use packrat::Pool;

pub fn exec(program: Program, pool: Pool) -> Result<Value, String> {
    let mut env = Environ::new(&pool);
    match program.eval(&mut env) {
        Ok(_) => Ok(Value::Void),
        Err(Signal::Return(value)) => Ok(value),
        Err(Signal::Error(e)) => Err(e),
        _ => Err("unknown signal".to_string())
    }
}