use crate::error::LexingError;
use crate::lexer::*;
use crate::lexer::cursor::Cursor;

impl Cursor<'_> {
    pub fn en(&mut self) -> Option<Result<Token, LexingError>> {
        if let Some(first) = self.chars.next() {
            let token = match first {
                '0'..='9' => self.number(first),
                '+' | '-' | '*' | '/' | '%' => self.arithmetic(first),
                '\'' | '"' => self.en_string(first),
                'a'..='z' | 'A'..='Z' | '_' => self.en_ident(first),
                '>' | '<' | '=' | '!' => self.en_comparative(first),
                '(' | ')' | '{' | '}' | '|' | ';' | ',' => self.en_symbol(first),
                other => Err(LexingError::InvalidChar(other))
            };
            Some(token)
        } else {
            None
        }
    }

    pub fn en_symbol(&mut self, first: char) -> Result<Token, LexingError> {
        let tt = match first {
            '(' => SymbolType::LParen.into(),
            ')' => SymbolType::RParen.into(),
            '{' => SymbolType::LBrace.into(),
            '}' => SymbolType::RBrace.into(),
            ';' => SymbolType::Semicol.into(),
            '|' => SymbolType::Pipe.into(),
            ',' => SymbolType::Comma.into(),
            other => return Err(LexingError::InvalidChar(other))
        };
        Ok(Token::new(tt, first.to_string()))
    }

    pub fn en_ident(&mut self, first: char) -> Result<Token, LexingError> {
        let mut value = String::from(first);
        while let Some(ch) = self.chars.peek() {
            if matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
                value.push(*ch);
                self.chars.next();
            } else {
                break;
            }
        }
        let tt = match value.as_str() {
            "and" => BinoptrType::And.into(),
            "xor" => BinoptrType::Xor.into(),
            "or" => BinoptrType::Or.into(),
            "if" => KeywordType::If.into(),
            "elif" => KeywordType::Elif.into(),
            "else" => KeywordType::Else.into(),
            "while" => KeywordType::While.into(),
            "return" => KeywordType::Return.into(),
            "true" => ValueType::Boolean.into(),
            "false" => ValueType::Boolean.into(),
            "none" => ValueType::None.into(),
            _ => TokenType::Identifier
        };
        Ok(Token::new(tt, value))
    }

    pub fn en_string(&mut self, first: char) -> Result<Token, LexingError> {
        let mut value = String::new();
        for ch in self.chars.by_ref() {
            if ch == first {
                return Ok(Token::new(ValueType::String.into(), value));
            } else {
                value.push(ch)
            }
        }
        Err(LexingError::StringNotClosed(value))
    }

    pub fn en_comparative(&mut self, first: char) -> Result<Token, LexingError> {
        let mut value = String::from(first);
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
            _ => return Err(LexingError::UnknownBinoptr(value))
        };
        Ok(Token::new(tt, value))
    }
}
