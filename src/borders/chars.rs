/// Border character definitions and utilities
use crate::options::BorderChars;

impl BorderChars {
    /// Create a new BorderChars with all characters set to the same value
    pub fn uniform(ch: char) -> Self {
        Self {
            top_left: ch,
            top_right: ch,
            bottom_left: ch,
            bottom_right: ch,
            left: ch,
            right: ch,
            top: ch,
            bottom: ch,
        }
    }

    /// Create BorderChars for single-line box drawing
    pub fn single() -> Self {
        Self {
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            left: '│',
            right: '│',
            top: '─',
            bottom: '─',
        }
    }

    /// Create BorderChars for double-line box drawing
    pub fn double() -> Self {
        Self {
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
            left: '║',
            right: '║',
            top: '═',
            bottom: '═',
        }
    }

    /// Create BorderChars for rounded corners
    pub fn round() -> Self {
        Self {
            top_left: '╭',
            top_right: '╮',
            bottom_left: '╰',
            bottom_right: '╯',
            left: '│',
            right: '│',
            top: '─',
            bottom: '─',
        }
    }

    /// Create BorderChars for bold/thick lines
    pub fn bold() -> Self {
        Self {
            top_left: '┏',
            top_right: '┓',
            bottom_left: '┗',
            bottom_right: '┛',
            left: '┃',
            right: '┃',
            top: '━',
            bottom: '━',
        }
    }

    /// Create BorderChars for single horizontal, double vertical
    pub fn single_double() -> Self {
        Self {
            top_left: '╓',
            top_right: '╖',
            bottom_left: '╙',
            bottom_right: '╜',
            left: '║',
            right: '║',
            top: '─',
            bottom: '─',
        }
    }

    /// Create BorderChars for double horizontal, single vertical
    pub fn double_single() -> Self {
        Self {
            top_left: '╒',
            top_right: '╕',
            bottom_left: '╘',
            bottom_right: '╛',
            left: '│',
            right: '│',
            top: '═',
            bottom: '═',
        }
    }

    /// Create BorderChars for classic ASCII-style borders
    pub fn classic() -> Self {
        Self {
            top_left: '+',
            top_right: '+',
            bottom_left: '+',
            bottom_right: '+',
            left: '|',
            right: '|',
            top: '-',
            bottom: '-',
        }
    }

    /// Validate that all border characters are printable and not whitespace
    pub fn validate(&self) -> Result<(), String> {
        let chars = [
            ("top_left", self.top_left),
            ("top_right", self.top_right),
            ("bottom_left", self.bottom_left),
            ("bottom_right", self.bottom_right),
            ("left", self.left),
            ("right", self.right),
            ("top", self.top),
            ("bottom", self.bottom),
        ];

        for (name, ch) in chars.iter() {
            if ch.is_whitespace() {
                return Err(format!("Border character '{}' cannot be whitespace", name));
            }
            if !ch.is_ascii_graphic() && !is_box_drawing_char(*ch) {
                return Err(format!(
                    "Border character '{}' must be printable (got '{}')",
                    name, ch
                ));
            }
        }

        Ok(())
    }

    /// Get the width of the border (always 1 for single characters)
    pub fn border_width(&self) -> usize {
        1
    }
}

/// Check if a character is a Unicode box drawing character
fn is_box_drawing_char(ch: char) -> bool {
    matches!(ch as u32, 0x2500..=0x257F)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_border_chars() {
        let chars = BorderChars::single();
        assert_eq!(chars.top_left, '┌');
        assert_eq!(chars.top_right, '┐');
        assert_eq!(chars.bottom_left, '└');
        assert_eq!(chars.bottom_right, '┘');
        assert_eq!(chars.left, '│');
        assert_eq!(chars.right, '│');
        assert_eq!(chars.top, '─');
        assert_eq!(chars.bottom, '─');
    }

    #[test]
    fn test_double_border_chars() {
        let chars = BorderChars::double();
        assert_eq!(chars.top_left, '╔');
        assert_eq!(chars.top_right, '╗');
        assert_eq!(chars.bottom_left, '╚');
        assert_eq!(chars.bottom_right, '╝');
        assert_eq!(chars.left, '║');
        assert_eq!(chars.right, '║');
        assert_eq!(chars.top, '═');
        assert_eq!(chars.bottom, '═');
    }

    #[test]
    fn test_round_border_chars() {
        let chars = BorderChars::round();
        assert_eq!(chars.top_left, '╭');
        assert_eq!(chars.top_right, '╮');
        assert_eq!(chars.bottom_left, '╰');
        assert_eq!(chars.bottom_right, '╯');
    }

    #[test]
    fn test_bold_border_chars() {
        let chars = BorderChars::bold();
        assert_eq!(chars.top_left, '┏');
        assert_eq!(chars.top_right, '┓');
        assert_eq!(chars.bottom_left, '┗');
        assert_eq!(chars.bottom_right, '┛');
        assert_eq!(chars.left, '┃');
        assert_eq!(chars.right, '┃');
        assert_eq!(chars.top, '━');
        assert_eq!(chars.bottom, '━');
    }

    #[test]
    fn test_single_double_border_chars() {
        let chars = BorderChars::single_double();
        assert_eq!(chars.top_left, '╓');
        assert_eq!(chars.top_right, '╖');
        assert_eq!(chars.bottom_left, '╙');
        assert_eq!(chars.bottom_right, '╜');
        assert_eq!(chars.left, '║');
        assert_eq!(chars.right, '║');
        assert_eq!(chars.top, '─');
        assert_eq!(chars.bottom, '─');
    }

    #[test]
    fn test_double_single_border_chars() {
        let chars = BorderChars::double_single();
        assert_eq!(chars.top_left, '╒');
        assert_eq!(chars.top_right, '╕');
        assert_eq!(chars.bottom_left, '╘');
        assert_eq!(chars.bottom_right, '╛');
        assert_eq!(chars.left, '│');
        assert_eq!(chars.right, '│');
        assert_eq!(chars.top, '═');
        assert_eq!(chars.bottom, '═');
    }

    #[test]
    fn test_classic_border_chars() {
        let chars = BorderChars::classic();
        assert_eq!(chars.top_left, '+');
        assert_eq!(chars.top_right, '+');
        assert_eq!(chars.bottom_left, '+');
        assert_eq!(chars.bottom_right, '+');
        assert_eq!(chars.left, '|');
        assert_eq!(chars.right, '|');
        assert_eq!(chars.top, '-');
        assert_eq!(chars.bottom, '-');
    }

    #[test]
    fn test_uniform_border_chars() {
        let chars = BorderChars::uniform('*');
        assert_eq!(chars.top_left, '*');
        assert_eq!(chars.top_right, '*');
        assert_eq!(chars.bottom_left, '*');
        assert_eq!(chars.bottom_right, '*');
        assert_eq!(chars.left, '*');
        assert_eq!(chars.right, '*');
        assert_eq!(chars.top, '*');
        assert_eq!(chars.bottom, '*');
    }

    #[test]
    fn test_border_validation_success() {
        let chars = BorderChars::single();
        assert!(chars.validate().is_ok());

        let chars = BorderChars::classic();
        assert!(chars.validate().is_ok());

        let chars = BorderChars::uniform('*');
        assert!(chars.validate().is_ok());
    }

    #[test]
    fn test_border_validation_whitespace_error() {
        let chars = BorderChars {
            top_left: ' ',
            ..BorderChars::single()
        };
        let result = chars.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("whitespace"));
    }

    #[test]
    fn test_border_width() {
        let chars = BorderChars::single();
        assert_eq!(chars.border_width(), 1);
    }

    #[test]
    fn test_is_box_drawing_char() {
        assert!(is_box_drawing_char('┌'));
        assert!(is_box_drawing_char('═'));
        assert!(is_box_drawing_char('╭'));
        assert!(!is_box_drawing_char('a'));
        assert!(!is_box_drawing_char(' '));
        assert!(!is_box_drawing_char('*'));
    }
}
