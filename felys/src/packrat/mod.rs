pub use crate::packrat::memo::Memo;
pub use crate::packrat::intern::Intern;
pub use crate::packrat::stream::Stream;
use std::collections::HashSet;

mod memo;
mod stream;
mod intern;


pub struct Parser<R> {
    pub memo: Memo<R>,
    pub intern: Intern,
    pub stream: Stream,
    pub error: Option<(usize, &'static str)>
}

impl<R> Parser<R> {
    pub fn new(code: String, keywords: HashSet<&'static str>) -> Self {
        Self {
            memo: Memo {
                body: Default::default()
            },
            intern: Intern {
                body: Default::default(),
                fast: vec![],
                keywords,
            },
            stream: Stream {
                body: code,
                strict: false,
                cursor: 0,
            },
            error: None
        }
    }

    pub fn alter<T, F>(&mut self, f: F) -> Option<Option<T>>
    where
        F: Fn(&mut Parser<R>) -> Option<T>,
    {
        if self.error.is_some() {
            return Some(None)
        }
        
        let mode = self.stream.strict;
        let pos = self.stream.cursor;

        let result = f(self);
        
        self.stream.strict = mode;
        if result.is_none() {
            self.stream.cursor = pos;
        }

        if result.is_some() {
            self.error = None;
            Some(result)
        } else {
            None
        }
    }

    pub fn err(&mut self, msg: &'static str) -> &mut Self {
        self.error = Some((self.stream.cursor, msg));
        self
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
        let pos = self.stream.cursor;
        let saw = self.stream.next()?;
        if filter(saw) {
            Some(saw)
        } else {
            self.stream.cursor = pos;
            None
        }
    }

    pub fn lookahead(&mut self, filter: fn(char) -> bool) -> Option<char> {
        let pos = self.stream.cursor;
        let saw = self.stream.next().unwrap_or('\0');
        self.stream.cursor = pos;
        if filter(saw) {
            Some(saw)
        } else {
            None
        }
    }
}