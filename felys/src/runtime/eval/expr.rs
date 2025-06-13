use crate::ast::{AssOp, BinOp, Block, Expr, Pat, UnaOp};
use crate::runtime::context::backend::Backend;
use crate::runtime::context::value::Value;
use crate::runtime::shared::{Evaluation, Signal};
use std::rc::Rc;

impl Evaluation for Expr {
    fn __eval(&self, backend: &mut Backend) -> Result<Value, Signal> {
        match self {
            Expr::Assign(pat, op, expr) => __assign(backend, pat, op, expr),
            Expr::Block(block) => block.eval(backend, vec![]),
            Expr::Break(opt) => __break(backend, opt),
            Expr::Continue => Err(Signal::Continue),
            Expr::For(pat, expr, block) => __for(backend, pat, expr, block),
            Expr::If(expr, block, opt) => __if(backend, expr, block, opt),
            Expr::Loop(block) => __loop(backend, block),
            Expr::Return(opt) => __return(backend, opt),
            Expr::While(expr, block) => __while(backend, expr, block),
            Expr::Binary(rhs, op, lhs) => __binary(backend, rhs, op, lhs),
            Expr::Closure(params, expr) => Ok(Value::Closure(params.clone(), expr.clone())),
            Expr::Call(params, args) => __call(backend, params, args),
            Expr::Ident(ident) => backend.get(*ident),
            Expr::Tuple(tuple) => __tuple(backend, tuple),
            Expr::List(list) => __list(backend, list),
            Expr::Lit(lit) => lit.eval(backend),
            Expr::Paren(expr) => expr.eval(backend),
            Expr::Unary(op, expr) => __unary(backend, op, expr),
        }
    }
}

fn __assign(backend: &mut Backend, pat: &Pat, op: &AssOp, expr: &Expr) -> Result<Value, Signal> {
    let r = expr.eval(backend)?;
    let value = match op {
        AssOp::AddEq => pat.eval(backend)?.add(r)?,
        AssOp::SubEq => pat.eval(backend)?.sub(r)?,
        AssOp::MulEq => pat.eval(backend)?.mul(r)?,
        AssOp::DivEq => pat.eval(backend)?.div(r)?,
        AssOp::ModEq => pat.eval(backend)?.rem(r)?,
        AssOp::Eq => r,
    };
    for (id, val) in pat.unpack(backend, value)? {
        backend.put(id, val);
    }
    Ok(Value::Void)
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

fn __for(backend: &mut Backend, pat: &Pat, expr: &Expr, block: &Block) -> Result<Value, Signal> {
    for value in expr.eval(backend)?.list()? {
        let default = pat.unpack(backend, value)?;
        block.eval(backend, default)?.void()?;
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
        block.eval(backend, vec![])
    } else if let Some(alter) = opt {
        alter.eval(backend)
    } else {
        Ok(Value::Void)
    }
}

fn __loop(backend: &mut Backend, block: &Block) -> Result<Value, Signal> {
    loop {
        match block.eval(backend, vec![]) {
            Err(Signal::Continue) => continue,
            Err(Signal::Break(value)) => break Ok(value),
            other => {
                other?;
            }
        }
    }
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
        match block.eval(backend, vec![]) {
            Err(Signal::Continue) => continue,
            Err(Signal::Break(_)) => break,
            other => {
                other?;
            }
        }
    }
    Ok(Value::Void)
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

fn __unary(backend: &mut Backend, op: &UnaOp, expr: &Expr) -> Result<Value, Signal> {
    let e = expr.eval(backend)?;
    match op {
        UnaOp::Not => e.not(),
        UnaOp::Pos => e.pos(),
        UnaOp::Neg => e.neg(),
    }
}
