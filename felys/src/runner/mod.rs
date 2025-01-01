mod execute;
mod environ;
mod eval;
mod unpack;

use crate::ast::Program;
use crate::packrat::Intern;
use crate::runner::environ::{Environ, Value};
use crate::runner::execute::{Evaluation, Signal};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn exec(program: Program, mut pool: Intern, timeout: u64, depth: usize) -> Result<Value, &'static str> {
    let (tx, rx) = mpsc::channel();
    let mut env = Environ::new(&mut pool, &rx, depth);

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
        _ => Err("invalid signal")
    }
}