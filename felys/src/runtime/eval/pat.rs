use crate::ast::{BufVec, Ident, Pat};
use crate::runtime::context::backend::{Frame, Global};
use crate::runtime::context::value::Value;
use crate::runtime::shared::{Evaluation, Signal};

impl Evaluation for Pat {
    fn eval(&self, global: &mut Global, frame: &mut Frame) -> Result<Value, Signal> {
        match self {
            Pat::Any => Err(Signal::Error("pattern `_` does not evaluated".to_string())),
            Pat::Lit(lit) => lit.eval(global, frame),
            Pat::Tuple(tuple) => __tuple(global, frame, tuple),
            Pat::Ident(ident) => ident.local(global, frame),
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

impl Ident {
    pub fn constant(&self, global: &mut Global, _: &mut Frame) -> Result<Value, Signal> {
        if let Some(value) = global.constants.get(&self.0).cloned() {
            Ok(value)
        } else if let Some(name) = global.intern.get(&self.0) {
            Err(Signal::Error(format!("`{name}` is not a rust identifier")))
        } else {
            Err(Signal::Error(format!("`id {}` not found", self.0)))
        }
    }
    pub fn local(&self, global: &mut Global, frame: &mut Frame) -> Result<Value, Signal> {
        if let Some(value) = frame.get(&self.0) {
            Ok(value)
        } else if let Some(name) = global.intern.get(&self.0) {
            Err(Signal::Error(format!("`{name}` is not declared")))
        } else {
            Err(Signal::Error(format!("`id {}` not found", self.0)))
        }
    }
}
