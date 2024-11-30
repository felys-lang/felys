use crate::execute::Signal;
use ast::expr::Expr;
use ast::pat::Ident;
use std::ops::{Add, Div, Mul, Neg, Not, Rem, Sub};

pub enum Value {
    Bool(bool),
    Float(f64),
    Int(isize),
    Str(String),
    Closure(Vec<Ident>, Expr),
    Tuple(Vec<Value>),
    Void,
}

pub trait Order {
    type Output;
    fn eq(self, rhs: Self) -> Self::Output;
    fn ne(self, rhs: Self) -> Self::Output;
    fn gt(self, rhs: Self) -> Self::Output;
    fn ge(self, rhs: Self) -> Self::Output;
    fn lt(self, rhs: Self) -> Self::Output;
    fn le(self, rhs: Self) -> Self::Output;
}

pub trait Logical {
    type Output;
    fn and(self, rhs: Self) -> Self::Output;
    fn or(self, rhs: Self) -> Self::Output;
    fn not(self) -> Self::Output;
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

    pub fn void(&self) -> Result<(), Signal> {
        if let Value::Void = self {
            Ok(())
        } else {
            Err(Signal::Error("expect a `void` type".to_string()))
        }
    }
}

impl Add for Value {
    type Output = Result<Self, Signal>;

    fn add(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l + r),
            (Self::Float(l), Self::Float(r)) => Value::Float(l + r),
            _ => return Err(Signal::Error("".to_string()))
        };
        Ok(value)
    }
}

impl Sub for Value {
    type Output = Result<Self, Signal>;

    fn sub(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l - r),
            (Self::Float(l), Self::Float(r)) => Value::Float(l - r),
            _ => return Err(Signal::Error("".to_string()))
        };
        Ok(value)
    }
}

impl Mul for Value {
    type Output = Result<Self, Signal>;

    fn mul(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l * r),
            (Self::Float(l), Self::Float(r)) => Value::Float(l * r),
            _ => return Err(Signal::Error("".to_string()))
        };
        Ok(value)
    }
}

impl Div for Value {
    type Output = Result<Self, Signal>;

    fn div(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l / r),
            (Self::Float(l), Self::Float(r)) => Value::Float(l / r),
            _ => return Err(Signal::Error("".to_string()))
        };
        Ok(value)
    }
}

impl Rem for Value {
    type Output = Result<Self, Signal>;

    fn rem(self, rhs: Self) -> Self::Output {
        let value = match (self, rhs) {
            (Self::Int(l), Self::Int(r)) => Value::Int(l % r),
            (Self::Float(l), Self::Float(r)) => Value::Float(l % r),
            _ => return Err(Signal::Error("".to_string()))
        };
        Ok(value)
    }
}

impl Neg for Value {
    type Output = Result<Self, Signal>;

    fn neg(self) -> Self::Output {
        let value = match self {
            Self::Int(x) => Value::Int(-x),
            Self::Float(x) => Value::Float(-x),
            _ => return Err(Signal::Error("".to_string()))
        };
        Ok(value)
    }
}

impl Order for Value {
    type Output = Result<Value, Signal>;

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
}

impl Logical for Value {
    type Output = Result<Self, Signal>;

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
}