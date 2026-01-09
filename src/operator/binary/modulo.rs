use crate::error::{Error, EvalError};
use crate::operator::{BinaryOp, Operator};
use crate::value::{Value, unify};

pub fn apply(left: &Value, right: &Value) -> Result<(Value, bool), Error> {
    match unify(&[*left, *right]) {
        Ok(v) => match (v[0], v[1]) {
            (Value::Boolean(_a), Value::Boolean(_b)) => {
                Err(Error::EvalError(EvalError::OpNotSupported {
                    op: Operator::Binary(BinaryOp::AddAssign),
                    operand_types: Vec::from_iter([left.value_type(), right.value_type()]),
                }))
            }
            (Value::Int(a), Value::Int(b)) => {
                let (v, overflow) = a.overflowing_rem(b);
                Ok((Value::Int(v), overflow))
            }
            (Value::Float(a), Value::Float(b)) => {
                let v = a % b;
                Ok((Value::Float(v), !v.is_finite()))
            }
            _ => Err(Error::UnexpectedError),
        },
        Err(err) => Err(err),
    }
}
