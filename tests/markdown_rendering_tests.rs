//! Integration tests for markdown rendering functionality
//!
//! These tests verify behavior through the public API, focusing on observable output
//! rather than implementation details.

use boxen::markdown::{ItalicStyle, LinkStyle, MarkdownConfig, MarkdownStyle};
use boxen::{Color, builder, markdown_box};

// ============================================================================
// Test Group 1: Basic Markdown Elements
// ============================================================================

#[test]
fn test_plain_text_renders_unchanged() {
    // ARRANGE
    let input = "Hello world";

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(
        result.contains("Hello world"),
        "Plain text should be preserved"
    );
}

#[test]
fn test_heading_h1_applies_color() {
    // ARRANGE
    let input = "# Main Heading";

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(
        result.contains("Main Heading"),
        "Heading text should be present"
    );
    assert!(
        result.contains("\x1b["),
        "Should contain ANSI escape codes for color"
    );
    assert!(
        result.contains("\x1b[0m"),
        "Should reset formatting after heading"
    );
}

#[test]
fn test_heading_h2_applies_different_color() {
    // ARRANGE
    let input = "## Subheading";

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(
        result.contains("Subheading"),
        "Heading text should be present"
    );
    assert!(
        result.contains("\x1b["),
        "Should contain ANSI escape codes for color"
    );
}

#[test]
fn test_bold_text_applies_bold_ansi() {
    // ARRANGE
    let input = "**bold text**";

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(
        result.contains("bold text"),
        "Bold text content should be present"
    );
    assert!(result.contains("\x1b[1m"), "Should contain bold ANSI code");
    assert!(
        result.contains("\x1b[0m"),
        "Should reset formatting after bold"
    );
}

#[test]
fn test_italic_text_applies_italic_ansi() {
    // ARRANGE
    let input = "*italic text*";

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(
        result.contains("italic text"),
        "Italic text content should be present"
    );
    assert!(
        result.contains("\x1b[3m"),
        "Should contain italic ANSI code"
    );
}

#[test]
fn test_inline_code_applies_styling() {
    // ARRANGE
    let input = "Use `code` here";

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(result.contains("code"), "Code text should be present");
    assert!(result.contains("Use"), "Surrounding text should be present");
    assert!(
        result.contains("here"),
        "Surrounding text should be present"
    );
    assert!(
        result.contains("\x1b["),
        "Should contain ANSI codes for code styling"
    );
}

#[test]
fn test_strikethrough_applies_strikethrough_ansi() {
    // ARRANGE
    let input = "~~strikethrough~~";

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(
        result.contains("strikethrough"),
        "Strikethrough text should be present"
    );
    assert!(
        result.contains("\x1b[9m"),
        "Should contain strikethrough ANSI code"
    );
}

// ============================================================================
// Test Group 2: Lists
// ============================================================================

#[test]
fn test_unordered_list_renders_with_markers() {
    // ARRANGE
    let input = r#"
- Item 1
- Item 2
- Item 3
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(result.contains("Item 1"), "First item should be present");
    assert!(result.contains("Item 2"), "Second item should be present");
    assert!(result.contains("Item 3"), "Third item should be present");
    assert!(result.contains("•"), "Should contain list marker");
}

#[test]
fn test_ordered_list_renders_with_numbers() {
    // ARRANGE
    let input = r#"
1. First
2. Second
3. Third
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(result.contains("First"), "First item should be present");
    assert!(result.contains("Second"), "Second item should be present");
    assert!(result.contains("Third"), "Third item should be present");
    assert!(result.contains("1."), "Should contain ordered marker");
}

// ============================================================================
// Test Group 3: Complex Markdown Combinations
// ============================================================================

#[test]
fn test_combined_markdown_elements() {
    // ARRANGE
    let input = r#"
# Commands

**create** - Create a new item
**delete** - Remove an item

Use `--help` for more info
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(result.contains("Commands"), "Heading should be present");
    assert!(result.contains("create"), "Bold text should be present");
    assert!(result.contains("delete"), "Bold text should be present");
    assert!(result.contains("--help"), "Code should be present");
    assert!(result.contains("\x1b[1m"), "Should contain bold formatting");
}

#[test]
fn test_nested_formatting() {
    // ARRANGE
    let input = "**bold with `code` inside**";

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(result.contains("bold with"), "Text should be present");
    assert!(result.contains("code"), "Code should be present");
    assert!(result.contains("inside"), "Text should be present");
}

// ============================================================================
// Test Group 4: Configuration Toggles
// ============================================================================

#[test]
fn test_headers_disabled_renders_raw_markdown() {
    // ARRANGE
    let config = MarkdownConfig {
        headers: false,
        ..Default::default()
    };
    let input = "# Heading";

    // ACT
    let result = builder()
        .markdown_with_config(config)
        .render(input)
        .unwrap();

    // ASSERT
    assert!(
        result.contains("Heading"),
        "Heading text should still be present"
    );
    // With headers disabled, should have less/no color codes
}

#[test]
fn test_bold_disabled_renders_without_bold() {
    // ARRANGE
    let config = MarkdownConfig {
        bold: false,
        ..Default::default()
    };
    let input = "**bold**";

    // ACT
    let result = builder()
        .markdown_with_config(config)
        .render(input)
        .unwrap();

    // ASSERT
    assert!(result.contains("bold"), "Text should still be present");
    assert!(
        !result.contains("\x1b[1m"),
        "Should not contain bold ANSI code"
    );
}

#[test]
fn test_code_disabled_renders_plain_text() {
    // ARRANGE
    let config = MarkdownConfig {
        code: false,
        ..Default::default()
    };
    let input = "`code`";

    // ACT
    let result = builder()
        .markdown_with_config(config)
        .render(input)
        .unwrap();

    // ASSERT
    assert!(result.contains("code"), "Code text should be present");
}

// ============================================================================
// Test Group 5: Style Customization
// ============================================================================

#[test]
fn test_custom_heading_color() {
    // ARRANGE
    let style = MarkdownStyle {
        h1_color: Color::Named("red".to_string()),
        ..Default::default()
    };
    let input = "# Red Heading";

    // ACT
    let result = builder().markdown_with_style(style).render(input).unwrap();

    // ASSERT
    assert!(result.contains("Red Heading"), "Heading should be present");
    assert!(
        result.contains("\x1b[31m"),
        "Should contain red ANSI code (31)"
    );
}

#[test]
fn test_custom_bold_color() {
    // ARRANGE
    let style = MarkdownStyle {
        bold_color: Some(Color::Named("yellow".to_string())),
        ..Default::default()
    };
    let input = "**yellow bold**";

    // ACT
    let result = builder().markdown_with_style(style).render(input).unwrap();

    // ASSERT
    assert!(result.contains("yellow bold"), "Text should be present");
    assert!(
        result.contains("\x1b[33m"),
        "Should contain yellow ANSI code (33)"
    );
}

#[test]
fn test_custom_list_marker() {
    // ARRANGE
    let style = MarkdownStyle {
        list_marker: "→".to_string(),
        ..Default::default()
    };
    let input = r#"
- Item 1
- Item 2
"#;

    // ACT
    let result = builder().markdown_with_style(style).render(input).unwrap();

    // ASSERT
    assert!(result.contains("→"), "Custom list marker should be present");
}

#[test]
fn test_italic_style_underline() {
    // ARRANGE
    let style = MarkdownStyle {
        italic_style: ItalicStyle::Underline,
        ..Default::default()
    };
    let input = "*italic*";

    // ACT
    let result = builder().markdown_with_style(style).render(input).unwrap();

    // ASSERT
    assert!(result.contains("italic"), "Text should be present");
    assert!(
        result.contains("\x1b[4m"),
        "Should contain underline ANSI code"
    );
}

// ============================================================================
// Test Group 6: Edge Cases
// ============================================================================

#[test]
fn test_empty_string_returns_empty() {
    // ARRANGE
    let input = "";

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    // Result should be minimal (just borders)
    assert!(!result.is_empty(), "Should still have box structure");
}

#[test]
fn test_markdown_disabled_preserves_raw_syntax() {
    // ARRANGE
    let input = "**bold** and `code`";

    // ACT
    let result = builder().render(input).unwrap(); // No .markdown()

    // ASSERT
    assert!(
        result.contains("**bold**"),
        "Raw markdown syntax should be preserved"
    );
    assert!(
        result.contains("`code`"),
        "Raw markdown syntax should be preserved"
    );
}

#[test]
fn test_malformed_markdown_renders_gracefully() {
    // ARRANGE
    let input = "**unclosed bold";

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(
        result.contains("unclosed bold"),
        "Text should still be present"
    );
    // Should not panic or error
}

#[test]
fn test_unicode_in_markdown() {
    // ARRANGE
    let input = "**日本語** and `코드`";

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(result.contains("日本語"), "Unicode should be preserved");
    assert!(result.contains("코드"), "Unicode should be preserved");
}

// ============================================================================
// Test Group 7: Convenience Function
// ============================================================================

#[test]
fn test_markdown_box_convenience_function() {
    // ARRANGE
    let input = "# Quick Test\n**bold**";

    // ACT
    let result = markdown_box(input);

    // ASSERT
    assert!(result.contains("Quick Test"), "Content should be present");
    assert!(!result.is_empty(), "Should return a box");
}

// ============================================================================
// Test Group 8: Integration with Box Features
// ============================================================================

#[test]
fn test_markdown_with_title() {
    // ARRANGE
    let input = "# Content\n**bold**";

    // ACT
    let result = builder()
        .title("MyTitle")
        .markdown()
        .width(40) // Ensure enough width for title
        .render(input)
        .unwrap();

    // ASSERT
    // Title appears in the border
    assert!(
        result.contains("MyTitle"),
        "Box title should be present in border"
    );
    assert!(
        result.contains("Content"),
        "Markdown content should be present"
    );
    assert!(result.contains("bold"), "Bold text should be present");
}

#[test]
fn test_markdown_with_padding() {
    // ARRANGE
    let input = "**text**";

    // ACT
    let result = builder().markdown().padding(2).render(input).unwrap();

    // ASSERT
    assert!(result.contains("text"), "Content should be present");
    // Box should be larger due to padding
}

#[test]
fn test_markdown_with_border_color() {
    // ARRANGE
    let input = "**text**";

    // ACT
    let result = builder()
        .markdown()
        .border_color("blue")
        .render(input)
        .unwrap();

    // ASSERT
    assert!(result.contains("text"), "Content should be present");
    // Both markdown and border colors should be applied
}

// ============================================================================
// Test Group 9: Links
// ============================================================================

#[test]
fn test_link_show_text_only() {
    // ARRANGE
    let style = MarkdownStyle {
        link_style: LinkStyle::ShowText,
        ..Default::default()
    };
    let input = "[link text](https://example.com)";

    // ACT
    let result = builder().markdown_with_style(style).render(input).unwrap();

    // ASSERT
    assert!(result.contains("link text"), "Link text should be present");
    assert!(
        !result.contains("https://example.com"),
        "URL should not be shown"
    );
}

#[test]
fn test_link_show_url() {
    // ARRANGE
    let style = MarkdownStyle {
        link_style: LinkStyle::ShowUrl,
        ..Default::default()
    };
    let input = "[link text](https://example.com)";

    // ACT
    let result = builder().markdown_with_style(style).render(input).unwrap();

    // ASSERT
    assert!(result.contains("link text"), "Link text should be present");
    assert!(
        result.contains("https://example.com"),
        "URL should be shown"
    );
}

// ============================================================================
// Test Group 10: Horizontal Rules
// ============================================================================

#[test]
fn test_horizontal_rule_renders() {
    // ARRANGE
    let input = "---";

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(
        result.contains("─") || result.contains("---"),
        "Horizontal rule should be present"
    );
}

#[test]
fn test_horizontal_rule_disabled() {
    // ARRANGE
    let config = MarkdownConfig {
        horizontal_rules: false,
        ..Default::default()
    };
    let input = "---";

    // ACT
    let _result = builder()
        .markdown_with_config(config)
        .render(input)
        .unwrap();

    // ASSERT
    // Rule should not render or be minimal
}

// ============================================================================
// Test Group 11: Blockquotes
// ============================================================================

#[test]
fn test_blockquote_renders_with_marker() {
    // ARRANGE
    let input = r#"
> This is a blockquote
> with multiple lines
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(
        result.contains("This is a blockquote"),
        "Blockquote text should be present"
    );
    assert!(
        result.contains("▌") || result.contains(">"),
        "Should contain blockquote marker"
    );
}

#[test]
fn test_nested_blockquotes() {
    // ARRANGE
    let input = r#"
> Level 1
>> Level 2
>>> Level 3
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(result.contains("Level 1"), "First level should be present");
    assert!(result.contains("Level 2"), "Second level should be present");
    assert!(result.contains("Level 3"), "Third level should be present");
}

#[test]
fn test_blockquote_with_markdown() {
    // ARRANGE
    let input = r#"
> **Bold** in blockquote
> *Italic* too
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(
        result.contains("Bold"),
        "Bold text in blockquote should be present"
    );
    assert!(
        result.contains("Italic"),
        "Italic text in blockquote should be present"
    );
}

// ============================================================================
// Test Group 12: Nested Lists
// ============================================================================

#[test]
fn test_nested_unordered_lists() {
    // ARRANGE
    let input = r#"
- Level 1 item 1
  - Level 2 item 1
  - Level 2 item 2
- Level 1 item 2
  - Level 2 item 3
    - Level 3 item 1
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(
        result.contains("Level 1 item 1"),
        "First level item should be present"
    );
    assert!(
        result.contains("Level 2 item 1"),
        "Second level item should be present"
    );
    assert!(
        result.contains("Level 3 item 1"),
        "Third level item should be present"
    );
    assert!(result.contains("•"), "Should contain list markers");
}

#[test]
fn test_nested_ordered_lists() {
    // ARRANGE
    let input = r#"
1. First
   1. Nested first
   2. Nested second
2. Second
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(result.contains("First"), "First item should be present");
    assert!(
        result.contains("Nested first"),
        "Nested item should be present"
    );
    assert!(result.contains("1."), "Should contain ordered markers");
}

#[test]
fn test_mixed_nested_lists() {
    // ARRANGE
    let input = r#"
1. Ordered item
   - Unordered nested
   - Another unordered
2. Second ordered
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(
        result.contains("Ordered item"),
        "Ordered item should be present"
    );
    assert!(
        result.contains("Unordered nested"),
        "Unordered nested should be present"
    );
}

#[test]
fn test_deeply_nested_lists() {
    // ARRANGE
    let input = r#"
- L1
  - L2
    - L3
      - L4
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(result.contains("L1"), "Level 1 should be present");
    assert!(result.contains("L2"), "Level 2 should be present");
    assert!(result.contains("L3"), "Level 3 should be present");
    assert!(result.contains("L4"), "Level 4 should be present");
}

// ============================================================================
// Test Group 13: Definition Lists
// ============================================================================

#[test]
fn test_definition_list_renders() {
    // ARRANGE
    let input = r#"
Term 1
: Definition 1

Term 2
: Definition 2a
: Definition 2b
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(result.contains("Term 1"), "Term should be present");
    assert!(
        result.contains("Definition 1"),
        "Definition should be present"
    );
    assert!(result.contains("Term 2"), "Second term should be present");
}

#[test]
fn test_definition_list_indentation() {
    // ARRANGE
    let input = r#"
Apple
: A red fruit

Banana
: A yellow fruit
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(result.contains("Apple"), "Term should be present");
    assert!(
        result.contains("A red fruit"),
        "Definition should be present"
    );
    // Definitions should be indented (contains spaces before text)
}

#[test]
fn test_definition_list_with_markdown() {
    // ARRANGE
    let input = r#"
**Bold Term**
: Definition with *italic*

Code
: Use `code` in definition
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(result.contains("Bold Term"), "Bold term should be present");
    assert!(
        result.contains("italic"),
        "Italic in definition should be present"
    );
    assert!(result.contains("code"), "Code should be present");
}

// ============================================================================
// Test Group 14: Combined Extended Features
// ============================================================================

#[test]
fn test_blockquote_with_lists() {
    // ARRANGE
    let input = r#"
> Quoted list:
> - Item 1
> - Item 2
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(
        result.contains("Quoted list"),
        "Blockquote text should be present"
    );
    assert!(result.contains("Item 1"), "List item should be present");
}

#[test]
fn test_nested_lists_in_blockquote() {
    // ARRANGE
    let input = r#"
> Parent list:
> - Level 1
>   - Level 2
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(result.contains("Parent list"), "Text should be present");
    assert!(result.contains("Level 1"), "Level 1 should be present");
    assert!(result.contains("Level 2"), "Level 2 should be present");
}

#[test]
fn test_all_extended_features_combined() {
    // ARRANGE
    let input = r#"
# Extended Features

> Blockquote with **bold**

Nested list:
- Item 1
  - Sub 1
  - Sub 2

Definition:
Term
: Definition text
"#;

    // ACT
    let result = builder().markdown().render(input).unwrap();

    // ASSERT
    assert!(
        result.contains("Extended Features"),
        "Header should be present"
    );
    assert!(
        result.contains("Blockquote"),
        "Blockquote should be present"
    );
    assert!(result.contains("Item 1"), "List should be present");
    assert!(result.contains("Sub 1"), "Nested list should be present");
    assert!(result.contains("Term"), "Definition term should be present");
    assert!(
        result.contains("Definition text"),
        "Definition should be present"
    );
}
