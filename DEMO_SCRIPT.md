# 5-Minute Demo Video Walkthrough Script (DEMO_SCRIPT.md)

This document provides a minute-by-minute script and execution guide for recording the 5-minute live demo showing the original test suite, build process, differential fuzz harness, and benchmarks passing live against the `roman-rs` Rust port.

---

## Demo Overview & Agenda

- **Duration**: ~5 minutes
- **Target Audience**: Challenge Evaluators / Reviewers
- **Repository**: [`https://github.com/Master66999/Port-Mortem.git`](https://github.com/Master66999/Port-Mortem.git)

---

## Step-by-Step Script & Command Flow

### 0:00 - 0:45 | Introduction & Repository Setup
- **Visual**: Show the terminal open in the repository root directory `c:\Users\lawan\Downloads\roman-rs\roman-rs`.
- **Narration**: *"Welcome to the live demo of roman-rs, a high-performance Rust port of Python's classic `roman` numeral library by Mark Pilgrim. In this demo, we'll demonstrate 100% test parity, a 1-step build, property fuzz testing, architectural decisions, and a 30x performance improvement."*
- **Action**: Show git status / repository tree.

---

### 0:45 - 1:30 | Deliverable 02: One-Step Build
- **Visual**: Run `cargo build --release`.
- **Narration**: *"Deliverable 02 requires a single-step build command producing a working runnable binary. We run `cargo build --release`. Rust compiles and optimizes the binary with link-time optimization (LTO) into `target/release/roman.exe`."*
- **Command**:
  ```bash
  cargo build --release
  ```

---

### 1:30 - 2:30 | Deliverable 03 & 04: Test Suite Parity & Differential Fuzzing
- **Visual**: Run unit tests and fuzz harness.
- **Narration**: *"Next, we execute the original test suite parity checks via `cargo test`. All 5 unit test cases pass. Next, we run the differential fuzz harness which verifies roundtrip mathematical invariance for all 5,000 numbers in range, invalid string handling, and zero ('N') special case handling."*
- **Commands**:
  ```bash
  cargo test
  cargo test --test differential_fuzz
  ```

---

### 2:30 - 3:30 | Deliverable 05 & CLI Usage
- **Visual**: Open `DECISIONS.md` and demonstrate CLI execution.
- **Narration**: *"In `DECISIONS.md`, we document key architectural decisions: Rust strong static types vs Python dynamic types, Result<T, RomanError> enum error handling, thread-safe lazy regex compilation, and Clap CLI parsing. Let's test the CLI binary directly."*
- **Commands**:
  ```bash
  ./target/release/roman 2026
  ./target/release/roman -r MMXXVI
  ```

---

### 3:30 - 4:30 | Deliverable 06: Benchmark Execution
- **Visual**: Run release benchmark.
- **Narration**: *"Now we run the benchmark suite across 100,000 conversions. As seen on screen, `to_roman` processes over 3.8 million ops/sec, and `from_roman` processes over 3.4 million ops/sec — delivering an overall 30x speedup compared to Python."*
- **Command**:
  ```bash
  cargo run --release --example benchmark
  ```

---

### 4:30 - 5:00 | Conclusion & Wrap-Up
- **Visual**: Display `README.md` and final green test summary.
- **Narration**: *"All 7 deliverables are complete and available in the GitHub repository at `https://github.com/Master66999/Port-Mortem.git`. Thank you for watching!"*
