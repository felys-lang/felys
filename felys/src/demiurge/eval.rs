use crate::demiurge::error::Error;
use crate::utils::ast::{BinOp, UnaOp};
use crate::utils::function::Const;
use std::ops::{Add, Div, Mul, Rem, Sub};

impl Const {
    pub fn binary(&self, op: &BinOp, rhs: &Const) -> Result<Const, Error> {
        match op {
            BinOp::Or => self.or(rhs),
            BinOp::And => self.and(rhs),
            BinOp::Gt => self.gt(rhs),
            BinOp::Ge => self.ge(rhs),
            BinOp::Lt => self.lt(rhs),
            BinOp::Le => self.le(rhs),
            BinOp::Eq => self.eq(rhs),
            BinOp::Ne => self.ne(rhs),
            BinOp::Add => self.add(rhs),
            BinOp::Sub => self.sub(rhs),
            BinOp::Mul => self.mul(rhs),
            BinOp::Div => self.div(rhs),
            BinOp::Mod => self.rem(rhs),
            BinOp::Dot => self.dot(rhs),
        }
    }

    pub fn unary(&self, op: &UnaOp) -> Result<Const, Error> {
        match op {
            UnaOp::Not => self.not(),
            UnaOp::Pos => self.pos(),
            UnaOp::Neg => self.neg(),
        }
    }

    pub fn bool(&self) -> Result<bool, Error> {
        if let Const::Bool(x) = self {
            Ok(*x)
        } else {
            Err(Error::ConstantType(self.clone(), "bool"))
        }
    }

    fn int(&self) -> Result<i32, Error> {
        if let Const::Int(x) = self {
            Ok(*x)
        } else {
            Err(Error::ConstantType(self.clone(), "int"))
        }
    }

    fn float(&self) -> Result<f32, Error> {
        if let Const::Float(x) = self {
            Ok(f32::from_bits(*x))
        } else {
            Err(Error::ConstantType(self.clone(), "float"))
        }
    }

    fn str(&self) -> Result<&str, Error> {
        if let Const::Str(x) = self {
            Ok(x)
        } else {
            Err(Error::ConstantType(self.clone(), "str"))
        }
    }

    fn or(&self, rhs: &Const) -> Result<Const, Error> {
        let value = match self {
            Const::Bool(x) => *x || rhs.bool()?,
            _ => {
                return Err(Error::BinaryOperation("or", self.clone(), rhs.clone()));
            }
        };
        Ok(Const::Bool(value))
    }

    fn and(&self, rhs: &Const) -> Result<Const, Error> {
        let value = match self {
            Const::Bool(x) => *x && rhs.bool()?,
            _ => {
                return Err(Error::BinaryOperation("and", self.clone(), rhs.clone()));
            }
        };
        Ok(Const::Bool(value))
    }

    fn gt(&self, rhs: &Const) -> Result<Const, Error> {
        let value = match self {
            Const::Int(x) => (*x) > rhs.int()?,
            Const::Float(x) => f32::from_bits(*x) > rhs.float()?,
            _ => {
                return Err(Error::BinaryOperation(">", self.clone(), rhs.clone()));
            }
        };
        Ok(Const::Bool(value))
    }

    fn ge(&self, rhs: &Const) -> Result<Const, Error> {
        let value = match self {
            Const::Int(x) => (*x) >= rhs.int()?,
            Const::Float(x) => f32::from_bits(*x) >= rhs.float()?,
            _ => {
                return Err(Error::BinaryOperation(">=", self.clone(), rhs.clone()));
            }
        };
        Ok(Const::Bool(value))
    }

    fn lt(&self, rhs: &Const) -> Result<Const, Error> {
        let value = match self {
            Const::Int(x) => (*x) < rhs.int()?,
            Const::Float(x) => f32::from_bits(*x) < rhs.float()?,
            _ => {
                return Err(Error::BinaryOperation("<", self.clone(), rhs.clone()));
            }
        };
        Ok(Const::Bool(value))
    }

    fn le(&self, rhs: &Const) -> Result<Const, Error> {
        let value = match self {
            Const::Int(x) => (*x) <= rhs.int()?,
            Const::Float(x) => f32::from_bits(*x) <= rhs.float()?,
            _ => {
                return Err(Error::BinaryOperation("<=", self.clone(), rhs.clone()));
            }
        };
        Ok(Const::Bool(value))
    }

    fn eq(&self, rhs: &Const) -> Result<Const, Error> {
        let value = match self {
            Const::Int(x) => (*x) == rhs.int()?,
            Const::Float(x) => f32::from_bits(*x) == rhs.float()?,
            Const::Bool(x) => *x == rhs.bool()?,
            Const::Str(x) => x.as_ref() == rhs.str()?,
        };
        Ok(Const::Bool(value))
    }

    fn ne(&self, rhs: &Const) -> Result<Const, Error> {
        let value = match self {
            Const::Int(x) => (*x) != rhs.int()?,
            Const::Float(x) => f32::from_bits(*x) != rhs.float()?,
            Const::Bool(x) => *x != rhs.bool()?,
            Const::Str(x) => x.as_ref() != rhs.str()?,
        };
        Ok(Const::Bool(value))
    }

    fn add(&self, rhs: &Const) -> Result<Const, Error> {
        let value = match self {
            Const::Int(x) => (*x)
                .checked_add(rhs.int()?)
                .ok_or(Error::BinaryOperation("+", self.clone(), rhs.clone()))?
                .into(),
            Const::Float(x) => f32::from_bits(*x).add(rhs.float()?).into(),
            Const::Str(x) => format!("{}{}", x, rhs.str()?).into(),
            _ => {
                return Err(Error::BinaryOperation("+", self.clone(), rhs.clone()));
            }
        };
        Ok(value)
    }

    fn sub(&self, rhs: &Const) -> Result<Const, Error> {
        let value = match self {
            Const::Int(x) => {
                Const::from((*x).checked_sub(rhs.int()?).ok_or(Error::BinaryOperation(
                    "-",
                    self.clone(),
                    rhs.clone(),
                ))?)
            }
            Const::Float(x) => f32::from_bits(*x).sub(rhs.float()?).into(),
            _ => {
                return Err(Error::BinaryOperation("-", self.clone(), rhs.clone()));
            }
        };
        Ok(value)
    }

    fn mul(&self, rhs: &Const) -> Result<Const, Error> {
        let value = match self {
            Const::Int(x) => (*x)
                .checked_mul(rhs.int()?)
                .ok_or(Error::BinaryOperation("*", self.clone(), rhs.clone()))?
                .into(),
            Const::Float(x) => f32::from_bits(*x).mul(rhs.float()?).into(),
            _ => {
                return Err(Error::BinaryOperation("*", self.clone(), rhs.clone()));
            }
        };
        Ok(value)
    }

    fn div(&self, rhs: &Const) -> Result<Const, Error> {
        let value = match self {
            Const::Int(x) => (*x)
                .checked_div(rhs.int()?)
                .ok_or(Error::BinaryOperation("/", self.clone(), rhs.clone()))?
                .into(),
            Const::Float(x) => f32::from_bits(*x).div(rhs.float()?).into(),
            _ => {
                return Err(Error::BinaryOperation("/", self.clone(), rhs.clone()));
            }
        };
        Ok(value)
    }

    fn rem(&self, rhs: &Const) -> Result<Const, Error> {
        let value = match self {
            Const::Int(x) => (*x).rem(rhs.int()?).into(),
            Const::Float(x) => f32::from_bits(*x).rem(rhs.float()?).into(),
            _ => {
                return Err(Error::BinaryOperation("%", self.clone(), rhs.clone()));
            }
        };
        Ok(value)
    }

    fn dot(&self, _: &Const) -> Result<Const, Error> {
        Ok(self.clone())
    }

    fn not(&self) -> Result<Const, Error> {
        let value = match self {
            Const::Bool(x) => (!x).into(),
            _ => return Err(Error::UnaryOperation("not", self.clone())),
        };
        Ok(value)
    }

    fn pos(&self) -> Result<Const, Error> {
        if matches!(self, Const::Int(_) | Const::Float(_)) {
            Ok(self.clone())
        } else {
            Err(Error::UnaryOperation("+", self.clone()))
        }
    }

    fn neg(&self) -> Result<Const, Error> {
        let value = match self {
            Const::Int(x) => (-*x).into(),
            Const::Float(x) => (-f32::from_bits(*x)).into(),
            _ => return Err(Error::UnaryOperation("-", self.clone())),
        };
        Ok(value)
    }
}

impl From<f32> for Const {
    fn from(x: f32) -> Const {
        Const::Float(x.to_bits())
    }
}

impl From<i32> for Const {
    fn from(x: i32) -> Const {
        Const::Int(x)
    }
}

impl From<bool> for Const {
    fn from(x: bool) -> Const {
        Const::Bool(x)
    }
}

impl From<String> for Const {
    fn from(x: String) -> Const {
        Const::Str(x.into())
    }
}
