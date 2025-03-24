use crate::ast::pat::{Ident, Pat};
use crate::parser::Parser;

impl Parser {
    #[felysian::memoize]
    pub fn pat(&mut self) -> Option<Pat> {
        if let Some(res) = self.alter(|x| {
            x.keyword("_")?;
            Some(Pat::Any)
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let body = x.ident()?;
            Some(Pat::Ident(body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.expect("(")?;
            let first = x.pat()?;
            x.expect(",")?;
            let second = x.pat()?;
            let mut body = vec![first, second];
            while x.expect(",").is_some() {
                let pat = x.pat()?;
                body.push(pat)
            }
            x.expect(")")?;
            Some(Pat::Tuple(body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let body = x.lit()?;
            Some(Pat::Lit(body))
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
