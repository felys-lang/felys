use crate::ast::Grammar;
use crate::parser::{Intern, Packrat};

impl Packrat {
    pub fn parse(mut self) -> Result<(Grammar, Intern), String> {
        let result = self.grammar();
        if let Some((loc, msg)) = self.snapshot {
            return Err(format!("{msg} @ {loc}"));
        }
        Ok((result.ok_or("unknown".to_string())?, self.intern))
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
