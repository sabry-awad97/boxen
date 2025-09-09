# Usage Guide

This guide provides comprehensive examples and patterns for using the boxen library effectively.

## Table of Contents

- [Basic Usage](#basic-usage)
- [Builder Pattern](#builder-pattern)
- [Text Alignment](#text-alignment)
- [Spacing Control](#spacing-control)
- [Dimensions and Layout](#dimensions-and-layout)
- [Error Handling](#error-handling)
- [Performance Tips](#performance-tips)

## Basic Usage

### Simple Boxes

The simplest way to create a box is using the `boxen` function:

```rust
use ::boxen::boxen;

// Basic box with default settings
let result = boxen("Hello, World!", None).unwrap();
println!("{}", result);
// ┌─────────────┐
// │Hello, World!│
// └─────────────┘
```

### Convenience Functions

For common use cases, use the convenience functions:

```rust
use ::boxen::{simple_box, double_box, round_box};

println!("{}", simple_box("Single border"));
// ┌─────────────┐
// │Single border│
// └─────────────┘

println!("{}", double_box("Double border"));
// ╔═════════════╗
// ║Double border║
// ╚═════════════╝

println!("{}", round_box("Round corners"));
// ╭─────────────╮
// │Round corners│
// ╰─────────────╯
```

## Builder Pattern

The builder pattern provides the most flexibility and is the recommended approach for complex boxes:

### Basic Builder Usage

```rust
use ::boxen::{builder, BorderStyle, TextAlignment};

let result = builder()
    .border_style(BorderStyle::Double)
    .padding(1)
    .text_alignment(TextAlignment::Center)
    .render("Centered text")
    .unwrap();
```

### Method Chaining

All builder methods return `self`, allowing for fluent method chaining:

```rust
use ::boxen::{builder, BorderStyle, TextAlignment, TitleAlignment, Float};

let result = builder()
    .border_style(BorderStyle::Round)
    .padding(2)
    .margin(1)
    .text_alignment(TextAlignment::Center)
    .title("Status Report")
    .title_alignment(TitleAlignment::Center)
    .float(Float::Center)
    .width(40)
    .border_color("green")
    .background_color("#f0f0f0")
    .render("All systems operational")
    .unwrap();
```

### Convenience Builder Methods

The builder provides convenience methods for common patterns:

```rust
use ::boxen::builder;

let result = builder()
    .spacing(2)  // Sets both padding and margin to 2
    .colors("blue", "white")  // Sets border and background colors
    .size(50, 10)  // Sets width and height
    .center_all()  // Centers text, title, and float
    .title("Quick Setup")
    .render("Using convenience methods")
    .unwrap();
```

## Text Alignment

Control how text is aligned within the box:

### Left Alignment (Default)

```rust
use ::boxen::{builder, TextAlignment};

let result = builder()
    .text_alignment(TextAlignment::Left)
    .width(20)
    .render("Left aligned\ntext content")
    .unwrap();
// ┌──────────────────┐
// │Left aligned      │
// │text content      │
// └──────────────────┘
```

### Center Alignment

```rust
use ::boxen::{builder, TextAlignment};

let result = builder()
    .text_alignment(TextAlignment::Center)
    .width(20)
    .render("Center aligned\ntext content")
    .unwrap();
// ┌──────────────────┐
// │  Center aligned  │
// │  text content    │
// └──────────────────┘
```

### Right Alignment

```rust
use ::boxen::{builder, TextAlignment};

let result = builder()
    .text_alignment(TextAlignment::Right)
    .width(20)
    .render("Right aligned\ntext content")
    .unwrap();
// ┌──────────────────┐
// │     Right aligned│
// │     text content │
// └──────────────────┘
```

## Spacing Control

### Padding

Padding adds space between the border and content:

```rust
use ::boxen::{builder, Spacing};

// Uniform padding
let result = builder()
    .padding(2)
    .render("Padded content")
    .unwrap();

// Asymmetric padding (follows CSS convention: top, right, bottom, left)
let result = builder()
    .padding((1, 3, 1, 3))
    .render("Custom padding")
    .unwrap();

// Using Spacing struct for fine control
let spacing = Spacing {
    top: 1,
    right: 4,
    bottom: 2,
    left: 4,
};
let result = builder()
    .padding(spacing)
    .render("Fine-tuned padding")
    .unwrap();
```

### Margins

Margins add space around the entire box:

```rust
use ::boxen::builder;

// Uniform margin
let result = builder()
    .margin(2)
    .render("Margined box")
    .unwrap();

// Custom margins
let result = builder()
    .margin((0, 2, 1, 2))  // No top margin, 2 sides, 1 bottom
    .render("Custom margins")
    .unwrap();
```

### Combined Spacing

```rust
use ::boxen::builder;

let result = builder()
    .padding(1)
    .margin(2)
    .render("Padded and margined")
    .unwrap();
```

## Dimensions and Layout

### Fixed Dimensions

```rust
use ::boxen::builder;

// Fixed width
let result = builder()
    .width(30)
    .render("This text will wrap at 30 characters width")
    .unwrap();

// Fixed height (content will be truncated if too long)
let result = builder()
    .height(5)
    .render("Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7")
    .unwrap();

// Both width and height
let result = builder()
    .size(25, 6)
    .render("Fixed dimensions box")
    .unwrap();
```

### Float Positioning

Control horizontal positioning within the terminal:

```rust
use ::boxen::{builder, Float};

// Left-aligned (default)
let result = builder()
    .float(Float::Left)
    .render("Left positioned")
    .unwrap();

// Center-aligned
let result = builder()
    .float(Float::Center)
    .render("Center positioned")
    .unwrap();

// Right-aligned
let result = builder()
    .float(Float::Right)
    .render("Right positioned")
    .unwrap();
```

### Fullscreen Mode

```rust
use ::boxen::{builder, FullscreenMode};

// Auto fullscreen (fills terminal)
let result = builder()
    .fullscreen(FullscreenMode::Auto)
    .render("Fullscreen content")
    .unwrap();

// Custom fullscreen function
let custom_fn = |width: usize, height: usize| {
    (width - 10, height - 5)  // Leave some margin
};
let result = builder()
    .fullscreen(FullscreenMode::Custom(Box::new(custom_fn)))
    .render("Custom fullscreen")
    .unwrap();
```

## Error Handling

Boxen provides comprehensive error handling with descriptive messages:

### Common Error Patterns

```rust
use ::boxen::{builder, BoxenError};

// Handle configuration errors
match builder()
    .width(5)  // Too narrow
    .padding(10)  // Too much padding
    .render("This won't fit") {
    Ok(result) => println!("{}", result),
    Err(BoxenError::ConfigurationError(msg)) => {
        eprintln!("Configuration error: {}", msg);
        // The error message will suggest fixes
    }
    Err(e) => eprintln!("Unexpected error: {}", e),
}

// Handle color errors
match builder()
    .border_color("invalid-color")
    .render("Test") {
    Ok(result) => println!("{}", result),
    Err(BoxenError::InvalidColor(msg)) => {
        eprintln!("Color error: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Error Recovery

```rust
use ::boxen::{builder, BoxenError, simple_box};

fn safe_boxen(text: &str) -> String {
    builder()
        .border_style(BorderStyle::Double)
        .padding(2)
        .render(text)
        .unwrap_or_else(|_| simple_box(text))  // Fallback to simple box
}
```

### Validation Helpers

```rust
use ::boxen::{validate_configuration, BoxenOptions, BorderStyle};

let options = BoxenOptions {
    border_style: BorderStyle::Double,
    width: Some(20),
    padding: Spacing::from(2),
    ..Default::default()
};

match validate_configuration(&options, "Sample text") {
    Ok(_) => println!("Configuration is valid"),
    Err(errors) => {
        for error in errors {
            eprintln!("Validation error: {}", error);
        }
    }
}
```

## Performance Tips

### Efficient String Handling

```rust
use ::boxen::builder;

// Prefer &str over String when possible
fn create_box(text: &str) -> String {
    builder().render(text).unwrap()
}

// For repeated operations, reuse builder
let builder_template = builder()
    .border_style(BorderStyle::Double)
    .padding(1);

let box1 = builder_template.clone().render("Text 1").unwrap();
let box2 = builder_template.clone().render("Text 2").unwrap();
```

### Large Text Optimization

```rust
use ::boxen::builder;

// For very large text, consider chunking
fn create_large_box(lines: &[String]) -> String {
    let text = lines.join("\n");
    builder()
        .width(80)  // Set explicit width to avoid expensive calculations
        .render(text)
        .unwrap()
}
```

### Caching Terminal Dimensions

```rust
use ::boxen::{get_terminal_width, builder};

// Cache terminal width for multiple boxes
let terminal_width = get_terminal_width();
let max_width = terminal_width.saturating_sub(10);  // Leave margin

let box1 = builder().width(max_width).render("Box 1").unwrap();
let box2 = builder().width(max_width).render("Box 2").unwrap();
```

## Advanced Patterns

### Conditional Styling

```rust
use ::boxen::{builder, BorderStyle};

fn status_box(message: &str, is_error: bool) -> String {
    let mut box_builder = builder()
        .padding(1)
        .text_alignment(TextAlignment::Center);
    
    if is_error {
        box_builder = box_builder
            .border_style(BorderStyle::Bold)
            .border_color("red")
            .title("❌ Error");
    } else {
        box_builder = box_builder
            .border_style(BorderStyle::Round)
            .border_color("green")
            .title("✅ Success");
    }
    
    box_builder.render(message).unwrap()
}
```

### Template Pattern

```rust
use ::boxen::{BoxenBuilder, BorderStyle, TextAlignment};

struct BoxTemplate {
    builder: BoxenBuilder,
}

impl BoxTemplate {
    fn new() -> Self {
        Self {
            builder: builder()
                .border_style(BorderStyle::Round)
                .padding(2)
                .text_alignment(TextAlignment::Center),
        }
    }
    
    fn info_box(&self, message: &str) -> String {
        self.builder.clone()
            .border_color("blue")
            .title("ℹ Info")
            .render(message)
            .unwrap()
    }
    
    fn warning_box(&self, message: &str) -> String {
        self.builder.clone()
            .border_color("yellow")
            .title("⚠ Warning")
            .render(message)
            .unwrap()
    }
}
```

This usage guide covers the most common patterns and use cases for the boxen library. For more advanced customization options, see the [Customization Guide](customization.md).