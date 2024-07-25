use crate::error::LexingError;
use crate::lexer::cursor::Cursor;
use crate::lexer::{BinoptrType, KeywordType, SymbolType, Token, TokenType, ValueType};

impl Cursor<'_> {
    pub fn scan_chn_string(&mut self) -> Result<Token, LexingError> {
        let sos = self.chars.next().ok_or(LexingError::EndOfCharStream)?;
        if sos != '‘' && sos != '“' {
            return Err(LexingError::InvalidString { c: sos })
        }
        
        let mut value = String::new();
        for ch in self.chars.by_ref() {
            match (sos, ch) {
                ('‘', '’') | ('“', '”') => return Ok(Token::new(ValueType::String.into(), value)),
                _ => value.push(ch)
            }
        }
        Err(LexingError::StringNotClosed { s: value })
    }
    
    pub fn scan_chn_ident(&mut self) -> Result<Token, LexingError> {
        let mut value = String::new();
        while let Some(ch) = self.chars.peek() {
            if ('\u{4e00}'..='\u{9fa5}').contains(ch) || *ch == '—' {
                value.push(*ch);
                self.chars.next();
            } else {
                break;
            }
        }

        let ttype = match value.as_str() {
            "和" => BinoptrType::And.into(),
            "异或" => BinoptrType::Xor.into(),
            "或" => BinoptrType::Or.into(),
            "如果" => KeywordType::If.into(),
            "否如" => KeywordType::Elif.into(),
            "否则" => KeywordType::Else.into(),
            "直到" => KeywordType::While.into(),
            "返回" => KeywordType::Return.into(),
            "真" => ValueType::Boolean.into(),
            "加" => ValueType::Boolean.into(),
            "无" => ValueType::None.into(),
            "" => return Err(LexingError::EndOfCharStream),
            _ => TokenType::Identifier
        };
        Ok(Token::new(ttype, value))
    }

    pub fn scan_chn_symbol(&mut self) -> Result<Token, LexingError> {
        let ch = self.chars.next().ok_or(LexingError::EndOfCharStream)?;
        let ttype = match ch {
            '（' => SymbolType::LParen.into(),
            '）' => SymbolType::RParen.into(),
            '「' => SymbolType::LBrace.into(),
            '」' => SymbolType::RBrace.into(),
            '；' => SymbolType::Semicol.into(),
            '｜' => SymbolType::Pipe.into(),
            '，' => SymbolType::Comma.into(),
            ch => return Err(LexingError::UnknownSymbol { c: ch })
        };
        Ok(Token::new(ttype, ch.to_string()))
    }
}
