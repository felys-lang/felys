use crate::ast::expr::{AssOp, BinOp, Expr, UnaOp};
use crate::ast::pat::{Ident, Pat};
use crate::ast::stmt::Block;
use crate::runner::environ::{Environ, Operator, Value};
use crate::runner::execute::{Evaluation, Signal, Unpack};
use std::rc::Rc;

impl Evaluation for Expr {
    fn _eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        match self {
            Expr::Assign(pat, op, expr) => _assign(env, pat, op, expr),
            Expr::Block(block) => block.eval(env),
            Expr::Break(expr) => _break(env, expr),
            Expr::Continue => Err(Signal::Continue),
            Expr::For(_, _, _) => Err(Signal::Error("nice try, but parsed != supported")),
            Expr::Match(_, _) => Err(Signal::Error("nice try, but parsed != supported")),
            Expr::If(expr, block, opt) => _if(env, expr, block, opt),
            Expr::Loop(block) => _loop(env, block),
            Expr::Return(expr) => _return(env, expr),
            Expr::While(expr, block) => _while(env, expr, block),
            Expr::Binary(lhs, op, rhs) => _binary(env, lhs, op, rhs),
            Expr::Call(func, args) => _call(env, func, args),
            Expr::Field(_, _) => Err(Signal::Error("nice try, but parsed != supported")),
            Expr::Func(params, expr) => _func(env, params, expr),
            Expr::Ident(ident) => env.warehouse.get(ident.into()),
            Expr::Tuple(tup) => _tuple(env, tup),
            Expr::Lit(lit) => lit.eval(env),
            Expr::Paren(expr) => expr.eval(env),
            Expr::Unary(op, rhs) => _unary(env, op, rhs),
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

fn _break(env: &mut Environ, opt: &Option<Rc<Expr>>) -> Result<Value, Signal> {
    let result = if let Some(expr) = opt {
        let value = expr.eval(env)?;
        Signal::Break(value)
    } else {
        Signal::Break(Value::Void)
    };
    Err(result)
}

fn _if(env: &mut Environ, expr: &Expr, block: &Block, opt: &Option<Rc<Expr>>) -> Result<Value, Signal> {
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

fn _return(env: &mut Environ, opt: &Option<Rc<Expr>>) -> Result<Value, Signal> {
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

fn _call(env: &mut Environ, func: &Expr, args: &[Expr]) -> Result<Value, Signal> {
    let mut values = Vec::with_capacity(args.len());
    for expr in args {
        let value = expr.eval(env)?;
        values.push(value)
    }
    let (params, expr) = func.eval(env)?.func()?;
    if params.len() != values.len() {
        return Err(Signal::Error("incorrect numbers of arguments"));
    }
    let mut sandbox = env.sandbox()?;
    for (param, value) in params.iter().zip(values) {
        sandbox.warehouse.put(param.into(), value)
    }
    let result = expr.eval(&mut sandbox);
    match result {
        Err(Signal::Return(value)) => Ok(value),
        Err(Signal::Break(_)) |
        Err(Signal::Continue) => Err(Signal::Error("invalid signal")),
        _ => result
    }
}

fn _func(_: &mut Environ, params: &[Ident], expr: &Expr) -> Result<Value, Signal> {
    Ok(Value::Func(Vec::from(params), expr.clone()))
}

fn _tuple(env: &mut Environ, tup: &[Expr]) -> Result<Value, Signal> {
    let mut result = Vec::with_capacity(tup.len());
    for expr in tup {
        let value = expr.eval(env)?;
        result.push(value)
    }
    Ok(Value::Tuple(result))
}

fn _binary(env: &mut Environ, lhs: &Expr, op: &BinOp, rhs: &Expr) -> Result<Value, Signal> {
    let l = lhs.eval(env)?;
    let r = rhs.eval(env)?;
    match op {
        BinOp::Or => l.or(r),
        BinOp::And => l.and(r),
        BinOp::Gt => l.gt(r),
        BinOp::Ge => l.ge(r),
        BinOp::Lt => l.lt(r),
        BinOp::Le => l.le(r),
        BinOp::Eq => l.eq(r),
        BinOp::Ne => l.ne(r),
        BinOp::Add => l.add(r),
        BinOp::Sub => l.sub(r),
        BinOp::Mul => l.mul(r),
        BinOp::Div => l.div(r),
        BinOp::Mod => l.rem(r),
    }
}

fn _unary(env: &mut Environ, op: &UnaOp, rhs: &Expr) -> Result<Value, Signal> {
    let r = rhs.eval(env)?;
    match op {
        UnaOp::Not => r.not(),
        UnaOp::Pos => r.pos(),
        UnaOp::Neg => r.neg()
    }
}