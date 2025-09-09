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
    #[error("Invalid border style: {0}")]
    InvalidBorderStyle(String),

    #[error("Invalid color specification: {0}")]
    InvalidColor(String),

    #[error("Invalid dimensions: {message}")]
    InvalidDimensions {
        message: String,
        width: Option<usize>,
        height: Option<usize>,
        recommendations: Vec<ErrorRecommendation>,
    },

    #[error("Terminal size detection failed")]
    TerminalSizeError,

    #[error("Text processing error: {0}")]
    TextProcessingError(String),

    #[error("Configuration conflict: {message}")]
    ConfigurationError {
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

    /// Get recommendations for fixing this error
    pub fn recommendations(&self) -> Vec<ErrorRecommendation> {
        match self {
            Self::InvalidDimensions {
                recommendations, ..
            } => recommendations.clone(),
            Self::ConfigurationError {
                recommendations, ..
            } => recommendations.clone(),
            _ => vec![],
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
