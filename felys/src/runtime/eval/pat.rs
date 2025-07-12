use crate::ast::{BufVec, Pat};
use crate::runtime::context::backend::{Frame, Global};
use crate::runtime::context::value::Value;
use crate::runtime::shared::{Evaluation, Signal};

impl Evaluation for Pat {
    fn eval(&self, global: &mut Global, frame: &mut Frame) -> Result<Value, Signal> {
        match self {
            Pat::Any => Err(Signal::Error("pattern `_` does not evaluated".to_string())),
            Pat::Lit(lit) => lit.eval(global, frame),
            Pat::Tuple(tuple) => __tuple(global, frame, tuple),
            Pat::Ident(ident) => frame.get(*ident),
        }
    }
}

fn __tuple(
    global: &mut Global,
    frame: &mut Frame,
    tuple: &BufVec<Pat, 2>,
) -> Result<Value, Signal> {
    let result = tuple
        .iter()
        .map(|x| x.eval(global, frame))
        .collect::<Result<Vec<Value>, Signal>>()?;
    Ok(Value::Tuple(result))
}
