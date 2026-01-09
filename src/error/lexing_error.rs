
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexingError {
    InvalidToken { src: String, index: usize },
}

pub fn error_to_string(err: LexingError) -> String {
    match err {
        LexingError::InvalidToken { src: _, index } => {
            format!("Invalid token at position {}", index)
        }
    }
}
