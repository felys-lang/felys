use crate::elysia::runtime::object::Object;
use crate::elysia::runtime::vm::DEPTH;

pub enum Error {
    DataType(Object, &'static str),
    BinaryOperation(&'static str, Object, Object),
    UnaryOperation(&'static str, Object),
    NumArgsNotMatch(usize, usize),
    IndexOutOfBounds(Object, i32),
    NotEnoughToUnpack(Object, u32),
    StackOverflow,
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        let mut msg = "Elysia: ".to_string();
        match value {
            Error::DataType(obj, ty) => {
                let s = format!("expecting `{obj}` to be `{ty}`");
                msg.push_str(&s);
            }
            Error::BinaryOperation(op, lhs, rhs) => {
                let s = format!("cannot apply `{op}` to `{lhs}` and `{rhs}`");
                msg.push_str(&s);
            }
            Error::UnaryOperation(op, src) => {
                let s = format!("cannot apply `{op}` to `{src}`");
                msg.push_str(&s);
            }
            Error::NumArgsNotMatch(expected, args) => {
                let s = format!("expected {expected} arguments, got {args}");
                msg.push_str(&s);
            }
            Error::IndexOutOfBounds(obj, index) => {
                let s = format!("index {index} is out of boundaries for `{obj}`");
                msg.push_str(&s);
            }
            Error::NotEnoughToUnpack(obj, index) => {
                let s = format!("cannot unpack element at index {index} for `{obj}`");
                msg.push_str(&s);
            }
            Error::StackOverflow => {
                let s = format!("stack overflow, max depth set to {DEPTH}");
                msg.push_str(&s);
            }
        }
        msg.push('\n');
        msg
    }
}
