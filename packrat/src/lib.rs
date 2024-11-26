mod memo;
mod stream;
mod pool;

pub use crate::memo::Memo;
pub use crate::pool::Pool;
pub use crate::stream::Stream;

pub use helper::*;

pub struct Parser<R> {
    pub memo: Memo<R>,
    pub pool: Pool,
    pub stream: Stream,
    pub cut: bool,
}

impl<R> Parser<R> {
    pub fn new(code: String) -> Self {
        Self {
            memo: Memo {
                body: Default::default()
            },
            pool: Pool {
                body: Default::default(),
                fast: vec![],
            },
            stream: Stream {
                body: code,
                strict: false,
                cursor: 0,
            },
            cut: false,
        }
    }

    pub fn alter<T, F>(&mut self, f: F) -> Option<Option<T>>
    where
        F: Fn(&mut Parser<R>) -> Option<T>,
    {
        let mode = self.stream.strict;
        let pos = self.stream.cursor;

        let result = f(self);
        let cut = self.cut;

        self.cut = false;
        self.stream.strict = mode;
        if result.is_none() {
            self.stream.cursor = pos;
        }

        if cut || result.is_some() {
            Some(result)
        } else {
            None
        }
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