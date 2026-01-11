use crate::philia093::Intern;

#[derive(Debug, Clone)]
pub enum Fault {
    MainNotFound,
    BlockEarlyReturn,
    BreakOutsideLoop,
    ContinueOutsideLoop,
    UndeterminedValue,
    NotImplemented,
    Internal,
    UnacceptableVoid,
    ValueNotDefined,
    InvalidPath,
}

impl Fault {
    pub fn recover(self, intern: &Intern) -> String {
        format!("{:?}", self)
    }
}
