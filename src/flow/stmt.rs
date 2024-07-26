use std::fmt::{Debug, Formatter};
use crate::expr::Node;


#[derive(PartialEq, Clone)]
pub enum Statement {
    Cond {
        expr: Node,
        body: Block,
        alter: Option<Box<Statement>>
    },

    Else {
        body: Block,
    },

    While {
        expr: Node,
        body: Block,
    },

    Return {
        expr: Node,
    },

    Simple {
        expr: Node,
    }
}


impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Cond { 
                expr,
                body,
                alter
            } => if let Some(stmt) = alter {
                write!(f, "if {:?} {:?} else {:?}", expr, body, stmt)
            } else {
                write!(f, "if {:?} {:?}", expr, body)
            },
            Statement::Else { 
                body
            } => write!(f, "{:?}", body),
            Statement::While { 
                expr,
                body
            } => write!(f, "while {:?} {:?}", expr, body),
            Statement::Return { 
                expr
            } => write!(f, "return {:?}", expr),
            Statement::Simple { 
                expr
            } => write!(f, "{:?}", expr)
        }
    }
}


#[derive(PartialEq, Clone)]
pub struct Block {
    pub body: Vec<Statement>
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.body)
    }
}


impl Block {
    pub fn new(body: Vec<Statement>) -> Self {
        Self { body }
    }
}


impl From<Statement> for Block {
    fn from(value: Statement) -> Self {
        Block { body: vec![value] }
    }
}