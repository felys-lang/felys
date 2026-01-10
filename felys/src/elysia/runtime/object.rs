use crate::error::Fault;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Object {
    Pointer(Pointer, usize),
    List(Rc<[Object]>),
    Tuple(Rc<[Object]>),
    Group(usize, Rc<[Object]>),
    Str(Rc<str>),
    Int(isize),
    Float(f64),
    Bool(bool),
    Void,
}

#[derive(Clone, Debug)]
pub enum Pointer {
    Function,
    Group,
}

impl Object {
    pub fn list(&self) -> Result<Rc<[Object]>, Fault> {
        if let Object::List(x) = self {
            Ok(x.clone())
        } else {
            Err(Fault::Runtime)
        }
    }

    pub fn group(&self) -> Result<(usize, Rc<[Object]>), Fault> {
        if let Object::Group(x, elements) = self {
            Ok((*x, elements.clone()))
        } else {
            Err(Fault::Runtime)
        }
    }

    pub fn pointer(&self) -> Result<(Pointer, usize), Fault> {
        if let Object::Pointer(ty, idx) = self {
            Ok((ty.clone(), *idx))
        } else {
            Err(Fault::Runtime)
        }
    }

    pub fn bool(&self) -> Result<bool, Fault> {
        if let Object::Bool(x) = self {
            Ok(*x)
        } else {
            Err(Fault::Runtime)
        }
    }

    pub fn int(&self) -> Result<isize, Fault> {
        if let Object::Int(x) = self {
            Ok(*x)
        } else {
            Err(Fault::Runtime)
        }
    }
}
