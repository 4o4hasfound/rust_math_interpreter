#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub span: Span,
    pub data: T,
}

impl Span {
    pub fn single(pos: usize) -> Self {
        Span {
            start: pos,
            end: pos + 1,
        }
    }

    pub fn from(pos: usize, n: usize) -> Self {
        Span {
            start: pos,
            end: pos + n,
        }
    }

    pub fn merge(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}
