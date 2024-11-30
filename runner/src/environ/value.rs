use crate::execute::Signal;
use ast::expr::Expr;
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
            Value::Func(_, _) => todo!(),
            Value::Tuple(_) => todo!(),
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
            _ => return Err(Signal::Error("".to_string()))
        };
        Ok(result)
    }

    pub fn void(self) -> Result<(), Signal> {
        if let Value::Void = self {
            Ok(())
        } else {
            Err(Signal::Error("expect a `void` type".to_string()))
        }
    }

    pub fn func(self) -> Result<(Vec<Ident>, Expr), Signal> {
        if let Value::Func(params, expr) = self {
            Ok((params, expr))
        } else {
            Err(Signal::Error("expect a `func` type".to_string()))
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
        todo!()
    }

    fn ne(self, rhs: Self) -> Self::Output {
        todo!()
    }

    fn gt(self, rhs: Self) -> Self::Output {
        todo!()
    }

    fn ge(self, rhs: Self) -> Self::Output {
        todo!()
    }

    fn lt(self, rhs: Self) -> Self::Output {
        todo!()
    }

    fn le(self, rhs: Self) -> Self::Output {
        todo!()
    }

    fn add(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l + r),
            (Self::Float(l), Self::Float(r)) => Value::Float(l + r),
            _ => return Err(Signal::Error("".to_string()))
        };
        Ok(value)
    }

    fn sub(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l - r),
            (Self::Float(l), Self::Float(r)) => Value::Float(l - r),
            _ => return Err(Signal::Error("".to_string()))
        };
        Ok(value)
    }

    fn mul(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l * r),
            (Self::Float(l), Self::Float(r)) => Value::Float(l * r),
            _ => return Err(Signal::Error("".to_string()))
        };
        Ok(value)
    }

    fn div(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l / r),
            (Self::Float(l), Self::Float(r)) => Value::Float(l / r),
            _ => return Err(Signal::Error("".to_string()))
        };
        Ok(value)
    }

    fn rem(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l % r),
            (Self::Float(l), Self::Float(r)) => Value::Float(l % r),
            _ => return Err(Signal::Error("".to_string()))
        };
        Ok(value)
    }

    fn pos(self) -> Self::Output {
        Ok(self)
    }

    fn neg(self) -> Self::Output {
        let value = match self {
            Self::Int(x) => Value::Int(-x),
            Self::Float(x) => Value::Float(-x),
            _ => return Err(Signal::Error("".to_string()))
        };
        Ok(value)
    }
}