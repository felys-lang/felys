use crate::registry::{Base, Helper, CR};
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