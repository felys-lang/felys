use crate::environ::{Environ, Value};
use crate::execute::{Evaluation, Signal};
use ast::pat::Pat;

impl Evaluation for Pat {
    fn eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        match self {
            Pat::Any => Err(Signal::Error("".to_string())),
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