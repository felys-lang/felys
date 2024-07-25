use crate::Error;
use SyntaxError::*;

pub enum SyntaxError {
    EndOfTokenSteam,
    IncompleteCall,
    IncompleteFunc,
    TokenNotPrimary { s: String },
    TokenNotALiteral { s: String },
    InvalidArgs { s: String },
    EatWrongToken { s: String }
}

impl From<SyntaxError> for Error {
    fn from(value: SyntaxError) -> Self {
        let msg = match value {
            EndOfTokenSteam => "no more token to parse".to_string(),
            IncompleteCall => "expect `,` or `)` to call a function".to_string(),
            IncompleteFunc => "expect `,` or `|` to define a function".to_string(),
            TokenNotPrimary { s } => format!("token `{}` is not primary", s),
            TokenNotALiteral { s } => format!("token `{}` is not a literal", s),
            InvalidArgs { s } => format!("token `{}` cannot be a function arg", s),
            EatWrongToken { s } => format!("token `{}` is unexpected", s)
        };
        Self { msg: format!("SyntaxError: {}", msg) }
    }
}
