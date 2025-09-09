//! # Border System
//!
//! This module provides comprehensive border functionality for terminal boxes, including
//! predefined border styles, custom border character sets, and utilities for working
//! with different border configurations.
//!
//! ## Overview
//!
//! The border system consists of three main components:
//! - **Border Characters**: Individual Unicode characters that make up the border
//! - **Border Styles**: Predefined sets of characters for common border types
//! - **Border Utilities**: Helper functions for validation, comparison, and manipulation
//!
//! ## Quick Start
//!
//! ```rust
//! use ::boxen::{BorderStyle, BorderChars};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Use a predefined border style
//! let style = BorderStyle::Double;
//! let chars = style.get_chars()?;
//!
//! // Create a custom border
//! let custom_chars = BorderChars::uniform('*');
//! let custom_style = BorderStyle::Custom(custom_chars);
//! # Ok(())
//! # }
//! ```
//!
//! ## Available Border Styles
//!
//! ### Predefined Styles
//!
//! - **Single**: Standard single-line Unicode box drawing characters
//! - **Double**: Double-line Unicode box drawing characters for emphasis
//! - **Round**: Rounded corners with single lines for a softer appearance
//! - **Bold**: Thick/bold lines for strong visual impact
//! - **`SingleDouble`**: Single horizontal lines with double vertical lines
//! - **`DoubleSingle`**: Double horizontal lines with single vertical lines
//! - **Classic**: ASCII-compatible characters (+, -, |) for maximum compatibility
//! - **None**: No visible border (content only)
//! - **Custom**: User-defined character set with validation
//!
//! ### Visual Examples
//!
//! ```text
//! Single:        Double:        Round:         Bold:
//! ┌─────┐        ╔═════╗        ╭─────╮        ┏━━━━━┓
//! │Hello│        ║Hello║        │Hello│        ┃Hello┃
//! └─────┘        ╚═════╝        ╰─────╯        ┗━━━━━┛
//!
//! Classic:       SingleDouble:  DoubleSingle:
//! +-----+        ╓─────╖        ╒═════╕
//! |Hello|        ║Hello║        │Hello│
//! +-----+        ╙─────╜        ╘═════╛
//! ```
//!
//! ## Border Character Validation
//!
//! All border characters undergo validation to ensure they are:
//! - Printable (not control characters)
//! - Non-whitespace (visible borders)
//! - Compatible with terminal rendering
//!
//! ```rust
//! use ::boxen::BorderChars;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let chars = BorderChars::single();
//! chars.validate()?; // Validates all 8 border characters
//! # Ok(())
//! # }
//! ```
//!
//! ## Performance Considerations
//!
//! - Border character lookup is O(1) for predefined styles
//! - Custom border validation is performed once during creation
//! - Border width calculation is constant time
//! - Character sets are cached and reused
//!
//! ## Error Handling
//!
//! Border operations can fail in the following scenarios:
//! - Invalid custom border characters (whitespace, control characters)
//! - Unknown border style names during parsing
//! - Malformed custom border definitions
//!
//! All errors include detailed messages and actionable recommendations for resolution.
//!
//! ## Thread Safety
//!
//! All border types and utilities are thread-safe and can be safely shared
//! between threads or used in concurrent rendering operations.

pub mod chars;
pub mod styles;

// Re-export key utilities for convenient access
pub use styles::BorderStyleUtils;
