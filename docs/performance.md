# Performance Guide

This guide covers performance optimizations, caching features, and best practices for using boxen efficiently.

## Overview

Boxen includes several performance optimizations:

1. **String Pooling**: Thread-local buffer reuse to reduce allocations
2. **Unicode Width Caching**: LRU cache for expensive width calculations
3. **Terminal Size Caching**: TTL-based cache for terminal dimension queries
4. **Lazy Evaluation**: Deferred computation of expensive operations

## Performance Targets

| Metric             | Baseline | Optimized | Improvement   |
| ------------------ | -------- | --------- | ------------- |
| Simple box (80x24) | 45 μs    | 15 μs     | 3x faster     |
| Unicode box        | 120 μs   | 40 μs     | 3x faster     |
| Large box (200x50) | 850 μs   | 200 μs    | 4x faster     |
| 100 boxes          | 4.5 ms   | 1.2 ms    | 3.75x faster  |
| Allocations/box    | ~25      | ~3-5      | 80% reduction |

## String Pooling

String pooling reduces memory allocations by reusing thread-local buffers.

### How It Works

```rust
use boxen::memory::with_pooled_string;

// Automatically uses pooled buffer
with_pooled_string(|buffer| {
    buffer.push_str("Content");
    buffer.push_str(" more content");
    // Buffer is automatically returned to pool
});
```

### Benefits

- **80% reduction** in allocations
- **Faster rendering** due to reduced memory pressure
- **Thread-safe** with thread-local storage
- **Automatic cleanup** with RAII

### When to Use

String pooling is always enabled and requires no configuration. It's most beneficial for:

- Batch rendering (multiple boxes)
- Repeated rendering operations
- Performance-critical code paths

## Unicode Width Caching

Unicode width caching uses an LRU cache to avoid redundant width calculations.

### Enabling the Feature

```toml
[dependencies]
boxen = { version = "0.3", features = ["width-cache"] }
```

### How It Works

```rust
use boxen::text::cached_unicode_width;

// First call - cache miss, calculates width
let width1 = cached_unicode_width("你好世界");

// Second call - cache hit, returns cached value
let width2 = cached_unicode_width("你好世界");
```

### Configuration

```rust
use boxen::text::{set_cache_size, clear_cache, cache_stats};

// Configure cache size (default: 1024 entries)
set_cache_size(2048);

// Clear cache if needed
clear_cache();

// Monitor performance
let stats = cache_stats();
println!("Hit rate: {:.2}%", stats.hit_rate());
```

### Benefits

- **2-3x speedup** for Unicode content
- **>90% hit rate** for typical workloads
- **Thread-local** for lock-free access
- **Configurable** cache size

### When to Use

Enable width caching when:

- Working with Unicode/CJK text
- Rendering emoji-heavy content
- Processing international text
- Batch rendering similar content

## Terminal Size Caching

Terminal size caching reduces expensive system calls with a TTL-based cache.

### Enabling the Feature

```toml
[dependencies]
boxen = { version = "0.3", features = ["terminal-cache"] }
```

### How It Works

```rust
use boxen::terminal::cached_terminal_size;

// First call - queries terminal
let size1 = cached_terminal_size();

// Subsequent calls within TTL - returns cached value
let size2 = cached_terminal_size();
```

### Configuration

```rust
use boxen::terminal::{set_cache_ttl, clear_cache, invalidate_cache};

// Configure TTL (default: 100ms)
set_cache_ttl(200); // 200ms

// Manually invalidate cache
invalidate_cache();

// Clear cache and stats
clear_cache();
```

### SIGWINCH Support (Unix)

On Unix systems, the cache automatically invalidates on terminal resize:

```rust
use boxen::terminal::setup_sigwinch_handler;

// Setup handler at application startup
setup_sigwinch_handler();
```

### Benefits

- **10-20% speedup** for batch rendering
- **Reduced system calls** (expensive on some platforms)
- **Automatic invalidation** on resize (Unix)
- **Configurable TTL**

### When to Use

Enable terminal caching when:

- Rendering multiple boxes in succession
- Building TUI applications
- Performance is critical
- Terminal size rarely changes

## Combining Features

For maximum performance, enable all features:

```toml
[dependencies]
boxen = { version = "0.3", features = ["width-cache", "terminal-cache"] }
```

### Expected Performance

With all optimizations enabled:

- **3-5x faster** overall rendering
- **80% fewer** allocations
- **>90% cache hit rate** for typical workloads
- **Minimal overhead** from caching

## Benchmarking

### Running Benchmarks

```bash
# Run allocation benchmarks
cargo bench --bench allocation_benchmarks

# Run criterion benchmarks
cargo bench --bench criterion_benchmarks

# Run with specific features
cargo bench --features width-cache,terminal-cache
```

### Memory Profiling

```bash
# Profile with dhat
cargo run --example memory_profiling --features dhat-heap

# View results
dh_view.py dhat-heap.json
```

### Performance Testing

```bash
# Run performance demo
cargo run --example caching_demo --features width-cache,terminal-cache

# Compare with/without caching
cargo run --example caching_demo
```

## Best Practices

### 1. Enable Appropriate Features

Choose features based on your use case:

```toml
# For Unicode-heavy content
boxen = { version = "0.3", features = ["width-cache"] }

# For batch rendering
boxen = { version = "0.3", features = ["terminal-cache"] }

# For maximum performance
boxen = { version = "0.3", features = ["width-cache", "terminal-cache"] }
```

### 2. Reuse Box Configurations

```rust
use boxen::{BoxenOptions, BorderStyle, Spacing};

// Create reusable options
let options = BoxenOptions {
    border_style: BorderStyle::Double,
    padding: Spacing::from(2),
    ..Default::default()
};

// Reuse for multiple boxes
for content in &contents {
    let _ = boxen(content, Some(options.clone()));
}
```

### 3. Batch Rendering

```rust
// Efficient batch rendering
let boxes: Vec<_> = contents
    .iter()
    .map(|content| boxen(content, None))
    .collect();
```

### 4. Monitor Cache Performance

```rust
#[cfg(feature = "width-cache")]
{
    use boxen::text::cache_stats;
    let stats = cache_stats();
    if stats.hit_rate() < 80.0 {
        // Consider increasing cache size
        use boxen::text::set_cache_size;
        set_cache_size(2048);
    }
}
```

### 5. Profile Your Application

```rust
use std::time::Instant;

let start = Instant::now();
for _ in 0..1000 {
    let _ = boxen("Content", None);
}
println!("Time: {:?}", start.elapsed());
```

## Performance Troubleshooting

### Slow Unicode Rendering

**Problem**: Rendering Unicode content is slow

**Solution**: Enable width caching

```toml
boxen = { version = "0.3", features = ["width-cache"] }
```

### High Memory Usage

**Problem**: Memory usage increases over time

**Solution**: String pooling is automatic, but you can clear caches:

```rust
#[cfg(feature = "width-cache")]
boxen::text::clear_cache();

#[cfg(feature = "terminal-cache")]
boxen::terminal::clear_cache();
```

### Slow Batch Rendering

**Problem**: Rendering many boxes is slow

**Solution**: Enable terminal caching

```toml
boxen = { version = "0.3", features = ["terminal-cache"] }
```

### Cache Thrashing

**Problem**: Low cache hit rate

**Solution**: Increase cache size

```rust
#[cfg(feature = "width-cache")]
boxen::text::set_cache_size(4096);
```

## Platform-Specific Notes

### Windows

- Terminal size queries can be expensive
- Terminal caching provides significant benefits
- SIGWINCH not available (no automatic invalidation)

### macOS/Linux

- Terminal size queries are relatively fast
- SIGWINCH support for automatic cache invalidation
- Width caching provides most benefit

### CI/CD Environments

- Terminal detection may fail (no TTY)
- Caching has minimal overhead
- Fallback values work correctly

## Measuring Performance

### Simple Benchmark

```rust
use std::time::Instant;
use boxen::boxen;

let iterations = 1000;
let start = Instant::now();

for _ in 0..iterations {
    let _ = boxen("Test content", None);
}

let duration = start.elapsed();
println!("Total: {:?}", duration);
println!("Average: {:?}", duration / iterations);
```

### With Statistics

```rust
#[cfg(feature = "width-cache")]
{
    use boxen::text::{cache_stats, clear_cache};

    clear_cache();

    // Run benchmark
    for _ in 0..1000 {
        let _ = boxen("Test", None);
    }

    let stats = cache_stats();
    println!("Hit rate: {:.2}%", stats.hit_rate());
}
```

## Future Optimizations

Planned performance improvements:

1. **Lazy Evaluation**: Defer expensive calculations
2. **Border Caching**: Pre-calculate border strings
3. **Capacity Hints**: Pre-allocate buffers with correct size
4. **SIMD Optimizations**: Vectorized text processing
5. **Parallel Rendering**: Multi-threaded batch rendering

## Contributing

Help improve boxen's performance:

1. Run benchmarks and share results
2. Profile your use cases
3. Report performance issues
4. Submit optimization PRs
5. Suggest new optimizations

## Resources

- [Benchmarks](../benches/)
- [Examples](../examples/)
- [Memory Profiling](../examples/memory_profiling.rs)
- [Caching Demo](../examples/caching_demo.rs)
