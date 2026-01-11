use crate::{
    debug::{ print_debug_expr, print_debug_tokens },
    error::Error,
    lexer::lex_string,
    operator::{ BinaryOp, GroupingOp, Operator, TernaryOp, UnaryOp },
    span::{ Span, Spanned },
    token::Token,
    value::Value,
};

struct Cursor<'a> {
    src: &'a [Spanned<Token>],
    i: usize,
}

impl<'a> Cursor<'a> {
    fn new(src: &'a [Spanned<Token>]) -> Self {
        Cursor { src, i: 0 }
    }

    fn is_eof(&self) -> bool {
        self.i >= self.src.len()
    }

    fn peek(&self) -> Option<Spanned<Token>> {
        self.src.get(self.i).cloned()
    }

    fn next(&mut self) -> Option<Spanned<Token>> {
        let t = self.peek()?.clone();
        self.advance(1);
        Some(t)
    }

    fn advance(&mut self, n: usize) -> bool {
        self.i += n;
        self.is_eof()
    }

    fn expect(&mut self, target: &Token) -> bool {
        if let Some(t) = self.peek() && t.data == *target {
            self.advance(1);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Value(Value),
    Identifier(String),
    Macro(String),
    Unary {
        op: Operator,
        rhs: Box<Spanned<Expr>>,
    },
    Binary {
        op: Operator,
        lhs: Box<Spanned<Expr>>,
        rhs: Box<Spanned<Expr>>,
    },
    Comma {
        exprs: Vec<Spanned<Expr>>,
    },
    Ternary {
        cond: Box<Spanned<Expr>>,
        statement1: Box<Spanned<Expr>>,
        statement2: Box<Spanned<Expr>>,
    },
    Call {
        func: Box<Spanned<Expr>>,
        args: Vec<Spanned<Expr>>,
    },
}

fn nud(cursor: &mut Cursor, t: &Spanned<Token>) -> Option<Spanned<Expr>> {
    match &t.data {
        Token::Value(v) =>
            Some(Spanned {
                span: t.span,
                data: Expr::Value(*v),
            }),
        Token::Identifier(s) =>
            Some(Spanned {
                span: t.span,
                data: Expr::Identifier(s.clone()),
            }),

        Token::Macro(s) =>
            Some(Spanned {
                span: t.span,
                data: Expr::Macro(s.clone()),
            }),

        Token::Operator(Operator::Binary(BinaryOp::Subtraction)) => {
            let rhs = parse_expression(cursor, 70)?;
            Some(Spanned {
                span: t.span,
                data: Expr::Unary {
                    op: Operator::Unary(UnaryOp::Negation),
                    rhs: Box::new(rhs),
                },
            })
        }

        Token::Operator(Operator::Unary(UnaryOp::BitwiseNot)) => {
            let rhs = parse_expression(cursor, 70)?;
            Some(Spanned {
                span: t.span,
                data: Expr::Unary {
                    op: Operator::Unary(UnaryOp::BitwiseNot),
                    rhs: Box::new(rhs),
                },
            })
        }

        Token::Operator(Operator::Grouping(GroupingOp::LeftParen)) => {
            let e: Spanned<Expr> = parse_expression(cursor, 0)?;

            if cursor.expect(&Token::Operator(Operator::Grouping(GroupingOp::RightParen))) {
                Some(e)
            } else {
                None
            }
        }

        _ => None,
    }
}

fn led(cursor: &mut Cursor, left: &Spanned<Expr>, t: &Spanned<Token>) -> Option<Spanned<Expr>> {
    match t.data {
        Token::Operator(Operator::Binary(op)) => {
            let lbp = Token::Operator(Operator::Binary(op)).lbp();
            let rbp: u32 = match op {
                BinaryOp::Exponentiation => lbp - 1,
                | BinaryOp::Assign
                | BinaryOp::AddAssign
                | BinaryOp::SubAssign
                | BinaryOp::MulAssign
                | BinaryOp::DivAssign
                | BinaryOp::ModAssign
                | BinaryOp::AndAssign
                | BinaryOp::OrAssign
                | BinaryOp::BitAndAssign
                | BinaryOp::BitOrAssign
                | BinaryOp::BitXorAssign => lbp - 1,
                _ => lbp,
            };

            let right = parse_expression(cursor, rbp)?;

            Some(Spanned {
                span: Span { start: left.span.start, end: right.span.end },
                data: Expr::Binary {
                    op: Operator::Binary(op),
                    lhs: Box::new(left.clone()),
                    rhs: Box::new(right),
                },
            })
        }

        Token::Operator(Operator::Grouping(GroupingOp::LeftParen)) => {
            const COMMA_BP: u32 = 1;
            let mut args = Vec::new();

            if
                cursor
                    .peek()
                    .is_some_and(|s| {
                        s.data != Token::Operator(Operator::Grouping(GroupingOp::RightParen))
                    })
            {
                loop {
                    args.push(parse_expression(cursor, COMMA_BP + 1)?);

                    if
                        cursor
                            .peek()
                            .is_some_and(|s| {
                                s.data == Token::Operator(Operator::Grouping(GroupingOp::Comma))
                            })
                    {
                        cursor.advance(1);
                        continue;
                    }
                    break;
                }
            }

            if
                cursor
                    .peek()
                    .is_some_and(|s| {
                        s.data == Token::Operator(Operator::Grouping(GroupingOp::RightParen))
                    })
            {
                let rparen = cursor.peek().unwrap(); // before advance
                let end = rparen.span.end;
                cursor.advance(1);

                Some(Spanned {
                    span: Span { start: left.span.start, end },
                    data: Expr::Call {
                        func: Box::new(left.clone()),
                        args,
                    },
                })
            } else {
                None
            }
        }

        Token::Operator(Operator::Grouping(GroupingOp::Comma)) => {
            const COMMA_BP: u32 = 1;

            let mut exprs = match &left.data {
                Expr::Comma { exprs } => { exprs.clone() }
                _ => vec![left.clone()],
            };

            exprs.push(parse_expression(cursor, COMMA_BP)?);

            while
                cursor
                    .peek()
                    .is_some_and(|t| {
                        t.data == Token::Operator(Operator::Grouping(GroupingOp::Comma))
                    })
            {
                cursor.advance(1);
                exprs.push(parse_expression(cursor, COMMA_BP)?);
            }

            let end = exprs.last().unwrap().span.end;

            Some(Spanned {
                span: Span { start: exprs[0].span.start, end },
                data: Expr::Comma { exprs },
            })
        }

        Token::Operator(Operator::TernaryOp(TernaryOp::TernaryCond)) => {
            let lbp = Token::Operator(Operator::TernaryOp(TernaryOp::TernaryCond)).lbp();
            let rbp: u32 = lbp - 1;

            let statement1 = parse_expression(cursor, 0)?;
            if !cursor.expect(&Token::Operator(Operator::TernaryOp(TernaryOp::TernaryElse))) {
                return None;
            }
            let statement2 = parse_expression(cursor, rbp)?;

            Some(Spanned {
                span: Span {
                    start: left.span.start,
                    end: statement2.span.end,
                },
                data: Expr::Ternary {
                    cond: Box::new(left.clone()),
                    statement1: Box::new(statement1.clone()),
                    statement2: Box::new(statement2.clone()),
                },
            })
        }

        _ => None,
    }
}

fn starts_expression(t: &Spanned<Token>) -> bool {
    match t.data {
        | Token::Value(_)
        | Token::Identifier(_)
        | Token::Macro(_)
        | Token::Operator(Operator::Unary(UnaryOp::Not))
        | Token::Operator(Operator::Grouping(GroupingOp::LeftParen))
        | Token::Operator(Operator::Unary(UnaryOp::Negation))
        | Token::Operator(Operator::Unary(UnaryOp::BitwiseNot)) => true,
        _ => false,
    }
}

fn parse_expression(cursor: &mut Cursor, min_bp: u32) -> Option<Spanned<Expr>> {
    const IMPLICIT_MUL_LBP: u32 = 65;

    let t = cursor.next()?;

    let mut left = nud(cursor, &t)?;

    while let Some(t) = cursor.peek() {
        if
            starts_expression(&t) &&
            !matches!(left.data, Expr::Identifier(_)) &&
            IMPLICIT_MUL_LBP > min_bp
        {
            let rhs = parse_expression(cursor, IMPLICIT_MUL_LBP)?;
            left = Spanned {
                span: Span {
                    start: left.span.start,
                    end: rhs.span.end,
                },
                data: Expr::Binary {
                    op: Operator::Binary(BinaryOp::Multiplication),
                    lhs: Box::from(left),
                    rhs: Box::from(rhs),
                },
            };
            continue;
        }

        if t.data.lbp() <= min_bp {
            break;
        }
        cursor.advance(1);
        left = led(cursor, &left, &t)?;
    }

    Some(left)
}

pub fn recompute_expr_span(expr: &mut Spanned<Expr>) {
    let new_span = match &mut expr.data {
        Expr::Value(_) | Expr::Identifier(_) | Expr::Macro(_) => expr.span,

        Expr::Unary { rhs, .. } => {
            recompute_expr_span(rhs);
            Span::merge(&expr.span, &rhs.span)
        }

        Expr::Binary { lhs, rhs, .. } => {
            recompute_expr_span(lhs);
            recompute_expr_span(rhs);
            Span::merge(&lhs.span, &rhs.span)
        }

        Expr::Ternary { cond, statement1, statement2 } => {
            recompute_expr_span(cond);
            recompute_expr_span(statement1);
            recompute_expr_span(statement2);

            Span::merge(&Span::merge(&cond.span, &statement1.span), &statement2.span)
        }
        Expr::Comma { exprs } => {
            let mut span = expr.span;

            for e in exprs {
                recompute_expr_span(e);
                span = Span::merge(&span, &e.span);
            }

            span
        }

        Expr::Call { func, args } => {
            recompute_expr_span(func);
            let mut span = func.span;

            for arg in args {
                recompute_expr_span(arg);
                span = Span::merge(&span, &arg.span);
            }

            span
        }
    };

    expr.span = new_span;
}

pub fn parse_string(s: &str, debug: bool) -> Result<Box<Spanned<Expr>>, Spanned<Error>> {
    let tokens = lex_string(s)?;
    let mut cursor = Cursor::new(&tokens);

    if debug {
        print_debug_tokens(&tokens);
    }

    let expr = parse_expression(&mut cursor, 0);

    match expr {
        Some(mut e) => {
            recompute_expr_span(&mut e);
            if debug {
                print_debug_expr(e.clone(), 0);
            }
            Ok(Box::from(e))
        }
        None =>
            Err(Spanned {
                span: Span { start: 0, end: 0 },
                data: Error::UnexpectedError,
            }),
    }
}
