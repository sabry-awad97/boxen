use crate::error::BoxenError;
use unicode_width::UnicodeWidthStr;

/// Calculate the display width of text, handling Unicode and ANSI escape sequences
/// Optimized version that avoids allocation when no ANSI codes are present
pub fn text_width(text: &str) -> usize {
    // Fast path: if no ANSI codes, measure directly
    if !text.contains('\x1b') {
        return UnicodeWidthStr::width(text);
    }

    // Slow path: strip ANSI codes first
    let clean_text = strip_ansi_codes(text);
    UnicodeWidthStr::width(clean_text.as_str())
}

/// Strip ANSI escape sequences from text
/// Optimized version that pre-allocates capacity and uses efficient iteration
pub fn strip_ansi_codes(text: &str) -> String {
    // Quick check: if no escape sequences, return clone to avoid allocation
    if !text.contains('\x1b') {
        return text.to_string();
    }

    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Found escape sequence start
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['

                // Skip until we find the end of the escape sequence
                for escape_char in chars.by_ref() {
                    // ANSI escape sequences end with a letter (A-Z, a-z)
                    if escape_char.is_ascii_alphabetic() {
                        break;
                    }
                }
            } else {
                // Not a CSI sequence, keep the escape character
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Calculate the maximum width of multiple lines of text
pub fn max_line_width(lines: &[&str]) -> usize {
    lines.iter().map(|line| text_width(line)).max().unwrap_or(0)
}

/// Calculate the display width of each line in a multi-line string
pub fn line_widths(text: &str) -> Vec<usize> {
    text.lines().map(text_width).collect()
}

/// Validate that text width calculation is working correctly
pub fn validate_text_measurement(text: &str) -> Result<usize, BoxenError> {
    let width = text_width(text);

    // Basic validation - width should never be negative (impossible with usize)
    // and should be reasonable for terminal display
    if width > 10000 {
        return Err(BoxenError::text_processing_error(
            format!("Text width {} seems unreasonably large", width),
            vec![
                crate::error::ErrorRecommendation::suggestion_only(
                    "Excessive text width".to_string(),
                    "This may indicate an issue with text measurement or very wide content"
                        .to_string(),
                ),
                crate::error::ErrorRecommendation::with_auto_fix(
                    "Use width constraint".to_string(),
                    "Limit the box width to prevent issues".to_string(),
                    ".width(80)".to_string(),
                ),
            ],
        ));
    }

    Ok(width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ascii_width() {
        assert_eq!(text_width("hello"), 5);
        assert_eq!(text_width(""), 0);
        assert_eq!(text_width("a"), 1);
    }

    #[test]
    fn test_unicode_width() {
        // Wide characters (CJK)
        assert_eq!(text_width("你好"), 4); // 2 characters, 2 width each
        assert_eq!(text_width("こんにちは"), 10); // 5 characters, 2 width each

        // Combining characters
        assert_eq!(text_width("é"), 1); // e + combining acute accent

        // Zero-width characters
        assert_eq!(text_width("a\u{200B}b"), 2); // a + zero-width space + b
    }

    #[test]
    fn test_ansi_escape_sequences() {
        // Basic color codes
        assert_eq!(text_width("\x1b[31mred\x1b[0m"), 3);
        assert_eq!(text_width("\x1b[1;32mbold green\x1b[0m"), 10);

        // Complex escape sequences
        assert_eq!(text_width("\x1b[38;5;196mhello\x1b[0m"), 5);
        assert_eq!(text_width("\x1b[48;2;255;0;0mworld\x1b[0m"), 5);
    }

    #[test]
    fn test_mixed_content() {
        // Unicode + ANSI
        assert_eq!(text_width("\x1b[31m你好\x1b[0m"), 4);

        // Multiple lines with different content types
        let lines = vec!["hello", "\x1b[31mred\x1b[0m", "你好"];
        assert_eq!(max_line_width(&lines), 5);
    }

    #[test]
    fn test_strip_ansi_codes() {
        assert_eq!(strip_ansi_codes("hello"), "hello");
        assert_eq!(strip_ansi_codes("\x1b[31mred\x1b[0m"), "red");
        assert_eq!(
            strip_ansi_codes("\x1b[1;32mbold green\x1b[0m"),
            "bold green"
        );
        assert_eq!(
            strip_ansi_codes("no\x1b[31mcolor\x1b[0mhere"),
            "nocolorhere"
        );
    }

    #[test]
    fn test_line_widths() {
        let text = "hello\nworld\n你好";
        let widths = line_widths(text);
        assert_eq!(widths, vec![5, 5, 4]);
    }

    #[test]
    fn test_validate_text_measurement() {
        assert!(validate_text_measurement("hello").is_ok());
        assert!(validate_text_measurement("你好").is_ok());
        assert!(validate_text_measurement("\x1b[31mred\x1b[0m").is_ok());

        // Test with very long string (should still be ok)
        let long_text = "a".repeat(1000);
        assert!(validate_text_measurement(&long_text).is_ok());
    }

    #[test]
    fn test_empty_and_whitespace() {
        assert_eq!(text_width(""), 0);
        assert_eq!(text_width(" "), 1);
        assert_eq!(text_width("   "), 3);
        assert_eq!(text_width("\t"), 1); // Tab is treated as single character
        assert_eq!(text_width("\n"), 1); // Newline has width 1 in unicode-width
    }
}
