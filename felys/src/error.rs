#[derive(Debug)]
pub enum Fault {
    FailedToParse,
    NoReturnValue,
    InvalidPath,
    BlockEarlyEnd,
    EntryNotFound,
    StrNotInterned,
    InvalidConst
}
