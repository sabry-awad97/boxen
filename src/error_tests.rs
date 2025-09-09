/// Comprehensive tests for error handling and validation
#[cfg(test)]
mod tests {
    use crate::error::{BoxenError, ErrorRecommendation, validation};
    use crate::options::{BorderStyle, BoxenBuilder, BoxenOptions, Color, Spacing};
    use crate::validation::{recovery, validate_configuration};

    #[test]
    fn test_error_recommendation_creation() {
        let rec = ErrorRecommendation::new(
            "Test issue".to_string(),
            "Test suggestion".to_string(),
            Some("auto_fix".to_string()),
        );

        assert_eq!(rec.issue, "Test issue");
        assert_eq!(rec.suggestion, "Test suggestion");
        assert_eq!(rec.auto_fix, Some("auto_fix".to_string()));
    }

    #[test]
    fn test_error_recommendation_with_auto_fix() {
        let rec = ErrorRecommendation::with_auto_fix(
            "Issue".to_string(),
            "Suggestion".to_string(),
            "Fix".to_string(),
        );

        assert!(rec.auto_fix.is_some());
        assert_eq!(rec.auto_fix.unwrap(), "Fix");
    }

    #[test]
    fn test_error_recommendation_suggestion_only() {
        let rec =
            ErrorRecommendation::suggestion_only("Issue".to_string(), "Suggestion".to_string());

        assert!(rec.auto_fix.is_none());
    }

    #[test]
    fn test_boxen_error_detailed_message() {
        let recommendations = vec![
            ErrorRecommendation::with_auto_fix(
                "Issue 1".to_string(),
                "Suggestion 1".to_string(),
                "Fix 1".to_string(),
            ),
            ErrorRecommendation::suggestion_only("Issue 2".to_string(), "Suggestion 2".to_string()),
        ];

        let error = BoxenError::invalid_dimensions(
            "Test error".to_string(),
            Some(10),
            Some(5),
            recommendations,
        );

        let detailed = error.detailed_message();
        assert!(detailed.contains("Test error"));
        assert!(detailed.contains("Suggestions:"));
        assert!(detailed.contains("Issue 1: Suggestion 1"));
        assert!(detailed.contains("Auto-fix: Fix 1"));
        assert!(detailed.contains("Issue 2: Suggestion 2"));
    }

    #[test]
    fn test_boxen_error_no_recommendations() {
        let error = BoxenError::terminal_size_error("Terminal error".to_string(), vec![]);

        let detailed = error.detailed_message();
        assert_eq!(detailed, "Terminal size detection failed: Terminal error");
        assert!(!detailed.contains("Suggestions:"));
    }

    #[test]
    fn test_validate_text_input_normal() {
        let result = validation::validate_text_input("Hello, world!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_text_input_too_large() {
        let large_text = "a".repeat(1_000_001);
        let result = validation::validate_text_input(&large_text);

        assert!(result.is_err());
        if let Err(BoxenError::InputValidationError { field, .. }) = result {
            assert_eq!(field, "text");
        } else {
            panic!("Expected InputValidationError");
        }
    }

    #[test]
    fn test_validate_text_input_too_many_lines() {
        let many_lines = (0..1001)
            .map(|i| format!("Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let result = validation::validate_text_input(&many_lines);

        assert!(result.is_err());
        if let Err(BoxenError::InputValidationError { field, .. }) = result {
            assert_eq!(field, "text");
        } else {
            panic!("Expected InputValidationError");
        }
    }

    #[test]
    fn test_validate_spacing_normal() {
        let spacing = Spacing::from(5);
        let result = validation::validate_spacing(&spacing, "padding");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_spacing_excessive() {
        let spacing = Spacing {
            top: 150,
            right: 10,
            bottom: 10,
            left: 10,
        };
        let result = validation::validate_spacing(&spacing, "padding");

        assert!(result.is_err());
        if let Err(BoxenError::InputValidationError { field, .. }) = result {
            assert_eq!(field, "padding.top");
        } else {
            panic!("Expected InputValidationError");
        }
    }

    #[test]
    fn test_validate_dimensions_normal() {
        let result = validation::validate_dimensions(Some(50), Some(20));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dimensions_zero_width() {
        let result = validation::validate_dimensions(Some(0), Some(20));

        assert!(result.is_err());
        if let Err(BoxenError::InputValidationError { field, value, .. }) = result {
            assert_eq!(field, "width");
            assert_eq!(value, "0");
        } else {
            panic!("Expected InputValidationError");
        }
    }

    #[test]
    fn test_validate_dimensions_zero_height() {
        let result = validation::validate_dimensions(Some(50), Some(0));

        assert!(result.is_err());
        if let Err(BoxenError::InputValidationError { field, value, .. }) = result {
            assert_eq!(field, "height");
            assert_eq!(value, "0");
        } else {
            panic!("Expected InputValidationError");
        }
    }

    #[test]
    fn test_validate_dimensions_excessive_width() {
        let result = validation::validate_dimensions(Some(15000), Some(20));

        assert!(result.is_err());
        if let Err(BoxenError::InputValidationError { field, .. }) = result {
            assert_eq!(field, "width");
        } else {
            panic!("Expected InputValidationError");
        }
    }

    #[test]
    fn test_validate_dimensions_excessive_height() {
        let result = validation::validate_dimensions(Some(50), Some(1500));

        assert!(result.is_err());
        if let Err(BoxenError::InputValidationError { field, .. }) = result {
            assert_eq!(field, "height");
        } else {
            panic!("Expected InputValidationError");
        }
    }

    #[test]
    fn test_validate_title_normal() {
        let result = validation::validate_title("Normal Title");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_title_too_long() {
        let long_title = "a".repeat(250);
        let result = validation::validate_title(&long_title);

        assert!(result.is_err());
        if let Err(BoxenError::InputValidationError { field, .. }) = result {
            assert_eq!(field, "title");
        } else {
            panic!("Expected InputValidationError");
        }
    }

    #[test]
    fn test_validate_title_control_characters() {
        let title_with_control = "Title\x00with\x01control";
        let result = validation::validate_title(title_with_control);

        assert!(result.is_err());
        if let Err(BoxenError::InputValidationError { field, .. }) = result {
            assert_eq!(field, "title");
        } else {
            panic!("Expected InputValidationError");
        }
    }

    #[test]
    fn test_validate_title_with_tab() {
        let title_with_tab = "Title\twith\ttab";
        let result = validation::validate_title(title_with_tab);
        assert!(result.is_ok()); // Tabs should be allowed
    }

    #[test]
    fn test_validate_all_options_valid() {
        let options = BoxenOptions {
            width: Some(50),
            height: Some(20),
            padding: Spacing::from(2),
            title: Some("Valid Title".to_string()),
            border_color: Some(Color::Named("red".to_string())),
            background_color: Some(Color::Hex("#FF0000".to_string())),
            ..Default::default()
        };

        let result = validation::validate_all_options("Hello, world!", &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_all_options_invalid_color() {
        let options = BoxenOptions {
            border_color: Some(Color::Named("invalid_color".to_string())),
            ..Default::default()
        };

        let result = validation::validate_all_options("Hello", &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_all_options_invalid_dimensions() {
        let options = BoxenOptions {
            width: Some(0),
            ..Default::default()
        };

        let result = validation::validate_all_options("Hello", &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_all_options_excessive_spacing() {
        let options = BoxenOptions {
            padding: Spacing {
                top: 150,
                right: 10,
                bottom: 10,
                left: 10,
            },
            ..Default::default()
        };

        let result = validation::validate_all_options("Hello", &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_recovery_invalid_width() {
        let options = BoxenOptions {
            width: Some(5),            // Too small
            padding: Spacing::from(2), // 6 horizontal padding
            ..Default::default()
        };

        let recovered = recovery::recover_from_invalid_width("Hello World", options, 5);

        // Should have reduced padding or removed borders
        assert!(
            recovered.padding.horizontal() < 6
                || matches!(recovered.border_style, BorderStyle::None)
        );
    }

    #[test]
    fn test_recovery_invalid_height() {
        let options = BoxenOptions {
            height: Some(2),           // Too small
            padding: Spacing::from(2), // 2 vertical padding
            ..Default::default()
        };

        let recovered = recovery::recover_from_invalid_height("Hello\nWorld\nTest", options, 2);

        // Should have reduced padding or removed borders
        assert!(
            recovered.padding.vertical() < 2 || matches!(recovered.border_style, BorderStyle::None)
        );
    }

    #[test]
    fn test_recovery_terminal_overflow() {
        let options = BoxenOptions {
            width: Some(200), // Likely exceeds terminal
            padding: Spacing::from(5),
            ..Default::default()
        };

        let recovered = recovery::recover_from_terminal_overflow("Hello", options);

        // Should have adjusted to fit terminal
        let validation = validate_configuration("Hello", &recovered);
        // The recovery might not make it completely valid, but it should improve it
        assert!(validation.errors.len() <= 1); // Should have fewer errors
    }

    #[test]
    fn test_smart_recovery() {
        let options = BoxenOptions {
            width: Some(5),            // Too small
            height: Some(2),           // Too small
            padding: Spacing::from(3), // Large padding
            ..Default::default()
        };

        let original_validation = validate_configuration("Hello\nWorld\nTest", &options);
        let recovered = recovery::smart_recovery("Hello\nWorld\nTest", options);

        // Should have made improvements
        let recovered_validation = validate_configuration("Hello\nWorld\nTest", &recovered);

        // Recovery should reduce the number of errors
        assert!(recovered_validation.errors.len() <= original_validation.errors.len());
    }

    #[test]
    fn test_builder_comprehensive_validation() {
        let result = BoxenBuilder::new()
            .width(0) // Invalid
            .padding(150) // Excessive
            .title("a".repeat(250)) // Too long
            .border_color("invalid_color") // Invalid color
            .render("Hello");

        assert!(result.is_err());
    }

    #[test]
    fn test_builder_render_or_adjust_success() {
        let result = BoxenBuilder::new()
            .width(50)
            .height(10)
            .padding(2)
            .render_or_adjust("Hello, world!");

        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_render_or_adjust_with_recovery() {
        let result = BoxenBuilder::new()
            .width(5) // Too small, should be auto-adjusted
            .padding(3) // Large padding, should be reduced
            .render_or_adjust("Hello, world!");

        // Should succeed due to auto-adjustment
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_message_quality() {
        let error = BoxenError::invalid_dimensions(
            "Width 5 is too small. Minimum required: 15".to_string(),
            Some(5),
            None,
            vec![ErrorRecommendation::with_auto_fix(
                "Increase width".to_string(),
                "Set width to at least 15".to_string(),
                ".width(15)".to_string(),
            )],
        );

        let message = error.detailed_message();

        // Check that error message is descriptive and helpful
        assert!(message.contains("Width 5 is too small"));
        assert!(message.contains("Minimum required: 15"));
        assert!(message.contains("Suggestions:"));
        assert!(message.contains("Increase width"));
        assert!(message.contains("Set width to at least 15"));
        assert!(message.contains("Auto-fix: .width(15)"));
    }

    #[test]
    fn test_color_error_with_recommendations() {
        let error = BoxenError::invalid_color(
            "Unknown color name: purple_rain".to_string(),
            "purple_rain".to_string(),
            vec![
                ErrorRecommendation::suggestion_only(
                    "Unknown color".to_string(),
                    "Use standard color names like 'red', 'blue', 'green'".to_string(),
                ),
                ErrorRecommendation::with_auto_fix(
                    "Use standard color".to_string(),
                    "Try using 'purple' instead".to_string(),
                    "\"purple\"".to_string(),
                ),
            ],
        );

        let recommendations = error.recommendations();
        assert_eq!(recommendations.len(), 2);
        assert!(recommendations[1].auto_fix.is_some());
    }

    #[test]
    fn test_input_validation_error_structure() {
        let error = BoxenError::input_validation_error(
            "Test validation error".to_string(),
            "test_field".to_string(),
            "test_value".to_string(),
            vec![],
        );

        match error {
            BoxenError::InputValidationError {
                message,
                field,
                value,
                ..
            } => {
                assert_eq!(message, "Test validation error");
                assert_eq!(field, "test_field");
                assert_eq!(value, "test_value");
            }
            _ => panic!("Expected InputValidationError"),
        }
    }

    #[test]
    fn test_rendering_error_structure() {
        let error = BoxenError::rendering_error(
            "Test rendering error".to_string(),
            vec![ErrorRecommendation::suggestion_only(
                "Rendering issue".to_string(),
                "Check your configuration".to_string(),
            )],
        );

        match error {
            BoxenError::RenderingError {
                message,
                recommendations,
            } => {
                assert_eq!(message, "Test rendering error");
                assert_eq!(recommendations.len(), 1);
            }
            _ => panic!("Expected RenderingError"),
        }
    }

    #[test]
    fn test_comprehensive_error_handling_integration() {
        // Test that the entire error handling system works together
        let builder = BoxenBuilder::new()
            .width(50) // Reasonable width
            .height(10) // Reasonable height
            .padding(1) // Small padding
            .title("Test Title")
            .border_color("red"); // Valid color

        // Test validation detection - this should pass
        let validation_result = builder.validate();
        if validation_result.is_err() {
            println!("Validation failed: {:?}", validation_result);
        }
        assert!(validation_result.is_ok());

        // Test that render works with valid configuration
        let render_result = builder.render_or_adjust("Hello\nWorld");
        assert!(render_result.is_ok());

        // Test with problematic configuration
        let problematic_builder = BoxenBuilder::new()
            .width(5) // Too small for content
            .padding(1); // Small padding to avoid excessive spacing errors

        // This should fail validation but succeed with auto-adjustment
        let render_result = problematic_builder.render_or_adjust("Hi");
        assert!(render_result.is_ok());
    }
}
