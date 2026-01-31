use crate::philia093::Intern;
use crate::utils::ast::{Block, BufVec, Chunk, Expr, Lit, Path, Root};
use std::rc::Rc;

pub enum Fault {
    MainNotFound(Root),
    BlockEarlyReturn(Block, usize),
    OutsideLoop(Expr),
    PathNotExist(Path),
    DuplicatePath(usize, Vec<usize>),
    FunctionNoReturn(Block),
    InvalidInt(Lit),
    InvalidFloat(Lit),
    InvalidStrChunk(Chunk),
    NoReturnValue(Rc<Expr>),
    ValueNotDefined(usize),
}

impl Fault {
    pub fn recover(self, intern: &Intern) -> String {
        const OK: &str = "--- | ";
        const ERROR: &str = ">>> | ";

        let mut msg = "Cyrene: ".to_string();
        match self {
            Fault::MainNotFound(root) => {
                msg.push_str("program entry not found\n");
                msg.push_str(ERROR);
                root.recover(&mut msg, ERROR, 0, intern).unwrap();
            }
            Fault::BlockEarlyReturn(block, i) => {
                msg.push_str("the node below appears after the block is returned\n");
                msg.push_str(OK);
                block
                    .recover(&mut msg, OK, 0, Some((i, ERROR)), intern)
                    .unwrap();
            }
            Fault::OutsideLoop(expr) => {
                msg.push_str("`break` or `continue` below must stay inside a loop\n");
                msg.push_str(ERROR);
                expr.recover(&mut msg, ERROR, 0, intern).unwrap();
            }
            Fault::PathNotExist(path) => {
                msg.push_str("this path does not lead to anywhere\n");
                msg.push_str(ERROR);
                path.recover(&mut msg, intern).unwrap();
            }
            Fault::DuplicatePath(buf, vec) => {
                msg.push_str("this path is already defined\n");
                msg.push_str(ERROR);
                Path(BufVec::new([buf], vec))
                    .recover(&mut msg, intern)
                    .unwrap();
            }
            Fault::FunctionNoReturn(block) => {
                msg.push_str("function body does not have return value\n");
                msg.push_str(ERROR);
                block.recover(&mut msg, ERROR, 0, None, intern).unwrap();
            }
            Fault::InvalidInt(lit) => {
                msg.push_str("this integer cannot be stored as `isize`\n");
                msg.push_str(ERROR);
                lit.recover(&mut msg, intern).unwrap();
            }
            Fault::InvalidFloat(lit) => {
                msg.push_str("this decimal cannot be stored as `f64`\n");
                msg.push_str(ERROR);
                lit.recover(&mut msg, intern).unwrap();
            }
            Fault::InvalidStrChunk(chunk) => {
                msg.push_str("this escaped character is invalid\n");
                msg.push_str(ERROR);
                chunk.recover(&mut msg, intern).unwrap();
            }
            Fault::NoReturnValue(expr) => {
                msg.push_str("this expression does not have a return value\n");
                msg.push_str(ERROR);
                expr.recover(&mut msg, ERROR, 0, intern).unwrap();
            }
            Fault::ValueNotDefined(id) => {
                msg.push_str("this value is not defined\n");
                msg.push_str(ERROR);
                let value = intern.get(&id).unwrap();
                msg.push_str(value);
            }
        }
        msg.push_str("\nNote: ast recovery does not reflect the raw code\n");
        msg
    }
}
