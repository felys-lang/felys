use crate::Error;
use LexingError::*;

pub enum LexingError {
    EndOfCharStream,
    NumberJustADot,
    InvalidString { c: char },
    StringNotClosed { s: String },
    UnknownBinoptr { s: String },
    UnknownSymbol { c: char },
    InvalidChar { c: char }
}

impl From<LexingError> for Error {
    fn from(value: LexingError) -> Self {
        let msg = match value {
            EndOfCharStream => "no more characters to parse".to_string(),
            NumberJustADot => "number `.` is not valid".to_string(),
            InvalidString { c } => format!("char `{}` cannot start a string", c),
            StringNotClosed{ s } => format!("string `{}` is not closed", s),
            UnknownBinoptr { s } => format!("operator `{}` is unknown", s),
            UnknownSymbol { c } => format!("symbol `{}` is unknown", c),
            InvalidChar { c } => format!("character `{}` is not valid", c),
        };
        Self { msg: format!("LexingError: {}", msg) }
    }
}
