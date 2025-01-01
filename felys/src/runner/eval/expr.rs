use crate::runner::environ::{Environ, Operator, Value};
use crate::runner::execute::{Evaluation, Signal};
use crate::ast::expr::{BinOp, Expr, UnaOp};
use crate::ast::pat::Ident;

impl Evaluation for Expr {
    fn _eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        match self {
            Expr::Binary(lhs, op, rhs) => _binary(env, lhs, op, rhs),
            Expr::Call(func, args) => _call(env, func, args),
            Expr::Field(_, _) => Err(Signal::Error("nice try, but parsed != supported")),
            Expr::Func(params, expr) => _func(env, params, expr),
            Expr::Ident(ident) => env.warehouse.get(ident.into()),
            Expr::Tuple(tup) => _tuple(env, tup),
            Expr::Lit(lit) => lit.eval(env),
            Expr::Paren(expr) => expr.eval(env),
            Expr::Ctrl(ctrl) => ctrl.eval(env),
            Expr::Unary(op, rhs) => _unary(env, op, rhs),
        }
    }
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