use crate::ast::{Block, Stmt};
use crate::runtime::context::backend::Backend;
use crate::runtime::context::value::Value;
use crate::runtime::shared::{Evaluation, Signal};

impl Evaluation for Stmt {
    fn __eval(&self, backend: &mut Backend) -> Result<Value, Signal> {
        match self {
            Stmt::Empty => Ok(Value::Void),
            Stmt::Expr(expr) => expr.eval(backend),
            Stmt::Semi(expr) => {
                expr.eval(backend)?;
                Ok(Value::Void)
            }
        }
    }
}

impl Evaluation for Block {
    fn __eval(&self, backend: &mut Backend) -> Result<Value, Signal> {
        backend.stack();
        let length = self.0.len().saturating_sub(1);
        for stmt in self.0.iter().take(length) {
            stmt.eval(backend)?.void()?
        }
        let result = match self.0.last() {
            Some(stmt) => stmt.eval(backend),
            None => Ok(Value::Void),
        };
        backend.unstack();
        result
    }
}
