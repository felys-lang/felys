use crate::environ::{Environ, Operator, Value};
use crate::execute::{Evaluation, Signal};
use ast::expr::{BinOp, Expr, UnaOp};

impl Evaluation for Expr {
    fn eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        match self {
            Expr::Binary(lhs, op, rhs) => _binary(env, lhs, op, rhs),
            Expr::Closure(params, expr) => todo!(),
            Expr::Call(callee, args) => todo!(),
            Expr::Field(_, _) => unimplemented!("feature not supported, though it gets parsed"),
            Expr::Ident(ident) => env.warehouse.get(ident.0),
            Expr::Tuple(tup) => _tuple(env, tup),
            Expr::Lit(lit) => lit.eval(env),
            Expr::Paren(expr) => expr.eval(env),
            Expr::Ctrl(ctrl) => ctrl.eval(env),
            Expr::Unary(op, rhs) => _unary(env, op, rhs),
        }
    }
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