use crate::error::eval_error::*;
use crate::error::lexing_error::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    LexingError(LexingError),
    EvalError(EvalError),
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    None,
    LexingError,
    EvalError,
    UnexpectedError,
}

impl Error {
    pub fn error_type(self) -> ErrorType {
        match self {
            Error::LexingError(_) => ErrorType::LexingError,
            Error::EvalError(_) => ErrorType::EvalError,
            Error::UnexpectedError => ErrorType::UnexpectedError,
            _ => ErrorType::None,
        }
    }

    pub fn as_lexing_error(self) -> Option<LexingError> {
        match self {
            Error::LexingError(err) => Some(err),
            _ => None,
        }
    }

    pub fn as_eval_error(self) -> Option<EvalError> {
        match self {
            Error::EvalError(err) => Some(err),
            _ => None,
        }
    }
}
