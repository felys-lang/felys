use AssignType::*;
use BinoptrType::*;
use UnaoptrType::*;

use crate::env::{Environ, Object};
use crate::error::RuntimeError;
use crate::expr::Node;
use crate::lexer::*;

pub(super) fn eval_identifier(
    env: &mut Environ, out: &mut Vec<String>,
    ident: &String, args: &[Node], callable: &bool,
) -> Result<Object, RuntimeError> {
    let evaled = args.iter()
        .map(|arg| arg.eval(env, out))
        .collect::<Result<Vec<Object>, RuntimeError>>()?;
    env.eval(out, ident, evaled, callable)
}


pub(super) fn eval_literal(optr: &ValueType, value: &String) -> Result<Object, RuntimeError> {
    let result = match optr {
        ValueType::Boolean => Object::Boolean(value == "true" || value == "真"),
        ValueType::String => Object::String(value.clone()),
        ValueType::Number => Object::Number(
            value.parse()
                .map_err(|_| RuntimeError::NoF64Convertion { s: value.clone() })?
        ),
        ValueType::None => Object::None
    };
    Ok(result)
}


pub(super) fn eval_assignment(
    env: &mut Environ, out: &mut Vec<String>,
    optr: &AssignType, left: &Node, right: &Node,
) -> Result<Object, RuntimeError> {
    if let Node::Identifier { ident, args: _, callable } = left {
        if *callable {
            return Err(RuntimeError::CannotAssign);
        }

        let result = match optr {
            Asn => {
                let new = right.eval(env, out)?;
                env.store(ident.clone(), new.clone());
                new
            }
            Ade => {
                let new = eval_binary_optr(env, out, &Add, left, right)?;
                env.store(ident.clone(), new);
                Object::None
            }
            Sue => {
                let new = eval_binary_optr(env, out, &Sub, left, right)?;
                env.store(ident.clone(), new);
                Object::None
            }
            Mue => {
                let new = eval_binary_optr(env, out, &Mul, left, right)?;
                env.store(ident.clone(), new);
                Object::None
            }
            Die => {
                let new = eval_binary_optr(env, out, &Div, left, right)?;
                env.store(ident.clone(), new);
                Object::None
            }
            Moe => {
                let new = eval_binary_optr(env, out, &Mod, left, right)?;
                env.store(ident.clone(), new);
                Object::None
            }
        };
        Ok(result)
    } else {
        Err(RuntimeError::CannotAssign)
    }
}


pub(super) fn eval_binary_optr(
    env: &mut Environ, out: &mut Vec<String>,
    optr: &BinoptrType, left: &Node, right: &Node,
) -> Result<Object, RuntimeError> {
    let lval = left.eval(env, out)?;
    let rval = right.eval(env, out)?;
    let result = match optr {
        Add => {
            let value = lval.f64()? + rval.f64()?;
            Object::Number(value)
        }
        Sub => {
            let value = lval.f64()? - rval.f64()?;
            Object::Number(value)
        }
        Mul => {
            let value = lval.f64()? * rval.f64()?;
            Object::Number(value)
        }
        Div => {
            let value = lval.f64()? / rval.f64()?;
            Object::Number(value)
        }
        Mod => {
            let value = lval.f64()? % rval.f64()?;
            Object::Number(value)
        }
        Eq => {
            let value = lval.f64()? == rval.f64()?;
            Object::Boolean(value)
        }
        Ne => {
            let value = lval.f64()? != rval.f64()?;
            Object::Boolean(value)
        }
        Gt => {
            let value = lval.f64()? > rval.f64()?;
            Object::Boolean(value)
        }
        Lt => {
            let value = lval.f64()? < rval.f64()?;
            Object::Boolean(value)
        }
        Ge => {
            let value = lval.f64()? >= rval.f64()?;
            Object::Boolean(value)
        }
        Le => {
            let value = lval.f64()? <= rval.f64()?;
            Object::Boolean(value)
        }
        And => {
            if lval.bool() && rval.bool() {
                rval
            } else {
                Object::Boolean(false)
            }
        }
        Xor => {
            if lval.bool() && !rval.bool() {
                lval
            } else if !lval.bool() && rval.bool() {
                rval
            } else {
                Object::Boolean(false)
            }
        }
        Or => {
            if lval.bool() {
                lval
            } else if rval.bool() {
                rval
            } else {
                Object::Boolean(false)
            }
        }
    };
    Ok(result)
}


pub(super) fn eval_unary_optr(
    env: &mut Environ, out: &mut Vec<String>,
    optr: &UnaoptrType, inner: &Node,
) -> Result<Object, RuntimeError> {
    let ival = inner.eval(env, out)?;
    let result = match optr {
        Not => {
            let value = !ival.bool();
            Object::Boolean(value)
        }
        Pos => {
            let value = ival.f64()?;
            Object::Number(value)
        }
        Neg => {
            let value = -ival.f64()?;
            Object::Number(value)
        }
    };
    Ok(result)
}
