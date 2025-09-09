/// Core boxen rendering functionality
use crate::error::BoxenResult;
use crate::options::{BoxenOptions, Spacing, TitleAlignment};
use crate::text::text_width;
use crate::text::wrapping::wrap_text;
use std::fmt::Write;

/// Main boxen function that renders text within a styled box
pub fn boxen<S: AsRef<str>>(text: S, options: Option<BoxenOptions>) -> BoxenResult<String> {
    let text = text.as_ref();
    let options = options.unwrap_or_default();

    // Process the text content
    let processed_content = process_content(text, &options)?;

    // Calculate final layout dimensions
    let layout = options.calculate_layout_dimensions(
        processed_content.content_width,
        processed_content.content_height,
    )?;

    // Render the box
    render_box(&processed_content, &options, &layout)
}

/// Processed text content with dimensions
#[derive(Debug)]
struct ProcessedContent {
    lines: Vec<String>,
    content_width: usize,
    content_height: usize,
}

/// Process text content according to options
fn process_content(text: &str, options: &BoxenOptions) -> BoxenResult<ProcessedContent> {
    // Calculate maximum content width available
    let max_content_width = options.calculate_max_content_width()?;

    // Wrap text if needed
    let wrapped_lines = if text.is_empty() {
        vec![String::new()]
    } else {
        wrap_text(text, max_content_width)?
    };

    // Calculate the natural width of the content
    let natural_content_width = wrapped_lines
        .iter()
        .map(|line| text_width(line))
        .max()
        .unwrap_or(0);

    // Use the smaller of natural width or max width for better layout
    // But if a specific width is set, use the max available content width
    let target_width = if options.width.is_some() {
        max_content_width
    } else {
        natural_content_width.min(max_content_width)
    };

    // Apply text alignment without padding (padding will be applied during rendering)
    let aligned_lines =
        crate::text::align_lines(&wrapped_lines, options.text_alignment.clone(), target_width);

    let content_height = aligned_lines.len();

    Ok(ProcessedContent {
        lines: aligned_lines,
        content_width: target_width,
        content_height,
    })
}

/// Render the complete box with borders, content, and margins
fn render_box(
    content: &ProcessedContent,
    options: &BoxenOptions,
    layout: &crate::options::LayoutDimensions,
) -> BoxenResult<String> {
    let border_chars = options.border_style.get_chars()?;
    let has_border = options.border_style.is_visible();

    let mut result = String::new();

    // Add top margins
    for _ in 0..options.margin.top {
        writeln!(result).unwrap();
    }

    if has_border {
        // Render top border with title
        let top_border = render_top_border(&border_chars, &options, layout.inner_width)?;
        add_line_with_margins(&mut result, &top_border, &options.margin);

        // Render content lines with borders and padding
        render_content_with_borders(&mut result, content, options, layout, &border_chars)?;

        // Render bottom border
        let bottom_border = render_bottom_border(&border_chars, layout.inner_width);
        add_line_with_margins(&mut result, &bottom_border, &options.margin);
    } else {
        // No border - just render content with padding and margins
        render_content_without_borders(&mut result, content, options, layout)?;
    }

    // Add bottom margins as empty lines
    for _ in 0..options.margin.bottom {
        result.push_str("\n");
    }

    // Only remove the final newline if there are no bottom margins
    if options.margin.bottom == 0 && result.ends_with('\n') {
        result.pop();
    }

    Ok(result)
}

/// Render the top border with optional title embedding
fn render_top_border(
    border_chars: &crate::options::BorderChars,
    options: &BoxenOptions,
    inner_width: usize,
) -> BoxenResult<String> {
    let mut border = String::new();

    // Start with left corner
    border.push(border_chars.top_left);

    if let Some(title) = &options.title {
        render_top_border_with_title(&mut border, title, border_chars, options, inner_width)?;
    } else {
        // No title - just fill with horizontal border chars
        for _ in 0..inner_width {
            border.push(border_chars.top);
        }
    }

    // End with right corner
    border.push(border_chars.top_right);

    Ok(border)
}

/// Render top border with embedded title
fn render_top_border_with_title(
    border: &mut String,
    title: &str,
    border_chars: &crate::options::BorderChars,
    options: &BoxenOptions,
    inner_width: usize,
) -> BoxenResult<()> {
    let title_width = text_width(title);

    // If title is too long, truncate it
    let effective_title = if title_width > inner_width {
        // Truncate title to fit
        let mut truncated = String::new();
        let mut current_width = 0;
        for ch in title.chars() {
            let char_width = text_width(&ch.to_string());
            if current_width + char_width > inner_width {
                break;
            }
            truncated.push(ch);
            current_width += char_width;
        }
        truncated
    } else {
        title.to_string()
    };

    let effective_title_width = text_width(&effective_title);
    let remaining_width = inner_width - effective_title_width;

    match options.title_alignment {
        TitleAlignment::Left => {
            border.push_str(&effective_title);
            for _ in 0..remaining_width {
                border.push(border_chars.top);
            }
        }
        TitleAlignment::Right => {
            for _ in 0..remaining_width {
                border.push(border_chars.top);
            }
            border.push_str(&effective_title);
        }
        TitleAlignment::Center => {
            let left_padding = remaining_width / 2;
            let right_padding = remaining_width - left_padding;

            for _ in 0..left_padding {
                border.push(border_chars.top);
            }
            border.push_str(&effective_title);
            for _ in 0..right_padding {
                border.push(border_chars.top);
            }
        }
    }

    Ok(())
}

/// Render the bottom border
fn render_bottom_border(border_chars: &crate::options::BorderChars, inner_width: usize) -> String {
    let mut border = String::new();

    border.push(border_chars.bottom_left);
    for _ in 0..inner_width {
        border.push(border_chars.bottom);
    }
    border.push(border_chars.bottom_right);

    border
}

/// Render content lines with left and right borders and padding
fn render_content_with_borders(
    result: &mut String,
    content: &ProcessedContent,
    options: &BoxenOptions,
    layout: &crate::options::LayoutDimensions,
    border_chars: &crate::options::BorderChars,
) -> BoxenResult<()> {
    // Add top padding
    for _ in 0..options.padding.top {
        let padded_line = render_padded_empty_line(border_chars, layout.inner_width);
        add_line_with_margins(result, &padded_line, &options.margin);
    }

    // Render content lines
    for line in &content.lines {
        let content_line = render_content_line(line, border_chars, options, layout.inner_width)?;
        add_line_with_margins(result, &content_line, &options.margin);
    }

    // Add bottom padding
    for _ in 0..options.padding.bottom {
        let padded_line = render_padded_empty_line(border_chars, layout.inner_width);
        add_line_with_margins(result, &padded_line, &options.margin);
    }

    Ok(())
}

/// Render content without borders (border style is None)
fn render_content_without_borders(
    result: &mut String,
    content: &ProcessedContent,
    options: &BoxenOptions,
    layout: &crate::options::LayoutDimensions,
) -> BoxenResult<()> {
    // Render title if present (requirement 5.4)
    if let Some(title) = &options.title {
        let title_line = render_title_without_border(title, options, layout.inner_width)?;
        add_line_with_margins(result, &title_line, &options.margin);
    }

    // Add top padding
    for _ in 0..options.padding.top {
        let empty_line = " ".repeat(layout.inner_width);
        add_line_with_margins(result, &empty_line, &options.margin);
    }

    // Render content lines with padding
    for line in &content.lines {
        let padded_line = format!(
            "{}{}{}",
            " ".repeat(options.padding.left),
            line,
            " ".repeat(options.padding.right)
        );
        add_line_with_margins(result, &padded_line, &options.margin);
    }

    // Add bottom padding
    for _ in 0..options.padding.bottom {
        let empty_line = " ".repeat(layout.inner_width);
        add_line_with_margins(result, &empty_line, &options.margin);
    }

    Ok(())
}

/// Render a single content line with borders and padding
fn render_content_line(
    line: &str,
    border_chars: &crate::options::BorderChars,
    options: &BoxenOptions,
    inner_width: usize,
) -> BoxenResult<String> {
    let mut content_line = String::new();

    // Left border
    content_line.push(border_chars.left);

    // Left padding
    for _ in 0..options.padding.left {
        content_line.push(' ');
    }

    // Content
    content_line.push_str(line);

    // Right padding (fill to inner width)
    let current_content_width = text_width(&content_line) - 1; // Subtract left border
    let remaining_width = inner_width - current_content_width;
    for _ in 0..remaining_width {
        content_line.push(' ');
    }

    // Right border
    content_line.push(border_chars.right);

    Ok(content_line)
}

/// Render an empty line with borders and padding (for top/bottom padding)
fn render_padded_empty_line(
    border_chars: &crate::options::BorderChars,
    inner_width: usize,
) -> String {
    let mut line = String::new();

    line.push(border_chars.left);
    for _ in 0..inner_width {
        line.push(' ');
    }
    line.push(border_chars.right);

    line
}

/// Render title without border (for BorderStyle::None)
fn render_title_without_border(
    title: &str,
    options: &BoxenOptions,
    inner_width: usize,
) -> BoxenResult<String> {
    let title_width = text_width(title);

    // If title is too long, truncate it
    let effective_title = if title_width > inner_width {
        // Truncate title to fit
        let mut truncated = String::new();
        let mut current_width = 0;
        for ch in title.chars() {
            let char_width = text_width(&ch.to_string());
            if current_width + char_width > inner_width {
                break;
            }
            truncated.push(ch);
            current_width += char_width;
        }
        truncated
    } else {
        title.to_string()
    };

    let effective_title_width = text_width(&effective_title);
    let remaining_width = inner_width - effective_title_width;

    let title_line = match options.title_alignment {
        TitleAlignment::Left => {
            format!("{}{}", effective_title, " ".repeat(remaining_width))
        }
        TitleAlignment::Right => {
            format!("{}{}", " ".repeat(remaining_width), effective_title)
        }
        TitleAlignment::Center => {
            let left_padding = remaining_width / 2;
            let right_padding = remaining_width - left_padding;
            format!(
                "{}{}{}",
                " ".repeat(left_padding),
                effective_title,
                " ".repeat(right_padding)
            )
        }
    };

    Ok(title_line)
}

/// Add a line to the result with left and right margins
fn add_line_with_margins(result: &mut String, line: &str, margin: &Spacing) {
    // Left margin
    for _ in 0..margin.left {
        result.push(' ');
    }

    // Content
    result.push_str(line);

    // Right margin
    for _ in 0..margin.right {
        result.push(' ');
    }

    // Newline
    result.push('\n');
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::{BorderStyle, BoxenOptions, Spacing, TextAlignment, TitleAlignment};

    #[test]
    fn test_basic_box_rendering() {
        let result = boxen("Hello", None).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Should have single-line borders (default)
        assert!(result.contains("┌")); // Top-left corner
        assert!(result.contains("┐")); // Top-right corner
        assert!(result.contains("└")); // Bottom-left corner
        assert!(result.contains("┘")); // Bottom-right corner
        assert!(result.contains("│")); // Vertical borders
        assert!(result.contains("─")); // Horizontal borders

        // Should be 3 lines (top border, content, bottom border)
        assert_eq!(result.lines().count(), 3);
    }

    #[test]
    fn test_empty_text() {
        let result = boxen("", None).unwrap();

        // Should still render a box
        assert!(result.contains("┌"));
        assert!(result.contains("┐"));
        assert!(result.contains("└"));
        assert!(result.contains("┘"));

        // Should be 3 lines
        assert_eq!(result.lines().count(), 3);
    }

    #[test]
    fn test_multiline_text() {
        let result = boxen("Hello\nWorld", None).unwrap();

        // Should contain both lines
        assert!(result.contains("Hello"));
        assert!(result.contains("World"));

        // Should be 4 lines (top border, 2 content lines, bottom border)
        assert_eq!(result.lines().count(), 4);
    }

    #[test]
    fn test_no_border_style() {
        let options = BoxenOptions {
            border_style: BorderStyle::None,
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Should not contain border characters
        assert!(!result.contains("┌"));
        assert!(!result.contains("│"));

        // Should be 1 line (just content)
        assert_eq!(result.lines().count(), 1);
    }

    #[test]
    fn test_different_border_styles() {
        let styles = vec![
            BorderStyle::Single,
            BorderStyle::Double,
            BorderStyle::Round,
            BorderStyle::Bold,
            BorderStyle::Classic,
        ];

        for style in styles {
            let options = BoxenOptions {
                border_style: style,
                ..Default::default()
            };

            let result = boxen("Test", Some(options)).unwrap();

            // Should contain the text
            assert!(result.contains("Test"));

            // Should be 3 lines
            assert_eq!(result.lines().count(), 3);
        }
    }

    #[test]
    fn test_padding() {
        let options = BoxenOptions {
            padding: Spacing {
                top: 1,
                right: 2,
                bottom: 1,
                left: 2,
            },
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should be 5 lines (top border, top padding, content, bottom padding, bottom border)
        assert_eq!(result.lines().count(), 5);

        // Content line should have padding
        let lines: Vec<&str> = result.lines().collect();
        let content_line = lines[2]; // Middle line should be content
        assert!(content_line.contains("Hello"));

        // Should have left padding (2 spaces after left border)
        assert!(content_line.starts_with("│  Hello"));
    }

    #[test]
    fn test_margins() {
        let options = BoxenOptions {
            margin: Spacing {
                top: 1,
                right: 1,
                bottom: 1,
                left: 1,
            },
            ..Default::default() // Don't specify width to avoid calculation issues
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should be 5 lines (top margin, top border, content, bottom border, bottom margin)
        assert_eq!(result.lines().count(), 5);

        // First line should be empty (top margin)
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines[0], "");

        // Last line should be empty (bottom margin)
        assert_eq!(lines[4], "");

        // Border lines should have left margin (1 space)
        assert!(lines[1].starts_with(" ┌"));
        assert!(lines[3].starts_with(" └"));
    }

    #[test]
    fn test_title_basic() {
        let options = BoxenOptions {
            title: Some("Title".to_string()),
            ..Default::default()
        };

        let result = boxen("Content", Some(options)).unwrap();

        // Should contain both title and content
        assert!(result.contains("Title"));
        assert!(result.contains("Content"));

        // Title should be in the top border
        let lines: Vec<&str> = result.lines().collect();
        let top_border = lines[0];
        assert!(top_border.contains("Title"));
        assert!(top_border.starts_with("┌"));
        assert!(top_border.ends_with("┐"));
    }

    #[test]
    fn test_title_alignment() {
        let alignments = vec![
            TitleAlignment::Left,
            TitleAlignment::Center,
            TitleAlignment::Right,
        ];

        for alignment in alignments {
            let options = BoxenOptions {
                title: Some("Title".to_string()),
                title_alignment: alignment,
                width: Some(20), // Fixed width for predictable alignment
                ..Default::default()
            };

            let result = boxen("Content", Some(options)).unwrap();

            // Should contain the title
            assert!(result.contains("Title"));

            let lines: Vec<&str> = result.lines().collect();
            let top_border = lines[0];
            assert!(top_border.contains("Title"));
        }
    }

    #[test]
    fn test_title_truncation() {
        let options = BoxenOptions {
            title: Some("Very Long Title That Should Be Truncated".to_string()),
            width: Some(15), // Small width to force truncation
            ..Default::default()
        };

        let result = boxen("Content", Some(options)).unwrap();

        // Should contain some part of the title
        let lines: Vec<&str> = result.lines().collect();
        let top_border = lines[0];

        // Should start and end with border characters
        assert!(top_border.starts_with("┌"));
        assert!(top_border.ends_with("┐"));

        // Should be exactly the specified width
        assert_eq!(text_width(top_border), 15);
    }

    #[test]
    fn test_text_alignment() {
        let alignments = vec![
            TextAlignment::Left,
            TextAlignment::Center,
            TextAlignment::Right,
        ];

        for alignment in alignments {
            let options = BoxenOptions {
                text_alignment: alignment,
                width: Some(20), // Fixed width for predictable alignment
                ..Default::default()
            };

            let result = boxen("Hello", Some(options)).unwrap();

            // Should contain the text
            assert!(result.contains("Hello"));

            // Should be 3 lines
            assert_eq!(result.lines().count(), 3);
        }
    }

    #[test]
    fn test_complex_configuration() {
        let options = BoxenOptions {
            border_style: BorderStyle::Double,
            padding: Spacing {
                top: 1,
                right: 1,
                bottom: 1,
                left: 1,
            },
            margin: Spacing {
                top: 1,
                right: 1,
                bottom: 1,
                left: 1,
            },
            title: Some("Test".to_string()),
            title_alignment: TitleAlignment::Center,
            text_alignment: TextAlignment::Center,
            ..Default::default() // Don't specify width to avoid calculation issues
        };

        let result = boxen("Hello World", Some(options)).unwrap();

        // Should contain all elements
        assert!(result.contains("Test"));
        assert!(result.contains("Hello World"));

        // Should use double border
        assert!(result.contains("╔"));
        assert!(result.contains("╗"));
        assert!(result.contains("╚"));
        assert!(result.contains("╝"));

        // Should have correct number of lines
        // 1 top margin + 1 top border + 1 top padding + 1 content + 1 bottom padding + 1 bottom border + 1 bottom margin
        assert_eq!(result.lines().count(), 7);
    }

    #[test]
    fn test_width_constraint() {
        let options = BoxenOptions {
            width: Some(10),
            ..Default::default()
        };

        let result = boxen("Hi", Some(options)).unwrap();

        // Each line should be exactly 10 characters wide (the specified width)
        for line in result.lines() {
            assert_eq!(text_width(line), 10);
        }
    }

    #[test]
    fn test_process_content() {
        let options = BoxenOptions::default();
        let content = process_content("Hello\nWorld", &options).unwrap();

        assert_eq!(content.lines.len(), 2);
        assert!(content.lines[0].contains("Hello"));
        assert!(content.lines[1].contains("World"));
        // Content width should be the natural text width (5 for "World")
        assert_eq!(content.content_width, 5);
        assert_eq!(content.content_height, 2);
    }

    #[test]
    fn test_render_top_border_no_title() {
        use crate::options::BorderChars;

        let border_chars = BorderChars::single();
        let options = BoxenOptions::default();
        let result = render_top_border(&border_chars, &options, 10).unwrap();

        assert_eq!(result, "┌──────────┐");
        assert_eq!(text_width(&result), 12); // 10 + 2 corners
    }

    #[test]
    fn test_render_bottom_border() {
        use crate::options::BorderChars;

        let border_chars = BorderChars::single();
        let result = render_bottom_border(&border_chars, 10);

        assert_eq!(result, "└──────────┘");
        assert_eq!(text_width(&result), 12); // 10 + 2 corners
    }

    #[test]
    fn test_add_line_with_margins() {
        let mut result = String::new();
        let margin = Spacing {
            top: 0,
            right: 2,
            bottom: 0,
            left: 3,
        };

        add_line_with_margins(&mut result, "test", &margin);

        assert_eq!(result, "   test  \n");
    }

    #[test]
    fn test_error_handling() {
        // Test with invalid width constraint
        let options = BoxenOptions {
            width: Some(1),            // Too small for any content
            padding: Spacing::from(2), // Large padding
            ..Default::default()
        };

        let result = boxen("Hello", Some(options));
        assert!(result.is_err());
    }

    #[test]
    fn test_unicode_content() {
        let result = boxen("你好世界", None).unwrap();

        // Should contain the Unicode text
        assert!(result.contains("你好世界"));

        // Should still render properly
        assert!(result.contains("┌"));
        assert!(result.contains("┐"));
        assert_eq!(result.lines().count(), 3);
    }

    #[test]
    fn test_long_text_wrapping() {
        let long_text = "This is a very long line of text that should be wrapped when it exceeds the available width";
        let options = BoxenOptions {
            width: Some(30),
            ..Default::default()
        };

        let result = boxen(long_text, Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("This is a very long"));

        // Should have multiple content lines due to wrapping
        assert!(result.lines().count() > 3); // More than just top border, content, bottom border
    }

    #[test]
    fn test_width_calculation_fix() {
        // This test verifies that the width calculation issue is fixed
        // Previously, specifying width: Some(70) would cause the calculated width
        // to grow by 2 each time, leading to errors like "72 exceeds 70"

        let options = BoxenOptions {
            margin: Spacing {
                top: 1,
                right: 1,
                bottom: 1,
                left: 1,
            },
            width: Some(70),
            ..Default::default()
        };

        // This should work without errors
        let result = boxen("Hello", Some(options)).unwrap();

        // Verify the result has the expected structure
        assert_eq!(result.lines().count(), 5); // top margin, top border, content, bottom border, bottom margin

        // The box content lines should have the correct total width (70)
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(text_width(lines[1]), 70); // top border line
        assert_eq!(text_width(lines[2]), 70); // content line  
        assert_eq!(text_width(lines[3]), 70); // bottom border line

        // Margin lines should be empty
        assert_eq!(text_width(lines[0]), 0); // top margin
        assert_eq!(text_width(lines[4]), 0); // bottom margin
    }

    #[test]
    fn test_title_with_no_border_left_alignment() {
        let options = BoxenOptions {
            title: Some("Left Title".to_string()),
            title_alignment: TitleAlignment::Left,
            border_style: BorderStyle::None,
            width: Some(20),
            ..Default::default()
        };

        let result = boxen("Content", Some(options)).unwrap();

        // Should contain both title and content
        assert!(result.contains("Left Title"));
        assert!(result.contains("Content"));

        // Should be 2 lines (title line, content line)
        assert_eq!(result.lines().count(), 2);

        let lines: Vec<&str> = result.lines().collect();
        let title_line = lines[0];

        // Title should be left-aligned
        assert!(title_line.starts_with("Left Title"));
        assert_eq!(text_width(title_line), 20);
    }

    #[test]
    fn test_title_with_no_border_center_alignment() {
        let options = BoxenOptions {
            title: Some("Center".to_string()),
            title_alignment: TitleAlignment::Center,
            border_style: BorderStyle::None,
            width: Some(20),
            ..Default::default()
        };

        let result = boxen("Content", Some(options)).unwrap();

        let lines: Vec<&str> = result.lines().collect();
        let title_line = lines[0];

        // Title should be centered (with spaces on both sides)
        assert!(title_line.contains("Center"));
        assert_eq!(text_width(title_line), 20);

        // Should have roughly equal padding on both sides
        let title_start = title_line.find("Center").unwrap();
        assert!(title_start > 5); // Should have some left padding
    }

    #[test]
    fn test_title_with_no_border_right_alignment() {
        let options = BoxenOptions {
            title: Some("Right Title".to_string()),
            title_alignment: TitleAlignment::Right,
            border_style: BorderStyle::None,
            width: Some(20),
            ..Default::default()
        };

        let result = boxen("Content", Some(options)).unwrap();

        let lines: Vec<&str> = result.lines().collect();
        let title_line = lines[0];

        // Title should be right-aligned
        assert!(title_line.ends_with("Right Title"));
        assert_eq!(text_width(title_line), 20);
    }

    #[test]
    fn test_title_truncation_no_border() {
        let options = BoxenOptions {
            title: Some("This is a very long title that should be truncated".to_string()),
            border_style: BorderStyle::None,
            width: Some(15),
            ..Default::default()
        };

        let result = boxen("Content", Some(options)).unwrap();

        let lines: Vec<&str> = result.lines().collect();
        let title_line = lines[0];

        // Title should be truncated to fit width
        assert_eq!(text_width(title_line), 15);
        assert!(title_line.contains("This is a very"));
        assert!(!title_line.contains("truncated")); // Should be cut off
    }

    #[test]
    fn test_title_with_no_border_and_padding() {
        let options = BoxenOptions {
            title: Some("Title".to_string()),
            border_style: BorderStyle::None,
            padding: Spacing {
                top: 1,
                right: 2,
                bottom: 1,
                left: 2,
            },
            width: Some(20),
            ..Default::default()
        };

        let result = boxen("Content", Some(options)).unwrap();

        // Should have title, top padding, content, bottom padding
        assert_eq!(result.lines().count(), 4);

        let lines: Vec<&str> = result.lines().collect();
        assert!(lines[0].contains("Title")); // Title line
        assert_eq!(lines[1].trim(), ""); // Top padding (empty line)
        assert!(lines[2].contains("Content")); // Content with padding
        assert_eq!(lines[3].trim(), ""); // Bottom padding (empty line)
    }

    #[test]
    fn test_title_with_unicode_characters() {
        let options = BoxenOptions {
            title: Some("测试标题".to_string()),
            title_alignment: TitleAlignment::Center,
            width: Some(20),
            ..Default::default()
        };

        let result = boxen("内容", Some(options)).unwrap();

        // Should contain Unicode title and content
        assert!(result.contains("测试标题"));
        assert!(result.contains("内容"));

        let lines: Vec<&str> = result.lines().collect();
        let top_border = lines[0];
        assert!(top_border.contains("测试标题"));
    }

    #[test]
    fn test_title_edge_cases() {
        // Empty title
        let options = BoxenOptions {
            title: Some("".to_string()),
            ..Default::default()
        };
        let result = boxen("Content", Some(options)).unwrap();
        assert!(result.contains("Content"));

        // Title same width as box
        let options = BoxenOptions {
            title: Some("12345".to_string()),
            width: Some(7), // 5 content + 2 borders = 7
            ..Default::default()
        };
        let result = boxen("X", Some(options)).unwrap();
        assert!(result.contains("12345"));

        // Single character title
        let options = BoxenOptions {
            title: Some("T".to_string()),
            title_alignment: TitleAlignment::Center,
            width: Some(10),
            ..Default::default()
        };
        let result = boxen("Content", Some(options)).unwrap();
        assert!(result.contains("T"));
    }

    #[test]
    fn test_render_title_without_border_function() {
        let options = BoxenOptions {
            title_alignment: TitleAlignment::Left,
            ..Default::default()
        };

        // Test basic functionality
        let result = render_title_without_border("Test Title", &options, 20).unwrap();
        assert_eq!(text_width(&result), 20);
        assert!(result.starts_with("Test Title"));

        // Test center alignment
        let options = BoxenOptions {
            title_alignment: TitleAlignment::Center,
            ..Default::default()
        };
        let result = render_title_without_border("Test", &options, 20).unwrap();
        assert_eq!(text_width(&result), 20);
        assert!(result.contains("Test"));

        // Test right alignment
        let options = BoxenOptions {
            title_alignment: TitleAlignment::Right,
            ..Default::default()
        };
        let result = render_title_without_border("Test", &options, 20).unwrap();
        assert_eq!(text_width(&result), 20);
        assert!(result.ends_with("Test"));

        // Test truncation
        let result = render_title_without_border("Very Long Title", &options, 5).unwrap();
        assert_eq!(text_width(&result), 5);
        assert_eq!(result, "Very ");
    }
}
