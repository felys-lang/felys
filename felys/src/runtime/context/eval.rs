use crate::ast::{Expr, Ident};
use crate::nn::layers::{Add, Differentiable, Div, Dot, Mul, Neg, Operator, Sub};
use crate::runtime::context::value::Value;
use crate::runtime::shared::Signal;
use std::rc::Rc;

impl Value {
    pub fn bool(&self) -> Result<bool, Signal> {
        let result = match self {
            Value::Bool(x) => *x,
            Value::Float(x) => *x != 0.0,
            Value::Int(x) => *x != 0,
            Value::Str(x) => !x.is_empty(),
            Value::List(x) => !x.is_empty(),
            _ => Err(Signal::Error("boolean value not available".to_string()))?,
        };
        Ok(result)
    }

    pub fn tuple(self) -> Result<Vec<Value>, Signal> {
        if let Value::Tuple(tuple) = self {
            Ok(tuple)
        } else {
            Err(Signal::Error("expect a `tuple` type".to_string()))
        }
    }

    pub fn list(self) -> Result<Vec<Value>, Signal> {
        if let Value::List(list) = self {
            Ok(list)
        } else {
            Err(Signal::Error("expect a `list` type".to_string()))
        }
    }

    pub fn void(self) -> Result<(), Signal> {
        if let Value::Void = self {
            Ok(())
        } else {
            Err(Signal::Error("expect a `void` type".to_string()))
        }
    }

    pub fn operator(self) -> Result<Operator, Signal> {
        if let Value::Operator(op) = self {
            Ok(op)
        } else {
            Err(Signal::Error("expect a `operator` type".to_string()))
        }
    }

    pub fn float(self) -> Result<f64, Signal> {
        if let Value::Float(float) = self {
            Ok(float)
        } else {
            Err(Signal::Error("expect a `float` type".to_string()))
        }
    }

    pub fn int(self) -> Result<isize, Signal> {
        if let Value::Int(int) = self {
            Ok(int)
        } else {
            Err(Signal::Error("expect a `int` type".to_string()))
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
            _ => Err(Signal::Error("operator `==` does not evaluate".to_string()))?,
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
            _ => Err(Signal::Error("operator `!=` does not evaluate".to_string()))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn gt(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => x > y,
            (Value::Float(x), Value::Float(y)) => x > y,
            _ => Err(Signal::Error("operator `>` does not evaluate".to_string()))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn ge(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => x >= y,
            (Value::Float(x), Value::Float(y)) => x >= y,
            _ => Err(Signal::Error("operator `>=` does not evaluate".to_string()))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn lt(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => x < y,
            (Value::Float(x), Value::Float(y)) => x < y,
            _ => Err(Signal::Error("operator `<` does not evaluate".to_string()))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn le(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => x <= y,
            (Value::Float(x), Value::Float(y)) => x <= y,
            _ => Err(Signal::Error("operator `<=` does not evaluate".to_string()))?,
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
            _ => Err(Signal::Error("operator `+` does not evaluate".to_string()))?,
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
            _ => Err(Signal::Error(
                "operator `-` does not evaluate for".to_string(),
            ))?,
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
            _ => Err(Signal::Error("operator `*` does not evaluate".to_string()))?,
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
            _ => Err(Signal::Error("operator `/` does not evaluate".to_string()))?,
        };
        Ok(value)
    }

    pub fn rem(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x % y),
            (Value::Float(x), Value::Float(y)) => Value::Float(x % y),
            _ => Err(Signal::Error("operator `%` does not evaluate".to_string()))?,
        };
        Ok(value)
    }

    pub fn dot(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Operator(x), Value::Operator(y)) => {
                let op = Dot::build([x, y]).map_err(Signal::Error)?;
                Value::Operator(op)
            }
            _ => Err(Signal::Error("operator `@` does not evaluate".to_string()))?,
        };
        Ok(value)
    }

    pub fn pos(self) -> Result<Value, Signal> {
        let value = match self {
            Value::Int(_) | Value::Float(_) => self,
            _ => Err(Signal::Error("operator `+` does not evaluate".to_string()))?,
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
            _ => Err(Signal::Error("operator `-` does not evaluate".to_string()))?,
        };
        Ok(value)
    }
}
