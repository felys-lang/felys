use crate::ast::{Bool, Chunk, Float, Int, Lit, Str};
use crate::runtime::context::backend::Backend;
use crate::runtime::context::value::Value;
use crate::runtime::shared::{Evaluation, Signal};

impl Evaluation for Lit {
    fn __eval(&self, backend: &mut Backend) -> Result<Value, Signal> {
        match self {
            Lit::Int(x) => __int(backend, x),
            Lit::Float(x) => __float(backend, x),
            Lit::Bool(x) => __bool(backend, x),
            Lit::Str(x) => __str(backend, x),
        }
    }
}

fn __int(backend: &mut Backend, x: &Int) -> Result<Value, Signal> {
    let raw = backend
        .intern
        .get(x)
        .ok_or(Signal::Error("id does not exist"))?;
    let value = raw
        .parse()
        .map_err(|_| Signal::Error("parsing to `int` failed"))?;
    Ok(Value::Int(value))
}

fn __float(backend: &mut Backend, x: &Float) -> Result<Value, Signal> {
    let raw = backend
        .intern
        .get(x)
        .ok_or(Signal::Error("id does not exist"))?;
    let value = raw
        .parse()
        .map_err(|_| Signal::Error("parsing to `float` failed"))?;
    Ok(Value::Float(value))
}

fn __bool(_: &mut Backend, x: &Bool) -> Result<Value, Signal> {
    let value = match x {
        Bool::True => true,
        Bool::False => false,
    };
    Ok(Value::Bool(value))
}

fn __str(backend: &mut Backend, x: &Str) -> Result<Value, Signal> {
    let value = x
        .iter()
        .map(|chunk| match chunk {
            Chunk::Slice(x) => Ok(backend.intern.get(x).unwrap().to_string()),
            Chunk::Unicode(x) => {
                let hex = backend.intern.get(x).unwrap();
                let Ok(x) = u32::from_str_radix(hex, 16) else {
                    return Err("invalid hex");
                };
                let Some(c) = char::from_u32(x) else {
                    return Err("invalid unicode");
                };
                Ok(c.to_string())
            }
            Chunk::Escape(x) => {
                let str = backend.intern.get(x).unwrap();
                let c = match str {
                    "\"" => '"',
                    "n" => '\n',
                    "t" => '\t',
                    "r" => '\r',
                    "\\" => '\\',
                    _ => return Err("invalid escape character"),
                };
                Ok(c.to_string())
            }
        })
        .collect::<Result<String, &str>>()
        .map_err(Signal::Error)?;
    Ok(Value::Str(value))
}
