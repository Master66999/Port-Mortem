use roman::{from_roman, from_roman_with_special_case, to_roman, RomanError};

#[test]
fn fuzz_roundtrip_all_valid_range() {
    // Test full valid range [0..4999] for mathematical roundtrip invariance
    for n in 0..5000 {
        let roman_str = to_roman(n).expect(&format!("to_roman failed for {}", n));
        let converted_back = from_roman(&roman_str).expect(&format!("from_roman failed for {}", roman_str));
        assert_eq!(
            n, converted_back,
            "Roundtrip invariance broken for n={}: got {} -> {}",
            n, roman_str, converted_back
        );
    }
}

#[test]
fn fuzz_case_insensitivity_all_valid_range() {
    // Test case insensitivity for all generated valid Roman numerals
    for n in 0..5000 {
        let roman_upper = to_roman(n).unwrap();
        let roman_lower = roman_upper.to_lowercase();
        
        let back_upper = from_roman(&roman_upper).unwrap();
        let back_lower = from_roman(&roman_lower).unwrap();
        
        assert_eq!(back_upper, back_lower);
        assert_eq!(back_lower, n);
    }
}

#[test]
fn fuzz_out_of_range_inputs() {
    // Inputs below 0 must yield OutOfRange error
    for n in -100..0 {
        match to_roman(n) {
            Err(RomanError::OutOfRange(_)) => {}
            res => panic!("Expected OutOfRange for n={}, got {:?}", n, res),
        }
    }

    // Inputs >= 5000 must yield OutOfRange error
    for n in 5000..5100 {
        match to_roman(n) {
            Err(RomanError::OutOfRange(_)) => {}
            res => panic!("Expected OutOfRange for n={}, got {:?}", n, res),
        }
    }
}

#[test]
fn fuzz_invalid_roman_strings() {
    let invalid_strings = vec![
        "",
        "   ",
        "IIII",
        "VV",
        "XXXX",
        "CCCC",
        "MMMMM",
        "ABC",
        "123",
        "I V",
        "IXIX",
        "VX",
        "LC",
        "DM",
    ];

    for s in invalid_strings {
        match from_roman(s) {
            Err(RomanError::InvalidRomanNumeral(_)) => {}
            Ok(val) => panic!("Expected InvalidRomanNumeral error for '{}', but got Ok({})", s, val),
            Err(e) => panic!("Expected InvalidRomanNumeral error for '{}', got {:?}", s, e),
        }
    }
}

#[test]
fn fuzz_special_case_zero_toggle() {
    // 'N' with special_case=true should be 0
    assert_eq!(from_roman_with_special_case("N", true).unwrap(), 0);
    assert_eq!(from_roman_with_special_case("n", true).unwrap(), 0);

    // 'N' with special_case=false should fail as invalid Roman numeral
    assert!(matches!(
        from_roman_with_special_case("N", false),
        Err(RomanError::InvalidRomanNumeral(_))
    ));
}
