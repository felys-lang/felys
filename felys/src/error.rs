#[derive(Clone, Debug)]
pub enum Fault {
    FailedToParse,
    UnacceptableVoid,
    InvalidPath,
    BlockEarlyEnd,
    MainNotFound,
    InvalidConstant,
    InvalidOperation,
    UndeterminedValue,
    NotImplemented,
}
