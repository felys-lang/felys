use crate::ast::common::{Block, Ident, Program};
use crate::parser::Parser;

impl Parser {
    pub fn program(&mut self) -> Option<Program> {
        if let Some(res) = self.alter(|x| {
            let mut body = Vec::new();
            while let Some(stmt) = x.stmt() {
                x.memo.clear();
                body.push(stmt)
            }
            x.eof()?;
            Some(Program(body))
        }) {
            return res;
        }
        None
    }

    pub fn block(&mut self) -> Option<Block> {
        if let Some(res) = self.alter(|x| {
            x.expect("{")?;
            let mut body = Vec::new();
            while let Some(stmt) = x.stmt() {
                body.push(stmt)
            }
            x.expect("}")?;
            Some(Block(body))
        }) {
            return res;
        }
        None
    }

    pub fn ident(&mut self) -> Option<Ident> {
        if let Some(res) = self.alter(|x| {
            let first = x.scan(|c| c.is_ascii_alphabetic() || c == '_')?;
            x.stream.strict = true;
            let mut body = String::from(first);
            while let Some(ch) = x.scan(|c| c.is_ascii_alphanumeric() || c == '_') {
                body.push(ch)
            }
            if x.keywords.contains(body.as_str()) {
                return None;
            }
            let id = x.intern.id(body);
            Some(id.into())
        }) {
            return res;
        }
        None
    }
}
