use crate::ast::lit::{Bool, Float, Int, Lit, Str};
use crate::packrat::Parser;
use crate::parser::registry::{Helper, Literal, CR};

impl Literal for Parser<CR> {
    #[helper::memoize]
    fn lit(&mut self) -> Option<Lit> {
        if let Some(res) = self.alter(|x| {
            let body = x.float()?;
            Some(Lit::Float(body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let body = x.int()?;
            Some(Lit::Int(body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let body = x.str()?;
            Some(Lit::Str(body))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let body = x.bool()?;
            Some(Lit::Bool(body))
        }) {
            return res;
        }
        None
    }

    fn int(&mut self) -> Option<Int> {
        if let Some(res) = self.alter(|x| {
            x.expect("0x")?;
            x.stream.strict = true;
            x.cut = true;
            let first = x.scan(|c| c.is_ascii_hexdigit())?;
            let mut body = String::from(first);
            while let Some(more) = x.scan(|c| c.is_ascii_hexdigit()) {
                body.push(more)
            }
            let symbol = x.pool.id(body);
            Some(Int::Base16(symbol.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.expect("0o")?;
            x.stream.strict = true;
            x.cut = true;
            let first = x.scan(|c| matches!(c, '0'..='7'))?;
            let mut body = String::from(first);
            while let Some(more) = x.scan(|c| matches!(c, '0'..='7')) {
                body.push(more)
            }
            let symbol = x.pool.id(body);
            Some(Int::Base8(symbol.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.expect("0b")?;
            x.stream.strict = true;
            x.cut = true;
            let first = x.scan(|c| matches!(c, '0'|'1'))?;
            let mut body = String::from(first);
            while let Some(more) = x.scan(|c| matches!(c, '0'|'1')) {
                body.push(more)
            }
            let symbol = x.pool.id(body);
            Some(Int::Base2(symbol.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.expect("0")?;
            x.cut = true;
            x.lookahead(|c| !c.is_ascii_digit())?;
            let body = String::from("0");
            let symbol = x.pool.id(body);
            Some(Int::Base10(symbol.into()))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let first = x.scan(|c| c.is_ascii_digit())?;
            x.stream.strict = true;
            let mut body = String::from(first);
            while let Some(more) = x.scan(|c| c.is_ascii_digit()) {
                body.push(more)
            }
            let symbol = x.pool.id(body);
            Some(Int::Base10(symbol.into()))
        }) {
            return res;
        }
        None
    }

    fn float(&mut self) -> Option<Float> {
        if let Some(res) = self.alter(|x| {
            x.expect("0")?;
            x.stream.strict = true;
            x.cut = true;
            x.expect(".")?;
            let first = x.scan(|c| c.is_ascii_digit())?;
            let mut body = format!("0.{}", first);
            while let Some(x) = x.scan(|c| c.is_ascii_digit()) {
                body.push(x)
            }
            let symbol = x.pool.id(body);
            Some(symbol.into())
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let first = x.scan(|c| c.is_ascii_digit())?;
            x.stream.strict = true;
            let mut body = String::from(first);
            while let Some(x) = x.scan(|c| c.is_ascii_digit()) {
                body.push(x)
            }
            x.expect(".")?;
            body.push('.');
            let first = x.scan(|c| c.is_ascii_digit())?;
            body.push(first);
            while let Some(x) = x.scan(|c| c.is_ascii_digit()) {
                body.push(x)
            }
            let symbol = x.pool.id(body);
            Some(symbol.into())
        }) {
            return res;
        }
        None
    }

    fn bool(&mut self) -> Option<Bool> {
        if let Some(res) = self.alter(|x| {
            x.keyword("true")?;
            Some(Bool::True)
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("false")?;
            Some(Bool::False)
        }) {
            return res;
        }
        None
    }

    fn str(&mut self) -> Option<Str> {
        if let Some(res) = self.alter(|x| {
            x.expect("\"")?;
            x.stream.strict = true;
            let mut body = String::new();
            while let Some(ch) = x.scan(|c| c != '"') {
                body.push(ch)
            }
            x.expect("\"")?;
            let symbol = x.pool.id(body);
            Some(symbol.into())
        }) {
            return res;
        }
        None
    }
}