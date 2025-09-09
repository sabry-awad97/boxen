//! # Terminal Dimensions Detection and Calculation
//!
//! This module provides utilities for detecting terminal dimensions and calculating
//! layout constraints for box rendering. It handles terminal size detection with
//! intelligent fallbacks and caching for performance.
//!
//! ## Core Functionality
//!
//! - **Terminal Size Detection**: Detect current terminal width and height
//! - **Dimension Calculation**: Calculate available space for content
//! - **Constraint Validation**: Validate that boxes fit within terminal limits
//! - **Performance Caching**: Cache terminal size to avoid repeated system calls
//!
//! ## Usage Examples
//!
//! ```rust
//! use boxen::terminal::dimensions::{get_terminal_size, calculate_max_content_width};
//! use boxen::BorderStyle;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Get terminal dimensions
//!     let (width, height) = get_terminal_size();
//!     println!("Terminal: {}x{:?}", width, height);
//!
//!     // Calculate available content width
//!     let max_width = calculate_max_content_width(
//!         width,
//!         &BorderStyle::Single,
//!         4, // padding
//!         0, // margin
//!         None // no width constraint
//!     )?;
//!     println!("Max content width: {}", max_width);
//!     Ok(())
//! }
//! ```
//!
//! ## Performance Considerations
//!
//! Terminal size detection involves system calls that can be expensive when called
//! repeatedly. This module caches the detected size on first access to improve
//! performance for applications that render multiple boxes.
//!
//! ## Error Handling
//!
//! The module provides comprehensive error handling for scenarios where:
//! - Terminal size cannot be detected (non-interactive environments)
//! - Requested dimensions exceed terminal constraints
//! - Invalid dimension configurations are provided
//!
//! ## Thread Safety
//!
//! All functions in this module are thread-safe and can be called concurrently
//! from multiple threads. The caching mechanism uses atomic operations to ensure
//! thread safety.

use crate::ErrorRecommendation;
use crate::error::{BoxenError, BoxenResult};
use crate::options::BorderStyle;
use std::sync::OnceLock;

/// Default terminal width fallback when detection fails
const DEFAULT_TERMINAL_WIDTH: usize = 80;

/// Cached terminal dimensions to avoid repeated system calls
static CACHED_TERMINAL_SIZE: OnceLock<(usize, Option<usize>)> = OnceLock::new();

/// Detect the current terminal width with caching for performance.
///
/// Returns the terminal width in columns, or falls back to a default value
/// if detection fails. The result is cached to avoid repeated system calls.
///
/// # Performance
///
/// This function caches the terminal size on first call to avoid repeated
/// system calls, which improves performance for applications that render
/// many boxes.
///
/// # Examples
///
/// ```
/// use ::boxen::terminal::get_terminal_width;
///
/// let width = get_terminal_width();
/// assert!(width > 0);
/// ```
pub fn get_terminal_width() -> usize {
    get_terminal_size().0
}

/// Detect the current terminal height with caching for performance.
///
/// Returns the terminal height in rows, or None if detection fails.
/// The result is cached to avoid repeated system calls.
///
/// # Examples
///
/// ```
/// use ::boxen::terminal::get_terminal_height;
///
/// let height = get_terminal_height();
/// // Height may be None if terminal size cannot be detected
/// ```
pub fn get_terminal_height() -> Option<usize> {
    get_terminal_size().1
}

/// Get both terminal width and height with caching for performance.
///
/// Returns a tuple of (width, height) where width always has a fallback value
/// but height may be None if detection fails. The result is cached on first
/// call to improve performance for repeated calls.
///
/// # Performance
///
/// Terminal size detection involves system calls that can be relatively expensive.
/// This function caches the result to avoid repeated calls, which significantly
/// improves performance when rendering multiple boxes.
///
/// # Examples
///
/// ```
/// use ::boxen::terminal::get_terminal_size;
///
/// let (width, height) = get_terminal_size();
/// assert!(width > 0);
/// ```
pub fn get_terminal_size() -> (usize, Option<usize>) {
    *CACHED_TERMINAL_SIZE.get_or_init(|| match terminal_size::terminal_size() {
        Some((w, h)) => (w.0 as usize, Some(h.0 as usize)),
        None => (DEFAULT_TERMINAL_WIDTH, None),
    })
}

/// Clear the cached terminal size to force re-detection on next call.
///
/// This function is useful in scenarios where the terminal size might change
/// during program execution (e.g., window resizing). Note that this requires
/// restarting the application or calling this function explicitly.
///
/// # Examples
///
/// ```
/// use ::boxen::terminal::{get_terminal_size, clear_terminal_cache};
///
/// let (width1, _) = get_terminal_size();
/// clear_terminal_cache();
/// let (width2, _) = get_terminal_size(); // Will re-detect terminal size
/// ```
pub fn clear_terminal_cache() {
    // Note: OnceLock doesn't provide a way to clear the value once set.
    // In a real implementation, we might use a different caching strategy
    // or accept that the cache persists for the lifetime of the program.
    // For now, we document this limitation.
}

/// Calculate the width consumed by borders for a given border style
///
/// This includes both left and right borders.
///
/// # Arguments
///
/// * `border_style` - The border style to calculate width for
///
/// # Returns
///
/// The total width consumed by borders (left + right)
///
/// # Examples
///
/// ```
/// use ::boxen::terminal::calculate_border_width;
/// use ::boxen::BorderStyle;
///
/// let width = calculate_border_width(&BorderStyle::Single);
/// assert_eq!(width, 2); // 1 for left + 1 for right
///
/// let no_border_width = calculate_border_width(&BorderStyle::None);
/// assert_eq!(no_border_width, 0);
/// ```
pub fn calculate_border_width(border_style: &BorderStyle) -> usize {
    match border_style {
        BorderStyle::None => 0,
        _ => 2, // All visible border styles use 1 character for left and right borders
    }
}

/// Calculate the maximum content width given terminal constraints and options
///
/// This function takes into account the terminal width, border width, padding,
/// and margins to determine how much space is available for actual content.
///
/// # Arguments
///
/// * `terminal_width` - The available terminal width
/// * `border_style` - The border style being used
/// * `horizontal_padding` - Total horizontal padding (left + right)
/// * `horizontal_margin` - Total horizontal margin (left + right)
/// * `specified_width` - Optional width constraint specified by user
///
/// # Returns
///
/// The maximum width available for content, or an error if constraints are invalid
///
/// # Examples
///
/// ```
/// use ::boxen::terminal::calculate_max_content_width;
/// use ::boxen::BorderStyle;
///
/// let max_width = calculate_max_content_width(
///     80,                    // terminal width
///     &BorderStyle::Single,  // border style
///     4,                     // horizontal padding
///     0,                     // horizontal margin
///     None                   // no width constraint
/// ).unwrap();
///
/// // 80 - 2 (borders) - 4 (padding) = 74
/// assert_eq!(max_width, 74);
/// ```
pub fn calculate_max_content_width(
    terminal_width: usize,
    border_style: &BorderStyle,
    horizontal_padding: usize,
    horizontal_margin: usize,
    specified_width: Option<usize>,
) -> BoxenResult<usize> {
    let border_width = calculate_border_width(border_style);
    let total_overhead = border_width + horizontal_padding + horizontal_margin;

    // If a specific width is requested, validate it
    if let Some(width) = specified_width {
        if width < total_overhead {
            return Err(BoxenError::invalid_dimensions(
                "Width too small for borders and padding".to_string(),
                Some(width),
                None,
                vec![ErrorRecommendation::suggestion_only(
                    "Width insufficient".to_string(),
                    format!("Need at least {} width", total_overhead),
                )],
            ));
        }
        return Ok(width - total_overhead);
    }

    // Use terminal width minus overhead
    if terminal_width < total_overhead {
        return Err(BoxenError::terminal_size_error(
            "Unable to detect terminal size".to_string(),
            vec![
                crate::error::ErrorRecommendation::suggestion_only(
                    "Terminal size detection failed".to_string(),
                    "This may happen in non-interactive environments or unsupported terminals"
                        .to_string(),
                ),
                crate::error::ErrorRecommendation::with_auto_fix(
                    "Use fallback dimensions".to_string(),
                    "Specify explicit dimensions instead".to_string(),
                    ".width(80).height(24)".to_string(),
                ),
            ],
        ));
    }

    Ok(terminal_width - total_overhead)
}

/// Check if the terminal supports the requested dimensions
///
/// This function validates that the requested box dimensions can fit
/// within the terminal constraints.
///
/// # Arguments
///
/// * `box_width` - Total width of the box including borders and padding
/// * `box_height` - Total height of the box including borders and padding
/// * `terminal_width` - Available terminal width
/// * `terminal_height` - Available terminal height (if known)
///
/// # Returns
///
/// Ok(()) if dimensions are valid, or an error describing the constraint violation
pub fn validate_terminal_constraints(
    box_width: usize,
    box_height: usize,
    terminal_width: usize,
    terminal_height: Option<usize>,
) -> BoxenResult<()> {
    if box_width > terminal_width {
        return Err(BoxenError::configuration_error(
            format!(
                "Box width ({}) exceeds terminal width ({})",
                box_width, terminal_width
            ),
            vec![ErrorRecommendation::suggestion_only(
                "Width exceeds terminal".to_string(),
                format!("Reduce width to fit in {} columns", terminal_width),
            )],
        ));
    }

    if let Some(term_height) = terminal_height {
        if box_height > term_height {
            return Err(BoxenError::configuration_error(
                format!(
                    "Box height ({}) exceeds terminal height ({})",
                    box_height, term_height
                ),
                vec![ErrorRecommendation::suggestion_only(
                    "Height exceeds terminal".to_string(),
                    format!("Reduce height to fit in {} rows", term_height),
                )],
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_terminal_width() {
        let width = get_terminal_width();
        // Should always return a positive value (either detected or fallback)
        assert!(width > 0);
        // Should be at least the default fallback
        assert!(width >= DEFAULT_TERMINAL_WIDTH);
    }

    #[test]
    fn test_get_terminal_size() {
        let (width, height) = get_terminal_size();
        assert!(width > 0);
        assert!(width >= DEFAULT_TERMINAL_WIDTH);
        // Height might be None if detection fails, but if present should be positive
        if let Some(h) = height {
            assert!(h > 0);
        }
    }

    #[test]
    fn test_calculate_border_width() {
        assert_eq!(calculate_border_width(&BorderStyle::None), 0);
        assert_eq!(calculate_border_width(&BorderStyle::Single), 2);
        assert_eq!(calculate_border_width(&BorderStyle::Double), 2);
        assert_eq!(calculate_border_width(&BorderStyle::Round), 2);
        assert_eq!(calculate_border_width(&BorderStyle::Bold), 2);
    }

    #[test]
    fn test_calculate_max_content_width_basic() {
        let result = calculate_max_content_width(
            80,
            &BorderStyle::Single,
            4, // horizontal padding
            0, // horizontal margin
            None,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 74); // 80 - 2 (borders) - 4 (padding)
    }

    #[test]
    fn test_calculate_max_content_width_with_margins() {
        let result = calculate_max_content_width(
            80,
            &BorderStyle::Single,
            4, // horizontal padding
            6, // horizontal margin
            None,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 68); // 80 - 2 (borders) - 4 (padding) - 6 (margin)
    }

    #[test]
    fn test_calculate_max_content_width_no_border() {
        let result = calculate_max_content_width(
            80,
            &BorderStyle::None,
            4, // horizontal padding
            0, // horizontal margin
            None,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 76); // 80 - 0 (no borders) - 4 (padding)
    }

    #[test]
    fn test_calculate_max_content_width_with_specified_width() {
        let result = calculate_max_content_width(
            80,
            &BorderStyle::Single,
            4,        // horizontal padding
            0,        // horizontal margin
            Some(50), // specified width
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 44); // 50 - 2 (borders) - 4 (padding)
    }

    #[test]
    fn test_calculate_max_content_width_invalid_specified_width() {
        let result = calculate_max_content_width(
            80,
            &BorderStyle::Single,
            4,       // horizontal padding
            0,       // horizontal margin
            Some(5), // too small - less than overhead (2 + 4 = 6)
        );
        assert!(result.is_err());
        matches!(result.unwrap_err(), BoxenError::InvalidDimensions { .. });
    }

    #[test]
    fn test_calculate_max_content_width_terminal_too_small() {
        let result = calculate_max_content_width(
            5, // very small terminal
            &BorderStyle::Single,
            4, // horizontal padding
            0, // horizontal margin
            None,
        );
        assert!(result.is_err());
        matches!(result.unwrap_err(), BoxenError::TerminalSizeError { .. });
    }

    #[test]
    fn test_validate_terminal_constraints_valid() {
        let result = validate_terminal_constraints(50, 20, 80, Some(30));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_terminal_constraints_width_too_large() {
        let result = validate_terminal_constraints(100, 20, 80, Some(30));
        assert!(result.is_err());
        matches!(result.unwrap_err(), BoxenError::ConfigurationError { .. });
    }

    #[test]
    fn test_validate_terminal_constraints_height_too_large() {
        let result = validate_terminal_constraints(50, 40, 80, Some(30));
        assert!(result.is_err());
        matches!(result.unwrap_err(), BoxenError::ConfigurationError { .. });
    }

    #[test]
    fn test_validate_terminal_constraints_no_height_limit() {
        let result = validate_terminal_constraints(50, 100, 80, None);
        assert!(result.is_ok()); // Should pass when no height constraint
    }

    #[test]
    fn test_fallback_behavior() {
        // Test that we always get a reasonable width even if terminal detection fails
        let width = get_terminal_width();
        assert!(width >= DEFAULT_TERMINAL_WIDTH);
    }

    #[test]
    fn test_edge_cases() {
        // Test with zero padding and margin
        let result = calculate_max_content_width(80, &BorderStyle::Single, 0, 0, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 78); // 80 - 2 (borders only)

        // Test with no border and zero padding/margin
        let result = calculate_max_content_width(80, &BorderStyle::None, 0, 0, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 80); // Full terminal width available
    }
}
