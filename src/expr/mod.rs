pub use node::*;

use crate::env::{Environ, Object};
use crate::error::RuntimeError;
use crate::expr::eval::*;

mod node;
mod eval;

impl Node {
    pub fn eval(
        &self, env: &mut Environ, out: &mut Vec<String>,
    ) -> Result<Object, RuntimeError> {
        match self {
            Node::Binary {
                optr,
                left,
                right
            } => eval_binary_optr(env, out, optr, left, right),
            Node::Unary {
                optr,
                inner
            } => eval_unary_optr(env, out, optr, inner),
            Node::Identifier {
                ident,
                args,
                callable
            } => eval_identifier(env, out, ident, args, callable),
            Node::Literal {
                kind,
                value
            } => eval_literal(kind, value),
            Node::Assign {
                optr,
                left,
                right
            } => eval_assignment(env, out, optr, left, right),
            Node::Function {
                args,
                body
            } => Ok(Object::Function {
                args: args.clone(),
                body: body.clone(),
            })
        }
    }
}