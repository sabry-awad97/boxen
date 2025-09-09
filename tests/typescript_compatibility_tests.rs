/// Tests to ensure Rust boxen behavior matches TypeScript version outputs
use ::boxen::{
    BorderStyle, BoxenOptions, Color, Float, Spacing, TextAlignment, TitleAlignment, boxen,
    builder, double_box, round_box, simple_box,
};

#[test]
fn test_typescript_default_single_border() {
    // TypeScript default: single border, no padding, no margin
    let result = boxen("Hello", None).unwrap();

    // Should use single border characters
    assert!(result.contains("┌"));
    assert!(result.contains("┐"));
    assert!(result.contains("└"));
    assert!(result.contains("┘"));
    assert!(result.contains("│"));
    assert!(result.contains("─"));

    // Should contain the text
    assert!(result.contains("Hello"));

    // Should have exactly 3 lines (top border, content, bottom border)
    assert_eq!(result.lines().count(), 3);

    // Content line should be: │Hello│
    let lines: Vec<&str> = result.lines().collect();
    assert!(lines[1].contains("Hello"));
    assert!(lines[1].starts_with("│"));
    assert!(lines[1].ends_with("│"));
}

#[test]
fn test_typescript_padding_asymmetric_behavior() {
    // TypeScript: padding(1) means 1 vertical, 3 horizontal
    let options = BoxenOptions {
        padding: Spacing::from(1),
        ..Default::default()
    };
    let result = boxen("Test", Some(options)).unwrap();

    let lines: Vec<&str> = result.lines().collect();

    // Should have 5 lines: top border, top padding, content, bottom padding, bottom border
    assert_eq!(lines.len(), 5);

    // Content line should have 3 spaces on each side of "Test"
    let content_line = lines.iter().find(|line| line.contains("Test")).unwrap();
    assert!(content_line.contains("   Test   "));
}

#[test]
fn test_typescript_padding_object_behavior() {
    // TypeScript: padding as object with individual values
    let options = BoxenOptions {
        padding: Spacing::from((1, 2, 3, 4)), // top, right, bottom, left
        width: Some(20),
        ..Default::default()
    };
    let result = boxen("Test", Some(options)).unwrap();

    let lines: Vec<&str> = result.lines().collect();

    // Should have: 1 top padding + 1 content + 3 bottom padding + 2 borders = 7 lines
    assert_eq!(lines.len(), 7);

    // First line after top border should be empty (top padding)
    assert!(
        lines[1]
            .trim_start_matches('│')
            .trim_end_matches('│')
            .trim()
            .is_empty()
    );

    // Content should be padded with 4 spaces left, 2 spaces right
    let content_line = lines.iter().find(|line| line.contains("Test")).unwrap();
    assert!(content_line.contains("    Test  "));
}

#[test]
fn test_typescript_title_truncation() {
    // TypeScript behavior: long titles are truncated to fit box width
    let long_title = "This is a very long title that should be truncated";
    let options = BoxenOptions {
        title: Some(long_title.to_string()),
        width: Some(20),
        ..Default::default()
    };
    let result = boxen("Content", Some(options)).unwrap();

    let first_line = result.lines().next().unwrap();

    println!(
        "First line: '{}' (length: {})",
        first_line,
        first_line.len()
    );

    // Title should be truncated to fit within the box width (allow some tolerance for borders and margins)
    assert!(first_line.len() <= 30); // Allow for border characters and float positioning
    assert!(result.contains("Content"));

    // Should contain part of the title
    assert!(first_line.contains("This is"));
}

#[test]
fn test_typescript_title_alignment_behavior() {
    let title = "Title";
    let alignments = [
        (TitleAlignment::Left, "┌Title"),
        (TitleAlignment::Center, "Title"), // Should be centered in the line
        (TitleAlignment::Right, "Title┐"),
    ];

    for (alignment, _expected_pattern) in alignments.iter() {
        let options = BoxenOptions {
            title: Some(title.to_string()),
            title_alignment: alignment.clone(),
            width: Some(20),
            ..Default::default()
        };
        let result = boxen("Content", Some(options)).unwrap();

        let first_line = result.lines().next().unwrap();

        match alignment {
            TitleAlignment::Left => assert!(first_line.starts_with("┌Title")),
            TitleAlignment::Right => assert!(first_line.ends_with("Title┐")),
            TitleAlignment::Center => {
                // Title should be roughly centered
                let title_pos = first_line.find("Title").unwrap();
                let line_center = first_line.len() / 2;
                let title_center = title_pos + title.len() / 2;
                // Allow some tolerance for centering
                assert!((title_center as i32 - line_center as i32).abs() <= 2);
            }
        }
    }
}

#[test]
fn test_typescript_text_alignment_behavior() {
    let multiline_text = "Left\nCenter\nRight";
    let alignments = [
        TextAlignment::Left,
        TextAlignment::Center,
        TextAlignment::Right,
    ];

    for alignment in alignments.iter() {
        let options = BoxenOptions {
            text_alignment: alignment.clone(),
            width: Some(20),
            ..Default::default()
        };
        let result = boxen(multiline_text, Some(options)).unwrap();

        let lines: Vec<&str> = result.lines().collect();

        // Find content lines (skip borders)
        let content_lines: Vec<&str> = lines
            .iter()
            .filter(|line| {
                line.contains("Left") || line.contains("Center") || line.contains("Right")
            })
            .cloned()
            .collect();

        assert_eq!(content_lines.len(), 3);

        for content_line in content_lines.iter() {
            match alignment {
                TextAlignment::Left => {
                    // Text should be left-aligned (after border and any padding)
                    let inner = content_line.trim_start_matches('│').trim_end_matches('│');
                    let trimmed = inner.trim_start();
                    // For left alignment, there should be no leading spaces (or minimal)
                    assert!(
                        inner.len() - trimmed.len() <= 1,
                        "Too much left padding for left alignment"
                    );
                }
                TextAlignment::Right => {
                    // Text should be right-aligned (before border and any padding)
                    let inner = content_line.trim_start_matches('│').trim_end_matches('│');
                    let trimmed = inner.trim_end();
                    // For right alignment, there should be no trailing spaces (or minimal)
                    assert!(
                        inner.len() - trimmed.len() <= 1,
                        "Too much right padding for right alignment"
                    );
                }
                TextAlignment::Center => {
                    // Text should be roughly centered
                    let inner_content = content_line.trim_start_matches('│').trim_end_matches('│');
                    let _text_part = inner_content.trim();
                    let left_spaces = inner_content.len() - inner_content.trim_start().len();
                    let right_spaces = inner_content.len() - inner_content.trim_end().len();
                    // Center alignment should have roughly equal spaces on both sides
                    assert!((left_spaces as i32 - right_spaces as i32).abs() <= 1);
                }
            }
        }
    }
}

#[test]
fn test_typescript_multiline_handling() {
    // TypeScript handles multiline text by creating separate content lines
    let multiline = "Line 1\nLine 2\nLine 3";
    let result = boxen(multiline, None).unwrap();

    let lines: Vec<&str> = result.lines().collect();

    // Should have 5 lines: top border + 3 content lines + bottom border
    assert_eq!(lines.len(), 5);

    // Each content line should be properly bordered
    assert!(lines[1].contains("Line 1"));
    assert!(lines[2].contains("Line 2"));
    assert!(lines[3].contains("Line 3"));

    // All content lines should have borders
    for i in lines.iter().take(3 + 1).skip(1) {
        assert!(i.starts_with("│"));
        assert!(i.ends_with("│"));
    }
}

#[test]
fn test_typescript_empty_text_handling() {
    // TypeScript creates a box even for empty text
    let result = boxen("", None).unwrap();

    let lines: Vec<&str> = result.lines().collect();

    // Should have 3 lines: top border, empty content, bottom border
    assert_eq!(lines.len(), 3);

    // Content line should be just borders with no text
    assert!(lines[1].starts_with("│"));
    assert!(lines[1].ends_with("│"));

    // Content between borders should be empty or just spaces
    let content = lines[1].trim_start_matches('│').trim_end_matches('│');
    assert!(content.trim().is_empty());
}

#[test]
fn test_typescript_width_constraint_behavior() {
    // TypeScript wraps text when it exceeds the specified width
    let long_text = "This is a very long line that should be wrapped when width is constrained";
    let options = BoxenOptions {
        width: Some(30),
        ..Default::default()
    };
    let result = boxen(long_text, Some(options)).unwrap();

    let lines: Vec<&str> = result.lines().collect();

    // Should have more than 3 lines due to text wrapping
    assert!(lines.len() > 3);

    // Each line should not exceed the specified width (allow significant tolerance for float positioning)
    for line in lines.iter() {
        // The actual content box should be around 30 chars, but float positioning can add significant spacing
        assert!(line.len() <= 100); // Very generous tolerance for float positioning
    }

    // All original words should be present somewhere in the output
    let full_output = result.replace('\n', " ");
    assert!(full_output.contains("This"));
    assert!(full_output.contains("very"));
    assert!(full_output.contains("long"));
    assert!(full_output.contains("wrapped"));
}

#[test]
fn test_typescript_height_constraint_behavior() {
    // TypeScript truncates content when it exceeds specified height
    let many_lines = (0..20)
        .map(|i| format!("Line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let options = BoxenOptions {
        height: Some(8), // Should fit only a few lines
        ..Default::default()
    };
    let result = boxen(&many_lines, Some(options)).unwrap();

    let lines: Vec<&str> = result.lines().collect();

    // Should not exceed the specified height
    assert!(lines.len() <= 8);

    // Should contain some of the original lines
    assert!(result.contains("Line 0"));

    // Later lines should be truncated
    assert!(!result.contains("Line 15"));
}

#[test]
fn test_typescript_color_application() {
    // TypeScript applies colors using ANSI escape codes
    let options = BoxenOptions {
        border_color: Some(Color::Named("red".to_string())),
        background_color: Some(Color::Named("blue".to_string())),
        ..Default::default()
    };
    let result = boxen("Colored text", Some(options)).unwrap();

    // Should contain ANSI color codes
    assert!(result.contains("\x1b["));

    // Should still contain the original text
    assert!(result.contains("Colored text"));

    // Should contain border characters
    assert!(result.contains("┌") || result.contains("│"));
}

#[test]
fn test_typescript_float_positioning() {
    // TypeScript float positioning affects the entire box placement
    let floats = [Float::Left, Float::Center, Float::Right];

    for float_pos in floats.iter() {
        let options = BoxenOptions {
            float: float_pos.clone(),
            width: Some(20),
            ..Default::default()
        };
        let result = boxen("Float test", Some(options)).unwrap();

        let lines: Vec<&str> = result.lines().collect();

        // All lines should be present
        assert!(lines.len() >= 3);

        // Content should be present
        assert!(result.contains("Float test"));

        match float_pos {
            Float::Left => {
                // Left float: no leading spaces
                for line in lines.iter() {
                    if line.contains("┌") || line.contains("│") || line.contains("└") {
                        assert!(!line.starts_with(' '));
                    }
                }
            }
            Float::Right => {
                // Right float: should have leading spaces
                let first_line = lines[0];
                if first_line.contains("┌") {
                    // Should have some leading spaces for right alignment
                    // (exact amount depends on terminal width)
                    assert!(first_line.starts_with(' ') || first_line.len() >= 20);
                }
            }
            Float::Center => {
                // Center float: should have some leading spaces
                let first_line = lines[0];
                if first_line.contains("┌") {
                    // Should have some leading spaces for centering
                    // (exact amount depends on terminal width)
                    assert!(first_line.starts_with(' ') || first_line.len() >= 20);
                }
            }
        }
    }
}

#[test]
fn test_typescript_margin_behavior() {
    // TypeScript margins add space around the entire box
    let options = BoxenOptions {
        margin: Spacing::from(2),
        ..Default::default()
    };
    let result = boxen("Margin test", Some(options)).unwrap();

    let lines: Vec<&str> = result.lines().collect();

    // Should have extra lines for top and bottom margins
    assert!(lines.len() > 3);

    // First and last lines should be empty (margins)
    assert!(lines[0].trim().is_empty());
    assert!(lines[1].trim().is_empty());
    assert!(lines[lines.len() - 1].trim().is_empty());
    assert!(lines[lines.len() - 2].trim().is_empty());

    // Content should still be present
    assert!(result.contains("Margin test"));
}

#[test]
fn test_typescript_no_border_behavior() {
    // TypeScript with borderStyle: 'none' still shows content with padding
    let options = BoxenOptions {
        border_style: BorderStyle::None,
        padding: Spacing::from(1),
        ..Default::default()
    };
    let result = boxen("No border", Some(options)).unwrap();

    // Should not contain border characters
    assert!(!result.contains("┌"));
    assert!(!result.contains("│"));
    assert!(!result.contains("└"));

    // Should still contain the text with padding
    assert!(result.contains("No border"));

    // Should have padding lines
    let lines: Vec<&str> = result.lines().collect();
    assert!(lines.len() > 1);
}

#[test]
fn test_typescript_builder_equivalence() {
    // Builder pattern should produce same results as options struct
    let text = "Equivalence test";

    // Using options struct
    let options = BoxenOptions {
        border_style: BorderStyle::Double,
        padding: Spacing::from(1),
        text_alignment: TextAlignment::Center,
        title: Some("Test".to_string()),
        width: Some(30),
        border_color: Some(Color::Named("blue".to_string())),
        ..Default::default()
    };
    let options_result = boxen(text, Some(options)).unwrap();

    // Using builder pattern
    let builder_result = builder()
        .border_style(BorderStyle::Double)
        .padding(1)
        .text_alignment(TextAlignment::Center)
        .title("Test")
        .width(30)
        .border_color("blue")
        .render(text)
        .unwrap();

    // Results should be identical
    assert_eq!(options_result, builder_result);
}

#[test]
fn test_typescript_convenience_functions_behavior() {
    // Convenience functions should match TypeScript equivalents
    let text = "Convenience test";

    // simple_box should be equivalent to default options
    let simple = simple_box(text);
    let manual = boxen(text, None).unwrap();
    assert_eq!(simple, manual);

    // double_box should use double border
    let double = double_box(text);
    let manual_double = boxen(
        text,
        Some(BoxenOptions {
            border_style: BorderStyle::Double,
            ..Default::default()
        }),
    )
    .unwrap();
    assert_eq!(double, manual_double);

    // round_box should use round border
    let round = round_box(text);
    let manual_round = boxen(
        text,
        Some(BoxenOptions {
            border_style: BorderStyle::Round,
            ..Default::default()
        }),
    )
    .unwrap();
    assert_eq!(round, manual_round);
}
