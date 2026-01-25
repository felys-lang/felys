use crate::elysia::Object;
use crate::utils::group::Group;

pub enum Fault {
    DataType(Object, &'static str),
    BinaryOperation(&'static str, Object, Object),
    UnaryOperation(&'static str, Object),
    NumArgsNotMatch(usize, Vec<Object>),
    IndexOutOfBounds(Object, isize),
    NotEnoughToUnpack(Object, usize),
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
            Fault::NumArgsNotMatch(expected, args) => {
                let s = format!("expected {} arguments, got {}: [", expected, args.len());
                msg.push_str(&s);
                let mut iter = args.iter();
                if let Some(first) = iter.next() {
                    msg.push('`');
                    first.recover(&mut msg, 0, groups).unwrap();
                    msg.push('`');
                }
                for arg in iter {
                    msg.push_str(", `");
                    arg.recover(&mut msg, 0, groups).unwrap();
                    msg.push('`');
                }
                msg.push(']');
            }
            Fault::IndexOutOfBounds(obj, index) => {
                let s = format!("index {} is out of boundaries for `", index);
                msg.push_str(&s);
                obj.recover(&mut msg, 0, groups).unwrap();
                msg.push('`');
            }
            Fault::NotEnoughToUnpack(obj, index) => {
                let s = format!("cannot unpack element at index {} for `", index);
                msg.push_str(&s);
                obj.recover(&mut msg, 0, groups).unwrap();
                msg.push('`');
            }
        }
        msg.push('\n');
        msg
    }
}
