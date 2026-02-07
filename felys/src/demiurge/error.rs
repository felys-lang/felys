use crate::utils::function::Const;
use std::fmt::{Display, Formatter};

pub enum Error {
    BinaryOperation(&'static str, Const, Const),
    UnaryOperation(&'static str, Const),
    ConstantType(Const, &'static str),
    ExitBlockUnreachable,
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        let mut msg = "Demiurge: ".to_string();
        match value {
            Error::BinaryOperation(op, lhs, rhs) => {
                let s = format!("cannot apply `{op}` to `{lhs}` and `{rhs}`");
                msg.push_str(&s);
            }
            Error::UnaryOperation(op, src) => {
                let s = format!("cannot apply `{op}` to `{src}`");
                msg.push_str(&s);
            }
            Error::ConstantType(c, ty) => {
                let s = format!("expecting `{c}` to be `{ty}`");
                msg.push_str(&s);
            }
            Error::ExitBlockUnreachable => {
                msg.push_str("infinite loop detected");
            }
        }
        msg.push('\n');
        msg
    }
}

impl Display for Const {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Const::Int(x) => write!(f, "{}", x),
            Const::Float(x) => write!(f, "{}", f32::from_bits(*x)),
            Const::Bool(x) => write!(f, "{}", x),
            Const::Str(x) => write!(f, "\"{}\"", x),
        }
    }
}
