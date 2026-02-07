use crate::philia093::Intern;
use crate::utils::ast::{Block, Chunk, Expr, Impl, Item, Lit};

pub enum Error {
    MainNotFound,
    BlockEarlyReturn(Block, usize),
    OutsideLoop(Expr),
    FunctionNoReturn(Block),
    InvalidInt(Lit),
    InvalidFloat(Lit),
    InvalidStrChunk(Chunk),
    NoReturnValue(Expr),
    RedeclaredItem(Item),
    RedeclaredImpl(Impl),
    VariableNotDefined(usize),
    InvalidPath(Expr),
}

impl Error {
    pub fn recover(self, intern: &Intern) -> String {
        const OK: &str = "--- | ";
        const ERROR: &str = ">>> | ";
        let mut tailing = true;

        let mut msg = "Cyrene: ".to_string();
        match self {
            Error::MainNotFound => {
                msg.push_str("program entry not found\n");
                tailing = false;
            }
            Error::BlockEarlyReturn(block, i) => {
                msg.push_str("the node below appears after the block is returned\n");
                msg.push_str(OK);
                block
                    .recover(&mut msg, OK, 0, Some((i, ERROR)), intern)
                    .unwrap();
            }
            Error::OutsideLoop(expr) => {
                msg.push_str("`break` or `continue` below must stay inside a loop\n");
                msg.push_str(ERROR);
                expr.recover(&mut msg, ERROR, 0, intern).unwrap();
            }
            Error::FunctionNoReturn(block) => {
                msg.push_str("function body does not have return value\n");
                msg.push_str(ERROR);
                block.recover(&mut msg, ERROR, 0, None, intern).unwrap();
            }
            Error::InvalidInt(lit) => {
                msg.push_str("this integer cannot be stored as `isize`\n");
                msg.push_str(ERROR);
                lit.recover(&mut msg, intern).unwrap();
            }
            Error::InvalidFloat(lit) => {
                msg.push_str("this decimal cannot be stored as `f64`\n");
                msg.push_str(ERROR);
                lit.recover(&mut msg, intern).unwrap();
            }
            Error::InvalidStrChunk(chunk) => {
                msg.push_str("this escaped character is invalid\n");
                msg.push_str(ERROR);
                chunk.recover(&mut msg, intern).unwrap();
            }
            Error::NoReturnValue(expr) => {
                msg.push_str("this expression does not have a return value\n");
                msg.push_str(ERROR);
                expr.recover(&mut msg, ERROR, 0, intern).unwrap();
            }
            Error::RedeclaredItem(item) => {
                msg.push_str("this item is redeclared\n");
                msg.push_str(ERROR);
                item.recover(&mut msg, ERROR, 0, intern).unwrap();
            }
            Error::RedeclaredImpl(implementation) => {
                msg.push_str("this function is redeclared\n");
                msg.push_str(ERROR);
                implementation.recover(&mut msg, ERROR, 0, intern).unwrap();
            }
            Error::VariableNotDefined(id) => {
                msg.push_str("this variable is not defined\n");
                msg.push_str(ERROR);
                msg.push_str(intern.get(&id).unwrap())
            }
            Error::InvalidPath(path) => {
                msg.push_str("this path does not lead to anywhere\n");
                msg.push_str(ERROR);
                path.recover(&mut msg, ERROR, 0, intern).unwrap();
            }
        }
        if tailing {
            msg.push_str("\nNote: ast recovery does not reflect the raw code\n");
        }
        msg
    }
}
