use crate::error::LexingError;
use crate::lexer::*;
use crate::lexer::cursor::Cursor;

impl Cursor<'_> {
    pub fn zh(&mut self) -> Option<Result<Token, LexingError>> {
        if let Some(first) = self.chars.next() {
            let token = match first {
                '0'..='9' => self.number(first),
                '+' | '-' | '*' | '/' | '%' => self.arithmetic(first),
                '‘' | '“' => self.zh_string(first),
                '\u{4e00}'..='\u{9fa5}' | '—' => self.zh_ident(first),
                '（' | '）' |
                '「' | '」' |
                '｜' | '；' | '，' | '=' | '！' => self.zh_symbol(first),
                other => Err(LexingError::InvalidChar { c: other })
            };
            Some(token)
        } else {
            None
        }
    }

    pub fn zh_symbol(&mut self, first: char) -> Result<Token, LexingError> {
        let tt = match first {
            '（' => SymbolType::LParen.into(),
            '）' => SymbolType::RParen.into(),
            '「' => SymbolType::LBrace.into(),
            '」' => SymbolType::RBrace.into(),
            '；' => SymbolType::Semicol.into(),
            '｜' => SymbolType::Pipe.into(),
            '，' => SymbolType::Comma.into(),
            '=' => AssignType::Asn.into(),
            '！' => UnaoptrType::Not.into(),
            other => return Err(LexingError::InvalidChar { c: other })
        };
        Ok(Token::new(tt, first.to_string()))
    }

    pub fn zh_ident(&mut self, first: char) -> Result<Token, LexingError> {
        let mut value = String::from(first);
        while let Some(ch) = self.chars.peek() {
            if ('\u{4e00}'..='\u{9fa5}').contains(ch) || *ch == '—' {
                value.push(*ch);
                self.chars.next();
            } else {
                break;
            }
        }
        let tt = match value.as_str() {
            "和" => BinoptrType::And.into(),
            "异或" => BinoptrType::Xor.into(),
            "或" => BinoptrType::Or.into(),
            "如果" => KeywordType::If.into(),
            "否如" => KeywordType::Elif.into(),
            "否则" => KeywordType::Else.into(),
            "直到" => KeywordType::While.into(),
            "返回" => KeywordType::Return.into(),
            "大于" => BinoptrType::Gt.into(),
            "小于" => BinoptrType::Lt.into(),
            "大于等于" => BinoptrType::Ge.into(),
            "小于等于" => BinoptrType::Le.into(),
            "等于" => BinoptrType::Eq.into(),
            "不等于" => BinoptrType::Ne.into(),
            "无" => ValueType::None.into(),
            "真" => {
                value = true.to_string();
                ValueType::Boolean.into()
            }
            "假" => {
                value = false.to_string();
                ValueType::Boolean.into()
            }
            _ => TokenType::Identifier
        };
        Ok(Token::new(tt, value))
    }

    pub fn zh_string(&mut self, first: char) -> Result<Token, LexingError> {
        let mut value = String::new();
        for ch in self.chars.by_ref() {
            match (first, ch) {
                ('‘', '’') |
                ('“', '”') => return Ok(Token::new(ValueType::String.into(), value)),
                _ => value.push(ch)
            }
        }
        Err(LexingError::StringNotClosed { s: value })
    }
}
