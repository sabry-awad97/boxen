/// Color handling functionality for boxen
use crate::error::{BoxenError, BoxenResult};
use crate::options::Color;
use colored::{ColoredString, Colorize};

/// Parse and validate a color specification
pub fn parse_color(color: &Color) -> BoxenResult<colored::Color> {
    match color {
        Color::Named(name) => parse_named_color(name),
        Color::Hex(hex) => parse_hex_color(hex),
        Color::Rgb(r, g, b) => Ok(colored::Color::TrueColor {
            r: *r,
            g: *g,
            b: *b,
        }),
    }
}

/// Parse a named color into a colored::Color
pub fn parse_named_color(name: &str) -> BoxenResult<colored::Color> {
    let normalized = name.to_lowercase();
    match normalized.as_str() {
        // Standard terminal colors
        "black" => Ok(colored::Color::Black),
        "red" => Ok(colored::Color::Red),
        "green" => Ok(colored::Color::Green),
        "yellow" => Ok(colored::Color::Yellow),
        "blue" => Ok(colored::Color::Blue),
        "magenta" | "purple" => Ok(colored::Color::Magenta),
        "cyan" => Ok(colored::Color::Cyan),
        "white" => Ok(colored::Color::White),

        // Bright colors
        "bright_black" | "brightblack" | "gray" | "grey" => Ok(colored::Color::BrightBlack),
        "bright_red" | "brightred" => Ok(colored::Color::BrightRed),
        "bright_green" | "brightgreen" => Ok(colored::Color::BrightGreen),
        "bright_yellow" | "brightyellow" => Ok(colored::Color::BrightYellow),
        "bright_blue" | "brightblue" => Ok(colored::Color::BrightBlue),
        "bright_magenta" | "brightmagenta" | "bright_purple" | "brightpurple" => {
            Ok(colored::Color::BrightMagenta)
        }
        "bright_cyan" | "brightcyan" => Ok(colored::Color::BrightCyan),
        "bright_white" | "brightwhite" => Ok(colored::Color::BrightWhite),

        _ => Err(BoxenError::InvalidColor(format!(
            "Unknown color name: {}",
            name
        ))),
    }
}

/// Parse a hex color string into a colored::Color
pub fn parse_hex_color(hex: &str) -> BoxenResult<colored::Color> {
    let hex = hex.trim_start_matches('#');

    // Validate hex string length
    if hex.len() != 3 && hex.len() != 6 {
        return Err(BoxenError::InvalidColor(format!(
            "Invalid hex color format: #{}",
            hex
        )));
    }

    // Validate hex characters
    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(BoxenError::InvalidColor(format!(
            "Invalid hex color format: #{}",
            hex
        )));
    }

    let (r, g, b) = if hex.len() == 3 {
        // Short format: #RGB -> #RRGGBB
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
            .map_err(|_| BoxenError::InvalidColor(format!("Invalid hex color: #{}", hex)))?;
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
            .map_err(|_| BoxenError::InvalidColor(format!("Invalid hex color: #{}", hex)))?;
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
            .map_err(|_| BoxenError::InvalidColor(format!("Invalid hex color: #{}", hex)))?;
        (r, g, b)
    } else {
        // Long format: #RRGGBB
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| BoxenError::InvalidColor(format!("Invalid hex color: #{}", hex)))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| BoxenError::InvalidColor(format!("Invalid hex color: #{}", hex)))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| BoxenError::InvalidColor(format!("Invalid hex color: #{}", hex)))?;
        (r, g, b)
    };

    Ok(colored::Color::TrueColor { r, g, b })
}

/// Validate that a color specification is valid
pub fn validate_color(color: &Color) -> BoxenResult<()> {
    parse_color(color).map(|_| ())
}

/// Apply foreground color to text
pub fn apply_foreground_color(text: &str, color: &Color) -> BoxenResult<ColoredString> {
    let parsed_color = parse_color(color)?;
    Ok(text.color(parsed_color))
}

/// Apply background color to text
pub fn apply_background_color(text: &str, color: &Color) -> BoxenResult<ColoredString> {
    let parsed_color = parse_color(color)?;
    Ok(text.on_color(parsed_color))
}

/// Apply both foreground and background colors to text
pub fn apply_colors(
    text: &str,
    fg_color: Option<&Color>,
    bg_color: Option<&Color>,
) -> BoxenResult<ColoredString> {
    let mut styled = ColoredString::from(text);

    if let Some(fg) = fg_color {
        let parsed_fg = parse_color(fg)?;
        styled = styled.color(parsed_fg);
    }

    if let Some(bg) = bg_color {
        let parsed_bg = parse_color(bg)?;
        styled = styled.on_color(parsed_bg);
    }

    Ok(styled)
}

/// Apply dim styling to text (for dim borders)
pub fn apply_dim(text: &str) -> ColoredString {
    text.dimmed()
}

/// Apply color and dim styling to text
pub fn apply_color_with_dim(
    text: &str,
    color: Option<&Color>,
    dim: bool,
) -> BoxenResult<ColoredString> {
    let mut styled = if let Some(c) = color {
        apply_foreground_color(text, c)?
    } else {
        ColoredString::from(text)
    };

    if dim {
        styled = styled.dimmed();
    }

    Ok(styled)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::Color;

    #[test]
    fn test_parse_named_colors_basic() {
        assert!(parse_named_color("red").is_ok());
        assert!(parse_named_color("green").is_ok());
        assert!(parse_named_color("blue").is_ok());
        assert!(parse_named_color("black").is_ok());
        assert!(parse_named_color("white").is_ok());
        assert!(parse_named_color("yellow").is_ok());
        assert!(parse_named_color("magenta").is_ok());
        assert!(parse_named_color("cyan").is_ok());
    }

    #[test]
    fn test_parse_named_colors_bright() {
        assert!(parse_named_color("bright_red").is_ok());
        assert!(parse_named_color("brightred").is_ok());
        assert!(parse_named_color("bright_green").is_ok());
        assert!(parse_named_color("bright_blue").is_ok());
        assert!(parse_named_color("gray").is_ok());
        assert!(parse_named_color("grey").is_ok());
        assert!(parse_named_color("bright_black").is_ok());
        assert!(parse_named_color("brightblack").is_ok());
    }

    #[test]
    fn test_parse_named_colors_aliases() {
        assert!(parse_named_color("purple").is_ok());
        assert!(parse_named_color("bright_purple").is_ok());
        assert!(parse_named_color("brightpurple").is_ok());
    }

    #[test]
    fn test_parse_named_colors_case_insensitive() {
        assert!(parse_named_color("RED").is_ok());
        assert!(parse_named_color("Red").is_ok());
        assert!(parse_named_color("rEd").is_ok());
        assert!(parse_named_color("BRIGHT_BLUE").is_ok());
        assert!(parse_named_color("Bright_Blue").is_ok());
    }

    #[test]
    fn test_parse_named_colors_invalid() {
        assert!(parse_named_color("invalid_color").is_err());
        assert!(parse_named_color("").is_err());
        assert!(parse_named_color("orange").is_err());
        assert!(parse_named_color("pink").is_err());
    }

    #[test]
    fn test_parse_hex_colors_long_format() {
        // Valid 6-digit hex colors
        assert!(parse_hex_color("#FF0000").is_ok()); // Red
        assert!(parse_hex_color("#00FF00").is_ok()); // Green
        assert!(parse_hex_color("#0000FF").is_ok()); // Blue
        assert!(parse_hex_color("#FFFFFF").is_ok()); // White
        assert!(parse_hex_color("#000000").is_ok()); // Black
        assert!(parse_hex_color("#123456").is_ok()); // Random valid hex
        assert!(parse_hex_color("#ABCDEF").is_ok()); // With letters
        assert!(parse_hex_color("#abcdef").is_ok()); // Lowercase
    }

    #[test]
    fn test_parse_hex_colors_short_format() {
        // Valid 3-digit hex colors
        assert!(parse_hex_color("#F00").is_ok()); // Red
        assert!(parse_hex_color("#0F0").is_ok()); // Green
        assert!(parse_hex_color("#00F").is_ok()); // Blue
        assert!(parse_hex_color("#FFF").is_ok()); // White
        assert!(parse_hex_color("#000").is_ok()); // Black
        assert!(parse_hex_color("#123").is_ok()); // Random valid hex
        assert!(parse_hex_color("#ABC").is_ok()); // With letters
        assert!(parse_hex_color("#abc").is_ok()); // Lowercase
    }

    #[test]
    fn test_parse_hex_colors_without_hash() {
        // Should work without # prefix
        assert!(parse_hex_color("FF0000").is_ok());
        assert!(parse_hex_color("F00").is_ok());
        assert!(parse_hex_color("123456").is_ok());
        assert!(parse_hex_color("ABC").is_ok());
    }

    #[test]
    fn test_parse_hex_colors_invalid_length() {
        assert!(parse_hex_color("#FF").is_err()); // Too short
        assert!(parse_hex_color("#FFFF").is_err()); // Invalid length
        assert!(parse_hex_color("#FFFFF").is_err()); // Invalid length
        assert!(parse_hex_color("#FFFFFFF").is_err()); // Too long
        assert!(parse_hex_color("").is_err()); // Empty
        assert!(parse_hex_color("#").is_err()); // Just hash
    }

    #[test]
    fn test_parse_hex_colors_invalid_characters() {
        assert!(parse_hex_color("#GGGGGG").is_err()); // Invalid hex chars
        assert!(parse_hex_color("#FF00ZZ").is_err()); // Invalid hex chars
        assert!(parse_hex_color("#FF 000").is_err()); // Space
        assert!(parse_hex_color("#FF-000").is_err()); // Dash
        assert!(parse_hex_color("#FF.000").is_err()); // Dot
    }

    #[test]
    fn test_parse_hex_color_values() {
        // Test that hex parsing produces correct RGB values
        let red = parse_hex_color("#FF0000").unwrap();
        if let colored::Color::TrueColor { r, g, b } = red {
            assert_eq!(r, 255);
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        } else {
            panic!("Expected TrueColor");
        }

        let green = parse_hex_color("#00FF00").unwrap();
        if let colored::Color::TrueColor { r, g, b } = green {
            assert_eq!(r, 0);
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        } else {
            panic!("Expected TrueColor");
        }

        // Test short format expansion
        let short_red = parse_hex_color("#F00").unwrap();
        if let colored::Color::TrueColor { r, g, b } = short_red {
            assert_eq!(r, 255); // F -> FF
            assert_eq!(g, 0); // 0 -> 00
            assert_eq!(b, 0); // 0 -> 00
        } else {
            panic!("Expected TrueColor");
        }
    }

    #[test]
    fn test_parse_color_enum_variants() {
        // Test Color::Named
        let named = Color::Named("red".to_string());
        assert!(parse_color(&named).is_ok());

        // Test Color::Hex
        let hex = Color::Hex("#FF0000".to_string());
        assert!(parse_color(&hex).is_ok());

        // Test Color::Rgb
        let rgb = Color::Rgb(255, 0, 0);
        let parsed = parse_color(&rgb).unwrap();
        if let colored::Color::TrueColor { r, g, b } = parsed {
            assert_eq!(r, 255);
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        } else {
            panic!("Expected TrueColor");
        }
    }

    #[test]
    fn test_validate_color() {
        // Valid colors
        assert!(validate_color(&Color::Named("red".to_string())).is_ok());
        assert!(validate_color(&Color::Hex("#FF0000".to_string())).is_ok());
        assert!(validate_color(&Color::Rgb(255, 0, 0)).is_ok());

        // Invalid colors
        assert!(validate_color(&Color::Named("invalid".to_string())).is_err());
        assert!(validate_color(&Color::Hex("#GGGGGG".to_string())).is_err());
    }

    #[test]
    fn test_apply_foreground_color() {
        let color = Color::Named("red".to_string());
        let result = apply_foreground_color("test", &color);
        assert!(result.is_ok());

        // Test with invalid color
        let invalid_color = Color::Named("invalid".to_string());
        let result = apply_foreground_color("test", &invalid_color);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_background_color() {
        let color = Color::Named("blue".to_string());
        let result = apply_background_color("test", &color);
        assert!(result.is_ok());

        // Test with invalid color
        let invalid_color = Color::Named("invalid".to_string());
        let result = apply_background_color("test", &invalid_color);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_colors_both() {
        let fg_color = Color::Named("red".to_string());
        let bg_color = Color::Named("blue".to_string());
        let result = apply_colors("test", Some(&fg_color), Some(&bg_color));
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_colors_foreground_only() {
        let fg_color = Color::Named("red".to_string());
        let result = apply_colors("test", Some(&fg_color), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_colors_background_only() {
        let bg_color = Color::Named("blue".to_string());
        let result = apply_colors("test", None, Some(&bg_color));
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_colors_neither() {
        let result = apply_colors("test", None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_colors_invalid() {
        let invalid_color = Color::Named("invalid".to_string());
        let result = apply_colors("test", Some(&invalid_color), None);
        assert!(result.is_err());

        let result = apply_colors("test", None, Some(&invalid_color));
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_dim() {
        let result = apply_dim("test");
        // We can't easily test the actual ANSI codes, but we can verify it doesn't panic
        assert!(!result.to_string().is_empty());
    }

    #[test]
    fn test_apply_color_with_dim() {
        let color = Color::Named("red".to_string());

        // Test with color and dim
        let result = apply_color_with_dim("test", Some(&color), true);
        assert!(result.is_ok());

        // Test with color but no dim
        let result = apply_color_with_dim("test", Some(&color), false);
        assert!(result.is_ok());

        // Test with no color but dim
        let result = apply_color_with_dim("test", None, true);
        assert!(result.is_ok());

        // Test with no color and no dim
        let result = apply_color_with_dim("test", None, false);
        assert!(result.is_ok());

        // Test with invalid color
        let invalid_color = Color::Named("invalid".to_string());
        let result = apply_color_with_dim("test", Some(&invalid_color), false);
        assert!(result.is_err());
    }

    #[test]
    fn test_color_from_implementations() {
        // Test From<String> for Color
        let color1: Color = "red".to_string().into();
        assert!(matches!(color1, Color::Named(_)));

        let color2: Color = "#FF0000".to_string().into();
        assert!(matches!(color2, Color::Hex(_)));

        // Test From<&str> for Color
        let color3: Color = "blue".into();
        assert!(matches!(color3, Color::Named(_)));

        let color4: Color = "#00FF00".into();
        assert!(matches!(color4, Color::Hex(_)));

        // Test From<(u8, u8, u8)> for Color
        let color5: Color = (255, 0, 0).into();
        assert!(matches!(color5, Color::Rgb(255, 0, 0)));
    }

    #[test]
    fn test_edge_cases() {
        // Test empty string
        assert!(parse_named_color("").is_err());

        // Test whitespace
        assert!(parse_named_color(" red ").is_err()); // We don't trim in named colors

        // Test hex with whitespace (should fail)
        assert!(parse_hex_color(" #FF0000 ").is_err());

        // Test very long strings
        let long_name = "a".repeat(1000);
        assert!(parse_named_color(&long_name).is_err());

        // Test RGB edge values
        let rgb_max = Color::Rgb(255, 255, 255);
        assert!(parse_color(&rgb_max).is_ok());

        let rgb_min = Color::Rgb(0, 0, 0);
        assert!(parse_color(&rgb_min).is_ok());
    }

    #[test]
    fn test_hex_color_case_sensitivity() {
        // Both should work
        assert!(parse_hex_color("#ABCDEF").is_ok());
        assert!(parse_hex_color("#abcdef").is_ok());
        assert!(parse_hex_color("#AbCdEf").is_ok());

        // Short format too
        assert!(parse_hex_color("#ABC").is_ok());
        assert!(parse_hex_color("#abc").is_ok());
        assert!(parse_hex_color("#AbC").is_ok());
    }

    #[test]
    fn test_comprehensive_color_validation() {
        // Test a variety of valid colors
        let valid_colors = vec![
            Color::Named("red".to_string()),
            Color::Named("BLUE".to_string()),
            Color::Named("bright_green".to_string()),
            Color::Hex("#FF0000".to_string()),
            Color::Hex("#F00".to_string()),
            Color::Hex("00FF00".to_string()),
            Color::Rgb(128, 64, 192),
            Color::Rgb(0, 0, 0),
            Color::Rgb(255, 255, 255),
        ];

        for color in valid_colors {
            assert!(
                validate_color(&color).is_ok(),
                "Color should be valid: {:?}",
                color
            );
        }

        // Test a variety of invalid colors
        let invalid_colors = vec![
            Color::Named("invalid_color_name".to_string()),
            Color::Named("".to_string()),
            Color::Hex("#GGGGGG".to_string()),
            Color::Hex("#FF".to_string()),
            Color::Hex("#FFFFFFF".to_string()),
            Color::Hex("".to_string()),
        ];

        for color in invalid_colors {
            assert!(
                validate_color(&color).is_err(),
                "Color should be invalid: {:?}",
                color
            );
        }
    }
}
