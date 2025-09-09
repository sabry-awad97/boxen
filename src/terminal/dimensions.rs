use crate::ErrorRecommendation;
use crate::error::{BoxenError, BoxenResult};
use crate::options::BorderStyle;

/// Default terminal width fallback when detection fails
const DEFAULT_TERMINAL_WIDTH: usize = 80;

/// Detect the current terminal width
///
/// Returns the terminal width in columns, or falls back to a default value
/// if detection fails.
///
/// # Examples
///
/// ```
/// use boxen::terminal::get_terminal_width;
///
/// let width = get_terminal_width();
/// assert!(width > 0);
/// ```
pub fn get_terminal_width() -> usize {
    terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(DEFAULT_TERMINAL_WIDTH)
}

/// Detect the current terminal height
///
/// Returns the terminal height in rows, or None if detection fails.
///
/// # Examples
///
/// ```
/// use boxen::terminal::get_terminal_height;
///
/// let height = get_terminal_height();
/// // Height may be None if terminal size cannot be detected
/// ```
pub fn get_terminal_height() -> Option<usize> {
    terminal_size::terminal_size().map(|(_, h)| h.0 as usize)
}

/// Get both terminal width and height
///
/// Returns a tuple of (width, height) where width always has a fallback value
/// but height may be None if detection fails.
///
/// # Examples
///
/// ```
/// use boxen::terminal::get_terminal_size;
///
/// let (width, height) = get_terminal_size();
/// assert!(width > 0);
/// ```
pub fn get_terminal_size() -> (usize, Option<usize>) {
    match terminal_size::terminal_size() {
        Some((w, h)) => (w.0 as usize, Some(h.0 as usize)),
        None => (DEFAULT_TERMINAL_WIDTH, None),
    }
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
/// use boxen::terminal::calculate_border_width;
/// use boxen::BorderStyle;
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
/// use boxen::terminal::calculate_max_content_width;
/// use boxen::BorderStyle;
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
        return Err(BoxenError::TerminalSizeError);
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
        matches!(result.unwrap_err(), BoxenError::TerminalSizeError);
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
