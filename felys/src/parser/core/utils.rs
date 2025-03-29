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
}
