use std::fmt::{Display, Formatter};

use crate::{Context, Output};
use crate::error::RuntimeError;
use crate::flow::Block;

/// Union type of different data types
#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
    Rust(fn(&mut Context) -> Output),
    
    #[doc(hidden)]
    Function { 
        args: Vec<String>, 
        body: Block 
    },
}


impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Number(value) => write!(f, "{}", value),
            Object::String(value) => write!(f, "\"{}\"", value),
            Object::Boolean(value) => write!(f, "{}", value),
            Object::None => write!(f, "none"),
            Object::Function { args, body } => 
                write!(f, "|{}| {:?}", args.join(", "), body),
            Object::Rust(func) => write!(f, "{:?}", func)
        }
    }
}


impl Object {
    pub fn f64(&self) -> Result<f64, RuntimeError> {
        if let Object::Number(value) = self {
            Ok(*value)
        } else {
            Err(RuntimeError::NoF64Convertion { s: self.to_string() })
        }
    }

    pub fn bool(&self) -> bool {
        match self {
            Object::Number(value) => *value != 0.0,
            Object::String(value) => !value.is_empty(),
            Object::Boolean(value) => *value,
            _ => true
        }
    }
}
