use crate::error::BoxenError;
use crate::text::measurement::{strip_ansi_codes, text_width};
use textwrap::{Options, WordSeparator, WordSplitter, wrap};

/// Wrap text to fit within a specified width, preserving ANSI escape sequences
pub fn wrap_text(text: &str, width: usize) -> Result<Vec<String>, BoxenError> {
    if width == 0 {
        return Err(BoxenError::TextProcessingError(
            "Cannot wrap text with zero width".to_string(),
        ));
    }

    let lines: Vec<String> = text
        .lines()
        .flat_map(|line| wrap_line(line, width))
        .collect();

    Ok(lines)
}

/// Wrap a single line of text, handling ANSI escape sequences properly
pub fn wrap_line(line: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![line.to_string()];
    }

    // If the line fits within the width, return it as-is
    if text_width(line) <= width {
        return vec![line.to_string()];
    }

    // For lines with ANSI codes, we need custom wrapping logic
    if line.contains('\x1b') {
        wrap_line_with_ansi(line, width)
    } else {
        // Use textwrap for plain text
        let options = Options::new(width)
            .word_separator(WordSeparator::AsciiSpace)
            .word_splitter(WordSplitter::HyphenSplitter);

        wrap(line, &options)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }
}

/// Wrap a line containing ANSI escape sequences
fn wrap_line_with_ansi(line: &str, width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;
    let mut chars = line.chars().peekable();
    let mut active_styles = String::new(); // Track active ANSI styles

    while let Some(ch) = chars.next() {
        if ch == '\x1b' && chars.peek() == Some(&'[') {
            // Handle ANSI escape sequence
            let mut escape_seq = String::from("\x1b");
            escape_seq.push(chars.next().unwrap()); // consume '['

            // Collect the full escape sequence
            for escape_char in chars.by_ref() {
                escape_seq.push(escape_char);
                if escape_char.is_ascii_alphabetic() {
                    break;
                }
            }

            // Add to current line without affecting width
            current_line.push_str(&escape_seq);

            // Track styles for line continuation
            if escape_seq.ends_with('m') {
                if escape_seq == "\x1b[0m" {
                    active_styles.clear(); // Reset
                } else {
                    active_styles.push_str(&escape_seq);
                }
            }
        } else {
            // Regular character
            let char_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);

            if current_width + char_width > width && !current_line.is_empty() {
                // Need to wrap - finish current line and start new one
                result.push(current_line);
                current_line = active_styles.clone(); // Start new line with active styles
                current_width = 0;
            }

            current_line.push(ch);
            current_width += char_width;
        }
    }

    // Add the last line if it has content
    if !current_line.is_empty() || result.is_empty() {
        result.push(current_line);
    }

    result
}

/// Wrap text with word boundary preservation when possible
pub fn wrap_text_preserve_words(text: &str, width: usize) -> Result<Vec<String>, BoxenError> {
    if width == 0 {
        return Err(BoxenError::TextProcessingError(
            "Cannot wrap text with zero width".to_string(),
        ));
    }

    let lines: Vec<String> = text
        .lines()
        .flat_map(|line| {
            if text_width(line) <= width {
                vec![line.to_string()]
            } else if line.contains('\x1b') {
                // For ANSI text, use our custom wrapper
                wrap_line_with_ansi(line, width)
            } else {
                // Use textwrap with word boundary preservation
                let options = Options::new(width)
                    .word_separator(WordSeparator::AsciiSpace)
                    .word_splitter(WordSplitter::NoHyphenation);

                let wrapped = wrap(line, &options);
                if wrapped.is_empty() {
                    // If textwrap couldn't wrap (e.g., single word too long), force wrap
                    wrap_line(line, width)
                } else {
                    wrapped.into_iter().map(|s| s.to_string()).collect()
                }
            }
        })
        .collect();

    Ok(lines)
}

/// Calculate the minimum width needed to wrap text without breaking words
pub fn minimum_wrap_width(text: &str) -> usize {
    text.lines()
        .flat_map(|line| {
            let clean_line = strip_ansi_codes(line);
            clean_line
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .map(|word| text_width(&word))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_wrapping() {
        let result = wrap_text("hello world", 5).unwrap();
        assert_eq!(result, vec!["hello", "world"]);

        let result = wrap_text("hello world", 11).unwrap();
        assert_eq!(result, vec!["hello world"]);
    }

    #[test]
    fn test_multiline_wrapping() {
        let text = "hello world\nfoo bar baz";
        let result = wrap_text(text, 5).unwrap();
        assert_eq!(result, vec!["hello", "world", "foo", "bar", "baz"]);
    }

    #[test]
    fn test_ansi_wrapping() {
        let text = "\x1b[31mhello world\x1b[0m";
        let result = wrap_text(text, 5).unwrap();
        // Should preserve ANSI codes and wrap properly
        assert!(result.len() >= 2);
        assert!(result[0].contains("\x1b[31m"));
        // Check that total content is preserved (ignoring ANSI codes)
        let combined = result.join("");
        let clean_original = strip_ansi_codes(text);
        let clean_combined = strip_ansi_codes(&combined);
        assert_eq!(
            clean_original.replace(" ", ""),
            clean_combined.replace(" ", "")
        );
    }

    #[test]
    fn test_unicode_wrapping() {
        let text = "你好 世界";
        let result = wrap_text(text, 4).unwrap();
        assert_eq!(result, vec!["你好", "世界"]);
    }

    #[test]
    fn test_zero_width_error() {
        let result = wrap_text("hello", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_preserve_words() {
        let text = "hello world foo";
        let result = wrap_text_preserve_words(text, 10).unwrap();
        assert_eq!(result, vec!["hello", "world foo"]);
    }

    #[test]
    fn test_long_word_force_wrap() {
        let text = "supercalifragilisticexpialidocious";
        let result = wrap_text(text, 10).unwrap();
        assert!(result.len() > 1);
        assert!(result.iter().all(|line| text_width(line) <= 10));
    }

    #[test]
    fn test_minimum_wrap_width() {
        assert_eq!(minimum_wrap_width("hello world"), 5);
        assert_eq!(minimum_wrap_width("a bb ccc"), 3);
        assert_eq!(minimum_wrap_width("你好 世界"), 4); // 2 chars * 2 width each
        assert_eq!(minimum_wrap_width(""), 0);
    }

    #[test]
    fn test_empty_lines() {
        let result = wrap_text("hello\n\nworld", 10).unwrap();
        assert_eq!(result, vec!["hello", "", "world"]);
    }

    #[test]
    fn test_whitespace_only() {
        let result = wrap_text("   ", 5).unwrap();
        assert_eq!(result, vec!["   "]);

        // textwrap trims trailing whitespace, so test with mixed content
        let result = wrap_text("hello     world", 5).unwrap();
        assert!(result.len() >= 2);
        assert!(result.iter().any(|line| line.contains("hello")));
        assert!(result.iter().any(|line| line.contains("world")));
    }

    #[test]
    fn test_complex_ansi_sequences() {
        let text = "\x1b[1;32mbold green\x1b[0m normal \x1b[31mred\x1b[0m";
        let result = wrap_text(text, 8).unwrap();
        assert!(result.len() >= 2);
        // Verify ANSI codes are preserved
        assert!(result.iter().any(|line| line.contains("\x1b[")));
    }
}
