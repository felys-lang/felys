use crate::runtime::context::backend::{Frame, Global};
use crate::runtime::context::value::Value;

pub enum Signal {
    Error(String),
    Return(Value),
    Break(Value),
    Continue,
}

impl Signal {
    pub fn error(&self) -> String {
        match self {
            Signal::Error(e) => e.to_string(),
            _ => "invalid signal".to_string(),
        }
    }
}

pub trait Evaluation {
    fn eval(&self, global: &mut Global, frame: &mut Frame) -> Result<Value, Signal>;
}
