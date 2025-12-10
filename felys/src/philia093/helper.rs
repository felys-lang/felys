use crate::ast::Ident;
use crate::philia093::PhiLia093;

impl PhiLia093 {
    pub fn n2i(&mut self) -> Option<Ident> {
        let id = self.NAME()?;
        let ident = self.__intern.get(&id).unwrap();
        if self.__keywords.contains(ident) {
            None
        } else {
            Some(Ident(id))
        }
    }
}
