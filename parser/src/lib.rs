use crate::registry::{Statement, CR};
use ast::Program;
use packrat::{Parser, Pool};

mod registry;
mod core;
mod helper;

pub fn parse(code: String) -> Option<(Program, Pool)> {
    let mut parser = Parser::<CR>::new(code);
    let mut body = Vec::new();
    while let Some(stmt) = parser.stmt() {
        body.push(stmt)
    }
    if parser.stream.next().is_none() {
        Some((Program(body), parser.pool))
    } else {
        None
    }
}