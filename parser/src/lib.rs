use crate::registry::{Entry, CR};
use ast::Program;
use packrat::{Parser, Pool};
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

pub fn parse(code: String) -> Option<(Program, Pool)> {
    let keywords = HashSet::from(KEYWORDS);
    let mut parser = Parser::<CR>::new(code, keywords);
    let program = parser.program()?;
    Some((program, parser.pool))
}