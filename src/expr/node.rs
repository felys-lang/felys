use std::fmt::{Debug, Formatter};
use crate::lexer::*;
use crate::flow::Block;

#[derive(PartialEq, Clone)]
pub enum Node {
    Binary {
        optr: BinoptrType,
        left: Box<Node>,
        right: Box<Node>
    },

    Unary {
        optr: UnaoptrType,
        inner: Box<Node>,
    },

    Function {
        args: Vec<String>,
        body: Block
    },

    Identifier {
        ident: String,
        args: Vec<Node>,
        callable: bool
    },

    Literal {
        kind: ValueType,
        value: String
    },
    
    Assign {
        optr: AssignType,
        left: Box<Node>,
        right: Box<Node>
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Binary { 
                optr, 
                left, 
                right 
            } => write!(f, "({:?} {:?} {:?})", left, optr, right),
            Node::Unary { 
                optr,
                inner
            } => write!(f, "({:?} {:?})", optr, inner),
            Node::Function { 
                args, 
                body 
            } => write!(f, "|{}| {{ {:?} }}", args.join(", "), body),
            Node::Identifier { 
                ident,
                args,
                callable
            } => match (callable, args.len()) {
                (true, 0) => write!(f, "{}()", ident),
                (true, _) => write!(f, "{}({:?})", ident, args),
                _ => write!(f, "{}", ident)
            },
            Node::Literal { 
                kind: _,
                value
            } => write!(f, "{}", value),
            Node::Assign {
                optr,
                left,
                right
            } => write!(f, "({:?} {:?} {:?})", left, optr, right),
        }
    }
}
