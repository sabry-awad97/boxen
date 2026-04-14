/// Thread safety verification tests for boxen types
/// These tests verify that public types implement Send + Sync where appropriate
use ::boxen::{
    BorderChars, BorderStyle, BoxenBuilder, BoxenOptions, Color, DimensionConstraints, Float,
    FullscreenMode, LayoutDimensions, Spacing, TextAlignment, TitleAlignment,
};

// Helper functions to assert Send + Sync bounds at compile time
#[allow(dead_code)]
fn assert_send<T: Send>() {}
#[allow(dead_code)]
fn assert_sync<T: Sync>() {}
fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn test_boxen_options_is_send_sync() {
    assert_send_sync::<BoxenOptions>();
}

#[test]
fn test_boxen_builder_is_send_sync() {
    assert_send_sync::<BoxenBuilder>();
}

#[test]
fn test_border_style_is_send_sync() {
    assert_send_sync::<BorderStyle>();
}

#[test]
fn test_border_chars_is_send_sync() {
    assert_send_sync::<BorderChars>();
}

#[test]
fn test_color_is_send_sync() {
    assert_send_sync::<Color>();
}

#[test]
fn test_spacing_is_send_sync() {
    assert_send_sync::<Spacing>();
}

#[test]
fn test_text_alignment_is_send_sync() {
    assert_send_sync::<TextAlignment>();
}

#[test]
fn test_title_alignment_is_send_sync() {
    assert_send_sync::<TitleAlignment>();
}

#[test]
fn test_float_is_send_sync() {
    assert_send_sync::<Float>();
}

#[test]
fn test_fullscreen_mode_is_send_sync() {
    assert_send_sync::<FullscreenMode>();
}

#[test]
fn test_dimension_constraints_is_send_sync() {
    assert_send_sync::<DimensionConstraints>();
}

#[test]
fn test_layout_dimensions_is_send_sync() {
    assert_send_sync::<LayoutDimensions>();
}

// Test that we can actually send types across threads
#[test]
fn test_can_send_boxen_options_across_threads() {
    let options = BoxenOptions {
        border_style: BorderStyle::Double,
        padding: Spacing::from(2),
        ..Default::default()
    };

    let handle = std::thread::spawn(move || {
        // Use the options in another thread
        assert_eq!(options.border_style, BorderStyle::Double);
        options
    });

    let returned_options = handle.join().unwrap();
    assert_eq!(returned_options.border_style, BorderStyle::Double);
}

#[test]
fn test_can_send_boxen_builder_across_threads() {
    let builder = ::boxen::builder()
        .border_style(BorderStyle::Round)
        .padding(1);

    let handle = std::thread::spawn(move || {
        // Use the builder in another thread
        builder.render("Thread test")
    });

    let result = handle.join().unwrap();
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Thread test"));
}

#[test]
fn test_can_share_color_across_threads() {
    use std::sync::Arc;

    let color = Arc::new(Color::Named("red".to_string()));
    let color_clone = Arc::clone(&color);

    let handle = std::thread::spawn(move || {
        // Access color in another thread
        matches!(*color_clone, Color::Named(_))
    });

    assert!(handle.join().unwrap());
}

#[test]
fn test_can_share_spacing_across_threads() {
    use std::sync::Arc;

    let spacing = Arc::new(Spacing::from(2));
    let spacing_clone = Arc::clone(&spacing);

    let handle = std::thread::spawn(move || {
        // Access spacing in another thread
        spacing_clone.top
    });

    assert_eq!(handle.join().unwrap(), 2);
}
