use crate::ast::Grammar;
use crate::nn::optim::{Optimizer, Parameters};
use crate::parser::Intern;

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
        let stdout = self.grammar.exec(
            &mut self.intern,
            &mut self.optimizer,
            self.timeout,
            self.depth,
        )?;
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
