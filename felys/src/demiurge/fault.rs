use crate::utils::ir::Const;
use std::fmt::{Display, Formatter};

pub enum Fault {
    BinaryOperation(&'static str, Const, Const),
    UnaryOperation(&'static str, Const),
    ConstantType(Const, &'static str),
}

impl Fault {
    pub fn recover(self) -> String {
        let mut msg = "Demiurge: ".to_string();
        match self {
            Fault::BinaryOperation(op, lhs, rhs) => {
                let s = format!("cannot apply `{op}` to `{lhs}` and `{rhs}`");
                msg.push_str(&s);
            }
            Fault::UnaryOperation(op, src) => {
                let s = format!("cannot apply `{op}` to `{src}`");
                msg.push_str(&s);
            }
            Fault::ConstantType(c, ty) => {
                let s = format!("expecting `{c}` to be `{ty}`");
                msg.push_str(&s);
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
            Const::Float(x) => write!(f, "{}", f64::from_bits(*x)),
            Const::Bool(x) => write!(f, "{}", x),
            Const::Str(x) => write!(f, "\"{}\"", x),
        }
    }
}
