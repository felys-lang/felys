use KeywordType::*;

use crate::ast::{ASTFactory, Eat};
use crate::error::SyntaxError;
use crate::flow::{Block, Statement};
use crate::lexer::*;

impl ASTFactory {
    pub(super) fn parse_statement(&mut self) -> Option<Result<Statement, SyntaxError>> {
        if let Some(keyword) = self.tokens.last() {
            let stmt = match keyword.kind {
                TokenType::Key(If) => self.parse_cond(false),
                TokenType::Key(While) => self.parse_while(),
                TokenType::Key(Return) => self.parse_return(),
                _ => self.parse_simple()
            };
            Some(stmt)
        } else { None }
    }

    pub(super) fn parse_block(&mut self) -> Result<Block, SyntaxError> {
        self.eat(SymbolType::LBrace)?;
        let mut body = Vec::new();
        while let Some(stmt) = self.parse_statement() {
            body.push(stmt?);
            if let Some(token) = self.tokens.last() {
                if token.kind == TokenType::Sym(SymbolType::RBrace) {
                    break;
                }
            }
        }
        self.eat(SymbolType::RBrace)?;
        Ok(Block::new(body))
    }

    fn parse_cond(&mut self, elif: bool) -> Result<Statement, SyntaxError> {
        if elif {
            self.eat(Elif)?;
        } else {
            self.eat(If)?;
        }

        let expr = self.parse_expression()?;
        let body = self.parse_block()?;
        let alter = if let Some(token) = self.tokens.last() {
            match token.kind {
                TokenType::Key(Elif) => Some(Box::new(self.parse_cond(true)?)),
                TokenType::Key(Else) => Some(Box::new(self.parse_else()?)),
                _ => None
            }
        } else { None };
        Ok(Statement::Cond { expr, body, alter })
    }

    fn parse_else(&mut self) -> Result<Statement, SyntaxError> {
        self.eat(Else)?;
        let body = self.parse_block()?;
        Ok(Statement::Else { body })
    }

    fn parse_while(&mut self) -> Result<Statement, SyntaxError> {
        self.eat(While)?;
        let expr = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(Statement::While { expr, body })
    }

    fn parse_return(&mut self) -> Result<Statement, SyntaxError> {
        self.eat(Return)?;
        let expr = self.parse_expression()?;
        self.eat(SymbolType::Semicol)?;
        Ok(Statement::Return { expr })
    }

    fn parse_simple(&mut self) -> Result<Statement, SyntaxError> {
        let expr = self.parse_expression()?;
        self.eat(SymbolType::Semicol)?;
        Ok(Statement::Simple { expr })
    }
}
