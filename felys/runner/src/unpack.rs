use crate::environ::{Environ, Operator, Value};
use crate::execute::{Evaluation, Pairs, Signal, Unpack};
use ast::lit::Lit;
use ast::pat::{Ident, Pat};

impl Unpack for Pat {
    fn unpack(&self, env: &mut Environ, pairs: &mut Pairs, value: Value) -> Result<(), Signal> {
        match self {
            Pat::Any => Ok(()),
            Pat::Tuple(tup) => _tuple(env, pairs, value, tup),
            Pat::Lit(lit) => _lit(env, pairs, value, lit),
            Pat::Ident(ident) => _ident(env, pairs, value, ident),
        }
    }
}

fn _tuple(env: &mut Environ, pairs: &mut Pairs, value: Value, tup: &[Pat]) -> Result<(), Signal> {
    let Value::Tuple(inner) = value else {
        return Err(Signal::Error("only `tuple` can be unpacked"))
    };
    if tup.len() != inner.len() {
        return Err(Signal::Error("incorrect numbers of value to unpack"));
    }
    for (pat, val) in tup.iter().zip(inner) {
        pat.unpack(env, pairs, val)?;
    }
    Ok(())
}

fn _lit(env: &mut Environ, _: &mut Pairs, value: Value, lit: &Lit) -> Result<(), Signal> {
    if lit.eval(env)?.eq(value)?.bool()? {
        Ok(())
    } else {
        Err(Signal::Error("pattern not matched"))
    }
}

fn _ident(_: &mut Environ, pairs: &mut Pairs, value: Value, ident: &Ident) -> Result<(), Signal> {
    pairs.push((ident.clone(), value));
    Ok(())
}