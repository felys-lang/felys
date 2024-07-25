use crate::error::LexingError;
use crate::lexer::cursor::Cursor;
use crate::lexer::{BinoptrType, KeywordType, SymbolType, Token, TokenType, ValueType};

impl Cursor<'_> {
    pub fn scan_eng_string(&mut self) -> Result<Token, LexingError> {
        let sos = self.chars.next().ok_or(LexingError::EndOfCharStream)?;
        if sos != '\'' && sos != '"' {
            return Err(LexingError::InvalidString { c: sos })
        }
        
        let mut value = String::new();
        for ch in self.chars.by_ref() {
            if ch != sos {
                value.push(ch);
            } else {
                return Ok(Token::new(ValueType::String.into(), value));
            }
        }
        Err(LexingError::StringNotClosed { s: value })
    }
    
    pub fn scan_eng_ident(&mut self) -> Result<Token, LexingError> {
        let mut value = String::new();
        while let Some(ch) = self.chars.peek() {
            if ch.is_ascii_alphanumeric() || *ch == '_' {
                value.push(*ch);
                self.chars.next();
            } else {
                break;
            }
        }

        let ttype = match value.as_str() {
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
            "" => return Err(LexingError::EndOfCharStream),
            _ => TokenType::Identifier
        };
        Ok(Token::new(ttype, value))
    }

    pub fn scan_eng_symbol(&mut self) -> Result<Token, LexingError> {
        let ch = self.chars.next().ok_or(LexingError::EndOfCharStream)?;
        let ttype = match ch {
            '(' => SymbolType::LParen.into(),
            ')' => SymbolType::RParen.into(),
            '{' => SymbolType::LBrace.into(),
            '}' => SymbolType::RBrace.into(),
            ';' => SymbolType::Semicol.into(),
            '|' => SymbolType::Pipe.into(),
            ',' => SymbolType::Comma.into(),
            ch => return Err(LexingError::UnknownSymbol { c: ch })
        };
        Ok(Token::new(ttype, ch.to_string()))
    }
}
