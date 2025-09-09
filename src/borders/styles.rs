/// Predefined border styles and style utilities
use crate::error::BoxenError;
use crate::options::{BorderChars, BorderStyle};

impl BorderStyle {
    /// Get the BorderChars for this style
    pub fn get_chars(&self) -> Result<BorderChars, BoxenError> {
        match self {
            BorderStyle::None => Ok(BorderChars::uniform(' ')),
            BorderStyle::Single => Ok(BorderChars::single()),
            BorderStyle::Double => Ok(BorderChars::double()),
            BorderStyle::Round => Ok(BorderChars::round()),
            BorderStyle::Bold => Ok(BorderChars::bold()),
            BorderStyle::SingleDouble => Ok(BorderChars::single_double()),
            BorderStyle::DoubleSingle => Ok(BorderChars::double_single()),
            BorderStyle::Classic => Ok(BorderChars::classic()),
            BorderStyle::Custom(chars) => {
                chars.validate().map_err(|msg| {
                    BoxenError::invalid_border_style(
                        format!("Custom border validation failed: {}", msg),
                        vec![crate::error::ErrorRecommendation::suggestion_only(
                            "Border validation failed".to_string(),
                            "Check that all border characters are valid and visible".to_string(),
                        )],
                    )
                })?;
                Ok(chars.clone())
            }
        }
    }

    /// Check if this border style is visible (not None)
    pub fn is_visible(&self) -> bool {
        !matches!(self, BorderStyle::None)
    }

    /// Get the display name of this border style
    pub fn name(&self) -> &'static str {
        match self {
            BorderStyle::None => "none",
            BorderStyle::Single => "single",
            BorderStyle::Double => "double",
            BorderStyle::Round => "round",
            BorderStyle::Bold => "bold",
            BorderStyle::SingleDouble => "singleDouble",
            BorderStyle::DoubleSingle => "doubleSingle",
            BorderStyle::Classic => "classic",
            BorderStyle::Custom(_) => "custom",
        }
    }

    /// Parse a border style from a string name
    pub fn from_name(name: &str) -> Result<BorderStyle, BoxenError> {
        match name.to_lowercase().as_str() {
            "none" => Ok(BorderStyle::None),
            "single" => Ok(BorderStyle::Single),
            "double" => Ok(BorderStyle::Double),
            "round" => Ok(BorderStyle::Round),
            "bold" => Ok(BorderStyle::Bold),
            "singledouble" | "single_double" => Ok(BorderStyle::SingleDouble),
            "doublesingle" | "double_single" => Ok(BorderStyle::DoubleSingle),
            "classic" => Ok(BorderStyle::Classic),
            _ => Err(BoxenError::invalid_border_style(
                format!(
                    "Unknown border style: '{}'. Valid styles are: none, single, double, round, bold, singleDouble, doubleSingle, classic",
                    name
                ),
                vec![
                    crate::error::ErrorRecommendation::suggestion_only(
                        "Unknown border style".to_string(),
                        "Use one of the predefined styles: single, double, round, bold, etc."
                            .to_string(),
                    ),
                    crate::error::ErrorRecommendation::with_auto_fix(
                        "Use default style".to_string(),
                        "Try using the default single border style".to_string(),
                        "BorderStyle::Single".to_string(),
                    ),
                ],
            )),
        }
    }

    /// Get all available predefined border style names
    pub fn available_styles() -> Vec<&'static str> {
        vec![
            "none",
            "single",
            "double",
            "round",
            "bold",
            "singleDouble",
            "doubleSingle",
            "classic",
        ]
    }

    /// Create a custom border style with validation
    pub fn custom(chars: BorderChars) -> Result<BorderStyle, BoxenError> {
        chars.validate().map_err(|msg| {
            BoxenError::invalid_border_style(
                format!("Custom border validation failed: {}", msg),
                vec![crate::error::ErrorRecommendation::suggestion_only(
                    "Border validation failed".to_string(),
                    "Ensure all border characters are valid and visible".to_string(),
                )],
            )
        })?;
        Ok(BorderStyle::Custom(chars))
    }
}

/// Utility functions for working with border styles
pub struct BorderStyleUtils;

impl BorderStyleUtils {
    /// Get the effective border width for a style (0 for None, 1 for others)
    pub fn get_border_width(style: &BorderStyle) -> usize {
        match style {
            BorderStyle::None => 0,
            _ => 1,
        }
    }

    /// Check if two border styles are equivalent
    pub fn styles_equal(a: &BorderStyle, b: &BorderStyle) -> bool {
        match (a, b) {
            (BorderStyle::None, BorderStyle::None) => true,
            (BorderStyle::Single, BorderStyle::Single) => true,
            (BorderStyle::Double, BorderStyle::Double) => true,
            (BorderStyle::Round, BorderStyle::Round) => true,
            (BorderStyle::Bold, BorderStyle::Bold) => true,
            (BorderStyle::SingleDouble, BorderStyle::SingleDouble) => true,
            (BorderStyle::DoubleSingle, BorderStyle::DoubleSingle) => true,
            (BorderStyle::Classic, BorderStyle::Classic) => true,
            (BorderStyle::Custom(a_chars), BorderStyle::Custom(b_chars)) => {
                a_chars.top_left == b_chars.top_left
                    && a_chars.top_right == b_chars.top_right
                    && a_chars.bottom_left == b_chars.bottom_left
                    && a_chars.bottom_right == b_chars.bottom_right
                    && a_chars.left == b_chars.left
                    && a_chars.right == b_chars.right
                    && a_chars.top == b_chars.top
                    && a_chars.bottom == b_chars.bottom
            }
            _ => false,
        }
    }

    /// Get a preview string showing what the border style looks like
    pub fn preview(style: &BorderStyle) -> Result<String, BoxenError> {
        let chars = style.get_chars()?;
        if matches!(style, BorderStyle::None) {
            return Ok("(no border)".to_string());
        }

        Ok(format!(
            "{}{}{}\n{} {}\n{}{}{}",
            chars.top_left,
            chars.top,
            chars.top_right,
            chars.left,
            chars.right,
            chars.bottom_left,
            chars.bottom,
            chars.bottom_right
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_style_get_chars() {
        assert!(BorderStyle::Single.get_chars().is_ok());
        assert!(BorderStyle::Double.get_chars().is_ok());
        assert!(BorderStyle::Round.get_chars().is_ok());
        assert!(BorderStyle::Bold.get_chars().is_ok());
        assert!(BorderStyle::SingleDouble.get_chars().is_ok());
        assert!(BorderStyle::DoubleSingle.get_chars().is_ok());
        assert!(BorderStyle::Classic.get_chars().is_ok());
        assert!(BorderStyle::None.get_chars().is_ok());
    }

    #[test]
    fn test_border_style_visibility() {
        assert!(!BorderStyle::None.is_visible());
        assert!(BorderStyle::Single.is_visible());
        assert!(BorderStyle::Double.is_visible());
        assert!(BorderStyle::Round.is_visible());
        assert!(BorderStyle::Bold.is_visible());
        assert!(BorderStyle::Classic.is_visible());
    }

    #[test]
    fn test_border_style_names() {
        assert_eq!(BorderStyle::None.name(), "none");
        assert_eq!(BorderStyle::Single.name(), "single");
        assert_eq!(BorderStyle::Double.name(), "double");
        assert_eq!(BorderStyle::Round.name(), "round");
        assert_eq!(BorderStyle::Bold.name(), "bold");
        assert_eq!(BorderStyle::SingleDouble.name(), "singleDouble");
        assert_eq!(BorderStyle::DoubleSingle.name(), "doubleSingle");
        assert_eq!(BorderStyle::Classic.name(), "classic");
        assert_eq!(BorderStyle::Custom(BorderChars::single()).name(), "custom");
    }

    #[test]
    fn test_border_style_from_name() {
        assert!(matches!(
            BorderStyle::from_name("single").unwrap(),
            BorderStyle::Single
        ));
        assert!(matches!(
            BorderStyle::from_name("double").unwrap(),
            BorderStyle::Double
        ));
        assert!(matches!(
            BorderStyle::from_name("round").unwrap(),
            BorderStyle::Round
        ));
        assert!(matches!(
            BorderStyle::from_name("bold").unwrap(),
            BorderStyle::Bold
        ));
        assert!(matches!(
            BorderStyle::from_name("classic").unwrap(),
            BorderStyle::Classic
        ));
        assert!(matches!(
            BorderStyle::from_name("none").unwrap(),
            BorderStyle::None
        ));

        // Test case insensitive
        assert!(matches!(
            BorderStyle::from_name("SINGLE").unwrap(),
            BorderStyle::Single
        ));

        // Test underscore variants
        assert!(matches!(
            BorderStyle::from_name("single_double").unwrap(),
            BorderStyle::SingleDouble
        ));
        assert!(matches!(
            BorderStyle::from_name("double_single").unwrap(),
            BorderStyle::DoubleSingle
        ));
    }

    #[test]
    fn test_border_style_from_name_invalid() {
        let result = BorderStyle::from_name("invalid");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Unknown border style")
        );
    }

    #[test]
    fn test_available_styles() {
        let styles = BorderStyle::available_styles();
        assert!(styles.contains(&"single"));
        assert!(styles.contains(&"double"));
        assert!(styles.contains(&"round"));
        assert!(styles.contains(&"bold"));
        assert!(styles.contains(&"classic"));
        assert!(styles.contains(&"none"));
        assert_eq!(styles.len(), 8);
    }

    #[test]
    fn test_custom_border_style() {
        let chars = BorderChars::single();
        let style = BorderStyle::custom(chars).unwrap();
        assert!(matches!(style, BorderStyle::Custom(_)));
    }

    #[test]
    fn test_custom_border_style_validation_error() {
        let chars = BorderChars {
            top_left: ' ', // Invalid whitespace
            ..BorderChars::single()
        };
        let result = BorderStyle::custom(chars);
        assert!(result.is_err());
    }

    #[test]
    fn test_border_width() {
        assert_eq!(BorderStyleUtils::get_border_width(&BorderStyle::None), 0);
        assert_eq!(BorderStyleUtils::get_border_width(&BorderStyle::Single), 1);
        assert_eq!(BorderStyleUtils::get_border_width(&BorderStyle::Double), 1);
        assert_eq!(BorderStyleUtils::get_border_width(&BorderStyle::Round), 1);
        assert_eq!(BorderStyleUtils::get_border_width(&BorderStyle::Bold), 1);
        assert_eq!(BorderStyleUtils::get_border_width(&BorderStyle::Classic), 1);
    }

    #[test]
    fn test_styles_equal() {
        assert!(BorderStyleUtils::styles_equal(
            &BorderStyle::Single,
            &BorderStyle::Single
        ));
        assert!(BorderStyleUtils::styles_equal(
            &BorderStyle::None,
            &BorderStyle::None
        ));
        assert!(!BorderStyleUtils::styles_equal(
            &BorderStyle::Single,
            &BorderStyle::Double
        ));

        let custom1 = BorderStyle::Custom(BorderChars::single());
        let custom2 = BorderStyle::Custom(BorderChars::single());
        let custom3 = BorderStyle::Custom(BorderChars::double());

        assert!(BorderStyleUtils::styles_equal(&custom1, &custom2));
        assert!(!BorderStyleUtils::styles_equal(&custom1, &custom3));
    }

    #[test]
    fn test_border_preview() {
        let preview = BorderStyleUtils::preview(&BorderStyle::Single).unwrap();
        assert!(preview.contains('┌'));
        assert!(preview.contains('┐'));
        assert!(preview.contains('└'));
        assert!(preview.contains('┘'));

        let none_preview = BorderStyleUtils::preview(&BorderStyle::None).unwrap();
        assert_eq!(none_preview, "(no border)");
    }

    #[test]
    fn test_custom_border_get_chars() {
        let chars = BorderChars::single();
        let style = BorderStyle::Custom(chars.clone());
        let retrieved_chars = style.get_chars().unwrap();

        assert_eq!(retrieved_chars.top_left, chars.top_left);
        assert_eq!(retrieved_chars.top_right, chars.top_right);
        assert_eq!(retrieved_chars.bottom_left, chars.bottom_left);
        assert_eq!(retrieved_chars.bottom_right, chars.bottom_right);
    }

    #[test]
    fn test_custom_border_validation_in_get_chars() {
        let invalid_chars = BorderChars {
            top_left: ' ', // Invalid whitespace
            ..BorderChars::single()
        };
        let style = BorderStyle::Custom(invalid_chars);
        let result = style.get_chars();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Custom border validation failed")
        );
    }
}
