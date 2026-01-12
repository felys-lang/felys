use crate::cyrene::Group;
use crate::elysia::Object;

pub enum Fault {
    DataType(Object, &'static str),
    BinaryOperation(&'static str, Object, Object),
    UnaryOperation(&'static str, Object),
    CallableNotExist(usize),
    BytecodeNotExist(usize),
    RegisterNotExist(usize),
    NumArgsNotMatch(usize, Vec<Object>),
    Internal,
}

impl Fault {
    pub fn recover(self, groups: &[Group]) -> String {
        let mut msg = "Elysia: ".to_string();
        match self {
            Fault::DataType(obj, ty) => {
                msg.push_str("expecting `");
                obj.recover(&mut msg, 0, groups).unwrap();
                msg.push_str("` to be `");
                msg.push_str(ty);
                msg.push('`');
            }
            Fault::BinaryOperation(op, lhs, rhs) => {
                msg.push_str("cannot apply `");
                msg.push_str(op);
                msg.push_str("` to `");
                lhs.recover(&mut msg, 0, groups).unwrap();
                msg.push_str("` and `");
                rhs.recover(&mut msg, 0, groups).unwrap();
                msg.push('`');
            }
            Fault::UnaryOperation(op, src) => {
                msg.push_str("cannot apply `");
                msg.push_str(op);
                msg.push_str("` to `");
                src.recover(&mut msg, 0, groups).unwrap();
                msg.push('`');
            }
            Fault::CallableNotExist(idx) => {
                let s = format!("callable at index `{}` does not exist", idx);
                msg.push_str(&s);
            }
            Fault::BytecodeNotExist(idx) => {
                let s = format!("bytecode at index `{}` does not exist", idx);
                msg.push_str(&s);
            }
            Fault::RegisterNotExist(idx) => {
                let s = format!("register at index `{}` does not exist", idx);
                msg.push_str(&s);
            }
            Fault::NumArgsNotMatch(expected, args) => {
                let s = format!("expected `{}` arguments, but got (", expected);
                msg.push_str(&s);
                let mut iter = args.iter();
                if let Some(first) = iter.next() {
                    first.recover(&mut msg, 0, groups).unwrap();
                }
                for arg in iter {
                    msg.push_str(", ");
                    arg.recover(&mut msg, 0, groups).unwrap();
                }
                msg.push(')');
            }
            Fault::Internal => msg.push_str("internal error"),
        }
        msg.push('\n');
        msg
    }
}
