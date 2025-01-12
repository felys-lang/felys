use crate::ast::Program;
use crate::packrat::Parser;
use crate::parser::registry::{Base, Entry, Helper, Statement, CR};

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

    fn eof(&mut self) -> Option<char> {
        if let Some(res) = self.alter(|x| {
            x.lookahead(|c| c == '\0')
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
                x.memo.clear();
                body.push(stmt)
            }
            x.e("not parsed to the end").eof()?;
            Some(Program(body))
        }) {
            return res;
        }
        None
    }
}