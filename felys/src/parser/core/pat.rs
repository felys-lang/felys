use crate::ast::common::Ident;
use crate::ast::pat::Pat;
use crate::parser::Parser;

impl Parser {
    #[felysium::memoize]
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
}
