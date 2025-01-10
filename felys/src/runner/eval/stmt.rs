use crate::ast::stmt::{Block, Stmt};
use crate::runner::environ::{Environ, Value};
use crate::runner::execute::{Evaluation, Signal};

impl Evaluation for Stmt {
    fn _eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        match self {
            Stmt::Empty => Ok(Value::Void),
            Stmt::Expr(expr) => expr.eval(env),
            Stmt::Semi(expr) => {
                expr.eval(env)?;
                Ok(Value::Void)
            }
        }
    }
}

impl Evaluation for Block {
    fn _eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        env.warehouse.stack();
        for stmt in self.0.iter().take(self.0.len().saturating_sub(1)) {
            stmt.eval(env)?.void()?
        }
        let result = match self.0.last() {
            Some(stmt) => stmt.eval(env),
            None => Ok(Value::Void)
        };
        env.warehouse.unstack();
        result
    }
}
