//! Markdown styling configuration types.
//!
//! This module defines all configuration structures for controlling how markdown
//! elements are visually styled in the terminal output.
//!
//! # Configuration Levels
//!
//! There are two levels of configuration:
//!
//! 1. **MarkdownConfig**: Controls which markdown features are enabled/disabled
//! 2. **MarkdownStyle**: Controls the visual appearance of enabled features
//!
//! # Examples
//!
//! ## Disable specific features
//!
//! ```rust
//! use boxen::markdown::MarkdownConfig;
//!
//! let config = MarkdownConfig {
//!     headers: true,
//!     bold: true,
//!     italic: false,  // No italic rendering
//!     code: true,
//!     ..Default::default()
//! };
//! ```
//!
//! ## Customize visual styling
//!
//! ```rust
//! use boxen::{Color, markdown::MarkdownStyle};
//!
//! let style = MarkdownStyle {
//!     h1_color: Color::Named("magenta".to_string()),
//!     bold_color: Some(Color::Named("yellow".to_string())),
//!     list_marker: "→".to_string(),
//!     ..Default::default()
//! };
//! ```

use crate::Color;

/// Configuration for which markdown features to enable
///
/// All features are enabled by default. Disable specific features by setting
/// their flags to `false`.
///
/// # Examples
///
/// ```rust
/// use boxen::markdown::MarkdownConfig;
///
/// // Enable only headers and bold
/// let config = MarkdownConfig {
///     headers: true,
///     bold: true,
///     italic: false,
///     code: false,
///     lists: false,
///     links: false,
///     blockquotes: false,
///     horizontal_rules: false,
///     strikethrough: false,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct MarkdownConfig {
    /// Enable header parsing and styling
    pub headers: bool,
    /// Enable bold text parsing and styling
    pub bold: bool,
    /// Enable italic text parsing and styling
    pub italic: bool,
    /// Enable inline code and code block parsing
    pub code: bool,
    /// Enable list parsing and styling
    pub lists: bool,
    /// Enable link parsing and styling
    pub links: bool,
    /// Enable blockquote parsing and styling
    pub blockquotes: bool,
    /// Enable horizontal rule parsing
    pub horizontal_rules: bool,
    /// Enable strikethrough parsing and styling
    pub strikethrough: bool,
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self {
            headers: true,
            bold: true,
            italic: true,
            code: true,
            lists: true,
            links: true,
            blockquotes: true,
            horizontal_rules: true,
            strikethrough: true,
        }
    }
}

/// Visual styling configuration for markdown elements
///
/// Defines colors, styles, and visual appearance for all markdown elements.
/// Use `Default::default()` for sensible defaults, then override specific fields.
///
/// # Examples
///
/// ```rust
/// use boxen::{Color, markdown::{MarkdownStyle, BoldStyle, LinkStyle}};
///
/// let style = MarkdownStyle {
///     h1_color: Color::Named("blue".to_string()),
///     h2_color: Color::Named("cyan".to_string()),
///     bold_color: Some(Color::Named("yellow".to_string())),
///     bold_style: BoldStyle::BoldColor,
///     link_style: LinkStyle::ShowUrl,
///     list_marker: "•".to_string(),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct MarkdownStyle {
    // Header colors (H1-H6)
    /// Color for level 1 headers
    pub h1_color: Color,
    /// Color for level 2 headers
    pub h2_color: Color,
    /// Color for level 3 headers
    pub h3_color: Color,
    /// Color for level 4 headers
    pub h4_color: Color,
    /// Color for level 5 headers
    pub h5_color: Color,
    /// Color for level 6 headers
    pub h6_color: Color,

    // Text styling
    /// Optional color for bold text
    pub bold_color: Option<Color>,
    /// Style to use for bold text
    pub bold_style: BoldStyle,
    /// Style to use for italic text
    pub italic_style: ItalicStyle,

    // Code styling
    /// Foreground color for inline code
    pub inline_code_fg: Option<Color>,
    /// Background color for inline code
    pub inline_code_bg: Option<Color>,
    /// Foreground color for code blocks
    pub code_block_fg: Option<Color>,
    /// Background color for code blocks
    pub code_block_bg: Option<Color>,

    // List styling
    /// Character/string to use for unordered list markers
    pub list_marker: String,
    /// Optional color for list markers
    pub list_marker_color: Option<Color>,

    // Link styling
    /// How to display links in the terminal
    pub link_style: LinkStyle,
    /// Optional color for link text
    pub link_color: Option<Color>,

    // Blockquote styling
    /// Character/string to use for blockquote marker
    pub blockquote_marker: String,
    /// Optional color for blockquote marker
    pub blockquote_color: Option<Color>,

    // Horizontal rule
    /// Character to use for horizontal rules
    pub hr_char: char,
    /// Optional color for horizontal rules
    pub hr_color: Option<Color>,
}

impl Default for MarkdownStyle {
    fn default() -> Self {
        Self {
            h1_color: Color::Named("bright-white".to_string()),
            h2_color: Color::Named("bright-cyan".to_string()),
            h3_color: Color::Named("cyan".to_string()),
            h4_color: Color::Named("white".to_string()),
            h5_color: Color::Named("white".to_string()),
            h6_color: Color::Named("white".to_string()),

            bold_color: None,
            bold_style: BoldStyle::Bold,
            italic_style: ItalicStyle::Italic,

            inline_code_fg: Some(Color::Named("green".to_string())),
            inline_code_bg: None,
            code_block_fg: Some(Color::Named("green".to_string())),
            code_block_bg: None,

            list_marker: "•".to_string(),
            list_marker_color: None,

            link_style: LinkStyle::ShowText,
            link_color: Some(Color::Named("blue".to_string())),

            blockquote_marker: "▌".to_string(),
            blockquote_color: Some(Color::Named("bright-black".to_string())),

            hr_char: '─',
            hr_color: Some(Color::Named("bright-black".to_string())),
        }
    }
}

/// How to render bold text
///
/// Controls the ANSI styling applied to bold markdown text (`**bold**`).
///
/// # Examples
///
/// ```rust
/// use boxen::markdown::BoldStyle;
///
/// // Use ANSI bold escape code
/// let style = BoldStyle::Bold;
///
/// // Use bright color variant (if colored)
/// let style = BoldStyle::BrightColor;
///
/// // Combine both
/// let style = BoldStyle::BoldColor;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoldStyle {
    /// Use ANSI bold formatting
    Bold,
    /// Use bright color variant (if colored)
    BrightColor,
    /// Use both bold formatting and bright color
    BoldColor,
}

/// How to render italic text
///
/// Controls the ANSI styling applied to italic markdown text (`*italic*`).
/// Not all terminals support italic; `Underline` provides a fallback.
///
/// # Examples
///
/// ```rust
/// use boxen::markdown::ItalicStyle;
///
/// // Use ANSI italic (if terminal supports it)
/// let style = ItalicStyle::Italic;
///
/// // Use underline as fallback for unsupported terminals
/// let style = ItalicStyle::Underline;
///
/// // Use dim text instead
/// let style = ItalicStyle::Dim;
///
/// // No styling for italic
/// let style = ItalicStyle::None;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItalicStyle {
    /// Use ANSI italic formatting (if terminal supports it)
    Italic,
    /// Use underline as fallback
    Underline,
    /// Use dim text
    Dim,
    /// No styling for italic
    None,
}

/// How to display links in terminal output
///
/// Controls how markdown links `[text](url)` are rendered in the terminal.
///
/// # Examples
///
/// ```rust
/// use boxen::markdown::LinkStyle;
///
/// // Show only "text"
/// let style = LinkStyle::ShowText;
///
/// // Show "text (url)"
/// let style = LinkStyle::ShowUrl;
///
/// // Show "text: url"
/// let style = LinkStyle::ShowBoth;
///
/// // Remove links entirely
/// let style = LinkStyle::Hide;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkStyle {
    /// Show only the link text
    ShowText,
    /// Show text with URL in parentheses: "text (url)"
    ShowUrl,
    /// Show text with URL after colon: "text: url"
    ShowBoth,
    /// Remove links entirely
    Hide,
}
