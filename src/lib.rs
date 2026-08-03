pub mod error;
pub mod roman;

pub use error::RomanError;
pub use roman::{from_roman, from_roman_with_special_case, to_roman, ROMAN_NUMERAL_MAP};

/// Alias for Python casing parity `toRoman`
#[allow(non_snake_case)]
pub fn toRoman(n: i32) -> Result<String, RomanError> {
    to_roman(n)
}

/// Alias for Python casing parity `fromRoman`
#[allow(non_snake_case)]
pub fn fromRoman(s: &str) -> Result<i32, RomanError> {
    from_roman(s)
}
