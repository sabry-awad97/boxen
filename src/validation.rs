/// Intelligent validation and recommendation system for boxen configurations
use crate::error::{BoxenError, ErrorRecommendation};
use crate::options::{BoxenOptions, Spacing};
use crate::terminal::{get_terminal_height, get_terminal_width};
use crate::text::text_width;

/// Minimum dimensions required for a box configuration
#[derive(Debug, Clone)]
pub struct MinimumDimensions {
    pub width: usize,
    pub height: usize,
    pub content_width: usize,
    pub content_height: usize,
}

/// Validation result with recommendations
#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub warnings: Vec<ErrorRecommendation>,
    pub errors: Vec<BoxenError>,
    pub minimum_dimensions: Option<MinimumDimensions>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            warnings: vec![],
            errors: vec![],
            minimum_dimensions: None,
        }
    }

    pub fn with_warnings(warnings: Vec<ErrorRecommendation>) -> Self {
        Self {
            is_valid: true,
            warnings,
            errors: vec![],
            minimum_dimensions: None,
        }
    }

    pub fn with_errors(errors: Vec<BoxenError>) -> Self {
        Self {
            is_valid: false,
            warnings: vec![],
            errors,
            minimum_dimensions: None,
        }
    }

    pub fn add_warning(&mut self, warning: ErrorRecommendation) {
        self.warnings.push(warning);
    }

    pub fn add_error(&mut self, error: BoxenError) {
        self.is_valid = false;
        self.errors.push(error);
    }
}

/// Calculate minimum dimensions required for given text and options
pub fn calculate_minimum_dimensions(text: &str, options: &BoxenOptions) -> MinimumDimensions {
    let lines: Vec<&str> = text.lines().collect();
    let content_height = if lines.is_empty() { 1 } else { lines.len() };
    let content_width = lines
        .iter()
        .map(|line| text_width(line))
        .max()
        .unwrap_or(0)
        .max(1); // Minimum 1 character width

    let border_width = if options.border_style.is_visible() {
        2
    } else {
        0
    };
    let total_padding_width = options.padding.horizontal();
    let total_padding_height = options.padding.vertical();

    let min_width = content_width + border_width + total_padding_width;
    let min_height = content_height + border_width + total_padding_height;

    MinimumDimensions {
        width: min_width,
        height: min_height,
        content_width,
        content_height,
    }
}

/// Validate a boxen configuration and provide intelligent recommendations
pub fn validate_configuration(text: &str, options: &BoxenOptions) -> ValidationResult {
    let mut result = ValidationResult::valid();
    let min_dims = calculate_minimum_dimensions(text, options);
    result.minimum_dimensions = Some(min_dims.clone());

    // Check width constraints
    if let Some(specified_width) = options.width {
        if specified_width < min_dims.width {
            let recommendations = vec![
                ErrorRecommendation::with_auto_fix(
                    "Specified width is too small".to_string(),
                    format!(
                        "Increase width to at least {} (current: {})",
                        min_dims.width, specified_width
                    ),
                    format!(".width({})", min_dims.width),
                ),
                ErrorRecommendation::suggestion_only(
                    "Alternative: Reduce padding".to_string(),
                    format!(
                        "Current padding adds {} to width. Consider reducing padding.",
                        options.padding.horizontal()
                    ),
                ),
                ErrorRecommendation::suggestion_only(
                    "Alternative: Use no border".to_string(),
                    "Set border_style to BorderStyle::None to save 2 characters width".to_string(),
                ),
            ];

            result.add_error(BoxenError::invalid_dimensions(
                format!(
                    "Width {} is too small. Minimum required: {}",
                    specified_width, min_dims.width
                ),
                Some(specified_width),
                None,
                recommendations,
            ));
        }
    }

    // Check height constraints
    if let Some(specified_height) = options.height {
        if specified_height < min_dims.height {
            let recommendations = vec![
                ErrorRecommendation::with_auto_fix(
                    "Specified height is too small".to_string(),
                    format!(
                        "Increase height to at least {} (current: {})",
                        min_dims.height, specified_height
                    ),
                    format!(".height({})", min_dims.height),
                ),
                ErrorRecommendation::suggestion_only(
                    "Alternative: Reduce padding".to_string(),
                    format!(
                        "Current padding adds {} to height. Consider reducing padding.",
                        options.padding.vertical()
                    ),
                ),
                ErrorRecommendation::suggestion_only(
                    "Alternative: Use no border".to_string(),
                    "Set border_style to BorderStyle::None to save 2 characters height".to_string(),
                ),
            ];

            result.add_error(BoxenError::invalid_dimensions(
                format!(
                    "Height {} is too small. Minimum required: {}",
                    specified_height, min_dims.height
                ),
                None,
                Some(specified_height),
                recommendations,
            ));
        }
    }

    // Check terminal size constraints
    let terminal_width: usize = get_terminal_width();
    let terminal_height: Option<usize> = get_terminal_height();

    let total_width = options.width.unwrap_or(min_dims.width) + options.margin.horizontal();
    let total_height = options.height.unwrap_or(min_dims.height) + options.margin.vertical();

    if total_width > terminal_width {
        let recommendations = vec![
            ErrorRecommendation::with_auto_fix(
                "Box exceeds terminal width".to_string(),
                format!(
                    "Reduce width or margins. Current total: {}, terminal: {}",
                    total_width, terminal_width
                ),
                format!(
                    ".width({})",
                    terminal_width.saturating_sub(options.margin.horizontal() + 4)
                ),
            ),
            ErrorRecommendation::suggestion_only(
                "Alternative: Reduce margins".to_string(),
                format!(
                    "Current margins add {} to width",
                    options.margin.horizontal()
                ),
            ),
        ];

        result.add_error(BoxenError::configuration_error(
            format!(
                "Box width ({}) exceeds terminal width ({})",
                total_width, terminal_width
            ),
            recommendations,
        ));
    }

    if let Some(term_height) = terminal_height {
        if total_height > term_height {
            let recommendations = vec![
                ErrorRecommendation::with_auto_fix(
                    "Box exceeds terminal height".to_string(),
                    format!(
                        "Reduce height or margins. Current total: {}, terminal: {}",
                        total_height, term_height
                    ),
                    format!(
                        ".height({})",
                        term_height.saturating_sub(options.margin.vertical() + 4)
                    ),
                ),
                ErrorRecommendation::suggestion_only(
                    "Alternative: Reduce margins".to_string(),
                    format!(
                        "Current margins add {} to height",
                        options.margin.vertical()
                    ),
                ),
            ];

            result.add_error(BoxenError::configuration_error(
                format!(
                    "Box height ({}) exceeds terminal height ({})",
                    total_height, term_height
                ),
                recommendations,
            ));
        }
    }

    // Add warnings for potentially problematic configurations
    if options.padding.horizontal() > 20 {
        result.add_warning(ErrorRecommendation::suggestion_only(
            "Large horizontal padding".to_string(),
            format!(
                "Horizontal padding of {} might be excessive",
                options.padding.horizontal()
            ),
        ));
    }

    if options.padding.vertical() > 10 {
        result.add_warning(ErrorRecommendation::suggestion_only(
            "Large vertical padding".to_string(),
            format!(
                "Vertical padding of {} might be excessive",
                options.padding.vertical()
            ),
        ));
    }

    if text.lines().count() > 50 {
        result.add_warning(ErrorRecommendation::suggestion_only(
            "Large text content".to_string(),
            "Text has many lines, consider using height constraints or text wrapping".to_string(),
        ));
    }

    result
}

/// Suggest optimal dimensions for given text and constraints
pub fn suggest_optimal_dimensions(text: &str, options: &BoxenOptions) -> (usize, usize) {
    let min_dims = calculate_minimum_dimensions(text, options);
    let terminal_width = get_terminal_width();
    let terminal_height = get_terminal_height();

    // Calculate optimal width (leave some margin for readability)
    let max_usable_width = terminal_width.saturating_sub(options.margin.horizontal() + 4);
    let optimal_width = min_dims.width.min(max_usable_width).max(20); // At least 20 chars wide

    // Calculate optimal height (leave some margin)
    let max_usable_height =
        terminal_height.map_or(50, |h| h.saturating_sub(options.margin.vertical() + 4));
    let optimal_height = min_dims.height.min(max_usable_height).max(3); // At least 3 lines high

    (optimal_width, optimal_height)
}

/// Auto-adjust options to fix common configuration issues
pub fn auto_adjust_options(text: &str, mut options: BoxenOptions) -> BoxenOptions {
    let validation = validate_configuration(text, &options);

    if !validation.is_valid {
        let min_dims = calculate_minimum_dimensions(text, &options);
        let terminal_width = get_terminal_width();
        let terminal_height = get_terminal_height();

        // Auto-adjust width if too small
        if let Some(width) = options.width {
            if width < min_dims.width {
                options.width = Some(min_dims.width);
            }
        }

        // Auto-adjust height if too small
        if let Some(height) = options.height {
            if height < min_dims.height {
                options.height = Some(min_dims.height);
            }
        }

        // Auto-adjust if exceeds terminal size
        let total_width = options.width.unwrap_or(min_dims.width) + options.margin.horizontal();
        if total_width > terminal_width {
            let max_width = terminal_width.saturating_sub(options.margin.horizontal());
            if max_width >= min_dims.width {
                options.width = Some(max_width);
            } else {
                // Reduce margins if necessary
                let required_margin_reduction = total_width - terminal_width;
                let new_horizontal_margin = options
                    .margin
                    .horizontal()
                    .saturating_sub(required_margin_reduction);
                options.margin = Spacing {
                    left: new_horizontal_margin / 2,
                    right: new_horizontal_margin / 2,
                    ..options.margin
                };
            }
        }

        let total_height = options.height.unwrap_or(min_dims.height) + options.margin.vertical();
        if let Some(term_height) = terminal_height {
            if total_height > term_height {
                let max_height = term_height.saturating_sub(options.margin.vertical());
                if max_height >= min_dims.height {
                    options.height = Some(max_height);
                } else {
                    // Reduce margins if necessary
                    let required_margin_reduction = total_height - term_height;
                    let new_vertical_margin = options
                        .margin
                        .vertical()
                        .saturating_sub(required_margin_reduction);
                    options.margin = Spacing {
                        top: new_vertical_margin / 2,
                        bottom: new_vertical_margin / 2,
                        ..options.margin
                    };
                }
            }
        }
    }

    options
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_minimum_dimensions() {
        let options = BoxenOptions::default();
        let min_dims = calculate_minimum_dimensions("Hello", &options);

        assert_eq!(min_dims.content_width, 5); // "Hello" is 5 chars
        assert_eq!(min_dims.content_height, 1); // Single line
        assert_eq!(min_dims.width, 7); // 5 + 2 border
        assert_eq!(min_dims.height, 3); // 1 + 2 border
    }

    #[test]
    fn test_calculate_minimum_dimensions_with_padding() {
        let options = BoxenOptions {
            padding: Spacing::from(1), // 3 horizontal, 1 vertical each side
            ..Default::default()
        };
        let min_dims = calculate_minimum_dimensions("Hello", &options);

        assert_eq!(min_dims.width, 13); // 5 + 2 border + 6 padding
        assert_eq!(min_dims.height, 5); // 1 + 2 border + 2 padding
    }

    #[test]
    fn test_validate_configuration_valid() {
        let options = BoxenOptions::default();
        let result = validate_configuration("Hello", &options);

        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_configuration_width_too_small() {
        let options = BoxenOptions {
            width: Some(5), // Too small for "Hello" + borders
            ..Default::default()
        };
        let result = validate_configuration("Hello", &options);

        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);

        let recommendations = result.errors[0].recommendations();
        assert!(!recommendations.is_empty());
        assert!(recommendations[0].auto_fix.is_some());
    }

    #[test]
    fn test_suggest_optimal_dimensions() {
        let options = BoxenOptions::default();
        let (width, height) = suggest_optimal_dimensions("Hello\nWorld", &options);

        assert!(width >= 7); // At least minimum required
        assert!(height >= 4); // At least minimum required
        assert!(width <= get_terminal_width()); // Not exceeding terminal
        if let Some(term_height) = get_terminal_height() {
            assert!(height <= term_height); // Not exceeding terminal
        }
    }

    #[test]
    fn test_auto_adjust_options() {
        let original_options = BoxenOptions {
            width: Some(5),  // Too small
            height: Some(2), // Too small
            ..Default::default()
        };

        let adjusted = auto_adjust_options("Hello\nWorld", original_options);

        // Should be adjusted to minimum required
        assert!(adjusted.width.unwrap() >= 7); // Minimum for "Hello" + borders
        assert!(adjusted.height.unwrap() >= 4); // Minimum for 2 lines + borders
    }

    #[test]
    fn test_validation_warnings() {
        let options = BoxenOptions {
            padding: Spacing {
                top: 15,
                right: 25,
                bottom: 15,
                left: 25,
            },
            ..Default::default()
        };

        let result = validate_configuration("Hello", &options);

        // Should have warnings about excessive padding
        assert!(!result.warnings.is_empty());
    }
}
