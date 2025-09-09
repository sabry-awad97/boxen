# Customization Guide

This guide covers advanced customization techniques for creating unique and sophisticated terminal boxes with boxen.

## Table of Contents

- [Custom Border Styles](#custom-border-styles)
- [Advanced Color Techniques](#advanced-color-techniques)
- [Dynamic Layouts](#dynamic-layouts)
- [Unicode and International Text](#unicode-and-international-text)
- [ANSI Escape Sequences](#ansi-escape-sequences)
- [Performance Optimization](#performance-optimization)
- [Integration Patterns](#integration-patterns)

## Custom Border Styles

### Creating Custom Border Characters

You can define completely custom border styles using the `BorderChars` struct:

```rust
use ::boxen::{builder, BorderStyle, BorderChars};

let custom_chars = BorderChars {
    top_left: '╭',
    top_right: '╮',
    bottom_left: '╰',
    bottom_right: '╯',
    left: '│',
    right: '│',
    top: '─',
    bottom: '─',
};

let result = builder()
    .border_style(BorderStyle::Custom(custom_chars))
    .render("Custom border style")
    .unwrap();
```

### ASCII-Only Borders

For environments that don't support Unicode:

```rust
use ::boxen::{builder, BorderStyle, BorderChars};

let ascii_chars = BorderChars {
    top_left: '+',
    top_right: '+',
    bottom_left: '+',
    bottom_right: '+',
    left: '|',
    right: '|',
    top: '-',
    bottom: '-',
};

let result = builder()
    .border_style(BorderStyle::Custom(ascii_chars))
    .render("ASCII-only border")
    .unwrap();
```

### Themed Border Sets

Create themed border collections:

```rust
use ::boxen::{BorderChars, BorderStyle};

struct BorderThemes;

impl BorderThemes {
    fn retro() -> BorderChars {
        BorderChars {
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
            left: '║',
            right: '║',
            top: '═',
            bottom: '═',
        }
    }

    fn minimal() -> BorderChars {
        BorderChars {
            top_left: ' ',
            top_right: ' ',
            bottom_left: ' ',
            bottom_right: ' ',
            left: '│',
            right: '│',
            top: ' ',
            bottom: ' ',
        }
    }

    fn heavy() -> BorderChars {
        BorderChars {
            top_left: '┏',
            top_right: '┓',
            bottom_left: '┗',
            bottom_right: '┛',
            left: '┃',
            right: '┃',
            top: '━',
            bottom: '━',
        }
    }
}

// Usage
let result = builder()
    .border_style(BorderStyle::Custom(BorderThemes::retro()))
    .render("Retro-styled box")
    .unwrap();
```

## Advanced Color Techniques

### Gradient-Like Effects

While true gradients aren't supported in most terminals, you can create gradient-like effects:

```rust
use ::boxen::{builder, Color};

// Create multiple boxes with varying color intensities
let colors = ["#ff0000", "#ff3333", "#ff6666", "#ff9999", "#ffcccc"];

for (i, color) in colors.iter().enumerate() {
    let result = builder()
        .border_color(*color)
        .margin((0, i * 2, 0, 0))  // Offset each box
        .render(&format!("Level {}", i + 1))
        .unwrap();
    println!("{}", result);
}
```

### Color Palettes

Define consistent color schemes:

```rust
use ::boxen::Color;

struct ColorPalette {
    primary: Color,
    secondary: Color,
    success: Color,
    warning: Color,
    error: Color,
    info: Color,
}

impl ColorPalette {
    fn dark_theme() -> Self {
        Self {
            primary: Color::from("#61dafb"),
            secondary: Color::from("#282c34"),
            success: Color::from("#98d982"),
            warning: Color::from("#ffcc02"),
            error: Color::from("#f56565"),
            info: Color::from("#63b3ed"),
        }
    }

    fn light_theme() -> Self {
        Self {
            primary: Color::from("#0066cc"),
            secondary: Color::from("#f8f9fa"),
            success: Color::from("#28a745"),
            warning: Color::from("#ffc107"),
            error: Color::from("#dc3545"),
            info: Color::from("#17a2b8"),
        }
    }
}

// Usage
let palette = ColorPalette::dark_theme();
let result = builder()
    .border_color(palette.primary)
    .background_color(palette.secondary)
    .render("Themed box")
    .unwrap();
```

### Conditional Coloring

Apply colors based on content or context:

```rust
use ::boxen::{builder, Color};

fn status_color(status: &str) -> Color {
    match status.to_lowercase().as_str() {
        "success" | "ok" | "pass" => Color::from("green"),
        "warning" | "warn" => Color::from("yellow"),
        "error" | "fail" | "critical" => Color::from("red"),
        "info" | "debug" => Color::from("blue"),
        _ => Color::from("white"),
    }
}

fn create_status_box(message: &str, status: &str) -> String {
    builder()
        .border_color(status_color(status))
        .title(&format!("Status: {}", status.to_uppercase()))
        .padding(1)
        .render(message)
        .unwrap()
}
```

## Dynamic Layouts

### Responsive Boxes

Create boxes that adapt to terminal size:

```rust
use ::boxen::{builder, get_terminal_width, Float};

fn responsive_box(content: &str) -> String {
    let terminal_width = get_terminal_width();

    let (width, float) = match terminal_width {
        0..=40 => (None, Float::Left),           // Very narrow: no width limit
        41..=80 => (Some(terminal_width - 4), Float::Left),  // Narrow: small margin
        81..=120 => (Some(60), Float::Center),   // Medium: centered with fixed width
        _ => (Some(80), Float::Center),          // Wide: larger centered box
    };

    let mut box_builder = builder().float(float);

    if let Some(w) = width {
        box_builder = box_builder.width(w);
    }

    box_builder.render(content).unwrap()
}
```

### Multi-Column Layouts

Create side-by-side boxes:

```rust
use ::boxen::{builder, get_terminal_width};

fn two_column_layout(left_content: &str, right_content: &str) -> String {
    let terminal_width = get_terminal_width();
    let column_width = (terminal_width / 2).saturating_sub(4);

    let left_box = builder()
        .width(column_width)
        .title("Left Column")
        .render(left_content)
        .unwrap();

    let right_box = builder()
        .width(column_width)
        .title("Right Column")
        .render(right_content)
        .unwrap();

    // Combine boxes side by side (simplified example)
    let left_lines: Vec<&str> = left_box.lines().collect();
    let right_lines: Vec<&str> = right_box.lines().collect();

    let max_lines = left_lines.len().max(right_lines.len());
    let mut result = String::new();

    for i in 0..max_lines {
        let left_line = left_lines.get(i).unwrap_or(&"");
        let right_line = right_lines.get(i).unwrap_or(&"");
        result.push_str(&format!("{:<width$} {}\n", left_line, right_line, width = column_width + 2));
    }

    result
}
```

### Nested Boxes

Create boxes within boxes:

```rust
use ::boxen::{builder, BorderStyle};

fn nested_box_example() -> String {
    let inner_box = builder()
        .border_style(BorderStyle::Single)
        .padding(1)
        .render("Inner content")
        .unwrap();

    builder()
        .border_style(BorderStyle::Double)
        .padding(2)
        .title("Outer Box")
        .render(&inner_box)
        .unwrap()
}
```

## Unicode and International Text

### Handling Wide Characters

Boxen properly handles wide Unicode characters:

```rust
use ::boxen::builder;

let result = builder()
    .width(20)
    .text_alignment(TextAlignment::Center)
    .render("こんにちは世界")  // Japanese text
    .unwrap();

let result = builder()
    .width(25)
    .render("🎉 Emoji support! 🚀")
    .unwrap();
```

### Right-to-Left Text

For RTL languages, you might want to adjust alignment:

```rust
use ::boxen::{builder, TextAlignment};

let result = builder()
    .text_alignment(TextAlignment::Right)  // Better for RTL text
    .width(30)
    .render("مرحبا بالعالم")  // Arabic text
    .unwrap();
```

### Mixed Scripts

Handle text with mixed writing systems:

```rust
use ::boxen::builder;

let mixed_text = "English + 中文 + العربية + русский";
let result = builder()
    .width(40)
    .text_alignment(TextAlignment::Center)
    .render(mixed_text)
    .unwrap();
```

## ANSI Escape Sequences

### Preserving Existing Colors

Boxen preserves ANSI escape sequences in text:

```rust
use ::boxen::builder;
use colored::Colorize;

let colored_text = format!(
    "{} and {} text",
    "Red".red(),
    "Blue".blue()
);

let result = builder()
    .render(&colored_text)
    .unwrap();
```

### Combining with External Styling

```rust
use ::boxen::builder;
use colored::Colorize;

fn styled_error_box(message: &str) -> String {
    let styled_message = format!(
        "{} {}",
        "ERROR:".red().bold(),
        message.white()
    );

    builder()
        .border_style(BorderStyle::Bold)
        .border_color("red")
        .padding(1)
        .render(&styled_message)
        .unwrap()
}
```

### Progress Indicators

Create dynamic progress boxes:

```rust
use ::boxen::{builder, BorderStyle};

fn progress_box(current: usize, total: usize, message: &str) -> String {
    let percentage = (current * 100) / total;
    let bar_width = 20;
    let filled = (current * bar_width) / total;

    let progress_bar = format!(
        "[{}{}] {}%",
        "█".repeat(filled),
        "░".repeat(bar_width - filled),
        percentage
    );

    let content = format!("{}\n{}", message, progress_bar);

    builder()
        .border_style(BorderStyle::Round)
        .padding(1)
        .title(&format!("Progress ({}/{})", current, total))
        .render(&content)
        .unwrap()
}
```

## Performance Optimization

### Batch Processing

For multiple boxes, optimize by reusing configurations:

```rust
use ::boxen::{BoxenBuilder, BorderStyle};

struct BoxRenderer {
    template: BoxenBuilder,
}

impl BoxRenderer {
    fn new() -> Self {
        Self {
            template: builder()
                .border_style(BorderStyle::Round)
                .padding(1),
        }
    }

    fn render_batch(&self, items: &[&str]) -> Vec<String> {
        items.iter()
            .map(|item| self.template.clone().render(item).unwrap())
            .collect()
    }
}
```

### Memory-Efficient Large Text

For very large text content:

```rust
use ::boxen::builder;

fn efficient_large_box(lines: impl Iterator<Item = String>) -> String {
    // Process in chunks to avoid loading everything into memory
    let content: String = lines
        .take(1000)  // Limit to prevent memory issues
        .collect::<Vec<_>>()
        .join("\n");

    builder()
        .width(80)  // Fixed width avoids expensive calculations
        .render(content)
        .unwrap()
}
```

### Caching Strategies

```rust
use std::collections::HashMap;
use ::boxen::{builder, BoxenOptions};

struct BoxCache {
    cache: HashMap<String, String>,
}

impl BoxCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn get_or_create(&mut self, content: &str, options: &BoxenOptions) -> String {
        let key = format!("{:?}:{}", options, content);

        self.cache.entry(key).or_insert_with(|| {
            builder()
                .from_options(options.clone())
                .render(content)
                .unwrap()
        }).clone()
    }
}
```

## Integration Patterns

### CLI Application Integration

```rust
use ::boxen::{builder, BorderStyle, TextAlignment};
use clap::{Arg, Command};

fn create_cli_box(message: &str, style: &str, color: Option<&str>) -> String {
    let border_style = match style {
        "double" => BorderStyle::Double,
        "round" => BorderStyle::Round,
        "bold" => BorderStyle::Bold,
        _ => BorderStyle::Single,
    };

    let mut box_builder = builder()
        .border_style(border_style)
        .padding(1)
        .text_alignment(TextAlignment::Center);

    if let Some(color) = color {
        box_builder = box_builder.border_color(color);
    }

    box_builder.render(message).unwrap()
}

fn main() {
    let matches = Command::new("boxen-cli")
        .arg(Arg::new("message").required(true))
        .arg(Arg::new("style").long("style").default_value("single"))
        .arg(Arg::new("color").long("color"))
        .get_matches();

    let message = matches.get_one::<String>("message").unwrap();
    let style = matches.get_one::<String>("style").unwrap();
    let color = matches.get_one::<String>("color");

    println!("{}", create_cli_box(message, style, color.map(|s| s.as_str())));
}
```

### Logging Integration

```rust
use ::boxen::{builder, BorderStyle};
use log::{info, warn, error};

struct BoxedLogger;

impl BoxedLogger {
    fn info_box(message: &str) {
        let boxed = builder()
            .border_style(BorderStyle::Round)
            .border_color("blue")
            .title("ℹ INFO")
            .padding(1)
            .render(message)
            .unwrap();
        info!("\n{}", boxed);
    }

    fn warn_box(message: &str) {
        let boxed = builder()
            .border_style(BorderStyle::Bold)
            .border_color("yellow")
            .title("⚠ WARNING")
            .padding(1)
            .render(message)
            .unwrap();
        warn!("\n{}", boxed);
    }

    fn error_box(message: &str) {
        let boxed = builder()
            .border_style(BorderStyle::Bold)
            .border_color("red")
            .title("❌ ERROR")
            .padding(1)
            .render(message)
            .unwrap();
        error!("\n{}", boxed);
    }
}
```

### Testing Utilities

```rust
use ::boxen::{builder, BorderStyle};

#[cfg(test)]
mod test_utils {
    use super::*;

    pub fn test_result_box(test_name: &str, passed: bool, details: &str) -> String {
        let (symbol, color, status) = if passed {
            ("✅", "green", "PASSED")
        } else {
            ("❌", "red", "FAILED")
        };

        let content = format!("{}\n{}", test_name, details);

        builder()
            .border_style(BorderStyle::Round)
            .border_color(color)
            .title(&format!("{} {}", symbol, status))
            .padding(1)
            .render(&content)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;

    #[test]
    fn example_test() {
        let result = some_function();
        let passed = result == expected_value;

        println!("{}", test_result_box(
            "example_test",
            passed,
            &format!("Expected: {}, Got: {}", expected_value, result)
        ));

        assert!(passed);
    }
}
```

This customization guide provides advanced techniques for creating sophisticated and highly customized terminal boxes. Combine these patterns with the basic usage from the [Usage Guide](usage.md) to create powerful terminal interfaces.
