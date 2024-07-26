use std::fmt::{Display, Formatter};

pub use lexing::*;
pub use runtime::*;
pub use syntax::*;

mod lexing;
mod syntax;
mod runtime;

pub struct Error {
    msg: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
