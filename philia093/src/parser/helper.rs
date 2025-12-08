use crate::parser::Packrat;

impl Packrat {
    pub fn eof(&mut self) -> Option<()> {
        loop {
            if self.WS().is_none() {
                break;
            }
        }
        if self.__stream.next().is_none() {
            Some(())
        } else {
            None
        }
    }
}
