//! # Text Processing Engine
//!
//! This module provides a comprehensive text processing system for the boxen library,
//! handling text measurement, alignment, wrapping, and formatting operations with
//! Unicode awareness and terminal-optimized algorithms.
//!
//! ## Overview
//!
//! The text processing engine is responsible for transforming raw text input into
//! properly formatted, aligned, and sized content that fits perfectly within box
//! constraints. It handles complex text operations while maintaining visual consistency
//! and optimal performance.
//!
//! ## Core Components
//!
//! ### Text Measurement (`measurement`)
//! - **Unicode-Aware Width Calculation**: Accurate character width detection
//! - **Line Length Analysis**: Determines visual width of text lines
//! - **Content Sizing**: Calculates space requirements for text blocks
//!
//! ### Text Alignment (`alignment`)
//! - **Horizontal Alignment**: Left, center, and right text positioning
//! - **Vertical Alignment**: Content positioning within height constraints
//! - **Padding Application**: Consistent spacing around text content
//!
//! ### Text Wrapping (`wrapping`)
//! - **Intelligent Line Breaking**: Smart word wrapping with overflow handling
//! - **Width Constraint Handling**: Ensures text fits within specified bounds
//! - **Preserve Formatting**: Maintains intentional line breaks and spacing
//!
//! ## Quick Start
//!
//! ```rust
//! use boxen::text::{wrap_text, align_lines, text_width};
//! use boxen::TextAlignment;
//!
//! // Wrap text to fit within width constraints
//! let wrapped = wrap_text("Long text that needs wrapping", 20).unwrap();
//!
//! // Align text lines within a container
//! let aligned = align_lines(&wrapped, TextAlignment::Center, 30);
//!
//! // Measure text dimensions
//! let width = text_width("Sample text");
//! ```
//!
//! ## Text Measurement System
//!
//! The measurement system provides accurate text dimension calculations:
//!
//! ### Unicode Support
//! Handles complex Unicode characters including:
//! - **Combining Characters**: Properly measures accented characters
//! - **Wide Characters**: Accounts for CJK characters (2-column width)
//! - **Zero-Width Characters**: Ignores non-printing characters
//! - **Emoji Support**: Correctly measures emoji width
//!
//! ```rust
//! use boxen::text::text_width;
//!
//! // Accurate width calculation for various text types
//! assert_eq!(text_width("Hello"), 5);
//! assert_eq!(text_width("こんにちは"), 10); // Wide characters
//! assert_eq!(text_width("café"), 4);      // Combining characters
//! ```
//!
//! ### Line Analysis
//! Analyzes text blocks to determine optimal layout:
//!
//! ```rust
//! use boxen::text::{max_line_width, line_widths};
//!
//! let text = "Line 1\nLonger line 2\nShort";
//! let lines: Vec<&str> = text.lines().collect();
//! let max_width = max_line_width(&lines);
//! let widths = line_widths(text);
//! let line_count = widths.len();
//! ```
//!
//! ## Text Alignment System
//!
//! The alignment system provides flexible text positioning within containers:
//!
//! ### Horizontal Alignment
//! - **Left**: Text aligned to the left edge (default)
//! - **Center**: Text centered within available width
//! - **Right**: Text aligned to the right edge
//!
//! ```rust
//! use boxen::text::align_line;
//! use boxen::TextAlignment;
//!
//! let line = "Centered text";
//! let width = 20;
//!
//! let left = align_line(line, TextAlignment::Left, width);
//! let center = align_line(line, TextAlignment::Center, width);
//! let right = align_line(line, TextAlignment::Right, width);
//! ```
//!
//! ### Vertical Alignment
//! Controls text positioning within height constraints:
//!
//! ```rust
//! use boxen::text::apply_height_constraints;
//!
//! let lines = vec!["Line 1".to_string(), "Line 2".to_string()];
//! let constrained = apply_height_constraints(&lines, 5); // Pad to 5 lines
//! ```
//!
//! ### Padding Integration
//! Applies consistent spacing around text content:
//!
//! ```rust
//! use boxen::text::apply_padding;
//! use boxen::Spacing;
//!
//! let lines = vec!["Content".to_string()];
//! let padding = Spacing::from((1, 2, 1, 2)); // top, right, bottom, left
//! let padded = apply_padding(&lines, &padding, 20);
//! ```
//!
//! ## Text Wrapping System
//!
//! The wrapping system intelligently breaks text to fit within width constraints:
//!
//! ### Smart Word Breaking
//! - **Word Boundaries**: Prefers breaking at word boundaries
//! - **Hyphenation**: Handles long words that don't fit
//! - **Preserve Breaks**: Maintains intentional line breaks
//! - **Overflow Handling**: Gracefully handles edge cases
//!
//! ```rust
//! use ::boxen::text::wrap_text;
//!
//! // Intelligent word wrapping
//! let text = "This is a long sentence that needs to be wrapped";
//! let wrapped = wrap_text(text, 15).unwrap();
//!
//! // Result: ["This is a long", "sentence that", "needs to be", "wrapped"]
//! ```
//!
//! ### Constraint Handling
//! Ensures text fits within specified bounds while maintaining readability:
//!
//! ```rust
//! use boxen::text::wrap_text_preserve_words;
//!
//! // Advanced wrapping with word preservation
//! let wrapped = wrap_text_preserve_words(
//!     "Very long text content",
//!     20     // max width
//! ).unwrap();
//! ```
//!
//! ## Performance Optimizations
//!
//! The text processing engine is optimized for performance:
//!
//! ### Efficient Algorithms
//! - **Linear Complexity**: Most operations scale linearly with text length
//! - **Minimal Allocations**: Reuses buffers where possible
//! - **Unicode Optimization**: Efficient character width calculations
//! - **Caching**: Memoizes expensive calculations
//!
//! ### Memory Management
//! - **String Reuse**: Minimizes string allocations
//! - **Lazy Processing**: Only processes text when needed
//! - **Streaming Support**: Handles large text blocks efficiently
//!
//! ## Error Handling
//!
//! Text processing operations include comprehensive error handling:
//!
//! ### Common Error Scenarios
//! - **Width Constraints**: When text cannot fit in specified width
//! - **Invalid Input**: Malformed or problematic text content
//! - **Resource Limits**: When text exceeds reasonable size limits
//!
//! ```rust
//! use boxen::text::wrap_text;
//!
//! match wrap_text("Content", 0) {
//!     Ok(lines) => println!("Wrapped: {:?}", lines),
//!     Err(e) => println!("Error: {}", e), // Width too small
//! }
//! ```
//!
//! ## Integration with Boxen
//!
//! The text processing system integrates seamlessly with boxen's rendering pipeline:
//!
//! ### Automatic Processing
//! Text is automatically processed during box rendering:
//!
//! ```rust
//! use boxen::BoxenBuilder;
//! use boxen::TextAlignment;
//!
//! // Text is automatically measured, wrapped, and aligned
//! let result = BoxenBuilder::new()
//!     .width(30)
//!     .text_alignment(TextAlignment::Center)
//!     .render("This text will be automatically processed")
//!     .unwrap();
//! ```
//!
//! ### Custom Processing
//! Advanced users can access text processing functions directly:
//!
//! ```rust
//! use boxen::text::{wrap_text, align_lines, apply_padding};
//! use boxen::{TextAlignment, Spacing};
//!
//! // Manual text processing pipeline
//! let wrapped = wrap_text("Custom processing", 20).unwrap();
//! let aligned = align_lines(&wrapped, TextAlignment::Center, 25);
//! let padded = apply_padding(&aligned, &Spacing::from(1), 27);
//! ```
//!
//! ## Thread Safety
//!
//! All text processing operations are thread-safe and can be used in concurrent environments.
//! The functions are pure and don't maintain internal state, making them safe for parallel processing.

/// Text alignment functionality
pub mod alignment;
/// Text measurement and width calculation
pub mod measurement;
/// Text wrapping and line breaking
pub mod wrapping;

pub use alignment::{
    align_line, align_lines, apply_height_constraints, apply_padding, calculate_content_height,
    calculate_content_width, process_text_alignment, process_text_with_height_constraints,
};
pub use measurement::*;
pub use wrapping::*;
