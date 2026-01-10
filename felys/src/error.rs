use std::panic::Location;

#[derive(Clone, Debug)]
pub enum Fault {
    OutsideLoop,
    FailedToParse,
    UnacceptableVoid(&'static Location<'static>),
    InvalidPath,
    BlockEarlyEnd,
    InvalidConstant,
    InvalidOperation,
    UndeterminedValue,
    ValueUnreachable,
    NotImplemented,
    ValueNotDefined,
    Here(String),
}

impl Fault {
    #[track_caller]
    pub fn here() -> Self {
        let location = Location::caller();
        Fault::Here(location.to_string())
    }
}
