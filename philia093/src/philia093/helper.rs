use crate::philia093::PhiLia093;

impl PhiLia093 {
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
