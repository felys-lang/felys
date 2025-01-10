mod execute;
mod environ;
mod eval;
mod unpack;

use std::collections::HashMap;
use crate::ast::Program;
use crate::packrat::Intern;
use crate::runner::environ::{Environ, Value, Warehouse};
use crate::runner::execute::{Evaluation, Signal};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn exec(program: Program, mut intern: Intern, timeout: u64, depth: u64) -> Result<Value, &'static str> {
    let (tx, rx) = mpsc::channel();
    let limit = Duration::from_millis(timeout);
    if !limit.is_zero() {
        thread::spawn(move || {
            thread::sleep(limit);
            tx.send(true)
        });
    }

    let base = HashMap::from([
        (intern.id("__elysia__".to_string()), Value::Str("粉色妖精小姐♪".to_string())),
        (intern.id("__author__".to_string()), Value::Str("jonny.jin@uwaterloo.ca".to_string())),
    ]);

    let mut env = Environ {
        intern: &intern,
        timer: &rx,
        depth: (0, depth),
        warehouse: Warehouse { floors: vec![base] },
    };

    match program.eval(&mut env) {
        Ok(_) => Ok(Value::Void),
        Err(Signal::Return(value)) => Ok(value),
        Err(Signal::Error(e)) => Err(e),
        _ => Err("invalid signal")
    }
}