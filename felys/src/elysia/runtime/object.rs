use crate::elysia::error::Error;
use crate::utils::ast::{BinOp, UnaOp};
use crate::utils::bytecode::Index;
use crate::utils::function::Pointer;
use crate::utils::stdlib::nn::operator::Node;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Object {
    Pointer(Pointer, Index),
    List(Rc<[Object]>),
    Tuple(Rc<[Object]>),
    Group(Index, Rc<[Object]>),
    Str(Rc<str>),
    Int(i32),
    Float(f32),
    Bool(bool),
    Node(Rc<Node>),
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
                    write!(f, "{first}")?;
                }
                for obj in iter {
                    write!(f, ", {obj}")?;
                }
                write!(f, "]")
            }
            Object::Tuple(objs) => {
                write!(f, "(")?;
                let mut iter = objs.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{first}")?;
                }
                for obj in iter {
                    write!(f, ", {obj}")?;
                }
                write!(f, ")")
            }
            Object::Group(id, objs) => {
                write!(f, "<")?;
                let mut iter = objs.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{first}")?;
                }
                for obj in iter {
                    write!(f, ", {obj}")?;
                }
                write!(f, "> as {id:#010x}")
            }
            Object::Str(x) => write!(f, "{}", x),
            Object::Int(x) => write!(f, "{}", x),
            Object::Float(x) => write!(f, "{}", x),
            Object::Bool(x) => write!(f, "{}", x),
            Object::Node(x) => write!(f, "{:?}", x),
        }
    }
}

impl Object {
    pub fn list(&self) -> Result<Rc<[Object]>, Error> {
        if let Object::List(x) = self {
            Ok(x.clone())
        } else {
            Err(Error::DataType(self.clone(), "list"))
        }
    }

    pub fn tuple(&self) -> Result<Rc<[Object]>, Error> {
        if let Object::Tuple(x) = self {
            Ok(x.clone())
        } else {
            Err(Error::DataType(self.clone(), "tuple"))
        }
    }

    pub fn group(&self) -> Result<(Index, Rc<[Object]>), Error> {
        if let Object::Group(x, elements) = self {
            Ok((*x, elements.clone()))
        } else {
            Err(Error::DataType(self.clone(), "group"))
        }
    }

    pub fn pointer(&self) -> Result<(Pointer, Index), Error> {
        if let Object::Pointer(ty, idx) = self {
            Ok((*ty, *idx))
        } else {
            Err(Error::DataType(self.clone(), "ptr"))
        }
    }

    pub fn node(&self) -> Result<Rc<Node>, Error> {
        if let Object::Node(x) = self {
            Ok(x.clone())
        } else {
            Err(Error::DataType(self.clone(), "node"))
        }
    }

    pub fn bool(&self) -> Result<bool, Error> {
        if let Object::Bool(x) = self {
            Ok(*x)
        } else {
            Err(Error::DataType(self.clone(), "bool"))
        }
    }

    pub fn int(&self) -> Result<i32, Error> {
        if let Object::Int(x) = self {
            Ok(*x)
        } else {
            Err(Error::DataType(self.clone(), "int"))
        }
    }

    pub fn float(&self) -> Result<f32, Error> {
        if let Object::Float(x) = self {
            Ok(*x)
        } else {
            Err(Error::DataType(self.clone(), "float"))
        }
    }

    pub fn str(&self) -> Result<&str, Error> {
        if let Object::Str(x) = self {
            Ok(x)
        } else {
            Err(Error::DataType(self.clone(), "str"))
        }
    }

    pub fn binary(self, op: BinOp, rhs: Object) -> Result<Object, Error> {
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
            BinOp::At => self.matmul(rhs),
        }
    }

    pub fn unary(self, op: UnaOp) -> Result<Object, Error> {
        match op {
            UnaOp::Not => self.not(),
            UnaOp::Pos => self.pos(),
            UnaOp::Neg => self.neg(),
        }
    }

    fn or(self, rhs: Object) -> Result<Object, Error> {
        let value = match (self, rhs) {
            (Object::Bool(x), Object::Bool(y)) => x || y,
            (lhs, rhs) => return Err(Error::BinaryOperation("or", lhs, rhs)),
        };
        Ok(Object::Bool(value))
    }

    fn and(self, rhs: Object) -> Result<Object, Error> {
        let value = match (self, rhs) {
            (Object::Bool(x), Object::Bool(y)) => x && y,
            (lhs, rhs) => return Err(Error::BinaryOperation("and", lhs, rhs)),
        };
        Ok(Object::Bool(value))
    }

    fn gt(self, rhs: Object) -> Result<Object, Error> {
        let value = match (self, rhs) {
            (Object::Int(x), Object::Int(y)) => x > y,
            (Object::Float(x), Object::Float(y)) => x > y,
            (lhs, rhs) => return Err(Error::BinaryOperation(">", lhs, rhs)),
        };
        Ok(Object::Bool(value))
    }

    fn ge(self, rhs: Object) -> Result<Object, Error> {
        let value = match (self, rhs) {
            (Object::Int(x), Object::Int(y)) => x >= y,
            (Object::Float(x), Object::Float(y)) => x >= y,
            (lhs, rhs) => return Err(Error::BinaryOperation(">=", lhs, rhs)),
        };
        Ok(Object::Bool(value))
    }

    fn lt(self, rhs: Object) -> Result<Object, Error> {
        let value = match (self, rhs) {
            (Object::Int(x), Object::Int(y)) => x < y,
            (Object::Float(x), Object::Float(y)) => x < y,
            (lhs, rhs) => return Err(Error::BinaryOperation("<", lhs, rhs)),
        };
        Ok(Object::Bool(value))
    }

    fn le(self, rhs: Object) -> Result<Object, Error> {
        let value = match (self, rhs) {
            (Object::Int(x), Object::Int(y)) => x <= y,
            (Object::Float(x), Object::Float(y)) => x <= y,
            (lhs, rhs) => return Err(Error::BinaryOperation("<=", lhs, rhs)),
        };
        Ok(Object::Bool(value))
    }

    fn eq(self, rhs: Object) -> Result<Object, Error> {
        let value = match (self, rhs) {
            (Object::Int(x), Object::Int(y)) => x == y,
            (Object::Float(x), Object::Float(y)) => x == y,
            (Object::Bool(x), Object::Bool(y)) => x == y,
            (Object::Str(x), Object::Str(y)) => x == y,
            (Object::Pointer(xp, xi), Object::Pointer(yp, yi)) => xp == yp && xi == yi,
            (Object::Tuple(lhs), Object::Tuple(rhs)) => {
                if lhs.len() != rhs.len() {
                    false
                } else {
                    let mut res = true;
                    for (x, y) in lhs.iter().zip(rhs.iter()) {
                        if !x.clone().eq(y.clone())?.bool()? {
                            res = false;
                            break;
                        }
                    }
                    res
                }
            }
            (Object::List(lhs), Object::List(rhs)) => {
                if lhs.len() != rhs.len() {
                    false
                } else {
                    let mut res = true;
                    for (x, y) in lhs.iter().zip(rhs.iter()) {
                        if !x.clone().eq(y.clone())?.bool()? {
                            res = false;
                            break;
                        }
                    }
                    res
                }
            }
            (Object::Group(xi, lhs), Object::Group(yi, rhs)) => {
                if xi != yi || lhs.len() != rhs.len() {
                    false
                } else {
                    let mut res = true;
                    for (x, y) in lhs.iter().zip(rhs.iter()) {
                        if !x.clone().eq(y.clone())?.bool()? {
                            res = false;
                            break;
                        }
                    }
                    res
                }
            }
            (lhs, rhs) => return Err(Error::BinaryOperation("==", lhs, rhs)),
        };
        Ok(Object::Bool(value))
    }

    fn ne(self, rhs: Object) -> Result<Object, Error> {
        let value = !self.eq(rhs)?.bool()?;
        Ok(Object::Bool(value))
    }

    fn add(self, rhs: Object) -> Result<Object, Error> {
        let value = match (self, rhs) {
            (Object::Int(x), Object::Int(y)) => x
                .checked_add(y)
                .ok_or(Error::BinaryOperation("+", x.into(), y.into()))?
                .into(),
            (Object::Float(x), Object::Float(y)) => (x + y).into(),
            (Object::Str(x), Object::Str(y)) => format!("{}{}", x, y).into(),
            (Object::Node(x), Object::Node(y)) => Node::add(x, y).map_err(Error::Any)?.into(),
            (lhs, rhs) => return Err(Error::BinaryOperation("+", lhs, rhs)),
        };
        Ok(value)
    }

    fn sub(self, rhs: Object) -> Result<Object, Error> {
        let value = match (self, rhs) {
            (Object::Int(x), Object::Int(y)) => x
                .checked_sub(y)
                .ok_or(Error::BinaryOperation("-", x.into(), y.into()))?
                .into(),
            (Object::Float(x), Object::Float(y)) => (x - y).into(),
            (Object::Node(x), Object::Node(y)) => Node::sub(x, y).map_err(Error::Any)?.into(),
            (lhs, rhs) => return Err(Error::BinaryOperation("-", lhs, rhs)),
        };
        Ok(value)
    }

    fn mul(self, rhs: Object) -> Result<Object, Error> {
        let value = match (self, rhs) {
            (Object::Int(x), Object::Int(y)) => x
                .checked_mul(y)
                .ok_or(Error::BinaryOperation("*", x.into(), y.into()))?
                .into(),
            (Object::Float(x), Object::Float(y)) => (x * y).into(),
            (Object::Node(x), Object::Node(y)) => Node::mul(x, y).map_err(Error::Any)?.into(),
            (lhs, rhs) => return Err(Error::BinaryOperation("*", lhs, rhs)),
        };
        Ok(value)
    }

    fn div(self, rhs: Object) -> Result<Object, Error> {
        let value = match (self, rhs) {
            (Object::Int(x), Object::Int(y)) => x
                .checked_div(y)
                .ok_or(Error::BinaryOperation("/", x.into(), y.into()))?
                .into(),
            (Object::Float(x), Object::Float(y)) => (x / y).into(),
            (Object::Node(x), Object::Node(y)) => Node::div(x, y).map_err(Error::Any)?.into(),
            (lhs, rhs) => return Err(Error::BinaryOperation("/", lhs, rhs)),
        };
        Ok(value)
    }

    fn rem(self, rhs: Object) -> Result<Object, Error> {
        let value = match (self, rhs) {
            (Object::Int(x), Object::Int(y)) => (x % y).into(),
            (lhs, rhs) => return Err(Error::BinaryOperation("%", lhs, rhs)),
        };
        Ok(value)
    }

    fn matmul(self, rhs: Object) -> Result<Object, Error> {
        let value = match (self, rhs) {
            (Object::Node(lhs), Object::Node(rhs)) => {
                Node::matmul(lhs, rhs).map_err(Error::Any)?.into()
            }
            (lhs, rhs) => return Err(Error::BinaryOperation("@", lhs, rhs)),
        };
        Ok(value)
    }

    fn not(self) -> Result<Object, Error> {
        let value = match self {
            Object::Bool(x) => (!x).into(),
            other => return Err(Error::UnaryOperation("not", other)),
        };
        Ok(value)
    }

    fn pos(self) -> Result<Object, Error> {
        if matches!(self, Object::Int(_) | Object::Float(_) | Object::Node(_)) {
            Ok(self.clone())
        } else {
            Err(Error::UnaryOperation("+", self))
        }
    }

    fn neg(self) -> Result<Object, Error> {
        let value = match self {
            Object::Int(x) => (-x).into(),
            Object::Float(x) => (-x).into(),
            Object::Node(x) => Node::neg(x).map_err(Error::Any)?.into(),
            other => return Err(Error::UnaryOperation("-", other)),
        };
        Ok(value)
    }
}

impl From<f32> for Object {
    fn from(x: f32) -> Object {
        Object::Float(x)
    }
}

impl From<i32> for Object {
    fn from(x: i32) -> Object {
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

impl From<Rc<Node>> for Object {
    fn from(value: Rc<Node>) -> Self {
        Object::Node(value)
    }
}
