use crate::philia093::PhiLia093;

impl PhiLia093 {
    pub fn n2i(&mut self) -> Option<usize> {
        let id = self.NAME()?;
        let ident = self.__intern.get(&id).unwrap();
        if (self.__keywords)(ident) {
            None
        } else {
            Some(id)
        }
    }
}
