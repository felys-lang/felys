use crate::ast::Grammar;
use crate::nn::optim::{Optimizer, Parameters};
use crate::parser::Intern;
use crate::runtime::context::backend::{Frame, Global};
use crate::runtime::context::value::Value;
use crate::runtime::shared::Evaluation;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct Program {
    grammar: Grammar,
    intern: Intern,
}

impl Program {
    pub fn new(grammar: Grammar, intern: Intern) -> Self {
        Self { grammar, intern }
    }

    pub fn config(self, params: Option<Parameters>, timeout: usize, depth: usize) -> Executable {
        let parameters = match params {
            Some(x) => x,
            None => todo!(),
        };
        let optimizer = Optimizer::new(parameters, 0.9);
        Executable {
            grammar: self.grammar,
            intern: self.intern,
            optimizer,
            timeout,
            depth,
        }
    }
}

pub struct Executable {
    grammar: Grammar,
    intern: Intern,
    optimizer: Optimizer,
    timeout: usize,
    depth: usize,
}

impl Executable {
    pub fn exec(mut self) -> Result<Output, String> {
        let (tx, rx) = mpsc::channel();
        let limit = Duration::from_millis(self.timeout as u64);
        if !limit.is_zero() {
            thread::spawn(move || {
                thread::sleep(limit);
                tx.send(true)
            });
        }

        let mut stdout = Vec::new();
        let base = HashMap::from([
            (
                self.intern.id("__elysia__"),
                Value::Str("粉色妖精小姐♪".to_string()),
            ),
            (
                self.intern.id("__author__"),
                Value::Str("jonny.jin@uwaterloo.ca".to_string()),
            ),
        ]);

        let mut global = Global {
            optim: &mut self.optimizer,
            stdout: &mut stdout,
            intern: &mut self.intern,
            timer: &rx,
        };
        let mut frame = Frame {
            depth: (0, self.depth),
            data: vec![base],
        };

        self.grammar.eval(&mut global, &mut frame).map_err(|e| e.error())?;
        Ok(Output {
            parameters: self.optimizer.export(),
            stdout
        })
    }
}

pub struct Output {
    parameters: Parameters,
    stdout: Vec<String>,
}
