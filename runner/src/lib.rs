mod execute;
mod environ;
mod eval;

use crate::environ::{Environ, Warehouse, Writer};
use crate::execute::Evaluation;
use ast::Program;
use packrat::Pool;

pub fn exec(program: Program, pool: Pool) {
    let mut env = Environ {
        writer: Writer { buffer: String::new() },
        warehouse: Warehouse { floors: Vec::new() },
        pool,
    };
    let _ = program.eval(&mut env);
}