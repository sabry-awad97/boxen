/// Text alignment functionality
use crate::options::{Spacing, TextAlignment};
use crate::text::measurement::text_width;

/// Align a single line of text within a given width
pub fn align_line(line: &str, alignment: TextAlignment, width: usize) -> String {
    let line_width = text_width(line);

    // If line is already wider than target width, return as-is
    if line_width >= width {
        return line.to_string();
    }

    let padding_needed = width - line_width;

    match alignment {
        TextAlignment::Left => {
            format!("{}{}", line, " ".repeat(padding_needed))
        }
        TextAlignment::Right => {
            format!("{}{}", " ".repeat(padding_needed), line)
        }
        TextAlignment::Center => {
            let left_padding = padding_needed / 2;
            let right_padding = padding_needed - left_padding;
            format!(
                "{}{}{}",
                " ".repeat(left_padding),
                line,
                " ".repeat(right_padding)
            )
        }
    }
}

/// Align multiple lines of text within a given width
pub fn align_lines(lines: &[String], alignment: TextAlignment, width: usize) -> Vec<String> {
    lines
        .iter()
        .map(|line| align_line(line, alignment.clone(), width))
        .collect()
}

/// Apply padding to text content
pub fn apply_padding(lines: &[String], padding: &Spacing, content_width: usize) -> Vec<String> {
    let mut result = Vec::new();

    // Add top padding (empty lines)
    for _ in 0..padding.top {
        result.push(" ".repeat(content_width));
    }

    // Add left and right padding to each content line
    for line in lines {
        let mut padded_line = String::with_capacity(content_width);

        // Left padding
        padded_line.push_str(&" ".repeat(padding.left));

        // Original content
        padded_line.push_str(line);

        // Right padding (fill to content width)
        let current_width = text_width(&padded_line);
        if current_width < content_width {
            padded_line.push_str(&" ".repeat(content_width - current_width));
        }

        result.push(padded_line);
    }

    // Add bottom padding (empty lines)
    for _ in 0..padding.bottom {
        result.push(" ".repeat(content_width));
    }

    result
}

/// Calculate the content width needed for text with padding
pub fn calculate_content_width(text_lines: &[String], padding: &Spacing) -> usize {
    let max_text_width = text_lines
        .iter()
        .map(|line| text_width(line))
        .max()
        .unwrap_or(0);

    max_text_width + padding.left + padding.right
}

/// Calculate the content height needed for text with padding
pub fn calculate_content_height(text_lines: &[String], padding: &Spacing) -> usize {
    text_lines.len() + padding.top + padding.bottom
}

/// Process text with alignment and padding
pub fn process_text_alignment(
    text: &str,
    alignment: TextAlignment,
    padding: &Spacing,
    target_width: Option<usize>,
) -> Vec<String> {
    // Handle empty text case - create one empty line
    let lines: Vec<String> = if text.is_empty() {
        vec![String::new()]
    } else {
        text.lines().map(|s| s.to_string()).collect()
    };

    // Calculate dimensions
    let content_width = if let Some(width) = target_width {
        // Use specified width, accounting for padding
        if width > padding.left + padding.right {
            width - padding.left - padding.right
        } else {
            // When target width is too small, use natural content width
            lines.iter().map(|line| text_width(line)).max().unwrap_or(0)
        }
    } else {
        // Use natural width of content
        lines.iter().map(|line| text_width(line)).max().unwrap_or(0)
    };

    // Align the text lines
    let aligned_lines = align_lines(&lines, alignment, content_width);

    // Apply padding
    let total_content_width = content_width + padding.left + padding.right;
    apply_padding(&aligned_lines, padding, total_content_width)
}

/// Process text with alignment, padding, and height constraints
pub fn process_text_with_height_constraints(
    text: &str,
    alignment: TextAlignment,
    padding: &Spacing,
    target_width: Option<usize>,
    max_content_height: Option<usize>,
) -> Vec<String> {
    // Handle empty text case - create one empty line
    let lines: Vec<String> = if text.is_empty() {
        vec![String::new()]
    } else {
        text.lines().map(|s| s.to_string()).collect()
    };

    // Calculate dimensions
    let content_width = if let Some(width) = target_width {
        // Use specified width, accounting for padding
        if width > padding.left + padding.right {
            width - padding.left - padding.right
        } else {
            // When target width is too small, use natural content width
            lines.iter().map(|line| text_width(line)).max().unwrap_or(0)
        }
    } else {
        // Use natural width of content
        lines.iter().map(|line| text_width(line)).max().unwrap_or(0)
    };

    // Apply height constraints if specified
    let constrained_lines = if let Some(max_height) = max_content_height {
        apply_height_constraints(&lines, max_height)
    } else {
        lines
    };

    // Align the text lines
    let aligned_lines = align_lines(&constrained_lines, alignment, content_width);

    // Apply padding
    let total_content_width = content_width + padding.left + padding.right;
    apply_padding(&aligned_lines, padding, total_content_width)
}

/// Apply height constraints to text lines - truncate or pad as needed
pub fn apply_height_constraints(lines: &[String], max_height: usize) -> Vec<String> {
    if max_height == 0 {
        // If max height is 0, return empty content
        return vec![];
    }

    if lines.len() <= max_height {
        // Content fits within height constraint - pad if needed
        let mut result = lines.to_vec();

        // Add empty lines to reach the target height
        while result.len() < max_height {
            result.push(String::new());
        }

        result
    } else {
        // Content exceeds height constraint - truncate
        lines[..max_height].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_line_left() {
        assert_eq!(align_line("hello", TextAlignment::Left, 10), "hello     ");
        assert_eq!(align_line("", TextAlignment::Left, 5), "     ");
        assert_eq!(align_line("exact", TextAlignment::Left, 5), "exact");
    }

    #[test]
    fn test_align_line_right() {
        assert_eq!(align_line("hello", TextAlignment::Right, 10), "     hello");
        assert_eq!(align_line("", TextAlignment::Right, 5), "     ");
        assert_eq!(align_line("exact", TextAlignment::Right, 5), "exact");
    }

    #[test]
    fn test_align_line_center() {
        assert_eq!(align_line("hello", TextAlignment::Center, 10), "  hello   ");
        assert_eq!(align_line("hi", TextAlignment::Center, 6), "  hi  ");
        assert_eq!(align_line("odd", TextAlignment::Center, 7), "  odd  "); // Left-biased for odd padding
        assert_eq!(align_line("", TextAlignment::Center, 4), "    ");
    }

    #[test]
    fn test_align_line_with_unicode() {
        // Wide characters should be handled correctly
        assert_eq!(align_line("你好", TextAlignment::Left, 6), "你好  ");
        assert_eq!(align_line("你好", TextAlignment::Right, 6), "  你好");
        assert_eq!(align_line("你好", TextAlignment::Center, 6), " 你好 ");
    }

    #[test]
    fn test_align_line_with_ansi() {
        let colored_text = "\x1b[31mred\x1b[0m";
        assert_eq!(
            align_line(colored_text, TextAlignment::Left, 6),
            format!("{}   ", colored_text)
        );
        assert_eq!(
            align_line(colored_text, TextAlignment::Right, 6),
            format!("   {}", colored_text)
        );
    }

    #[test]
    fn test_align_line_overflow() {
        // When text is wider than target, return as-is
        assert_eq!(align_line("toolong", TextAlignment::Left, 5), "toolong");
        assert_eq!(align_line("toolong", TextAlignment::Right, 5), "toolong");
        assert_eq!(align_line("toolong", TextAlignment::Center, 5), "toolong");
    }

    #[test]
    fn test_align_lines() {
        let lines = vec!["hello".to_string(), "world".to_string()];
        let result = align_lines(&lines, TextAlignment::Center, 10);
        assert_eq!(result, vec!["  hello   ", "  world   "]);
    }

    #[test]
    fn test_apply_padding_basic() {
        let lines = vec!["hello".to_string(), "world".to_string()];
        let padding = Spacing {
            top: 1,
            right: 2,
            bottom: 1,
            left: 2,
        };
        let result = apply_padding(&lines, &padding, 9); // 5 + 2 + 2 = 9

        assert_eq!(result.len(), 4); // 1 top + 2 content + 1 bottom
        assert_eq!(result[0], "         "); // Top padding line
        assert_eq!(result[1], "  hello  "); // First content line with padding
        assert_eq!(result[2], "  world  "); // Second content line with padding
        assert_eq!(result[3], "         "); // Bottom padding line
    }

    #[test]
    fn test_apply_padding_no_padding() {
        let lines = vec!["hello".to_string()];
        let padding = Spacing::default();
        let result = apply_padding(&lines, &padding, 5);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "hello");
    }

    #[test]
    fn test_calculate_content_width() {
        let lines = vec!["hello".to_string(), "world!".to_string()];
        let padding = Spacing {
            top: 0,
            right: 2,
            bottom: 0,
            left: 3,
        };

        assert_eq!(calculate_content_width(&lines, &padding), 11); // 6 + 3 + 2
    }

    #[test]
    fn test_calculate_content_height() {
        let lines = vec!["hello".to_string(), "world".to_string()];
        let padding = Spacing {
            top: 1,
            right: 0,
            bottom: 2,
            left: 0,
        };

        assert_eq!(calculate_content_height(&lines, &padding), 5); // 2 + 1 + 2
    }

    #[test]
    fn test_process_text_alignment_basic() {
        let text = "hello\nworld";
        let padding = Spacing {
            top: 1,
            right: 1,
            bottom: 1,
            left: 1,
        };
        let result = process_text_alignment(text, TextAlignment::Center, &padding, Some(10));

        assert_eq!(result.len(), 4); // 1 top + 2 content + 1 bottom
        // Content width should be 10 - 1 - 1 = 8
        // "hello" centered in 8 chars: " hello  "
        // With left/right padding: " " + " hello  " + " " = "  hello   "
        assert_eq!(result[1], "  hello   ");
        assert_eq!(result[2], "  world   ");
    }

    #[test]
    fn test_process_text_alignment_no_target_width() {
        let text = "hello\nworld!";
        let padding = Spacing {
            top: 0,
            right: 1,
            bottom: 0,
            left: 1,
        };
        let result = process_text_alignment(text, TextAlignment::Left, &padding, None);

        // Natural width is 6 ("world!"), with padding becomes 8
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], " hello  ");
        assert_eq!(result[1], " world! ");
    }

    #[test]
    fn test_process_text_alignment_with_unicode() {
        let text = "你好\nworld";
        let padding = Spacing {
            top: 0,
            right: 1,
            bottom: 0,
            left: 1,
        };
        let result = process_text_alignment(text, TextAlignment::Right, &padding, Some(10));

        // Content width: 10 - 1 - 1 = 8
        // "你好" (width 4) right-aligned in 8: "    你好"
        // "world" (width 5) right-aligned in 8: "   world"
        assert_eq!(result[0], "     你好 ");
        assert_eq!(result[1], "    world ");
    }

    #[test]
    fn test_process_text_alignment_minimum_width() {
        let text = "hello";
        let padding = Spacing {
            top: 0,
            right: 5,
            bottom: 0,
            left: 5,
        };
        let result = process_text_alignment(text, TextAlignment::Left, &padding, Some(5));

        // Target width 5 is less than padding (5+5=10), so use natural content width (5)
        // Total width becomes 5 (content) + 5 (left) + 5 (right) = 15
        assert_eq!(result[0], "     hello     ");
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let padding = Spacing {
            top: 1,
            right: 1,
            bottom: 1,
            left: 1,
        };
        let result = process_text_alignment(text, TextAlignment::Center, &padding, Some(6));

        assert_eq!(result.len(), 3); // 1 top + 1 content + 1 bottom
        assert_eq!(result[0], "      "); // Top padding
        assert_eq!(result[1], "      "); // Empty content with padding  
        assert_eq!(result[2], "      "); // Bottom padding
    }

    #[test]
    fn test_apply_height_constraints_no_constraint() {
        let lines = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];
        let result = apply_height_constraints(&lines, 5);

        // Should pad to reach target height
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], "line1");
        assert_eq!(result[1], "line2");
        assert_eq!(result[2], "line3");
        assert_eq!(result[3], ""); // Padding
        assert_eq!(result[4], ""); // Padding
    }

    #[test]
    fn test_apply_height_constraints_truncation() {
        let lines = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
            "line4".to_string(),
            "line5".to_string(),
        ];
        let result = apply_height_constraints(&lines, 3);

        // Should truncate to max height
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "line1");
        assert_eq!(result[1], "line2");
        assert_eq!(result[2], "line3");
    }

    #[test]
    fn test_apply_height_constraints_exact_fit() {
        let lines = vec!["line1".to_string(), "line2".to_string()];
        let result = apply_height_constraints(&lines, 2);

        // Should return as-is when exact fit
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "line1");
        assert_eq!(result[1], "line2");
    }

    #[test]
    fn test_apply_height_constraints_zero_height() {
        let lines = vec!["line1".to_string(), "line2".to_string()];
        let result = apply_height_constraints(&lines, 0);

        // Should return empty when max height is 0
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_apply_height_constraints_empty_input() {
        let lines: Vec<String> = vec![];
        let result = apply_height_constraints(&lines, 3);

        // Should pad empty input to target height
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "");
        assert_eq!(result[1], "");
        assert_eq!(result[2], "");
    }

    #[test]
    fn test_process_text_with_height_constraints_padding() {
        let text = "hello\nworld";
        let padding = Spacing {
            top: 1,
            right: 1,
            bottom: 1,
            left: 1,
        };
        let result = process_text_with_height_constraints(
            text,
            TextAlignment::Left,
            &padding,
            Some(10),
            Some(4), // Max content height of 4, but we only have 2 lines
        );

        // Should have: 1 top padding + 4 content lines (2 actual + 2 empty) + 1 bottom padding = 6 total
        assert_eq!(result.len(), 6);

        // Check content lines (accounting for left/right padding)
        assert_eq!(result[1], " hello    "); // Content with padding
        assert_eq!(result[2], " world    "); // Content with padding
        assert_eq!(result[3], "          "); // Empty content line with padding
        assert_eq!(result[4], "          "); // Empty content line with padding
    }

    #[test]
    fn test_process_text_with_height_constraints_truncation() {
        let text = "line1\nline2\nline3\nline4\nline5";
        let padding = Spacing {
            top: 0,
            right: 1,
            bottom: 0,
            left: 1,
        };
        let result = process_text_with_height_constraints(
            text,
            TextAlignment::Left,
            &padding,
            Some(10),
            Some(3), // Max content height of 3, but we have 5 lines
        );

        // Should have exactly 3 content lines (truncated)
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], " line1    ");
        assert_eq!(result[1], " line2    ");
        assert_eq!(result[2], " line3    ");
    }

    #[test]
    fn test_process_text_with_height_constraints_no_constraint() {
        let text = "hello\nworld";
        let padding = Spacing {
            top: 0,
            right: 1,
            bottom: 0,
            left: 1,
        };
        let result = process_text_with_height_constraints(
            text,
            TextAlignment::Center,
            &padding,
            Some(10),
            None, // No height constraint
        );

        // Should process normally without height constraints
        assert_eq!(result.len(), 2);
        assert!(result[0].contains("hello"));
        assert!(result[1].contains("world"));
    }

    #[test]
    fn test_process_text_with_height_constraints_empty_text() {
        let text = "";
        let padding = Spacing {
            top: 0,
            right: 1,
            bottom: 0,
            left: 1,
        };
        let result = process_text_with_height_constraints(
            text,
            TextAlignment::Center,
            &padding,
            Some(8),
            Some(3), // Height constraint of 3
        );

        // Should create 3 empty lines
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "        "); // Empty line with padding
        assert_eq!(result[1], "        "); // Empty line with padding
        assert_eq!(result[2], "        "); // Empty line with padding
    }

    #[test]
    fn test_height_constraints_with_unicode() {
        let text = "你好\n世界\n测试\n内容";
        let padding = Spacing::default();
        let result = process_text_with_height_constraints(
            text,
            TextAlignment::Left,
            &padding,
            None,
            Some(2), // Truncate to 2 lines
        );

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "你好");
        assert_eq!(result[1], "世界");
    }
}
