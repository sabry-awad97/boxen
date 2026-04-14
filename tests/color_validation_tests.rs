//! Tests for Color::validated() fallible constructor
//!
//! This test suite verifies early validation of color specifications using the
//! validated() method, allowing errors to be caught at construction time
//! rather than at render time.

use boxen::{BoxenError, Color};

// ============================================================================
// Valid Color Tests
// ============================================================================

#[test]
fn test_validated_valid_named_colors() {
    // Standard colors
    assert!(Color::validated("red").is_ok());
    assert!(Color::validated("green").is_ok());
    assert!(Color::validated("blue").is_ok());
    assert!(Color::validated("black").is_ok());
    assert!(Color::validated("white").is_ok());
    assert!(Color::validated("yellow").is_ok());
    assert!(Color::validated("magenta").is_ok());
    assert!(Color::validated("cyan").is_ok());
}

#[test]
fn test_validated_valid_bright_colors() {
    // Bright colors with underscore
    assert!(Color::validated("bright_red").is_ok());
    assert!(Color::validated("bright_green").is_ok());
    assert!(Color::validated("bright_blue").is_ok());
    assert!(Color::validated("bright_yellow").is_ok());
    assert!(Color::validated("bright_magenta").is_ok());
    assert!(Color::validated("bright_cyan").is_ok());
    assert!(Color::validated("bright_white").is_ok());
    assert!(Color::validated("bright_black").is_ok());

    // Also test compact format
    assert!(Color::validated("brightred").is_ok());
    assert!(Color::validated("brightgreen").is_ok());
}

#[test]
fn test_validated_valid_hex_colors() {
    // 6-digit hex colors
    assert!(Color::validated("#FF0000").is_ok());
    assert!(Color::validated("#00FF00").is_ok());
    assert!(Color::validated("#0000FF").is_ok());
    assert!(Color::validated("#FFFFFF").is_ok());
    assert!(Color::validated("#000000").is_ok());
    assert!(Color::validated("#123456").is_ok());
    assert!(Color::validated("#ABCDEF").is_ok());
    assert!(Color::validated("#abcdef").is_ok());
}

#[test]
fn test_validated_returns_correct_variant() {
    // Named colors should return Color::Named
    match Color::validated("red").unwrap() {
        Color::Named(name) => assert_eq!(name, "red"),
        _ => panic!("Expected Color::Named variant"),
    }

    // Hex colors should return Color::Hex
    match Color::validated("#FF0000").unwrap() {
        Color::Hex(hex) => assert_eq!(hex, "#FF0000"),
        _ => panic!("Expected Color::Hex variant"),
    }
}

// ============================================================================
// Invalid Hex Color Tests
// ============================================================================

#[test]
fn test_validated_invalid_hex_length() {
    // Too short
    let result = Color::validated("#FF");
    assert!(result.is_err());
    if let Err(BoxenError::InvalidColor { message, .. }) = result {
        assert!(message.contains("Invalid hex color format"));
    } else {
        panic!("Expected InvalidColor error");
    }

    // Too long
    let result = Color::validated("#FFFFFFF");
    assert!(result.is_err());

    // Invalid length (4 or 5 digits)
    assert!(Color::validated("#FFFF").is_err());
    assert!(Color::validated("#FFFFF").is_err());
}

#[test]
fn test_validated_invalid_hex_characters() {
    let result = Color::validated("#GGGGGG");
    assert!(result.is_err());
    if let Err(BoxenError::InvalidColor {
        message,
        color_value,
        recommendations,
    }) = result
    {
        assert!(message.contains("Invalid hex color format"));
        assert_eq!(color_value, "#GGGGGG");
        assert!(!recommendations.is_empty());
        assert!(
            recommendations[0]
                .suggestion
                .contains("Hex colors can only contain")
        );
    } else {
        panic!("Expected InvalidColor error with recommendations");
    }
}

#[test]
fn test_validated_hex_without_hash() {
    // Should fail - we require the # prefix for clarity
    let result = Color::validated("FF0000");
    assert!(result.is_err());
}

// ============================================================================
// Invalid Named Color Tests
// ============================================================================

#[test]
fn test_validated_invalid_named_color() {
    let result = Color::validated("invalid_color");
    assert!(result.is_err());
    if let Err(BoxenError::InvalidColor {
        message,
        color_value,
        recommendations,
    }) = result
    {
        assert!(message.contains("Unknown color name"));
        assert_eq!(color_value, "invalid_color");
        assert!(!recommendations.is_empty());
    } else {
        panic!("Expected InvalidColor error");
    }
}

#[test]
fn test_validated_empty_string() {
    let result = Color::validated("");
    assert!(result.is_err());
}

#[test]
fn test_validated_common_typos() {
    // Common typos should fail with helpful errors
    assert!(Color::validated("rad").is_err()); // typo for "red"
    assert!(Color::validated("blu").is_err()); // typo for "blue"
    assert!(Color::validated("grean").is_err()); // typo for "green"
}

// ============================================================================
// Error Recommendation Tests
// ============================================================================

#[test]
fn test_validated_error_has_recommendations() {
    let result = Color::validated("orange");
    assert!(result.is_err());

    if let Err(e) = result {
        let recommendations = e.recommendations();
        assert!(
            !recommendations.is_empty(),
            "Error should include recommendations"
        );

        // Check that recommendations are helpful
        let has_suggestion = recommendations
            .iter()
            .any(|r| r.suggestion.contains("red") || r.suggestion.contains("blue"));
        assert!(
            has_suggestion,
            "Recommendations should suggest valid colors"
        );
    }
}

#[test]
fn test_validated_hex_error_has_format_suggestion() {
    let result = Color::validated("#GGG");
    assert!(result.is_err());

    if let Err(e) = result {
        let recommendations = e.recommendations();
        let has_format_help = recommendations
            .iter()
            .any(|r| r.suggestion.contains("#FF0000") || r.suggestion.contains("6 characters"));
        assert!(
            has_format_help,
            "Hex errors should explain the correct format"
        );
    }
}

// ============================================================================
// Case Sensitivity Tests
// ============================================================================

#[test]
fn test_validated_case_insensitive_named_colors() {
    // Named colors should be case-insensitive
    assert!(Color::validated("RED").is_ok());
    assert!(Color::validated("Red").is_ok());
    assert!(Color::validated("rEd").is_ok());
    assert!(Color::validated("BRIGHT_BLUE").is_ok());
}

#[test]
fn test_validated_case_sensitive_hex() {
    // Hex colors should accept both cases
    assert!(Color::validated("#ABCDEF").is_ok());
    assert!(Color::validated("#abcdef").is_ok());
    assert!(Color::validated("#AbCdEf").is_ok());
}

// ============================================================================
// Comparison with From<&str> Tests
// ============================================================================

#[test]
fn test_from_str_accepts_invalid_colors() {
    // The From<&str> impl should accept any string without validation
    let color: Color = "invalid_color".into();
    match color {
        Color::Named(name) => assert_eq!(name, "invalid_color"),
        _ => panic!("Expected Color::Named variant"),
    }

    // But validated should reject it
    assert!(Color::validated("invalid_color").is_err());
}

#[test]
fn test_from_string_accepts_any_string() {
    // The From<String> impl should accept any string without validation
    let color: Color = "invalid_color".to_string().into();
    match color {
        Color::Named(name) => assert_eq!(name, "invalid_color"),
        _ => panic!("Expected Color::Named variant"),
    }

    // But validated should reject it
    assert!(Color::validated("invalid_color").is_err());
}

#[test]
fn test_from_str_vs_validated_valid_colors() {
    // For valid colors, both should work
    let from_impl: Color = "red".into();
    let validated = Color::validated("red").unwrap();

    // Both should produce the same result
    match (from_impl, validated) {
        (Color::Named(n1), Color::Named(n2)) => assert_eq!(n1, n2),
        _ => panic!("Both should produce Color::Named"),
    }
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_validated_whitespace() {
    // Leading/trailing whitespace should fail
    assert!(Color::validated(" red").is_err());
    assert!(Color::validated("red ").is_err());
    assert!(Color::validated(" red ").is_err());
    assert!(Color::validated(" #FF0000").is_err());
    assert!(Color::validated("#FF0000 ").is_err());
}

#[test]
fn test_validated_special_characters() {
    // Special characters should fail (except # for hex)
    assert!(Color::validated("red!").is_err());
    assert!(Color::validated("@blue").is_err());
    assert!(Color::validated("#FF-00-00").is_err());
}
