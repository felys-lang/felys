use crate::elysia::Object;

pub enum Fault {
    DataType(Object, &'static str),
    BinaryOperation(&'static str, Object, Object),
    UnaryOperation(&'static str, Object),
    NumArgsNotMatch(usize, usize),
    IndexOutOfBounds(Object, i32),
    NotEnoughToUnpack(Object, u32),
}

impl From<Fault> for String {
    fn from(value: Fault) -> Self {
        let mut msg = "Elysia: ".to_string();
        match value {
            Fault::DataType(obj, ty) => {
                let s = format!("expecting `{obj}` to be `{ty}`");
                msg.push_str(&s);
            }
            Fault::BinaryOperation(op, lhs, rhs) => {
                let s = format!("cannot apply `{op}` to `{lhs}` and `{rhs}`");
                msg.push_str(&s);
            }
            Fault::UnaryOperation(op, src) => {
                let s = format!("cannot apply `{op}` to `{src}`");
                msg.push_str(&s);
            }
            Fault::NumArgsNotMatch(expected, args) => {
                let s = format!("expected {expected} arguments, got {args}");
                msg.push_str(&s);
            }
            Fault::IndexOutOfBounds(obj, index) => {
                let s = format!("index {index} is out of boundaries for `{obj}`");
                msg.push_str(&s);
            }
            Fault::NotEnoughToUnpack(obj, index) => {
                let s = format!("cannot unpack element at index {index} for `{obj}`");
                msg.push_str(&s);
            }
        }
        msg.push('\n');
        msg
    }
}
