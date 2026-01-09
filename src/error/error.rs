use crate::error::eval_error::*;
use crate::error::lexing_error::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    LexingError(LexingError),
    EvalError(EvalError),
    UnexpectedError,
}
