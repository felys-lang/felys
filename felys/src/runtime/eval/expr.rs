use crate::ast::{BinOp, Block, Expr, UnaOp};
use crate::runtime::context::backend::Backend;
use crate::runtime::context::value::Value;
use crate::runtime::shared::{Evaluation, Signal};
use std::rc::Rc;

impl Evaluation for Expr {
    fn __eval(&self, backend: &mut Backend) -> Result<Value, Signal> {
        match self {
            Expr::Assign(pat, expr, opt) => todo!(),
            Expr::Block(x) => x.eval(backend),
            Expr::Break(x) => __break(backend, x),
            Expr::Continue => Err(Signal::Continue),
            Expr::For(pat, expr, block) => todo!(),
            Expr::If(expr, block, opt) => __if(backend, expr, block, opt),
            Expr::Loop(x) => __loop(backend, x),
            Expr::Return(x) => __return(backend, x),
            Expr::While(expr, block) => __while(backend, expr, block),
            Expr::Binary(rhs, op, lhs) => __binary(backend, rhs, op, lhs),
            Expr::Closure(params, expr) => Ok(Value::Closure(params.clone(), expr.clone())),
            Expr::Call(params, args) => __call(backend, params, args),
            Expr::Ident(ident) => backend.get(*ident),
            Expr::Tuple(tuple) => __tuple(backend, tuple),
            Expr::List(list) => __list(backend, list),
            Expr::Lit(lit) => lit.eval(backend),
            Expr::Paren(expr) => expr.eval(backend),
            Expr::Unary(op, x) => __unary(backend, op, x),
        }
    }
}

fn __break(backend: &mut Backend, opt: &Option<Rc<Expr>>) -> Result<Value, Signal> {
    let result = if let Some(expr) = opt {
        let value = expr.eval(backend)?;
        Signal::Break(value)
    } else {
        Signal::Break(Value::Void)
    };
    Err(result)
}

fn __return(backend: &mut Backend, opt: &Option<Rc<Expr>>) -> Result<Value, Signal> {
    let result = if let Some(expr) = opt {
        let value = expr.eval(backend)?;
        Signal::Return(value)
    } else {
        Signal::Return(Value::Void)
    };
    Err(result)
}

fn __while(backend: &mut Backend, expr: &Expr, block: &Block) -> Result<Value, Signal> {
    while expr.eval(backend)?.bool()? {
        match block.eval(backend) {
            Err(Signal::Continue) => continue,
            Err(Signal::Break(_)) => break,
            other => {
                other?;
            }
        }
    }
    Ok(Value::Void)
}

fn __if(
    backend: &mut Backend,
    expr: &Expr,
    block: &Block,
    opt: &Option<Rc<Expr>>,
) -> Result<Value, Signal> {
    if expr.eval(backend)?.bool()? {
        block.eval(backend)
    } else if let Some(alter) = opt {
        alter.eval(backend)
    } else {
        Ok(Value::Void)
    }
}

fn __loop(backend: &mut Backend, block: &Block) -> Result<Value, Signal> {
    loop {
        match block.eval(backend) {
            Err(Signal::Continue) => continue,
            Err(Signal::Break(value)) => break Ok(value),
            other => {
                other?;
            }
        }
    }
}

fn __call(backend: &mut Backend, func: &Expr, args: &[Expr]) -> Result<Value, Signal> {
    let values = args
        .iter()
        .map(|x| x.eval(backend))
        .collect::<Result<Vec<Value>, Signal>>()?;
    let (params, expr) = func.eval(backend)?.closure()?;
    if params.len() != values.len() {
        return Err(Signal::Error("incorrect numbers of arguments"));
    }
    let mut sandbox = backend.sandbox()?;
    for (param, value) in params.iter().zip(values) {
        sandbox.put(*param, value)
    }
    match expr.eval(&mut sandbox) {
        Err(Signal::Return(value)) => Ok(value),
        Err(Signal::Break(_)) | Err(Signal::Continue) => Err(Signal::Error("invalid signal")),
        other => other,
    }
}

fn __tuple(backend: &mut Backend, tuple: &[Expr]) -> Result<Value, Signal> {
    let result = tuple
        .iter()
        .map(|x| x.eval(backend))
        .collect::<Result<Vec<Value>, Signal>>()?;
    Ok(Value::Tuple(result))
}

fn __list(backend: &mut Backend, list: &[Expr]) -> Result<Value, Signal> {
    let result = list
        .iter()
        .map(|x| x.eval(backend))
        .collect::<Result<Vec<Value>, Signal>>()?;
    Ok(Value::List(result))
}

fn __binary(backend: &mut Backend, lhs: &Expr, op: &BinOp, rhs: &Expr) -> Result<Value, Signal> {
    let l = lhs.eval(backend)?;
    let r = rhs.eval(backend)?;
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

fn __unary(backend: &mut Backend, op: &UnaOp, rhs: &Expr) -> Result<Value, Signal> {
    let r = rhs.eval(backend)?;
    match op {
        UnaOp::Not => r.not(),
        UnaOp::Pos => r.pos(),
        UnaOp::Neg => r.neg(),
    }
}
