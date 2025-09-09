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
#[derive(Debug, Default, Clone, Copy)]
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
    /// Automatically use terminal dimensions
    Auto,
    /// Use custom function to calculate dimensions from terminal size
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
        // Validate configuration before rendering
        self.options.validate_constraints()?;
        crate::boxen(text.as_ref(), Some(self.options))
    }

    /// Validate the current builder configuration without building
    pub fn validate(&self) -> BoxenResult<()> {
        self.options.validate_constraints()
    }

    /// Convenience method to set both padding and margin to the same value
    pub fn spacing<T: Into<Spacing>>(mut self, spacing: T) -> Self {
        let spacing_value = spacing.into();
        self.options.padding = spacing_value;
        self.options.margin = spacing_value;
        self
    }

    /// Convenience method to set both border and background color
    pub fn colors<C1: Into<Color>, C2: Into<Color>>(mut self, border: C1, background: C2) -> Self {
        self.options.border_color = Some(border.into());
        self.options.background_color = Some(background.into());
        self
    }

    /// Convenience method to set both width and height
    pub fn size(mut self, width: usize, height: usize) -> Self {
        self.options.width = Some(width);
        self.options.height = Some(height);
        self
    }

    /// Convenience method to center align both text and title
    pub fn center_all(mut self) -> Self {
        self.options.text_alignment = TextAlignment::Center;
        self.options.title_alignment = TitleAlignment::Center;
        self.options.float = Float::Center;
        self
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

// Additional convenient From implementations
impl From<(usize, usize)> for Spacing {
    /// Creates spacing from (horizontal, vertical) tuple
    /// Horizontal value is applied to left and right, vertical to top and bottom
    fn from((horizontal, vertical): (usize, usize)) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
}

impl From<[usize; 4]> for Spacing {
    /// Creates spacing from [top, right, bottom, left] array
    fn from([top, right, bottom, left]: [usize; 4]) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

impl From<[usize; 2]> for Spacing {
    /// Creates spacing from [horizontal, vertical] array
    fn from([horizontal, vertical]: [usize; 2]) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
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

    #[test]
    fn test_fullscreen_mode_auto() {
        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Auto),
            ..Default::default()
        };

        let constraints = options.calculate_constraints().unwrap();

        // Should use terminal dimensions
        assert_eq!(constraints.max_width, constraints.terminal_width);
        assert_eq!(constraints.max_height, constraints.terminal_height);
    }

    #[test]
    fn test_fullscreen_mode_auto_with_margins() {
        let margin = Spacing::from(2);
        println!(
            "Margin: top={}, right={}, bottom={}, left={}",
            margin.top, margin.right, margin.bottom, margin.left
        );
        println!("Horizontal margin: {}", margin.horizontal());

        let expected_horizontal = margin.horizontal();
        let expected_vertical = margin.vertical();

        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Auto),
            margin,
            ..Default::default()
        };

        let constraints = options.calculate_constraints().unwrap();

        // Should account for margins - Spacing::from(2) creates 12 horizontal (left=6, right=6), 4 vertical
        assert_eq!(
            constraints.max_width,
            constraints.terminal_width - expected_horizontal
        );
        if let Some(terminal_height) = constraints.terminal_height {
            assert_eq!(
                constraints.max_height,
                Some(terminal_height - expected_vertical)
            );
        }
    }

    #[test]
    fn test_fullscreen_mode_custom() {
        let custom_func = |width: usize, height: usize| -> (usize, usize) {
            // Use 3/4 of dimensions to ensure we have enough space for borders and padding
            (width * 3 / 4, height * 3 / 4)
        };

        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Custom(custom_func)),
            ..Default::default()
        };

        let constraints = options.calculate_constraints().unwrap();

        // Should use custom dimensions
        assert_eq!(constraints.max_width, constraints.terminal_width * 3 / 4);
        if let Some(terminal_height) = constraints.terminal_height {
            assert_eq!(constraints.max_height, Some(terminal_height * 3 / 4));
        }
    }

    #[test]
    fn test_fullscreen_mode_with_padding() {
        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Auto),
            padding: Spacing::from(1), // 6 horizontal, 2 vertical
            ..Default::default()
        };

        let max_content_width = options.calculate_max_content_width().unwrap();
        let max_content_height = options.calculate_max_content_height().unwrap();

        // Should account for borders and padding
        let constraints = options.calculate_constraints().unwrap();
        let expected_width = constraints.max_width - 2 - 6; // borders + padding horizontal
        assert_eq!(max_content_width, expected_width);

        if let Some(height) = max_content_height {
            let expected_height = constraints.max_height.unwrap() - 2 - 2; // borders + padding vertical
            assert_eq!(height, expected_height);
        }
    }

    #[test]
    fn test_fullscreen_mode_insufficient_space() {
        // Create a scenario where fullscreen mode doesn't have enough space
        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Custom(|_, _| (5, 3))), // Very small dimensions
            padding: Spacing::from(3), // Large padding: 18 horizontal (9*2), 6 vertical (3*2)
            ..Default::default()
        };

        let result = options.calculate_constraints();

        // Should fail due to insufficient space (5 total width < 18 padding + 2 borders = 20)
        assert!(result.is_err());
        matches!(result.unwrap_err(), BoxenError::InvalidDimensions { .. });
    }

    #[test]
    fn test_fullscreen_mode_overrides_width_height() {
        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Auto),
            width: Some(50),  // Should be ignored in fullscreen mode
            height: Some(20), // Should be ignored in fullscreen mode
            ..Default::default()
        };

        let constraints = options.calculate_constraints().unwrap();

        // Should use terminal dimensions, not specified width/height
        assert_eq!(constraints.max_width, constraints.terminal_width);
        assert_eq!(constraints.max_height, constraints.terminal_height);
    }

    #[test]
    fn test_fullscreen_layout_dimensions() {
        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Auto),
            padding: Spacing::from(1), // 6 horizontal, 2 vertical
            margin: Spacing::from(1),  // 6 horizontal, 2 vertical
            ..Default::default()
        };

        // Use small content that should be expanded to fill fullscreen
        let layout = options.calculate_layout_dimensions(10, 3).unwrap();

        // Content should be expanded to fill available space
        let max_content_width = options.calculate_max_content_width().unwrap();
        let max_content_height = options.calculate_max_content_height().unwrap();

        assert_eq!(layout.content_width, max_content_width);
        if let Some(expected_height) = max_content_height {
            assert_eq!(layout.content_height, expected_height);
        }
    }

    // Builder pattern tests
    #[test]
    fn test_builder_new() {
        let builder = BoxenBuilder::new();
        let options = builder.build();

        // Should have default values
        assert!(matches!(options.border_style, BorderStyle::Single));
        assert!(options.padding.is_empty());
        assert!(options.margin.is_empty());
        assert!(matches!(options.text_alignment, TextAlignment::Left));
        assert!(options.title.is_none());
        assert!(matches!(options.title_alignment, TitleAlignment::Left));
        assert!(matches!(options.float, Float::Left));
        assert!(options.width.is_none());
        assert!(options.height.is_none());
        assert!(options.border_color.is_none());
        assert!(options.background_color.is_none());
        assert!(!options.dim_border);
        assert!(options.fullscreen.is_none());
    }

    #[test]
    fn test_builder_default() {
        let builder = BoxenBuilder::default();
        let options = builder.build();

        // Should be same as new()
        assert!(matches!(options.border_style, BorderStyle::Single));
        assert!(options.padding.is_empty());
    }

    #[test]
    fn test_builder_method_chaining() {
        let result = BoxenBuilder::new()
            .border_style(BorderStyle::Double)
            .padding(2)
            .margin((1, 2, 3, 4))
            .text_alignment(TextAlignment::Center)
            .title("Test Title")
            .title_alignment(TitleAlignment::Right)
            .float(Float::Center)
            .width(50)
            .height(20)
            .border_color("red")
            .background_color("#ff0000")
            .dim_border(true)
            .fullscreen(FullscreenMode::Auto)
            .render("Hello World");

        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_border_style() {
        let options = BoxenBuilder::new().border_style(BorderStyle::Round).build();

        assert!(matches!(options.border_style, BorderStyle::Round));
    }

    #[test]
    fn test_builder_padding_from_usize() {
        let options = BoxenBuilder::new().padding(3).build();

        assert_eq!(options.padding.top, 3);
        assert_eq!(options.padding.right, 9); // 3x horizontal
        assert_eq!(options.padding.bottom, 3);
        assert_eq!(options.padding.left, 9); // 3x horizontal
    }

    #[test]
    fn test_builder_padding_from_tuple() {
        let options = BoxenBuilder::new().padding((1, 2, 3, 4)).build();

        assert_eq!(options.padding.top, 1);
        assert_eq!(options.padding.right, 2);
        assert_eq!(options.padding.bottom, 3);
        assert_eq!(options.padding.left, 4);
    }

    #[test]
    fn test_builder_margin_from_usize() {
        let options = BoxenBuilder::new().margin(2).build();

        assert_eq!(options.margin.top, 2);
        assert_eq!(options.margin.right, 6); // 3x horizontal
        assert_eq!(options.margin.bottom, 2);
        assert_eq!(options.margin.left, 6); // 3x horizontal
    }

    #[test]
    fn test_builder_margin_from_tuple() {
        let options = BoxenBuilder::new().margin((5, 6, 7, 8)).build();

        assert_eq!(options.margin.top, 5);
        assert_eq!(options.margin.right, 6);
        assert_eq!(options.margin.bottom, 7);
        assert_eq!(options.margin.left, 8);
    }

    #[test]
    fn test_builder_text_alignment() {
        let options = BoxenBuilder::new()
            .text_alignment(TextAlignment::Right)
            .build();

        assert!(matches!(options.text_alignment, TextAlignment::Right));
    }

    #[test]
    fn test_builder_title() {
        let options = BoxenBuilder::new().title("My Title").build();

        assert_eq!(options.title, Some("My Title".to_string()));
    }

    #[test]
    fn test_builder_title_string() {
        let title = String::from("Dynamic Title");
        let options = BoxenBuilder::new().title(title.clone()).build();

        assert_eq!(options.title, Some(title));
    }

    #[test]
    fn test_builder_title_alignment() {
        let options = BoxenBuilder::new()
            .title_alignment(TitleAlignment::Center)
            .build();

        assert!(matches!(options.title_alignment, TitleAlignment::Center));
    }

    #[test]
    fn test_builder_float() {
        let options = BoxenBuilder::new().float(Float::Right).build();

        assert!(matches!(options.float, Float::Right));
    }

    #[test]
    fn test_builder_dimensions() {
        let options = BoxenBuilder::new().width(80).height(25).build();

        assert_eq!(options.width, Some(80));
        assert_eq!(options.height, Some(25));
    }

    #[test]
    fn test_builder_border_color_string() {
        let options = BoxenBuilder::new().border_color("blue").build();

        if let Some(Color::Named(name)) = options.border_color {
            assert_eq!(name, "blue");
        } else {
            panic!("Expected named color");
        }
    }

    #[test]
    fn test_builder_border_color_hex() {
        let options = BoxenBuilder::new().border_color("#00ff00").build();

        if let Some(Color::Hex(hex)) = options.border_color {
            assert_eq!(hex, "#00ff00");
        } else {
            panic!("Expected hex color");
        }
    }

    #[test]
    fn test_builder_border_color_rgb() {
        let options = BoxenBuilder::new().border_color((255, 128, 0)).build();

        if let Some(Color::Rgb(r, g, b)) = options.border_color {
            assert_eq!((r, g, b), (255, 128, 0));
        } else {
            panic!("Expected RGB color");
        }
    }

    #[test]
    fn test_builder_background_color() {
        let options = BoxenBuilder::new().background_color("yellow").build();

        if let Some(Color::Named(name)) = options.background_color {
            assert_eq!(name, "yellow");
        } else {
            panic!("Expected named color");
        }
    }

    #[test]
    fn test_builder_dim_border() {
        let options = BoxenBuilder::new().dim_border(true).build();

        assert!(options.dim_border);
    }

    #[test]
    fn test_builder_fullscreen_auto() {
        let options = BoxenBuilder::new().fullscreen(FullscreenMode::Auto).build();

        assert!(matches!(options.fullscreen, Some(FullscreenMode::Auto)));
    }

    #[test]
    fn test_builder_fullscreen_custom() {
        let custom_func = |w: usize, h: usize| (w / 2, h / 2);
        let options = BoxenBuilder::new()
            .fullscreen(FullscreenMode::Custom(custom_func))
            .build();

        assert!(matches!(
            options.fullscreen,
            Some(FullscreenMode::Custom(_))
        ));
    }

    #[test]
    fn test_builder_render_success() {
        let result = BoxenBuilder::new()
            .border_style(BorderStyle::Single)
            .padding(1)
            .render("Test content");

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Test content"));
    }

    #[test]
    fn test_builder_render_with_validation_error() {
        let result = BoxenBuilder::new()
            .width(5) // Too small
            .padding(10) // Too large padding
            .render("Test");

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            BoxenError::InvalidDimensions { .. }
        ));
    }

    #[test]
    fn test_builder_validate_success() {
        let builder = BoxenBuilder::new().width(50).padding(2);

        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_builder_validate_error() {
        let builder = BoxenBuilder::new()
            .width(3) // Too small
            .padding(5); // Too large padding

        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_builder_complex_configuration() {
        let result = BoxenBuilder::new()
            .border_style(BorderStyle::Double)
            .padding((2, 4, 2, 4))
            .margin(1)
            .text_alignment(TextAlignment::Center)
            .title("Complex Box")
            .title_alignment(TitleAlignment::Center)
            .float(Float::Center)
            .width(60)
            .border_color("#0066cc")
            .background_color("white")
            .dim_border(false)
            .render("This is a complex box configuration test");

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("This is a complex box configuration test"));
    }

    #[test]
    fn test_builder_empty_text() {
        let result = BoxenBuilder::new().render("");

        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_multiline_text() {
        let text = "Line 1\nLine 2\nLine 3";
        let result = BoxenBuilder::new().padding(1).render(text);

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Line 1"));
        assert!(output.contains("Line 2"));
        assert!(output.contains("Line 3"));
    }

    #[test]
    fn test_builder_method_chaining_order_independence() {
        // Test that method order doesn't matter
        let options1 = BoxenBuilder::new()
            .width(50)
            .padding(2)
            .border_style(BorderStyle::Round)
            .build();

        let options2 = BoxenBuilder::new()
            .border_style(BorderStyle::Round)
            .padding(2)
            .width(50)
            .build();

        // Both should produce equivalent options
        assert!(matches!(options1.border_style, BorderStyle::Round));
        assert!(matches!(options2.border_style, BorderStyle::Round));
        assert_eq!(options1.width, Some(50));
        assert_eq!(options2.width, Some(50));
        assert_eq!(options1.padding.top, 2);
        assert_eq!(options2.padding.top, 2);
    }

    #[test]
    fn test_builder_overwrite_values() {
        // Test that later calls overwrite earlier ones
        let options = BoxenBuilder::new()
            .width(30)
            .width(50) // Should overwrite the 30
            .padding(1)
            .padding(3) // Should overwrite the 1
            .build();

        assert_eq!(options.width, Some(50));
        assert_eq!(options.padding.top, 3);
        assert_eq!(options.padding.right, 9); // 3x horizontal
    }

    // Test additional From implementations
    #[test]
    fn test_spacing_from_horizontal_vertical_tuple() {
        let spacing = Spacing::from((4, 2));
        assert_eq!(spacing.top, 2);
        assert_eq!(spacing.right, 4);
        assert_eq!(spacing.bottom, 2);
        assert_eq!(spacing.left, 4);
    }

    #[test]
    fn test_spacing_from_array_4() {
        let spacing = Spacing::from([1, 2, 3, 4]);
        assert_eq!(spacing.top, 1);
        assert_eq!(spacing.right, 2);
        assert_eq!(spacing.bottom, 3);
        assert_eq!(spacing.left, 4);
    }

    #[test]
    fn test_spacing_from_array_2() {
        let spacing = Spacing::from([6, 3]);
        assert_eq!(spacing.top, 3);
        assert_eq!(spacing.right, 6);
        assert_eq!(spacing.bottom, 3);
        assert_eq!(spacing.left, 6);
    }

    // Test convenience builder methods
    #[test]
    fn test_builder_spacing_convenience() {
        let options = BoxenBuilder::new().spacing(2).build();

        // Both padding and margin should be set
        assert_eq!(options.padding.top, 2);
        assert_eq!(options.padding.right, 6); // 3x horizontal
        assert_eq!(options.margin.top, 2);
        assert_eq!(options.margin.right, 6); // 3x horizontal
    }

    #[test]
    fn test_builder_colors_convenience() {
        let options = BoxenBuilder::new().colors("red", "#00ff00").build();

        if let Some(Color::Named(border_name)) = options.border_color {
            assert_eq!(border_name, "red");
        } else {
            panic!("Expected named border color");
        }

        if let Some(Color::Hex(bg_hex)) = options.background_color {
            assert_eq!(bg_hex, "#00ff00");
        } else {
            panic!("Expected hex background color");
        }
    }

    #[test]
    fn test_builder_size_convenience() {
        let options = BoxenBuilder::new().size(80, 25).build();

        assert_eq!(options.width, Some(80));
        assert_eq!(options.height, Some(25));
    }

    #[test]
    fn test_builder_center_all_convenience() {
        let options = BoxenBuilder::new().center_all().build();

        assert!(matches!(options.text_alignment, TextAlignment::Center));
        assert!(matches!(options.title_alignment, TitleAlignment::Center));
        assert!(matches!(options.float, Float::Center));
    }

    #[test]
    fn test_builder_with_array_spacing() {
        let options = BoxenBuilder::new()
            .padding([1, 2, 3, 4])
            .margin([5, 6])
            .build();

        assert_eq!(options.padding.top, 1);
        assert_eq!(options.padding.right, 2);
        assert_eq!(options.padding.bottom, 3);
        assert_eq!(options.padding.left, 4);

        assert_eq!(options.margin.top, 6);
        assert_eq!(options.margin.right, 5);
        assert_eq!(options.margin.bottom, 6);
        assert_eq!(options.margin.left, 5);
    }

    #[test]
    fn test_builder_with_tuple_spacing() {
        let options = BoxenBuilder::new()
            .padding((8, 4)) // horizontal, vertical
            .build();

        assert_eq!(options.padding.top, 4);
        assert_eq!(options.padding.right, 8);
        assert_eq!(options.padding.bottom, 4);
        assert_eq!(options.padding.left, 8);
    }

    #[test]
    fn test_builder_convenience_methods_chaining() {
        let result = BoxenBuilder::new()
            .spacing(1)
            .colors("blue", "white")
            .size(60, 10) // Use smaller height to avoid terminal size issues
            .center_all()
            .title("Centered Box")
            .render("This box uses all convenience methods");

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("This box uses all convenience methods"));
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

        // Handle fullscreen mode first
        if let Some(fullscreen_mode) = &self.fullscreen {
            return self.calculate_fullscreen_constraints(
                fullscreen_mode,
                terminal_width,
                terminal_height,
                border_width,
            );
        }

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

        // In fullscreen mode, expand content to fill available space
        let (final_content_width, final_content_height) = if self.fullscreen.is_some() {
            let max_content_width = self.calculate_max_content_width()?;
            let max_content_height = self.calculate_max_content_height()?;

            let expanded_width = max_content_width;
            let expanded_height = if let Some(max_height) = max_content_height {
                max_height
            } else {
                content_height // Use original height if no height constraint
            };

            (expanded_width, expanded_height)
        } else {
            (content_width, content_height)
        };

        // Calculate inner dimensions (content + padding)
        let inner_width = final_content_width + self.padding.horizontal();
        let inner_height = final_content_height + self.padding.vertical();

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
        // In fullscreen mode, ignore specified width and use terminal width as limit
        let width_limit = if self.fullscreen.is_some() {
            constraints.terminal_width // In fullscreen mode, use terminal width as limit
        } else if let Some(specified_width) = self.width {
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
            content_width: final_content_width,
            content_height: final_content_height,
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

    /// Calculate constraints for fullscreen mode
    fn calculate_fullscreen_constraints(
        &self,
        fullscreen_mode: &FullscreenMode,
        terminal_width: usize,
        terminal_height: Option<usize>,
        border_width: usize,
    ) -> BoxenResult<DimensionConstraints> {
        let (target_width, target_height) = match fullscreen_mode {
            FullscreenMode::Auto => {
                // Use full terminal dimensions
                (terminal_width, terminal_height)
            }
            FullscreenMode::Custom(func) => {
                // Use custom function to calculate dimensions
                let height = terminal_height.unwrap_or(24); // Fallback height
                let (custom_width, custom_height) = func(terminal_width, height);
                (custom_width, Some(custom_height))
            }
        };

        // In fullscreen mode, the target dimensions represent the total box size
        // We need to calculate the available space for content
        let max_width = if target_width > self.margin.horizontal() {
            target_width - self.margin.horizontal()
        } else {
            return Err(BoxenError::InvalidDimensions {
                width: Some(target_width),
                height: target_height,
            });
        };

        let max_height = if let Some(height) = target_height {
            if height > self.margin.vertical() {
                Some(height - self.margin.vertical())
            } else {
                return Err(BoxenError::InvalidDimensions {
                    width: Some(target_width),
                    height: Some(height),
                });
            }
        } else {
            None
        };

        // Validate that we have enough space for borders and padding
        if max_width < border_width + self.padding.horizontal() {
            return Err(BoxenError::InvalidDimensions {
                width: Some(target_width),
                height: target_height,
            });
        }

        if let Some(height) = max_height {
            let vertical_border_overhead = if matches!(self.border_style, BorderStyle::None) {
                0
            } else {
                2
            };
            if height < vertical_border_overhead + self.padding.vertical() {
                return Err(BoxenError::InvalidDimensions {
                    width: Some(target_width),
                    height: Some(height + self.margin.vertical()),
                });
            }
        }

        Ok(DimensionConstraints {
            max_width,
            max_height,
            terminal_width,
            terminal_height,
            border_width,
        })
    }

    /// Validate that the current options are compatible with terminal constraints
    pub fn validate_constraints(&self) -> BoxenResult<()> {
        let _constraints = self.calculate_constraints()?;
        let _max_content_width = self.calculate_max_content_width()?;
        let _max_content_height = self.calculate_max_content_height()?;
        Ok(())
    }
}
