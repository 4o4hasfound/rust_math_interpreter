use crate::error::{Error, EvalError};
use crate::operator::{BinaryOp, Operator};
use crate::value::Value;

pub fn apply(left: &mut Value, right: &Value) -> Result<(Value, bool), Error> {
    let promoted = right
        .promote(left.value_type())
        .ok_or(Error::UnexpectedError)?;

    match (*left, promoted) {
        (Value::Boolean(_), Value::Boolean(_)) => {
            Err(Error::EvalError(EvalError::OpNotSupported {
                op: Operator::Binary(BinaryOp::AddAssign),
                operand_types: Vec::from_iter([left.value_type(), right.value_type()]),
            }))
        }
        (Value::Int(a), Value::Int(b)) => {
            left.set_int(a * b);
            Ok((Value::Int(a * b), false))
        }
        (Value::Float(a), Value::Float(b)) => {
            left.set_float(a * b);
            Ok((Value::Float(a * b), false))
        }
        _ => Err(Error::UnexpectedError),
    }
}
