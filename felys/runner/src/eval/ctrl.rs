use crate::environ::{Environ, Operator, Value};
use crate::execute::{Evaluation, Signal, Unpack};
use ast::ctrl::{AssOp, Ctrl};
use ast::expr::Expr;
use ast::pat::Pat;
use ast::stmt::Block;

impl Evaluation for Ctrl {
    fn _eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        match self {
            Ctrl::Assign(pat, op, expr) => _assign(env, pat, op, expr),
            Ctrl::Block(block) => block.eval(env),
            Ctrl::Break(expr) => _break(env, expr),
            Ctrl::Continue => Err(Signal::Continue),
            Ctrl::For(_, _, _) => Err(Signal::Error("nice try, but parsed != supported")),
            Ctrl::Match(_, _) => Err(Signal::Error("nice try, but parsed != supported")),
            Ctrl::If(expr, block, opt) => _if(env, expr, block, opt),
            Ctrl::Loop(block) => _loop(env, block),
            Ctrl::Return(expr) => _return(env, expr),
            Ctrl::While(expr, block) => _while(env, expr, block),
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

fn _if(env: &mut Environ, expr: &Expr, block: &Block, opt: &Option<Expr>) -> Result<Value, Signal> {
    if expr.eval(env)?.bool()? {
        block.eval(env)
    } else if let Some(alter) = opt {
        alter.eval(env)
    } else {
        Ok(Value::Void)
    }
}

fn _loop(env: &mut Environ, block: &Block) -> Result<Value, Signal> {
    loop {
        match block.eval(env) {
            Err(Signal::Continue) => continue,
            Err(Signal::Break(value)) => break Ok(value),
            other => { other?; }
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

fn _while(env: &mut Environ, expr: &Expr, block: &Block) -> Result<Value, Signal> {
    while expr.eval(env)?.bool()? {
        match block.eval(env) {
            Err(Signal::Continue) => continue,
            Err(Signal::Break(_)) => break,
            other => { other?; }
        }
    }
    Ok(Value::Void)
}