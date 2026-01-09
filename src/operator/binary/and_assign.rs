use crate::error::{Error, EvalError};
use crate::operator::{BinaryOp, Operator};
use crate::value::Value;

pub fn apply(left: &mut Value, right: &Value) -> Result<(Value, bool), Error> {
    let promoted = right
        .promote(left.value_type())
        .ok_or(Error::UnexpectedError)?;

    match (*left, promoted) {
        (Value::Boolean(a), Value::Boolean(b)) => {
            left.set_boolean(a && b);
            Ok((Value::Boolean(a && b), false))
        }
        (Value::Int(_a), Value::Int(_b)) => Err(Error::EvalError(EvalError::OpNotSupported {
            op: Operator::Binary(BinaryOp::AddAssign),
            operand_types: Vec::from_iter([left.value_type(), right.value_type()]),
        })),
        (Value::Float(_a), Value::Float(_b)) => Err(Error::EvalError(EvalError::OpNotSupported {
            op: Operator::Binary(BinaryOp::AddAssign),
            operand_types: Vec::from_iter([left.value_type(), right.value_type()]),
        })),
        _ => Err(Error::UnexpectedError),
    }
}
