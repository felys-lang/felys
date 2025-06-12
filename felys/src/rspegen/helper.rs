use crate::ast::Grammar;
use crate::rspegen::Packrat;

impl Packrat {
    pub fn parse(&mut self) -> Result<Grammar, String> {
        let result = self.grammar();
        if let Some((loc, msg)) = self.snapshot {
            return Err(format!("{} @ {}", msg, loc));
        }
        result.ok_or("unknown".to_string())
    }

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
