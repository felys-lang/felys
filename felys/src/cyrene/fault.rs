use crate::ast::{Expr, Stmt};
use crate::philia093::Intern;

#[derive(Debug, Clone)]
pub enum Fault {
    MainNotFound,
    BlockEarlyReturn(Stmt),
    OutsideLoop(Expr),
    PathNotExist(Expr),
    DuplicatePath,
    UndeterminedValue,
    Internal,
    UnacceptableVoid,
    ValueNotDefined,
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
                msg.push_str("this flow controller below must stay inside a loop\n");
                msg.push_str(START);
                expr.recover(&mut msg, START, 0, intern).unwrap();
            }
            Fault::PathNotExist(expr) => {
                msg.push_str("this path does not lead to anywhere\n");
                msg.push_str(START);
                expr.recover(&mut msg, START, 0, intern).unwrap();
            }
            Fault::DuplicatePath => msg.push_str("duplicate path"),
            Fault::UndeterminedValue => msg.push_str("undetermined value"),
            Fault::Internal => msg.push_str("internal error"),
            Fault::UnacceptableVoid => msg.push_str("unacceptable void"),
            Fault::ValueNotDefined => msg.push_str("value not defined"),
        }
        msg.push('\n');
        msg
    }
}
