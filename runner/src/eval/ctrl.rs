use crate::environ::{Environ, Value};
use crate::execute::{Evaluation, Signal};
use ast::ctrl::Ctrl;

impl Evaluation for Ctrl {
    fn eval(&self, env: &mut Environ) -> Result<Value, Signal> {
        match self {
            Ctrl::Assign(_, _, _) => todo!(),
            Ctrl::Block(block) => block.eval(env),
            Ctrl::Break(_) => todo!(),
            Ctrl::Continue => todo!(),
            Ctrl::For(_, _, _) => todo!(),
            Ctrl::Match(_, _) => todo!(),
            Ctrl::If(_, _, _) => todo!(),
            Ctrl::Loop(_) => todo!(),
            Ctrl::Return(_) => todo!(),
            Ctrl::While(_, _) => todo!(),
        }
    }
}