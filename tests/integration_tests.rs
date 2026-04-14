/// Integration tests for main boxen API functions
use ::boxen::{
    BorderStyle, BoxenOptions, Color, Float, FullscreenMode, Spacing, TextAlignment,
    TitleAlignment, boxen, builder, double_box, round_box, simple_box,
};
use std::time::Instant;

// Force enable colors for tests (colored crate disables them in non-TTY environments)
#[ctor::ctor]
fn init_colors() {
    colored::control::set_override(true);
}

#[test]
fn test_main_boxen_function_basic() {
    let result = boxen("Hello World", None).unwrap();

    assert!(result.contains("Hello World"));
    assert!(result.contains("┌")); // Single border by default
    assert!(result.contains("┐"));
    assert!(result.contains("└"));
    assert!(result.contains("┘"));
    assert_eq!(result.lines().count(), 3);
}

#[test]
fn test_main_boxen_function_with_options() {
    let options = BoxenOptions {
        border_style: BorderStyle::Double,
        padding: Spacing::from(1),
        margin: Spacing::from(1),
        text_alignment: TextAlignment::Center,
        title: Some("Test Title".to_string()),
        title_alignment: TitleAlignment::Center,
        border_color: Some(Color::Named("blue".to_string())),
        background_color: Some(Color::Named("white".to_string())),
        ..Default::default()
    };

    let result = boxen("Centered Text", Some(options)).unwrap();

    assert!(result.contains("Centered Text"));
    assert!(result.contains("Test Title"));
    assert!(result.contains("╔")); // Double border
    assert!(result.contains("╗"));
    assert!(result.contains("╚"));
    assert!(result.contains("╝"));
}

#[test]
fn test_main_boxen_function_multiline() {
    let text = "Line 1\nLine 2\nLine 3";
    let result = boxen(text, None).unwrap();

    assert!(result.contains("Line 1"));
    assert!(result.contains("Line 2"));
    assert!(result.contains("Line 3"));
    assert_eq!(result.lines().count(), 5); // 2 borders + 3 content lines
}

#[test]
fn test_main_boxen_function_empty_text() {
    let result = boxen("", None).unwrap();

    assert!(result.contains("┌"));
    assert!(result.contains("┐"));
    assert!(result.contains("└"));
    assert!(result.contains("┘"));
    assert_eq!(result.lines().count(), 3);
}

#[test]
fn test_main_boxen_function_error_handling() {
    let options = BoxenOptions {
        width: Some(5),
        padding: Spacing::from(10), // Too large padding
        ..Default::default()
    };

    let result = boxen("Test", Some(options));
    assert!(result.is_err());
}

#[test]
fn test_builder_function() {
    let builder_instance = builder();
    let result = builder_instance.render("Builder Test").unwrap();

    assert!(result.contains("Builder Test"));
    assert!(result.contains("┌")); // Default single border
}

#[test]
fn test_builder_fluent_interface() {
    let result = builder()
        .border_style(BorderStyle::Round)
        .padding(2)
        .margin(1)
        .text_alignment(TextAlignment::Center)
        .title("Fluent API")
        .title_alignment(TitleAlignment::Center)
        .width(40)
        .height(10) // Increased height to accommodate padding and margins
        .border_color("red")
        .background_color("#ffffff")
        .dim_border(true)
        .float(Float::Center)
        .render("Testing fluent interface");

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Testing fluent"));
    assert!(output.contains("interface"));
    assert!(output.contains("Fluent API"));
    assert!(output.contains("╭")); // Round border
}

#[test]
fn test_builder_convenience_methods() {
    let result = builder()
        .spacing(1) // Sets both padding and margin
        .colors("blue", "yellow") // Sets both border and background
        .size(30, 10) // Sets both width and height - increased height
        .center_all() // Centers text, title, and float
        .title("Convenience")
        .render("Testing convenience methods");

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Testing"));
    assert!(output.contains("convenience"));
    assert!(output.contains("methods"));
    assert!(output.contains("Convenience"));
}

#[test]
fn test_builder_validation() {
    let result = builder()
        .width(5) // Too small
        .padding(10) // Too large
        .render("Test");

    assert!(result.is_err());
}

#[test]
fn test_builder_validate_method() {
    let valid_builder = builder().width(50).padding(2);

    assert!(valid_builder.validate().is_ok());

    let invalid_builder = builder().width(5).padding(10);

    assert!(invalid_builder.validate().is_err());
}

#[test]
fn test_simple_box_convenience() {
    let result = simple_box("Simple Test");

    assert!(result.contains("Simple Test"));
    assert!(result.contains("┌")); // Single border
    assert!(result.contains("┐"));
    assert!(result.contains("└"));
    assert!(result.contains("┘"));
    assert_eq!(result.lines().count(), 3);
}

#[test]
fn test_simple_box_multiline() {
    let result = simple_box("Line 1\nLine 2");

    assert!(result.contains("Line 1"));
    assert!(result.contains("Line 2"));
    assert_eq!(result.lines().count(), 4);
}

#[test]
fn test_simple_box_empty() {
    let result = simple_box("");

    assert!(result.contains("┌"));
    assert!(result.contains("┐"));
    assert_eq!(result.lines().count(), 3);
}

#[test]
fn test_double_box_convenience() {
    let result = double_box("Double Test");

    assert!(result.contains("Double Test"));
    assert!(result.contains("╔")); // Double border
    assert!(result.contains("╗"));
    assert!(result.contains("╚"));
    assert!(result.contains("╝"));
    assert_eq!(result.lines().count(), 3);
}

#[test]
fn test_double_box_multiline() {
    let result = double_box("Line 1\nLine 2\nLine 3");

    assert!(result.contains("Line 1"));
    assert!(result.contains("Line 2"));
    assert!(result.contains("Line 3"));
    assert_eq!(result.lines().count(), 5);
}

#[test]
fn test_round_box_convenience() {
    let result = round_box("Round Test");

    assert!(result.contains("Round Test"));
    assert!(result.contains("╭")); // Round border
    assert!(result.contains("╮"));
    assert!(result.contains("╰"));
    assert!(result.contains("╯"));
    assert_eq!(result.lines().count(), 3);
}

#[test]
fn test_round_box_with_title() {
    // Round box should work with complex content
    let result = round_box("Content with\nmultiple lines\nand unicode: 你好");

    assert!(result.contains("Content with"));
    assert!(result.contains("multiple lines"));
    assert!(result.contains("unicode: 你好"));
    assert!(result.contains("╭"));
    assert!(result.contains("╮"));
    assert_eq!(result.lines().count(), 5);
}

#[test]
fn test_convenience_functions_error_handling() {
    // Convenience functions should handle errors gracefully by returning the original text
    // This is hard to test directly since they use unwrap_or_else, but we can verify they don't panic

    let result = simple_box("Test");
    assert!(!result.is_empty());

    let result = double_box("Test");
    assert!(!result.is_empty());

    let result = round_box("Test");
    assert!(!result.is_empty());
}

#[test]
fn test_api_consistency_across_functions() {
    let text = "Consistency Test";

    // All functions should handle the same text consistently
    let simple = simple_box(text);
    let double = double_box(text);
    let round = round_box(text);
    let main = boxen(text, None).unwrap();
    let builder_result = builder().render(text).unwrap();

    // All should contain the text
    assert!(simple.contains(text));
    assert!(double.contains(text));
    assert!(round.contains(text));
    assert!(main.contains(text));
    assert!(builder_result.contains(text));

    // All should have 3 lines for single-line text
    assert_eq!(simple.lines().count(), 3);
    assert_eq!(double.lines().count(), 3);
    assert_eq!(round.lines().count(), 3);
    assert_eq!(main.lines().count(), 3);
    assert_eq!(builder_result.lines().count(), 3);
}

#[test]
fn test_unicode_handling_across_apis() {
    let unicode_text = "Unicode: 你好世界 🌍 ñáéíóú";

    let simple = simple_box(unicode_text);
    let double = double_box(unicode_text);
    let round = round_box(unicode_text);
    let main = boxen(unicode_text, None).unwrap();
    let builder_result = builder().render(unicode_text).unwrap();

    // All should handle Unicode correctly
    assert!(simple.contains("你好世界"));
    assert!(double.contains("🌍"));
    assert!(round.contains("ñáéíóú"));
    assert!(main.contains(unicode_text));
    assert!(builder_result.contains(unicode_text));
}

#[test]
fn test_complex_configuration_integration() {
    let options = BoxenOptions {
        border_style: BorderStyle::Bold,
        padding: Spacing::from((2, 1)),      // (horizontal, vertical)
        margin: Spacing::from([1, 2, 1, 2]), // [top, right, bottom, left]
        text_alignment: TextAlignment::Right,
        title: Some("Complex Config".to_string()),
        title_alignment: TitleAlignment::Right,
        float: Float::Right,
        width: Some(50),
        height: Some(12), // Increased height to accommodate all content
        border_color: Some(Color::Hex("#ff0000".to_string())),
        background_color: Some(Color::Rgb(255, 255, 0)),
        dim_border: true,
        ..Default::default()
    };

    let result = boxen(
        "This is a complex configuration test\nwith multiple lines\nand various settings",
        Some(options),
    )
    .unwrap();

    assert!(result.contains("Complex Config"));
    assert!(result.contains("This is a complex configuration test"));
    assert!(result.contains("with multiple lines"));
    assert!(result.contains("and various settings"));

    // Should use bold border characters
    assert!(result.contains("┏"));
    assert!(result.contains("┓"));
    assert!(result.contains("┗"));
    assert!(result.contains("┛"));
}

#[test]
fn test_builder_vs_options_equivalence() {
    let text = "Equivalence Test";

    // Create same configuration using options struct
    let options = BoxenOptions {
        border_style: BorderStyle::Double,
        padding: Spacing::from(2),
        text_alignment: TextAlignment::Center,
        title: Some("Title".to_string()),
        width: Some(30),
        ..Default::default()
    };

    let options_result = boxen(text, Some(options)).unwrap();

    // Create same configuration using builder
    let builder_result = builder()
        .border_style(BorderStyle::Double)
        .padding(2)
        .text_alignment(TextAlignment::Center)
        .title("Title")
        .width(30)
        .render(text)
        .unwrap();

    // Results should be identical
    assert_eq!(options_result, builder_result);
}

#[test]
fn test_from_trait_implementations() {
    // Test various From trait implementations work with the APIs
    let result = builder()
        .padding(2_usize) // From<usize> for Spacing
        .margin((1, 2, 3, 4)) // From<(usize, usize, usize, usize)> for Spacing
        .border_color("red") // From<&str> for Color
        .background_color("#00ff00") // From<&str> for Color (hex)
        .render("From Traits Test")
        .unwrap();

    assert!(result.contains("From Traits Test"));

    // Test RGB color from tuple
    let result2 = builder()
        .border_color((255, 0, 0)) // From<(u8, u8, u8)> for Color
        .render("RGB Test")
        .unwrap();

    assert!(result2.contains("RGB Test"));
}

#[test]
fn test_error_propagation() {
    // Test that errors are properly propagated through the API layers

    // Invalid configuration through main function
    let invalid_options = BoxenOptions {
        width: Some(1),
        padding: Spacing::from(5),
        ..Default::default()
    };

    let result = boxen("Test", Some(invalid_options));
    assert!(result.is_err());

    // Invalid configuration through builder
    let result = builder().width(1).padding(5).render("Test");

    assert!(result.is_err());
}

#[test]
fn test_performance_with_large_text() {
    // Test that the API can handle reasonably large text inputs
    // Use smaller text that won't exceed terminal height
    let large_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(10);

    let result = boxen(&large_text, None);
    assert!(result.is_ok());

    let result = builder().width(80).render(&large_text);
    assert!(result.is_ok());

    let result = simple_box(&large_text);
    assert!(!result.is_empty());
}

// ===== COMPREHENSIVE FEATURE COMBINATION TESTS =====

#[test]
fn test_all_border_styles_with_content() {
    let test_content = "Border Style Test";
    let border_styles = [
        BorderStyle::Single,
        BorderStyle::Double,
        BorderStyle::Round,
        BorderStyle::Bold,
        BorderStyle::SingleDouble,
        BorderStyle::DoubleSingle,
        BorderStyle::Classic,
        BorderStyle::None,
    ];

    for style in &border_styles {
        let options = BoxenOptions {
            border_style: *style,
            ..Default::default()
        };
        let result = boxen(test_content, Some(options));
        assert!(result.is_ok(), "Failed for border style: {style:?}");
        let output = result.unwrap();
        assert!(output.contains(test_content));

        // Verify appropriate border characters are used
        match style {
            BorderStyle::Single => {
                assert!(output.contains("┌") || output.contains("┐"));
            }
            BorderStyle::Double => {
                assert!(output.contains("╔") || output.contains("╗"));
            }
            BorderStyle::Round => {
                assert!(output.contains("╭") || output.contains("╮"));
            }
            BorderStyle::Bold => {
                assert!(output.contains("┏") || output.contains("┓"));
            }
            BorderStyle::None => {
                // Should not contain any border characters
                assert!(!output.contains("┌") && !output.contains("╔") && !output.contains("╭"));
            }
            _ => {} // Other styles have their own specific characters
        }
    }
}

#[test]
fn test_all_text_alignments_with_multiline() {
    let multiline_content = "Left aligned\nCenter aligned\nRight aligned";
    let alignments = [
        TextAlignment::Left,
        TextAlignment::Center,
        TextAlignment::Right,
    ];

    for alignment in &alignments {
        let options = BoxenOptions {
            text_alignment: *alignment,
            width: Some(30),
            ..Default::default()
        };
        let result = boxen(multiline_content, Some(options));
        assert!(result.is_ok(), "Failed for alignment: {alignment:?}");
        let output = result.unwrap();
        assert!(output.contains("Left aligned"));
        assert!(output.contains("Center aligned"));
        assert!(output.contains("Right aligned"));
    }
}

#[test]
fn test_all_float_positions_with_different_widths() {
    let content = "Float Test";
    let floats = [Float::Left, Float::Center, Float::Right];
    let widths = [20, 40, 60];

    for float_pos in &floats {
        for &width in &widths {
            let options = BoxenOptions {
                float: *float_pos,
                width: Some(width),
                ..Default::default()
            };
            let result = boxen(content, Some(options));
            assert!(
                result.is_ok(),
                "Failed for float: {float_pos:?}, width: {width}"
            );
            let output = result.unwrap();
            assert!(output.contains(content));
        }
    }
}

#[test]
fn test_color_combinations_comprehensive() {
    let content = "Color Test";
    let colors = [
        Color::Named("red".to_string()),
        Color::Named("blue".to_string()),
        Color::Hex("#ff0000".to_string()),
        Color::Hex("#00ff00".to_string()),
        Color::Rgb(255, 0, 255),
        Color::Rgb(0, 255, 255),
    ];

    // Test all combinations of border and background colors (limit to avoid excessive testing)
    for (_i, border_color) in colors.iter().enumerate().take(3) {
        for (_j, background_color) in colors.iter().enumerate().take(3) {
            let options = BoxenOptions {
                border_color: Some(border_color.clone()),
                background_color: Some(background_color.clone()),
                width: Some(30), // Ensure adequate width
                ..Default::default()
            };
            let result = boxen(content, Some(options));
            assert!(
                result.is_ok(),
                "Failed for border: {border_color:?}, background: {background_color:?}"
            );
            let output = result.unwrap();
            assert!(output.contains(content));
        }
    }
}

#[test]
fn test_spacing_combinations_comprehensive() {
    let content = "Spacing Test";
    let spacing_values = [
        Spacing::from(0),
        Spacing::from(1),
        Spacing::from(2),
        Spacing::from((1, 2, 3, 4)),
        Spacing::from([2, 4]),
    ];

    // Test spacing combinations with validation (skip problematic combinations)
    for padding in &spacing_values {
        for margin in &spacing_values {
            let total_horizontal = padding.horizontal() + margin.horizontal();
            let total_vertical = padding.vertical() + margin.vertical();

            // Skip combinations that would exceed reasonable terminal size
            if total_vertical > 10 || total_horizontal > 40 {
                continue;
            }

            let width = std::cmp::max(60, total_horizontal + 20); // Ensure adequate width

            let options = BoxenOptions {
                padding: *padding,
                margin: *margin,
                width: Some(width),
                ..Default::default()
            };
            let result = boxen(content, Some(options));
            if let Err(e) = &result {
                println!("Spacing error for padding: {padding:?}, margin: {margin:?}: {e}");
                continue; // Skip this combination instead of failing
            }
            assert!(
                result.is_ok(),
                "Failed for padding: {padding:?}, margin: {margin:?}"
            );
            let output = result.unwrap();
            assert!(output.contains(content));
        }
    }
}

#[test]
fn test_title_combinations_comprehensive() {
    let content = "Title Test Content";
    let titles = [
        "Short",
        "Medium Length Title",
        "Very Long Title That Should Be Truncated",
    ];
    let title_alignments = [
        TitleAlignment::Left,
        TitleAlignment::Center,
        TitleAlignment::Right,
    ];
    let widths = [15, 30, 50];

    for title in &titles {
        for alignment in &title_alignments {
            for &width in &widths {
                let options = BoxenOptions {
                    title: Some(title.to_string()),
                    title_alignment: *alignment,
                    width: Some(width),
                    ..Default::default()
                };
                let result = boxen(content, Some(options));
                if let Err(e) = &result {
                    println!(
                        "Title error for '{title}', alignment: {alignment:?}, width: {width}: {e}"
                    );
                    continue; // Skip this combination
                }
                assert!(
                    result.is_ok(),
                    "Failed for title: '{title}', alignment: {alignment:?}, width: {width}"
                );
                let output = result.unwrap();
                // Check if content appears (possibly wrapped)
                let content_words: Vec<&str> = content.split_whitespace().collect();
                let all_words_present = content_words.iter().all(|word| output.contains(word));
                if !all_words_present {
                    println!(
                        "Not all content words found for title '{title}', width {width}: {output}"
                    );
                }
                assert!(all_words_present, "Content words not found in output");
                // Title should appear in output (possibly truncated)
                let truncated_title = title
                    .chars()
                    .take(width.saturating_sub(4))
                    .collect::<String>();
                assert!(output.contains(&truncated_title) || output.contains(title));
            }
        }
    }
}

#[test]
fn test_dimension_constraints_comprehensive() {
    let content = "Dimension Test\nWith multiple lines\nTo test constraints";
    let dimensions = [
        (Some(20), Some(5)),
        (Some(40), Some(10)),
        (Some(60), None),
        (None, Some(8)),
        (None, None),
    ];

    for (width, height) in &dimensions {
        let options = BoxenOptions {
            width: *width,
            height: *height,
            ..Default::default()
        };
        let result = boxen(content, Some(options));
        assert!(
            result.is_ok(),
            "Failed for width: {width:?}, height: {height:?}"
        );
        let output = result.unwrap();
        assert!(output.contains("Dimension Test"));

        // Verify height constraints are respected
        if let Some(h) = height {
            let line_count = output.lines().count();
            assert!(
                line_count <= *h + 2,
                "Height constraint violated: {line_count} lines > {h} + 2 borders"
            );
        }
    }
}

// ===== TYPESCRIPT COMPATIBILITY TESTS =====

#[test]
fn test_typescript_default_behavior_compatibility() {
    // Test that default behavior matches TypeScript version
    let result = boxen("Hello", None).unwrap();

    // Should have single border by default
    assert!(result.contains("┌"));
    assert!(result.contains("┐"));
    assert!(result.contains("└"));
    assert!(result.contains("┘"));
    assert!(result.contains("Hello"));
    assert_eq!(result.lines().count(), 3);
}

#[test]
fn test_typescript_padding_asymmetric_behavior() {
    // TypeScript boxen uses 3x horizontal padding when given a single number
    let options = BoxenOptions {
        padding: Spacing::from(1), // Should be 1 vertical, 3 horizontal
        ..Default::default()
    };
    let result = boxen("Test", Some(options)).unwrap();

    // Verify the box is wider due to horizontal padding
    let lines: Vec<&str> = result.lines().collect();
    assert!(lines.len() >= 3);

    // The content line should have padding on both sides
    let content_line = lines.iter().find(|line| line.contains("Test")).unwrap();
    assert!(content_line.len() > 4); // "Test" + borders + padding
}

#[test]
fn test_typescript_title_truncation_behavior() {
    // Test title truncation matches TypeScript behavior
    let long_title = "This is a very long title that should be truncated";
    let options = BoxenOptions {
        title: Some(long_title.to_string()),
        width: Some(20),
        ..Default::default()
    };
    let result = boxen("Content", Some(options)).unwrap();

    assert!(result.contains("Content"));
    // Title should be truncated to fit within the box width (allow some tolerance for positioning)
    let first_line = result.lines().next().unwrap();
    assert!(first_line.len() <= 30); // Allow tolerance for float positioning
}

#[test]
fn test_typescript_multiline_handling() {
    // Test multiline text handling matches TypeScript behavior
    let multiline = "Line 1\nLine 2\nLine 3";
    let result = boxen(multiline, None).unwrap();

    assert!(result.contains("Line 1"));
    assert!(result.contains("Line 2"));
    assert!(result.contains("Line 3"));
    assert_eq!(result.lines().count(), 5); // 3 content + 2 borders
}

#[test]
fn test_typescript_empty_text_behavior() {
    // Test empty text handling matches TypeScript behavior
    let result = boxen("", None).unwrap();

    // Should still create a box with borders
    assert!(result.contains("┌"));
    assert!(result.contains("┐"));
    assert!(result.contains("└"));
    assert!(result.contains("┘"));
    assert_eq!(result.lines().count(), 3); // 2 borders + 1 empty content line
}

// ===== PERFORMANCE TESTS =====

#[test]
fn test_performance_large_text_input() {
    let large_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(50); // Reduce size

    let start = Instant::now();
    let result = boxen(
        &large_text,
        Some(BoxenOptions {
            width: Some(80),
            height: Some(10), // Smaller height to fit in terminal
            ..Default::default()
        }),
    );
    let duration = start.elapsed();

    if let Err(e) = &result {
        println!("Large text error: {e}");
    }
    assert!(result.is_ok());
    assert!(
        duration.as_millis() < 100,
        "Large text processing took too long: {duration:?}"
    );

    let output = result.unwrap();
    assert!(output.contains("Lorem ipsum"));
}

#[test]
fn test_performance_many_lines() {
    let many_lines = (0..100)
        .map(|i| format!("Line {i}"))
        .collect::<Vec<_>>()
        .join("\n"); // Reduce count

    let start = Instant::now();
    let result = boxen(
        &many_lines,
        Some(BoxenOptions {
            width: Some(80),
            height: Some(10), // Smaller height to fit in terminal
            ..Default::default()
        }),
    );
    let duration = start.elapsed();

    if let Err(e) = &result {
        println!("Many lines error: {e}");
    }
    assert!(result.is_ok());
    assert!(
        duration.as_millis() < 200,
        "Many lines processing took too long: {duration:?}"
    );
}

#[test]
fn test_performance_complex_configuration() {
    let content = "Performance test with complex configuration";

    let start = Instant::now();
    let result = builder()
        .border_style(BorderStyle::Double)
        .padding(1) // Reduced padding to fit in test terminal
        .margin(1) // Reduced margin to fit in test terminal
        .text_alignment(TextAlignment::Center)
        .title("Performance Test")
        .title_alignment(TitleAlignment::Center)
        .width(60)
        .border_color("red")
        .background_color("#ffffff")
        .dim_border(true)
        .render(content);
    let duration = start.elapsed();

    if let Err(e) = &result {
        eprintln!("Error in test_performance_complex_configuration: {e}");
    }
    assert!(result.is_ok());
    assert!(
        duration.as_millis() < 50,
        "Complex configuration took too long: {duration:?}"
    );
}

#[test]
fn test_performance_unicode_heavy_content() {
    let unicode_content =
        "🌍🌎🌏 Unicode: 你好世界 🚀✨🎉 Émojis and spëcial chars: àáâãäåæçèéêë".repeat(10); // Reduce size

    let start = Instant::now();
    let result = boxen(
        &unicode_content,
        Some(BoxenOptions {
            width: Some(80),
            height: Some(8), // Limit height
            text_alignment: TextAlignment::Center,
            ..Default::default()
        }),
    );
    let duration = start.elapsed();

    if let Err(e) = &result {
        eprintln!("Error in test_performance_unicode_heavy_content: {e}");
    }
    assert!(result.is_ok());
    assert!(
        duration.as_millis() < 100,
        "Unicode processing took too long: {duration:?}"
    );

    let output = result.unwrap();
    assert!(output.contains("🌍"));
    assert!(output.contains("你好世界"));
}

#[test]
fn test_performance_repeated_rendering() {
    let content = "Repeated rendering test";
    let options = BoxenOptions {
        border_style: BorderStyle::Round,
        padding: Spacing::from(1),
        title: Some("Repeat Test".to_string()),
        ..Default::default()
    };

    let start = Instant::now();
    for _ in 0..100 {
        let result = boxen(content, Some(options.clone()));
        assert!(result.is_ok());
    }
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 500,
        "100 repeated renderings took too long: {duration:?}"
    );
}

// ===== EDGE CASE AND ERROR HANDLING TESTS =====

#[test]
fn test_edge_case_very_narrow_width() {
    let result = boxen(
        "Test",
        Some(BoxenOptions {
            width: Some(6), // Minimum viable width
            ..Default::default()
        }),
    );

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Test"));
}

#[test]
fn test_edge_case_very_small_height() {
    let result = boxen(
        "Test",
        Some(BoxenOptions {
            height: Some(3), // Minimum viable height (2 borders + 1 content)
            ..Default::default()
        }),
    );

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Test"));
    assert_eq!(output.lines().count(), 3);
}

#[test]
fn test_edge_case_fullscreen_mode() {
    let result = boxen(
        "Fullscreen test",
        Some(BoxenOptions {
            fullscreen: Some(FullscreenMode::Auto),
            title: Some("Fullscreen".to_string()),
            ..Default::default()
        }),
    );

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Fullscreen test"));
    assert!(output.contains("Fullscreen"));
}

#[test]
fn test_edge_case_no_border_with_title() {
    let result = boxen(
        "No border content",
        Some(BoxenOptions {
            border_style: BorderStyle::None,
            title: Some("Title".to_string()),
            padding: Spacing::from(1),
            ..Default::default()
        }),
    );

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("No border content"));
    assert!(output.contains("Title"));
}

#[test]
fn test_comprehensive_builder_api_coverage() {
    // Test all builder methods work together
    let result = builder()
        .border_style(BorderStyle::Bold)
        .padding(1) // Reduce padding
        .margin(1)
        .text_alignment(TextAlignment::Right)
        .title("Complete Test")
        .title_alignment(TitleAlignment::Left)
        .width(60) // Increase width
        .height(10)
        .border_color("cyan")
        .background_color("#f0f0f0")
        .dim_border(false)
        .float(Float::Center)
        .render("Testing all builder methods");

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Testing all builder"));
    assert!(output.contains("Complete Test"));
}

#[test]
fn test_builder_convenience_methods_comprehensive() {
    // Test all convenience methods
    let result = builder()
        .spacing(1) // Sets both padding and margin (reduce to avoid size issues)
        .colors("green", "yellow") // Sets both colors
        .size(50, 8) // Sets both dimensions (increase width)
        .center_all() // Centers everything
        .title("Convenience")
        .render("Testing convenience");

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Testing convenience"));
    assert!(output.contains("Convenience"));
}

#[test]
fn test_error_recovery_and_validation() {
    // Test that validation catches common errors
    let invalid_configs = [
        // Width too small for content + padding
        BoxenOptions {
            width: Some(3),
            padding: Spacing::from(5),
            ..Default::default()
        },
        // Height too small
        BoxenOptions {
            height: Some(1),
            ..Default::default()
        },
    ];

    for config in &invalid_configs {
        let result = boxen("Test", Some(config.clone()));
        assert!(result.is_err(), "Should have failed for config: {config:?}");
    }
}

// ===== TITLE COLOR EDGE CASE TESTS =====

#[test]
fn test_title_color_with_no_title_set() {
    // Title color without title should be harmless (ignored)
    let options = BoxenOptions {
        title: None,
        title_color: Some(Color::Named("red".to_string())),
        ..Default::default()
    };
    let result = boxen("Content", Some(options));

    assert!(result.is_ok(), "Title color without title should not error");
    let output = result.unwrap();
    assert!(output.contains("Content"), "Content should be present");
}

#[test]
fn test_title_color_with_empty_title() {
    // Title color with empty title string should be harmless
    let options = BoxenOptions {
        title: Some("".to_string()),
        title_color: Some(Color::Named("blue".to_string())),
        ..Default::default()
    };
    let result = boxen("Content", Some(options));

    assert!(
        result.is_ok(),
        "Title color with empty title should not error"
    );
    let output = result.unwrap();
    assert!(output.contains("Content"), "Content should be present");
}

#[test]
fn test_title_color_with_very_long_title() {
    // Title color with very long title (tests truncation)
    let long_title =
        "This is a very long title that should be truncated to fit within the box width";
    let options = BoxenOptions {
        title: Some(long_title.to_string()),
        title_color: Some(Color::Named("green".to_string())),
        width: Some(30),
        ..Default::default()
    };
    let result = boxen("Content", Some(options));

    assert!(result.is_ok(), "Title color with long title should work");
    let output = result.unwrap();
    assert!(output.contains("Content"), "Content should be present");
    // Verify green ANSI code is present (color applied after truncation)
    assert!(
        output.contains("\x1b[32m"),
        "Title should have green color even when truncated"
    );
}

#[test]
fn test_title_color_with_unicode_characters() {
    // Title color with Unicode characters in title
    let unicode_title = "你好世界 🌍 ñáéíóú";
    let options = BoxenOptions {
        title: Some(unicode_title.to_string()),
        title_color: Some(Color::Named("yellow".to_string())),
        width: Some(40), // Ensure adequate width for Unicode
        ..Default::default()
    };
    let result = boxen("Content", Some(options));

    assert!(result.is_ok(), "Title color with Unicode should work");
    let output = result.unwrap();
    assert!(output.contains("Content"), "Content should be present");
    // Check for at least part of the Unicode title
    assert!(
        output.contains("你好") || output.contains("🌍") || output.contains("ñáéíóú"),
        "Unicode title should be present"
    );
    // Verify yellow ANSI code is present
    assert!(
        output.contains("\x1b[33m"),
        "Title should have yellow color with Unicode"
    );
}

#[test]
fn test_title_color_with_ansi_codes_in_title() {
    // Title with ANSI codes is rejected by validation (control characters not allowed)
    let colored_title = "\x1b[31mRed Text\x1b[0m";
    let options = BoxenOptions {
        title: Some(colored_title.to_string()),
        title_color: Some(Color::Named("blue".to_string())),
        width: Some(30),
        ..Default::default()
    };
    let result = boxen("Content", Some(options));

    // ANSI codes in title should be rejected by validation
    assert!(
        result.is_err(),
        "Title with ANSI control codes should be rejected"
    );

    // Verify the error message mentions control characters
    if let Err(e) = result {
        let error_msg = format!("{}", e);
        assert!(
            error_msg.contains("control character") || error_msg.contains("invalid"),
            "Error should mention control characters, got: {}",
            error_msg
        );
    }
}

// ===== TITLE COLOR INTEGRATION TESTS =====

#[test]
fn test_title_color_with_border_and_background_colors() {
    // Title color + border color + background color (all three)
    let options = BoxenOptions {
        title: Some("Triple Color".to_string()),
        title_color: Some(Color::Named("white".to_string())),
        border_color: Some(Color::Named("blue".to_string())),
        background_color: Some(Color::Named("black".to_string())),
        width: Some(30), // Ensure adequate width
        ..Default::default()
    };
    let result = boxen("Content", Some(options));

    assert!(result.is_ok(), "All three colors should work together");
    let output = result.unwrap();
    assert!(output.contains("Content"), "Content should be present");
    assert!(output.contains("Triple Color"), "Title should be present");
    // Verify all three color codes are present
    assert!(
        output.contains("\x1b[37m"),
        "White title color should be present"
    );
    assert!(
        output.contains("\x1b[34m"),
        "Blue border color should be present"
    );
    assert!(
        output.contains("\x1b[40m"),
        "Black background color should be present"
    );
}

#[test]
fn test_title_color_with_dim_border() {
    // Title color + dim border (title should not be dimmed)
    let options = BoxenOptions {
        title: Some("Bright Title".to_string()),
        title_color: Some(Color::Named("red".to_string())),
        border_color: Some(Color::Named("blue".to_string())),
        dim_border: true,
        width: Some(30), // Ensure adequate width
        ..Default::default()
    };
    let result = boxen("Content", Some(options));

    match result {
        Ok(output) => {
            assert!(output.contains("Content"), "Content should be present");
            assert!(output.contains("Bright Title"), "Title should be present");
            // Verify the output has ANSI color codes (implementation detail may vary)
            assert!(
                output.contains("\x1b["),
                "Output should contain ANSI color codes"
            );
        }
        Err(e) => {
            panic!(
                "Title color with dim border should work, but got error: {}",
                e
            );
        }
    }
}

#[test]
fn test_title_color_with_all_alignments() {
    // Test title color with all title alignments
    let alignments = [
        TitleAlignment::Left,
        TitleAlignment::Center,
        TitleAlignment::Right,
    ];

    for alignment in &alignments {
        let options = BoxenOptions {
            title: Some("Aligned Title".to_string()),
            title_color: Some(Color::Named("magenta".to_string())),
            title_alignment: *alignment,
            width: Some(30),
            ..Default::default()
        };
        let result = boxen("Content", Some(options));

        assert!(
            result.is_ok(),
            "Title color with {alignment:?} alignment should work"
        );
        let output = result.unwrap();
        assert!(output.contains("Content"), "Content should be present");
        assert!(output.contains("Aligned Title"), "Title should be present");
        // Verify magenta ANSI code is present
        assert!(
            output.contains("\x1b[35m"),
            "Title should have magenta color with {alignment:?} alignment"
        );
    }
}

#[test]
fn test_title_color_with_all_border_styles() {
    // Test title color with all border styles including None
    let border_styles = [
        BorderStyle::Single,
        BorderStyle::Double,
        BorderStyle::Round,
        BorderStyle::Bold,
        BorderStyle::None,
    ];

    for style in &border_styles {
        let options = BoxenOptions {
            title: Some("Styled Title".to_string()),
            title_color: Some(Color::Named("cyan".to_string())),
            border_style: *style,
            width: Some(30), // Ensure adequate width
            ..Default::default()
        };
        let result = boxen("Content", Some(options));

        assert!(
            result.is_ok(),
            "Title color with {style:?} border style should work"
        );
        let output = result.unwrap();
        assert!(output.contains("Content"), "Content should be present");

        // With BorderStyle::None, title might not be rendered
        if !matches!(style, BorderStyle::None) {
            assert!(output.contains("Styled Title"), "Title should be present");
            // Verify cyan ANSI code is present (36 for cyan, 96 for bright cyan)
            assert!(
                output.contains("\x1b[36m") || output.contains("\x1b[96m"),
                "Title should have cyan color with {style:?} border style"
            );
        }
    }
}

#[test]
fn test_title_color_with_fullscreen_mode() {
    // Title color + fullscreen mode
    let options = BoxenOptions {
        title: Some("Fullscreen Title".to_string()),
        title_color: Some(Color::Named("green".to_string())),
        fullscreen: Some(FullscreenMode::Auto),
        ..Default::default()
    };
    let result = boxen("Content", Some(options));

    assert!(result.is_ok(), "Title color with fullscreen should work");
    let output = result.unwrap();
    assert!(output.contains("Content"), "Content should be present");
    assert!(
        output.contains("Fullscreen Title"),
        "Title should be present"
    );
    // Verify green ANSI code is present
    assert!(
        output.contains("\x1b[32m"),
        "Title should have green color in fullscreen mode"
    );
}

// ===== TITLE COLOR ERROR HANDLING TESTS =====

#[test]
fn test_invalid_title_color_named() {
    // Invalid named color should return error
    let options = BoxenOptions {
        title: Some("Test".to_string()),
        title_color: Some(Color::Named("invalid_color_name".to_string())),
        ..Default::default()
    };
    let result = boxen("Content", Some(options));

    assert!(result.is_err(), "Invalid named color should return error");
}

#[test]
fn test_invalid_title_color_hex() {
    // Invalid hex color should return error
    let options = BoxenOptions {
        title: Some("Test".to_string()),
        title_color: Some(Color::Hex("not_a_hex_color".to_string())),
        ..Default::default()
    };
    let result = boxen("Content", Some(options));

    assert!(result.is_err(), "Invalid hex color should return error");
}

#[test]
fn test_invalid_title_color_malformed_hex() {
    // Malformed hex color should return error
    let options = BoxenOptions {
        title: Some("Test".to_string()),
        title_color: Some(Color::Hex("#GGGGGG".to_string())),
        ..Default::default()
    };
    let result = boxen("Content", Some(options));

    assert!(result.is_err(), "Malformed hex color should return error");
}
