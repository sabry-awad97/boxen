# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2026-01-25

### Fixed

- **Module naming conflict**: Renamed internal `boxen` module to `render` to avoid conflicts with crate name
  - Users can now use `use boxen::boxen` instead of requiring `use ::boxen::boxen`
  - Improves API ergonomics and follows Rust naming conventions
  - No breaking changes - all public APIs remain the same
- Fixed unused import warnings in `terminal/size_cache.rs` by adding proper feature gates

### Changed

- Internal module structure improved for better organization and clarity

## [0.3.0] - 2026-01-25

### Added

- **Performance Optimizations**: Major performance improvements across the board
  - Thread-local string pooling reduces allocations by 24-87%
  - Unicode width caching with LRU eviction (30-40x faster for repeated text)
  - Terminal size caching with TTL-based expiration (8x faster for large content)
- **New Features**:
  - `width-cache` feature for Unicode width caching
  - `terminal-cache` feature for terminal size caching
  - `dhat-heap` feature for memory profiling
- **Comprehensive Benchmarks**:
  - Allocation benchmarks for memory profiling
  - Performance benchmarks comparing cached vs uncached operations
  - Criterion-based benchmarks for statistical analysis
- **Documentation**:
  - Detailed performance guide (`docs/performance.md`)
  - Migration guide for upgrading from 0.2.x
  - Comprehensive examples for all features
  - API documentation improvements

### Changed

- Minimum Rust version bumped to 1.85.0 (Rust 2024 edition)
- Optimized string allocation patterns throughout the codebase
- Improved error messages with actionable recommendations

### Performance

- **Simple boxes**: 30x faster (2.5µs → 83ns)
- **Unicode text**: 40x faster (15µs → 375ns)
- **Large content**: 8x faster (45µs → 5.6µs)
- **Memory**: 24-87% reduction in allocations

## [0.2.0] - Previous Release

Initial stable release with core functionality.

[0.3.1]: https://github.com/sabry-awad97/boxen/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/sabry-awad97/boxen/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/sabry-awad97/boxen/releases/tag/v0.2.0
