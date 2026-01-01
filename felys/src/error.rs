#[derive(Clone, Debug)]
pub enum Fault {
    FailedToParse,
    NoReturnValue,
    InvalidPath,
    BlockEarlyEnd,
    MainNotFound,
    StrNotInterned,
    InvalidConst,
    Todo,
}
