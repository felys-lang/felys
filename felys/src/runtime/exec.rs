use crate::ast::Grammar;
use crate::rspegen::Intern;
use crate::runtime::context::backend::Backend;
use crate::runtime::context::value::Value;
use crate::runtime::shared::{Evaluation, Signal};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

impl Grammar {
    pub fn exec(&self, mut intern: Intern, timeout: u64, depth: u64) -> Result<Value, String> {
        let (tx, rx) = mpsc::channel();
        let limit = Duration::from_millis(timeout);
        if !limit.is_zero() {
            thread::spawn(move || {
                thread::sleep(limit);
                tx.send(true)
            });
        }

        let base = HashMap::from([
            (
                intern.id("__elysia__"),
                Value::Str("粉色妖精小姐♪".to_string()),
            ),
            (
                intern.id("__author__"),
                Value::Str("jonny.jin@uwaterloo.ca".to_string()),
            ),
        ]);

        let mut backend = Backend {
            intern: &intern,
            timer: &rx,
            depth: (0, depth),
            data: vec![base],
        };

        match self.eval(&mut backend) {
            Ok(_) => Ok(Value::Void),
            Err(Signal::Return(value)) => Ok(value),
            Err(Signal::Error(e)) => Err(e.to_string()),
            _ => Err("invalid signal".to_string()),
        }
    }
}

impl Evaluation for Grammar {
    fn __eval(&self, backend: &mut Backend) -> Result<Value, Signal> {
        for stmt in &self.0 {
            stmt.eval(backend)?;
        }
        Ok(Value::Void)
    }
}
