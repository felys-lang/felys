use crate::ast::{BinOp, UnaOp};
use crate::cyrene::Const;
use crate::fault::Fault;
use std::ops::{Add, Div, Mul, Rem, Sub};

impl Const {
    pub fn binary(&self, op: &BinOp, rhs: &Const) -> Result<Const, Fault> {
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

    pub fn unary(&self, op: &UnaOp) -> Result<Const, Fault> {
        match op {
            UnaOp::Not => self.not(),
            UnaOp::Pos => self.pos(),
            UnaOp::Neg => self.neg(),
        }
    }

    pub fn bool(&self) -> Result<bool, Fault> {
        if let Const::Bool(x) = self {
            Ok(*x)
        } else {
            Err(Fault::InvalidOperation)
        }
    }

    fn int(&self) -> Result<isize, Fault> {
        if let Const::Int(x) = self {
            Ok(*x)
        } else {
            Err(Fault::InvalidOperation)
        }
    }

    fn float(&self) -> Result<f64, Fault> {
        if let Const::Float(x) = self {
            Ok(f64::from_bits(*x))
        } else {
            Err(Fault::InvalidOperation)
        }
    }

    fn str(&self) -> Result<&str, Fault> {
        if let Const::Str(x) = self {
            Ok(x)
        } else {
            Err(Fault::InvalidOperation)
        }
    }

    fn or(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Bool(x) => *x || rhs.bool()?,
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(Const::Bool(value))
    }

    fn and(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Bool(x) => *x && rhs.bool()?,
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(Const::Bool(value))
    }

    fn gt(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x) > rhs.int()?,
            Const::Float(x) => f64::from_bits(*x) > rhs.float()?,
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(Const::Bool(value))
    }

    fn ge(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x) >= rhs.int()?,
            Const::Float(x) => f64::from_bits(*x) >= rhs.float()?,
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(Const::Bool(value))
    }

    fn lt(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x) < rhs.int()?,
            Const::Float(x) => f64::from_bits(*x) < rhs.float()?,
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(Const::Bool(value))
    }

    fn le(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x) <= rhs.int()?,
            Const::Float(x) => f64::from_bits(*x) <= rhs.float()?,
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(Const::Bool(value))
    }

    fn eq(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x) == rhs.int()?,
            Const::Float(x) => f64::from_bits(*x) == rhs.float()?,
            Const::Bool(x) => *x == rhs.bool()?,
            Const::Str(x) => x.as_ref() == rhs.str()?,
        };
        Ok(Const::Bool(value))
    }

    fn ne(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x) != rhs.int()?,
            Const::Float(x) => f64::from_bits(*x) != rhs.float()?,
            Const::Bool(x) => *x != rhs.bool()?,
            Const::Str(x) => x.as_ref() != rhs.str()?,
        };
        Ok(Const::Bool(value))
    }

    fn add(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x)
                .checked_add(rhs.int()?)
                .ok_or(Fault::InvalidOperation)?
                .into(),
            Const::Float(x) => f64::from_bits(*x).add(rhs.float()?).into(),
            Const::Str(x) => format!("{}{}", x, rhs.str()?).into(),
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(value)
    }

    fn sub(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => Const::from(
                (*x).checked_sub(rhs.int()?)
                    .ok_or(Fault::InvalidOperation)?,
            ),
            Const::Float(x) => f64::from_bits(*x).sub(rhs.float()?).into(),
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(value)
    }

    fn mul(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x)
                .checked_mul(rhs.int()?)
                .ok_or(Fault::InvalidOperation)?
                .into(),
            Const::Float(x) => f64::from_bits(*x).mul(rhs.float()?).into(),
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(value)
    }

    fn div(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x)
                .checked_div(rhs.int()?)
                .ok_or(Fault::InvalidOperation)?
                .into(),
            Const::Float(x) => f64::from_bits(*x).div(rhs.float()?).into(),
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(value)
    }

    fn rem(&self, rhs: &Const) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (*x).rem(rhs.int()?).into(),
            Const::Float(x) => f64::from_bits(*x).rem(rhs.float()?).into(),
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(value)
    }

    fn dot(&self, _: &Const) -> Result<Const, Fault> {
        Err(Fault::InvalidOperation)
    }

    fn not(&self) -> Result<Const, Fault> {
        let value = match self {
            Const::Bool(x) => (!x).into(),
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(value)
    }

    fn pos(&self) -> Result<Const, Fault> {
        if matches!(self, Const::Int(_) | Const::Float(_)) {
            Ok(self.clone())
        } else {
            Err(Fault::InvalidOperation)
        }
    }

    fn neg(&self) -> Result<Const, Fault> {
        let value = match self {
            Const::Int(x) => (-*x).into(),
            Const::Float(x) => (-f64::from_bits(*x)).into(),
            _ => return Err(Fault::InvalidOperation),
        };
        Ok(value)
    }
}

impl From<f64> for Const {
    fn from(x: f64) -> Const {
        Const::Float(x.to_bits())
    }
}

impl From<isize> for Const {
    fn from(x: isize) -> Const {
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
