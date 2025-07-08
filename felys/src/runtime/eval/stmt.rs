use crate::ast::{Block, Ident, Stmt};
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

impl Block {
    pub fn eval(
        &self,
        backend: &mut Backend,
        default: Vec<(Ident, Value)>,
    ) -> Result<Value, Signal> {
        fn __eval(x: &Block, backend: &mut Backend) -> Result<Value, Signal> {
            let length = x.0.len().saturating_sub(1);
            for stmt in x.0.iter().take(length) {
                stmt.eval(backend)?.void()?
            }
            match x.0.last() {
                Some(stmt) => stmt.eval(backend),
                None => Ok(Value::Void),
            }
        }
        if backend.timer.try_recv().unwrap_or(false) {
            return Err(Signal::Error("timeout".to_string()));
        }
        backend.stack(default);
        let result = __eval(self, backend);
        backend.unstack();
        result
    }
}
