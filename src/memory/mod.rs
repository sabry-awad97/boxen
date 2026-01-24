//! Memory management utilities for performance optimization.
//!
//! This module provides memory pooling and buffer reuse mechanisms to reduce
//! allocations during rendering operations.

pub mod pool;

pub use pool::{PooledString, StringPool, with_pooled_string};
