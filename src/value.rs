use crate::error::{Error, EvalError};

#[derive(Debug, Clone, Copy, PartialEq)]

pub enum ValueType {
    Int,
    Float,
    Boolean,
}

impl ValueType {
    pub fn rank(self) -> i8 {
        match self {
            ValueType::Boolean => 0,
            ValueType::Int => 1,
            ValueType::Float => 2,
            _ => -1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Boolean(bool),
}

impl Value {
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Int(_) => ValueType::Int,
            Value::Float(_) => ValueType::Float,
            Value::Boolean(_) => ValueType::Boolean,
        }
    }

    pub fn promote(&self, target: ValueType) -> Option<Value> {
        match (self, target) {
            (Value::Int(v), ValueType::Boolean) => Some(Value::Boolean(*v != 0)),
            (v @ Value::Int(_), ValueType::Int) => Some(*v),
            (Value::Int(v), ValueType::Float) => Some(Value::Float(*v as f64)),

            (Value::Float(v), ValueType::Boolean) => Some(Value::Boolean(*v != 0.0)),
            (Value::Float(v), ValueType::Int) => Some(Value::Int(*v as i64)),
            (v @ Value::Float(_), ValueType::Float) => Some(*v),

            (v @ Value::Boolean(_), ValueType::Boolean) => Some(*v),
            (Value::Boolean(v), ValueType::Int) => Some(Value::Int(if *v { 1i64 } else { 0i64 })),
            (Value::Boolean(v), ValueType::Float) => {
                Some(Value::Float(if *v { 1f64 } else { 0f64 }))
            }

            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(val) => Some(*val),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(val) => Some(*val),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(val) => Some(*val),
            _ => None,
        }
    }

    pub fn set_int(&mut self, value: i64) -> bool {
        match self {
            Value::Int(val) => {
                *val = value;
                true
            }
            _ => false,
        }
    }

    pub fn set_float(&mut self, value: f64) -> bool {
        match self {
            Value::Float(val) => {
                *val = value;
                true
            }
            _ => false,
        }
    }

    pub fn set_boolean(&mut self, value: bool) -> bool {
        match self {
            Value::Boolean(val) => {
                *val = value;
                true
            }
            _ => false,
        }
    }

    pub fn symbol(&self) -> String {
        match self {
            Value::Boolean(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            _ => String::new(),
        }
    }
}

pub fn unify(values: &[&Value]) -> Result<Vec<Value>, Error> {
    if values.is_empty() {
        Ok(Vec::new())
    } else {
        let target = values
            .iter()
            .map(|v| v.value_type())
            .max_by_key(|v| v.rank())
            .unwrap();

        let promoted = values.iter().cloned().map(|v| v.promote(target)).collect();

        match promoted {
            Some(v) => Ok(v),
            None => Err(Error::EvalError(EvalError::UnableToUnify {
                values: Vec::from_iter(values.iter().map(|v| (*v).clone())),
            })),
        }
    }
}

pub fn unify_to(values: &[&Value], target: ValueType) -> Result<Vec<Value>, Error> {
    if values.is_empty() {
        Ok(Vec::new())
    } else {
        let promoted = values.iter().cloned().map(|v| v.promote(target)).collect();

        match promoted {
            Some(v) => Ok(v),
            None => Err(Error::EvalError(EvalError::UnableToUnify {
                values: Vec::from_iter(values.iter().map(|v| (*v).clone())),
            })),
        }
    }
}
