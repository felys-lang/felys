pub enum Fault {
    InvalidOperation,
    ValueUnreachable,
}

impl Fault {
    pub fn recover(self) -> String {
        String::new()
    }
}
