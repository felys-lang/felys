mod ast;
mod nn;
mod parser;
mod program;
mod runtime;
mod rust;

type Fxx = f64;

pub use nn::matrix::Matrix;
pub use parser::Packrat;
pub use program::Config;
pub use program::Output;
