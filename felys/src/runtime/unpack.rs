use crate::ast::{BufVec, Id, Lit, Pat};
use crate::runtime::context::backend::{Frame, Global};
use crate::runtime::context::value::Value;
use crate::runtime::shared::{Evaluation, Signal};

impl Pat {
    pub fn unpack(
        &self,
        global: &mut Global,
        frame: &mut Frame,
        value: Value,
    ) -> Result<Vec<(Id, Value)>, Signal> {
        match self {
            Pat::Any => Ok(vec![]),
            Pat::Lit(lit) => __lit(global, frame, value, lit),
            Pat::Tuple(tuple) => __tuple(global, frame, value, tuple),
            Pat::Ident(ident) => Ok(vec![(ident.0, value)]),
        }
    }
}

fn __tuple(
    global: &mut Global,
    frame: &mut Frame,
    value: Value,
    tuple: &BufVec<Pat, 2>,
) -> Result<Vec<(Id, Value)>, Signal> {
    let values = value.tuple()?;
    if tuple.len() != values.len() {
        return Err(Signal::Error(format!(
            "expect {} elements, saw {}",
            tuple.len(),
            values.len()
        )));
    }
    let mut result = Vec::new();
    for (pat, val) in tuple.iter().zip(values) {
        let mut more = pat.unpack(global, frame, val)?;
        result.append(&mut more);
    }
    Ok(result)
}

fn __lit(
    global: &mut Global,
    frame: &mut Frame,
    value: Value,
    lit: &Lit,
) -> Result<Vec<(Id, Value)>, Signal> {
    if lit.eval(global, frame)?.eq(value)?.bool()? {
        Ok(vec![])
    } else {
        Err(Signal::Error("pattern not matched".to_string()))
    }
}
