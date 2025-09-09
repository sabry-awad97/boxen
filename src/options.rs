/// Configuration options and types for boxen
use crate::error::{BoxenError, BoxenResult};

/// Main configuration struct for boxen
#[derive(Debug, Clone)]
pub struct BoxenOptions {
    pub border_style: BorderStyle,
    pub padding: Spacing,
    pub margin: Spacing,
    pub text_alignment: TextAlignment,
    pub title: Option<String>,
    pub title_alignment: TitleAlignment,
    pub float: Float,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub border_color: Option<Color>,
    pub background_color: Option<Color>,
    pub dim_border: bool,
    pub fullscreen: Option<FullscreenMode>,
}

impl Default for BoxenOptions {
    fn default() -> Self {
        Self {
            border_style: BorderStyle::Single,
            padding: Spacing::default(),
            margin: Spacing::default(),
            text_alignment: TextAlignment::Left,
            title: None,
            title_alignment: TitleAlignment::Left,
            float: Float::Left,
            width: None,
            height: None,
            border_color: None,
            background_color: None,
            dim_border: false,
            fullscreen: None,
        }
    }
}

/// Border style definition
#[derive(Debug, Clone)]
pub enum BorderStyle {
    None,
    Single,
    Double,
    Round,
    Bold,
    SingleDouble,
    DoubleSingle,
    Classic,
    Custom(BorderChars),
}

/// Border character set for custom borders
#[derive(Debug, Clone)]
pub struct BorderChars {
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub left: char,
    pub right: char,
    pub top: char,
    pub bottom: char,
}

/// Spacing configuration for padding and margins
#[derive(Debug, Clone)]
pub struct Spacing {
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
    pub left: usize,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }
}

impl From<usize> for Spacing {
    /// Creates asymmetric spacing (3x horizontal, 1x vertical) to match TypeScript behavior
    fn from(value: usize) -> Self {
        Self {
            top: value,
            right: value * 3,
            bottom: value,
            left: value * 3,
        }
    }
}

impl From<(usize, usize, usize, usize)> for Spacing {
    /// Creates spacing from (top, right, bottom, left) tuple
    fn from((top, right, bottom, left): (usize, usize, usize, usize)) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

/// Text alignment within the box
#[derive(Debug, Clone)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

/// Title alignment within the top border
#[derive(Debug, Clone)]
pub enum TitleAlignment {
    Left,
    Center,
    Right,
}

/// Box positioning relative to terminal
#[derive(Debug, Clone)]
pub enum Float {
    Left,
    Center,
    Right,
}

/// Color specification for borders and backgrounds
#[derive(Debug, Clone)]
pub enum Color {
    Named(String),
    Hex(String),
    Rgb(u8, u8, u8),
}

/// Fullscreen mode configuration
#[derive(Debug, Clone)]
pub enum FullscreenMode {
    Auto,
    Custom(fn(usize, usize) -> (usize, usize)),
}

/// Builder pattern for creating BoxenOptions
pub struct BoxenBuilder {
    options: BoxenOptions,
}

impl BoxenBuilder {
    /// Create a new builder with default options
    pub fn new() -> Self {
        Self {
            options: BoxenOptions::default(),
        }
    }

    /// Set the border style
    pub fn border_style(mut self, style: BorderStyle) -> Self {
        self.options.border_style = style;
        self
    }

    /// Set padding (accepts usize or Spacing)
    pub fn padding<T: Into<Spacing>>(mut self, padding: T) -> Self {
        self.options.padding = padding.into();
        self
    }

    /// Set margin (accepts usize or Spacing)
    pub fn margin<T: Into<Spacing>>(mut self, margin: T) -> Self {
        self.options.margin = margin.into();
        self
    }

    /// Set text alignment
    pub fn text_alignment(mut self, alignment: TextAlignment) -> Self {
        self.options.text_alignment = alignment;
        self
    }

    /// Set title text
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.options.title = Some(title.into());
        self
    }

    /// Set title alignment
    pub fn title_alignment(mut self, alignment: TitleAlignment) -> Self {
        self.options.title_alignment = alignment;
        self
    }

    /// Set float positioning
    pub fn float(mut self, float: Float) -> Self {
        self.options.float = float;
        self
    }

    /// Set box width
    pub fn width(mut self, width: usize) -> Self {
        self.options.width = Some(width);
        self
    }

    /// Set box height
    pub fn height(mut self, height: usize) -> Self {
        self.options.height = Some(height);
        self
    }

    /// Set border color
    pub fn border_color<C: Into<Color>>(mut self, color: C) -> Self {
        self.options.border_color = Some(color.into());
        self
    }

    /// Set background color
    pub fn background_color<C: Into<Color>>(mut self, color: C) -> Self {
        self.options.background_color = Some(color.into());
        self
    }

    /// Enable dim border
    pub fn dim_border(mut self, dim: bool) -> Self {
        self.options.dim_border = dim;
        self
    }

    /// Set fullscreen mode
    pub fn fullscreen(mut self, mode: FullscreenMode) -> Self {
        self.options.fullscreen = Some(mode);
        self
    }

    /// Build the final options
    pub fn build(self) -> BoxenOptions {
        self.options
    }

    /// Build and render box with the given text
    pub fn render<S: AsRef<str>>(self, text: S) -> BoxenResult<String> {
        crate::boxen(text.as_ref(), Some(self.options))
    }
}

impl Default for BoxenBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for Color {
    fn from(value: String) -> Self {
        if value.starts_with('#') {
            Color::Hex(value)
        } else {
            Color::Named(value)
        }
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        Color::from(value.to_string())
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color::Rgb(r, g, b)
    }
}
