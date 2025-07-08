use crate::ast::{AssOp, BinOp, Block, BufVec, Expr, Float, Ident, Pat, UnaOp};
use crate::nn::layers::{Layer, Operator};
use crate::nn::matrix::Matrix;
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
            Expr::Matrix(matrix) => __matrix(backend, matrix),
            Expr::Return(opt) => __return(backend, opt),
            Expr::While(expr, block) => __while(backend, expr, block),
            Expr::Binary(rhs, op, lhs) => __binary(backend, rhs, op, lhs),
            Expr::Closure(params, expr) => __closure(backend, params, expr),
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
    let y = expr.eval(backend)?;
    let value = match op {
        AssOp::AddEq => pat.eval(backend)?.add(y)?,
        AssOp::SubEq => pat.eval(backend)?.sub(y)?,
        AssOp::MulEq => pat.eval(backend)?.mul(y)?,
        AssOp::DivEq => pat.eval(backend)?.div(y)?,
        AssOp::ModEq => pat.eval(backend)?.rem(y)?,
        AssOp::Eq => y,
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
        match block.eval(backend, default) {
            Err(Signal::Continue) => continue,
            Err(Signal::Break(_)) => break,
            other => {
                other?.void()?;
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
                other?.void()?;
            }
        }
    }
    Ok(Value::Void)
}

fn __call(
    backend: &mut Backend,
    func: &Expr,
    args: &Option<BufVec<Expr, 1>>,
) -> Result<Value, Signal> {
    let values = match args {
        Some(x) => x
            .iter()
            .map(|x| x.eval(backend))
            .collect::<Result<Vec<Value>, Signal>>()?,
        None => vec![],
    };
    let (params, expr) = func.eval(backend)?.closure()?;
    if params.len() != values.len() {
        return Err(Signal::Error("incorrect numbers of arguments".to_string()));
    }
    let mut sandbox = backend.sandbox()?;
    for (param, value) in params.iter().zip(values) {
        sandbox.put(*param, value)
    }
    match expr.eval(&mut sandbox) {
        Err(Signal::Return(value)) => Ok(value),
        Err(Signal::Break(_)) | Err(Signal::Continue) => {
            Err(Signal::Error("invalid signal".to_string()))
        }
        other => other,
    }
}

fn __closure(
    _: &mut Backend,
    params: &Option<BufVec<Ident, 1>>,
    expr: &Rc<Expr>,
) -> Result<Value, Signal> {
    let params = params.as_ref().map(|x| x.vec()).unwrap_or_default();
    Ok(Value::Closure(params, expr.clone()))
}

fn __tuple(backend: &mut Backend, tuple: &BufVec<Expr, 2>) -> Result<Value, Signal> {
    let result = tuple
        .iter()
        .map(|x| x.eval(backend))
        .collect::<Result<Vec<Value>, Signal>>()?;
    Ok(Value::Tuple(result))
}

fn __list(backend: &mut Backend, list: &Option<BufVec<Expr, 1>>) -> Result<Value, Signal> {
    let result = match list {
        Some(x) => x
            .iter()
            .map(|x| x.eval(backend))
            .collect::<Result<Vec<Value>, Signal>>()?,
        None => vec![],
    };
    Ok(Value::List(result))
}

fn __matrix(backend: &mut Backend, matrix: &BufVec<BufVec<Float, 1>, 1>) -> Result<Value, Signal> {
    let shape = (matrix.len(), matrix.buffer()[0].len());
    let length = shape.0 * shape.1;
    let mut data = Vec::with_capacity(length);
    for row in matrix.iter() {
        for x in row.iter() {
            let raw = backend
                .intern
                .get(x)
                .ok_or(Signal::Error("id does not exist".to_string()))?;
            let value = raw
                .parse()
                .map_err(|_| Signal::Error("parsing to `float` failed".to_string()))?;
            data.push(value);
        }
    }
    let mat = Matrix::new(data, shape).map_err(Signal::Error)?;
    let op = Operator::new(mat, Layer::Fixed);
    Ok(Value::Operator(op))
}

fn __binary(backend: &mut Backend, lhs: &Expr, op: &BinOp, rhs: &Expr) -> Result<Value, Signal> {
    let x = lhs.eval(backend)?;
    let y = rhs.eval(backend)?;
    match op {
        BinOp::Or => x.or(y),
        BinOp::And => x.and(y),
        BinOp::Gt => x.gt(y),
        BinOp::Ge => x.ge(y),
        BinOp::Lt => x.lt(y),
        BinOp::Le => x.le(y),
        BinOp::Eq => x.eq(y),
        BinOp::Ne => x.ne(y),
        BinOp::Add => x.add(y),
        BinOp::Sub => x.sub(y),
        BinOp::Mul => x.mul(y),
        BinOp::Div => x.div(y),
        BinOp::Mod => x.rem(y),
        BinOp::Dot => x.dot(y),
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
