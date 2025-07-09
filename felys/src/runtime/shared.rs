use crate::runtime::context::backend::{Frame, Global};
use crate::runtime::context::value::Value;

pub enum Signal {
    Error(String),
    Return(Value),
    Break(Value),
    Continue,
}

pub trait Evaluation {
    fn __eval(&self, global: &mut Global, frame: &mut Frame) -> Result<Value, Signal>;
    fn eval(&self, global: &mut Global, frame: &mut Frame) -> Result<Value, Signal> {
        if global.timer.try_recv().unwrap_or(false) {
            return Err(Signal::Error("timeout".to_string()));
        }
        self.__eval(global, frame)
    }
}
