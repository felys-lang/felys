use crate::environ::{Environ, Value};
use crate::execute::{Evaluation, Signal};
use ast::expr::{BinOp, Expr};

impl Evaluation for Expr {
    fn eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        match self {
            Expr::Binary(lhs, op, rhs) => binary(env, lhs, op, rhs),
            Expr::Closure(params, expr) => todo!(),
            Expr::Call(callee, args) => todo!(),
            Expr::Field(root, field) => todo!(),
            Expr::Ident(ident) => todo!(),
            Expr::Tuple(tuple) => todo!(),
            Expr::Lit(lit) => lit.eval(env),
            Expr::Paren(expr) => expr.eval(env),
            Expr::Ctrl(ctrl) => ctrl.eval(env),
            Expr::Unary(op, rhs) => todo!(),
        }
    }
}

fn binary(env: &mut Environ, lhs: &Box<Expr>, op: &BinOp, rhs: &Box<Expr>) -> Result<Value, Signal> {
    todo!()
}
