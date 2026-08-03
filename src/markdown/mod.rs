//! Markdown rendering support for terminal boxes.
//!
//! This module provides markdown parsing and styling capabilities that convert
//! markdown syntax to ANSI-styled terminal output. It integrates seamlessly with
//! boxen's builder pattern and supports extensive customization.
//!
//! # Features
//!
//! - **Headers**: H1-H6 with customizable colors
//! - **Text Formatting**: Bold, italic, strikethrough
//! - **Code**: Inline code and code blocks with styling
//! - **Lists**: Ordered and unordered with custom markers
//! - **Links**: Multiple display styles (text only, with URL, etc.)
//! - **Horizontal Rules**: Customizable characters and colors
//! - **Blockquotes**: Visual markers and styling
//!
//! # Quick Start
//!
//! ```rust
//! use boxen::builder;
//!
//! let markdown = r#"
//! # Welcome
//!
//! **Bold** and *italic* text work great!
//!
//! Use `inline code` for commands.
//! "#;
//!
//! let result = builder()
//!     .markdown()  // Enable markdown rendering
//!     .render(markdown)
//!     .unwrap();
//!
//! println!("{}", result);
//! ```
//!
//! # Custom Styling
//!
//! ```rust
//! use boxen::{builder, Color, markdown::{MarkdownStyle, LinkStyle}};
//!
//! let style = MarkdownStyle {
//!     h1_color: Color::Named("blue".to_string()),
//!     bold_color: Some(Color::Named("yellow".to_string())),
//!     link_style: LinkStyle::ShowUrl,
//!     ..Default::default()
//! };
//!
//! let result = builder()
//!     .markdown_with_style(style)
//!     .render("# Custom **colors**!")
//!     .unwrap();
//! ```
//!
//! # Configuration
//!
//! Control which markdown features are enabled:
//!
//! ```rust
//! use boxen::{builder, markdown::MarkdownConfig};
//!
//! let config = MarkdownConfig {
//!     headers: true,
//!     bold: true,
//!     italic: false,  // Disable italic
//!     code: true,
//!     ..Default::default()
//! };
//!
//! let result = builder()
//!     .markdown_with_config(config)
//!     .render("**bold** but not *italic*")
//!     .unwrap();
//! ```
//!
//! # Architecture
//!
//! The markdown module is architecturally separate from box rendering:
//! - Markdown conversion happens before text layout calculation
//! - ANSI codes are injected but don't affect width calculations
//! - The `MarkdownRenderer` is a pure transformation step
//!
//! This design ensures markdown rendering integrates cleanly with all other
//! boxen features like padding, alignment, and wrapping.

mod renderer;
mod styles;

pub use renderer::MarkdownRenderer;
pub use styles::{BoldStyle, ItalicStyle, LinkStyle, MarkdownConfig, MarkdownStyle};

/// Render markdown text to ANSI-styled string with default styling
///
/// This is a convenience function for quick markdown rendering without
/// needing to create a `MarkdownRenderer` instance.
///
/// # Examples
///
/// ```rust
/// use boxen::markdown::render_markdown;
///
/// let result = render_markdown("# Heading\n**bold text**");
/// assert!(result.contains("Heading"));
/// ```
pub fn render_markdown(text: &str) -> String {
    let renderer = MarkdownRenderer::new(MarkdownStyle::default(), MarkdownConfig::default());
    renderer.render(text)
}
