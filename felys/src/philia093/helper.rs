use crate::cyrene::Cyrene;
use crate::error::Fault;
use crate::philia093::PhiLia093;

impl PhiLia093 {
    pub fn parse(mut self) -> Result<Cyrene, Fault> {
        let root = self.root().ok_or(Fault::FailedToParse)?;
        Ok(Cyrene {
            root,
            intern: self.__intern,
        })
    }

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
