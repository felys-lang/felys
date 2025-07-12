use crate::ast::Grammar;
use crate::nn::optim::{Optimizer, Parameters};
use crate::parser::Intern;
use crate::runtime::context::backend::{Frame, Global};
use crate::runtime::context::value::Value;
use crate::runtime::shared::Evaluation;
use crate::rust::{ce, relu};
use std::collections::HashMap;

pub struct Config {
    params: Parameters,
    depth: usize,
    momentum: f64,
    seed: usize,
}

impl Config {
    pub fn new(params: Parameters, depth: usize, momentum: f64, seed: usize) -> Self {
        Self {
            params,
            depth,
            momentum,
            seed,
        }
    }
}

pub struct Program {
    grammar: Grammar,
    intern: Intern,
}

impl Program {
    pub fn new(grammar: Grammar, intern: Intern) -> Self {
        Self { grammar, intern }
    }

    pub fn config(self, config: Config) -> Executable {
        let optimizer = Optimizer::new(config.params, config.momentum, config.seed);
        Executable {
            grammar: self.grammar,
            intern: self.intern,
            optimizer,
            depth: config.depth,
        }
    }
}

pub struct Executable {
    grammar: Grammar,
    intern: Intern,
    optimizer: Optimizer,
    depth: usize,
}

impl Executable {
    pub fn exec(mut self) -> Result<Output, String> {
        let mut stdout = Vec::new();
        let constants = HashMap::from([
            (
                self.intern.id("__elysia__"),
                Value::Str("粉色妖精小姐♪".to_string()),
            ),
            (
                self.intern.id("__author__"),
                Value::Str("jonny.jin@uwaterloo.ca".to_string()),
            ),
            (self.intern.id("ReLU"), Value::Rust(relu)),
            (self.intern.id("CrossEntropy"), Value::Rust(ce)),
        ]);

        let mut global = Global {
            optim: &mut self.optimizer,
            stdout: &mut stdout,
            constants: &constants,
            intern: &mut self.intern,
        };
        let mut frame = Frame {
            depth: (0, self.depth),
            data: vec![HashMap::new()],
        };

        self.grammar
            .eval(&mut global, &mut frame)
            .map_err(|e| e.error())?;
        Ok(Output {
            parameters: self.optimizer.export(),
            stdout,
        })
    }
}

pub struct Output {
    pub parameters: Parameters,
    pub stdout: Vec<String>,
}
