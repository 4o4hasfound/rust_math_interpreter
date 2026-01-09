use crate::error::Error;
use crate::value::{Value, unify};

pub fn apply(left: &Value, right: &Value) -> Result<(Value, bool), Error> {
    match unify(&[left, right]) {
        Ok(v) => match (v[0], v[1]) {
            (Value::Boolean(a), Value::Boolean(b)) => {
                let ai = if a { 1i64 } else { 0i64 };
                let bi = if b { 1i64 } else { 0i64 };
                Ok((Value::Int(ai * bi), false))
            }
            (Value::Int(a), Value::Int(b)) => {
                let (v, overflow) = a.overflowing_mul(b);
                Ok((Value::Int(v), overflow))
            }
            (Value::Float(a), Value::Float(b)) => {
                let v = a * b;
                Ok((Value::Float(v), !v.is_finite()))
            }
            _ => Err(Error::UnexpectedError),
        },
        Err(err) => Err(err),
    }
}
