/// Configuration options and types for boxen
use crate::error::{BoxenError, BoxenResult};
use crate::terminal::{calculate_border_width, get_terminal_height, get_terminal_width};

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
#[derive(Debug, Default, Clone)]
pub struct Spacing {
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
    pub left: usize,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_horizontal_vertical() {
        let spacing = Spacing {
            top: 1,
            right: 2,
            bottom: 3,
            left: 4,
        };

        assert_eq!(spacing.horizontal(), 6); // left + right = 4 + 2
        assert_eq!(spacing.vertical(), 4); // top + bottom = 1 + 3
    }

    #[test]
    fn test_spacing_is_empty() {
        let empty_spacing = Spacing::default();
        assert!(empty_spacing.is_empty());

        let non_empty_spacing = Spacing {
            top: 1,
            right: 0,
            bottom: 0,
            left: 0,
        };
        assert!(!non_empty_spacing.is_empty());
    }

    #[test]
    fn test_spacing_from_usize() {
        let spacing = Spacing::from(2);
        assert_eq!(spacing.top, 2);
        assert_eq!(spacing.right, 6); // 3x horizontal
        assert_eq!(spacing.bottom, 2);
        assert_eq!(spacing.left, 6); // 3x horizontal
        assert_eq!(spacing.horizontal(), 12);
        assert_eq!(spacing.vertical(), 4);
    }

    #[test]
    fn test_spacing_from_tuple() {
        let spacing = Spacing::from((1, 2, 3, 4));
        assert_eq!(spacing.top, 1);
        assert_eq!(spacing.right, 2);
        assert_eq!(spacing.bottom, 3);
        assert_eq!(spacing.left, 4);
        assert_eq!(spacing.horizontal(), 6);
        assert_eq!(spacing.vertical(), 4);
    }

    #[test]
    fn test_calculate_constraints_default() {
        let options = BoxenOptions::default();
        let constraints = options.calculate_constraints().unwrap();

        assert!(constraints.terminal_width >= 80); // Should have fallback
        assert_eq!(constraints.border_width, 2); // Single border style
        assert!(constraints.max_width <= constraints.terminal_width);
    }

    #[test]
    fn test_calculate_constraints_with_specified_width() {
        let options = BoxenOptions {
            width: Some(50),
            ..Default::default()
        };
        let constraints = options.calculate_constraints().unwrap();

        assert_eq!(constraints.max_width, 50);
    }

    #[test]
    fn test_calculate_constraints_invalid_width() {
        let options = BoxenOptions {
            width: Some(1),            // Too small for borders + padding
            padding: Spacing::from(2), // 6 horizontal padding + 2 border = 8 total
            ..Default::default()
        };

        let result = options.calculate_constraints();
        assert!(result.is_err());
        matches!(result.unwrap_err(), BoxenError::InvalidDimensions { .. });
    }

    #[test]
    fn test_calculate_layout_dimensions_basic() {
        let options = BoxenOptions::default();
        let layout = options.calculate_layout_dimensions(10, 3).unwrap();

        assert_eq!(layout.content_width, 10);
        assert_eq!(layout.content_height, 3);
        assert_eq!(layout.inner_width, 10); // no padding
        assert_eq!(layout.inner_height, 3); // no padding
        assert_eq!(layout.total_width, 12); // 10 + 2 borders
        assert_eq!(layout.total_height, 5); // 3 + 2 borders
    }

    #[test]
    fn test_calculate_layout_dimensions_with_padding() {
        let options = BoxenOptions {
            padding: Spacing::from(1), // 3 horizontal, 1 vertical each side
            ..Default::default()
        };
        let layout = options.calculate_layout_dimensions(10, 3).unwrap();

        assert_eq!(layout.content_width, 10);
        assert_eq!(layout.content_height, 3);
        assert_eq!(layout.inner_width, 16); // 10 + 6 horizontal padding
        assert_eq!(layout.inner_height, 5); // 3 + 2 vertical padding
        assert_eq!(layout.total_width, 18); // 16 + 2 borders
        assert_eq!(layout.total_height, 7); // 5 + 2 borders
    }

    #[test]
    fn test_calculate_layout_dimensions_with_margins() {
        let options = BoxenOptions {
            margin: Spacing::from(1), // 3 horizontal, 1 vertical each side
            ..Default::default()
        };
        let layout = options.calculate_layout_dimensions(10, 3).unwrap();

        assert_eq!(layout.total_width, 18); // 10 + 2 borders + 6 margins
        assert_eq!(layout.total_height, 7); // 3 + 2 borders + 2 margins
    }

    #[test]
    fn test_calculate_layout_dimensions_no_border() {
        let options = BoxenOptions {
            border_style: BorderStyle::None,
            ..Default::default()
        };
        let layout = options.calculate_layout_dimensions(10, 3).unwrap();

        assert_eq!(layout.total_width, 10); // no borders
        assert_eq!(layout.total_height, 3); // no borders
    }

    #[test]
    fn test_calculate_max_content_width() {
        let options = BoxenOptions {
            width: Some(50),
            padding: Spacing::from(1), // 6 horizontal padding
            ..Default::default()
        };

        let max_width = options.calculate_max_content_width().unwrap();
        assert_eq!(max_width, 42); // 50 - 2 borders - 6 padding
    }

    #[test]
    fn test_calculate_max_content_height() {
        let options = BoxenOptions {
            height: Some(20),
            padding: Spacing::from(1), // 2 vertical padding
            ..Default::default()
        };

        let max_height = options.calculate_max_content_height().unwrap();
        assert_eq!(max_height, Some(16)); // 20 - 2 borders - 2 padding
    }

    #[test]
    fn test_calculate_max_content_height_no_constraint() {
        let options = BoxenOptions::default(); // No height specified

        let max_height = options.calculate_max_content_height().unwrap();
        // Should be None or Some value depending on terminal height detection
        // We can't assert a specific value since it depends on the actual terminal
        assert!(max_height.is_none() || max_height.unwrap() > 0);
    }

    #[test]
    fn test_validate_constraints_valid() {
        let options = BoxenOptions {
            width: Some(50),
            height: Some(20),
            padding: Spacing::from(1),
            ..Default::default()
        };

        assert!(options.validate_constraints().is_ok());
    }

    #[test]
    fn test_validate_constraints_invalid() {
        let options = BoxenOptions {
            width: Some(5),            // Too small
            padding: Spacing::from(2), // Large padding
            ..Default::default()
        };

        assert!(options.validate_constraints().is_err());
    }

    #[test]
    fn test_dimension_constraints_with_fullscreen() {
        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Auto),
            ..Default::default()
        };

        let constraints = options.calculate_constraints().unwrap();
        // Should use terminal dimensions
        assert!(constraints.max_width > 0);
        assert_eq!(constraints.max_width, constraints.terminal_width);
    }

    #[test]
    fn test_layout_dimensions_edge_cases() {
        // Test with zero content
        let options = BoxenOptions::default();
        let layout = options.calculate_layout_dimensions(0, 0).unwrap();

        assert_eq!(layout.content_width, 0);
        assert_eq!(layout.content_height, 0);
        assert_eq!(layout.total_width, 2); // Just borders
        assert_eq!(layout.total_height, 2); // Just borders
    }

    #[test]
    fn test_spacing_calculations_comprehensive() {
        let spacing = Spacing {
            top: 2,
            right: 4,
            bottom: 1,
            left: 3,
        };

        assert_eq!(spacing.horizontal(), 7); // 4 + 3
        assert_eq!(spacing.vertical(), 3); // 2 + 1
        assert!(!spacing.is_empty());

        let zero_spacing = Spacing::default();
        assert_eq!(zero_spacing.horizontal(), 0);
        assert_eq!(zero_spacing.vertical(), 0);
        assert!(zero_spacing.is_empty());
    }

    #[test]
    fn test_complex_layout_calculation() {
        let options = BoxenOptions {
            border_style: BorderStyle::Double,
            padding: Spacing {
                top: 1,
                right: 2,
                bottom: 1,
                left: 2,
            },
            margin: Spacing {
                top: 0,
                right: 1,
                bottom: 0,
                left: 1,
            },
            width: Some(60),
            height: Some(50), // Use a larger height that should work on most terminals
            ..Default::default()
        };

        let layout = options.calculate_layout_dimensions(40, 5).unwrap(); // Use smaller content height

        // Content: 40x5
        // Inner (content + padding): 44x7 (40+4, 5+2)
        // Total (inner + borders + margins): 48x9 (44+2+2, 7+2+0)
        assert_eq!(layout.content_width, 40);
        assert_eq!(layout.content_height, 5);
        assert_eq!(layout.inner_width, 44);
        assert_eq!(layout.inner_height, 7);
        assert_eq!(layout.total_width, 48);
        assert_eq!(layout.total_height, 9);
    }

    #[test]
    fn test_overflow_handling() {
        // Test when content would exceed terminal width
        let terminal_width = get_terminal_width();
        let options = BoxenOptions {
            padding: Spacing::from(1),
            ..Default::default()
        };

        // Try to create content that would exceed terminal
        let excessive_width = terminal_width + 100;
        let result = options.calculate_layout_dimensions(excessive_width, 5);

        // Should fail with configuration error
        assert!(result.is_err());
        matches!(result.unwrap_err(), BoxenError::ConfigurationError(_));
    }
}

/// Dimension constraints for box calculation
#[derive(Debug, Clone)]
pub struct DimensionConstraints {
    pub max_width: usize,
    pub max_height: Option<usize>,
    pub terminal_width: usize,
    pub terminal_height: Option<usize>,
    pub border_width: usize,
}

/// Final calculated layout dimensions
#[derive(Debug, Clone)]
pub struct LayoutDimensions {
    pub content_width: usize,
    pub content_height: usize,
    pub total_width: usize,
    pub total_height: usize,
    pub inner_width: usize,  // content width + padding
    pub inner_height: usize, // content height + padding
}

impl Spacing {
    /// Get total horizontal spacing (left + right)
    pub fn horizontal(&self) -> usize {
        self.left + self.right
    }

    /// Get total vertical spacing (top + bottom)
    pub fn vertical(&self) -> usize {
        self.top + self.bottom
    }

    /// Check if spacing has any non-zero values
    pub fn is_empty(&self) -> bool {
        self.top == 0 && self.right == 0 && self.bottom == 0 && self.left == 0
    }
}

impl BoxenOptions {
    /// Calculate dimension constraints based on terminal size and options
    pub fn calculate_constraints(&self) -> BoxenResult<DimensionConstraints> {
        let terminal_width = get_terminal_width();
        let terminal_height = get_terminal_height();
        let border_width = calculate_border_width(&self.border_style);

        let _total_horizontal_overhead =
            border_width + self.padding.horizontal() + self.margin.horizontal();

        // Calculate maximum available width
        let max_width = if let Some(specified_width) = self.width {
            // When width is specified, it represents the total box width including margins
            // So we need to subtract margins to get the available width for content + borders + padding
            let available_width_for_content = if specified_width > self.margin.horizontal() {
                specified_width - self.margin.horizontal()
            } else {
                return Err(BoxenError::InvalidDimensions {
                    width: Some(specified_width),
                    height: self.height,
                });
            };

            // Validate that we have enough space for borders and padding
            if available_width_for_content < border_width + self.padding.horizontal() {
                return Err(BoxenError::InvalidDimensions {
                    width: Some(specified_width),
                    height: self.height,
                });
            }

            available_width_for_content
        } else {
            // Use terminal width minus margins (borders and padding will be subtracted later)
            if terminal_width < self.margin.horizontal() {
                return Err(BoxenError::TerminalSizeError);
            }
            terminal_width - self.margin.horizontal()
        };

        // Calculate maximum available height
        let max_height = if let Some(specified_height) = self.height {
            // When height is specified, it represents the total box height including margins
            // So we need to subtract margins to get the available height for content + borders + padding
            if specified_height > self.margin.vertical() {
                Some(specified_height - self.margin.vertical())
            } else {
                return Err(BoxenError::InvalidDimensions {
                    width: None,
                    height: Some(specified_height),
                });
            }
        } else {
            // Don't apply height constraints unless explicitly specified by user
            None
        };

        Ok(DimensionConstraints {
            max_width,
            max_height,
            terminal_width,
            terminal_height,
            border_width,
        })
    }

    /// Calculate final layout dimensions for given content
    pub fn calculate_layout_dimensions(
        &self,
        content_width: usize,
        content_height: usize,
    ) -> BoxenResult<LayoutDimensions> {
        let constraints = self.calculate_constraints()?;

        // Calculate inner dimensions (content + padding)
        let inner_width = content_width + self.padding.horizontal();
        let inner_height = content_height + self.padding.vertical();

        // Calculate box dimensions without margins (for constraint validation)
        let box_width = inner_width + constraints.border_width;
        let box_height = inner_height
            + (if matches!(self.border_style, BorderStyle::None) {
                0
            } else {
                2
            }); // top and bottom borders

        // Calculate total dimensions (box + margins)
        let total_width = box_width + self.margin.horizontal();
        let total_height = box_height + self.margin.vertical();

        // Validate against constraints
        // If a specific width was set, compare against that; otherwise use max_width
        let width_limit = if let Some(specified_width) = self.width {
            specified_width
        } else {
            constraints.max_width + self.margin.horizontal()
        };

        if total_width > width_limit {
            return Err(BoxenError::ConfigurationError(format!(
                "Calculated box width ({}) exceeds maximum available width ({})",
                total_width, width_limit
            )));
        }

        // For height validation, compare box height (without margins) against max_height (which already has margins subtracted)
        if let Some(max_height) = constraints.max_height {
            if box_height > max_height {
                return Err(BoxenError::ConfigurationError(format!(
                    "Calculated box height ({}) exceeds maximum available height ({})",
                    box_height, max_height
                )));
            }
        }

        // Validate against terminal constraints
        if total_width > constraints.terminal_width {
            return Err(BoxenError::ConfigurationError(format!(
                "Box width ({}) exceeds terminal width ({})",
                total_width, constraints.terminal_width
            )));
        }

        if let Some(terminal_height) = constraints.terminal_height {
            if total_height > terminal_height {
                return Err(BoxenError::ConfigurationError(format!(
                    "Box height ({}) exceeds terminal height ({})",
                    total_height, terminal_height
                )));
            }
        }

        Ok(LayoutDimensions {
            content_width,
            content_height,
            total_width,
            total_height,
            inner_width,
            inner_height,
        })
    }

    /// Calculate the maximum content width available given the current options
    pub fn calculate_max_content_width(&self) -> BoxenResult<usize> {
        let constraints = self.calculate_constraints()?;
        let total_overhead = constraints.border_width + self.padding.horizontal();

        if constraints.max_width < total_overhead {
            return Err(BoxenError::InvalidDimensions {
                width: Some(constraints.max_width),
                height: None,
            });
        }

        Ok(constraints.max_width - total_overhead)
    }

    /// Calculate the maximum content height available given the current options
    pub fn calculate_max_content_height(&self) -> BoxenResult<Option<usize>> {
        let constraints = self.calculate_constraints()?;

        if let Some(max_height) = constraints.max_height {
            let vertical_overhead = self.padding.vertical()
                + (if matches!(self.border_style, BorderStyle::None) {
                    0
                } else {
                    2
                });

            if max_height < vertical_overhead {
                return Err(BoxenError::InvalidDimensions {
                    width: None,
                    height: Some(max_height),
                });
            }

            Ok(Some(max_height - vertical_overhead))
        } else {
            Ok(None)
        }
    }

    /// Validate that the current options are compatible with terminal constraints
    pub fn validate_constraints(&self) -> BoxenResult<()> {
        let _constraints = self.calculate_constraints()?;
        let _max_content_width = self.calculate_max_content_width()?;
        let _max_content_height = self.calculate_max_content_height()?;
        Ok(())
    }
}
