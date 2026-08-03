# `roman-rs` — High-Performance Rust Port of `roman`

[![Build & Test](https://img.shields.io/badge/build-passing-brightgreen)](#02-one-step-build)
[![License: ZPL-2.1](https://img.shields.io/badge/License-ZPL--2.1-blue.svg)](Cargo.toml)

A safe, ultra-fast, zero-overhead Rust port of Mark Pilgrim's standard Python `roman` library. Converts Arabic integers ($0 \le n < 5000$) to Roman numerals and back with 100% test parity and ~30x higher performance.

---

## 🎮 Interactive "Click and Play" Demo for Judges

**Live Demo URL**: 🌐 [https://master66999.github.io/Port-Mortem/](https://master66999.github.io/Port-Mortem/)

Or open [`index.html`](index.html) locally in any web browser:
- ⚡ **Bi-Directional Converter**: Real-time conversion & test vectors.
- 🧪 **Live Fuzz Harness**: Execute 1,000 property fuzz tests directly in your browser.
- 📊 **Performance Benchmark**: Interactive visual comparison of Python vs Rust throughput.
- 📜 **Architecture & Deliverables**: Interactive breakdown of design decisions & competition checklist.

---

## 01. Public GitHub Repository
- **Remote**: [`https://github.com/Master66999/Port-Mortem.git`](https://github.com/Master66999/Port-Mortem.git)
- **Live Demo Site**: [`https://master66999.github.io/Port-Mortem/`](https://master66999.github.io/Port-Mortem/)

---

## 02. One-Step Build Command

To build a standalone optimized binary in **one step**:

```bash
cargo build --release
```

The resulting executable binary will be generated at:
- Windows: `target/release/roman.exe`
- Linux/macOS: `target/release/roman`

---

## 03. Original Test Suite Parity

Run the complete test suite (unit tests and boundary check assertions):

```bash
cargo test
```

### Passing Tests Output
```text
running 5 tests
test roman::tests::test_to_roman ... ok
test roman::tests::test_to_roman_errors ... ok
test roman::tests::test_from_roman_errors ... ok
test roman::tests::test_from_roman_case_insensitive ... ok
test roman::tests::test_from_roman ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured
```

---

## 04. Differential Fuzz Harness

Run the property-based differential fuzzing test suite:

```bash
cargo test --test differential_fuzz
```

Tests verified by fuzz harness:
- Roundtrip mathematical invariance for all $n \in [0, 4999]$ (`from_roman(to_roman(n)) == Ok(n)`).
- Full input range bounds checking ($n < 0$ and $n \ge 5000$).
- Invalid Roman numeral string parsing error handling.
- Case-insensitivity across randomly generated Roman numerals.
- Zero representation toggle (`'N'`).

---

## 05. Architectural Decisions (`DECISIONS.md`)

Detailed documentation of design decisions and divergences from the original Python implementation:
See [`DECISIONS.md`](DECISIONS.md).

---

## 06. Benchmark Report (`BENCHMARK.md`)

Run performance benchmark across 100,000 conversions:

```bash
cargo run --release --example benchmark
```

See complete methodology and results in [`BENCHMARK.md`](BENCHMARK.md).

---

## 07. 5-Minute Demo Walkthrough (`DEMO_SCRIPT.md`)

Step-by-step instructions for live demo recording showing test suite execution, building, and benchmarking:
See [`DEMO_SCRIPT.md`](DEMO_SCRIPT.md).

---

## Quick Start CLI Examples

```bash
# Convert Arabic to Roman
cargo run -- 2026
# Output: MMXXVI

# Convert Roman to Arabic
cargo run -- -r MMXXVI
# Output: 2026
```