use crate::operator::Operator;
use crate::value::{Value, ValueType};

#[derive(Debug, Clone, PartialEq)]
pub enum Arity {
    Unary,
    Binary,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NameKind {
    Identifier,
    Function,
    Variable,
    // add more if you need them
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    /// Operand types do not match what the operator requires.
    TypeMismatch {
        op: Operator,
        arity: Arity,
        found: Vec<ValueType>,
        expected: Vec<ValueType>,
    },

    /// Operator is not defined for the given operand types.
    OpNotSupported {
        op: Operator,
        operand_types: Vec<ValueType>,
    },

    /// Attempted to divide by zero (or equivalent invalid divisor).
    DivideByZero { lhs: Value, rhs: Value },

    /// A set of values could not be unified to a single type/value.
    UnableToUnify { values: Vec<Value> },

    /// A referenced name was not found in the current environment/scope.
    NameNotFound { kind: NameKind, name: String },

    /// Operands are invalid for this operation (even if types look acceptable).
    InvalidOperands { op: Operator, operands: Vec<Value> },

    /// Operation produced an invalid/undefined result.
    InvalidResult {
        op: Operator,
        operands: Vec<Value>,
        result: Value,
    },

    /// Tried to assign/update something that is not assignable.
    NotAssignable { op: Operator },
}

pub fn error_to_string(err: EvalError) -> String {
    match err {
        EvalError::TypeMismatch {
            op,
            arity,
            found,
            expected,
        } => {
            format!(
                "Type mismatch for {:?} {:?} operator: expected {:?}, found {:?}",
                arity, op, expected, found
            )
        }

        EvalError::OpNotSupported { op, operand_types } => {
            format!(
                "Operator {:?} is not supported for operand types {:?}",
                op, operand_types
            )
        }

        EvalError::DivideByZero { .. } => "Division by zero".to_string(),

        EvalError::UnableToUnify { values } => {
            format!("Unable to unify values {:?}", values)
        }

        EvalError::NameNotFound { kind, name } => {
            format!("{:?} '{}' not found", kind, name)
        }

        EvalError::InvalidOperands { op, operands } => {
            format!("Invalid operands {:?} for operator {:?}", operands, op)
        }

        EvalError::InvalidResult {
            op,
            operands,
            result,
        } => {
            format!(
                "Operator {:?} applied to {:?} produced invalid result {:?}",
                op, operands, result
            )
        }

        EvalError::NotAssignable { op } => {
            format!("Result of operator {:?} is not assignable", op)
        }
    }
}
