use crate::runtime::context::backend::Backend;
use crate::runtime::context::value::Value;

pub enum Signal {
    Error(&'static str),
    Return(Value),
    Break(Value),
    Continue,
}

pub trait Evaluation {
    fn __eval(&self, backend: &mut Backend) -> Result<Value, Signal>;
    fn eval(&self, backend: &mut Backend) -> Result<Value, Signal> {
        if backend.timer.try_recv().unwrap_or(false) {
            return Err(Signal::Error("timeout"));
        }
        self.__eval(backend)
    }
}
