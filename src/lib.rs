/// A Rust implementation of the boxen library for drawing styled boxes around text in terminals
pub mod borders;
pub mod boxen;
pub mod color;
pub mod error;
pub mod options;
pub mod terminal;
pub mod text;
pub mod validation;

#[cfg(test)]
mod error_tests;

// Re-export main types and functions
pub use boxen::boxen;
pub use error::{BoxenError, BoxenResult, ErrorRecommendation};
pub use options::{
    BorderChars, BorderStyle, BoxenBuilder, BoxenOptions, Color, DimensionConstraints, Float,
    FullscreenMode, LayoutDimensions, Spacing, TextAlignment, TitleAlignment,
};
pub use validation::{
    MinimumDimensions, ValidationResult, auto_adjust_options, calculate_minimum_dimensions,
    suggest_optimal_dimensions, validate_configuration,
};

/// Create a new BoxenBuilder for fluent configuration
pub fn builder() -> BoxenBuilder {
    BoxenBuilder::new()
}

/// Create a simple box with default single border
pub fn simple_box<S: AsRef<str>>(text: S) -> String {
    let text_ref = text.as_ref();
    boxen(text_ref, None).unwrap_or_else(|_| text_ref.to_string())
}

/// Create a box with double border style
pub fn double_box<S: AsRef<str>>(text: S) -> String {
    let text_ref = text.as_ref();
    let options = BoxenOptions {
        border_style: BorderStyle::Double,
        ..Default::default()
    };
    boxen(text_ref, Some(options)).unwrap_or_else(|_| text_ref.to_string())
}

/// Create a box with round border style
pub fn round_box<S: AsRef<str>>(text: S) -> String {
    let text_ref = text.as_ref();
    let options = BoxenOptions {
        border_style: BorderStyle::Round,
        ..Default::default()
    };
    boxen(text_ref, Some(options)).unwrap_or_else(|_| text_ref.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_boxen() {
        let result = boxen("Hello", None);
        assert!(result.is_ok());
        let box_str = result.unwrap();
        assert!(box_str.contains("Hello"));
    }

    #[test]
    fn test_builder_pattern() {
        let builder = builder();
        assert!(builder.render("Test").is_ok());
    }

    #[test]
    fn test_builder_fluent_interface() {
        let result = builder()
            .border_style(BorderStyle::Double)
            .padding(2)
            .margin(1)
            .text_alignment(TextAlignment::Center)
            .title("Builder Test")
            .width(50)
            .border_color("blue")
            .render("Testing fluent interface");

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Testing fluent interface"));
        assert!(output.contains("Builder Test"));
    }

    #[test]
    fn test_builder_validation() {
        // Test that builder validates configuration
        let result = builder()
            .width(5) // Too small
            .padding(10) // Too large
            .render("Test");

        assert!(result.is_err());
    }

    #[test]
    fn test_builder_convenience_methods() {
        let result = builder()
            .spacing(1) // Sets both padding and margin
            .colors("red", "white") // Sets both border and background color
            .size(50, 8) // Use wider box to avoid text wrapping
            .center_all() // Centers text, title, and float
            .title("Convenience Test")
            .render("Testing convenience methods");

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Testing convenience methods"));
        assert!(output.contains("Convenience Test"));
    }

    #[test]
    fn test_convenience_functions() {
        assert!(simple_box("Test").contains("Test"));
        assert!(double_box("Test").contains("Test"));
        assert!(round_box("Test").contains("Test"));
    }

    #[test]
    fn test_spacing_from_usize() {
        let spacing = Spacing::from(2);
        assert_eq!(spacing.top, 2);
        assert_eq!(spacing.right, 6); // 3x horizontal
        assert_eq!(spacing.bottom, 2);
        assert_eq!(spacing.left, 6); // 3x horizontal
    }

    #[test]
    fn test_spacing_from_tuple() {
        let spacing = Spacing::from((1, 2, 3, 4));
        assert_eq!(spacing.top, 1);
        assert_eq!(spacing.right, 2);
        assert_eq!(spacing.bottom, 3);
        assert_eq!(spacing.left, 4);
    }

    #[test]
    fn test_color_from_string() {
        let color = Color::from("red");
        matches!(color, Color::Named(_));

        let hex_color = Color::from("#ff0000");
        matches!(hex_color, Color::Hex(_));
    }

    #[test]
    fn test_color_from_rgb() {
        let color = Color::from((255, 0, 0));
        matches!(color, Color::Rgb(255, 0, 0));
    }
}
