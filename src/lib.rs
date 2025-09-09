#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![warn(clippy::all)]
// #![warn(clippy::pedantic)]
// #![warn(clippy::nursery)]
// #![warn(clippy::cargo)]
#![doc(html_root_url = "https://docs.rs/boxen")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/sabry-awad97/boxen/main/assets/logo.png")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sabry-awad97/boxen/main/assets/favicon.ico"
)]

//! # Boxen
//!
//! A Rust implementation of the boxen library for drawing styled boxes around text in terminals.
//!
//! Boxen provides a simple and flexible API for creating beautiful terminal boxes with support for:
//! - Multiple border styles (single, double, round, bold, etc.)
//! - Text alignment (left, center, right)
//! - Padding and margins with fine-grained control
//! - Colors for borders and backgrounds
//! - Titles with customizable positioning
//! - Unicode and ANSI escape sequence support
//! - Fullscreen mode and responsive layouts
//!
//! ## Quick Start
//!
//! ```rust
//! use ::boxen::{boxen, builder, BorderStyle, TextAlignment};
//!
//! // Simple box with default settings
//! let simple = boxen("Hello, World!", None).unwrap();
//! println!("{}", simple);
//!
//! // Using the builder pattern for more control
//! let fancy = builder()
//!     .border_style(BorderStyle::Double)
//!     .padding(2)
//!     .margin(1)
//!     .text_alignment(TextAlignment::Center)
//!     .title("Greeting")
//!     .border_color("blue")
//!     .render("Hello, World!")
//!     .unwrap();
//! println!("{}", fancy);
//! ```
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```rust
//! use ::boxen::boxen;
//!
//! let result = boxen("Simple box", None).unwrap();
//! // ┌──────────┐
//! // │Simple box│
//! // └──────────┘
//! ```
//!
//! ### Builder Pattern
//!
//! ```rust
//! use ::boxen::{builder, BorderStyle, TextAlignment, Color};
//!
//! let result = builder()
//!     .border_style(BorderStyle::Round)
//!     .padding(1)
//!     .text_alignment(TextAlignment::Center)
//!     .width(20)
//!     .title("Status")
//!     .border_color("green")
//!     .render("All systems operational")
//!     .unwrap();
//! ```
//!
//! ### Convenience Functions
//!
//! ```rust
//! use ::boxen::{simple_box, double_box, round_box};
//!
//! println!("{}", simple_box("Default style"));
//! println!("{}", double_box("Double border"));
//! println!("{}", round_box("Round corners"));
//! ```
//!
//! ## Performance
//!
//! Boxen is optimized for performance with:
//! - Minimal memory allocations in hot paths
//! - Efficient Unicode width calculation
//! - Smart ANSI escape sequence handling
//! - Pre-allocated string buffers for large content
//!
//! ## Error Handling
//!
//! All fallible operations return `Result<T, BoxenError>` with descriptive error messages
//! and helpful recommendations for fixing common issues.

pub mod borders;
pub mod boxen;
pub mod color;
pub mod error;
pub mod options;
pub mod terminal;
pub mod text;
pub mod validation;

#[cfg(test)]
mod error_tests;

// Re-export main types and functions for public API
pub use boxen::boxen;
pub use error::{BoxenError, BoxenResult, ErrorRecommendation};
pub use options::{
    BorderChars, BorderStyle, BoxenBuilder, BoxenOptions, Color, DimensionConstraints, Float,
    FullscreenMode, LayoutDimensions, Spacing, TextAlignment, TitleAlignment,
};
pub use validation::{
    MinimumDimensions, ValidationResult, auto_adjust_options, calculate_minimum_dimensions,
    suggest_optimal_dimensions, validate_configuration,
};

// Re-export terminal utilities
pub use terminal::{get_terminal_height, get_terminal_size, get_terminal_width};

/// Create a new `BoxenBuilder` for fluent configuration.
///
/// The builder pattern is the recommended way to create boxes with custom styling.
/// It provides a fluent interface for setting all available options.
///
/// # Examples
///
/// ```rust
/// use ::boxen::{builder, BorderStyle, TextAlignment};
///
/// let result = builder()
///     .border_style(BorderStyle::Double)
///     .padding(2)
///     .text_alignment(TextAlignment::Center)
///     .title("My Box")
///     .width(30)
///     .render("Hello, World!")
///     .unwrap();
/// ```
///
/// # Performance
///
/// The builder validates configuration only when `render()` is called,
/// allowing for efficient method chaining without intermediate validations.
#[must_use]
pub fn builder() -> BoxenBuilder {
    BoxenBuilder::new()
}

/// Create a simple box with default single border style.
///
/// This is a convenience function for quickly creating a basic box.
/// For more customization options, use [`builder()`] or [`boxen()`] with custom options.
///
/// # Examples
///
/// ```rust
/// use ::boxen::simple_box;
///
/// println!("{}", simple_box("Hello, World!"));
/// // ┌─────────────┐
/// // │Hello, World!│
/// // └─────────────┘
/// ```
///
/// # Error Handling
///
/// This function never panics. If box creation fails, it returns the original text.
pub fn simple_box<S: AsRef<str>>(text: S) -> String {
    let text_ref = text.as_ref();
    boxen(text_ref, None).unwrap_or_else(|_| text_ref.to_string())
}

/// Create a box with double border style.
///
/// This is a convenience function for creating a box with double-line borders.
///
/// # Examples
///
/// ```rust
/// use ::boxen::double_box;
///
/// println!("{}", double_box("Important!"));
/// // ╔═══════════╗
/// // ║Important! ║
/// // ╚═══════════╝
/// ```
///
/// # Error Handling
///
/// This function never panics. If box creation fails, it returns the original text.
pub fn double_box<S: AsRef<str>>(text: S) -> String {
    let text_ref = text.as_ref();
    let options = BoxenOptions {
        border_style: BorderStyle::Double,
        ..Default::default()
    };
    boxen(text_ref, Some(options)).unwrap_or_else(|_| text_ref.to_string())
}

/// Create a box with round border style.
///
/// This is a convenience function for creating a box with rounded corners.
///
/// # Examples
///
/// ```rust
/// use ::boxen::round_box;
///
/// println!("{}", round_box("Friendly message"));
/// // ╭─────────────────╮
/// // │Friendly message │
/// // ╰─────────────────╯
/// ```
///
/// # Error Handling
///
/// This function never panics. If box creation fails, it returns the original text.
pub fn round_box<S: AsRef<str>>(text: S) -> String {
    let text_ref = text.as_ref();
    let options = BoxenOptions {
        border_style: BorderStyle::Round,
        ..Default::default()
    };
    boxen(text_ref, Some(options)).unwrap_or_else(|_| text_ref.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_boxen() {
        let result = boxen("Hello", None);
        assert!(result.is_ok());
        let box_str = result.unwrap();
        assert!(box_str.contains("Hello"));
    }

    #[test]
    fn test_builder_pattern() {
        let builder = builder();
        assert!(builder.render("Test").is_ok());
    }

    #[test]
    fn test_builder_fluent_interface() {
        let result = builder()
            .border_style(BorderStyle::Double)
            .padding(2)
            .margin(1)
            .text_alignment(TextAlignment::Center)
            .title("Builder Test")
            .width(50)
            .border_color("blue")
            .render("Testing fluent interface");

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Testing fluent interface"));
        assert!(output.contains("Builder Test"));
    }

    #[test]
    fn test_builder_validation() {
        // Test that builder validates configuration
        let result = builder()
            .width(5) // Too small
            .padding(10) // Too large
            .render("Test");

        assert!(result.is_err());
    }

    #[test]
    fn test_builder_convenience_methods() {
        let result = builder()
            .spacing(1) // Sets both padding and margin
            .colors("red", "white") // Sets both border and background color
            .size(50, 8) // Use wider box to avoid text wrapping
            .center_all() // Centers text, title, and float
            .title("Convenience Test")
            .render("Testing convenience methods");

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Testing convenience methods"));
        assert!(output.contains("Convenience Test"));
    }

    #[test]
    fn test_convenience_functions() {
        assert!(simple_box("Test").contains("Test"));
        assert!(double_box("Test").contains("Test"));
        assert!(round_box("Test").contains("Test"));
    }

    #[test]
    fn test_spacing_from_usize() {
        let spacing = Spacing::from(2);
        assert_eq!(spacing.top, 2);
        assert_eq!(spacing.right, 6); // 3x horizontal
        assert_eq!(spacing.bottom, 2);
        assert_eq!(spacing.left, 6); // 3x horizontal
    }

    #[test]
    fn test_spacing_from_tuple() {
        let spacing = Spacing::from((1, 2, 3, 4));
        assert_eq!(spacing.top, 1);
        assert_eq!(spacing.right, 2);
        assert_eq!(spacing.bottom, 3);
        assert_eq!(spacing.left, 4);
    }

    #[test]
    fn test_color_from_string() {
        let color = Color::from("red");
        matches!(color, Color::Named(_));

        let hex_color = Color::from("#ff0000");
        matches!(hex_color, Color::Hex(_));
    }

    #[test]
    fn test_color_from_rgb() {
        let color = Color::from((255, 0, 0));
        matches!(color, Color::Rgb(255, 0, 0));
    }
}
