use crate::ast::Program;
use crate::packrat::{Parser, Intern};
use crate::parser::registry::{Entry, CR};
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

pub fn parse(code: String) -> Result<(Program, Intern), &'static str> {
    let keywords = HashSet::from(KEYWORDS);
    let mut parser = Parser::<CR>::new(code, keywords);
    match parser.program() {
        Some(prog) => Ok((prog, parser.intern)),
        None => Err("syntax error")
    }
}