use std::panic::Location;

#[derive(Clone, Debug)]
pub enum Fault {
    OutsideLoop,
    FailedToParse,
    UnacceptableVoid(&'static Location<'static>),
    InvalidPath,
    BlockEarlyEnd,
    MainNotFound,
    InvalidConstant,
    InvalidOperation,
    UndeterminedValue,
    ValueUnreachable,
    NotImplemented,
    ValueNotDefined,
    Runtime
}
