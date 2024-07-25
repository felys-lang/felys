use crate::error::SyntaxError;
use crate::flow::Statement;
use crate::lexer::*;

mod expression;
mod statement;


pub struct ASTFactory {
    pub tokens: Vec<Token>
}


impl ASTFactory {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
    
    pub fn produce(&mut self) -> Option<Result<Statement, SyntaxError>> {
        self.parse_statement()
    }
}


trait Eat<T> {
    fn eat(&mut self, t: T) -> Result<(), SyntaxError>;
}


impl Eat<SymbolType> for ASTFactory {
    fn eat(&mut self, t: SymbolType) -> Result<(), SyntaxError> {
        if let Some(token) = self.tokens.pop() {
            if token.kind != TokenType::Sym(t) {
                Err(SyntaxError::EatWrongToken { s: token.value })
            } else {
                Ok(())
            }
        } else {
            Err(SyntaxError::EndOfTokenSteam)
        }
    }
}


impl Eat<KeywordType> for ASTFactory {
    fn eat(&mut self, t: KeywordType) -> Result<(), SyntaxError> {
        if let Some(token) = self.tokens.pop() {
            if token.kind != TokenType::Key(t) {
                Err(SyntaxError::EatWrongToken { s: token.value })
            } else {
                Ok(())
            }
        } else {
            Err(SyntaxError::EndOfTokenSteam)
        }
    }
}
