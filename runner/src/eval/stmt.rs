use crate::environ::{Environ, Value};
use crate::execute::{Evaluation, Signal};
use ast::stmt::{Block, Stmt};

impl Evaluation for Stmt {
    fn eval(&self, env: &mut Environ) -> Result<Value, Signal> {
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
    fn eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        for stmt in self.0.iter().take(self.0.len() - 1) {
            stmt.eval(env)?.void()?
        }
        if let Some(stmt) = self.0.last() {
            stmt.eval(env)
        } else {
            Ok(Value::Void)
        }
    }
}
