use crate::ast::{BufVec, Pat};
use crate::runtime::context::backend::Backend;
use crate::runtime::context::value::Value;
use crate::runtime::shared::{Evaluation, Signal};

impl Evaluation for Pat {
    fn __eval(&self, backend: &mut Backend) -> Result<Value, Signal> {
        match self {
            Pat::Any => Err(Signal::Error("pattern `_` does not evaluated".to_string())),
            Pat::Lit(lit) => lit.eval(backend),
            Pat::Tuple(tuple) => __tuple(backend, tuple),
            Pat::Ident(ident) => backend.get(*ident),
        }
    }
}

fn __tuple(backend: &mut Backend, tuple: &BufVec<Pat, 2>) -> Result<Value, Signal> {
    let result = tuple
        .iter()
        .map(|x| x.eval(backend))
        .collect::<Result<Vec<Value>, Signal>>()?;
    Ok(Value::Tuple(result))
}
