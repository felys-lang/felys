use crate::ast::Ident;
use crate::parser::Packrat;
use crate::program::Program;

impl Packrat {
    pub fn parse(mut self) -> Result<Program, String> {
        let result = self.grammar();
        if let Some((loc, msg)) = self.__snapshot {
            return Err(format!("expect {msg} @ {loc}"));
        }
        let grammar = result.ok_or("unknown parser error".to_string())?;
        let program = Program::new(grammar, self.__intern);
        Ok(program)
    }

    pub fn ident(&mut self) -> Option<Ident> {
        let id = self.IDENT()?;
        let ident = self.__intern.get(&id).unwrap();
        if self.__keywords.contains(ident) {
            None
        } else {
            Some(Ident(id))
        }
    }

    pub fn eof(&mut self) -> Option<()> {
        self.__stream.trim();
        if self.__stream.next().is_none() {
            Some(())
        } else {
            None
        }
    }
}
