use crate::env::{Environ, Object};
use crate::error::RuntimeError;
use crate::expr::Node;
use crate::flow::{Block, Statement};


pub(super) fn run_condition(
    env: &mut Environ, out: &mut Vec<String>,
    expr: &Node, block: &Block, alter: &Option<Box<Statement>>
) -> Result<Option<Object>, RuntimeError> {
    if expr.eval(env, out)?.bool() {
        block.run(env, out)
    } else if let Some(stmt) = alter {
        stmt.run(env, out)
    } else {
        Ok(None)
    }
}


pub(super) fn run_while(
    env: &mut Environ, out: &mut Vec<String>,
    expr: &Node, block: &Block
) -> Result<Option<Object>, RuntimeError> {
    while expr.eval(env, out)?.bool() {
        if let Some(value) = block.run(env, out)? {
            return Ok(Some(value))
        }
    }
    Ok(None)
}


pub(super) fn run_return(
    env: &mut Environ, out: &mut Vec<String>,
    expr: &Node
) -> Result<Option<Object>, RuntimeError> {
    Ok(Some(expr.eval(env, out)?))
}


pub(super) fn run_simple(
    env: &mut Environ, out: &mut Vec<String>,
    expr: &Node
) -> Result<Option<Object>, RuntimeError> {
    expr.eval(env, out)?;
    Ok(None)
}
