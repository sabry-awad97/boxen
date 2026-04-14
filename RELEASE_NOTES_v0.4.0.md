# Boxen v0.4.0 - Dynamic Sizing Release 🎯

We're excited to announce Boxen v0.4.0, featuring **dynamic width and height support** with a JavaScript-inspired closure API!

## 🌟 Highlights

### Dynamic Sizing with Closures

Create responsive terminal boxes that adapt to available space:

```rust
use boxen::builder;

// Use 80% of terminal width, minimum 30 columns
builder()
    .width(|available: usize| (available * 4 / 5).max(30))
    .render("Responsive box!")
    .unwrap();

// Use 50% of terminal height
builder()
    .height(|available: usize| (available / 2).max(10))
    .render("Multi\nLine\nContent")
    .unwrap();

// Fully responsive - both width and height adapt
builder()
    .width(|available: usize| (available * 3 / 4).max(40))
    .height(|available: usize| (available / 3).max(8))
    .render("Fully responsive!")
    .unwrap();
```

### Key Features

- 🎯 **Unified API** - Same `.width()` and `.height()` methods accept both fixed values and closures
- 📏 **Terminal-aware** - Closures receive available terminal space as parameter
- 🔄 **100% backward compatible** - Existing code using `.width(50)` continues to work
- 🧵 **Thread-safe** - Uses `Arc` for shared ownership across threads
- 🎨 **Flexible** - Mix fixed and dynamic dimensions as needed

## 📦 What's New

### Added

- **Dynamic Width/Height Types**: New `Width` and `Height` enums supporting both fixed and dynamic sizing
  - `Width::Fixed(usize)` and `Width::Dynamic(Arc<dyn Fn(usize) -> usize>)`
  - `Height::Fixed(usize)` and `Height::Dynamic(Arc<dyn Fn(usize) -> usize>)`
  - New example: `examples/dynamic_sizing_demo.rs`

- **Color Validation**: New `Color::validated()` method for early validation
  - Validates named colors and hex format
  - Returns descriptive errors with recommendations
  - 19 new tests covering validation scenarios

- **Thread Safety Verification**: Comprehensive test suite verifying all types are `Send + Sync`
  - 16 new tests with compile-time and runtime verification
  - All public types verified for thread safety

- **Spacing API Improvements**: New `Spacing::terminal_balanced()` constructor
  - Explicit method for terminal-balanced spacing
  - Enhanced documentation with clear warnings
  - 11 new tests covering all spacing constructors

### Changed

- **BREAKING**: `BoxenOptions.width` and `BoxenOptions.height` now use `Width` and `Height` enums
  - **Migration**: Change `width: Some(50)` to `width: Some(Width::Fixed(50))`
  - **Migration**: Change `height: Some(20)` to `height: Some(Height::Fixed(20))`
  - **Note**: Builder API unchanged - `.width(50)` continues to work via `From` trait

- **Test Infrastructure**: Enhanced reliability with 654 tests passing consistently
- **API Future-Proofing**: Added `#[non_exhaustive]` to all public enums
- **API Safety**: Added `#[must_use]` attributes to prevent accidental misuse

### Documentation

- **Safety Guarantees**: Comprehensive safety documentation in main library docs
- **README Updates**: New "Dynamic Sizing" section with practical examples
- **Enhanced Examples**: Updated examples list with `dynamic_sizing_demo`

## 📊 Test Coverage

All **654 tests** passing:

- ✅ 311 lib tests
- ✅ 19 color validation tests
- ✅ 62 integration tests
- ✅ 15 performance tests
- ✅ 5 pooled string RAII tests
- ✅ 11 spacing API tests
- ✅ 66 testsprite generated tests
- ✅ 16 thread safety tests
- ✅ 8 title color tests
- ✅ 16 TypeScript compatibility tests
- ✅ 125 doctests

## 🚀 Getting Started

### Installation

```toml
[dependencies]
boxen = "0.4.0"
```

### Quick Example

```rust
use boxen::builder;

// Fixed width (traditional)
builder()
    .width(50)
    .render("Fixed width box")
    .unwrap();

// Dynamic width (new!)
builder()
    .width(|available: usize| (available * 4 / 5).max(30))
    .render("Responsive box!")
    .unwrap();
```

### Try the Demo

```bash
cargo run --example dynamic_sizing_demo
```

## 📖 Documentation

- [API Documentation](https://docs.rs/boxen/0.4.0)
- [GitHub Repository](https://github.com/sabry-awad97/boxen)
- [Changelog](https://github.com/sabry-awad97/boxen/blob/main/CHANGELOG.md)

## 🙏 Acknowledgments

Thanks to all contributors and users who provided feedback and suggestions!

## 📜 License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

---

**Full Changelog**: https://github.com/sabry-awad97/boxen/compare/v0.3.3...v0.4.0
