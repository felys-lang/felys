use crate::ast::{BufVec, Ident, Lit, Pat};
use crate::runtime::context::backend::Backend;
use crate::runtime::context::value::Value;
use crate::runtime::shared::{Evaluation, Signal};

impl Pat {
    pub fn unpack(
        &self,
        backend: &mut Backend,
        value: Value,
    ) -> Result<Vec<(Ident, Value)>, Signal> {
        match self {
            Pat::Any => Ok(vec![]),
            Pat::Lit(lit) => __lit(backend, value, lit),
            Pat::Tuple(tuple) => __tuple(backend, value, tuple),
            Pat::Ident(ident) => Ok(vec![(*ident, value)]),
        }
    }
}

fn __tuple(
    backend: &mut Backend,
    value: Value,
    tuple: &BufVec<Pat, 2>,
) -> Result<Vec<(Ident, Value)>, Signal> {
    let values = value.tuple()?;
    if tuple.len() != values.len() {
        return Err(Signal::Error(
            "incorrect numbers of value to unpack".to_string(),
        ));
    }
    let mut result = Vec::new();
    for (pat, val) in tuple.iter().zip(values) {
        let mut more = pat.unpack(backend, val)?;
        result.append(&mut more);
    }
    Ok(result)
}

fn __lit(backend: &mut Backend, value: Value, lit: &Lit) -> Result<Vec<(Ident, Value)>, Signal> {
    if lit.eval(backend)?.eq(value)?.bool()? {
        Ok(vec![])
    } else {
        Err(Signal::Error("pattern not matched".to_string()))
    }
}
