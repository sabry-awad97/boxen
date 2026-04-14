<div align="center">

# 📦 Boxen

**Create beautiful boxes in the terminal with Rust**

[![Crates.io](https://img.shields.io/crates/v/boxen.svg)](https://crates.io/crates/boxen)
[![Documentation](https://docs.rs/boxen/badge.svg)](https://docs.rs/boxen)
[![CI](https://github.com/sabry-awad97/boxen/workflows/CI/badge.svg)](https://github.com/sabry-awad97/boxen/actions/workflows/ci.yml)
[![Security Audit](https://github.com/sabry-awad97/boxen/workflows/Security%20Audit/badge.svg)](https://github.com/sabry-awad97/boxen/actions/workflows/security.yml)
[![License](https://img.shields.io/crates/l/boxen.svg)](https://github.com/sabry-awad97/boxen#license)
[![Downloads](https://img.shields.io/crates/d/boxen.svg)](https://crates.io/crates/boxen)

A Rust implementation of the popular [boxen](https://github.com/sindresorhus/boxen) library for creating styled terminal boxes around text.

[Features](#-features) •
[Installation](#-installation) •
[Quick Start](#-quick-start) •
[Examples](#-examples) •
[Documentation](#-documentation)

</div>

---

## ✨ Features

<table>
<tr>
<td width="50%">

### 🎨 Styling

- Multiple border styles (single, double, round, bold, custom)
- Rich color support (named, hex, RGB)
- Title support with positioning
- Dim borders and backgrounds

</td>
<td width="50%">

### 📐 Layout

- Flexible text alignment (left, center, right)
- Precise padding and margins
- Dynamic width/height with closures
- Fixed width/height constraints
- Fullscreen mode

</td>
</tr>
<tr>
<td width="50%">

### ⚡ Performance

- **30x faster** than baseline
- Thread-local string pooling
- Optional Unicode width caching
- Optional terminal size caching

</td>
<td width="50%">

### 🛡️ Quality

- Type-safe API with builder pattern
- Comprehensive error handling
- Unicode and ANSI aware
- 100% backward compatible

</td>
</tr>
</table>

---

## 📦 Installation

Add boxen to your `Cargo.toml`:

```toml
[dependencies]
boxen = "0.3"
```

For maximum performance, enable caching features:

```toml
[dependencies]
boxen = { version = "0.3", features = ["width-cache", "terminal-cache"] }
```

---

## 🚀 Quick Start

```rust
use boxen::{boxen, builder, BorderStyle, TextAlignment};

fn main() {
    // Simple box with default settings
    let simple = boxen("Hello, World!", None).unwrap();
    println!("{}", simple);
    // ┌─────────────┐
    // │Hello, World!│
    // └─────────────┘

    // Styled box with builder pattern
    let fancy = builder()
        .border_style(BorderStyle::Double)
        .padding(2)
        .text_alignment(TextAlignment::Center)
        .title("Greeting")
        .border_color("blue")
        .render("Hello, World!")
        .unwrap();
    println!("{}", fancy);
}
```

---

## 📚 Examples

### Basic Usage

<table>
<tr>
<td width="50%">

**Code:**

```rust
use boxen::boxen;

let result = boxen("Simple box", None)
    .unwrap();
println!("{}", result);
```

</td>
<td width="50%">

**Output:**

```
┌──────────┐
│Simple box│
└──────────┘
```

</td>
</tr>
</table>

### Styled Box

<table>
<tr>
<td width="50%">

**Code:**

```rust
use boxen::{builder, BorderStyle};

let result = builder()
    .border_style(BorderStyle::Round)
    .padding(1)
    .title("Status")
    .border_color("green")
    .render("All systems operational")
    .unwrap();
```

</td>
<td width="50%">

**Output:**

```
╭─── Status ────╮
│               │
│ All systems   │
│ operational   │
│               │
╰───────────────╯
```

</td>
</tr>
</table>

### Convenience Functions

```rust
use boxen::{simple_box, double_box, round_box};

println!("{}", simple_box("Default style"));
println!("{}", double_box("Double border"));
println!("{}", round_box("Round corners"));
```

### Advanced Styling

```rust
use boxen::{builder, BorderStyle, TextAlignment, TitleAlignment, Float};

let result = builder()
    .border_style(BorderStyle::Bold)
    .padding((2, 4, 2, 4))  // top, right, bottom, left
    .margin(1)
    .text_alignment(TextAlignment::Center)
    .title_alignment(TitleAlignment::Center)
    .float(Float::Center)
    .width(40)
    .title("🎉 Celebration")
    .border_color("#ff6b6b")
    .background_color("#ffe66d")
    .render("Congratulations!\nYou've mastered boxen!")
    .unwrap();
```

### Dynamic Sizing

Boxen supports both fixed and dynamic width/height using closures that adapt to available terminal space:

```rust
use boxen::builder;

// Fixed width (traditional approach)
let result = builder()
    .width(50)
    .render("Fixed width box")
    .unwrap();

// Dynamic width - use 80% of available terminal width
let result = builder()
    .width(|available: usize| (available * 4 / 5).max(30))
    .render("This box adapts to terminal width")
    .unwrap();

// Dynamic height - use 50% of available terminal height
let result = builder()
    .height(|available: usize| (available / 2).max(10))
    .render("Multi\nLine\nContent")
    .unwrap();

// Both dynamic - fully responsive box
let result = builder()
    .width(|available: usize| (available * 3 / 4).max(40))
    .height(|available: usize| (available / 3).max(8))
    .render("Fully responsive box")
    .unwrap();

// Mix fixed and dynamic
let result = builder()
    .width(|available: usize| available.min(60))  // Cap at 60 columns
    .height(15)  // Fixed height
    .render("Dynamic width, fixed height")
    .unwrap();
```

**Key features:**

- 🎯 **Unified API** - Same `.width()` and `.height()` methods accept both fixed values and closures
- 📏 **Terminal-aware** - Closures receive available terminal space as parameter
- 🔄 **100% backward compatible** - Existing code using `.width(50)` continues to work
- 🎨 **Flexible** - Mix fixed and dynamic dimensions as needed

---

## 🎨 Border Styles

Boxen supports various border styles:

<table>
<tr>
<th>Style</th>
<th>Preview</th>
<th>Description</th>
</tr>
<tr>
<td><code>Single</code></td>
<td><pre>┌─┐
│ │
└─┘</pre></td>
<td>Clean single-line borders</td>
</tr>
<tr>
<td><code>Double</code></td>
<td><pre>╔═╗
║ ║
╚═╝</pre></td>
<td>Bold double-line borders</td>
</tr>
<tr>
<td><code>Round</code></td>
<td><pre>╭─╮
│ │
╰─╯</pre></td>
<td>Smooth rounded corners</td>
</tr>
<tr>
<td><code>Bold</code></td>
<td><pre>┏━┓
┃ ┃
┗━┛</pre></td>
<td>Heavy bold borders</td>
</tr>
<tr>
<td><code>SingleDouble</code></td>
<td><pre>╓─╖
║ ║
╙─╜</pre></td>
<td>Single horizontal, double vertical</td>
</tr>
<tr>
<td><code>DoubleSingle</code></td>
<td><pre>╒═╕
│ │
╘═╛</pre></td>
<td>Double horizontal, single vertical</td>
</tr>
<tr>
<td><code>Classic</code></td>
<td><pre>+--+
|  |
+--+</pre></td>
<td>ASCII-compatible classic style</td>
</tr>
</table>

---

## 🌈 Color Support

Boxen supports multiple color formats:

```rust
use boxen::builder;

// Named colors (16 standard terminal colors)
builder()
    .border_color("red")
    .background_color("blue");

// Hex colors
builder()
    .border_color("#ff0000")
    .background_color("#0000ff");

// RGB colors
builder()
    .border_color((255, 0, 0))
    .background_color((0, 0, 255));

// Title colors (independent from border color)
builder()
    .title("Status")
    .title_color("green")
    .border_color("blue");

// Dim borders for subtle styling
builder()
    .border_color("cyan")
    .dim_border(true);
```

**Available named colors:**
`black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`,
`bright-black`, `bright-red`, `bright-green`, `bright-yellow`, `bright-blue`, `bright-magenta`, `bright-cyan`, `bright-white`

---

## ⚡ Performance

Boxen is highly optimized for speed and memory efficiency:

### Benchmark Results

| Operation               | Time         | vs Baseline   |
| ----------------------- | ------------ | ------------- |
| Simple box              | **1.57μs**   | 30x faster ⚡ |
| Unicode content         | **2.93μs**   | 40x faster ⚡ |
| Complex styled box      | **12.2μs**   | -             |
| Large text (1000 chars) | **102.75μs** | 8x faster ⚡  |
| Batch (100 boxes)       | **150ms**    | 30x faster ⚡ |

### Core Optimizations

✅ **Thread-local string pooling** - Reduces allocations by 24-87%  
✅ **Smart buffer management** - Pre-allocated buffers with capacity hints  
✅ **Efficient ANSI handling** - Proper escape sequence processing  
✅ **Unicode optimization** - Fast width calculations

### Optional Performance Features

Enable caching for even better performance:

```toml
[dependencies]
boxen = { version = "0.3", features = ["width-cache", "terminal-cache"] }
```

| Feature          | Benefit             | Use Case                   |
| ---------------- | ------------------- | -------------------------- |
| `width-cache`    | 2-3x faster Unicode | Apps with CJK text, emoji  |
| `terminal-cache` | 10-20% faster batch | Rendering multiple boxes   |
| `dhat-heap`      | Memory profiling    | Development & optimization |

**Performance gains:**

- > 90% cache hit rates for typical workloads
- Lock-free thread-local caching
- Automatic cache invalidation on terminal resize (Unix)
- Configurable cache sizes and TTL

📖 See [Performance Guide](docs/performance.md) for detailed information.

---

## 🎯 Use Cases

<table>
<tr>
<td width="33%">

### CLI Tools

```rust
// Success messages
println!("{}",
    simple_box("✓ Build successful!")
);

// Error messages
println!("{}",
    builder()
        .border_color("red")
        .render("✗ Build failed")
        .unwrap()
);
```

</td>
<td width="33%">

### Status Displays

```rust
// System status
println!("{}",
    builder()
        .title("System Status")
        .border_color("green")
        .render("All systems operational")
        .unwrap()
);
```

</td>
<td width="33%">

### Notifications

```rust
// User notifications
println!("{}",
    builder()
        .title("🔔 Notification")
        .padding(2)
        .render("You have 3 new messages")
        .unwrap()
);
```

</td>
</tr>
</table>

---

## 📖 Documentation

### Guides

- 📘 [API Documentation](https://docs.rs/boxen) - Complete API reference

### Examples

Run the included examples to see boxen in action:

```bash
# Basic usage patterns
cargo run --example main_api_demo

# Dynamic width/height sizing
cargo run --example dynamic_sizing_demo

# Color demonstrations
cargo run --example color_demo

# Comprehensive feature showcase
cargo run --example comprehensive_demo

# Performance testing
cargo run --example performance_demo

# Caching features demo
cargo run --example caching_demo --features width-cache,terminal-cache

# Memory profiling
cargo run --example memory_profiling --features dhat-heap

# Error handling patterns
cargo run --example error_handling_demo

# Fullscreen mode
cargo run --example fullscreen_demo

# Interactive clock with spinner
cargo run --example clock_spinner
```

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. 🐛 **Report bugs** - Open an issue with details
2. 💡 **Suggest features** - Share your ideas
3. 📝 **Improve docs** - Help others learn
4. 🔧 **Submit PRs** - Fix bugs or add features

Please read our [Contributing Guide](CONTRIBUTING.md) for details.

---

## 📜 License

This project is licensed under either of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- **MIT license** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

## 🙏 Acknowledgments

- Inspired by the original [boxen](https://github.com/sindresorhus/boxen) TypeScript library by [Sindre Sorhus](https://github.com/sindresorhus)
- Built with ❤️ for the Rust community
- Thanks to all [contributors](https://github.com/sabry-awad97/boxen/graphs/contributors)

---

<div align="center">

**[⬆ back to top](#-boxen)**

Made with 🦀 Rust • [Report Bug](https://github.com/sabry-awad97/boxen/issues) • [Request Feature](https://github.com/sabry-awad97/boxen/issues)

</div>
