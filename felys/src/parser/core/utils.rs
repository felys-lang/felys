use crate::ast::program::Program;
use crate::parser::packrat::Parser;

impl Parser {
    pub fn keyword(&mut self, s: &'static str) -> Option<&'static str> {
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

    pub fn eof(&mut self) -> Option<char> {
        self.lookahead(|c| c == '\0')
    }

    pub fn program(&mut self) -> Option<Program> {
        if let Some(res) = self.alter(|x| {
            let mut body = Vec::new();
            while let Some(stmt) = x.stmt() {
                x.memo.clear();
                body.push(stmt)
            }
            x.eof()?;
            Some(Program(body))
        }) {
            return res;
        }
        None
    }
}
