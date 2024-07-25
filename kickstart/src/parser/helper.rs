use crate::parser::Packrat;

impl Packrat {
    pub fn eof(&mut self) -> Option<()> {
        self.__stream.trim();
        if self.__stream.next().is_none() {
            Some(())
        } else {
            None
        }
    }
}
