# Architectural Decisions & Divergence Report (DECISIONS.md)

## Overview
This document records all non-trivial architectural divergences, design decisions, and trade-offs between the original Python `roman` library (by Mark Pilgrim) and this high-performance Rust port (**`roman-rs`**).

---

## 1. Type System & Static Typing
- **Original (Python)**: Uses dynamic typing. Function arguments are validated at runtime using `isinstance()` checks and string regex parsing. Integers can be arbitrary precision (Python integers).
- **Rust Port**: Enforces strict compile-time types with `i32` for numeric inputs and `&str` / `String` for textual representation.
- **Rationale**: Rust's `i32` covers the valid Roman numeral range ($0 \le n < 5000$) efficiently without allocation overhead or dynamic type checking overhead.

---

## 2. Error Handling & Control Flow
- **Original (Python)**: Raises standard Python exceptions:
  - `OutOfRangeError` (derived from `ValueError`) for inputs outside the $0..4999$ range.
  - `InvalidRomanNumeralError` (derived from `ValueError`) for malformed strings.
  - `NotIntegerError` for floating point or non-integer numeric types.
- **Rust Port**: Uses explicit `Result<T, RomanError>` return types where `RomanError` is a custom enum:
  ```rust
  #[derive(Debug, PartialEq, Eq)]
  pub enum RomanError {
      OutOfRange(String),
      InvalidRomanNumeral(String),
      NotInteger(String),
  }
  ```
- **Rationale**: Idiomatic Rust avoids panics/exceptions for expected domain errors. Using `Result` provides explicit error handling, compile-time safety, zero-cost error propagation via `?`, and full memory safety.

---

## 3. Regular Expression Compilation & Thread Safety
- **Original (Python)**: Compiles the regex pattern `^M{0,4}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})$` on module import using Python's `re.compile()` with `re.VERBOSE`.
- **Rust Port**: Uses `std::sync::LazyLock<Regex>` to lazily compile the regular expression thread-safely on first use:
  ```rust
  static ROMAN_NUMERAL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
      Regex::new(r"^M{0,4}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})$").unwrap()
  });
  ```
- **Rationale**: Prevents regex initialization overhead during binary startup unless string parsing is actually invoked, while maintaining zero global state mutation and thread safety (`LazyLock` is standard in modern Rust).

---

## 4. Special Case Zero Handling ('N')
- **Original (Python)**: Version 3.x+ of the Python `roman` package introduced support for `0` represented by `'N'` (nulla / none).
- **Rust Port**: Implements `from_roman_with_special_case(s: &str, special_case: bool)` alongside `from_roman(s: &str)` and `to_roman(n: i32)`.
- **Rationale**: Preserves 100% parity with Python `roman` behavior while exposing granular control over whether `'N'` is accepted as zero.

---

## 5. CLI Interface & Argument Parsing
- **Original (Python)**: CLI interface handled via basic `sys.argv` or custom parsing scripts.
- **Rust Port**: Utilizes `clap` v4 with derive macros (`#[derive(Parser)]`) for standard UNIX CLI syntax support (`-r` / `--reverse` flag, auto-generated `--help` and `--version`).
- **Rationale**: Provides user experience parity with modern rust command-line tools while returning standard OS exit status codes (`ExitCode::SUCCESS` / `ExitCode::FAILURE`).
