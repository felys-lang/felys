use crate::ast::{AssOp, BinOp, Block, BufVec, Expr, Float, Ident, Int, Pat, UnaOp};
use crate::nn::layers::{Layer, Operator};
use crate::nn::matrix::Matrix;
use crate::runtime::context::backend::{Frame, Global};
use crate::runtime::context::value::Value;
use crate::runtime::shared::{Evaluation, Signal};
use std::rc::Rc;

impl Evaluation for Expr {
    fn eval(&self, global: &mut Global, frame: &mut Frame) -> Result<Value, Signal> {
        match self {
            Expr::Assign(pat, op, expr) => __assign(global, frame, pat, op, expr),
            Expr::Block(block) => block.eval(global, frame, vec![]),
            Expr::Break(option) => __break(global, frame, option),
            Expr::Continue => Err(Signal::Continue),
            Expr::Rust(ident) => ident.constant(global, frame),
            Expr::For(pat, expr, block) => __for(global, frame, pat, expr, block),
            Expr::If(expr, block, option) => __if(global, frame, expr, block, option),
            Expr::Loop(block) => __loop(global, frame, block),
            Expr::Matrix(matrix) => __matrix(global, frame, matrix),
            Expr::Return(option) => __return(global, frame, option),
            Expr::While(expr, block) => __while(global, frame, expr, block),
            Expr::Binary(rhs, op, lhs) => __binary(global, frame, rhs, op, lhs),
            Expr::Closure(params, expr) => __closure(global, frame, params, expr),
            Expr::Call(params, args) => __call(global, frame, params, args),
            Expr::Ident(ident) => ident.local(global, frame),
            Expr::Step(loss, lr) => __step(global, frame, loss, lr),
            Expr::Tuple(tuple) => __tuple(global, frame, tuple),
            Expr::List(list) => __list(global, frame, list),
            Expr::Lit(lit) => lit.eval(global, frame),
            Expr::Param(rows, cols, id) => __parameter(global, frame, rows, cols, id),
            Expr::Paren(expr) => expr.eval(global, frame),
            Expr::Print(expr) => __print(global, frame, expr),
            Expr::Unary(op, expr) => __unary(global, frame, op, expr),
        }
    }
}

fn __assign(
    global: &mut Global,
    frame: &mut Frame,
    pat: &Pat,
    op: &AssOp,
    expr: &Expr,
) -> Result<Value, Signal> {
    let y = expr.eval(global, frame)?;
    let value = match op {
        AssOp::AddEq => pat.eval(global, frame)?.add(y)?,
        AssOp::SubEq => pat.eval(global, frame)?.sub(y)?,
        AssOp::MulEq => pat.eval(global, frame)?.mul(y)?,
        AssOp::DivEq => pat.eval(global, frame)?.div(y)?,
        AssOp::ModEq => pat.eval(global, frame)?.rem(y)?,
        AssOp::Eq => y,
    };
    for (id, val) in pat.unpack(global, frame, value)? {
        frame.put(id, val);
    }
    Ok(Value::Void)
}

fn __break(
    global: &mut Global,
    frame: &mut Frame,
    option: &Option<Rc<Expr>>,
) -> Result<Value, Signal> {
    let result = if let Some(expr) = option {
        let value = expr.eval(global, frame)?;
        Signal::Break(value)
    } else {
        Signal::Break(Value::Void)
    };
    Err(result)
}

fn __for(
    global: &mut Global,
    frame: &mut Frame,
    pat: &Pat,
    expr: &Expr,
    block: &Block,
) -> Result<Value, Signal> {
    for value in expr.eval(global, frame)?.list()? {
        let default = pat.unpack(global, frame, value)?;
        match block.eval(global, frame, default) {
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
    global: &mut Global,
    frame: &mut Frame,
    expr: &Expr,
    block: &Block,
    option: &Option<Rc<Expr>>,
) -> Result<Value, Signal> {
    if expr.eval(global, frame)?.bool()? {
        block.eval(global, frame, vec![])
    } else if let Some(alter) = option {
        alter.eval(global, frame)
    } else {
        Ok(Value::Void)
    }
}

fn __loop(global: &mut Global, frame: &mut Frame, block: &Block) -> Result<Value, Signal> {
    loop {
        match block.eval(global, frame, vec![]) {
            Err(Signal::Continue) => continue,
            Err(Signal::Break(value)) => break Ok(value),
            other => {
                other?;
            }
        }
    }
}

fn __return(
    global: &mut Global,
    frame: &mut Frame,
    option: &Option<Rc<Expr>>,
) -> Result<Value, Signal> {
    let result = if let Some(expr) = option {
        let value = expr.eval(global, frame)?;
        Signal::Return(value)
    } else {
        Signal::Return(Value::Void)
    };
    Err(result)
}

fn __while(
    global: &mut Global,
    frame: &mut Frame,
    expr: &Expr,
    block: &Block,
) -> Result<Value, Signal> {
    while expr.eval(global, frame)?.bool()? {
        match block.eval(global, frame, vec![]) {
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
    global: &mut Global,
    frame: &mut Frame,
    func: &Expr,
    args: &Option<BufVec<Expr, 1>>,
) -> Result<Value, Signal> {
    let values = match args {
        Some(x) => x
            .iter()
            .map(|x| x.eval(global, frame))
            .collect::<Result<Vec<Value>, Signal>>()?,
        None => vec![],
    };

    let result = match func.eval(global, frame)? {
        Value::Closure(params, expr) => {
            if params.len() != values.len() {
                return Err(Signal::Error(format!(
                    "expected {} arguments, saw {}",
                    params.len(),
                    values.len()
                )));
            }
            let mut sandbox = frame.sandbox()?;
            for (param, value) in params.iter().zip(values) {
                sandbox.put(param.0, value)
            }
            expr.eval(global, &mut sandbox)
        }
        Value::Rust(f) => f(values),
        value => return Err(Signal::Error(format!("{value} is not callable"))),
    };

    match result {
        Err(Signal::Return(value)) => Ok(value),
        Err(Signal::Break(_)) | Err(Signal::Continue) => {
            Err(Signal::Error("invalid signal".to_string()))
        }
        other => other,
    }
}

fn __closure(
    _: &mut Global,
    _: &mut Frame,
    params: &Option<BufVec<Ident, 1>>,
    expr: &Rc<Expr>,
) -> Result<Value, Signal> {
    let params = params.as_ref().map(|x| x.vec()).unwrap_or_default();
    Ok(Value::Closure(params, expr.clone()))
}

fn __step(
    global: &mut Global,
    frame: &mut Frame,
    loss: &Rc<Expr>,
    lr: &Rc<Expr>,
) -> Result<Value, Signal> {
    let grads = loss
        .eval(global, frame)?
        .operator()?
        .backward()
        .map_err(Signal::Error)?;
    let lr = lr.eval(global, frame)?.float()?;
    global.optim.step(grads, lr).map_err(Signal::Error)?;
    Ok(Value::Void)
}

fn __tuple(
    global: &mut Global,
    frame: &mut Frame,
    tuple: &BufVec<Expr, 2>,
) -> Result<Value, Signal> {
    let result = tuple
        .iter()
        .map(|x| x.eval(global, frame))
        .collect::<Result<Vec<Value>, Signal>>()?;
    Ok(Value::Tuple(result))
}

fn __list(
    global: &mut Global,
    frame: &mut Frame,
    list: &Option<BufVec<Expr, 1>>,
) -> Result<Value, Signal> {
    let result = match list {
        Some(x) => x
            .iter()
            .map(|x| x.eval(global, frame))
            .collect::<Result<Vec<Value>, Signal>>()?,
        None => vec![],
    };
    Ok(Value::List(result))
}

fn __matrix(
    global: &mut Global,
    frame: &mut Frame,
    matrix: &BufVec<BufVec<Float, 1>, 1>,
) -> Result<Value, Signal> {
    let rows = matrix.len();
    let cols = matrix.buffer()[0].len();
    let mut data = Vec::with_capacity(rows * cols);
    for row in matrix.iter() {
        if row.len() != cols {
            return Err(Signal::Error(format!(
                "expected {cols} columns, saw {}",
                row.len()
            )));
        }
        for x in row.iter() {
            let value = x.eval(global, frame)?.float()?;
            data.push(value);
        }
    }
    let mat = Matrix::new(data, (rows, cols)).map_err(Signal::Error)?;
    let op = Operator::new(mat, Layer::Fixed);
    Ok(Value::Operator(op))
}

fn __binary(
    global: &mut Global,
    frame: &mut Frame,
    lhs: &Expr,
    op: &BinOp,
    rhs: &Expr,
) -> Result<Value, Signal> {
    let x = lhs.eval(global, frame)?;
    let y = rhs.eval(global, frame)?;
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

fn __parameter(
    global: &mut Global,
    frame: &mut Frame,
    rows: &Int,
    cols: &Int,
    id: &usize,
) -> Result<Value, Signal> {
    let rows = rows.eval(global, frame)?.int()? as usize;
    let cols = cols.eval(global, frame)?.int()? as usize;
    let shape = (rows, cols);
    let matrix = global.optim.get(id, shape).map_err(Signal::Error)?;
    let op = Operator::new(matrix, Layer::Learnable(*id));
    Ok(Value::Operator(op))
}

fn __print(global: &mut Global, frame: &mut Frame, expr: &Rc<Expr>) -> Result<Value, Signal> {
    let string = expr.eval(global, frame)?.to_string();
    global.stdout.push(string);
    Ok(Value::Void)
}

fn __unary(
    global: &mut Global,
    frame: &mut Frame,
    op: &UnaOp,
    expr: &Expr,
) -> Result<Value, Signal> {
    let x = expr.eval(global, frame)?;
    match op {
        UnaOp::Not => x.not(),
        UnaOp::Pos => x.pos(),
        UnaOp::Neg => x.neg(),
    }
}
