#[derive(Debug)]
#[derive(Clone)]
pub enum Fault {
    FailedToParse,
    NoReturnValue,
    InvalidPath,
    BlockEarlyEnd,
    EntryNotFound,
    StrNotInterned,
    InvalidConst,
    Todo
}
