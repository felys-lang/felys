use crate::ast::Program;
use crate::packrat::{Intern, Parser};
use crate::parser::registry::{Entry, CR};
use std::collections::HashSet;

mod registry;
mod core;
mod helper;

const KEYWORDS: [&str; 15] = [
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
    "false",
    "and",
    "or",
    "not"
];

pub fn parse(code: String) -> Result<(Program, Intern), &'static str> {
    let keywords = HashSet::from(KEYWORDS);
    let mut parser = Parser::<CR>::new(code, keywords);
    if let Some(prog) = parser.program() {
        Ok((prog, parser.intern))
    } else if let Some(msg) = parser.error {
        Err(msg)
    } else {
        Err("uncaught")
    }
}