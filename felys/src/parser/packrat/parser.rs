use crate::parser::packrat::intern::Intern;
use crate::parser::packrat::memo::Memo;
use crate::parser::packrat::stream::Stream;

pub struct Parser {
    memo: Memo,
    stream: Stream,
    intern: Intern,
}

impl Parser {
    pub fn alter<T, F>(&mut self, f: F) -> Option<Option<T>>
    where
        F: Fn(&mut Parser) -> Option<T>,
    {
        let mode = self.stream.strict;
        let pos = self.stream.cursor;

        let result = f(self);

        self.stream.strict = mode;
        if result.is_none() {
            self.stream.cursor = pos;
        }

        if result.is_some() { Some(result) } else { None }
    }

    pub fn expect(&mut self, s: &'static str) -> Option<&'static str> {
        if let Some(res) = self.alter(|x| {
            x.stream.trim();
            x.stream.strict = true;
            s.chars().all(|c| x.stream.next() == Some(c)).then_some(s)
        }) {
            return res;
        }
        None
    }
    pub fn scan(&mut self, filter: fn(char) -> bool) -> Option<char> {
        let cur = self.stream.cursor;
        let saw = self.stream.next()?;
        if filter(saw) {
            Some(saw)
        } else {
            self.stream.cursor = cur;
            None
        }
    }

    pub fn lookahead(&mut self, filter: fn(char) -> bool) -> Option<char> {
        let cur = self.stream.cursor;
        let saw = self.stream.next().unwrap_or('\0');
        self.stream.cursor = cur;
        if filter(saw) { Some(saw) } else { None }
    }
}
