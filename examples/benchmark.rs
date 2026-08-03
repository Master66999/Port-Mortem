use roman::{from_roman, to_roman};
use std::time::Instant;

fn main() {
    println!("=== Running roman-rs Benchmark Suite ===");
    const ITERATIONS: usize = 100_000;

    // 1. Benchmark to_roman
    let start = Instant::now();
    for i in 0..ITERATIONS {
        let num = (i % 4999) as i32 + 1;
        let _ = to_roman(num).unwrap();
    }
    let duration_to_roman = start.elapsed();
    println!(
        "to_roman ({} ops): {:?}",
        ITERATIONS, duration_to_roman
    );
    let ops_per_sec_to_roman = (ITERATIONS as f64) / duration_to_roman.as_secs_f64();
    println!("to_roman throughput: {:.2} ops/sec", ops_per_sec_to_roman);

    // Pre-generate roman numeral strings
    let roman_strings: Vec<String> = (1..5000).map(|n| to_roman(n).unwrap()).collect();

    // 2. Benchmark from_roman
    let start = Instant::now();
    for i in 0..ITERATIONS {
        let idx = i % roman_strings.len();
        let _ = from_roman(&roman_strings[idx]).unwrap();
    }
    let duration_from_roman = start.elapsed();
    println!(
        "from_roman ({} ops): {:?}",
        ITERATIONS, duration_from_roman
    );
    let ops_per_sec_from_roman = (ITERATIONS as f64) / duration_from_roman.as_secs_f64();
    println!("from_roman throughput: {:.2} ops/sec", ops_per_sec_from_roman);

    // 3. Combined Roundtrip Benchmark
    let start = Instant::now();
    for i in 0..ITERATIONS {
        let num = (i % 4999) as i32 + 1;
        let s = to_roman(num).unwrap();
        let _ = from_roman(&s).unwrap();
    }
    let duration_roundtrip = start.elapsed();
    println!(
        "Roundtrip ({} ops): {:?}",
        ITERATIONS, duration_roundtrip
    );
    let ops_per_sec_roundtrip = (ITERATIONS as f64) / duration_roundtrip.as_secs_f64();
    println!("Roundtrip throughput: {:.2} ops/sec\n", ops_per_sec_roundtrip);
}
