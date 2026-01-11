use crate::{
    error::{ Error, EvalError, NameKind },
    operator::*,
    parser::Expr,
    span::{ Span, Spanned },
    value::{ Value, ValueType },
};

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum EvalResult<'a> {
    Value(Value),
    Ref(&'a mut Value),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalResultType {
    Value,
    Ref,
}

impl<'a> EvalResult<'a> {
    pub fn result_type(&self) -> EvalResultType {
        match self {
            EvalResult::Value(_) => EvalResultType::Value,
            EvalResult::Ref(_) => EvalResultType::Ref,
        }
    }

    pub fn as_value(&self) -> Option<Value> {
        match self {
            EvalResult::Value(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_ref(&mut self) -> Option<&mut Value> {
        match self {
            EvalResult::Ref(v) => Some(v),
            _ => None,
        }
    }
}

fn is_assign(op: Operator) -> bool {
    match op {
        | Operator::Binary(BinaryOp::Assign)
        | Operator::Binary(BinaryOp::AddAssign)
        | Operator::Binary(BinaryOp::SubAssign)
        | Operator::Binary(BinaryOp::MulAssign)
        | Operator::Binary(BinaryOp::DivAssign)
        | Operator::Binary(BinaryOp::ModAssign)
        | Operator::Binary(BinaryOp::AndAssign)
        | Operator::Binary(BinaryOp::OrAssign)
        | Operator::Binary(BinaryOp::BitAndAssign)
        | Operator::Binary(BinaryOp::BitOrAssign)
        | Operator::Binary(BinaryOp::BitXorAssign) => true,
        _ => false,
    }
}

fn is_cond(op: Operator) -> bool {
    match op {
        Operator::Binary(BinaryOp::And) | Operator::Binary(BinaryOp::Or) => true,
        _ => false,
    }
}

pub fn evaluate_expr<'a>(
    expr: &Spanned<Expr>,
    variables: &'a mut HashMap<String, Value>,
    user_def_functions: &'a mut HashMap<String, Box<Spanned<Expr>>>,
    functions: &HashMap<String, fn(&Vec<Value>) -> Result<Value, Error>>
) -> Result<EvalResult<'a>, Spanned<Error>> {
    match &expr.data {
        Expr::Value(v) => Ok(EvalResult::Value(*v)),
        Expr::Identifier(s) => {
            if let Some(v) = variables.get_mut(s) {
                Ok(EvalResult::Ref(v))
            } else {
                Err(Spanned {
                    span: expr.span,
                    data: Error::EvalError(EvalError::NameNotFound {
                        kind: NameKind::Variable,
                        name: s.to_string(),
                    }),
                })
            }
        }
        Expr::Macro(s) => {
            if let Some(v) = user_def_functions.get(s).cloned() {
                evaluate_expr(&v, variables, user_def_functions, functions)
            } else {
                Err(Spanned {
                    span: expr.span,
                    data: Error::EvalError(EvalError::NameNotFound {
                        kind: NameKind::Macro,
                        name: s.to_string(),
                    }),
                })
            }
        }
        Expr::Binary { op, lhs, rhs } => {
            let mut l: EvalResult<'_>;
            let left: Value;
            let right: Value;

            if !is_assign(*op) {
                l = evaluate_expr(&lhs, variables, user_def_functions, functions)?;
                left = (
                    match l.result_type() {
                        EvalResultType::Value => l.as_value(),
                        EvalResultType::Ref => l.as_ref().cloned(),
                    }
                ).ok_or(Spanned {
                    span: expr.span,
                    data: Error::UnexpectedError,
                })?;
            } else {
                left = Value::Boolean(false); // dummy
            }

            if !is_cond(*op) {
                let mut r = evaluate_expr(&rhs, variables, user_def_functions, functions)?;
                right = (
                    match r.result_type() {
                        EvalResultType::Value => r.as_value(),
                        EvalResultType::Ref => r.as_ref().cloned(),
                    }
                ).ok_or(Spanned {
                    span: expr.span,
                    data: Error::UnexpectedError,
                })?;
            } else {
                right = Value::Boolean(false); // dummy
            }

            let result = (
                match op {
                    Operator::Binary(BinaryOp::Addition) => add::apply(&left, &right),
                    Operator::Binary(BinaryOp::Subtraction) => sub::apply(&left, &right),
                    Operator::Binary(BinaryOp::Multiplication) => mul::apply(&left, &right),
                    Operator::Binary(BinaryOp::Division) => div::apply(&left, &right),
                    Operator::Binary(BinaryOp::Modulo) => modulo::apply(&left, &right),
                    Operator::Binary(BinaryOp::Exponentiation) => exp::apply(&left, &right),
                    Operator::Binary(BinaryOp::BitwiseAnd) => bit_and::apply(&left, &right),
                    Operator::Binary(BinaryOp::BitwiseOr) => bit_or::apply(&left, &right),
                    Operator::Binary(BinaryOp::BitwiseXor) => bit_xor::apply(&left, &right),

                    Operator::Binary(BinaryOp::And) => {
                        if let Some(Value::Boolean(b)) = left.promote(ValueType::Boolean) && !b {
                            return Ok(EvalResult::Value(Value::Boolean(false)));
                        }
                        let mut r = evaluate_expr(&rhs, variables, user_def_functions, functions)?;
                        let right = (
                            match r.result_type() {
                                EvalResultType::Value => r.as_value(),
                                EvalResultType::Ref => r.as_ref().cloned(),
                            }
                        ).ok_or(Spanned {
                            span: expr.span,
                            data: Error::UnexpectedError,
                        })?;
                        and::apply(&left, &right)
                    }
                    Operator::Binary(BinaryOp::Or) => {
                        if let Some(Value::Boolean(b)) = left.promote(ValueType::Boolean) && b {
                            return Ok(EvalResult::Value(Value::Boolean(true)));
                        }
                        let mut r = evaluate_expr(&rhs, variables, user_def_functions, functions)?;
                        let right = (
                            match r.result_type() {
                                EvalResultType::Value => r.as_value(),
                                EvalResultType::Ref => r.as_ref().cloned(),
                            }
                        ).ok_or(Spanned {
                            span: expr.span,
                            data: Error::UnexpectedError,
                        })?;
                        and::apply(&left, &right)
                    }

                    Operator::Binary(BinaryOp::Assign) =>
                        match &(**lhs).data {
                            Expr::Identifier(name) => {
                                let slot = variables.entry(name.clone()).or_insert_with(|| {
                                    match right.value_type() {
                                        ValueType::Boolean => Value::Boolean(false),
                                        ValueType::Int => Value::Int(0),
                                        ValueType::Float => Value::Float(0.0),
                                    }
                                });

                                assign::apply(slot, &right)
                            }
                            _ =>
                                Err(
                                    Error::EvalError(EvalError::NotAssignable {
                                        op: Operator::Binary(BinaryOp::Assign),
                                    })
                                ),
                        } // =
                    Operator::Binary(BinaryOp::AddAssign) =>
                        match &(**lhs).data {
                            Expr::Identifier(name) => {
                                let slot = variables.get_mut(name).ok_or(Spanned {
                                    span: expr.span,
                                    data: Error::EvalError(EvalError::NameNotFound {
                                        kind: NameKind::Variable,
                                        name: name.clone(),
                                    }),
                                })?;

                                add_assign::apply(slot, &right)
                            }
                            _ =>
                                Err(
                                    Error::EvalError(EvalError::NotAssignable {
                                        op: Operator::Binary(BinaryOp::Assign),
                                    })
                                ),
                        } // +=
                    Operator::Binary(BinaryOp::SubAssign) =>
                        match &(**lhs).data {
                            Expr::Identifier(name) => {
                                let slot = variables.get_mut(name).ok_or(Spanned {
                                    span: expr.span,
                                    data: Error::EvalError(EvalError::NameNotFound {
                                        kind: NameKind::Variable,
                                        name: name.clone(),
                                    }),
                                })?;

                                sub_assign::apply(slot, &right)
                            }
                            _ =>
                                Err(
                                    Error::EvalError(EvalError::NotAssignable {
                                        op: Operator::Binary(BinaryOp::Assign),
                                    })
                                ),
                        } // -=
                    Operator::Binary(BinaryOp::MulAssign) =>
                        match &(**lhs).data {
                            Expr::Identifier(name) => {
                                let slot = variables.get_mut(name).ok_or(Spanned {
                                    span: expr.span,
                                    data: Error::EvalError(EvalError::NameNotFound {
                                        kind: NameKind::Variable,
                                        name: name.clone(),
                                    }),
                                })?;

                                mul_assign::apply(slot, &right)
                            }
                            _ =>
                                Err(
                                    Error::EvalError(EvalError::NotAssignable {
                                        op: Operator::Binary(BinaryOp::Assign),
                                    })
                                ),
                        } // *=
                    Operator::Binary(BinaryOp::DivAssign) =>
                        match &(**lhs).data {
                            Expr::Identifier(name) => {
                                let slot = variables.get_mut(name).ok_or(Spanned {
                                    span: expr.span,
                                    data: Error::EvalError(EvalError::NameNotFound {
                                        kind: NameKind::Variable,
                                        name: name.clone(),
                                    }),
                                })?;

                                div_assign::apply(slot, &right)
                            }
                            _ =>
                                Err(
                                    Error::EvalError(EvalError::NotAssignable {
                                        op: Operator::Binary(BinaryOp::Assign),
                                    })
                                ),
                        } // /=
                    Operator::Binary(BinaryOp::ModAssign) =>
                        match &(**lhs).data {
                            Expr::Identifier(name) => {
                                let slot = variables.get_mut(name).ok_or(Spanned {
                                    span: expr.span,
                                    data: Error::EvalError(EvalError::NameNotFound {
                                        kind: NameKind::Variable,
                                        name: name.clone(),
                                    }),
                                })?;

                                mod_assign::apply(slot, &right)
                            }
                            _ =>
                                Err(
                                    Error::EvalError(EvalError::NotAssignable {
                                        op: Operator::Binary(BinaryOp::Assign),
                                    })
                                ),
                        } // %=
                    Operator::Binary(BinaryOp::AndAssign) =>
                        match &(**lhs).data {
                            Expr::Identifier(name) => {
                                let slot = variables.get_mut(name).ok_or(Spanned {
                                    span: expr.span,
                                    data: Error::EvalError(EvalError::NameNotFound {
                                        kind: NameKind::Variable,
                                        name: name.clone(),
                                    }),
                                })?;

                                and_assign::apply(slot, &right)
                            }
                            _ =>
                                Err(
                                    Error::EvalError(EvalError::NotAssignable {
                                        op: Operator::Binary(BinaryOp::Assign),
                                    })
                                ),
                        } // &&=
                    Operator::Binary(BinaryOp::OrAssign) =>
                        match &(**lhs).data {
                            Expr::Identifier(name) => {
                                let slot = variables.get_mut(name).ok_or(Spanned {
                                    span: expr.span,
                                    data: Error::EvalError(EvalError::NameNotFound {
                                        kind: NameKind::Variable,
                                        name: name.clone(),
                                    }),
                                })?;

                                or_assign::apply(slot, &right)
                            }
                            _ =>
                                Err(
                                    Error::EvalError(EvalError::NotAssignable {
                                        op: Operator::Binary(BinaryOp::Assign),
                                    })
                                ),
                        } // ||=
                    Operator::Binary(BinaryOp::BitAndAssign) =>
                        match &(**lhs).data {
                            Expr::Identifier(name) => {
                                let slot = variables.get_mut(name).ok_or(Spanned {
                                    span: expr.span,
                                    data: Error::EvalError(EvalError::NameNotFound {
                                        kind: NameKind::Variable,
                                        name: name.clone(),
                                    }),
                                })?;

                                bitand_assign::apply(slot, &right)
                            }
                            _ =>
                                Err(
                                    Error::EvalError(EvalError::NotAssignable {
                                        op: Operator::Binary(BinaryOp::Assign),
                                    })
                                ),
                        } // &=
                    Operator::Binary(BinaryOp::BitOrAssign) =>
                        match &(**lhs).data {
                            Expr::Identifier(name) => {
                                let slot = variables.get_mut(name).ok_or(Spanned {
                                    span: expr.span,
                                    data: Error::EvalError(EvalError::NameNotFound {
                                        kind: NameKind::Variable,
                                        name: name.clone(),
                                    }),
                                })?;

                                bitor_assign::apply(slot, &right)
                            }
                            _ =>
                                Err(
                                    Error::EvalError(EvalError::NotAssignable {
                                        op: Operator::Binary(BinaryOp::Assign),
                                    })
                                ),
                        } // |=
                    Operator::Binary(BinaryOp::BitXorAssign) =>
                        match &(**lhs).data {
                            Expr::Identifier(name) => {
                                let slot = variables.get_mut(name).ok_or(Spanned {
                                    span: expr.span,
                                    data: Error::EvalError(EvalError::NameNotFound {
                                        kind: NameKind::Variable,
                                        name: name.clone(),
                                    }),
                                })?;

                                bitxor_assign::apply(slot, &right)
                            }
                            _ =>
                                Err(
                                    Error::EvalError(EvalError::NotAssignable {
                                        op: Operator::Binary(BinaryOp::Assign),
                                    })
                                ),
                        } // ^=

                    Operator::Binary(BinaryOp::Equal) => equal::apply(&left, &right),
                    Operator::Binary(BinaryOp::NotEqual) => nequal::apply(&left, &right),
                    Operator::Binary(BinaryOp::Less) => less::apply(&left, &right),
                    Operator::Binary(BinaryOp::LessEqual) => lequal::apply(&left, &right),
                    Operator::Binary(BinaryOp::Greater) => greater::apply(&left, &right),
                    Operator::Binary(BinaryOp::GreaterEqual) => gequal::apply(&left, &right),

                    _ => Err(Error::UnexpectedError),
                }
            ).map_err(|err| Spanned {
                span: Span {
                    start: lhs.span.start,
                    end: rhs.span.end,
                },
                data: err,
            })?;

            if result.1 {
                Err(Spanned {
                    span: expr.span,
                    data: Error::EvalError(EvalError::InvalidResult {
                        op: *op,
                        operands: Vec::from_iter([left, right]),
                        result: result.0,
                    }),
                })
            } else {
                Ok(EvalResult::Value(result.0))
            }
        }
        Expr::Unary { op, rhs } => {
            let mut v = evaluate_expr(&rhs, variables, user_def_functions, functions)?;
            let value = (
                match v.result_type() {
                    EvalResultType::Value => v.as_value(),
                    EvalResultType::Ref => v.as_ref().cloned(),
                }
            ).ok_or(Spanned {
                span: Span::merge(&expr.span, &rhs.span),
                data: Error::UnexpectedError,
            })?;

            let result = (
                match op {
                    Operator::Unary(UnaryOp::Not) => not::apply(&value),
                    Operator::Unary(UnaryOp::Negation) => neg::apply(&value),
                    Operator::Unary(UnaryOp::BitwiseNot) => bit_not::apply(&value),
                    _ => Err(Error::UnexpectedError),
                }
            ).map_err(|err| Spanned {
                span: Span::merge(&expr.span, &rhs.span),
                data: err,
            })?;

            if result.1 {
                Err(Spanned {
                    span: Span::merge(&expr.span, &rhs.span),
                    data: Error::EvalError(EvalError::InvalidResult {
                        op: *op,
                        operands: Vec::from_iter([value]),
                        result: result.0,
                    }),
                })
            } else {
                Ok(EvalResult::Value(result.0))
            }
        }
        Expr::Ternary { cond, statement1, statement2 } => {
            let mut cond_v = evaluate_expr(&cond, variables, user_def_functions, functions)?;
            let cond_value = (
                match cond_v.result_type() {
                    EvalResultType::Value => cond_v.as_value(),
                    EvalResultType::Ref => cond_v.as_ref().cloned(),
                }
            )
                .ok_or(Error::UnexpectedError)
                .map_err(|err| Spanned {
                    span: Span {
                        start: cond.span.start,
                        end: statement2.span.end,
                    },
                    data: err,
                })?;

            let cond_bool = cond_value
                .promote(ValueType::Boolean)
                .ok_or(Error::UnexpectedError)
                .map_err(|err| Spanned {
                    span: Span {
                        start: cond.span.start,
                        end: statement2.span.end,
                    },
                    data: err,
                })?;

            if let Some(b) = cond_bool.as_boolean() {
                if b {
                    evaluate_expr(&statement1, variables, user_def_functions, functions)
                } else {
                    evaluate_expr(&statement2, variables, user_def_functions, functions)
                }
            } else {
                Err(Spanned {
                    span: Span {
                        start: cond.span.start,
                        end: statement2.span.end,
                    },
                    data: Error::UnexpectedError,
                })
            }
        }
        Expr::Call { func, args } =>
            match &func.data {
                Expr::Identifier(s) => {
                    let f = functions.get(s).ok_or(Spanned {
                        span: Span {
                            start: func.span.start,
                            end: if let Some(e) = args.last() {
                                e.span.end
                            } else {
                                func.span.end
                            },
                        },
                        data: Error::EvalError(EvalError::NameNotFound {
                            kind: NameKind::Function,
                            name: s.to_string(),
                        }),
                    })?;

                    let mut v = Vec::new();

                    for in_arg in args.iter() {
                        let res = evaluate_expr(&in_arg, variables, user_def_functions, functions);

                        if let Err(err) = res {
                            return Err(err);
                        } else if let Ok(r) = res {
                            match r {
                                EvalResult::Value(res_val) => v.push(res_val),
                                EvalResult::Ref(res_ref) => v.push(res_ref.clone()),
                            }
                        }
                    }

                    let result = f(&v).map_err(|err| Spanned {
                        span: Span {
                            start: func.span.start,
                            end: if let Some(e) = args.last() {
                                e.span.end
                            } else {
                                func.span.end
                            },
                        },
                        data: err,
                    })?;
                    Ok(EvalResult::Value(result))
                }
                _ =>
                    Err(Spanned {
                        span: expr.span,
                        data: Error::UnexpectedError,
                    }),
            }
        Expr::Comma { exprs } => {
            for i in 0..exprs.len() {
                if i == exprs.len() - 1 {
                    return evaluate_expr(&exprs[i], variables, user_def_functions, functions);
                } else {
                    let _ = evaluate_expr(&exprs[i], variables, user_def_functions, functions);
                }
            }
            Err(Spanned { span: expr.span, data: Error::UnexpectedError })
        }
    }
}
