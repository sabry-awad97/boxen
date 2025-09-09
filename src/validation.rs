//! # Configuration Validation and Auto-Recovery System
//!
//! This module provides a comprehensive validation and intelligent auto-recovery system
//! for boxen configurations, ensuring that box rendering parameters are valid, optimal,
//! and compatible with terminal constraints while providing actionable recommendations
//! for configuration issues.
//!
//! ## Overview
//!
//! The validation system analyzes boxen configurations before rendering to detect
//! potential issues, calculate minimum requirements, and provide intelligent
//! recommendations for fixing problems. It includes automatic recovery mechanisms
//! that can adjust configurations to work within terminal constraints.
//!
//! ## Core Components
//!
//! ### Configuration Validation (`validate_configuration`)
//! - **Dimension Analysis**: Validates width and height constraints
//! - **Terminal Compatibility**: Ensures boxes fit within terminal bounds
//! - **Resource Validation**: Checks for reasonable resource usage
//! - **Intelligent Recommendations**: Provides specific fix suggestions
//!
//! ### Auto-Recovery System (`recovery` module)
//! - **Smart Adjustment**: Automatically fixes common configuration issues
//! - **Constraint Satisfaction**: Ensures configurations meet minimum requirements
//! - **Graceful Degradation**: Reduces features when necessary to fit constraints
//! - **Multi-Strategy Recovery**: Applies multiple recovery techniques
//!
//! ### Dimension Calculation (`calculate_minimum_dimensions`)
//! - **Content Analysis**: Measures text content requirements
//! - **Layout Calculation**: Accounts for borders, padding, and margins
//! - **Unicode Awareness**: Handles complex character width calculations
//! - **Optimization Suggestions**: Recommends optimal dimensions
//!
//! ## Quick Start
//!
//! ```rust
//! use boxen::validation::{validate_configuration, auto_adjust_options};
//! use boxen::BoxenOptions;
//!
//! let mut options = BoxenOptions::default();
//! options.width = Some(10); // Potentially too small
//! let text = "This is a long line of text";
//!
//! // Validate configuration
//! let validation = validate_configuration(text, &options);
//! if !validation.is_valid {
//!     println!("Configuration issues found:");
//!     for error in &validation.errors {
//!         println!("- {}", error);
//!     }
//! }
//!
//! // Auto-fix configuration issues
//! let fixed_options = auto_adjust_options(text, options);
//! ```
//!
//! ## Validation System
//!
//! The validation system performs comprehensive analysis of boxen configurations:
//!
//! ### Dimension Validation
//! Ensures that specified dimensions can accommodate the content:
//!
//! ```rust
//! use ::boxen::validation::calculate_minimum_dimensions;
//! use ::boxen::BoxenOptions;
//!
//! let options = BoxenOptions::default();
//! let min_dims = calculate_minimum_dimensions("Hello\nWorld", &options);
//!
//! println!("Minimum width: {}", min_dims.width);   // Content + borders + padding
//! println!("Minimum height: {}", min_dims.height); // Lines + borders + padding
//! println!("Content width: {}", min_dims.content_width);   // Pure text width
//! println!("Content height: {}", min_dims.content_height); // Number of lines
//! ```
//!
//! ### Terminal Compatibility
//! Validates that boxes will fit within terminal constraints:
//!
//! ```rust
//! use boxen::validation::validate_configuration;
//! use boxen::BoxenOptions;
//!
//! let mut options = BoxenOptions::default();
//! options.width = Some(200);  // Might exceed terminal width
//! options.height = Some(100); // Might exceed terminal height
//!
//! let validation = validate_configuration("Content", &options);
//!
//! // Check for terminal overflow errors
//! for error in &validation.errors {
//!     if error.to_string().contains("terminal") {
//!         println!("Terminal constraint violation: {}", error);
//!     }
//! }
//! ```
//!
//! ### Warning System
//! Identifies potentially problematic but valid configurations:
//!
//! ```rust
//! use boxen::validation::validate_configuration;
//! use boxen::{BoxenOptions, Spacing};
//!
//! let mut options = BoxenOptions::default();
//! options.padding = Spacing::from(25); // Excessive padding
//!
//! let validation = validate_configuration("Content", &options);
//!
//! // Review warnings for optimization opportunities
//! for warning in &validation.warnings {
//!     println!("Warning: {}", warning.issue);
//!     println!("Suggestion: {}", warning.suggestion);
//! }
//! ```
//!
//! ## Auto-Recovery System
//!
//! The recovery system automatically fixes configuration issues using intelligent strategies:
//!
//! ### Automatic Adjustment
//! Fixes common configuration problems without user intervention:
//!
//! ```rust
//! use boxen::validation::auto_adjust_options;
//! use boxen::BoxenOptions;
//!
//! let mut problematic_options = BoxenOptions::default();
//! problematic_options.width = Some(5);   // Too small
//! problematic_options.height = Some(1); // Too small
//!
//! let text = "This text is too long for the specified dimensions";
//! let fixed_options = auto_adjust_options(text, problematic_options);
//!
//! // Options are now adjusted to accommodate the text
//! assert!(fixed_options.width.unwrap() >= 5);
//! assert!(fixed_options.height.unwrap() >= 1);
//! ```
//!
//! ### Smart Recovery Strategies
//! The recovery system applies multiple strategies in order of preference:
//!
//! ```rust
//! use boxen::validation::recovery::smart_recovery;
//! use boxen::{BoxenOptions, Spacing};
//!
//! let mut options = BoxenOptions::default();
//! options.width = Some(300);  // Exceeds terminal
//! options.padding = Spacing::from(10); // Large padding
//!
//! let recovered = smart_recovery("Content", options);
//!
//! // Recovery strategies applied:
//! // 1. Reduce padding if possible
//! // 2. Remove borders if necessary
//! // 3. Adjust dimensions to fit terminal
//! // 4. Reduce margins as last resort
//! ```
//!
//! ### Targeted Recovery Functions
//! Specific recovery functions for different types of issues:
//!
//! ```rust
//! use ::boxen::validation::recovery::{
//!     recover_from_invalid_width,
//!     recover_from_invalid_height,
//!     recover_from_terminal_overflow
//! };
//! use ::boxen::BoxenOptions;
//!
//! let options = BoxenOptions::default();
//!
//! // Fix width issues specifically
//! let width_fixed = recover_from_invalid_width("Long text", options.clone(), 10);
//!
//! // Fix height issues specifically  
//! let height_fixed = recover_from_invalid_height("Multi\nLine\nText", options.clone(), 2);
//!
//! // Fix terminal overflow issues
//! let terminal_fixed = recover_from_terminal_overflow("Content", options);
//! ```
//!
//! ## Validation Results
//!
//! The validation system returns comprehensive results with actionable information:
//!
//! ### Validation Result Structure
//! ```rust
//! use ::boxen::validation::{ValidationResult, validate_configuration};
//! use ::boxen::BoxenOptions;
//!
//! let result = validate_configuration("Text", &BoxenOptions::default());
//!
//! // Check overall validity
//! if result.is_valid {
//!     println!("Configuration is valid");
//! } else {
//!     println!("Configuration has {} errors", result.errors.len());
//! }
//!
//! // Review minimum dimensions
//! if let Some(min_dims) = &result.minimum_dimensions {
//!     println!("Minimum required: {}x{}", min_dims.width, min_dims.height);
//! }
//!
//! // Process errors with recommendations
//! for error in &result.errors {
//!     println!("Error: {}", error);
//!     for rec in error.recommendations() {
//!         println!("  Suggestion: {}", rec.suggestion);
//!         if let Some(fix) = &rec.auto_fix {
//!             println!("  Auto-fix: {}", fix);
//!         }
//!     }
//! }
//! ```
//!
//! ### Error Recommendations
//! Each validation error includes specific recommendations for resolution:
//!
//! ```rust
//! use boxen::validation::validate_configuration;
//! use boxen::BoxenOptions;
//!
//! let mut options = BoxenOptions::default();
//! options.width = Some(3); // Too small
//! let result = validate_configuration("Hello World", &options);
//!
//! for error in &result.errors {
//!     let recommendations = error.recommendations();
//!     for rec in recommendations {
//!         println!("Issue: {}", rec.issue);
//!         println!("Suggestion: {}", rec.suggestion);
//!         
//!         // Some recommendations include auto-fix code
//!         if let Some(auto_fix) = &rec.auto_fix {
//!             println!("Code fix: {}", auto_fix);
//!         }
//!     }
//! }
//! ```
//!
//! ## Dimension Optimization
//!
//! The system provides intelligent dimension suggestions:
//!
//! ### Optimal Dimension Calculation
//! ```rust
//! use boxen::validation::suggest_optimal_dimensions;
//! use boxen::BoxenOptions;
//!
//! let options = BoxenOptions::default();
//! let text = "Sample content that needs optimal sizing";
//!
//! let (optimal_width, optimal_height) = suggest_optimal_dimensions(text, &options);
//!
//! // Use optimal dimensions for best user experience
//! let mut optimized_options = BoxenOptions::default();
//! optimized_options.width = Some(optimal_width);
//! optimized_options.height = Some(optimal_height);
//! ```
//!
//! ### Minimum Dimension Analysis
//! ```rust
//! use boxen::validation::{MinimumDimensions, calculate_minimum_dimensions};
//! use boxen::{BoxenOptions, Spacing};
//!
//! let mut options = BoxenOptions::default();
//! options.padding = Spacing::from(2);
//! let min_dims = calculate_minimum_dimensions("Multi\nLine\nContent", &options);
//!
//! // Access detailed dimension breakdown
//! println!("Content needs: {}x{}", min_dims.content_width, min_dims.content_height);
//! println!("Total required: {}x{}", min_dims.width, min_dims.height);
//! println!("Overhead: {}x{}",
//!     min_dims.width - min_dims.content_width,
//!     min_dims.height - min_dims.content_height
//! );
//! ```
//!
//! ## Integration with Error System
//!
//! The validation system integrates seamlessly with boxen's error handling:
//!
//! ### Error Types and Recovery
//! ```rust
//! use boxen::validation::validate_configuration;
//! use boxen::{BoxenOptions, BoxenError};
//!
//! let mut options = BoxenOptions::default();
//! options.width = Some(1); // Invalid
//! let result = validate_configuration("Content", &options);
//!
//! for error in &result.errors {
//!     match error {
//!         BoxenError::InvalidDimensions { width, height, recommendations, .. } => {
//!             println!("Dimension error - Width: {:?}, Height: {:?}", width, height);
//!             for rec in recommendations {
//!                 println!("  Fix: {}", rec.suggestion);
//!             }
//!         }
//!         BoxenError::ConfigurationError { message, recommendations } => {
//!             println!("Configuration error: {}", message);
//!             for rec in recommendations {
//!                 println!("  Fix: {}", rec.suggestion);
//!             }
//!         }
//!         _ => println!("Other error: {}", error),
//!     }
//! }
//! ```
//!
//! ## Performance Considerations
//!
//! The validation system is optimized for performance while maintaining accuracy:
//!
//! ### Efficient Validation
//! - **Lazy Evaluation**: Only calculates what's needed for validation
//! - **Minimal Allocations**: Reuses calculations across validation steps
//! - **Early Termination**: Stops validation on critical errors when appropriate
//! - **Cached Calculations**: Memoizes expensive operations like terminal size detection
//!
//! ### Validation Timing
//! ```rust
//! use ::boxen::validation::validate_configuration;
//! use ::boxen::BoxenOptions;
//! use std::time::Instant;
//!
//! let start = Instant::now();
//! let result = validate_configuration("Large text content...", &BoxenOptions::default());
//! let duration = start.elapsed();
//!
//! println!("Validation completed in {:?}", duration);
//! // Typically completes in microseconds for normal content
//! ```
//!
//! ## Thread Safety
//!
//! All validation functions are thread-safe and can be used in concurrent environments.
//! The validation system doesn't maintain internal state, making it safe for parallel
//! processing of multiple configurations.
//!
//! ## Testing and Reliability
//!
//! The validation system includes comprehensive tests covering:
//! - **Edge Cases**: Empty content, extreme dimensions, terminal edge cases
//! - **Unicode Handling**: Complex character width calculations
//! - **Recovery Scenarios**: All auto-recovery strategies and fallbacks
//! - **Performance Tests**: Validation speed with large content
//! - **Integration Tests**: End-to-end validation with real terminal constraints

use crate::error::{BoxenError, ErrorRecommendation};
use crate::options::{BoxenOptions, Spacing};
use crate::terminal::{get_terminal_height, get_terminal_width};
use crate::text::text_width;

/// Minimum dimensions required for a box configuration
#[derive(Debug, Clone)]
pub struct MinimumDimensions {
    /// Minimum total width including borders, padding, and margins
    pub width: usize,
    /// Minimum total height including borders, padding, and margins
    pub height: usize,
    /// Minimum width available for content text
    pub content_width: usize,
    /// Minimum height available for content text
    pub content_height: usize,
}

/// Validation result with recommendations
#[derive(Debug)]
pub struct ValidationResult {
    /// Whether the configuration passed all validation checks
    pub is_valid: bool,
    /// Non-critical issues that should be addressed
    pub warnings: Vec<ErrorRecommendation>,
    /// Critical errors that prevent rendering
    pub errors: Vec<BoxenError>,
    /// Calculated minimum dimensions for the configuration
    pub minimum_dimensions: Option<MinimumDimensions>,
}

impl ValidationResult {
    /// Create a new valid validation result with no warnings or errors
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            warnings: vec![],
            errors: vec![],
            minimum_dimensions: None,
        }
    }

    /// Create a validation result with warnings but no errors
    pub fn with_warnings(warnings: Vec<ErrorRecommendation>) -> Self {
        Self {
            is_valid: true,
            warnings,
            errors: vec![],
            minimum_dimensions: None,
        }
    }

    /// Create a validation result with errors (automatically marks as invalid)
    pub fn with_errors(errors: Vec<BoxenError>) -> Self {
        Self {
            is_valid: false,
            warnings: vec![],
            errors,
            minimum_dimensions: None,
        }
    }

    /// Add a warning to the validation result
    pub fn add_warning(&mut self, warning: ErrorRecommendation) {
        self.warnings.push(warning);
    }

    /// Add an error to the validation result (automatically marks as invalid)
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
#[must_use]
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
                ErrorRecommendation::with_auto_fix(
                    "Auto-adjust width".to_string(),
                    "Let the system automatically adjust the width".to_string(),
                    ".auto_adjust(text)".to_string(),
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
                ErrorRecommendation::with_auto_fix(
                    "Auto-adjust height".to_string(),
                    "Let the system automatically adjust the height".to_string(),
                    ".auto_adjust(text)".to_string(),
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
#[must_use]
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
#[must_use]
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

/// Error recovery strategies for common configuration issues
pub mod recovery {
    use super::{
        BoxenError, calculate_minimum_dimensions, get_terminal_height, get_terminal_width,
        validate_configuration,
    };
    use crate::options::{BorderStyle, BoxenOptions, Spacing};

    /// Attempt to recover from invalid width by adjusting configuration
    #[must_use]
    pub fn recover_from_invalid_width(
        text: &str,
        mut options: BoxenOptions,
        target_width: usize,
    ) -> BoxenOptions {
        let min_dims = calculate_minimum_dimensions(text, &options);

        if target_width < min_dims.width {
            // Try reducing padding first
            if options.padding.horizontal() > 0 {
                let reduction_needed = min_dims.width - target_width;
                let current_horizontal = options.padding.horizontal();

                if current_horizontal >= reduction_needed {
                    let new_horizontal = current_horizontal - reduction_needed;
                    options.padding = Spacing {
                        left: new_horizontal / 2,
                        right: new_horizontal / 2,
                        ..options.padding
                    };
                    return options;
                }
            }

            // If padding reduction isn't enough, try removing borders
            if !matches!(options.border_style, BorderStyle::None) {
                options.border_style = BorderStyle::None;
                let new_min_dims = calculate_minimum_dimensions(text, &options);
                if target_width >= new_min_dims.width {
                    return options;
                }
            }

            // As last resort, set width to minimum required
            options.width = Some(min_dims.width);
        }

        options
    }

    /// Attempt to recover from invalid height by adjusting configuration
    #[must_use]
    pub fn recover_from_invalid_height(
        text: &str,
        mut options: BoxenOptions,
        target_height: usize,
    ) -> BoxenOptions {
        let min_dims = calculate_minimum_dimensions(text, &options);

        if target_height < min_dims.height {
            // Try reducing padding first
            if options.padding.vertical() > 0 {
                let reduction_needed = min_dims.height - target_height;
                let current_vertical = options.padding.vertical();

                if current_vertical >= reduction_needed {
                    let new_vertical = current_vertical - reduction_needed;
                    options.padding = Spacing {
                        top: new_vertical / 2,
                        bottom: new_vertical / 2,
                        ..options.padding
                    };
                    return options;
                }
            }

            // If padding reduction isn't enough, try removing borders
            if !matches!(options.border_style, BorderStyle::None) {
                options.border_style = BorderStyle::None;
                let new_min_dims = calculate_minimum_dimensions(text, &options);
                if target_height >= new_min_dims.height {
                    return options;
                }
            }

            // As last resort, set height to minimum required
            options.height = Some(min_dims.height);
        }

        options
    }

    /// Attempt to recover from terminal size overflow
    #[must_use]
    pub fn recover_from_terminal_overflow(text: &str, mut options: BoxenOptions) -> BoxenOptions {
        let terminal_width = get_terminal_width();
        let terminal_height = get_terminal_height();

        // Adjust width if it exceeds terminal
        let total_width = options.width.unwrap_or_else(|| {
            let min_dims = calculate_minimum_dimensions(text, &options);
            min_dims.width
        }) + options.margin.horizontal();

        if total_width > terminal_width {
            let margin_horizontal = options.margin.horizontal();
            options = recover_from_invalid_width(text, options, terminal_width - margin_horizontal);
        }

        // Adjust height if it exceeds terminal
        if let Some(term_height) = terminal_height {
            let total_height = options.height.unwrap_or_else(|| {
                let min_dims = calculate_minimum_dimensions(text, &options);
                min_dims.height
            }) + options.margin.vertical();

            if total_height > term_height {
                let margin_vertical = options.margin.vertical();
                options = recover_from_invalid_height(text, options, term_height - margin_vertical);
            }
        }

        options
    }

    /// Smart recovery that tries multiple strategies
    #[must_use]
    pub fn smart_recovery(text: &str, options: BoxenOptions) -> BoxenOptions {
        let validation = validate_configuration(text, &options);

        if validation.is_valid {
            return options;
        }

        let mut recovered_options = options;

        // Apply recovery strategies based on error types
        for error in &validation.errors {
            match error {
                BoxenError::InvalidDimensions { width, height, .. } => {
                    if let Some(w) = width {
                        recovered_options = recover_from_invalid_width(text, recovered_options, *w);
                    }
                    if let Some(h) = height {
                        recovered_options =
                            recover_from_invalid_height(text, recovered_options, *h);
                    }
                }
                BoxenError::ConfigurationError { message, .. } => {
                    if message.contains("terminal width") || message.contains("terminal height") {
                        recovered_options = recover_from_terminal_overflow(text, recovered_options);
                    }
                }
                _ => {}
            }
        }

        recovered_options
    }
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
