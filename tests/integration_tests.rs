/// Integration tests for main boxen API functions
use ::boxen::{
    BorderStyle, BoxenOptions, Color, Float, Spacing, TextAlignment, TitleAlignment, boxen,
    builder, double_box, round_box, simple_box,
};

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
