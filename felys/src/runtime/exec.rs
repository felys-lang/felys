use crate::ast::Grammar;
use crate::nn::optim::Optimizer;
use crate::parser::Intern;
use crate::runtime::context::backend::{Frame, Global};
use crate::runtime::context::value::Value;
use crate::runtime::shared::{Evaluation, Signal};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

impl Grammar {
    pub fn exec(
        &self,
        intern: &mut Intern,
        optim: &mut Optimizer,
        timeout: usize,
        depth: usize,
    ) -> Result<Vec<String>, String> {
        let (tx, rx) = mpsc::channel();
        let limit = Duration::from_millis(timeout as u64);
        if !limit.is_zero() {
            thread::spawn(move || {
                thread::sleep(limit);
                tx.send(true)
            });
        }

        let mut stdout = Vec::new();
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

        let mut global = Global {
            optim,
            stdout: &mut stdout,
            intern,
            timer: &rx,
        };
        let mut frame = Frame {
            depth: (0, depth),
            data: vec![base],
        };

        match self.eval(&mut global, &mut frame) {
            Ok(_) => Ok(stdout),
            Err(Signal::Error(e)) => Err(e.to_string()),
            _ => Err("invalid signal".to_string()),
        }
    }
}

impl Evaluation for Grammar {
    fn __eval(&self, global: &mut Global, frame: &mut Frame) -> Result<Value, Signal> {
        for stmt in &self.0 {
            stmt.eval(global, frame)?.void()?;
        }
        Ok(Value::Void)
    }
}
