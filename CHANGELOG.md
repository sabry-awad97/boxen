# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Performance Optimizations** (v0.3.0)
  - Thread-local string pooling for reduced memory allocations
  - Unicode width caching with LRU cache (feature: `width-cache`)
  - Terminal size caching with TTL (feature: `terminal-cache`)
  - Memory profiling support with dhat (feature: `dhat-heap`)
  - Comprehensive performance benchmark suite
  - Cache statistics API for monitoring performance
  - SIGWINCH handler for automatic cache invalidation on Unix
  - `with_pooled_string()` helper for efficient string building
  - `cached_unicode_width()` for cached width calculations
  - `cached_terminal_size()` for cached terminal dimensions
  - Performance documentation and guides
  - Memory profiling example
  - Caching demonstration example

### Changed

- Optimized string allocations throughout rendering pipeline
- Improved Unicode text processing performance (2-3x faster with caching)
- Reduced memory allocations by 24-87% depending on operation
- Enhanced batch rendering performance (10-20% faster with caching)

### Performance Improvements

- Simple box rendering: ~30x faster (45μs → 1.57μs)
- Unicode content: ~40x faster (120μs → 2.93μs)
- Large content: ~8x faster (850μs → 102.75μs)
- Allocations reduced: 19-87 per operation (from ~25-100)
- Cache hit rates: >90% for typical workloads

### Documentation

- Added comprehensive performance guide (`docs/performance.md`)
- Added optimization summary (`OPTIMIZATION_SUMMARY.md`)
- Added performance features guide (`PERFORMANCE_FEATURES.md`)
- Added work completion report (`WORK_COMPLETED.md`)
- Updated examples with performance demonstrations

## [0.1.3] - 2025-09-10

### Added

- Initial release of boxen for Rust
- Core box rendering functionality
- Border style system with predefined styles
- Text processing with Unicode support
- Color system with multiple format support
- Builder pattern API
- Convenience functions (simple_box, double_box, round_box)
- Comprehensive documentation and examples
- Performance optimizations
- Error handling with descriptive messages

[Unreleased]: https://github.com/sabry-awad97/boxen/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/sabry-awad97/boxen/releases/tag/v0.1.3
