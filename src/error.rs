use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RomanError {
    OutOfRange(String),
    NotInteger(String),
    InvalidRomanNumeral(String),
}

impl fmt::Display for RomanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RomanError::OutOfRange(msg) => write!(f, "{}", msg),
            RomanError::NotInteger(msg) => write!(f, "{}", msg),
            RomanError::InvalidRomanNumeral(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for RomanError {}
