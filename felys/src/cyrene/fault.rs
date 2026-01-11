use crate::ast::{Expr, Stmt};
use crate::philia093::Intern;

#[derive(Debug, Clone)]
pub enum Fault {
    MainNotFound,
    BlockEarlyReturn(Stmt),
    OutsideLoop(Expr),
    UndeterminedValue,
    NotImplemented,
    Internal,
    UnacceptableVoid,
    ValueNotDefined,
    InvalidPath,
}

impl Fault {
    pub fn recover(self, intern: &Intern) -> String {
        const START: &str = ">>> ";
        let mut msg = "Cyrene: ".to_string();
        match self {
            Fault::MainNotFound => msg.push_str("program entry not found"),
            Fault::BlockEarlyReturn(stmt) => {
                msg.push_str("the node below appears after the block is returned\n");
                msg.push_str(START);
                stmt.recover(&mut msg, START, 0, intern).unwrap();
            }
            Fault::OutsideLoop(expr) => {
                msg.push_str("the node below is not inside a loop\n");
                msg.push_str(START);
                expr.recover(&mut msg, START, 0, intern).unwrap();
            }
            Fault::UndeterminedValue => {}
            Fault::NotImplemented => {}
            Fault::Internal => {}
            Fault::UnacceptableVoid => {}
            Fault::ValueNotDefined => {}
            Fault::InvalidPath => {}
        }
        msg.push('\n');
        msg
    }
}
