use LexingError::*;

use crate::Error;

pub enum LexingError {
    StringNotClosed { s: String },
    UnknownBinoptr { s: String },
    InvalidChar { c: char },
}

impl From<LexingError> for Error {
    fn from(value: LexingError) -> Self {
        let msg = match value {
            StringNotClosed { s } => format!("string `{}` is not closed", s),
            UnknownBinoptr { s } => format!("operator `{}` is unknown", s),
            InvalidChar { c } => format!("character `{}` is not valid", c),
        };
        Self { msg: format!("LexingError: {}", msg) }
    }
}
