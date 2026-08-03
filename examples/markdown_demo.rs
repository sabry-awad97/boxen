//! Comprehensive demonstration of all markdown rendering features in boxen
//!
//! This example showcases:
//! - Headers (H1-H6)
//! - Text formatting (bold, italic, strikethrough)
//! - Code (inline and blocks)
//! - Lists (ordered and unordered)
//! - Links
//! - Horizontal rules
//! - Blockquotes
//! - Complex combinations

use boxen::markdown::{ItalicStyle, LinkStyle, MarkdownStyle};
use boxen::{Color, builder, markdown_box};

fn main() {
    println!("\n=== MARKDOWN DEMO: All Features ===\n");

    // Demo 1: Basic markdown with default styling
    demo_basic_features();

    // Demo 2: Headers showcase
    demo_headers();

    // Demo 3: Text formatting
    demo_text_formatting();

    // Demo 4: Code examples
    demo_code();

    // Demo 5: Lists
    demo_lists();

    // Demo 6: Links
    demo_links();

    // Demo 7: Custom styling
    demo_custom_styling();

    // Demo 8: Complex real-world example
    demo_real_world();
}

fn demo_basic_features() {
    println!("📦 Demo 1: Basic Features\n");

    let content = r#"
# Welcome to Boxen

This is **markdown rendering** in action!

You can use *italic*, **bold**, and even ~~strikethrough~~.

Inline code like `cargo run` works too.

---

That was a horizontal rule above.
"#;

    println!("{}\n", markdown_box(content));
}

fn demo_headers() {
    println!("📦 Demo 2: All Header Levels\n");

    let content = r#"
# H1 - Largest Heading
## H2 - Major Section
### H3 - Subsection
"#;

    let result = builder()
        .title("Headers Showcase")
        .markdown()
        .width(50)
        .render(content)
        .unwrap();

    println!("{}\n", result);
}

fn demo_text_formatting() {
    println!("📦 Demo 3: Text Formatting\n");

    let content = r#"
# Text Formatting

**Bold**, *italic*, ~~strikethrough~~

You can **combine *bold and italic* together**
"#;

    let result = builder()
        .title("Formatting")
        .markdown()
        .width(60)
        .render(content)
        .unwrap();

    println!("{}\n", result);
}

fn demo_code() {
    println!("📦 Demo 4: Code Examples\n");

    let content = r#"
# Code Examples

Use `inline code` for short snippets.

Commands: `cargo build --release`
"#;

    let result = builder()
        .title("Code Demo")
        .markdown()
        .width(50)
        .render(content)
        .unwrap();

    println!("{}\n", result);
}

fn demo_lists() {
    println!("📦 Demo 5: Lists\n");

    let content = r#"
# Lists

- First item
- Second item
- Third item

1. Step one
2. Step two
"#;

    let result = builder()
        .title("Lists")
        .markdown()
        .width(50)
        .render(content)
        .unwrap();

    println!("{}\n", result);
}

fn demo_links() {
    println!("📦 Demo 6: Links\n");

    // Show text only (default)
    let content1 = r#"
Check out [Rust](https://rust-lang.org) for more info.
"#;

    println!("Link style: ShowText (default)\n");
    let result1 = builder().markdown().width(50).render(content1).unwrap();
    println!("{}\n", result1);

    // Show URL in parentheses
    let style = MarkdownStyle {
        link_style: LinkStyle::ShowUrl,
        ..Default::default()
    };

    println!("Link style: ShowUrl\n");
    let result2 = builder()
        .markdown_with_style(style)
        .width(60)
        .render(content1)
        .unwrap();
    println!("{}\n", result2);
}

fn demo_custom_styling() {
    println!("📦 Demo 7: Custom Styling\n");

    let style = MarkdownStyle {
        h1_color: Color::Named("magenta".to_string()),
        bold_color: Some(Color::Named("yellow".to_string())),
        inline_code_fg: Some(Color::Named("bright-green".to_string())),
        list_marker: "→".to_string(),
        italic_style: ItalicStyle::Underline,
        ..Default::default()
    };

    let content = r#"
# Custom Colors

**Yellow bold** and *underlined italic*

- Item one
- Item two
"#;

    let result = builder()
        .title("Custom Style")
        .markdown_with_style(style)
        .width(50)
        .render(content)
        .unwrap();

    println!("{}\n", result);
}

fn demo_real_world() {
    println!("📦 Demo 8: Real-World CLI Help\n");

    let help_content = r#"
# boxen CLI

**Version:** 0.4.0

**Commands:** new, render, config

Use `boxen render "text"` to create boxes

Options: `--width`, `--style`, `--color`
"#;

    let result = builder()
        .title("Help")
        .markdown()
        .width(50)
        .border_color("cyan")
        .render(help_content)
        .unwrap();

    println!("{}\n", result);
}
