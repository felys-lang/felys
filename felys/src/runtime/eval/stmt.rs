use crate::ast::{Block, Ident, Stmt};
use crate::runtime::context::backend::{Frame, Global};
use crate::runtime::context::value::Value;
use crate::runtime::shared::{Evaluation, Signal};

impl Evaluation for Stmt {
    fn __eval(&self, global: &mut Global, frame: &mut Frame) -> Result<Value, Signal> {
        match self {
            Stmt::Empty => Ok(Value::Void),
            Stmt::Expr(expr) => expr.eval(global, frame),
            Stmt::Semi(expr) => {
                expr.eval(global, frame)?;
                Ok(Value::Void)
            }
        }
    }
}

impl Block {
    pub fn eval(
        &self,
        global: &mut Global,
        frame: &mut Frame,
        default: Vec<(Ident, Value)>,
    ) -> Result<Value, Signal> {
        if global.timer.try_recv().unwrap_or(false) {
            return Err(Signal::Error("timeout".to_string()));
        }
        frame.stack(default);
        let result = __eval(self, global, frame);
        frame.unstack();
        result
    }
}

fn __eval(x: &Block, global: &mut Global, frame: &mut Frame) -> Result<Value, Signal> {
    let length = x.0.len().saturating_sub(1);
    for stmt in x.0.iter().take(length) {
        stmt.eval(global, frame)?.void()?
    }
    match x.0.last() {
        Some(stmt) => stmt.eval(global, frame),
        None => Ok(Value::Void),
    }
}
