use crate::ast::{Bool, Chunk, Float, Int, Lit, Str};
use crate::runtime::context::backend::{Frame, Global};
use crate::runtime::context::value::Value;
use crate::runtime::shared::{Evaluation, Signal};

impl Evaluation for Lit {
    fn eval(&self, global: &mut Global, frame: &mut Frame) -> Result<Value, Signal> {
        match self {
            Lit::Int(x) => x.eval(global, frame),
            Lit::Float(x) => x.eval(global, frame),
            Lit::Bool(x) => __bool(global, frame, x),
            Lit::Str(x) => __str(global, frame, x),
        }
    }
}

impl Evaluation for Float {
    fn eval(&self, global: &mut Global, _: &mut Frame) -> Result<Value, Signal> {
        let raw = global
            .intern
            .get(&self.0)
            .ok_or(Signal::Error("id does not exist".to_string()))?;
        let value = raw
            .parse()
            .map_err(|_| Signal::Error("parsing to `float` failed".to_string()))?;
        Ok(Value::Float(value))
    }
}

impl Evaluation for Int {
    fn eval(&self, global: &mut Global, _: &mut Frame) -> Result<Value, Signal> {
        let raw = global
            .intern
            .get(&self.0)
            .ok_or(Signal::Error("id does not exist".to_string()))?;
        let value = raw
            .parse()
            .map_err(|_| Signal::Error("parsing to `int` failed".to_string()))?;
        Ok(Value::Int(value))
    }
}

fn __bool(_: &mut Global, _: &mut Frame, x: &Bool) -> Result<Value, Signal> {
    let value = match x {
        Bool::True => true,
        Bool::False => false,
    };
    Ok(Value::Bool(value))
}

fn __str(global: &mut Global, _: &mut Frame, x: &Str) -> Result<Value, Signal> {
    let value = x
        .iter()
        .map(|chunk| match chunk {
            Chunk::Slice(x) => Ok(global.intern.get(x).ok_or("unreachable")?.to_string()),
            Chunk::Unicode(x) => {
                let hex = global.intern.get(x).ok_or("unreachable")?;
                let Ok(x) = u32::from_str_radix(hex, 16) else {
                    return Err("invalid hex".to_string());
                };
                let Some(c) = char::from_u32(x) else {
                    return Err("invalid unicode".to_string());
                };
                Ok(c.to_string())
            }
            Chunk::Escape(x) => {
                let str = global.intern.get(x).ok_or("unreachable")?;
                let c = match str {
                    "\"" => '"',
                    "n" => '\n',
                    "t" => '\t',
                    "r" => '\r',
                    "\\" => '\\',
                    _ => return Err("invalid escape character".to_string()),
                };
                Ok(c.to_string())
            }
        })
        .collect::<Result<String, String>>()
        .map_err(Signal::Error)?;
    Ok(Value::Str(value))
}
