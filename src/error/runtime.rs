use RuntimeError::*;

use crate::Error;

pub enum RuntimeError {
    Timeout,
    CannotAssign,
    CallStackOverflow,
    NoF64Conversion { s: String },
    IdentNotCallable { s: String },
    ArgsMappingFailed { s: String },
    ObjectDoesNotExist { s: String },
    FromRust { s: String },
}

impl From<RuntimeError> for Error {
    fn from(value: RuntimeError) -> Self {
        let msg = match value {
            Timeout => "code execution timeout".to_string(),
            CannotAssign => "left hand side is not assignable".to_string(),
            CallStackOverflow => "call stack overflow".to_string(),
            NoF64Conversion { s } => format!("object `{}` does convert to f64", s),
            IdentNotCallable { s } => format!("identifier `{}` is not callable", s),
            ArgsMappingFailed { s } => format!("calling {} requires different numbers of args", s),
            ObjectDoesNotExist { s } => format!("identifier `{}` does not exist", s),
            FromRust { s } => s
        };
        Self { msg: format!("RuntimeError: {}", msg) }
    }
}
