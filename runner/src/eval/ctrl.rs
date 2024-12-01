use crate::environ::{Environ, Operator, Value};
use crate::execute::{Evaluation, Signal, Unpack};
use ast::ctrl::{AssOp, Ctrl};
use ast::expr::Expr;
use ast::pat::Pat;

impl Evaluation for Ctrl {
    fn eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        match self {
            Ctrl::Assign(pat, op, expr) => _assign(env, pat, op, expr),
            Ctrl::Block(block) => block.eval(env),
            Ctrl::Break(expr) => _break(env, expr),
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

fn _assign(env: &mut Environ, pat: &Pat, op: &AssOp, expr: &Expr) -> Result<Value, Signal> {
    let rhs = expr.eval(env)?;
    let value = match op {
        AssOp::AddEq => pat.eval(env)?.add(rhs)?,
        AssOp::SubEq => pat.eval(env)?.sub(rhs)?,
        AssOp::MulEq => pat.eval(env)?.mul(rhs)?,
        AssOp::DivEq => pat.eval(env)?.div(rhs)?,
        AssOp::ModEq => pat.eval(env)?.rem(rhs)?,
        AssOp::Eq => rhs
    };
    let mut pairs = Vec::new();
    pat.unpack(env, &mut pairs, value)?;
    for (ident, val) in pairs {
        env.warehouse.put(ident.into(), val);
    }
    Ok(Value::Void)
}

fn _break(env: &mut Environ, opt: &Option<Expr>) -> Result<Value, Signal> {
    let result = if let Some(expr) = opt {
        let value = expr.eval(env)?;
        Signal::Break(value)
    } else {
        Signal::Break(Value::Void)
    };
    Err(result)
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