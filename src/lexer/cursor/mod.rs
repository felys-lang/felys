mod chn;
mod eng;
mod shared;

use std::iter::Peekable;
use std::str::Chars;
use crate::lexer::token::*;
use crate::error::LexingError;
use crate::Language;
use crate::Language::*;

pub struct Cursor<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub buffer: Vec<Token>,
}


impl Cursor<'_> {
    pub(super) fn scan_next(&mut self, lang: &Language) -> Option<Result<Token, LexingError>> {
        self.skip_spaces();
        let ch = match self.chars.peek() {
            Some(ch) => ch,
            None => return None
        };
        
        let token = match (lang, ch) {
            (_, '0'..='9' | '.') => self.scan_number(),
            (_, '*' | '/' | '%') => self.scan_simple_binoptr(),
            (_, '+' | '-') => self.scan_additive_optr(),
            (_, '>' | '<' | '=' | '!') => self.scan_comparative_optr(),
            (ENG, '\'' | '"') => self.scan_eng_string(),
            (CHN, '‘' | '“') => self.scan_chn_string(),
            (ENG, 'a'..='z' | 'A'..='Z' | '_') => self.scan_eng_ident(),
            (CHN, '\u{4e00}'..='\u{9fa5}' | '—') => self.scan_chn_ident(),
            (ENG, '(' | ')' | '{' | '}' | '|' | ';' | ',') => self.scan_eng_symbol(),
            (CHN, '（' | '）' | '「' | '」' | '｜' | '；' | '，') => self.scan_chn_symbol(),
            _ => Err(LexingError::InvalidChar { c: *ch })
        };
        Some(token)
    }
}
