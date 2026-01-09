use crate::error::{Error, EvalError};
use crate::operator::{BinaryOp, Operator};
use crate::value::Value;

pub fn apply(value: &Value) -> Result<(Value, bool), Error> {
    match value {
        Value::Boolean(_) => Err(Error::EvalError(EvalError::OpNotSupported {
            op: Operator::Binary(BinaryOp::AddAssign),
            operand_types: Vec::from_iter([value.value_type()]),
        })),
        Value::Int(a) => Ok((Value::Int(-a), false)),
        Value::Float(a) => Ok((Value::Float(-a), false)),
    }
}
