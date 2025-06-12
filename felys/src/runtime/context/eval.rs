use crate::ast::{Expr, Ident};
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
            Value::Tuple(x) => !x.is_empty(),
            Value::List(x) => !x.is_empty(),
            _ => Err(Signal::Error("boolean value not available"))?,
        };
        Ok(result)
    }

    pub fn list(self) -> Result<Vec<Value>, Signal> {
        if let Value::List(list) = self {
            Ok(list)
        } else {
            Err(Signal::Error("expect a `list` type"))
        }
    }

    pub fn void(self) -> Result<(), Signal> {
        if let Value::Void = self {
            Ok(())
        } else {
            Err(Signal::Error("expect a `void` type"))
        }
    }

    pub fn closure(self) -> Result<(Vec<Ident>, Rc<Expr>), Signal> {
        if let Value::Closure(params, expr) = self {
            Ok((params, expr))
        } else {
            Err(Signal::Error("expect a `func` type"))
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
            (Value::Int(l), Value::Int(r)) => l == r,
            (Value::Float(l), Value::Float(r)) => l == r,
            (Value::Bool(l), Value::Bool(r)) => l == r,
            (Value::Str(l), Value::Str(r)) => l == r,
            (Value::Tuple(l), Value::Tuple(r)) => {
                if l.len() != r.len() {
                    return Ok(Value::Bool(false));
                }
                for (ll, rr) in l.into_iter().zip(r) {
                    if ll.ne(rr)?.bool()? {
                        return Ok(Value::Bool(false));
                    }
                }
                true
            }
            (Value::List(l), Value::List(r)) => {
                if l.len() != r.len() {
                    return Ok(Value::Bool(false));
                }
                for (ll, rr) in l.into_iter().zip(r) {
                    if ll.ne(rr)?.bool()? {
                        return Ok(Value::Bool(false));
                    }
                }
                true
            }
            _ => Err(Signal::Error("operator `==` does not evaluate"))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn ne(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => l != r,
            (Value::Float(l), Value::Float(r)) => l != r,
            (Value::Bool(l), Value::Bool(r)) => l != r,
            (Value::Str(l), Value::Str(r)) => l != r,
            (Value::Tuple(l), Value::Tuple(r)) => {
                if l.len() != r.len() {
                    return Ok(Value::Bool(true));
                }
                for (ll, rr) in l.into_iter().zip(r) {
                    if ll.ne(rr)?.bool()? {
                        return Ok(Value::Bool(true));
                    }
                }
                false
            }
            (Value::List(l), Value::List(r)) => {
                if l.len() != r.len() {
                    return Ok(Value::Bool(true));
                }
                for (ll, rr) in l.into_iter().zip(r) {
                    if ll.ne(rr)?.bool()? {
                        return Ok(Value::Bool(true));
                    }
                }
                false
            }
            _ => Err(Signal::Error("operator `!=` does not evaluate"))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn gt(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => l > r,
            (Value::Float(l), Value::Float(r)) => l > r,
            _ => Err(Signal::Error("operator `>` does not evaluate"))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn ge(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => l >= r,
            (Value::Float(l), Value::Float(r)) => l >= r,
            _ => Err(Signal::Error("operator `>=` does not evaluate"))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn lt(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => l < r,
            (Value::Float(l), Value::Float(r)) => l < r,
            _ => Err(Signal::Error("operator `<` does not evaluate"))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn le(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => l <= r,
            (Value::Float(l), Value::Float(r)) => l <= r,
            _ => Err(Signal::Error("operator `<=` does not evaluate"))?,
        };
        Ok(Value::Bool(value))
    }

    pub fn add(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l.saturating_add(r)),
            (Value::Float(l), Value::Float(r)) => Value::Float(l + r),
            (Value::Str(l), Value::Str(r)) => Value::Str(l + r.as_str()),
            (Value::List(mut l), Value::List(mut r)) => {
                l.append(&mut r);
                Value::List(l)
            }
            _ => Err(Signal::Error("operator `+` does not evaluate"))?,
        };
        Ok(value)
    }

    pub fn sub(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l.saturating_sub(r)),
            (Value::Float(l), Value::Float(r)) => Value::Float(l - r),
            _ => Err(Signal::Error("operator `-` does not evaluate"))?,
        };
        Ok(value)
    }

    pub fn mul(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l.saturating_mul(r)),
            (Value::Float(l), Value::Float(r)) => Value::Float(l * r),
            _ => Err(Signal::Error("operator `*` does not evaluate"))?,
        };
        Ok(value)
    }

    pub fn div(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l.saturating_div(r)),
            (Value::Float(l), Value::Float(r)) => Value::Float(l / r),
            _ => Err(Signal::Error("operator `/` does not evaluate"))?,
        };
        Ok(value)
    }

    pub fn rem(self, rhs: Value) -> Result<Value, Signal> {
        let value = match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l % r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l % r),
            _ => Err(Signal::Error("operator `%` does not evaluate"))?,
        };
        Ok(value)
    }

    pub fn pos(self) -> Result<Value, Signal> {
        let value = match self {
            Value::Int(_) | Value::Float(_) => self,
            _ => Err(Signal::Error("operator `+` does not evaluate"))?,
        };
        Ok(value)
    }

    pub fn neg(self) -> Result<Value, Signal> {
        let value = match self {
            Value::Int(x) => Value::Int(x.saturating_neg()),
            Value::Float(x) => Value::Float(-x),
            _ => Err(Signal::Error("operator `-` does not evaluate"))?,
        };
        Ok(value)
    }
}
