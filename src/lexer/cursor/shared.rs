use crate::error::LexingError;
use crate::lexer::*;
use crate::lexer::cursor::Cursor;

impl Cursor<'_> {
    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.chars.peek() {
            if [' ', '\n', '\r'].contains(ch) {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    pub fn number(&mut self, first: char) -> Result<Token, LexingError> {
        let mut value = String::from(first);
        let mut dotted = false;
        while let Some(ch) = self.chars.peek() {
            match (ch, dotted) {
                ('.', true) => return Ok(Token::new(ValueType::Number.into(), value)),
                ('.', false) => dotted = true,
                ('0'..='9', _) => (),
                _ => break
            }
            value.push(*ch);
            self.chars.next();
        }
        Ok(Token::new(ValueType::Number.into(), value))
    }

    pub fn arithmetic(&mut self, first: char) -> Result<Token, LexingError> {
        let mut value = String::from(first);
        if let Some(eq) = self.chars.peek() {
            if *eq == '=' {
                value.push('=');
                self.chars.next();
            }
        }
        let binary = matches!(
            self.buffer.last(),
            Some(prev) if matches!(
                prev.kind,
                TokenType::Val(_) |
                TokenType::Identifier |
                TokenType::Sym(SymbolType::RParen)
            )
        );
        let tt = match (value.as_str(), binary) {
            ("+", true) => BinoptrType::Add.into(),
            ("-", true) => BinoptrType::Sub.into(),
            ("+", false) => UnaoptrType::Pos.into(),
            ("-", false) => UnaoptrType::Neg.into(),
            ("*", _) => BinoptrType::Mul.into(),
            ("/", _) => BinoptrType::Div.into(),
            ("%", _) => BinoptrType::Mod.into(),
            ("+=", _) => AssignType::Ade.into(),
            ("-=", _) => AssignType::Sue.into(),
            ("*=", _) => AssignType::Mue.into(),
            ("/=", _) => AssignType::Die.into(),
            ("%=", _) => AssignType::Moe.into(),
            _ => return Err(LexingError::UnknownBinoptr { s: value })
        };
        Ok(Token::new(tt, value))
    }
}
