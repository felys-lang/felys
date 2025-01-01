use crate::runner::environ::{Environ, Value};
use crate::runner::execute::{Evaluation, Signal};
use crate::ast::lit::{Bool, Float, Int, Lit, Str};

impl Evaluation for Lit {
    fn _eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        match self {
            Lit::Int(val) => _int(env, val),
            Lit::Float(val) => _float(env, val),
            Lit::Bool(val) => _bool(env, val),
            Lit::Str(val) => _str(env, val),
        }
    }
}

fn _int(env: &mut Environ, val: &Int) -> Result<Value, Signal> {
    let symbol = match val {
        Int::Base16(s) => s,
        Int::Base10(s) => s,
        Int::Base8(s) => s,
        Int::Base2(s) => s,
    };
    let raw = env.pool
        .get(symbol.into())
        .ok_or(Signal::Error("id does not exist"))?;
    let value = raw.parse()
        .map_err(|_| Signal::Error("parsing to `int` failed"))?;
    Ok(Value::Int(value))
}

fn _float(env: &mut Environ, val: &Float) -> Result<Value, Signal> {
    let raw = env.pool
        .get(val.into())
        .ok_or(Signal::Error("id does not exist"))?;
    let value = raw.parse()
        .map_err(|_| Signal::Error("parsing to `float` failed"))?;
    Ok(Value::Float(value))
}

fn _bool(_: &mut Environ, val: &Bool) -> Result<Value, Signal> {
    let value = match val {
        Bool::True => Value::Bool(true),
        Bool::False => Value::Bool(false)
    };
    Ok(value)
}

fn _str(env: &mut Environ, val: &Str) -> Result<Value, Signal> {
    let raw = env.pool
        .get(val.into())
        .ok_or(Signal::Error("id does not exist"))?;
    Ok(Value::Str(raw))
}