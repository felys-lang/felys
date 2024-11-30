use crate::environ::{Environ, Value};
use crate::execute::{Evaluation, Signal};
use ast::lit::{Bool, Float, Int, Lit};

impl Evaluation for Lit {
    fn eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        match self {
            Lit::Int(val) => _int(env, val),
            Lit::Float(val) => _float(env, val),
            Lit::Bool(val) => _bool(env, val),
            Lit::Str(_) => todo!(),
        }
    }
}

fn _int(env: &mut Environ, val: &Int) -> Result<Value, Signal> {
    let symbol = match val {
        Int::Base16(_) => todo!(),
        Int::Base10(s) => s,
        Int::Base8(_) => todo!(),
        Int::Base2(_) => todo!(),
    };
    let raw = env.pool
        .get(symbol.0)
        .ok_or(Signal::Error("".to_string()))?;
    let value = raw.parse()
        .map_err(|_| Signal::Error("".to_string()))?;
    Ok(Value::Int(value))
}

fn _float(env: &mut Environ, val: &Float) -> Result<Value, Signal> {
    let raw = env.pool
        .get(val.0)
        .ok_or(Signal::Error("".to_string()))?;
    let value = raw.parse()
        .map_err(|_| Signal::Error("".to_string()))?;
    Ok(Value::Float(value))
}

fn _bool(_: &mut Environ, val: &Bool) -> Result<Value, Signal> {
    let value = match val {
        Bool::True => Value::Bool(true),
        Bool::False => Value::Bool(false)
    };
    Ok(value)
}