//! # Color System
//!
//! This module provides comprehensive color functionality for terminal boxes, including
//! color parsing, validation, and application to text content. It supports multiple
//! color specification formats and provides robust error handling with helpful recommendations.
//!
//! ## Overview
//!
//! The color system supports three main color specification formats:
//! - **Named Colors**: Standard terminal color names (red, blue, green, etc.)
//! - **Hex Colors**: CSS-style hexadecimal color codes (#FF0000, #F00)
//! - **RGB Colors**: Direct RGB component specification (255, 0, 0)
//!
//! ## Quick Start
//!
//! ```rust
//! use boxen::color::{parse_color, apply_colors, validate_color};
//! use boxen::Color;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Named colors
//! let red = Color::Named("red".to_string());
//! let blue = Color::Named("blue".to_string());
//!
//! // Hex colors (both 3 and 6 digit formats)
//! let orange = Color::Hex("#FF8000".to_string());
//! let green = Color::Hex("#0F0".to_string());
//!
//! // RGB colors
//! let purple = Color::Rgb(128, 0, 128);
//!
//! // Validate colors before use
//! validate_color(&red)?;
//!
//! // Apply colors to text
//! let styled = apply_colors("Hello", Some(&red), Some(&blue))?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Supported Named Colors
//!
//! ### Standard Colors
//! - `black`, `red`, `green`, `yellow`, `blue`, `magenta`/`purple`, `cyan`, `white`
//!
//! ### Bright Colors
//! - `bright_black`/`gray`/`grey`, `bright_red`, `bright_green`, `bright_yellow`
//! - `bright_blue`, `bright_magenta`/`bright_purple`, `bright_cyan`, `bright_white`
//!
//! ### Alternative Formats
//! - Underscore format: `bright_red`, `bright_blue`
//! - Compact format: `brightred`, `brightblue`
//! - Case insensitive: `RED`, `Blue`, `GREEN`
//!
//! ## Hex Color Formats
//!
//! ### 6-Digit Format
//! ```rust
//! use boxen::Color;
//!
//! let red = Color::Hex("#FF0000".to_string());
//! let green = Color::Hex("#00FF00".to_string());
//! let blue = Color::Hex("#0000FF".to_string());
//! ```
//!
//! ### 3-Digit Format (Shorthand)
//! ```rust
//! use boxen::Color;
//!
//! let red = Color::Hex("#F00".to_string());    // Expands to #FF0000
//! let green = Color::Hex("#0F0".to_string());  // Expands to #00FF00
//! let blue = Color::Hex("#00F".to_string());   // Expands to #0000FF
//! ```
//!
//! ### Optional Hash Prefix
//! ```rust
//! use boxen::Color;
//!
//! let with_hash = Color::Hex("#FF0000".to_string());
//! let without_hash = Color::Hex("FF0000".to_string());  // Also valid
//! ```
//!
//! ## RGB Color Format
//!
//! ```rust
//! use boxen::Color;
//!
//! let red = Color::Rgb(255, 0, 0);
//! let green = Color::Rgb(0, 255, 0);
//! let blue = Color::Rgb(0, 0, 255);
//! let black = Color::Rgb(0, 0, 0);       // Pure black
//! let white = Color::Rgb(255, 255, 255); // Pure white
//! ```
//!
//! ## Color Application
//!
//! ### Foreground Colors
//! ```rust
//! use boxen::color::apply_foreground_color;
//! use boxen::Color;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let red_text = apply_foreground_color("Error", &Color::Named("red".to_string()))?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Background Colors
//! ```rust
//! use boxen::color::apply_background_color;
//! use boxen::Color;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let highlighted = apply_background_color("Important", &Color::Named("yellow".to_string()))?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Combined Colors
//! ```rust
//! use boxen::color::apply_colors;
//! use boxen::Color;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let fg = Color::Named("white".to_string());
//! let bg = Color::Named("red".to_string());
//! let styled = apply_colors("Alert", Some(&fg), Some(&bg))?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Dim Styling
//! ```rust
//! use boxen::color::{apply_dim, apply_color_with_dim};
//! use boxen::Color;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Apply only dim effect
//! let dimmed = apply_dim("Subtle text");
//!
//! // Apply color with optional dim effect
//! let dim_red = apply_color_with_dim(
//!     "Warning",
//!     Some(&Color::Named("red".to_string())),
//!     true  // Enable dim
//! )?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! The color system provides detailed error messages with actionable recommendations:
//!
//! ```rust
//! use boxen::color::validate_color;
//! use boxen::Color;
//!
//! # fn main() {
//! let invalid = Color::Named("invalid_color".to_string());
//! match validate_color(&invalid) {
//!     Ok(_) => println!("Color is valid"),
//!     Err(e) => {
//!         println!("Error: {}", e);
//!         for rec in e.recommendations() {
//!             println!("Suggestion: {}", rec.suggestion);
//!             if let Some(fix) = &rec.auto_fix {
//!                 println!("Auto-fix: {}", fix);
//!             }
//!         }
//!     }
//! }
//! # }
//! ```
//!
//! ## Performance Considerations
//!
//! - Color parsing is cached internally by the `colored` crate
//! - Named color lookup is O(1) using match statements
//! - Hex parsing is optimized for both 3 and 6 digit formats
//! - RGB colors have no parsing overhead
//! - Color validation is performed once during creation
//!
//! ## Terminal Compatibility
//!
//! - Named colors work on all terminals supporting basic ANSI colors
//! - RGB/Hex colors require true color terminal support
//! - Graceful fallback to nearest colors on limited terminals
//! - Dim styling is widely supported across terminal emulators
//!
//! ## Thread Safety
//!
//! All color operations are thread-safe and can be used concurrently
//! without synchronization concerns.

use crate::ErrorRecommendation;
use crate::error::{BoxenError, BoxenResult};
use crate::options::Color;
use colored::{ColoredString, Colorize};

/// Parse and validate a color specification
pub fn parse_color(color: &Color) -> BoxenResult<colored::Color> {
    match color {
        Color::Named(name) => parse_named_color(name),
        Color::Hex(hex) => parse_hex_color(hex),
        Color::Rgb(r, g, b) => Ok(colored::Color::TrueColor {
            r: *r,
            g: *g,
            b: *b,
        }),
    }
}

/// Parse a named color into a `colored::Color`
pub fn parse_named_color(name: &str) -> BoxenResult<colored::Color> {
    let normalized = name.to_lowercase();
    match normalized.as_str() {
        // Standard terminal colors
        "black" => Ok(colored::Color::Black),
        "red" => Ok(colored::Color::Red),
        "green" => Ok(colored::Color::Green),
        "yellow" => Ok(colored::Color::Yellow),
        "blue" => Ok(colored::Color::Blue),
        "magenta" | "purple" => Ok(colored::Color::Magenta),
        "cyan" => Ok(colored::Color::Cyan),
        "white" => Ok(colored::Color::White),

        // Bright colors
        "bright_black" | "brightblack" | "gray" | "grey" => Ok(colored::Color::BrightBlack),
        "bright_red" | "brightred" => Ok(colored::Color::BrightRed),
        "bright_green" | "brightgreen" => Ok(colored::Color::BrightGreen),
        "bright_yellow" | "brightyellow" => Ok(colored::Color::BrightYellow),
        "bright_blue" | "brightblue" => Ok(colored::Color::BrightBlue),
        "bright_magenta" | "brightmagenta" | "bright_purple" | "brightpurple" => {
            Ok(colored::Color::BrightMagenta)
        }
        "bright_cyan" | "brightcyan" => Ok(colored::Color::BrightCyan),
        "bright_white" | "brightwhite" => Ok(colored::Color::BrightWhite),

        _ => Err(BoxenError::invalid_color(
            format!("Unknown color name: {}", name),
            name.to_string(),
            vec![
                ErrorRecommendation::suggestion_only(
                    "Unknown color name".to_string(),
                    "Use a standard terminal color name like 'red', 'blue', 'green', etc."
                        .to_string(),
                ),
                ErrorRecommendation::with_auto_fix(
                    "Use standard color".to_string(),
                    "Try using 'red' as a common color".to_string(),
                    "\"red\"".to_string(),
                ),
                ErrorRecommendation::suggestion_only(
                    "Alternative: Use hex color".to_string(),
                    "You can also use hex colors like '#FF0000' for red".to_string(),
                ),
            ],
        )),
    }
}

/// Parse a hex color string into a colored::Color
pub fn parse_hex_color(hex: &str) -> BoxenResult<colored::Color> {
    let hex = hex.trim_start_matches('#');

    // Validate hex string length
    if hex.len() != 3 && hex.len() != 6 {
        return Err(BoxenError::invalid_color(
            format!("Invalid hex color format: #{}", hex),
            format!("#{}", hex),
            vec![
                ErrorRecommendation::suggestion_only(
                    "Invalid hex length".to_string(),
                    "Hex colors must be 3 or 6 characters long (e.g., #F00 or #FF0000)".to_string(),
                ),
                ErrorRecommendation::with_auto_fix(
                    "Use 6-digit format".to_string(),
                    "Try using the full 6-digit hex format".to_string(),
                    "\"#FF0000\"".to_string(),
                ),
            ],
        ));
    }

    // Validate hex characters
    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(BoxenError::invalid_color(
            format!("Invalid hex color format: #{}", hex),
            format!("#{}", hex),
            vec![
                ErrorRecommendation::suggestion_only(
                    "Invalid hex characters".to_string(),
                    "Hex colors can only contain digits 0-9 and letters A-F".to_string(),
                ),
                ErrorRecommendation::with_auto_fix(
                    "Use valid hex color".to_string(),
                    "Try using a valid hex color".to_string(),
                    "\"#FF0000\"".to_string(),
                ),
            ],
        ));
    }

    let (r, g, b) = if hex.len() == 3 {
        // Short format: #RGB -> #RRGGBB
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| {
            BoxenError::invalid_color(
                format!("Invalid hex color: #{}", hex),
                format!("#{}", hex),
                vec![ErrorRecommendation::with_auto_fix(
                    "Invalid hex format".to_string(),
                    "Use a valid 3-digit hex color".to_string(),
                    "\"#F00\"".to_string(),
                )],
            )
        })?;
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| {
            BoxenError::invalid_color(
                format!("Invalid hex color: #{}", hex),
                format!("#{}", hex),
                vec![ErrorRecommendation::with_auto_fix(
                    "Invalid hex format".to_string(),
                    "Use a valid 3-digit hex color".to_string(),
                    "\"#0F0\"".to_string(),
                )],
            )
        })?;
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| {
            BoxenError::invalid_color(
                format!("Invalid hex color: #{}", hex),
                format!("#{}", hex),
                vec![ErrorRecommendation::with_auto_fix(
                    "Invalid hex format".to_string(),
                    "Use a valid 3-digit hex color".to_string(),
                    "\"#00F\"".to_string(),
                )],
            )
        })?;
        (r, g, b)
    } else {
        // Long format: #RRGGBB
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| {
            BoxenError::invalid_color(
                format!("Invalid hex color: #{}", hex),
                format!("#{}", hex),
                vec![ErrorRecommendation::with_auto_fix(
                    "Invalid hex format".to_string(),
                    "Use a valid 6-digit hex color".to_string(),
                    "\"#FF0000\"".to_string(),
                )],
            )
        })?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| {
            BoxenError::invalid_color(
                format!("Invalid hex color: #{}", hex),
                format!("#{}", hex),
                vec![ErrorRecommendation::with_auto_fix(
                    "Invalid hex format".to_string(),
                    "Use a valid 6-digit hex color".to_string(),
                    "\"#00FF00\"".to_string(),
                )],
            )
        })?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| {
            BoxenError::invalid_color(
                format!("Invalid hex color: #{}", hex),
                format!("#{}", hex),
                vec![ErrorRecommendation::with_auto_fix(
                    "Invalid hex format".to_string(),
                    "Use a valid 6-digit hex color".to_string(),
                    "\"#0000FF\"".to_string(),
                )],
            )
        })?;
        (r, g, b)
    };

    Ok(colored::Color::TrueColor { r, g, b })
}

/// Validate that a color specification is valid
pub fn validate_color(color: &Color) -> BoxenResult<()> {
    parse_color(color).map(|_| ())
}

/// Apply foreground color to text
pub fn apply_foreground_color(text: &str, color: &Color) -> BoxenResult<ColoredString> {
    let parsed_color = parse_color(color)?;
    Ok(text.color(parsed_color))
}

/// Apply background color to text
pub fn apply_background_color(text: &str, color: &Color) -> BoxenResult<ColoredString> {
    let parsed_color = parse_color(color)?;
    Ok(text.on_color(parsed_color))
}

/// Apply both foreground and background colors to text
pub fn apply_colors(
    text: &str,
    fg_color: Option<&Color>,
    bg_color: Option<&Color>,
) -> BoxenResult<ColoredString> {
    let mut styled = ColoredString::from(text);

    if let Some(fg) = fg_color {
        let parsed_fg = parse_color(fg)?;
        styled = styled.color(parsed_fg);
    }

    if let Some(bg) = bg_color {
        let parsed_bg = parse_color(bg)?;
        styled = styled.on_color(parsed_bg);
    }

    Ok(styled)
}

/// Apply dim styling to text (for dim borders)
#[must_use]
pub fn apply_dim(text: &str) -> ColoredString {
    text.dimmed()
}

/// Apply color and dim styling to text
pub fn apply_color_with_dim(
    text: &str,
    color: Option<&Color>,
    dim: bool,
) -> BoxenResult<ColoredString> {
    let mut styled = if let Some(c) = color {
        apply_foreground_color(text, c)?
    } else {
        ColoredString::from(text)
    };

    if dim {
        styled = styled.dimmed();
    }

    Ok(styled)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::Color;

    #[test]
    fn test_parse_named_colors_basic() {
        assert!(parse_named_color("red").is_ok());
        assert!(parse_named_color("green").is_ok());
        assert!(parse_named_color("blue").is_ok());
        assert!(parse_named_color("black").is_ok());
        assert!(parse_named_color("white").is_ok());
        assert!(parse_named_color("yellow").is_ok());
        assert!(parse_named_color("magenta").is_ok());
        assert!(parse_named_color("cyan").is_ok());
    }

    #[test]
    fn test_parse_named_colors_bright() {
        assert!(parse_named_color("bright_red").is_ok());
        assert!(parse_named_color("brightred").is_ok());
        assert!(parse_named_color("bright_green").is_ok());
        assert!(parse_named_color("bright_blue").is_ok());
        assert!(parse_named_color("gray").is_ok());
        assert!(parse_named_color("grey").is_ok());
        assert!(parse_named_color("bright_black").is_ok());
        assert!(parse_named_color("brightblack").is_ok());
    }

    #[test]
    fn test_parse_named_colors_aliases() {
        assert!(parse_named_color("purple").is_ok());
        assert!(parse_named_color("bright_purple").is_ok());
        assert!(parse_named_color("brightpurple").is_ok());
    }

    #[test]
    fn test_parse_named_colors_case_insensitive() {
        assert!(parse_named_color("RED").is_ok());
        assert!(parse_named_color("Red").is_ok());
        assert!(parse_named_color("rEd").is_ok());
        assert!(parse_named_color("BRIGHT_BLUE").is_ok());
        assert!(parse_named_color("Bright_Blue").is_ok());
    }

    #[test]
    fn test_parse_named_colors_invalid() {
        assert!(parse_named_color("invalid_color").is_err());
        assert!(parse_named_color("").is_err());
        assert!(parse_named_color("orange").is_err());
        assert!(parse_named_color("pink").is_err());
    }

    #[test]
    fn test_parse_hex_colors_long_format() {
        // Valid 6-digit hex colors
        assert!(parse_hex_color("#FF0000").is_ok()); // Red
        assert!(parse_hex_color("#00FF00").is_ok()); // Green
        assert!(parse_hex_color("#0000FF").is_ok()); // Blue
        assert!(parse_hex_color("#FFFFFF").is_ok()); // White
        assert!(parse_hex_color("#000000").is_ok()); // Black
        assert!(parse_hex_color("#123456").is_ok()); // Random valid hex
        assert!(parse_hex_color("#ABCDEF").is_ok()); // With letters
        assert!(parse_hex_color("#abcdef").is_ok()); // Lowercase
    }

    #[test]
    fn test_parse_hex_colors_short_format() {
        // Valid 3-digit hex colors
        assert!(parse_hex_color("#F00").is_ok()); // Red
        assert!(parse_hex_color("#0F0").is_ok()); // Green
        assert!(parse_hex_color("#00F").is_ok()); // Blue
        assert!(parse_hex_color("#FFF").is_ok()); // White
        assert!(parse_hex_color("#000").is_ok()); // Black
        assert!(parse_hex_color("#123").is_ok()); // Random valid hex
        assert!(parse_hex_color("#ABC").is_ok()); // With letters
        assert!(parse_hex_color("#abc").is_ok()); // Lowercase
    }

    #[test]
    fn test_parse_hex_colors_without_hash() {
        // Should work without # prefix
        assert!(parse_hex_color("FF0000").is_ok());
        assert!(parse_hex_color("F00").is_ok());
        assert!(parse_hex_color("123456").is_ok());
        assert!(parse_hex_color("ABC").is_ok());
    }

    #[test]
    fn test_parse_hex_colors_invalid_length() {
        assert!(parse_hex_color("#FF").is_err()); // Too short
        assert!(parse_hex_color("#FFFF").is_err()); // Invalid length
        assert!(parse_hex_color("#FFFFF").is_err()); // Invalid length
        assert!(parse_hex_color("#FFFFFFF").is_err()); // Too long
        assert!(parse_hex_color("").is_err()); // Empty
        assert!(parse_hex_color("#").is_err()); // Just hash
    }

    #[test]
    fn test_parse_hex_colors_invalid_characters() {
        assert!(parse_hex_color("#GGGGGG").is_err()); // Invalid hex chars
        assert!(parse_hex_color("#FF00ZZ").is_err()); // Invalid hex chars
        assert!(parse_hex_color("#FF 000").is_err()); // Space
        assert!(parse_hex_color("#FF-000").is_err()); // Dash
        assert!(parse_hex_color("#FF.000").is_err()); // Dot
    }

    #[test]
    fn test_parse_hex_color_values() {
        // Test that hex parsing produces correct RGB values
        let red = parse_hex_color("#FF0000").unwrap();
        if let colored::Color::TrueColor { r, g, b } = red {
            assert_eq!(r, 255);
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        } else {
            panic!("Expected TrueColor");
        }

        let green = parse_hex_color("#00FF00").unwrap();
        if let colored::Color::TrueColor { r, g, b } = green {
            assert_eq!(r, 0);
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        } else {
            panic!("Expected TrueColor");
        }

        // Test short format expansion
        let short_red = parse_hex_color("#F00").unwrap();
        if let colored::Color::TrueColor { r, g, b } = short_red {
            assert_eq!(r, 255); // F -> FF
            assert_eq!(g, 0); // 0 -> 00
            assert_eq!(b, 0); // 0 -> 00
        } else {
            panic!("Expected TrueColor");
        }
    }

    #[test]
    fn test_parse_color_enum_variants() {
        // Test Color::Named
        let named = Color::Named("red".to_string());
        assert!(parse_color(&named).is_ok());

        // Test Color::Hex
        let hex = Color::Hex("#FF0000".to_string());
        assert!(parse_color(&hex).is_ok());

        // Test Color::Rgb
        let rgb = Color::Rgb(255, 0, 0);
        let parsed = parse_color(&rgb).unwrap();
        if let colored::Color::TrueColor { r, g, b } = parsed {
            assert_eq!(r, 255);
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        } else {
            panic!("Expected TrueColor");
        }
    }

    #[test]
    fn test_validate_color() {
        // Valid colors
        assert!(validate_color(&Color::Named("red".to_string())).is_ok());
        assert!(validate_color(&Color::Hex("#FF0000".to_string())).is_ok());
        assert!(validate_color(&Color::Rgb(255, 0, 0)).is_ok());

        // Invalid colors
        assert!(validate_color(&Color::Named("invalid_color".to_string())).is_err());
        assert!(validate_color(&Color::Hex("#GGGGGG".to_string())).is_err());
    }

    #[test]
    fn test_apply_foreground_color() {
        let color = Color::Named("red".to_string());
        let result = apply_foreground_color("test", &color);
        assert!(result.is_ok());

        // Test with invalid color
        let invalid_color = Color::Named("invalid".to_string());
        let result = apply_foreground_color("test", &invalid_color);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_background_color() {
        let color = Color::Named("blue".to_string());
        let result = apply_background_color("test", &color);
        assert!(result.is_ok());

        // Test with invalid color
        let invalid_color = Color::Named("invalid".to_string());
        let result = apply_background_color("test", &invalid_color);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_colors_both() {
        let fg_color = Color::Named("red".to_string());
        let bg_color = Color::Named("blue".to_string());
        let result = apply_colors("test", Some(&fg_color), Some(&bg_color));
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_colors_foreground_only() {
        let fg_color = Color::Named("red".to_string());
        let result = apply_colors("test", Some(&fg_color), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_colors_background_only() {
        let bg_color = Color::Named("blue".to_string());
        let result = apply_colors("test", None, Some(&bg_color));
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_colors_neither() {
        let result = apply_colors("test", None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_colors_invalid() {
        let invalid_color = Color::Named("invalid".to_string());
        let result = apply_colors("test", Some(&invalid_color), None);
        assert!(result.is_err());

        let result = apply_colors("test", None, Some(&invalid_color));
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_dim() {
        let result = apply_dim("test");
        // We can't easily test the actual ANSI codes, but we can verify it doesn't panic
        assert!(!result.to_string().is_empty());
    }

    #[test]
    fn test_apply_color_with_dim() {
        let color = Color::Named("red".to_string());

        // Test with color and dim
        let result = apply_color_with_dim("test", Some(&color), true);
        assert!(result.is_ok());

        // Test with color but no dim
        let result = apply_color_with_dim("test", Some(&color), false);
        assert!(result.is_ok());

        // Test with no color but dim
        let result = apply_color_with_dim("test", None, true);
        assert!(result.is_ok());

        // Test with no color and no dim
        let result = apply_color_with_dim("test", None, false);
        assert!(result.is_ok());

        // Test with invalid color
        let invalid_color = Color::Named("invalid".to_string());
        let result = apply_color_with_dim("test", Some(&invalid_color), false);
        assert!(result.is_err());
    }

    #[test]
    fn test_color_from_implementations() {
        // Test From<String> for Color
        let color1: Color = "red".to_string().into();
        assert!(matches!(color1, Color::Named(_)));

        let color2: Color = "#FF0000".to_string().into();
        assert!(matches!(color2, Color::Hex(_)));

        // Test From<&str> for Color
        let color3: Color = "blue".into();
        assert!(matches!(color3, Color::Named(_)));

        let color4: Color = "#00FF00".into();
        assert!(matches!(color4, Color::Hex(_)));

        // Test From<(u8, u8, u8)> for Color
        let color5: Color = (255, 0, 0).into();
        assert!(matches!(color5, Color::Rgb(255, 0, 0)));
    }

    #[test]
    fn test_edge_cases() {
        // Test empty string
        assert!(parse_named_color("").is_err());

        // Test whitespace
        assert!(parse_named_color(" red ").is_err()); // We don't trim in named colors

        // Test hex with whitespace (should fail)
        assert!(parse_hex_color(" #FF0000 ").is_err());

        // Test very long strings
        let long_name = "a".repeat(1000);
        assert!(parse_named_color(&long_name).is_err());

        // Test RGB edge values
        let rgb_max = Color::Rgb(255, 255, 255);
        assert!(parse_color(&rgb_max).is_ok());

        let rgb_min = Color::Rgb(0, 0, 0);
        assert!(parse_color(&rgb_min).is_ok());
    }

    #[test]
    fn test_hex_color_case_sensitivity() {
        // Both should work
        assert!(parse_hex_color("#ABCDEF").is_ok());
        assert!(parse_hex_color("#abcdef").is_ok());
        assert!(parse_hex_color("#AbCdEf").is_ok());

        // Short format too
        assert!(parse_hex_color("#ABC").is_ok());
        assert!(parse_hex_color("#abc").is_ok());
        assert!(parse_hex_color("#AbC").is_ok());
    }

    #[test]
    fn test_comprehensive_color_validation() {
        // Test a variety of valid colors
        let valid_colors = vec![
            Color::Named("red".to_string()),
            Color::Named("BLUE".to_string()),
            Color::Named("bright_green".to_string()),
            Color::Hex("#FF0000".to_string()),
            Color::Hex("#F00".to_string()),
            Color::Hex("00FF00".to_string()),
            Color::Rgb(128, 64, 192),
            Color::Rgb(0, 0, 0),
            Color::Rgb(255, 255, 255),
        ];

        for color in valid_colors {
            assert!(
                validate_color(&color).is_ok(),
                "Color should be valid: {:?}",
                color
            );
        }

        // Test a variety of invalid colors
        let invalid_colors = vec![
            Color::Named("invalid_color_name".to_string()),
            Color::Named("".to_string()),
            Color::Hex("#GGGGGG".to_string()),
            Color::Hex("#FF".to_string()),
            Color::Hex("#FFFFFFF".to_string()),
            Color::Hex("".to_string()),
        ];

        for color in invalid_colors {
            assert!(
                validate_color(&color).is_err(),
                "Color should be invalid: {:?}",
                color
            );
        }
    }
}
