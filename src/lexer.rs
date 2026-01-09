use crate::{
    error::{Error, LexingError},
    operator::{BinaryOp, GroupingOp, Operator, TernaryOp},
    span::{Span, Spanned},
    token::Token,
    value::Value,
};

struct Cursor<'a> {
    src: &'a str,
    i: usize,
}

impl<'a> Cursor<'a> {
    fn new(src: &'a str) -> Self {
        Cursor { src, i: 0 }
    }

    fn is_eof(&self) -> bool {
        self.i >= self.src.len()
    }

    fn peek(&self) -> Option<u8> {
        self.src.as_bytes().get(self.i).copied()
    }

    fn rest(&self) -> &'a str {
        &self.src[self.i..]
    }

    fn advance(&mut self, n: usize) -> bool {
        self.i += n;
        self.is_eof()
    }

    fn skip_while(&mut self, pred: impl Fn(u8) -> bool) {
        while let Some(c) = self.peek() {
            if pred(c) {
                self.advance(1);
            } else {
                break;
            }
        }
    }
}

fn lex_ws(cursor: &mut Cursor) -> bool {
    let start = cursor.i;
    cursor.skip_while(|c| c.is_ascii_whitespace());
    cursor.i != start
}

fn lex_operator(cursor: &mut Cursor) -> Option<Spanned<Token>> {
    let s = cursor.rest();

    if s.starts_with("&&=") {
        cursor.advance(3);
        return Some(Spanned {
            span: Span::from(cursor.i - 3, 3),
            data: Token::Operator(Operator::Binary(BinaryOp::AndAssign)),
        });
    }
    if s.starts_with("||=") {
        cursor.advance(3);
        return Some(Spanned {
            span: Span::from(cursor.i - 3, 3),
            data: Token::Operator(Operator::Binary(BinaryOp::OrAssign)),
        });
    }
    if s.starts_with("**") {
        cursor.advance(2);
        return Some(Spanned {
            span: Span::from(cursor.i - 2, 2),
            data: Token::Operator(Operator::Binary(BinaryOp::Exponentiation)),
        });
    }
    if s.starts_with("==") {
        cursor.advance(2);
        return Some(Spanned {
            span: Span::from(cursor.i - 2, 2),
            data: Token::Operator(Operator::Binary(BinaryOp::Equal)),
        });
    }
    if s.starts_with("<=") {
        cursor.advance(2);
        return Some(Spanned {
            span: Span::from(cursor.i - 2, 2),
            data: Token::Operator(Operator::Binary(BinaryOp::LessEqual)),
        });
    }
    if s.starts_with(">=") {
        cursor.advance(2);
        return Some(Spanned {
            span: Span::from(cursor.i - 2, 2),
            data: Token::Operator(Operator::Binary(BinaryOp::GreaterEqual)),
        });
    }
    if s.starts_with("+=") {
        cursor.advance(2);
        return Some(Spanned {
            span: Span::from(cursor.i - 2, 2),
            data: Token::Operator(Operator::Binary(BinaryOp::AddAssign)),
        });
    }
    if s.starts_with("-=") {
        cursor.advance(2);
        return Some(Spanned {
            span: Span::from(cursor.i - 2, 2),
            data: Token::Operator(Operator::Binary(BinaryOp::SubAssign)),
        });
    }
    if s.starts_with("*=") {
        cursor.advance(2);
        return Some(Spanned {
            span: Span::from(cursor.i - 2, 2),
            data: Token::Operator(Operator::Binary(BinaryOp::MulAssign)),
        });
    }
    if s.starts_with("/=") {
        cursor.advance(2);
        return Some(Spanned {
            span: Span::from(cursor.i - 2, 2),
            data: Token::Operator(Operator::Binary(BinaryOp::DivAssign)),
        });
    }
    if s.starts_with("%=") {
        cursor.advance(2);
        return Some(Spanned {
            span: Span::from(cursor.i - 2, 2),
            data: Token::Operator(Operator::Binary(BinaryOp::ModAssign)),
        });
    }
    if s.starts_with("&=") {
        cursor.advance(2);
        return Some(Spanned {
            span: Span::from(cursor.i - 2, 2),
            data: Token::Operator(Operator::Binary(BinaryOp::BitAndAssign)),
        });
    }
    if s.starts_with("|=") {
        cursor.advance(2);
        return Some(Spanned {
            span: Span::from(cursor.i - 2, 2),
            data: Token::Operator(Operator::Binary(BinaryOp::BitOrAssign)),
        });
    }
    if s.starts_with("^=") {
        cursor.advance(2);
        return Some(Spanned {
            span: Span::from(cursor.i - 2, 2),
            data: Token::Operator(Operator::Binary(BinaryOp::BitXorAssign)),
        });
    }

    match cursor.peek()? {
        b'+' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::Binary(BinaryOp::Addition)),
            })
        }
        b'-' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::Binary(BinaryOp::Subtraction)),
            })
        }
        b'%' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::Binary(BinaryOp::Modulo)),
            })
        }
        b'=' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::Binary(BinaryOp::Assign)),
            })
        }
        b'*' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::Binary(BinaryOp::Multiplication)),
            })
        }
        b'/' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::Binary(BinaryOp::Division)),
            })
        }
        b'<' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::Binary(BinaryOp::Less)),
            })
        }
        b'>' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::Binary(BinaryOp::Greater)),
            })
        }
        b'&' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::Binary(BinaryOp::BitwiseAnd)),
            })
        }
        b'|' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::Binary(BinaryOp::BitwiseOr)),
            })
        }
        b'^' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::Binary(BinaryOp::BitwiseXor)),
            })
        }
        b'(' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::Grouping(GroupingOp::LeftParen)),
            })
        }
        b')' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::Grouping(GroupingOp::RightParen)),
            })
        }
        b',' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::Grouping(GroupingOp::Comma)),
            })
        }
        b'?' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::TernaryOp(TernaryOp::TernaryCond)),
            })
        }
        b':' => {
            cursor.advance(1);
            Some(Spanned {
                span: Span::single(cursor.i - 1),
                data: Token::Operator(Operator::TernaryOp(TernaryOp::TernaryElse)),
            })
        }
        _ => None,
    }
}

fn lex_value(cursor: &mut Cursor) -> Option<Spanned<Token>> {
    let sr = cursor.rest();

    if sr.starts_with("true") {
        cursor.advance(4);
        return Some(Spanned {
            span: Span::from(cursor.i - 4, 4),
            data: Token::Value(Value::Boolean(true)),
        });
    }
    if sr.starts_with("false") {
        cursor.advance(5);
        return Some(Spanned {
            span: Span::from(cursor.i - 5, 5),
            data: Token::Value(Value::Boolean(false)),
        });
    }

    let s = sr.as_bytes();
    let mut i = 0usize;

    let mut num_str = String::new();

    let mut seen_digit = false;
    let mut seen_fract = false;
    let mut seen_fpoint = false;

    if s.get(0) == Some(&b'-') {
        num_str.push('-');
        i += 1;
    }

    while s.get(i).is_some_and(|c| c.is_ascii_digit() || *c == b'_') {
        if s[i] != b'_' {
            num_str.push(s[i] as char);
        }
        seen_digit = true;
        i += 1;
    }

    if s.get(i) == Some(&b'.') {
        num_str.push('.');
        seen_fpoint = true;
        i += 1;

        while let Some(&c) = s.get(i)
            && (c.is_ascii_digit() || c == b'_')
        {
            if s[i] != b'_' {
                num_str.push(s[i] as char);
            }
            seen_fract = true;
            i += 1;
        }
    }

    if !(seen_digit || seen_fract) {
        return None;
    }

    cursor.advance(i);

    if seen_fpoint {
        let v = num_str.parse::<f64>().ok()?;
        Some(Spanned {
            span: Span::from(cursor.i - i, i),
            data: Token::Value(Value::Float(v)),
        })
    } else {
        let v = num_str.parse::<i64>().ok()?;
        Some(Spanned {
            span: Span::from(cursor.i - i, i),
            data: Token::Value(Value::Int(v)),
        })
    }
}

fn lex_identifier(cursor: &mut Cursor) -> Option<Spanned<Token>> {
    let sr = cursor.rest();
    let s = sr.as_bytes();
    let mut i = 0usize;

    if let Some(&c) = s.get(0)
        && (c.is_ascii_alphabetic() || c == b'_')
    {
        i += 1;

        while let Some(&c) = s.get(i)
            && (c.is_ascii_alphanumeric() || c == b'_')
        {
            i += 1;
        }
    } else {
        return None;
    }

    cursor.advance(i);

    Some(Spanned {
        span: Span::from(cursor.i - i, i),
        data: Token::Identifier(sr[0..i].to_string()),
    })
}

pub fn lex_string(s: &str) -> Result<Vec<Spanned<Token>>, Spanned<Error>> {
    let mut res = Vec::new();
    let mut cursor = Cursor::new(s);

    cursor.skip_while(|c| c.is_ascii_whitespace());

    while !cursor.is_eof() {
        if let Some(t) = lex_value(&mut cursor) {
            res.push(t);
        } else if let Some(t) = lex_operator(&mut cursor) {
            res.push(t);
        } else if let Some(t) = lex_identifier(&mut cursor) {
            res.push(t);
        } else {
            return Err(Spanned {
                span: Span::single(cursor.i),
                data: Error::LexingError(LexingError::InvalidToken {
                    src: s.to_string(),
                    index: cursor.i,
                }),
            });
        }

        cursor.skip_while(|c| c.is_ascii_whitespace());
    }

    Ok(res)
}
