pub use stmt::*;

use crate::env::{Environ, Object};
use crate::error::RuntimeError;
use crate::flow::run::*;

mod stmt;
mod run;

impl Statement {
    pub fn run(
        &self, env: &mut Environ, out: &mut Vec<String>,
    ) -> Result<Option<Object>, RuntimeError> {
        if env.timer.try_recv().unwrap_or(false) {
            return Err(RuntimeError::Timeout);
        }

        match self {
            Statement::Cond {
                expr,
                body,
                alter
            } => run_condition(env, out, expr, body, alter),
            Statement::Else {
                body
            } => body.run(env, out),
            Statement::While {
                expr,
                body
            } => run_while(env, out, expr, body),
            Statement::Return {
                expr
            } => run_return(env, out, expr),
            Statement::Simple {
                expr
            } => run_simple(env, out, expr)
        }
    }
}


impl Block {
    pub fn run(
        &self, env: &mut Environ, out: &mut Vec<String>,
    ) -> Result<Option<Object>, RuntimeError> {
        env.expand();
        for stmt in self.body.iter() {
            if let Some(value) = stmt.run(env, out)? {
                return Ok(Some(value));
            }
        }
        env.shrink();
        Ok(None)
    }
}
