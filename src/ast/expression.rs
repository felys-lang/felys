use crate::ast::{ASTFactory, Eat};
use crate::error::SyntaxError;
use crate::expr::Node;
use crate::lexer::*;
use BinoptrType::*;
use SymbolType::*;
use AssignType::*;
use crate::flow::Statement;


impl ASTFactory {
    pub(super) fn parse_expression(&mut self) -> Result<Node, SyntaxError> {
        self.parse_assignement()
    }

    fn parse_assignement(&mut self) -> Result<Node, SyntaxError> {
        let mut left = self.parse_logical()?;
        while let Some(token) = self.tokens.pop() {
            if let TokenType::Asn(
                t @ (Asn | Ade | Sue | Mue | Die | Moe)
            ) = token.kind {
                let right = self.parse_logical()?;
                left = Node::Assign {
                    optr: t,
                    left: Box::new(left),
                    right: Box::new(right)
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
                    right: Box::new(right)
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
                    right: Box::new(right)
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
                    right: Box::new(right)
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
                    right: Box::new(right)
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
            if let TokenType::Una(t ) = token.kind {
                let inner = self.parse_unary()?;
                let temp = Node::Unary {
                    optr: t,
                    inner: Box::new(inner),
                };
                return Ok(temp)
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
                _ => Err(SyntaxError::TokenNotPrimary { s: token.value.clone() })
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
                Err(SyntaxError::TokenNotALiteral { s: token.value })
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
        let args = if let Some(token) = self.tokens.last() {
            if token.kind == TokenType::Sym(LParen) {
                callable = true;
                self.parse_arguments()?
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        Ok(Node::Identifier { ident, args, callable })
    }

    fn parse_arguments(&mut self) -> Result<Vec<Node>, SyntaxError> {
        self.eat(LParen)?;

        let mut args = Vec::new();
        while let Some(token) = self.tokens.last() {
            if token.kind != TokenType::Sym(RParen) {
                let node = self.parse_expression()?;
                args.push(node);
            } else {
                self.eat(RParen)?;
                break;
            }

            if let Some(sym) = self.tokens.pop() {
                match sym.kind {
                    TokenType::Sym(Comma) => (),
                    TokenType::Sym(RParen) => break,
                    _ => return Err(SyntaxError::IncompleteCall)
                }
            } else {
                return Err(SyntaxError::EndOfTokenSteam)
            }
        }
        Ok(args)
    }

    fn parse_function(&mut self) -> Result<Node, SyntaxError> {
        self.eat(Pipe)?;

        let mut args = Vec::new();
        while let Some(token) = self.tokens.pop() {
            match token.kind {
                TokenType::Identifier => args.push(token.value),
                TokenType::Sym(Pipe) => break,
                _ => return Err(SyntaxError::InvalidArgs { s: token.value })
            }

            if let Some(sym) = self.tokens.pop() {
                match sym.kind {
                    TokenType::Sym(Comma) => (),
                    TokenType::Sym(Pipe) => break,
                    _ => return Err(SyntaxError::IncompleteFunc)
                }
            } else {
                return Err(SyntaxError::EndOfTokenSteam)
            }
        }

        let block = if let Some(token) = self.tokens.last() {
            if token.kind == TokenType::Sym(LBrace) {
                self.parse_block()?
            } else {
                let expr = self.parse_expression()?;
                Statement::Return { expr }.into()
            }
        } else {
            return Err(SyntaxError::EndOfTokenSteam)
        };

        Ok(Node::Function { args, body: block })
    }

    fn parse_parentheses(&mut self) -> Result<Node, SyntaxError> {
        self.eat(LParen)?;
        let inner = self.parse_expression()?;
        self.eat(RParen)?;
        Ok(inner)
    }
}
