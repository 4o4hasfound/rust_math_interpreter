use crate::operator::{BinaryOp, GroupingOp, Operator, TernaryOp};
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(Operator),
    Value(Value),
    Identifier(String),
}

impl Token {
    pub fn symbol(&self) -> String {
        match self {
            Token::Operator(s) => String::from(s.symbol()),
            Token::Value(v) => v.symbol(),
            Token::Identifier(s) => s.clone(),
        }
    }

    pub fn lbp(&self) -> u32 {
        match self {
            Token::Operator(s) => match s {
                Operator::Binary(v) => match v {
                    BinaryOp::Exponentiation => 80,

                    BinaryOp::Multiplication | BinaryOp::Division | BinaryOp::Modulo => 60,

                    BinaryOp::Addition | BinaryOp::Subtraction => 50,

                    BinaryOp::Less
                    | BinaryOp::LessEqual
                    | BinaryOp::Greater
                    | BinaryOp::GreaterEqual => 40,

                    BinaryOp::Equal | BinaryOp::NotEqual => 35,

                    BinaryOp::BitwiseAnd => 30,
                    BinaryOp::BitwiseXor => 25,
                    BinaryOp::BitwiseOr => 20,

                    BinaryOp::And => 15,
                    BinaryOp::Or => 10,

                    _ => 3,
                },
                Operator::Unary(_) => 0,
                Operator::TernaryOp(v) => match v {
                    TernaryOp::TernaryCond => 5,
                    TernaryOp::TernaryElse => 0,
                },
                Operator::Grouping(v) => match v {
                    GroupingOp::LeftParen => 90,
                    GroupingOp::RightParen => 0,
                    GroupingOp::Comma => 0,
                },
            },
            Token::Value(_) => 0,
            Token::Identifier(_) => 0,
        }
    }
}
