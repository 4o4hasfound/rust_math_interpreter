use crate::error::{Error, EvalError};
use crate::operator::{BinaryOp, Operator};
use crate::value::{Value, ValueType, unify_to};

pub fn apply(left: &Value, right: &Value) -> Result<(Value, bool), Error> {
    match unify_to(&[left, right], ValueType::Boolean) {
        Ok(v) => match (v[0], v[1]) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok((Value::Boolean(a && b), false)),
            (Value::Int(_), Value::Int(_)) | (Value::Float(_), Value::Float(_)) => {
                Err(Error::EvalError(EvalError::OpNotSupported {
                    op: Operator::Binary(BinaryOp::AddAssign),
                    operand_types: Vec::from_iter([left.value_type(), right.value_type()]),
                }))
            }
            _ => Err(Error::UnexpectedError),
        },
        Err(err) => Err(err),
    }
}
