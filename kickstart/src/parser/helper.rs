use crate::parser::Packrat;

impl Packrat {
    pub fn eof(&mut self) -> Option<()> {
        self.stream.trim();
        if self.stream.next().is_none() {
            Some(())
        } else {
            None
        }
    }
}
