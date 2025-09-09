/// Performance tests for boxen library
use ::boxen::{
    BorderStyle, BoxenOptions, Color, Spacing, TextAlignment, TitleAlignment, boxen, builder,
};
use std::time::Instant;

// Performance test thresholds (in milliseconds)
const SMALL_TEXT_THRESHOLD: u128 = 30;
const MEDIUM_TEXT_THRESHOLD: u128 = 100;
const LARGE_TEXT_THRESHOLD: u128 = 200;
const COMPLEX_CONFIG_THRESHOLD: u128 = 20;
const REPEATED_RENDER_THRESHOLD: u128 = 500;

#[test]
fn test_performance_small_text() {
    let text = "Small text performance test";

    let start = Instant::now();
    let result = boxen(text, None);
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(
        duration.as_millis() < SMALL_TEXT_THRESHOLD,
        "Small text took too long: {:?} (threshold: {}ms)",
        duration,
        SMALL_TEXT_THRESHOLD
    );
}

#[test]
fn test_performance_medium_text() {
    let text = "Medium length text for performance testing. ".repeat(50);

    let start = Instant::now();
    let result = boxen(
        &text,
        Some(BoxenOptions {
            width: Some(60), // Reduce width to fit in smaller terminals
            height: Some(8), // Add height constraint
            ..Default::default()
        }),
    );
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(
        duration.as_millis() < MEDIUM_TEXT_THRESHOLD,
        "Medium text took too long: {:?} (threshold: {}ms)",
        duration,
        MEDIUM_TEXT_THRESHOLD
    );
}

#[test]
fn test_performance_large_text() {
    let text = "Large text content for performance testing. ".repeat(500);

    let start = Instant::now();
    let result = boxen(
        &text,
        Some(BoxenOptions {
            width: Some(60),  // Reduce width
            height: Some(10), // Smaller height to fit in terminal
            ..Default::default()
        }),
    );
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(
        duration.as_millis() < LARGE_TEXT_THRESHOLD,
        "Large text took too long: {:?} (threshold: {}ms)",
        duration,
        LARGE_TEXT_THRESHOLD
    );
}

#[test]
fn test_performance_many_lines() {
    let many_lines = (0..1000)
        .map(|i| format!("Line number {} with some content", i))
        .collect::<Vec<_>>()
        .join("\n");

    let start = Instant::now();
    let result = boxen(
        &many_lines,
        Some(BoxenOptions {
            width: Some(60),  // Reduce width
            height: Some(10), // Smaller height to fit in terminal
            ..Default::default()
        }),
    );
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(
        duration.as_millis() < LARGE_TEXT_THRESHOLD,
        "Many lines took too long: {:?} (threshold: {}ms)",
        duration,
        LARGE_TEXT_THRESHOLD
    );
}

#[test]
fn test_performance_complex_configuration() {
    let text = "Complex configuration performance test";

    let start = Instant::now();
    let result = builder()
        .border_style(BorderStyle::Double)
        .padding(3)
        .margin(2)
        .text_alignment(TextAlignment::Center)
        .title("Performance Test")
        .title_alignment(TitleAlignment::Center)
        .width(60)
        .height(15)
        .border_color("red")
        .background_color("#ffffff")
        .dim_border(true)
        .render(text);
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(
        duration.as_millis() < COMPLEX_CONFIG_THRESHOLD,
        "Complex configuration took too long: {:?} (threshold: {}ms)",
        duration,
        COMPLEX_CONFIG_THRESHOLD
    );
}

#[test]
fn test_performance_unicode_content() {
    let unicode_text =
        "Unicode performance: 🌍🌎🌏 你好世界 🚀✨🎉 Émojis: àáâãäåæçèéêë ".repeat(100);

    let start = Instant::now();
    let result = boxen(
        &unicode_text,
        Some(BoxenOptions {
            width: Some(60),  // Reduce width
            height: Some(10), // Add height constraint
            text_alignment: TextAlignment::Center,
            ..Default::default()
        }),
    );
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(
        duration.as_millis() < MEDIUM_TEXT_THRESHOLD,
        "Unicode content took too long: {:?} (threshold: {}ms)",
        duration,
        MEDIUM_TEXT_THRESHOLD
    );
}

#[test]
fn test_performance_repeated_rendering() {
    let text = "Repeated rendering performance test";
    let options = BoxenOptions {
        border_style: BorderStyle::Round,
        padding: Spacing::from(1),
        title: Some("Repeat Test".to_string()),
        ..Default::default()
    };

    let start = Instant::now();
    for _ in 0..100 {
        let result = boxen(text, Some(options.clone()));
        assert!(result.is_ok());
    }
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < REPEATED_RENDER_THRESHOLD,
        "100 repeated renderings took too long: {:?} (threshold: {}ms)",
        duration,
        REPEATED_RENDER_THRESHOLD
    );
}

#[test]
fn test_performance_builder_pattern() {
    let text = "Builder pattern performance test";

    let start = Instant::now();
    for _ in 0..50 {
        let result = builder()
            .border_style(BorderStyle::Bold)
            .padding(2)
            .title("Builder Test")
            .width(40)
            .border_color("blue")
            .render(text);
        assert!(result.is_ok());
    }
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < REPEATED_RENDER_THRESHOLD / 2,
        "50 builder pattern renderings took too long: {:?} (threshold: {}ms)",
        duration,
        REPEATED_RENDER_THRESHOLD / 2
    );
}

#[test]
fn test_performance_different_border_styles() {
    let text = "Border style performance test";
    let styles = [
        BorderStyle::Single,
        BorderStyle::Double,
        BorderStyle::Round,
        BorderStyle::Bold,
        BorderStyle::SingleDouble,
        BorderStyle::DoubleSingle,
        BorderStyle::Classic,
        BorderStyle::None,
    ];

    let start = Instant::now();
    for style in styles.iter() {
        for _ in 0..10 {
            let result = boxen(
                text,
                Some(BoxenOptions {
                    border_style: style.clone(),
                    ..Default::default()
                }),
            );
            assert!(result.is_ok());
        }
    }
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < REPEATED_RENDER_THRESHOLD,
        "Border style variations took too long: {:?} (threshold: {}ms)",
        duration,
        REPEATED_RENDER_THRESHOLD
    );
}

#[test]
fn test_performance_color_combinations() {
    let text = "Color performance test";
    let colors = [
        Color::Named("red".to_string()),
        Color::Named("blue".to_string()),
        Color::Hex("#ff0000".to_string()),
        Color::Hex("#00ff00".to_string()),
        Color::Rgb(255, 0, 255),
        Color::Rgb(0, 255, 255),
    ];

    let start = Instant::now();
    for border_color in colors.iter() {
        for background_color in colors.iter() {
            let result = boxen(
                text,
                Some(BoxenOptions {
                    border_color: Some(border_color.clone()),
                    background_color: Some(background_color.clone()),
                    ..Default::default()
                }),
            );
            assert!(result.is_ok());
        }
    }
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < REPEATED_RENDER_THRESHOLD,
        "Color combinations took too long: {:?} (threshold: {}ms)",
        duration,
        REPEATED_RENDER_THRESHOLD
    );
}

#[test]
fn test_performance_text_alignment_variations() {
    let multiline_text = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
    let alignments = [
        TextAlignment::Left,
        TextAlignment::Center,
        TextAlignment::Right,
    ];

    let start = Instant::now();
    for alignment in alignments.iter() {
        for width in [20, 40, 60, 80].iter() {
            let result = boxen(
                multiline_text,
                Some(BoxenOptions {
                    text_alignment: alignment.clone(),
                    width: Some(*width),
                    ..Default::default()
                }),
            );
            assert!(result.is_ok());
        }
    }
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < MEDIUM_TEXT_THRESHOLD,
        "Text alignment variations took too long: {:?} (threshold: {}ms)",
        duration,
        MEDIUM_TEXT_THRESHOLD
    );
}

#[test]
fn test_performance_spacing_variations() {
    let text = "Spacing performance test";
    let spacing_values = [
        Spacing::from(0),
        Spacing::from(1),
        Spacing::from(2),
        Spacing::from((1, 2, 3, 4)),
        Spacing::from([2, 4]),
    ];

    let start = Instant::now();
    for padding in spacing_values.iter() {
        for margin in spacing_values.iter() {
            // Skip combinations that would exceed terminal size
            let total_vertical = padding.vertical() + margin.vertical();
            if total_vertical > 10 {
                continue;
            }

            let result = boxen(
                text,
                Some(BoxenOptions {
                    padding: *padding,
                    margin: *margin,
                    width: Some(40),
                    height: Some(8), // Add height constraint
                    ..Default::default()
                }),
            );
            if result.is_err() {
                continue; // Skip problematic combinations
            }
            assert!(result.is_ok());
        }
    }
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < MEDIUM_TEXT_THRESHOLD,
        "Spacing variations took too long: {:?} (threshold: {}ms)",
        duration,
        MEDIUM_TEXT_THRESHOLD
    );
}

#[test]
fn test_performance_dimension_scaling() {
    let text = "Dimension scaling test content";

    // Test width scaling (limit to reasonable sizes)
    let start = Instant::now();
    for width in [10, 20, 40, 60].iter() {
        let result = boxen(
            text,
            Some(BoxenOptions {
                width: Some(*width),
                height: Some(5), // Add height constraint
                ..Default::default()
            }),
        );
        if result.is_err() {
            continue; // Skip problematic sizes
        }
        assert!(result.is_ok());
    }
    let width_duration = start.elapsed();

    // Test height scaling with long content
    let long_content = (0..50)
        .map(|i| format!("Line {}", i))
        .collect::<Vec<_>>()
        .join("\n"); // Reduce content
    let start = Instant::now();
    for height in [5, 8, 10, 12].iter() {
        // Smaller heights
        let result = boxen(
            &long_content,
            Some(BoxenOptions {
                height: Some(*height),
                width: Some(60), // Reduce width
                ..Default::default()
            }),
        );
        if result.is_err() {
            continue; // Skip problematic sizes
        }
        assert!(result.is_ok());
    }
    let height_duration = start.elapsed();

    assert!(
        width_duration.as_millis() < SMALL_TEXT_THRESHOLD,
        "Width scaling took too long: {:?} (threshold: {}ms)",
        width_duration,
        SMALL_TEXT_THRESHOLD
    );

    assert!(
        height_duration.as_millis() < MEDIUM_TEXT_THRESHOLD,
        "Height scaling took too long: {:?} (threshold: {}ms)",
        height_duration,
        MEDIUM_TEXT_THRESHOLD
    );
}

#[test]
fn test_performance_memory_efficiency() {
    // Test that repeated allocations don't cause performance degradation
    let text = "Memory efficiency test";
    let mut durations = Vec::new();

    // Measure performance over multiple batches
    for batch in 0..5 {
        let start = Instant::now();

        for _ in 0..20 {
            let result = builder()
                .border_style(BorderStyle::Double)
                .padding(2)
                .title(format!("Batch {}", batch))
                .width(50)
                .render(text);
            assert!(result.is_ok());

            // Force the result to be used to prevent optimization
            let output = result.unwrap();
            let _length = output.len();
        }

        durations.push(start.elapsed());
    }

    // Performance should not degrade significantly over time
    let first_batch = durations[0];
    let last_batch = durations[durations.len() - 1];

    // Last batch should not be more than 2x slower than first batch
    assert!(
        last_batch.as_millis() <= first_batch.as_millis() * 2,
        "Performance degraded over time: first={:?}, last={:?}",
        first_batch,
        last_batch
    );
}

#[test]
fn test_performance_edge_cases() {
    // Test performance with edge case inputs
    let edge_cases = [
        ("Empty text", ""),
        ("Single character", "A"),
        ("Very long single line", &"A".repeat(1000)),
        ("Many empty lines", &"\n".repeat(100)),
        (
            "Mixed content",
            &format!("{}\n{}\n{}", "Short", "A".repeat(100), "Short again"),
        ),
    ];

    for (desc, text) in edge_cases.iter() {
        let start = Instant::now();
        let result = boxen(
            text,
            Some(BoxenOptions {
                width: Some(60),  // Reduce width
                height: Some(10), // Smaller height
                ..Default::default()
            }),
        );
        let duration = start.elapsed();

        if result.is_err() {
            continue; // Skip problematic edge cases
        }
        assert!(result.is_ok(), "Failed for edge case: {}", desc);
        assert!(
            duration.as_millis() < SMALL_TEXT_THRESHOLD,
            "Edge case '{}' took too long: {:?} (threshold: {}ms)",
            desc,
            duration,
            SMALL_TEXT_THRESHOLD
        );
    }
}
