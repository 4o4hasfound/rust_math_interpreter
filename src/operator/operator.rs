#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Not, // !
    Negation, // -
    BitwiseNot, // ~
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Addition, // +
    Subtraction, // -
    Multiplication, // *
    Division, // /
    Modulo, // %
    Exponentiation, // **
    And, // &&
    Or, // ||
    BitwiseAnd, // &
    BitwiseOr, // |
    BitwiseXor, // ^

    Equal, // ==
    NotEqual, // !=
    Less, // <
    Greater, // >
    LessEqual, // <=
    GreaterEqual, // >=

    Assign, // =
    AddAssign, // +=
    SubAssign, // -=
    MulAssign, // *=
    DivAssign, // /=
    ModAssign, // %=
    AndAssign, // &&=
    OrAssign, // ||=
    BitAndAssign, // &=
    BitOrAssign, // |=
    BitXorAssign, // ^=
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TernaryOp {
    TernaryCond, // a ? b : c
    TernaryElse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GroupingOp {
    LeftParen, // (
    RightParen, // )
    Comma, // ,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    Unary(UnaryOp),
    Binary(BinaryOp),
    TernaryOp(TernaryOp),
    Grouping(GroupingOp),
}

impl Operator {
    pub fn symbol(self) -> &'static str {
        match self {
            Operator::Unary(UnaryOp::Not) => "!",
            Operator::Unary(UnaryOp::Negation) => "-",
            Operator::Unary(UnaryOp::BitwiseNot) => "~",

            Operator::Binary(BinaryOp::Addition) => "+",
            Operator::Binary(BinaryOp::Subtraction) => "-",
            Operator::Binary(BinaryOp::Multiplication) => "*",
            Operator::Binary(BinaryOp::Division) => "/",
            Operator::Binary(BinaryOp::Modulo) => "%",
            Operator::Binary(BinaryOp::Exponentiation) => "**",
            Operator::Binary(BinaryOp::And) => "&&",
            Operator::Binary(BinaryOp::Or) => "||",
            Operator::Binary(BinaryOp::BitwiseAnd) => "&",
            Operator::Binary(BinaryOp::BitwiseOr) => "|",
            Operator::Binary(BinaryOp::BitwiseXor) => "^",

            Operator::Binary(BinaryOp::Assign) => "=", // =
            Operator::Binary(BinaryOp::AddAssign) => "+=", // +=
            Operator::Binary(BinaryOp::SubAssign) => "-=", // -=
            Operator::Binary(BinaryOp::MulAssign) => "*=", // *=
            Operator::Binary(BinaryOp::DivAssign) => "/=", // /=
            Operator::Binary(BinaryOp::ModAssign) => "%=", // %=
            Operator::Binary(BinaryOp::AndAssign) => "&&=", // &&=
            Operator::Binary(BinaryOp::OrAssign) => "||=", // ||=
            Operator::Binary(BinaryOp::BitAndAssign) => "&=", // &=
            Operator::Binary(BinaryOp::BitOrAssign) => "|=", // |=
            Operator::Binary(BinaryOp::BitXorAssign) => "^=", // ^=

            Operator::Binary(BinaryOp::Equal) => "==",
            Operator::Binary(BinaryOp::NotEqual) => "!=",
            Operator::Binary(BinaryOp::Less) => "<",
            Operator::Binary(BinaryOp::LessEqual) => "<=",
            Operator::Binary(BinaryOp::Greater) => ">",
            Operator::Binary(BinaryOp::GreaterEqual) => ">=",

            Operator::TernaryOp(TernaryOp::TernaryCond) => "?",
            Operator::TernaryOp(TernaryOp::TernaryElse) => ":",

            Operator::Grouping(GroupingOp::LeftParen) => "(",
            Operator::Grouping(GroupingOp::RightParen) => ")",
            Operator::Grouping(GroupingOp::Comma) => ",",
        }
    }
}
