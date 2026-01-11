pub enum Fault {
    InvalidOperation,
    ValueUnreachable,
}

impl Fault {
    pub fn recover(self) -> String {
        let mut msg = "Demiurge: ".to_string();
        match self {
            Fault::InvalidOperation => msg.push_str("invalid operation"),
            Fault::ValueUnreachable => msg.push_str("value unreachable"),
        }
        msg
    }
}
