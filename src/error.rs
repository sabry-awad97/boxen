/// Error types for the boxen library
use thiserror::Error;

/// Errors that can occur when creating or rendering boxes
#[derive(Debug, Error)]
pub enum BoxenError {
    #[error("Invalid border style: {0}")]
    InvalidBorderStyle(String),

    #[error("Invalid color specification: {0}")]
    InvalidColor(String),

    #[error("Invalid dimensions: width={width:?}, height={height:?}")]
    InvalidDimensions {
        width: Option<usize>,
        height: Option<usize>,
    },

    #[error("Terminal size detection failed")]
    TerminalSizeError,

    #[error("Text processing error: {0}")]
    TextProcessingError(String),

    #[error("Configuration conflict: {0}")]
    ConfigurationError(String),
}

/// Result type alias for boxen operations
pub type BoxenResult<T> = Result<T, BoxenError>;
