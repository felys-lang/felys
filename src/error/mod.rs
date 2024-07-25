mod lexing;
mod syntax;
mod runtime;

use std::fmt::{Display, Formatter};
pub use lexing::*;
pub use syntax::*;
pub use runtime::*;


pub struct Error {
    msg: String
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
