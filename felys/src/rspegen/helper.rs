use crate::ast::Chunk;
use crate::rspegen::Packrat;

impl Packrat {
    pub fn ident(&mut self) -> Option<usize> {
        let id = self.IDENT()?;
        let ident = self.intern.get(&id).unwrap();
        if self.keywords.contains(ident) {
            None
        } else {
            Some(id)
        }
    }
    
    pub fn eof(&mut self) -> Option<()> {
        self.stream.trim();
        if self.stream.next().is_none() {
            Some(())
        } else {
            None
        }
    }
}
