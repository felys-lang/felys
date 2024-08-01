use std::iter::Peekable;
use std::str::Chars;

use crate::error::LexingError;
use crate::Language;
use crate::Language::*;
use crate::lexer::token::*;

mod zh;
mod en;
mod shared;

pub struct Cursor<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub buffer: Vec<Token>,
}


impl Cursor<'_> {
    pub(super) fn scan_next(&mut self, lang: &Language) -> Option<Result<Token, LexingError>> {
        self.skip_whitespace();
        match lang {
            EN => self.en(),
            ZH => self.zh()
        }
    }
}
