use crate::error::Error;
use crate::Language;
use crate::lexer::cursor::Cursor;
pub use crate::lexer::token::*;

mod token;
mod cursor;

pub fn tokenize(c: String, lang: &Language) -> Result<Vec<Token>, Error> {
    let mut cursor = Cursor {
        chars: c.chars().peekable(),
        buffer: Vec::new(),
    };

    while let Some(tk) = cursor.scan_next(lang) {
        cursor.buffer.push(tk?)
    }

    cursor.buffer.reverse();
    Ok(cursor.buffer)
}
