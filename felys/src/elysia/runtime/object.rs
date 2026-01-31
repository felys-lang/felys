use crate::elysia::fault::Fault;
use crate::utils::ast::{BinOp, UnaOp};
use crate::utils::bytecode::Index;
use crate::utils::ir::Pointer;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Object {
    Pointer(Pointer, Index),
    List(Rc<[Object]>),
    Tuple(Rc<[Object]>),
    Group(Index, Rc<[Object]>),
    Str(Rc<str>),
    Int(isize),
    Float(f64),
    Bool(bool),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Pointer(pt, ptr) => match pt {
                Pointer::Function => write!(f, "F @ {ptr:#010x}"),
                Pointer::Group => write!(f, "G @ {ptr:#010x}"),
                Pointer::Rust => write!(f, "R @ {ptr:#010x}"),
            },
            Object::List(objs) => {
                write!(f, "[")?;
                let mut iter = objs.iter();
                if let Some(first) = iter.next() {
                    first.recover(f)?
                }
                for obj in iter {
                    write!(f, ", ")?;
                    obj.recover(f)?
                }
                write!(f, "]")
            }
            Object::Tuple(objs) => {
                write!(f, "(")?;
                let mut iter = objs.iter();
                if let Some(first) = iter.next() {
                    first.recover(f)?
                }
                for obj in iter {
                    write!(f, ", ")?;
                    obj.recover(f)?
                }
                write!(f, ")")
            }
            Object::Group(id, objs) => {
                write!(f, "<")?;
                let mut iter = objs.iter();
                if let Some(first) = iter.next() {
                    first.recover(f)?
                }
                for obj in iter {
                    write!(f, ", ")?;
                    obj.recover(f)?
                }
                write!(f, "> : {id:#010x}")
            }
            Object::Str(x) => write!(f, "\"{}\"", x),
            Object::Int(x) => write!(f, "{}", x),
            Object::Float(x) => write!(f, "{}", x),
            Object::Bool(x) => write!(f, "{}", x),
        }
    }
}

impl Object {
    pub fn list(&self) -> Result<Rc<[Object]>, Fault> {
        if let Object::List(x) = self {
            Ok(x.clone())
        } else {
            Err(Fault::DataType(self.clone(), "list"))
        }
    }

    pub fn tuple(&self) -> Result<Rc<[Object]>, Fault> {
        if let Object::Tuple(x) = self {
            Ok(x.clone())
        } else {
            Err(Fault::DataType(self.clone(), "tuple"))
        }
    }

    pub fn group(&self) -> Result<(Index, Rc<[Object]>), Fault> {
        if let Object::Group(x, elements) = self {
            Ok((*x, elements.clone()))
        } else {
            Err(Fault::DataType(self.clone(), "group"))
        }
    }

    pub fn pointer(&self) -> Result<(Pointer, Index), Fault> {
        if let Object::Pointer(ty, idx) = self {
            Ok((*ty, *idx))
        } else {
            Err(Fault::DataType(self.clone(), "ptr"))
        }
    }

    pub fn bool(&self) -> Result<bool, Fault> {
        if let Object::Bool(x) = self {
            Ok(*x)
        } else {
            Err(Fault::DataType(self.clone(), "bool"))
        }
    }

    pub fn int(&self) -> Result<isize, Fault> {
        if let Object::Int(x) = self {
            Ok(*x)
        } else {
            Err(Fault::DataType(self.clone(), "int"))
        }
    }

    pub fn float(&self) -> Result<f64, Fault> {
        if let Object::Float(x) = self {
            Ok(*x)
        } else {
            Err(Fault::DataType(self.clone(), "float"))
        }
    }

    pub fn str(&self) -> Result<&str, Fault> {
        if let Object::Str(x) = self {
            Ok(x)
        } else {
            Err(Fault::DataType(self.clone(), "str"))
        }
    }

    pub fn binary(self, op: BinOp, rhs: Object) -> Result<Object, Fault> {
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

    pub fn unary(self, op: UnaOp) -> Result<Object, Fault> {
        match op {
            UnaOp::Not => self.not(),
            UnaOp::Pos => self.pos(),
            UnaOp::Neg => self.neg(),
        }
    }

    fn or(self, rhs: Object) -> Result<Object, Fault> {
        let value = match self {
            Object::Bool(x) => x || rhs.bool()?,
            _ => return Err(Fault::BinaryOperation("or", self, rhs)),
        };
        Ok(Object::Bool(value))
    }

    fn and(self, rhs: Object) -> Result<Object, Fault> {
        let value = match self {
            Object::Bool(x) => x && rhs.bool()?,
            _ => return Err(Fault::BinaryOperation("and", self, rhs)),
        };
        Ok(Object::Bool(value))
    }

    fn gt(self, rhs: Object) -> Result<Object, Fault> {
        let value = match self {
            Object::Int(x) => x > rhs.int()?,
            Object::Float(x) => x > rhs.float()?,
            _ => return Err(Fault::BinaryOperation(">", self, rhs)),
        };
        Ok(Object::Bool(value))
    }

    fn ge(self, rhs: Object) -> Result<Object, Fault> {
        let value = match self {
            Object::Int(x) => x >= rhs.int()?,
            Object::Float(x) => x >= rhs.float()?,
            _ => return Err(Fault::BinaryOperation(">=", self, rhs)),
        };
        Ok(Object::Bool(value))
    }

    fn lt(self, rhs: Object) -> Result<Object, Fault> {
        let value = match self {
            Object::Int(x) => x < rhs.int()?,
            Object::Float(x) => x < rhs.float()?,
            _ => return Err(Fault::BinaryOperation("<", self, rhs)),
        };
        Ok(Object::Bool(value))
    }

    fn le(self, rhs: Object) -> Result<Object, Fault> {
        let value = match self {
            Object::Int(x) => x <= rhs.int()?,
            Object::Float(x) => x <= rhs.float()?,
            _ => return Err(Fault::BinaryOperation(">=", self, rhs)),
        };
        Ok(Object::Bool(value))
    }

    fn eq(self, rhs: Object) -> Result<Object, Fault> {
        let value = match self {
            Object::Int(x) => x == rhs.int()?,
            Object::Float(x) => x == rhs.float()?,
            Object::Bool(x) => x == rhs.bool()?,
            Object::Str(x) => x.as_ref() == rhs.str()?,
            Object::Tuple(lhs) => {
                let objs = rhs.tuple()?;
                if lhs.len() != objs.len() {
                    return Ok(Object::Bool(false));
                }
                for (x, y) in lhs.iter().zip(objs.iter()) {
                    if !x.clone().eq(y.clone())?.bool()? {
                        return Ok(Object::Bool(false));
                    }
                }
                true
            }
            Object::List(lhs) => {
                let objs = rhs.list()?;
                if lhs.len() != objs.len() {
                    return Ok(Object::Bool(false));
                }
                for (x, y) in lhs.iter().zip(objs.iter()) {
                    if !x.clone().eq(y.clone())?.bool()? {
                        return Ok(Object::Bool(false));
                    }
                }
                true
            }
            Object::Group(idx, lhs) => {
                let (i, objs) = rhs.group()?;
                if idx != i || lhs.len() != objs.len() {
                    return Ok(Object::Bool(false));
                }
                for (x, y) in lhs.iter().zip(objs.iter()) {
                    if !x.clone().eq(y.clone())?.bool()? {
                        return Ok(Object::Bool(false));
                    }
                }
                true
            }
            Object::Pointer(pt, ptr) => (pt, ptr) == rhs.pointer()?,
        };
        Ok(Object::Bool(value))
    }

    fn ne(self, rhs: Object) -> Result<Object, Fault> {
        let value = !self.eq(rhs)?.bool()?;
        Ok(Object::Bool(value))
    }

    fn add(self, rhs: Object) -> Result<Object, Fault> {
        let value = match self {
            Object::Int(x) => x
                .checked_add(rhs.int()?)
                .ok_or(Fault::BinaryOperation("+", self, rhs))?
                .into(),
            Object::Float(x) => (x + rhs.float()?).into(),
            Object::Str(x) => format!("{}{}", x, rhs.str()?).into(),
            _ => return Err(Fault::BinaryOperation("+", self, rhs)),
        };
        Ok(value)
    }

    fn sub(self, rhs: Object) -> Result<Object, Fault> {
        let value = match self {
            Object::Int(x) => Object::from(
                x.checked_sub(rhs.int()?)
                    .ok_or(Fault::BinaryOperation("-", self, rhs))?,
            ),
            Object::Float(x) => (x - rhs.float()?).into(),
            _ => return Err(Fault::BinaryOperation("-", self, rhs)),
        };
        Ok(value)
    }

    fn mul(self, rhs: Object) -> Result<Object, Fault> {
        let value = match self {
            Object::Int(x) => x
                .checked_mul(rhs.int()?)
                .ok_or(Fault::BinaryOperation("*", self, rhs))?
                .into(),
            Object::Float(x) => (x * rhs.float()?).into(),
            _ => return Err(Fault::BinaryOperation("*", self, rhs)),
        };
        Ok(value)
    }

    fn div(self, rhs: Object) -> Result<Object, Fault> {
        let value = match self {
            Object::Int(x) => x
                .checked_div(rhs.int()?)
                .ok_or(Fault::BinaryOperation("/", self, rhs))?
                .into(),
            Object::Float(x) => (x / rhs.float()?).into(),
            _ => return Err(Fault::BinaryOperation("/", self, rhs)),
        };
        Ok(value)
    }

    fn rem(self, rhs: Object) -> Result<Object, Fault> {
        let value = match self {
            Object::Int(x) => (x % rhs.int()?).into(),
            Object::Float(x) => (x % rhs.float()?).into(),
            _ => return Err(Fault::BinaryOperation("%", self, rhs)),
        };
        Ok(value)
    }

    fn dot(self, _: Object) -> Result<Object, Fault> {
        Ok(self)
    }

    fn not(self) -> Result<Object, Fault> {
        let value = match self {
            Object::Bool(x) => (!x).into(),
            _ => return Err(Fault::UnaryOperation("not", self)),
        };
        Ok(value)
    }

    fn pos(self) -> Result<Object, Fault> {
        if matches!(self, Object::Int(_) | Object::Float(_)) {
            Ok(self.clone())
        } else {
            Err(Fault::UnaryOperation("+", self))
        }
    }

    fn neg(self) -> Result<Object, Fault> {
        let value = match self {
            Object::Int(x) => (-x).into(),
            Object::Float(x) => (-x).into(),
            _ => return Err(Fault::UnaryOperation("-", self)),
        };
        Ok(value)
    }
}

impl From<f64> for Object {
    fn from(x: f64) -> Object {
        Object::Float(x)
    }
}

impl From<isize> for Object {
    fn from(x: isize) -> Object {
        Object::Int(x)
    }
}

impl From<bool> for Object {
    fn from(x: bool) -> Object {
        Object::Bool(x)
    }
}

impl From<String> for Object {
    fn from(x: String) -> Object {
        Object::Str(x.into())
    }
}
