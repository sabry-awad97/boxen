# Boxen

[![Crates.io](https://img.shields.io/crates/v/boxen.svg)](https://crates.io/crates/boxen)
[![Documentation](https://docs.rs/boxen/badge.svg)](https://docs.rs/boxen)
[![CI](https://github.com/sabry-awad97/boxen/workflows/CI/badge.svg)](https://github.com/sabry-awad97/boxen/actions/workflows/ci.yml)
[![Security Audit](https://github.com/sabry-awad97/boxen/workflows/Security%20Audit/badge.svg)](https://github.com/sabry-awad97/boxen/actions/workflows/security.yml)
[![License](https://img.shields.io/crates/l/boxen.svg)](https://github.com/sabry-awad97/boxen#license)
[![Downloads](https://img.shields.io/crates/d/boxen.svg)](https://crates.io/crates/boxen)
[![Rust Version](https://img.shields.io/badge/rust-1.85+-blue.svg)](https://www.rust-lang.org)
[![Dependabot](https://img.shields.io/badge/dependabot-enabled-brightgreen.svg)](https://github.com/sabry-awad97/boxen/network/dependencies)

A Rust implementation of the popular [boxen](https://github.com/sindresorhus/boxen) library for creating styled terminal boxes around text.

## Features

- 🎨 **Multiple border styles** - Single, double, round, bold, and custom borders
- 📐 **Flexible alignment** - Left, center, and right text alignment
- 🎯 **Precise spacing** - Fine-grained control over padding and margins
- 🌈 **Rich colors** - Support for named colors, hex codes, and RGB values
- 📝 **Title support** - Add titles with customizable positioning
- 🔤 **Unicode aware** - Proper handling of Unicode characters and ANSI escape sequences
- 📱 **Responsive** - Fullscreen mode and terminal-aware layouts
- ⚡ **Performance optimized** - Minimal allocations and efficient text processing
- 🛡️ **Type safe** - Comprehensive error handling with descriptive messages

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
boxen = "0.1.3"
```

## Quick Start

```rust
use ::boxen::{boxen, builder, BorderStyle, TextAlignment};

fn main() {
    // Simple box with default settings
    let simple = boxen("Hello, World!", None).unwrap();
    println!("{}", simple);

    // Using the builder pattern for more control
    let fancy = builder()
        .border_style(BorderStyle::Double)
        .padding(2)
        .margin(1)
        .text_alignment(TextAlignment::Center)
        .title("Greeting")
        .border_color("blue")
        .render("Hello, World!")
        .unwrap();
    println!("{}", fancy);
}
```

## Examples

### Basic Usage

```rust
use ::boxen::boxen;

let result = boxen("Simple box", None).unwrap();
println!("{}", result);
```

Output:

```
┌──────────┐
│Simple box│
└──────────┘
```

### Builder Pattern

```rust
use ::boxen::{builder, BorderStyle, TextAlignment};

let result = builder()
    .border_style(BorderStyle::Round)
    .padding(1)
    .text_alignment(TextAlignment::Center)
    .width(20)
    .title("Status")
    .border_color("green")
    .render("All systems operational")
    .unwrap();
println!("{}", result);
```

Output:

```
╭─── Status ────╮
│               │
│  All systems  │
│  operational  │
│               │
╰───────────────╯
```

### Convenience Functions

```rust
use ::boxen::{simple_box, double_box, round_box};

println!("{}", simple_box("Default style"));
println!("{}", double_box("Double border"));
println!("{}", round_box("Round corners"));
```

### Advanced Styling

```rust
use ::boxen::{builder, BorderStyle, TextAlignment, Float};

let result = builder()
    .border_style(BorderStyle::Bold)
    .padding((2, 4, 2, 4))  // top, right, bottom, left
    .margin(1)
    .text_alignment(TextAlignment::Center)
    .title_alignment(TitleAlignment::Center)
    .float(Float::Center)
    .width(40)
    .height(8)
    .title("🎉 Celebration")
    .border_color("#ff6b6b")
    .background_color("#ffe66d")
    .render("Congratulations!\nYou've mastered boxen!")
    .unwrap();
println!("{}", result);
```

### Error Handling

```rust
use ::boxen::{builder, BoxenError};

match builder()
    .width(5)  // Too narrow
    .padding(10)  // Too much padding
    .render("This won't fit") {
    Ok(result) => println!("{}", result),
    Err(BoxenError::ConfigurationError(msg)) => {
        eprintln!("Configuration error: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Border Styles

Boxen supports various border styles:

| Style          | Preview           |
| -------------- | ----------------- |
| `Single`       | `┌─┐ │ │ └─┘`     |
| `Double`       | `╔═╗ ║ ║ ╚═╝`     |
| `Round`        | `╭─╮ │ │ ╰─╯`     |
| `Bold`         | `┏━┓ ┃ ┃ ┗━┛`     |
| `SingleDouble` | `╓─╖ ║ ║ ╙─╜`     |
| `DoubleSingle` | `╒═╕ │ │ ╘═╛`     |
| `Classic`      | `+--+ \| \| +--+` |

## Color Support

Boxen supports multiple color formats:

```rust
use ::boxen::builder;

// Named colors
builder().border_color("red");
builder().background_color("blue");

// Hex colors
builder().border_color("#ff0000");
builder().background_color("#0000ff");

// RGB colors
builder().border_color((255, 0, 0));
builder().background_color((0, 0, 255));
```

## Performance

Boxen is highly optimized for speed and memory efficiency:

### Core Optimizations

- **Thread-local string pooling**: Reduces memory allocations by 24-87%
- **Unicode width caching**: 2-3x faster Unicode text processing
- **Terminal size caching**: 10-20% faster batch rendering
- **Smart buffer management**: Pre-allocated buffers with capacity hints
- **Efficient ANSI handling**: Proper escape sequence processing

### Benchmark Results

Measured on a modern machine:

- Simple box: **~1.57μs** (30x faster than baseline)
- Unicode content: **~2.93μs** (40x faster than baseline)
- Complex styled box: **~12.2μs**
- Large text (1000 chars): **~102.75μs** (8x faster than baseline)
- Batch rendering (100 boxes): **~150ms**

### Optional Performance Features

Enable caching for even better performance:

```toml
[dependencies]
boxen = { version = "0.1.3", features = ["width-cache", "terminal-cache"] }
```

**Available Features**:

- `width-cache`: LRU cache for Unicode width calculations (2-3x speedup)
- `terminal-cache`: TTL-based cache for terminal dimensions (10-20% speedup)
- `dhat-heap`: Memory profiling support for optimization

**Performance Gains**:

- > 90% cache hit rates for typical workloads
- Lock-free thread-local caching
- Automatic cache invalidation on terminal resize (Unix)
- Configurable cache sizes and TTL

See [Performance Guide](docs/performance.md) and [Performance Features](PERFORMANCE_FEATURES.md) for detailed information.

## Examples

Run the included examples to see boxen in action:

```bash
# Basic usage patterns
cargo run --example main_api_demo

# Color demonstrations
cargo run --example color_demo

# Comprehensive feature showcase
cargo run --example comprehensive_demo

# Performance testing
cargo run --example performance_demo

# Error handling patterns
cargo run --example error_handling_demo

# Fullscreen mode
cargo run --example fullscreen_demo

# Interactive clock with spinner
cargo run --example clock_spinner
```

## Documentation

- [API Documentation](https://docs.rs/boxen) - Complete API reference
- [Usage Guide](docs/usage.md) - Detailed usage examples
- [Customization Guide](docs/customization.md) - Advanced styling techniques

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- Inspired by the original [boxen](https://github.com/sindresorhus/boxen) TypeScript library by Sindre Sorhus
- Built with ❤️ for the Rust community

---

<div align="center">
  <sub>Built with 🦀 Rust</sub>
</div>
