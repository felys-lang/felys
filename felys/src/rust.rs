use crate::nn::layers::{CrossEntropy, Differentiable, ReLU};
use crate::runtime::context::value::Value;
use crate::runtime::shared::Signal;

pub fn relu(mut args: Vec<Value>) -> Result<Value, Signal> {
    let op = args
        .pop()
        .ok_or(Signal::Error("ReLU expected 1 argument".to_string()))?
        .operator()?;
    if args.pop().is_some() {
        return Err(Signal::Error("ReLU expect 1 argument".to_string()));
    }
    let result = ReLU::build([op]).map_err(Signal::Error)?;
    Ok(Value::Operator(result))
}

pub fn ce(mut args: Vec<Value>) -> Result<Value, Signal> {
    let label = args
        .pop()
        .ok_or(Signal::Error(
            "CrossEntropy expected 2 argument".to_string(),
        ))?
        .operator()?;
    let logits = args
        .pop()
        .ok_or(Signal::Error(
            "CrossEntropy expected 2 argument".to_string(),
        ))?
        .operator()?;
    if args.pop().is_some() {
        return Err(Signal::Error("CrossEntropy expect 2 argument".to_string()));
    }
    let result = CrossEntropy::build([logits, label]).map_err(Signal::Error)?;
    Ok(Value::Operator(result))
}
