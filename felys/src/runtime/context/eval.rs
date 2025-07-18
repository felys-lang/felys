use crate::nn::layers::{Add, Differentiable, Div, Dot, Mul, Neg, Operator, Sub};
use crate::runtime::context::value::Value;
use crate::runtime::shared::Signal;

impl Value {
    pub fn bool(&self) -> Result<bool, Signal> {
        let result = match self {
            Value::Bool(x) => *x,
            Value::Float(x) => *x != 0.0,
            Value::Int(x) => *x != 0,
            Value::Str(x) => !x.is_empty(),
            Value::List(x) => !x.is_empty(),
            x => Err(Signal::Error(format!("`{x}` does not have a boolean value")))?,
        };
        Ok(result)
    }

    pub fn tuple(self) -> Result<Vec<Value>, Signal> {
        match self {
            Value::Tuple(tuple) => Ok(tuple),
            x => Err(Signal::Error(format!("`{x}` is not a `tuple`"))),
        }
    }

    pub fn list(self) -> Result<Vec<Value>, Signal> {
        match self {
            Value::List(list) => Ok(list),
            x => Err(Signal::Error(format!("`{x}` is not a `list`"))),
        }
    }

    pub fn void(self) -> Result<(), Signal> {
        match self {
            Value::Void => Ok(()),
            x => Err(Signal::Error(format!("`{x}` is not a `void`"))),
        }
    }

    pub fn operator(self) -> Result<Operator, Signal> {
        match self {
            Value::Operator(op) => Ok(op),
            x => Err(Signal::Error(format!("`{x}` is not a `operator`"))),
        }
    }

    pub fn float(self) -> Result<f64, Signal> {
        match self {
            Value::Float(float) => Ok(float),
            x => Err(Signal::Error(format!("`{x}` is not a `float`"))),
        }
    }

    pub fn int(self) -> Result<isize, Signal> {
        match self {
            Value::Int(int) => Ok(int),
            x => Err(Signal::Error(format!("`{x}` is not a `int`"))),
        }
    }
}

impl Value {
    pub fn and(self, rhs: Value) -> Result<Value, Signal> {
        let value = if self.bool()? && rhs.bool()? {
            rhs
        } else {
            Value::Bool(false)
        };
        Ok(value)
    }

    pub fn or(self, rhs: Value) -> Result<Value, Signal> {
        let value = if self.bool()? {
            self
        } else if rhs.bool()? {
            rhs
        } else {
            Value::Bool(false)
        };
        Ok(value)
    }

    pub fn not(self) -> Result<Value, Signal> {
        let value = self.bool()?;
        Ok(Value::Bool(!value))
    }

    pub fn eq(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => x == y,
            (Value::Float(x), Value::Float(y)) => x == y,
            (Value::Bool(x), Value::Bool(y)) => x == y,
            (Value::Str(x), Value::Str(y)) => x == y,
            (Value::Tuple(x), Value::Tuple(y)) => {
                if x.len() != y.len() {
                    return Ok(Value::Bool(false));
                }
                for (ll, rr) in x.into_iter().zip(y) {
                    if ll.ne(rr)?.bool()? {
                        return Ok(Value::Bool(false));
                    }
                }
                true
            }
            (Value::List(x), Value::List(y)) => {
                if x.len() != y.len() {
                    return Ok(Value::Bool(false));
                }
                for (ll, rr) in x.into_iter().zip(y) {
                    if ll.ne(rr)?.bool()? {
                        return Ok(Value::Bool(false));
                    }
                }
                true
            }
            (x, y) => Err(Signal::Error(format!("`{x} == {y}` does not evaluate")))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn ne(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => x != y,
            (Value::Float(x), Value::Float(y)) => x != y,
            (Value::Bool(x), Value::Bool(y)) => x != y,
            (Value::Str(x), Value::Str(y)) => x != y,
            (Value::Tuple(x), Value::Tuple(y)) => {
                if x.len() != y.len() {
                    return Ok(Value::Bool(true));
                }
                for (x, y) in x.into_iter().zip(y) {
                    if x.ne(y)?.bool()? {
                        return Ok(Value::Bool(true));
                    }
                }
                false
            }
            (Value::List(x), Value::List(y)) => {
                if x.len() != y.len() {
                    return Ok(Value::Bool(true));
                }
                for (x, y) in x.into_iter().zip(y) {
                    if x.ne(y)?.bool()? {
                        return Ok(Value::Bool(true));
                    }
                }
                false
            }
            (x, y) => Err(Signal::Error(format!("`{x} != {y}` does not evaluate")))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn gt(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => x > y,
            (Value::Float(x), Value::Float(y)) => x > y,
            (x, y) => Err(Signal::Error(format!("`{x} > {y}` does not evaluate")))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn ge(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => x >= y,
            (Value::Float(x), Value::Float(y)) => x >= y,
            (x, y) => Err(Signal::Error(format!("`{x} >= {y}` does not evaluate")))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn lt(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => x < y,
            (Value::Float(x), Value::Float(y)) => x < y,
            (x, y) => Err(Signal::Error(format!("`{x} < {y}` does not evaluate")))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn le(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => x <= y,
            (Value::Float(x), Value::Float(y)) => x <= y,
            (x, y) => Err(Signal::Error(format!("`{x} <= {y}` does not evaluate")))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn add(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x.saturating_add(y)),
            (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
            (Value::Str(x), Value::Str(y)) => Value::Str(x + y.as_str()),
            (Value::List(mut x), Value::List(y)) => {
                x.extend(y);
                Value::List(x)
            }
            (Value::Operator(x), Value::Operator(y)) => {
                let op = Add::build([x, y]).map_err(Signal::Error)?;
                Value::Operator(op)
            }
            (x, y) => Err(Signal::Error(format!("`{x} + {y}` does not evaluate")))?,
        };
        Ok(value)
    }

    pub fn sub(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x.saturating_sub(y)),
            (Value::Float(x), Value::Float(y)) => Value::Float(x - y),
            (Value::Operator(x), Value::Operator(y)) => {
                let op = Sub::build([x, y]).map_err(Signal::Error)?;
                Value::Operator(op)
            }
            (x, y) => Err(Signal::Error(format!("`{x} - {y}` does not evaluate")))?,
        };
        Ok(value)
    }

    pub fn mul(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x.saturating_mul(y)),
            (Value::Float(x), Value::Float(y)) => Value::Float(x * y),
            (Value::Operator(x), Value::Operator(y)) => {
                let op = Mul::build([x, y]).map_err(Signal::Error)?;
                Value::Operator(op)
            }
            (x, y) => Err(Signal::Error(format!("`{x} * {y}` does not evaluate")))?,
        };
        Ok(value)
    }

    pub fn div(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x.saturating_div(y)),
            (Value::Float(x), Value::Float(y)) => Value::Float(x / y),
            (Value::Operator(x), Value::Operator(y)) => {
                let op = Div::build([x, y]).map_err(Signal::Error)?;
                Value::Operator(op)
            }
            (x, y) => Err(Signal::Error(format!("`{x} / {y}` does not evaluate")))?,
        };
        Ok(value)
    }

    pub fn rem(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x % y),
            (Value::Float(x), Value::Float(y)) => Value::Float(x % y),
            (x, y) => Err(Signal::Error(format!("`{x} % {y}` does not evaluate")))?,
        };
        Ok(value)
    }

    pub fn dot(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Operator(x), Value::Operator(y)) => {
                let op = Dot::build([x, y]).map_err(Signal::Error)?;
                Value::Operator(op)
            }
            (x, y) => Err(Signal::Error(format!("`{x} @ {y}` does not evaluate")))?,
        };
        Ok(value)
    }

    pub fn pos(self) -> Result<Value, Signal> {
        let value = match self {
            Value::Int(_) | Value::Float(_) => self,
            x => Err(Signal::Error(format!("`+{x}` does not evaluate")))?,
        };
        Ok(value)
    }

    pub fn neg(self) -> Result<Value, Signal> {
        let value = match self {
            Value::Int(x) => Value::Int(x.saturating_neg()),
            Value::Float(x) => Value::Float(-x),
            Value::Operator(x) => {
                let op = Neg::build([x]).map_err(Signal::Error)?;
                Value::Operator(op)
            }
            x => Err(Signal::Error(format!("`-{x}` does not evaluate")))?,
        };
        Ok(value)
    }
}
