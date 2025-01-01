use crate::runner::environ::{Environ, Value};
use crate::runner::execute::{Evaluation, Signal};
use crate::ast::pat::Pat;

impl Evaluation for Pat {
    fn _eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        match self {
            Pat::Any => Err(Signal::Error("pattern `_` does not evaluated")),
            Pat::Tuple(tup) => _tuple(env, tup),
            Pat::Lit(lit) => lit.eval(env),
            Pat::Ident(ident) => env.warehouse.get(ident.into())
        }
    }
}

fn _tuple(env: &mut Environ, tup: &[Pat]) -> Result<Value, Signal> {
    let mut result = Vec::with_capacity(tup.len());
    for pat in tup {
        let value = pat.eval(env)?;
        result.push(value)
    }
    Ok(Value::Tuple(result))
}