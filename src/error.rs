/// Error types for the boxen library
use thiserror::Error;

/// Recommendation for fixing a configuration error
#[derive(Debug, Clone)]
pub struct ErrorRecommendation {
    pub issue: String,
    pub suggestion: String,
    pub auto_fix: Option<String>,
}

/// Errors that can occur when creating or rendering boxes
#[derive(Debug, Error)]
pub enum BoxenError {
    #[error("Invalid border style: {message}")]
    InvalidBorderStyle {
        message: String,
        recommendations: Vec<ErrorRecommendation>,
    },

    #[error("Invalid color specification: {message}")]
    InvalidColor {
        message: String,
        color_value: String,
        recommendations: Vec<ErrorRecommendation>,
    },

    #[error("Invalid dimensions: {message}")]
    InvalidDimensions {
        message: String,
        width: Option<usize>,
        height: Option<usize>,
        recommendations: Vec<ErrorRecommendation>,
    },

    #[error("Terminal size detection failed: {message}")]
    TerminalSizeError {
        message: String,
        recommendations: Vec<ErrorRecommendation>,
    },

    #[error("Text processing error: {message}")]
    TextProcessingError {
        message: String,
        recommendations: Vec<ErrorRecommendation>,
    },

    #[error("Configuration conflict: {message}")]
    ConfigurationError {
        message: String,
        recommendations: Vec<ErrorRecommendation>,
    },

    #[error("Input validation error: {message}")]
    InputValidationError {
        message: String,
        field: String,
        value: String,
        recommendations: Vec<ErrorRecommendation>,
    },

    #[error("Rendering error: {message}")]
    RenderingError {
        message: String,
        recommendations: Vec<ErrorRecommendation>,
    },
}

impl BoxenError {
    /// Create an InvalidDimensions error with intelligent recommendations
    pub fn invalid_dimensions(
        message: String,
        width: Option<usize>,
        height: Option<usize>,
        recommendations: Vec<ErrorRecommendation>,
    ) -> Self {
        Self::InvalidDimensions {
            message,
            width,
            height,
            recommendations,
        }
    }

    /// Create a ConfigurationError with recommendations
    pub fn configuration_error(message: String, recommendations: Vec<ErrorRecommendation>) -> Self {
        Self::ConfigurationError {
            message,
            recommendations,
        }
    }

    /// Create an InvalidColor error with recommendations
    pub fn invalid_color(
        message: String,
        color_value: String,
        recommendations: Vec<ErrorRecommendation>,
    ) -> Self {
        Self::InvalidColor {
            message,
            color_value,
            recommendations,
        }
    }

    /// Create an InvalidBorderStyle error with recommendations
    pub fn invalid_border_style(
        message: String,
        recommendations: Vec<ErrorRecommendation>,
    ) -> Self {
        Self::InvalidBorderStyle {
            message,
            recommendations,
        }
    }

    /// Create a TerminalSizeError with recommendations
    pub fn terminal_size_error(message: String, recommendations: Vec<ErrorRecommendation>) -> Self {
        Self::TerminalSizeError {
            message,
            recommendations,
        }
    }

    /// Create a TextProcessingError with recommendations
    pub fn text_processing_error(
        message: String,
        recommendations: Vec<ErrorRecommendation>,
    ) -> Self {
        Self::TextProcessingError {
            message,
            recommendations,
        }
    }

    /// Create an InputValidationError with recommendations
    pub fn input_validation_error(
        message: String,
        field: String,
        value: String,
        recommendations: Vec<ErrorRecommendation>,
    ) -> Self {
        Self::InputValidationError {
            message,
            field,
            value,
            recommendations,
        }
    }

    /// Create a RenderingError with recommendations
    pub fn rendering_error(message: String, recommendations: Vec<ErrorRecommendation>) -> Self {
        Self::RenderingError {
            message,
            recommendations,
        }
    }

    /// Get recommendations for fixing this error
    pub fn recommendations(&self) -> Vec<ErrorRecommendation> {
        match self {
            Self::InvalidBorderStyle {
                recommendations, ..
            } => recommendations.clone(),
            Self::InvalidColor {
                recommendations, ..
            } => recommendations.clone(),
            Self::InvalidDimensions {
                recommendations, ..
            } => recommendations.clone(),
            Self::TerminalSizeError {
                recommendations, ..
            } => recommendations.clone(),
            Self::TextProcessingError {
                recommendations, ..
            } => recommendations.clone(),
            Self::ConfigurationError {
                recommendations, ..
            } => recommendations.clone(),
            Self::InputValidationError {
                recommendations, ..
            } => recommendations.clone(),
            Self::RenderingError {
                recommendations, ..
            } => recommendations.clone(),
        }
    }

    /// Get a user-friendly error message with suggestions
    pub fn detailed_message(&self) -> String {
        let base_message = self.to_string();
        let recommendations = self.recommendations();

        if recommendations.is_empty() {
            return base_message;
        }

        let mut message = format!("{}\n\nSuggestions:", base_message);
        for (i, rec) in recommendations.iter().enumerate() {
            message.push_str(&format!("\n{}. {}: {}", i + 1, rec.issue, rec.suggestion));
            if let Some(auto_fix) = &rec.auto_fix {
                message.push_str(&format!("\n   Auto-fix: {}", auto_fix));
            }
        }
        message
    }
}

impl ErrorRecommendation {
    /// Create a new recommendation
    pub fn new(issue: String, suggestion: String, auto_fix: Option<String>) -> Self {
        Self {
            issue,
            suggestion,
            auto_fix,
        }
    }

    /// Create a recommendation with auto-fix
    pub fn with_auto_fix(issue: String, suggestion: String, auto_fix: String) -> Self {
        Self {
            issue,
            suggestion,
            auto_fix: Some(auto_fix),
        }
    }

    /// Create a recommendation without auto-fix
    pub fn suggestion_only(issue: String, suggestion: String) -> Self {
        Self {
            issue,
            suggestion,
            auto_fix: None,
        }
    }
}

/// Result type alias for boxen operations
pub type BoxenResult<T> = Result<T, BoxenError>;

/// Input validation utilities
pub mod validation {
    use super::*;

    /// Validate text input
    pub fn validate_text_input(text: &str) -> BoxenResult<()> {
        // Check for extremely long text that might cause performance issues
        if text.len() > 1_000_000 {
            return Err(BoxenError::input_validation_error(
                "Text input is too large and may cause performance issues".to_string(),
                "text".to_string(),
                format!("{} characters", text.len()),
                vec![
                    ErrorRecommendation::suggestion_only(
                        "Text too large".to_string(),
                        "Consider splitting large text into smaller chunks or using height constraints".to_string(),
                    ),
                    ErrorRecommendation::with_auto_fix(
                        "Use height constraint".to_string(),
                        "Limit the visible height to prevent rendering issues".to_string(),
                        ".height(50)".to_string(),
                    ),
                ],
            ));
        }

        // Check for excessive line count
        let line_count = text.lines().count();
        if line_count > 1000 {
            return Err(BoxenError::input_validation_error(
                "Text has too many lines and may cause performance issues".to_string(),
                "text".to_string(),
                format!("{} lines", line_count),
                vec![
                    ErrorRecommendation::suggestion_only(
                        "Too many lines".to_string(),
                        "Consider using height constraints to limit visible content".to_string(),
                    ),
                    ErrorRecommendation::with_auto_fix(
                        "Use height constraint".to_string(),
                        "Limit the visible height to improve performance".to_string(),
                        ".height(30)".to_string(),
                    ),
                ],
            ));
        }

        Ok(())
    }

    /// Validate spacing values
    pub fn validate_spacing(
        spacing: &crate::options::Spacing,
        field_name: &str,
    ) -> BoxenResult<()> {
        // Check for extremely large spacing values
        let max_reasonable_spacing = 100;

        if spacing.top > max_reasonable_spacing {
            return Err(BoxenError::input_validation_error(
                format!("Top {} value is unreasonably large", field_name),
                format!("{}.top", field_name),
                spacing.top.to_string(),
                vec![
                    ErrorRecommendation::suggestion_only(
                        "Excessive spacing".to_string(),
                        format!(
                            "Top {} of {} is very large and may cause layout issues",
                            field_name, spacing.top
                        ),
                    ),
                    ErrorRecommendation::with_auto_fix(
                        "Use reasonable spacing".to_string(),
                        "Consider using smaller spacing values".to_string(),
                        format!(".{}(5)", field_name),
                    ),
                ],
            ));
        }

        if spacing.right > max_reasonable_spacing {
            return Err(BoxenError::input_validation_error(
                format!("Right {} value is unreasonably large", field_name),
                format!("{}.right", field_name),
                spacing.right.to_string(),
                vec![ErrorRecommendation::suggestion_only(
                    "Excessive spacing".to_string(),
                    format!(
                        "Right {} of {} is very large and may cause layout issues",
                        field_name, spacing.right
                    ),
                )],
            ));
        }

        if spacing.bottom > max_reasonable_spacing {
            return Err(BoxenError::input_validation_error(
                format!("Bottom {} value is unreasonably large", field_name),
                format!("{}.bottom", field_name),
                spacing.bottom.to_string(),
                vec![ErrorRecommendation::suggestion_only(
                    "Excessive spacing".to_string(),
                    format!(
                        "Bottom {} of {} is very large and may cause layout issues",
                        field_name, spacing.bottom
                    ),
                )],
            ));
        }

        if spacing.left > max_reasonable_spacing {
            return Err(BoxenError::input_validation_error(
                format!("Left {} value is unreasonably large", field_name),
                format!("{}.left", field_name),
                spacing.left.to_string(),
                vec![ErrorRecommendation::suggestion_only(
                    "Excessive spacing".to_string(),
                    format!(
                        "Left {} of {} is very large and may cause layout issues",
                        field_name, spacing.left
                    ),
                )],
            ));
        }

        Ok(())
    }

    /// Validate dimension values
    pub fn validate_dimensions(width: Option<usize>, height: Option<usize>) -> BoxenResult<()> {
        if let Some(w) = width {
            if w == 0 {
                return Err(BoxenError::input_validation_error(
                    "Width cannot be zero".to_string(),
                    "width".to_string(),
                    "0".to_string(),
                    vec![ErrorRecommendation::with_auto_fix(
                        "Zero width".to_string(),
                        "Width must be at least 1 character".to_string(),
                        ".width(10)".to_string(),
                    )],
                ));
            }

            if w > 10000 {
                return Err(BoxenError::input_validation_error(
                    "Width is unreasonably large".to_string(),
                    "width".to_string(),
                    w.to_string(),
                    vec![
                        ErrorRecommendation::suggestion_only(
                            "Excessive width".to_string(),
                            format!("Width of {} is very large and may cause display issues", w),
                        ),
                        ErrorRecommendation::with_auto_fix(
                            "Use reasonable width".to_string(),
                            "Consider using a more reasonable width value".to_string(),
                            ".width(80)".to_string(),
                        ),
                    ],
                ));
            }
        }

        if let Some(h) = height {
            if h == 0 {
                return Err(BoxenError::input_validation_error(
                    "Height cannot be zero".to_string(),
                    "height".to_string(),
                    "0".to_string(),
                    vec![ErrorRecommendation::with_auto_fix(
                        "Zero height".to_string(),
                        "Height must be at least 1 line".to_string(),
                        ".height(5)".to_string(),
                    )],
                ));
            }

            if h > 1000 {
                return Err(BoxenError::input_validation_error(
                    "Height is unreasonably large".to_string(),
                    "height".to_string(),
                    h.to_string(),
                    vec![
                        ErrorRecommendation::suggestion_only(
                            "Excessive height".to_string(),
                            format!("Height of {} is very large and may cause display issues", h),
                        ),
                        ErrorRecommendation::with_auto_fix(
                            "Use reasonable height".to_string(),
                            "Consider using a more reasonable height value".to_string(),
                            ".height(30)".to_string(),
                        ),
                    ],
                ));
            }
        }

        Ok(())
    }

    /// Validate title input
    pub fn validate_title(title: &str) -> BoxenResult<()> {
        if title.len() > 200 {
            return Err(BoxenError::input_validation_error(
                "Title is too long".to_string(),
                "title".to_string(),
                format!("{} characters", title.len()),
                vec![
                    ErrorRecommendation::suggestion_only(
                        "Long title".to_string(),
                        "Very long titles may be truncated or cause layout issues".to_string(),
                    ),
                    ErrorRecommendation::with_auto_fix(
                        "Shorten title".to_string(),
                        "Consider using a shorter, more concise title".to_string(),
                        format!(".title(\"{}\")", &title[..20.min(title.len())]),
                    ),
                ],
            ));
        }

        // Check for control characters in title
        if title.chars().any(|c| c.is_control() && c != '\t') {
            return Err(BoxenError::input_validation_error(
                "Title contains invalid control characters".to_string(),
                "title".to_string(),
                title.to_string(),
                vec![ErrorRecommendation::suggestion_only(
                    "Control characters".to_string(),
                    "Titles should not contain control characters (except tabs)".to_string(),
                )],
            ));
        }

        Ok(())
    }

    /// Comprehensive validation of all configuration options
    pub fn validate_all_options(
        text: &str,
        options: &crate::options::BoxenOptions,
    ) -> BoxenResult<()> {
        // Validate text input
        validate_text_input(text)?;

        // Validate spacing
        validate_spacing(&options.padding, "padding")?;
        validate_spacing(&options.margin, "margin")?;

        // Validate dimensions
        validate_dimensions(options.width, options.height)?;

        // Validate title if present
        if let Some(ref title) = options.title {
            validate_title(title)?;
        }

        // Validate colors if present
        if let Some(ref color) = options.border_color {
            crate::color::validate_color(color).map_err(|_e| {
                BoxenError::input_validation_error(
                    "Invalid border color".to_string(),
                    "border_color".to_string(),
                    format!("{:?}", color),
                    vec![
                        ErrorRecommendation::suggestion_only(
                            "Invalid color".to_string(),
                            "Use a valid color name (red, blue, etc.) or hex code (#FF0000)"
                                .to_string(),
                        ),
                        ErrorRecommendation::with_auto_fix(
                            "Use valid color".to_string(),
                            "Try using a standard color name".to_string(),
                            ".border_color(\"blue\")".to_string(),
                        ),
                    ],
                )
            })?;
        }

        if let Some(ref color) = options.background_color {
            crate::color::validate_color(color).map_err(|_e| {
                BoxenError::input_validation_error(
                    "Invalid background color".to_string(),
                    "background_color".to_string(),
                    format!("{:?}", color),
                    vec![
                        ErrorRecommendation::suggestion_only(
                            "Invalid color".to_string(),
                            "Use a valid color name (red, blue, etc.) or hex code (#FF0000)"
                                .to_string(),
                        ),
                        ErrorRecommendation::with_auto_fix(
                            "Use valid color".to_string(),
                            "Try using a standard color name".to_string(),
                            ".background_color(\"white\")".to_string(),
                        ),
                    ],
                )
            })?;
        }

        Ok(())
    }
}
