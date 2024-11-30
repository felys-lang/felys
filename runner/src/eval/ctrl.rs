use crate::environ::{Environ, Value};
use crate::execute::{Evaluation, Signal};
use ast::ctrl::Ctrl;
use ast::expr::Expr;

impl Evaluation for Ctrl {
    fn eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        match self {
            Ctrl::Assign(_, _, _) => todo!(),
            Ctrl::Block(block) => block.eval(env),
            Ctrl::Break(_) => todo!(),
            Ctrl::Continue => Err(Signal::Continue),
            Ctrl::For(_, _, _) => todo!(),
            Ctrl::Match(_, _) => todo!(),
            Ctrl::If(_, _, _) => todo!(),
            Ctrl::Loop(_) => todo!(),
            Ctrl::Return(expr) => _return(env, expr),
            Ctrl::While(_, _) => todo!(),
        }
    }
}

fn _return(env: &mut Environ, opt: &Option<Expr>) -> Result<Value, Signal> {
    let result = if let Some(expr) = opt {
        let value = expr.eval(env)?;
        Signal::Return(value)
    } else {
        Signal::Return(Value::Void)
    };
    Err(result)
}