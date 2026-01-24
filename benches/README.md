# Boxen Benchmarks

This directory contains performance and allocation benchmarks for the boxen library.

## Available Benchmarks

### 1. allocation_benchmarks.rs (Custom Harness)

Comprehensive benchmark suite that measures both performance and memory allocation patterns using a custom global allocator.

**Features**:

- Custom global allocator for precise allocation tracking
- 10 different benchmark scenarios covering various use cases
- Detailed statistics including:
  - Execution time (mean, min, max)
  - Throughput (operations per second)
  - Memory allocations (count and bytes)
  - Allocations per operation
  - Bytes allocated per operation

**Run with**:

```bash
cargo bench --bench allocation_benchmarks
```

**Documentation**: See this README for details.

### 2. criterion_benchmarks.rs (Criterion Framework)

Sophisticated statistical benchmarks using the Criterion framework for regression detection and performance analysis.

**Features**:

- Statistical analysis with confidence intervals
- Automatic regression detection
- HTML reports with plots and graphs
- Comparison across runs
- 5 benchmark groups covering all aspects of the library

**Run with**:

```bash
cargo bench --bench criterion_benchmarks
```

**Documentation**: See [CRITERION_README.md](./CRITERION_README.md) for detailed usage.

**CI/CD Integration**: See [CI_INTEGRATION.md](./CI_INTEGRATION.md) for automated testing.

## Quick Start

### Run All Benchmarks

```bash
# Run custom allocation benchmarks
cargo bench --bench allocation_benchmarks

# Run criterion benchmarks
cargo bench --bench criterion_benchmarks
```

### Run Specific Benchmark Groups

```bash
# Run only basic criterion benchmarks
cargo bench --bench criterion_benchmarks -- basic_benches

# Run only unicode benchmarks
cargo bench --bench criterion_benchmarks -- unicode
```

### Quick Test (CI Mode)

```bash
# Fast test mode for CI/CD
cargo bench --bench criterion_benchmarks -- --test
```

## Benchmark Comparison

| Feature                  | Allocation Benchmarks | Criterion Benchmarks   |
| ------------------------ | --------------------- | ---------------------- |
| **Framework**            | Custom harness        | Criterion.rs           |
| **Allocation Tracking**  | ✅ Detailed           | ❌ Not included        |
| **Statistical Analysis** | ❌ Basic              | ✅ Advanced            |
| **Regression Detection** | ❌ Manual             | ✅ Automatic           |
| **HTML Reports**         | ❌ No                 | ✅ Yes                 |
| **CI/CD Integration**    | ⚠️ Manual             | ✅ Built-in            |
| **Execution Time**       | ~30 seconds           | ~10-20 minutes         |
| **Best For**             | Memory optimization   | Performance regression |

## Allocation Benchmarks Details

### Benchmark Scenarios

1. Simple Box (80x24) - Basic rendering
2. Box with Options - Custom configuration
3. Box with Title - Title rendering
4. Multi-line Content - 5 lines of text
5. Large Content - 1000 characters
6. Unicode Content - Emojis and international characters
7. Complex Configuration - All features combined
8. Batch Rendering - 100 boxes in sequence
9. Border Style Variations - Different border styles
10. Text Alignment Variations - Left, center, right alignment

## Understanding Results

### Performance Metrics

```
Mean time:      1.598µs
Throughput:     625,782 ops/sec
```

- **Mean time**: Average time per operation
- **Throughput**: Operations per second (higher is better)

### Memory Metrics

```
Total allocated:      603000 bytes
Total deallocated:    587000 bytes
Net allocated:        16000 bytes
Allocation count:     19001
Allocations per op:   19.00
Bytes per op:         603.00
```

- **Total allocated**: Total bytes allocated during benchmark
- **Total deallocated**: Total bytes freed during benchmark
- **Net allocated**: Memory still allocated (potential leaks if high)
- **Allocation count**: Total number of allocations
- **Allocations per op**: Average allocations per operation (key metric)
- **Bytes per op**: Average bytes allocated per operation

## Optimization Targets

Based on the requirements specification:

| Metric      | Baseline | Target | Improvement   |
| ----------- | -------- | ------ | ------------- |
| Simple box  | 1.6µs    | 0.5µs  | 3x faster     |
| Unicode box | 2.3µs    | 0.8µs  | 3x faster     |
| Large box   | 108.8µs  | 27.2µs | 4x faster     |
| Batch (100) | 149.9µs  | 40µs   | 3.75x faster  |
| Allocations | 19-62    | 3-12   | 80% reduction |

## Comparing Results

### Before Optimization

1. Run benchmarks and save results:
   ```bash
   cargo bench --bench allocation_benchmarks > before.txt
   ```

### After Optimization

2. Apply optimizations (string pooling, caching, etc.)
3. Run benchmarks again:
   ```bash
   cargo bench --bench allocation_benchmarks > after.txt
   ```

### Calculate Improvements

4. Compare the results:

   ```bash
   # On Unix/Linux/macOS
   diff before.txt after.txt

   # Or manually compare key metrics:
   # - Mean time (should be 3-5x lower)
   # - Allocations per op (should be 80% lower)
   ```

## Baseline Results

See [BASELINE_RESULTS.md](./BASELINE_RESULTS.md) for the pre-optimization baseline metrics.

## Implementation Details

### Custom Allocator

The benchmark uses a custom global allocator that wraps the system allocator:

```rust
struct TrackingAllocator;

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Track allocation size and count
        let ret = unsafe { System.alloc(layout) };
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
            ALLOCATION_COUNT.fetch_add(1, Ordering::SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Track deallocation size
        unsafe { System.dealloc(ptr, layout) };
        DEALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
    }
}
```

This allows precise tracking of all memory allocations during benchmark execution.

### Benchmark Methodology

1. **Warmup**: 100 iterations to warm up caches
2. **Reset**: Clear allocation counters
3. **Measure**: 1000 iterations with timing and allocation tracking
4. **Report**: Calculate statistics and display results

## Adding New Benchmarks

To add a new benchmark scenario:

1. Add a new benchmark call in `main()`:

   ```rust
   let result = benchmark("My New Benchmark", || {
       // Your code to benchmark
       let _ = boxen("test", None);
   });
   result.print();
   ```

2. Update the summary section with expected improvements

3. Document the new benchmark in this README

## Troubleshooting

### High Net Allocated Memory

If you see high "Net allocated" values (> 1MB), this may indicate:

- Memory leaks
- Buffers not being released
- Pool sizes growing unbounded

Investigate with:

```bash
# Run with memory profiler
cargo install dhat
cargo bench --bench allocation_benchmarks --features dhat
```

### Inconsistent Results

If results vary significantly between runs:

- Close other applications
- Run multiple times and average
- Increase ITERATIONS constant
- Check system load

### Compilation Errors

If the benchmark fails to compile:

- Ensure Rust 1.85.0 or later
- Check that all dependencies are up to date
- Verify the global allocator is properly configured

## Related Documentation

- [Performance Optimization Requirements](../.kiro/specs/performance-optimization/requirements.md)
- [Performance Optimization Design](../.kiro/specs/performance-optimization/design.md)
- [Performance Optimization Tasks](../.kiro/specs/performance-optimization/tasks.md)

## License

Same as the main boxen library (MIT OR Apache-2.0).
