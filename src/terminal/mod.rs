//! # Terminal Integration System
//!
//! This module provides comprehensive terminal interaction capabilities for the boxen library,
//! enabling intelligent terminal size detection, dimension calculations, and adaptive rendering
//! that works seamlessly across different terminal environments and platforms.
//!
//! ## Overview
//!
//! The terminal system handles the complex task of interfacing with various terminal emulators
//! and environments to provide accurate dimension information and optimal rendering parameters.
//! It abstracts away platform-specific differences and provides a consistent API for terminal
//! operations.
//!
//! ## Core Capabilities
//!
//! ### Terminal Size Detection
//! - **Automatic Detection**: Intelligently detects current terminal dimensions
//! - **Fallback Handling**: Provides sensible defaults when detection fails
//! - **Cross-Platform Support**: Works on Windows, macOS, Linux, and other Unix-like systems
//! - **Environment Variable Support**: Respects `COLUMNS` and `LINES` environment variables
//!
//! ### Dimension Calculations
//! - **Border Width Calculation**: Determines space required for different border styles
//! - **Content Area Sizing**: Calculates available space for text content
//! - **Padding and Margin Integration**: Accounts for spacing in dimension calculations
//! - **Constraint Validation**: Ensures dimensions fit within terminal bounds
//!
//! ## Quick Start
//!
//! ```rust
//! use boxen::terminal::{get_terminal_width, get_terminal_height, calculate_border_width};
//! use boxen::BorderStyle;
//!
//! // Get current terminal dimensions
//! let width = get_terminal_width();
//! let height = get_terminal_height().unwrap_or(24);
//! println!("Terminal: {}x{}", width, height);
//!
//! // Calculate space needed for borders
//! let border_width = calculate_border_width(&BorderStyle::Double);
//! let content_width = width.saturating_sub(border_width);
//! ```
//!
//! ## Terminal Size Detection
//!
//! The terminal size detection system uses multiple strategies to determine dimensions:
//!
//! ### Detection Priority
//! 1. **Direct Terminal Query**: Uses platform-specific APIs to query terminal size
//! 2. **Environment Variables**: Falls back to `COLUMNS` and `LINES` variables
//! 3. **Sensible Defaults**: Uses 80x24 as final fallback for compatibility
//!
//! ### Platform Support
//! - **Unix/Linux**: Uses `ioctl` with `TIOCGWINSZ` for accurate detection
//! - **Windows**: Leverages Windows Console API for dimension queries
//! - **Cross-Platform**: Graceful degradation ensures functionality everywhere
//!
//! ```rust
//! use boxen::terminal::{get_terminal_width, get_terminal_height};
//!
//! // Safe terminal size detection
//! let width = get_terminal_width();
//! let height = get_terminal_height().unwrap_or(24);
//!
//! // Use dimensions for layout calculations
//! let max_box_width = width.saturating_sub(4); // Leave margin
//! ```
//!
//! ## Dimension Calculations
//!
//! The dimension calculation system provides utilities for determining space requirements:
//!
//! ### Border Width Calculation
//! Different border styles require different amounts of horizontal space:
//!
//! ```rust
//! use boxen::terminal::calculate_border_width;
//! use boxen::BorderStyle;
//!
//! // Calculate border overhead for different styles
//! let single_width = calculate_border_width(&BorderStyle::Single); // 2 chars
//! let double_width = calculate_border_width(&BorderStyle::Double); // 2 chars
//! let bold_width = calculate_border_width(&BorderStyle::Bold);     // 2 chars
//! ```
//!
//! ### Content Area Calculation
//! Determine available space for text content after accounting for borders and spacing:
//!
//! ```rust
//! use boxen::terminal::{get_terminal_width, calculate_border_width};
//! use boxen::{BorderStyle, Spacing};
//!
//! let terminal_width = get_terminal_width();
//! let border_width = calculate_border_width(&BorderStyle::Single);
//! let padding = Spacing::from(2); // 2 units on each side
//!
//! // Calculate available content width
//! let content_width = terminal_width
//!     .saturating_sub(border_width)
//!     .saturating_sub(padding.left + padding.right);
//! ```
//!
//! ## Error Handling and Fallbacks
//!
//! The terminal system is designed to be robust and handle various failure scenarios:
//!
//! ### Detection Failures
//! - **No Terminal**: When running in non-interactive environments
//! - **Permission Issues**: When terminal access is restricted
//! - **Platform Limitations**: When APIs are unavailable
//!
//! ### Graceful Degradation
//! ```rust
//! use boxen::terminal::get_terminal_width;
//!
//! // Always provides a usable width
//! let width = get_terminal_width();
//!
//! // Safe for all environments
//! assert!(width > 0);
//! ```
//!
//! ## Integration with Boxen Options
//!
//! The terminal system integrates seamlessly with boxen's configuration system:
//!
//! ### Automatic Sizing
//! When no explicit dimensions are provided, boxen uses terminal detection:
//!
//! ```rust
//! use ::boxen::builder;
//!
//! // Automatically sizes to terminal width
//! let result = builder()
//!     .render("Content adapts to terminal size")
//!     .unwrap();
//! ```
//!
//! ### Constraint Validation
//! Terminal dimensions are used to validate configuration options:
//!
//! ```rust
//! use boxen::BoxenBuilder;
//!
//! // Width is validated against terminal size
//! let result = BoxenBuilder::new()
//!     .width(60) // Reasonable width that fits in most terminals
//!     .render("Constrained content")
//!     .unwrap();
//! ```
//!
//! ## Performance Considerations
//!
//! - **Caching**: Terminal size is cached to avoid repeated system calls
//! - **Lazy Detection**: Size detection only occurs when needed
//! - **Minimal Overhead**: Calculations are optimized for common cases
//! - **No Blocking**: All operations are non-blocking and fast
//!
//! ## Thread Safety
//!
//! All terminal operations are thread-safe and can be used in concurrent environments:
//!
//! ```rust
//! use std::thread;
//! use boxen::terminal::get_terminal_width;
//!
//! // Safe to call from multiple threads
//! let handles: Vec<_> = (0..4)
//!     .map(|_| thread::spawn(|| get_terminal_width()))
//!     .collect();
//! ```
//!
//! ## Testing and Development
//!
//! The terminal system provides utilities for testing and development:
//!
//! ### Environment Variable Override
//! ```bash
//! # Override terminal size for testing
//! COLUMNS=100 LINES=30 cargo test
//! ```
//!
//! ### Predictable Behavior
//! When environment variables are set, the system uses those values consistently,
//! making testing and development predictable across different environments.

pub mod dimensions;

pub use dimensions::*;
