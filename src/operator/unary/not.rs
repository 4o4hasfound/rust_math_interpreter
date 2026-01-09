use crate::error::Error;
use crate::value::{Value, ValueType};

pub fn apply(value: &Value) -> Result<(Value, bool), Error> {
    match value.promote(ValueType::Boolean) {
        Some(Value::Boolean(v)) => Ok((Value::Boolean(!v), false)),
        _ => Err(Error::UnexpectedError),
    }
}
