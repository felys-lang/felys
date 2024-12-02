use crate::execute::Signal;
use ast::expr::Expr;
use ast::format::Indenter;
use ast::pat::Ident;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    Float(f64),
    Int(isize),
    Str(String),
    Func(Vec<Ident>, Expr),
    Tuple(Vec<Value>),
    Void,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(val) => write!(f, "{}", val),
            Value::Float(val) => write!(f, "{}", val),
            Value::Int(val) => write!(f, "{}", val),
            Value::Str(val) => write!(f, "{}", val),
            Value::Func(params, expr) => {
                write!(f, "|")?;
                if let Some(first) = params.first() {
                    write!(f, "{}", first)?
                }
                for each in params.iter().skip(1) {
                    write!(f, ", {}", each)?
                }
                write!(f, "| ")?;
                expr.print(0, f)
            }
            Value::Tuple(tup) => {
                write!(f, "(")?;
                if let Some(first) = tup.first() {
                    write!(f, "{}", first)?;
                }
                for val in tup.iter().skip(1) {
                    write!(f, ", {}", val)?
                }
                write!(f, ")")
            }
            Value::Void => write!(f, "<void>"),
        }
    }
}

pub trait Operator {
    type Output;
    fn and(self, rhs: Self) -> Self::Output;
    fn or(self, rhs: Self) -> Self::Output;
    fn not(self) -> Self::Output;
    fn eq(self, rhs: Self) -> Self::Output;
    fn ne(self, rhs: Self) -> Self::Output;
    fn gt(self, rhs: Self) -> Self::Output;
    fn ge(self, rhs: Self) -> Self::Output;
    fn lt(self, rhs: Self) -> Self::Output;
    fn le(self, rhs: Self) -> Self::Output;
    fn add(self, rhs: Self) -> Self::Output;
    fn sub(self, rhs: Self) -> Self::Output;
    fn mul(self, rhs: Self) -> Self::Output;
    fn div(self, rhs: Self) -> Self::Output;
    fn rem(self, rhs: Self) -> Self::Output;
    fn pos(self) -> Self::Output;
    fn neg(self) -> Self::Output;
}

impl Value {
    pub fn bool(&self) -> Result<bool, Signal> {
        let result = match self {
            Value::Bool(x) => *x,
            Value::Float(x) => *x != 0.0,
            Value::Int(x) => *x != 0,
            Value::Str(s) => !s.is_empty(),
            _ => Err(Signal::Error("boolean value not available"))?
        };
        Ok(result)
    }

    pub fn void(self) -> Result<(), Signal> {
        if let Value::Void = self {
            Ok(())
        } else {
            Err(Signal::Error("expect a `void` type"))
        }
    }

    pub fn func(self) -> Result<(Vec<Ident>, Expr), Signal> {
        if let Value::Func(params, expr) = self {
            Ok((params, expr))
        } else {
            Err(Signal::Error("expect a `func` type"))
        }
    }
}

impl Operator for Value {
    type Output = Result<Value, Signal>;

    fn and(self, rhs: Self) -> Self::Output {
        let value = if self.bool()? && rhs.bool()? {
            rhs
        } else {
            Self::Bool(false)
        };
        Ok(value)
    }

    fn or(self, rhs: Self) -> Self::Output {
        let value = if self.bool()? {
            self
        } else if rhs.bool()? {
            rhs
        } else {
            Self::Bool(false)
        };
        Ok(value)
    }

    fn not(self) -> Self::Output {
        let value = self.bool()?;
        Ok(Self::Bool(!value))
    }

    fn eq(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => l == r,
            (Self::Float(l), Self::Float(r)) => l == r,
            (Self::Bool(l), Self::Bool(r)) => l == r,
            (Self::Str(l), Self::Str(r)) => l == r,
            (Self::Tuple(l), Self::Tuple(r)) => {
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
            _ => Err(Signal::Error("operator `==` does not evaluated"))?
        };
        Ok(Value::Bool(value))
    }

    fn ne(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => l != r,
            (Self::Float(l), Self::Float(r)) => l != r,
            (Self::Bool(l), Self::Bool(r)) => l != r,
            (Self::Str(l), Self::Str(r)) => l != r,
            (Self::Tuple(l), Self::Tuple(r)) => {
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
            _ => Err(Signal::Error("operator `!=` does not evaluated"))?
        };
        Ok(Value::Bool(value))
    }

    fn gt(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => l > r,
            (Self::Float(l), Self::Float(r)) => l > r,
            _ => Err(Signal::Error("operator `>` does not evaluated"))?
        };
        Ok(Value::Bool(value))
    }

    fn ge(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => l >= r,
            (Self::Float(l), Self::Float(r)) => l >= r,
            _ => Err(Signal::Error("operator `>=` does not evaluated"))?
        };
        Ok(Value::Bool(value))
    }

    fn lt(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => l < r,
            (Self::Float(l), Self::Float(r)) => l < r,
            _ => Err(Signal::Error("operator `<` does not evaluated"))?
        };
        Ok(Value::Bool(value))
    }

    fn le(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => l <= r,
            (Self::Float(l), Self::Float(r)) => l <= r,
            _ => Err(Signal::Error("operator `<=` does not evaluated"))?
        };
        Ok(Value::Bool(value))
    }

    fn add(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l.saturating_add(r)),
            (Self::Float(l), Self::Float(r)) => Value::Float(l + r),
            _ => Err(Signal::Error("operator `+` does not evaluated"))?
        };
        Ok(value)
    }

    fn sub(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l.saturating_sub(r)),
            (Self::Float(l), Self::Float(r)) => Value::Float(l - r),
            _ => Err(Signal::Error("operator `-` does not evaluated"))?
        };
        Ok(value)
    }

    fn mul(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l.saturating_mul(r)),
            (Self::Float(l), Self::Float(r)) => Value::Float(l * r),
            _ => Err(Signal::Error("operator `*` does not evaluated"))?
        };
        Ok(value)
    }

    fn div(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l.saturating_div(r)),
            (Self::Float(l), Self::Float(r)) => Value::Float(l / r),
            _ => Err(Signal::Error("operator `/` does not evaluated"))?
        };
        Ok(value)
    }

    fn rem(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l % r),
            (Self::Float(l), Self::Float(r)) => Value::Float(l % r),
            _ => Err(Signal::Error("operator `%` does not evaluated"))?
        };
        Ok(value)
    }

    fn pos(self) -> Self::Output {
        let value = match self {
            Self::Int(_) |
            Self::Float(_) => self,
            _ => Err(Signal::Error("operator `+` does not evaluated"))?
        };
        Ok(value)
    }

    fn neg(self) -> Self::Output {
        let value = match self {
            Self::Int(x) => Value::Int(x.saturating_neg()),
            Self::Float(x) => Value::Float(-x),
            _ => Err(Signal::Error("operator `-` does not evaluated"))?
        };
        Ok(value)
    }
}