mod execute;
mod environ;
mod eval;
mod unpack;

use crate::environ::{Environ, Value};
use crate::execute::{Evaluation, Signal};
use ast::Program;
use packrat::Pool;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn exec(program: Program, pool: Pool, timeout: u64, depth: usize) -> Result<Value, String> {
    let (tx, rx) = mpsc::channel();
    let mut env = Environ::new(&pool, &rx, depth);

    let limit = Duration::from_millis(timeout);
    if !limit.is_zero() {
        thread::spawn(move || {
            thread::sleep(limit);
            tx.send(true)
        });
    }

    match program.eval(&mut env) {
        Ok(_) => Ok(Value::Void),
        Err(Signal::Return(value)) => Ok(value),
        Err(Signal::Error(e)) => Err(e),
        _ => Err("unknown signal".to_string())
    }
}