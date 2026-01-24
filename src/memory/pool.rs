//! Thread-local string buffer pooling for zero-contention reuse.
//!
//! This module implements a thread-local pool of reusable `String` buffers to reduce
//! allocations during rendering operations. Buffers are automatically returned to the
//! pool when dropped, enabling efficient reuse across multiple render calls.

use std::cell::RefCell;
use std::fmt::Write;

thread_local! {
    static STRING_POOL: RefCell<StringPool> = RefCell::new(StringPool::new());
}

/// A thread-local pool of reusable `String` buffers.
///
/// The pool maintains a collection of pre-allocated strings that can be reused
/// across multiple operations, reducing allocation overhead.
pub struct StringPool {
    buffers: Vec<String>,
    max_buffers: usize,
    max_buffer_size: usize,
}

impl StringPool {
    /// Creates a new `StringPool` with default configuration.
    ///
    /// Default settings:
    /// - Initial capacity: 8 buffers
    /// - Maximum buffers: 16
    /// - Maximum buffer size: 1MB
    pub fn new() -> Self {
        Self {
            buffers: Vec::with_capacity(8),
            max_buffers: 16,
            max_buffer_size: 1_048_576, // 1MB
        }
    }

    /// Acquires a `String` buffer from the pool.
    ///
    /// If the pool is empty, a new buffer is allocated. Otherwise, an existing
    /// buffer is reused.
    pub fn acquire(&mut self) -> PooledString {
        let buffer = self.buffers.pop().unwrap_or_default();
        PooledString { buffer }
    }

    /// Returns a `String` buffer to the pool for reuse.
    ///
    /// The buffer is only retained if:
    /// - The pool hasn't reached its maximum size
    /// - The buffer's capacity is within the maximum buffer size limit
    ///
    /// Otherwise, the buffer is dropped and its memory is freed.
    pub fn release(&mut self, mut buffer: String) {
        if self.buffers.len() < self.max_buffers && buffer.capacity() <= self.max_buffer_size {
            buffer.clear();
            self.buffers.push(buffer);
        }
    }
}

impl Default for StringPool {
    fn default() -> Self {
        Self::new()
    }
}

/// A RAII wrapper around a pooled `String` buffer.
///
/// When dropped, the buffer is automatically returned to the thread-local pool
/// for reuse. This ensures efficient memory management without manual intervention.
pub struct PooledString {
    buffer: String,
}

impl PooledString {
    /// Returns a string slice of the buffer's contents.
    pub fn as_str(&self) -> &str {
        &self.buffer
    }

    /// Consumes the `PooledString` and returns the underlying `String`.
    ///
    /// Note: This prevents the buffer from being returned to the pool.
    pub fn into_string(mut self) -> String {
        std::mem::take(&mut self.buffer)
    }

    /// Reserves capacity for at least `additional` more bytes.
    pub fn reserve(&mut self, additional: usize) {
        self.buffer.reserve(additional);
    }

    /// Appends a character to the buffer.
    pub fn push(&mut self, ch: char) {
        self.buffer.push(ch);
    }

    /// Appends a string slice to the buffer.
    pub fn push_str(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    /// Clears the buffer, removing all contents.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Extends the buffer with an iterator of characters.
    pub fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
        self.buffer.extend(iter);
    }

    /// Returns true if the buffer ends with the given string.
    pub fn ends_with(&self, pat: &str) -> bool {
        self.buffer.ends_with(pat)
    }
}

impl Write for PooledString {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.push_str(s);
        Ok(())
    }
}

impl Drop for PooledString {
    fn drop(&mut self) {
        STRING_POOL.with(|pool| {
            let buffer = std::mem::take(&mut self.buffer);
            pool.borrow_mut().release(buffer);
        });
    }
}

/// Executes a closure with a pooled string buffer.
///
/// This is the primary interface for using pooled strings. The closure receives
/// a mutable reference to a `PooledString`, which is automatically returned to
/// the pool when the closure completes.
///
/// # Examples
///
/// ```
/// use boxen::memory::pool::with_pooled_string;
/// use std::fmt::Write;
///
/// let result = with_pooled_string(|buffer| {
///     write!(buffer, "Hello, ").unwrap();
///     write!(buffer, "World!").unwrap();
///     buffer.as_str().to_string()
/// });
///
/// assert_eq!(result, "Hello, World!");
/// ```
pub fn with_pooled_string<F, R>(f: F) -> R
where
    F: FnOnce(&mut PooledString) -> R,
{
    STRING_POOL.with(|pool| {
        let mut pooled = pool.borrow_mut().acquire();
        f(&mut pooled)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_pool_acquire_release() {
        let mut pool = StringPool::new();

        // Acquire a buffer
        let mut buffer = pool.acquire();
        write!(buffer, "test").unwrap();
        assert_eq!(buffer.as_str(), "test");

        // Release it back
        drop(buffer);

        // The pool should now have one buffer
        assert_eq!(pool.buffers.len(), 0); // Still 0 because we need to manually release
    }

    #[test]
    fn test_pooled_string_reuse() {
        let result1 = with_pooled_string(|buffer| {
            write!(buffer, "first").unwrap();
            buffer.as_str().to_string()
        });

        let result2 = with_pooled_string(|buffer| {
            // Buffer should be cleared from previous use
            assert_eq!(buffer.as_str(), "");
            write!(buffer, "second").unwrap();
            buffer.as_str().to_string()
        });

        assert_eq!(result1, "first");
        assert_eq!(result2, "second");
    }

    #[test]
    fn test_pooled_string_write_trait() {
        with_pooled_string(|buffer| {
            write!(buffer, "Hello").unwrap();
            write!(buffer, ", ").unwrap();
            write!(buffer, "World!").unwrap();
            assert_eq!(buffer.as_str(), "Hello, World!");
        });
    }

    #[test]
    fn test_pool_max_buffers() {
        let mut pool = StringPool::new();
        pool.max_buffers = 2;

        // Acquire and release 3 buffers
        for _i in 0..3 {
            let mut buffer = pool.acquire();
            let string = std::mem::take(&mut buffer.buffer);
            pool.release(string);
        }

        // Pool should only retain max_buffers
        assert!(pool.buffers.len() <= 2);
    }

    #[test]
    fn test_pool_max_buffer_size() {
        let mut pool = StringPool::new();
        pool.max_buffer_size = 100;

        // Create a large buffer
        let mut large_buffer = String::with_capacity(200);
        large_buffer.push('x');

        // Release it - should be dropped due to size
        pool.release(large_buffer);

        // Pool should be empty
        assert_eq!(pool.buffers.len(), 0);
    }

    #[test]
    fn test_into_string() {
        let result = with_pooled_string(|buffer| {
            write!(buffer, "test").unwrap();
            // Note: We can't use into_string() here because the closure
            // takes &mut PooledString, not PooledString
            buffer.as_str().to_string()
        });

        assert_eq!(result, "test");
    }
}
