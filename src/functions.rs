use crate::{ error::{ Error, EvalError }, value::{ Value, ValueType, unify_ret_type, unify_to } };

pub fn to_bool(input: &Vec<Value>) -> Result<Value, Error> {
    if input.len() != 1 {
        Err(
            Error::EvalError(EvalError::ArityMismatch {
                func: "to_bool".to_string(),
                expected: 1,
                found: input.len(),
            })
        )
    } else {
        let value = input[0].promote(ValueType::Boolean).unwrap();
        Ok(value)
    }
}

pub fn to_int(input: &Vec<Value>) -> Result<Value, Error> {
    if input.len() != 1 {
        Err(
            Error::EvalError(EvalError::ArityMismatch {
                func: "to_int".to_string(),
                expected: 1,
                found: input.len(),
            })
        )
    } else {
        let value = input[0].promote(ValueType::Int).unwrap();
        Ok(value)
    }
}

pub fn to_float(input: &Vec<Value>) -> Result<Value, Error> {
    if input.len() != 1 {
        Err(
            Error::EvalError(EvalError::ArityMismatch {
                func: "to_float".to_string(),
                expected: 1,
                found: input.len(),
            })
        )
    } else {
        let value = input[0].promote(ValueType::Float).unwrap();
        Ok(value)
    }
}

pub fn any(input: &Vec<Value>) -> Result<Value, Error> {
    let promoted = unify_to(input, ValueType::Boolean)?;

    Ok(Value::Boolean(promoted.iter().any(|v| matches!(v, Value::Boolean(true)))))
}

pub fn all(input: &Vec<Value>) -> Result<Value, Error> {
    let promoted = unify_to(input, ValueType::Boolean)?;

    Ok(Value::Boolean(promoted.iter().all(|v| matches!(v, Value::Boolean(true)))))
}

pub fn max(input: &Vec<Value>) -> Result<Value, Error> {
    let (promoted, promoted_type) = unify_ret_type(input)?;

    match promoted_type {
        ValueType::Boolean => {
            let v = promoted
                .iter()
                .map(|v| v.as_boolean().unwrap())
                .max()
                .ok_or(Error::UnexpectedError)?;
            Ok(Value::Boolean(v))
        }
        ValueType::Int => {
            let v = promoted
                .iter()
                .map(|v| v.as_int().unwrap())
                .max()
                .ok_or(Error::UnexpectedError)?;
            Ok(Value::Int(v))
        }
        ValueType::Float => {
            let v = promoted
                .iter()
                .map(|v| v.as_float().unwrap())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .ok_or(Error::UnexpectedError)?;
            Ok(Value::Float(v))
        }
    }
}

pub fn min(input: &Vec<Value>) -> Result<Value, Error> {
    let (promoted, promoted_type) = unify_ret_type(input)?;

    match promoted_type {
        ValueType::Boolean => {
            let v = promoted
                .iter()
                .map(|v| v.as_boolean().unwrap())
                .min()
                .ok_or(Error::UnexpectedError)?;
            Ok(Value::Boolean(v))
        }
        ValueType::Int => {
            let v = promoted
                .iter()
                .map(|v| v.as_int().unwrap())
                .min()
                .ok_or(Error::UnexpectedError)?;
            Ok(Value::Int(v))
        }
        ValueType::Float => {
            let v = promoted
                .iter()
                .map(|v| v.as_float().unwrap())
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .ok_or(Error::UnexpectedError)?;
            Ok(Value::Float(v))
        }
    }
}

pub fn clamp(input: &Vec<Value>) -> Result<Value, Error> {
    if input.len() != 3 {
        Err(
            Error::EvalError(EvalError::ArityMismatch {
                func: "clamp".to_string(),
                expected: 3,
                found: input.len(),
            })
        )
    } else {
        let (promoted, promoted_type) = unify_ret_type(input)?;
        let (clamp_min, clamp_max, value) = (promoted[0], promoted[1], promoted[2]);

        match promoted_type {
            ValueType::Boolean => {
                let cmini = clamp_min.promote(ValueType::Int).unwrap().as_int().unwrap();
                let cmaxi = clamp_max.promote(ValueType::Int).unwrap().as_int().unwrap();
                let cvali = value.promote(ValueType::Int).unwrap().as_int().unwrap();

                Ok(Value::Int(cvali.clamp(cmini, cmaxi)))
            }
            ValueType::Int => {
                let cmini = clamp_min.as_int().unwrap();
                let cmaxi = clamp_max.as_int().unwrap();
                let cvali = value.as_int().unwrap();

                Ok(Value::Int(cvali.clamp(cmini, cmaxi)))
            }
            ValueType::Float => {
                let cminf = clamp_min.as_float().unwrap();
                let cmaxf = clamp_max.as_float().unwrap();
                let cvalf = value.as_float().unwrap();

                Ok(Value::Float(cvalf.clamp(cminf, cmaxf)))
            }
        }
    }
}
