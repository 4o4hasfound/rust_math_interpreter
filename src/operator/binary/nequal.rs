use crate::error::Error;
use crate::value::{Value, unify};

pub fn apply(left: &Value, right: &Value) -> Result<(Value, bool), Error> {
    match unify(&[*left, *right]) {
        Ok(v) => match (v[0], v[1]) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok((Value::Boolean(a != b), false)),
            (Value::Int(a), Value::Int(b)) => Ok((Value::Boolean(a != b), false)),
            (Value::Float(a), Value::Float(b)) => {
                Ok((Value::Boolean((a - b).abs() >= f64::EPSILON), false))
            }
            _ => Err(Error::UnexpectedError),
        },
        Err(err) => Err(err),
    }
}
