//! Integration test to verify PooledString RAII behavior

use boxen::memory::pool::with_pooled_string;
use std::fmt::Write;

#[test]
fn test_raii_automatic_return_to_pool() {
    // First use - buffer should be acquired from pool (or newly allocated)
    let result1 = with_pooled_string(|buffer| {
        write!(buffer, "first use").unwrap();
        buffer.as_str().to_string()
    });
    // PooledString is dropped here, buffer should be returned to pool

    // Second use - buffer should be reused from pool and cleared
    let result2 = with_pooled_string(|buffer| {
        // Buffer should be empty (cleared when returned to pool)
        assert_eq!(
            buffer.as_str(),
            "",
            "Buffer should be cleared when reused from pool"
        );
        write!(buffer, "second use").unwrap();
        buffer.as_str().to_string()
    });
    // PooledString is dropped here again

    assert_eq!(result1, "first use");
    assert_eq!(result2, "second use");
}

#[test]
fn test_write_trait_implementation() {
    with_pooled_string(|buffer| {
        // Test that fmt::Write trait works correctly
        write!(buffer, "Hello").unwrap();
        write!(buffer, " ").unwrap();
        write!(buffer, "World").unwrap();
        writeln!(buffer, "!").unwrap();

        assert!(buffer.as_str().contains("Hello World!"));
    });
}

#[test]
fn test_as_str_method() {
    with_pooled_string(|buffer| {
        write!(buffer, "test content").unwrap();

        // as_str() should return a string slice
        let s: &str = buffer.as_str();
        assert_eq!(s, "test content");
        assert_eq!(s.len(), 12);
    });
}

#[test]
fn test_multiple_sequential_uses() {
    // Test that multiple sequential uses work correctly
    for i in 0..10 {
        with_pooled_string(|buffer| {
            write!(buffer, "iteration {}", i).unwrap();
            assert!(buffer.as_str().starts_with("iteration"));
        });
    }
}

#[test]
fn test_buffer_reuse_and_clearing() {
    // Test that buffers are properly reused and cleared between calls
    // This verifies the RAII pattern is working correctly

    // First call - write some content
    let result1 = with_pooled_string(|buffer| {
        write!(buffer, "first call with longer content").unwrap();
        buffer.as_str().len()
    });

    // Second call - buffer should be cleared and reused
    let result2 = with_pooled_string(|buffer| {
        // Buffer should be empty at start
        assert_eq!(buffer.as_str(), "", "Buffer should be cleared between uses");
        write!(buffer, "second").unwrap();
        buffer.as_str().len()
    });

    assert_eq!(result1, 30);
    assert_eq!(result2, 6);

    // Third call - verify it still works
    with_pooled_string(|buffer| {
        assert_eq!(buffer.as_str(), "", "Buffer should still be cleared");
        write!(buffer, "third").unwrap();
        assert_eq!(buffer.as_str(), "third");
    });
}
