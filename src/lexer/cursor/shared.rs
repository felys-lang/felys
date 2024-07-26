use crate::error::LexingError;
use crate::lexer::{AssignType, BinoptrType, SymbolType, Token, TokenType, UnaoptrType, ValueType};
use crate::lexer::cursor::Cursor;

impl Cursor<'_> {
    pub fn skip_spaces(&mut self) {
        while let Some(ch) = self.chars.peek() {
            if *ch == ' ' || *ch == '\n' || *ch == '\r' {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    pub fn scan_number(&mut self) -> Result<Token, LexingError> {
        let mut value = String::new();
        let mut dotted = false;
        while let Some(ch) = self.chars.peek() {
            if ch.is_ascii_digit() || *ch == '.' {
                match (*ch, dotted) {
                    ('.', true) => return Ok(Token::new(ValueType::Number.into(), value)),
                    ('.', false) => dotted = true,
                    _ => ()
                }
                value.push(*ch);
                self.chars.next();
            } else {
                break;
            }
        }

        if value.as_str() == "." {
            Err(LexingError::NumberJustADot)
        } else {
            Ok(Token::new(ValueType::Number.into(), value))
        }
    }

    pub fn scan_simple_binoptr(&mut self) -> Result<Token, LexingError> {
        let mut value = String::new();

        if let Some(ch) = self.chars.next() {
            value.push(ch);
        } else {
            return Err(LexingError::EndOfCharStream);
        }

        if let Some(eq) = self.chars.peek() {
            if *eq == '=' {
                value.push('=');
                self.chars.next();
            }
        }

        let ttype = match value.as_str() {
            "*" => BinoptrType::Mul.into(),
            "/" => BinoptrType::Div.into(),
            "%" => BinoptrType::Mod.into(),
            "*=" => AssignType::Mue.into(),
            "/=" => AssignType::Die.into(),
            "%=" => AssignType::Moe.into(),
            _ => return Err(LexingError::UnknownBinoptr { s: value })
        };

        Ok(Token::new(ttype, value))
    }

    pub fn scan_additive_optr(&mut self) -> Result<Token, LexingError> {
        let mut value = String::new();

        if let Some(ch) = self.chars.next() {
            value.push(ch);
        } else {
            return Err(LexingError::EndOfCharStream);
        }

        if let Some(eq) = self.chars.peek() {
            if *eq == '=' {
                value.push('=');
                self.chars.next();
            }
        }

        let binary = matches!(self.buffer.last(),
            Some(prev) if matches!(prev.kind,
                TokenType::Val(_) |
                TokenType::Identifier |
                TokenType::Sym(SymbolType::RParen)
            )
        );

        let tt = match (binary, value.as_str()) {
            (true, "+") => BinoptrType::Add.into(),
            (true, "-") => BinoptrType::Sub.into(),
            (false, "+") => UnaoptrType::Pos.into(),
            (false, "-") => UnaoptrType::Neg.into(),
            (_, "+=") => AssignType::Ade.into(),
            (_, "-=") => AssignType::Sue.into(),
            _ => return Err(LexingError::UnknownBinoptr { s: value })
        };
        Ok(Token::new(tt, value))
    }

    pub fn scan_comparative_optr(&mut self) -> Result<Token, LexingError> {
        let mut value = String::new();

        if let Some(ch) = self.chars.next() {
            value.push(ch);
        } else {
            return Err(LexingError::EndOfCharStream);
        }

        if let Some(eq) = self.chars.peek() {
            if *eq == '=' {
                value.push('=');
                self.chars.next();
            }
        }

        let tt = match value.as_str() {
            ">" => BinoptrType::Gt.into(),
            "<" => BinoptrType::Lt.into(),
            "=" => AssignType::Asn.into(),
            "!" => UnaoptrType::Not.into(),
            ">=" => BinoptrType::Ge.into(),
            "<=" => BinoptrType::Le.into(),
            "==" => BinoptrType::Eq.into(),
            "!=" => BinoptrType::Ne.into(),
            _ => return Err(LexingError::UnknownBinoptr { s: value })
        };
        Ok(Token::new(tt, value))
    }
}