use crate::ast::{Block, BufVec, Expr, Path, Root};
use crate::philia093::Intern;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Fault {
    MainNotFound(Root),
    BlockEarlyReturn(Block, usize),
    OutsideLoop(Expr),
    PathNotExist(Path),
    DuplicatePath(BufVec<usize, 1>),
    InconsistentIfElse(Block, Option<Rc<Expr>>),
    UndeterminedValue,
    Internal,
    UnacceptableVoid,
    ValueNotDefined,
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
            Fault::DuplicatePath(buf) => {
                msg.push_str("this path is already defined\n");
                msg.push_str(ERROR);
                Path(buf).recover(&mut msg, intern).unwrap();
            }
            Fault::InconsistentIfElse(block, alter) => {
                if let Some(alter) = alter {
                    msg.push_str(
                        "one of `if` and `else` has return value while the other one doesn't\n",
                    );
                    msg.push_str(ERROR);
                    block.recover(&mut msg, ERROR, 0, None, intern).unwrap();
                    msg.push('\n');
                    msg.push_str(OK);
                    msg.push('\n');
                    msg.push_str(ERROR);
                    alter.recover(&mut msg, ERROR, 0, intern).unwrap();
                } else {
                    msg.push_str("`if` has return value but `else` is missing\n");
                    msg.push_str(ERROR);
                    block.recover(&mut msg, ERROR, 0, None, intern).unwrap();
                }
            }
            Fault::UndeterminedValue => msg.push_str("undetermined value"),
            Fault::Internal => msg.push_str("internal error"),
            Fault::UnacceptableVoid => msg.push_str("unacceptable void"),
            Fault::ValueNotDefined => msg.push_str("value not defined"),
        }
        msg.push_str("\nNote: ast recovery only reflect the structure but raw code");
        msg
    }
}
