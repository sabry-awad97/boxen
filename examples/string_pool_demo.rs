//! Demonstration of the StringPool memory optimization feature.
//!
//! This example shows how the StringPool reduces allocations by reusing
//! string buffers across multiple operations.

use boxen::memory::pool::with_pooled_string;
use std::fmt::Write;

fn main() {
    println!("=== StringPool Demonstration ===\n");

    // Example 1: Basic usage
    println!("Example 1: Basic string pooling");
    let result1 = with_pooled_string(|buffer| {
        write!(buffer, "Hello, ").unwrap();
        write!(buffer, "World!").unwrap();
        buffer.as_str().to_string()
    });
    println!("Result: {}\n", result1);

    // Example 2: Reusing buffers
    println!("Example 2: Buffer reuse across calls");
    for i in 1..=5 {
        let result = with_pooled_string(|buffer| {
            write!(buffer, "Iteration {}: ", i).unwrap();
            write!(buffer, "Buffer reused!").unwrap();
            buffer.as_str().to_string()
        });
        println!("{}", result);
    }
    println!();

    // Example 3: Building complex strings
    println!("Example 3: Building complex strings");
    let result3 = with_pooled_string(|buffer| {
        write!(buffer, "┌").unwrap();
        for _ in 0..20 {
            write!(buffer, "─").unwrap();
        }
        writeln!(buffer, "┐").unwrap();
        writeln!(buffer, "│ Pooled String Demo │").unwrap();
        write!(buffer, "└").unwrap();
        for _ in 0..20 {
            write!(buffer, "─").unwrap();
        }
        write!(buffer, "┘").unwrap();
        buffer.as_str().to_string()
    });
    println!("{}\n", result3);

    println!("=== Benefits ===");
    println!("✓ Reduced memory allocations");
    println!("✓ Thread-local storage (zero contention)");
    println!("✓ Automatic buffer return via RAII");
    println!("✓ Transparent to users");
}
