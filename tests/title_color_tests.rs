/// Integration tests for `title_color` feature
use ::boxen::{BoxenOptions, Color, TitleAlignment, boxen};

#[test]
fn test_title_color_basic() {
    let options = BoxenOptions {
        title: Some("Test".to_string()),
        title_color: Some(Color::Named("red".to_string())),
        ..Default::default()
    };
    let result = boxen("Content", Some(options)).unwrap();

    // Verify red ANSI code is present in output
    assert!(
        result.contains("\x1b[31m"),
        "Title should contain red ANSI code"
    );
    assert!(result.contains("Test"), "Title text should be present");
}

#[test]
fn test_title_color_fallback_to_border_color() {
    let options = BoxenOptions {
        title: Some("Test".to_string()),
        title_color: None,
        border_color: Some(Color::Named("blue".to_string())),
        width: Some(20),
        ..Default::default()
    };
    let result = boxen("Content", Some(options)).unwrap();

    // Verify blue ANSI code is present (fallback from border_color)
    assert!(
        result.contains("\x1b[34m"),
        "Title should use border_color when title_color is None"
    );
    assert!(result.contains("Test"), "Title text should be present");
}

#[test]
fn test_title_color_no_fallback() {
    let options = BoxenOptions {
        title: Some("Test".to_string()),
        title_color: None,
        border_color: None,
        width: Some(20),
        ..Default::default()
    };
    let result = boxen("Content", Some(options)).unwrap();

    // Title should be present but without color codes
    assert!(result.contains("Test"), "Title text should be present");
}

#[test]
fn test_title_color_with_different_alignments() {
    // Test with left alignment
    let options_left = BoxenOptions {
        title: Some("Left".to_string()),
        title_color: Some(Color::Named("green".to_string())),
        title_alignment: TitleAlignment::Left,
        ..Default::default()
    };
    let result_left = boxen("Content", Some(options_left)).unwrap();
    assert!(
        result_left.contains("\x1b[32m"),
        "Left-aligned title should have green color"
    );

    // Test with center alignment
    let options_center = BoxenOptions {
        title: Some("Center".to_string()),
        title_color: Some(Color::Named("yellow".to_string())),
        title_alignment: TitleAlignment::Center,
        ..Default::default()
    };
    let result_center = boxen("Content", Some(options_center)).unwrap();
    assert!(
        result_center.contains("\x1b[33m"),
        "Center-aligned title should have yellow color"
    );

    // Test with right alignment
    let options_right = BoxenOptions {
        title: Some("Right".to_string()),
        title_color: Some(Color::Named("magenta".to_string())),
        title_alignment: TitleAlignment::Right,
        ..Default::default()
    };
    let result_right = boxen("Content", Some(options_right)).unwrap();
    assert!(
        result_right.contains("\x1b[35m"),
        "Right-aligned title should have magenta color"
    );
}

#[test]
fn test_title_color_with_hex_color() {
    let options = BoxenOptions {
        title: Some("Hex Color".to_string()),
        title_color: Some(Color::Hex("#FF0000".to_string())),
        ..Default::default()
    };
    let result = boxen("Content", Some(options)).unwrap();

    // Verify RGB ANSI code is present (hex colors use TrueColor)
    assert!(
        result.contains("\x1b[38;2;255;0;0m"),
        "Title should contain RGB ANSI code for hex color"
    );
}

#[test]
fn test_title_color_with_rgb_color() {
    let options = BoxenOptions {
        title: Some("RGB Color".to_string()),
        title_color: Some(Color::Rgb(0, 255, 0)),
        ..Default::default()
    };
    let result = boxen("Content", Some(options)).unwrap();

    // Verify RGB ANSI code is present
    assert!(
        result.contains("\x1b[38;2;0;255;0m"),
        "Title should contain RGB ANSI code"
    );
}

#[test]
fn test_title_color_builder_api() {
    use ::boxen::builder;

    let result = builder()
        .title("Builder Test")
        .title_color("red")
        .width(30)
        .render("Content")
        .unwrap();

    // Verify red ANSI code is present
    assert!(
        result.contains("\x1b[31m"),
        "Title should have red color via builder API"
    );
    assert!(
        result.contains("Builder Test"),
        "Title text should be present"
    );
}

#[test]
fn test_title_color_builder_with_hex() {
    use ::boxen::builder;

    let result = builder()
        .title("Hex Test")
        .title_color("#00FF00")
        .width(30)
        .render("Content")
        .unwrap();

    // Verify RGB ANSI code for green
    assert!(
        result.contains("\x1b[38;2;0;255;0m"),
        "Title should have hex color via builder API"
    );
}
