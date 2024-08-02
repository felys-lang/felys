use crate::error::Error;
use crate::Language;
use crate::lexer::cursor::Cursor;
pub use crate::lexer::token::*;

mod token;
mod cursor;

pub fn tokenize(c: String, lang: &Language) -> Result<Vec<Token>, Error> {
    let mut cursor = Cursor {
        chars: c.chars().peekable(),
        buffer: Vec::new(),
    };

    while let Some(tk) = cursor.scan_next(lang) {
        cursor.buffer.push(tk?)
    }

    cursor.buffer.reverse();
    Ok(cursor.buffer)
}

#[cfg(test)]
mod lexer_unit_test {
    use crate::Language::*;
    use crate::lexer::{AssignType, BinoptrType, KeywordType, SymbolType, tokenize, UnaoptrType, ValueType};
    
    macro_rules! check {
        ($c: literal, $kind: expr, $lang: ident) => {
            assert_eq!(tokenize($c.to_string(), &$lang).unwrap().pop().unwrap().kind, $kind.into())
        };
    }

    #[test]
    fn en() {
        check!("true", ValueType::Boolean, EN);
        check!("false", ValueType::Boolean, EN);
        check!("'elysia'", ValueType::String, EN);
        check!("\"elysia\"", ValueType::String, EN);
        check!("11.11", ValueType::Number, EN);
        check!("11111", ValueType::Number, EN);
        check!("none", ValueType::None, EN);

        check!("return", KeywordType::Return, EN);
        check!("if", KeywordType::If, EN);
        check!("elif", KeywordType::Elif, EN);
        check!("else", KeywordType::Else, EN);
        check!("while", KeywordType::While, EN);

        check!("(", SymbolType::LParen, EN);
        check!(")", SymbolType::RParen, EN);
        check!("{", SymbolType::LBrace, EN);
        check!("}", SymbolType::RBrace, EN);
        check!("|", SymbolType::Pipe, EN);
        check!(";", SymbolType::Semicol, EN);
        check!(",", SymbolType::Comma, EN);

        check!("+", UnaoptrType::Pos, EN);
        check!("-", UnaoptrType::Neg, EN);
        check!("!", UnaoptrType::Not, EN);

        check!("=", AssignType::Asn, EN);
        check!("+=", AssignType::Ade, EN);
        check!("-=", AssignType::Sue, EN);
        check!("*=", AssignType::Mue, EN);
        check!("/=", AssignType::Die, EN);
        check!("%=", AssignType::Moe, EN);

        check!("*", BinoptrType::Mul, EN);
        check!("/", BinoptrType::Div, EN);
        check!("%", BinoptrType::Mod, EN);
        check!("==", BinoptrType::Eq, EN);
        check!("!=", BinoptrType::Ne, EN);
        check!(">", BinoptrType::Gt, EN);
        check!("<", BinoptrType::Lt, EN);
        check!(">=", BinoptrType::Ge, EN);
        check!("<=", BinoptrType::Le, EN);
        check!("and", BinoptrType::And, EN);
        check!("xor", BinoptrType::Xor, EN);
        check!("or", BinoptrType::Or, EN);
    }

    #[test]
    fn zh() {
        check!("真", ValueType::Boolean, ZH);
        check!("假", ValueType::Boolean, ZH);
        check!("‘elysia’", ValueType::String, ZH);
        check!("“爱莉希雅”", ValueType::String, ZH);
        check!("11.11", ValueType::Number, ZH);
        check!("11111", ValueType::Number, ZH);
        check!("无", ValueType::None, ZH);

        check!("返回", KeywordType::Return, ZH);
        check!("如果", KeywordType::If, ZH);
        check!("否如", KeywordType::Elif, ZH);
        check!("否则", KeywordType::Else, ZH);
        check!("循环", KeywordType::While, ZH);

        check!("（", SymbolType::LParen, ZH);
        check!("）", SymbolType::RParen, ZH);
        check!("「", SymbolType::LBrace, ZH);
        check!("」", SymbolType::RBrace, ZH);
        check!("｜", SymbolType::Pipe, ZH);
        check!("；", SymbolType::Semicol, ZH);
        check!("，", SymbolType::Comma, ZH);

        check!("+", UnaoptrType::Pos, ZH);
        check!("-", UnaoptrType::Neg, ZH);
        check!("！", UnaoptrType::Not, ZH);

        check!("=", AssignType::Asn, ZH);
        check!("+=", AssignType::Ade, ZH);
        check!("-=", AssignType::Sue, ZH);
        check!("*=", AssignType::Mue, ZH);
        check!("/=", AssignType::Die, ZH);
        check!("%=", AssignType::Moe, ZH);

        check!("*", BinoptrType::Mul, ZH);
        check!("/", BinoptrType::Div, ZH);
        check!("%", BinoptrType::Mod, ZH);
        check!("等于", BinoptrType::Eq, ZH);
        check!("不等于", BinoptrType::Ne, ZH);
        check!("大于", BinoptrType::Gt, ZH);
        check!("小于", BinoptrType::Lt, ZH);
        check!("大于等于", BinoptrType::Ge, ZH);
        check!("小于等于", BinoptrType::Le, ZH);
        check!("和", BinoptrType::And, ZH);
        check!("异或", BinoptrType::Xor, ZH);
        check!("或", BinoptrType::Or, ZH);
    }
}
