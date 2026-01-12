use crate::cyrene::Group;
use crate::elysia::Object;

pub enum Fault {
    DataType(Object, &'static str),
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
            Fault::Internal => msg.push_str("internal error"),
        }
        msg.push('\n');
        msg
    }
}
