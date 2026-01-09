use crate::error::Error;
use crate::value::Value;

pub fn apply(left: &mut Value, right: &Value) -> Result<(Value, bool), Error> {
    let promoted = right
        .promote(left.value_type())
        .ok_or(Error::UnexpectedError)?;

    match (left, promoted) {
        (Value::Boolean(a), Value::Boolean(b)) => {
            *a = b;
            Ok((Value::Boolean(*a), false))
        }
        (Value::Int(a), Value::Int(b)) => {
            *a = b;
            Ok((Value::Int(*a), false))
        }
        (Value::Float(a), Value::Float(b)) => {
            *a = b;
            Ok((Value::Float(*a), false))
        }
        _ => Err(Error::UnexpectedError),
    }
}
