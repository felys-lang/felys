use crate::registry::{Base, Entry, Helper, Statement, CR};
use ast::Program;
use packrat::Parser;

impl Base for Parser<CR> {
    type CR = CR;
}

impl Helper for Parser<CR> {
    fn keyword(&mut self, s: &'static str) -> Option<&'static str> {
        if let Some(res) = self.alter(|x| {
            x.expect(s)?;
            x.stream.strict = true;
            x.lookahead(|x| !x.is_ascii_alphanumeric())?;
            Some(s)
        }) {
            return res;
        }
        None
    }
}

impl Entry for Parser<CR> {
    fn program(&mut self) -> Option<Program> {
        if let Some(res) = self.alter(|x| {
            let mut body = Vec::new();
            while let Some(stmt) = x.stmt() {
                body.push(stmt)
            }
            Some(Program(body))
        }) {
            return res;
        }
        None
    }
}