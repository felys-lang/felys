use crate::registry::{Entry, CR};
use ast::Program;
use packrat::{Memo, Parser, Pool};
use std::collections::HashSet;

mod registry;
mod core;
mod helper;

const KEYWORDS: [&str; 12] = [
    "break",
    "continue",
    "for",
    "in",
    "match",
    "if",
    "else",
    "loop",
    "return",
    "while",
    "true",
    "false"
];

pub fn parse(code: String) -> Result<(Program, Pool), (Memo<CR>, Pool)> {
    let keywords = HashSet::from(KEYWORDS);
    let mut parser = Parser::<CR>::new(code, keywords);
    match parser.program() {
        Some(prog) => Ok((prog, parser.pool)),
        None => Err((parser.memo, parser.pool))
    }
}