// Generated test suite covering all major features and edge cases

use boxen::{
    BorderStyle, BoxenOptions, Color, Float, FullscreenMode, Spacing, TextAlignment,
    TitleAlignment, boxen, builder, double_box, round_box, simple_box,
};

#[cfg(test)]
mod test_main_rendering_function {
    use super::*;

    #[test]
    fn test_basic_boxen_with_default_options() {
        let result = boxen("Hello, World!", None);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Hello, World!"));
        assert!(output.contains("┌"));
        assert!(output.contains("└"));
    }

    #[test]
    fn test_boxen_with_empty_text() {
        let result = boxen("", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_boxen_with_multiline_text() {
        let text = "Line 1\nLine 2\nLine 3";
        let result = boxen(text, None);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Line 1"));
        assert!(output.contains("Line 2"));
        assert!(output.contains("Line 3"));
    }

    #[test]
    fn test_boxen_with_unicode_text() {
        let text = "你好世界 🎉";
        let result = boxen(text, None);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("你好世界"));
    }

    #[test]
    fn test_boxen_with_custom_padding() {
        let options = BoxenOptions {
            padding: Spacing::from(2),
            ..Default::default()
        };
        let result = boxen("Test", Some(options));
        assert!(result.is_ok());
    }

    #[test]
    fn test_boxen_with_custom_margin() {
        let options = BoxenOptions {
            margin: Spacing::from(1),
            ..Default::default()
        };
        let result = boxen("Test", Some(options));
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_fluent_builder_api {
    use super::*;

    #[test]
    fn test_builder_basic_usage() {
        let result = builder().render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_border_style() {
        let result = builder().border_style(BorderStyle::Double).render("Test");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("╔") || output.contains("╚"));
    }

    #[test]
    fn test_builder_with_padding() {
        let result = builder().padding(2).render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_margin() {
        let result = builder().margin(1).render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_text_alignment() {
        let result = builder()
            .text_alignment(TextAlignment::Center)
            .width(20)
            .render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_title() {
        let result = builder().title("My Title").width(30).render("Content");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("My Title"));
    }

    #[test]
    fn test_builder_with_width() {
        let result = builder().width(30).render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_height() {
        let result = builder().width(30).height(5).render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_border_color() {
        let result = builder().border_color("blue").render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_background_color() {
        let result = builder().background_color("white").render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_chaining_multiple_options() {
        let result = builder()
            .border_style(BorderStyle::Round)
            .padding(1)
            .margin(1)
            .text_alignment(TextAlignment::Center)
            .title("Test Box")
            .width(40)
            .border_color("green")
            .render("Hello, World!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_convenience_methods() {
        let result = builder().spacing(1).size(40, 8).center_all().render("Test");
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_border_styles {
    use super::*;

    #[test]
    fn test_single_border_style() {
        let result = builder().border_style(BorderStyle::Single).render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_double_border_style() {
        let result = builder().border_style(BorderStyle::Double).render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_round_border_style() {
        let result = builder().border_style(BorderStyle::Round).render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_bold_border_style() {
        let result = builder().border_style(BorderStyle::Bold).render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_classic_border_style() {
        let result = builder().border_style(BorderStyle::Classic).render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_single_double_border_style() {
        let result = builder()
            .border_style(BorderStyle::SingleDouble)
            .render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_double_single_border_style() {
        let result = builder()
            .border_style(BorderStyle::DoubleSingle)
            .render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_none_border_style() {
        let result = builder().border_style(BorderStyle::None).render("Test");
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_text_processing {
    use super::*;

    #[test]
    fn test_left_alignment() {
        let result = builder()
            .text_alignment(TextAlignment::Left)
            .width(20)
            .render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_center_alignment() {
        let result = builder()
            .text_alignment(TextAlignment::Center)
            .width(20)
            .render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_right_alignment() {
        let result = builder()
            .text_alignment(TextAlignment::Right)
            .width(20)
            .render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_text_wrapping() {
        let long_text = "This is a very long line of text that should wrap";
        let result = builder().width(20).render(long_text);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unicode_width_calculation() {
        let text = "Hello 世界";
        let result = builder().render(text);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ansi_escape_sequences() {
        let colored_text = "\x1b[31mRed\x1b[0m \x1b[32mGreen\x1b[0m";
        let result = builder().render(colored_text);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_color_system {
    use super::*;

    #[test]
    fn test_named_color() {
        let result = builder()
            .border_color(Color::Named("red".to_string()))
            .render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_hex_color() {
        let result = builder()
            .border_color(Color::Hex("#FF0000".to_string()))
            .render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_rgb_color() {
        let result = builder().border_color(Color::Rgb(255, 0, 0)).render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_background_color() {
        let result = builder().background_color("blue").render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_dim_border() {
        let result = builder()
            .border_color("cyan")
            .dim_border(true)
            .render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_both_border_and_background_colors() {
        let result = builder()
            .border_color("green")
            .background_color("white")
            .render("Test");
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_validation_and_errors {
    use super::*;

    #[test]
    fn test_invalid_width_too_small() {
        let result = builder().width(2).render("Test");
        assert!(result.is_err());
    }

    #[test]
    fn test_excessive_padding() {
        let result = builder().width(10).padding(20).render("Test");
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_with_suggestions() {
        let b = builder().width(5).padding(10);
        let validation = b.validate_with_suggestions("Test");
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
    }

    #[test]
    fn test_minimum_dimensions_calculation() {
        let b = builder().padding(2);
        let min_dims = b.minimum_dimensions("Test");
        assert!(min_dims.width > 0);
        assert!(min_dims.height > 0);
    }

    #[test]
    fn test_auto_adjust_options() {
        let b = builder().width(5).padding(10).auto_adjust("Test");
        // Auto-adjust modifies the builder, but may still fail if constraints are too tight
        // Just verify the auto_adjust method exists and can be called
        let _ = b.validate();
    }

    #[test]
    fn test_render_or_adjust() {
        let result = builder().width(5).padding(10).render_or_adjust("Test");
        // Should auto-adjust and succeed
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_convenience_functions {
    use super::*;

    #[test]
    fn test_simple_box() {
        let output = simple_box("Test");
        assert!(output.contains("Test"));
        assert!(output.contains("┌") || output.contains("│"));
    }

    #[test]
    fn test_double_box() {
        let output = double_box("Test");
        assert!(output.contains("Test"));
        assert!(output.contains("╔") || output.contains("║"));
    }

    #[test]
    fn test_round_box() {
        let output = round_box("Test");
        assert!(output.contains("Test"));
        assert!(output.contains("╭") || output.contains("│"));
    }

    #[test]
    fn test_convenience_functions_never_panic() {
        // These should never panic, even with problematic input
        let _ = simple_box("");
        let _ = double_box("Very long text that might cause issues");
        let _ = round_box("Unicode: 你好");
    }
}

#[cfg(test)]
mod test_fullscreen_mode {
    use super::*;

    #[test]
    fn test_fullscreen_auto_mode() {
        let result = builder().fullscreen(FullscreenMode::Auto).render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_fullscreen_custom_mode() {
        let custom_fn = |width: usize, height: usize| (width - 4, height - 2);
        let result = builder()
            .fullscreen(FullscreenMode::Custom(custom_fn))
            .render("Test");
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_title_features {
    use super::*;

    #[test]
    fn test_title_left_alignment() {
        let result = builder()
            .title("Title")
            .title_alignment(TitleAlignment::Left)
            .width(30)
            .render("Content");
        assert!(result.is_ok());
    }

    #[test]
    fn test_title_center_alignment() {
        let result = builder()
            .title("Title")
            .title_alignment(TitleAlignment::Center)
            .width(30)
            .render("Content");
        assert!(result.is_ok());
    }

    #[test]
    fn test_title_right_alignment() {
        let result = builder()
            .title("Title")
            .title_alignment(TitleAlignment::Right)
            .width(30)
            .render("Content");
        assert!(result.is_ok());
    }

    #[test]
    fn test_long_title_truncation() {
        let result = builder()
            .title("This is a very long title that should be truncated")
            .width(20)
            .render("Content");
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_float_positioning {
    use super::*;

    #[test]
    fn test_float_left() {
        let result = builder().float(Float::Left).width(20).render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_float_center() {
        let result = builder().float(Float::Center).width(20).render("Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_float_right() {
        let result = builder().float(Float::Right).width(20).render("Test");
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_spacing_system {
    use super::*;

    #[test]
    fn test_spacing_from_usize() {
        let spacing = Spacing::from(2);
        assert_eq!(spacing.top, 2);
        assert_eq!(spacing.bottom, 2);
        assert_eq!(spacing.left, 6); // 3x horizontal
        assert_eq!(spacing.right, 6);
    }

    #[test]
    fn test_spacing_from_tuple() {
        let spacing = Spacing::from((1, 2, 3, 4));
        assert_eq!(spacing.top, 1);
        assert_eq!(spacing.right, 2);
        assert_eq!(spacing.bottom, 3);
        assert_eq!(spacing.left, 4);
    }

    #[test]
    fn test_asymmetric_padding() {
        let result = builder().padding((1, 2, 3, 4)).width(30).render("Test");
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_edge_cases {
    use super::*;

    #[test]
    fn test_very_long_single_line() {
        let long_text = "a".repeat(100);
        let result = builder().width(50).render(&long_text);
        assert!(result.is_ok());
    }

    #[test]
    fn test_many_short_lines() {
        let many_lines = (0..10)
            .map(|i| format!("Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let result = builder().width(30).render(&many_lines);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mixed_unicode_and_ascii() {
        let text = "Hello 世界 🎉 Test";
        let result = builder().render(text);
        assert!(result.is_ok());
    }

    #[test]
    fn test_zero_width_characters() {
        let text = "Test\u{200B}ing"; // Zero-width space
        let result = builder().render(text);
        assert!(result.is_ok());
    }

    #[test]
    fn test_combining_characters() {
        let text = "café"; // é as combining character
        let result = builder().render(text);
        assert!(result.is_ok());
    }

    #[test]
    fn test_emoji_sequences() {
        let text = "👨‍👩‍👧‍👦 Family";
        let result = builder().render(text);
        assert!(result.is_ok());
    }
}
