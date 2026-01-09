use crate::error::{Error, EvalError};
use crate::operator::{BinaryOp, Operator};
use crate::value::{Value, unify};

pub fn apply(left: &Value, right: &Value) -> Result<(Value, bool), Error> {
    match unify(&[left, right]) {
        Ok(v) => match (v[0], v[1]) {
            (Value::Boolean(a), Value::Boolean(b)) => {
                let ai = if a { 1i64 } else { 0i64 };
                let bi = if b { 1i64 } else { 0i64 };
                Ok((Value::Int(ai & bi), false))
            }
            (Value::Int(a), Value::Int(b)) => Ok((Value::Int(a & b), false)),
            (Value::Float(_), Value::Float(_)) => {
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
