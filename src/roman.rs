use crate::error::RomanError;
use regex::Regex;
use std::sync::LazyLock;

pub static ROMAN_NUMERAL_MAP: &[(&str, i32)] = &[
    ("M", 1000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

static ROMAN_NUMERAL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^M{0,4}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})$").unwrap()
});

/// Convert integer to Roman numeral
pub fn to_roman(mut n: i32) -> Result<String, RomanError> {
    if !(-1 < n && n < 5000) {
        return Err(RomanError::OutOfRange(
            "number out of range (must be 0..4999)".to_string(),
        ));
    }

    if n == 0 {
        return Ok("N".to_string());
    }

    let mut result = String::new();
    for &(numeral, integer) in ROMAN_NUMERAL_MAP {
        while n >= integer {
            result.push_str(numeral);
            n -= integer;
        }
    }
    Ok(result)
}

/// Convert Roman numeral to integer with optional special case for zero ('N')
pub fn from_roman_with_special_case(s: &str, special_case: bool) -> Result<i32, RomanError> {
    if s.is_empty() {
        return Err(RomanError::InvalidRomanNumeral(
            "Input cannot be blank".to_string(),
        ));
    }

    let upper_s = s.to_uppercase();

    if upper_s == "N" && special_case {
        return Ok(0);
    }

    if !ROMAN_NUMERAL_PATTERN.is_match(&upper_s) {
        return Err(RomanError::InvalidRomanNumeral(format!(
            "Invalid Roman numeral: {}",
            s
        )));
    }

    let mut result = 0;
    let mut index = 0;
    let bytes = upper_s.as_bytes();

    for &(numeral, integer) in ROMAN_NUMERAL_MAP {
        let num_bytes = numeral.as_bytes();
        let num_len = num_bytes.len();
        while index + num_len <= bytes.len() && &bytes[index..index + num_len] == num_bytes {
            result += integer;
            index += num_len;
        }
    }

    Ok(result)
}

/// Convert Roman numeral to integer (default special_case = true)
pub fn from_roman(s: &str) -> Result<i32, RomanError> {
    from_roman_with_special_case(s, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MAP: &[(i32, &str)] = &[
        (0, "N"),
        (1, "I"),
        (3, "III"),
        (4, "IV"),
        (9, "IX"),
        (14, "XIV"),
        (19, "XIX"),
        (24, "XXIV"),
        (40, "XL"),
        (49, "XLIX"),
        (90, "XC"),
        (99, "XCIX"),
        (400, "CD"),
        (490, "CDXC"),
        (499, "CDXCIX"),
        (900, "CM"),
        (990, "CMXC"),
        (998, "CMXCVIII"),
        (999, "CMXCIX"),
        (2013, "MMXIII"),
    ];

    #[test]
    fn test_to_roman() {
        for &(num_arabic, num_roman) in TEST_MAP {
            assert_eq!(
                to_roman(num_arabic).unwrap(),
                num_roman,
                "{} should be {}",
                num_arabic,
                num_roman
            );
        }
    }

    #[test]
    fn test_to_roman_errors() {
        assert!(matches!(
            to_roman(100000),
            Err(RomanError::OutOfRange(_))
        ));
        assert!(matches!(to_roman(-1), Err(RomanError::OutOfRange(_))));
        assert!(matches!(to_roman(5000), Err(RomanError::OutOfRange(_))));
    }

    #[test]
    fn test_from_roman() {
        for &(num_arabic, num_roman) in TEST_MAP {
            assert_eq!(
                from_roman(num_roman).unwrap(),
                num_arabic,
                "{} should be {}",
                num_roman,
                num_arabic
            );
        }
    }

    #[test]
    fn test_from_roman_errors() {
        assert!(matches!(
            from_roman(""),
            Err(RomanError::InvalidRomanNumeral(_))
        ));
        assert!(matches!(
            from_roman("Q12"),
            Err(RomanError::InvalidRomanNumeral(_))
        ));
        assert!(matches!(
            from_roman_with_special_case("n", false),
            Err(RomanError::InvalidRomanNumeral(_))
        ));
    }

    #[test]
    fn test_from_roman_case_insensitive() {
        for &(num_arabic, num_roman) in TEST_MAP {
            assert_eq!(
                from_roman(&num_roman.to_lowercase()).unwrap(),
                num_arabic,
                "{} (lowercase) should be {}",
                num_roman,
                num_arabic
            );
        }
    }
}
