pub enum Fault {
    Internal,
}

impl Fault {
    pub fn recover(self) -> String {
        String::new()
    }
}
