use AssignType::*;
use BinoptrType::*;
use SymbolType::*;

use crate::ast::{ASTFactory, Eat};
use crate::error::SyntaxError;
use crate::expr::Node;
use crate::flow::Statement;
use crate::lexer::*;

impl ASTFactory {
    pub(super) fn parse_expression(&mut self) -> Result<Node, SyntaxError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Node, SyntaxError> {
        let mut left = self.parse_logical()?;
        while let Some(token) = self.tokens.pop() {
            if let TokenType::Asn(
                t @ (Asn | Ade | Sue | Mue | Die | Moe)
            ) = token.kind {
                let right = self.parse_logical()?;
                left = Node::Assign {
                    optr: t,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                self.tokens.push(token);
                break;
            }
        }
        Ok(left)
    }

    fn parse_logical(&mut self) -> Result<Node, SyntaxError> {
        let mut left = self.parse_compare()?;
        while let Some(token) = self.tokens.pop() {
            if let TokenType::Bin(
                t @ (And | Xor | Or)
            ) = token.kind {
                let right = self.parse_compare()?;
                left = Node::Binary {
                    optr: t,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                self.tokens.push(token);
                break;
            }
        }
        Ok(left)
    }

    fn parse_compare(&mut self) -> Result<Node, SyntaxError> {
        let mut left = self.parse_additive()?;
        while let Some(token) = self.tokens.pop() {
            if let TokenType::Bin(
                t @ (Gt | Ge | Lt | Le | Eq | Ne)
            ) = token.kind {
                let right = self.parse_additive()?;
                left = Node::Binary {
                    optr: t,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                self.tokens.push(token);
                break;
            }
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Node, SyntaxError> {
        let mut left = self.parse_multiply()?;
        while let Some(token) = self.tokens.pop() {
            if let TokenType::Bin(
                t @ (Add | Sub)
            ) = token.kind {
                let right = self.parse_multiply()?;
                left = Node::Binary {
                    optr: t,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                self.tokens.push(token);
                break;
            }
        }
        Ok(left)
    }

    fn parse_multiply(&mut self) -> Result<Node, SyntaxError> {
        let mut left = self.parse_unary()?;
        while let Some(token) = self.tokens.pop() {
            if let TokenType::Bin(
                t @ (Mod | Mul | Div)
            ) = token.kind {
                let right = self.parse_unary()?;
                left = Node::Binary {
                    optr: t,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                self.tokens.push(token);
                break;
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Node, SyntaxError> {
        if let Some(token) = self.tokens.pop() {
            if let TokenType::Una(t) = token.kind {
                let inner = self.parse_unary()?;
                let temp = Node::Unary {
                    optr: t,
                    inner: Box::new(inner),
                };
                return Ok(temp);
            } else {
                self.tokens.push(token);
            }
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Node, SyntaxError> {
        if let Some(token) = self.tokens.last() {
            match token.kind {
                TokenType::Val(_) => self.parse_literal(),
                TokenType::Identifier => self.parse_identifier(),
                TokenType::Sym(LParen) => self.parse_parentheses(),
                TokenType::Sym(Pipe) => self.parse_function(),
                _ => Err(SyntaxError::TokenNotPrimary(token.value.clone()))
            }
        } else {
            Err(SyntaxError::EndOfTokenSteam)
        }
    }

    fn parse_literal(&mut self) -> Result<Node, SyntaxError> {
        if let Some(token) = self.tokens.pop() {
            if let TokenType::Val(t) = token.kind {
                Ok(Node::Literal { kind: t, value: token.value })
            } else {
                Err(SyntaxError::TokenNotALiteral(token.value))
            }
        } else {
            Err(SyntaxError::EndOfTokenSteam)
        }
    }

    fn parse_identifier(&mut self) -> Result<Node, SyntaxError> {
        let ident = match self.tokens.pop() {
            Some(token) => token.value,
            None => return Err(SyntaxError::EndOfTokenSteam)
        };

        let mut callable = false;
        let args = if self.eat(LParen).is_ok() {
            callable = true;
            self.parse_identifier_arguments()?
        } else {
            Vec::new()
        };
        Ok(Node::Identifier { ident, args, callable })
    }

    fn parse_identifier_arguments(&mut self) -> Result<Vec<Node>, SyntaxError> {
        let mut args = Vec::new();
        if self.eat(RParen).is_ok() {
            return Ok(args);
        }
        loop {
            let expr = self.parse_expression()?;
            args.push(expr);

            if self.eat(RParen).is_ok() {
                break;
            } else if self.eat(Comma).is_err() {
                return Err(SyntaxError::IncompleteCall);
            }
        }
        Ok(args)
    }

    fn parse_function(&mut self) -> Result<Node, SyntaxError> {
        self.eat(Pipe)?;
        let args = self.parse_function_arguments()?;
        let block = if let Some(token) = self.tokens.last() {
            if token.kind == TokenType::Sym(LBrace) {
                self.parse_block()?
            } else {
                let expr = self.parse_expression()?;
                Statement::Return { expr }.into()
            }
        } else {
            return Err(SyntaxError::EndOfTokenSteam);
        };

        Ok(Node::Function { args, body: block })
    }

    fn parse_function_arguments(&mut self) -> Result<Vec<String>, SyntaxError> {
        let mut args = Vec::new();
        if self.eat(Pipe).is_ok() {
            return Ok(args);
        }
        loop {
            if let Some(token) = self.tokens.pop() {
                match token.kind {
                    TokenType::Identifier => args.push(token.value),
                    _ => return Err(SyntaxError::InvalidArgs(token.value))
                }
            }
            if self.eat(Pipe).is_ok() {
                break;
            } else if self.eat(Comma).is_err() {
                return Err(SyntaxError::IncompleteFunc);
            }
        }
        Ok(args)
    }

    fn parse_parentheses(&mut self) -> Result<Node, SyntaxError> {
        self.eat(LParen)?;
        let inner = self.parse_expression()?;
        self.eat(RParen)?;
        Ok(inner)
    }
}
