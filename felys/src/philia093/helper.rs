use crate::ast::Ident;
use crate::philia093::PhiLia093;

impl PhiLia093 {
    pub fn ident(&mut self) -> Option<Ident> {
        loop {
            if self.T().is_none() {
                break;
            }
        }
        let id = self.IDENT()?;
        let ident = self.__intern.get(&id).unwrap();
        if self.__keywords.contains(ident) {
            None
        } else {
            Some(Ident(id))
        }
    }

    pub fn eof(&mut self) -> Option<()> {
        loop {
            if self.T().is_none() {
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
