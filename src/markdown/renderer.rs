//! Markdown to ANSI terminal text renderer.

use super::styles::{BoldStyle, ItalicStyle, LinkStyle, MarkdownConfig, MarkdownStyle};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Converts markdown text to ANSI-styled terminal output
pub struct MarkdownRenderer {
    style: MarkdownStyle,
    config: MarkdownConfig,
}

impl MarkdownRenderer {
    /// Create a new markdown renderer with the given style and configuration
    pub fn new(style: MarkdownStyle, config: MarkdownConfig) -> Self {
        Self { style, config }
    }

    /// Render markdown text to ANSI-styled string
    pub fn render(&self, markdown: &str) -> String {
        let mut options = Options::empty();
        if self.config.strikethrough {
            options.insert(Options::ENABLE_STRIKETHROUGH);
        }
        if self.config.lists {
            options.insert(Options::ENABLE_TASKLISTS);
        }
        // Enable definition lists
        options.insert(Options::ENABLE_DEFINITION_LIST);

        let parser = Parser::new_ext(markdown, options);
        let mut output = String::with_capacity(markdown.len() * 2);
        let mut state = RenderState::default();

        for event in parser {
            self.handle_event(event, &mut output, &mut state);
        }

        output
    }

    fn handle_event(&self, event: Event, output: &mut String, state: &mut RenderState) {
        match event {
            Event::Start(tag) => self.handle_start_tag(tag, output, state),
            Event::End(tag_end) => self.handle_end_tag(tag_end, output, state),
            Event::Text(text) => {
                output.push_str(&text);
            }
            Event::Code(code) => {
                if self.config.code {
                    self.handle_inline_code(&code, output);
                } else {
                    output.push_str(&code);
                }
            }
            Event::SoftBreak => output.push('\n'),
            Event::HardBreak => output.push('\n'),
            Event::Rule => {
                if self.config.horizontal_rules {
                    self.handle_horizontal_rule(output);
                }
            }
            Event::Html(_) | Event::InlineHtml(_) => {}
            Event::FootnoteReference(_) => {}
            Event::TaskListMarker(_) => {}
            Event::InlineMath(_) | Event::DisplayMath(_) => {}
        }
    }

    fn handle_start_tag(&self, tag: Tag, output: &mut String, state: &mut RenderState) {
        match tag {
            Tag::Paragraph => {}
            Tag::Heading { level, .. } => {
                if self.config.headers {
                    state.in_heading = Some(level);
                    let color = self.get_heading_color(level);
                    output.push_str(&self.color_to_ansi_fg(&color));
                }
            }
            Tag::BlockQuote(_) => {
                state.in_blockquote = true;
                state.blockquote_depth += 1;
                if self.config.blockquotes {
                    // Add blockquote marker at the start of the line
                    if let Some(color) = &self.style.blockquote_color {
                        output.push_str(&self.color_to_ansi_fg(color));
                    }
                    output.push_str(&self.style.blockquote_marker);
                    output.push(' ');
                    if self.style.blockquote_color.is_some() {
                        output.push_str("\x1b[0m");
                    }
                }
            }
            Tag::CodeBlock(_) => {
                state.in_code_block = true;
                if self.config.code {
                    if let Some(fg) = &self.style.code_block_fg {
                        output.push_str(&self.color_to_ansi_fg(fg));
                    }
                }
            }
            Tag::List(start) => {
                if self.config.lists {
                    state.list_depth += 1;
                    state.list_item_number = start;
                }
            }
            Tag::Item => {
                if self.config.lists {
                    state.in_list_item = true;

                    // Handle nested lists with proper indentation
                    let indent = "  ".repeat(state.list_depth.saturating_sub(1));
                    output.push_str(&indent);

                    if let Some(num) = state.list_item_number {
                        output.push_str(&format!("{}. ", num));
                        state.list_item_number = Some(num + 1);
                    } else {
                        if let Some(color) = &self.style.list_marker_color {
                            output.push_str(&self.color_to_ansi_fg(color));
                        }
                        output.push_str(&self.style.list_marker);
                        output.push(' ');
                        if self.style.list_marker_color.is_some() {
                            output.push_str("\x1b[0m");
                        }
                    }
                }
            }
            Tag::Emphasis => {
                if self.config.italic {
                    match self.style.italic_style {
                        ItalicStyle::Italic => output.push_str("\x1b[3m"),
                        ItalicStyle::Underline => output.push_str("\x1b[4m"),
                        ItalicStyle::Dim => output.push_str("\x1b[2m"),
                        ItalicStyle::None => {}
                    }
                }
            }
            Tag::Strong => {
                if self.config.bold {
                    if let Some(color) = &self.style.bold_color {
                        output.push_str(&self.color_to_ansi_fg(color));
                    }
                    match self.style.bold_style {
                        BoldStyle::Bold | BoldStyle::BoldColor => output.push_str("\x1b[1m"),
                        BoldStyle::BrightColor => {}
                    }
                }
            }
            Tag::Strikethrough => {
                if self.config.strikethrough {
                    output.push_str("\x1b[9m");
                }
            }
            Tag::Link { dest_url, .. } => {
                state.link_url = Some(dest_url.to_string());
                if self.config.links {
                    if let Some(color) = &self.style.link_color {
                        output.push_str(&self.color_to_ansi_fg(color));
                    }
                }
            }
            Tag::Image { .. } => {}
            Tag::FootnoteDefinition(_) => {}
            Tag::HtmlBlock => {}
            Tag::Table(_) => {}
            Tag::TableHead => {}
            Tag::TableRow => {}
            Tag::TableCell => {}
            Tag::DefinitionList => {
                if self.config.lists {
                    state.in_definition_list = true;
                    output.push('\n');
                }
            }
            Tag::DefinitionListTitle => {
                if self.config.lists {
                    state.in_definition_title = true;
                    // Make definition titles bold
                    output.push_str("\x1b[1m");
                }
            }
            Tag::DefinitionListDefinition => {
                if self.config.lists {
                    state.in_definition_definition = true;
                    output.push_str("  "); // Indent definitions
                }
            }
            Tag::Superscript => {}
            Tag::Subscript => {}
            Tag::MetadataBlock(_) => {}
        }
    }

    fn handle_end_tag(&self, tag_end: TagEnd, output: &mut String, state: &mut RenderState) {
        match tag_end {
            TagEnd::Paragraph => {
                output.push('\n');
                if !state.in_blockquote && !state.in_list_item {
                    output.push('\n');
                }
            }
            TagEnd::Heading(_) => {
                output.push_str("\x1b[0m");
                output.push('\n');
                output.push('\n');
                state.in_heading = None;
            }
            TagEnd::BlockQuote(_) => {
                state.blockquote_depth = state.blockquote_depth.saturating_sub(1);
                if state.blockquote_depth == 0 {
                    state.in_blockquote = false;
                }
                output.push('\n');
            }
            TagEnd::CodeBlock => {
                output.push_str("\x1b[0m");
                output.push('\n');
                state.in_code_block = false;
            }
            TagEnd::List(_) => {
                if self.config.lists {
                    state.list_depth = state.list_depth.saturating_sub(1);
                    state.list_item_number = None;
                    if state.list_depth == 0 {
                        output.push('\n');
                    }
                }
            }
            TagEnd::Item => {
                if self.config.lists {
                    state.in_list_item = false;
                    output.push('\n');
                }
            }
            TagEnd::Emphasis => {
                if self.config.italic {
                    output.push_str("\x1b[0m");
                }
            }
            TagEnd::Strong => {
                if self.config.bold {
                    output.push_str("\x1b[0m");
                }
            }
            TagEnd::Strikethrough => {
                if self.config.strikethrough {
                    output.push_str("\x1b[0m");
                }
            }
            TagEnd::Link => {
                if self.config.links {
                    if let Some(url) = state.link_url.take() {
                        match self.style.link_style {
                            LinkStyle::ShowUrl => {
                                output.push_str(&format!(" ({})", url));
                            }
                            LinkStyle::ShowBoth => {
                                output.push_str(&format!(": {}", url));
                            }
                            LinkStyle::ShowText | LinkStyle::Hide => {}
                        }
                    }
                    output.push_str("\x1b[0m");
                }
            }
            TagEnd::Image => {}
            TagEnd::FootnoteDefinition => {}
            TagEnd::HtmlBlock => {}
            TagEnd::Table => {}
            TagEnd::TableHead => {}
            TagEnd::TableRow => {}
            TagEnd::TableCell => {}
            TagEnd::DefinitionList => {
                if self.config.lists {
                    state.in_definition_list = false;
                    output.push('\n');
                }
            }
            TagEnd::DefinitionListTitle => {
                if self.config.lists {
                    state.in_definition_title = false;
                    output.push_str("\x1b[0m"); // Reset bold
                    output.push('\n');
                }
            }
            TagEnd::DefinitionListDefinition => {
                if self.config.lists {
                    state.in_definition_definition = false;
                    output.push('\n');
                }
            }
            TagEnd::Superscript => {}
            TagEnd::Subscript => {}
            TagEnd::MetadataBlock(_) => {}
        }
    }

    fn handle_inline_code(&self, code: &str, output: &mut String) {
        if let Some(fg) = &self.style.inline_code_fg {
            output.push_str(&self.color_to_ansi_fg(fg));
        }
        if let Some(bg) = &self.style.inline_code_bg {
            output.push_str(&self.color_to_ansi_bg(bg));
        }

        output.push_str(code);
        output.push_str("\x1b[0m");
    }

    fn handle_horizontal_rule(&self, output: &mut String) {
        if let Some(color) = &self.style.hr_color {
            output.push_str(&self.color_to_ansi_fg(color));
        }

        output.push_str(&self.style.hr_char.to_string().repeat(40));

        if self.style.hr_color.is_some() {
            output.push_str("\x1b[0m");
        }
        output.push('\n');
    }

    fn get_heading_color(&self, level: HeadingLevel) -> &crate::Color {
        match level {
            HeadingLevel::H1 => &self.style.h1_color,
            HeadingLevel::H2 => &self.style.h2_color,
            HeadingLevel::H3 => &self.style.h3_color,
            HeadingLevel::H4 => &self.style.h4_color,
            HeadingLevel::H5 => &self.style.h5_color,
            HeadingLevel::H6 => &self.style.h6_color,
        }
    }

    fn color_to_ansi_fg(&self, color: &crate::Color) -> String {
        match color {
            crate::Color::Named(name) => match name.to_lowercase().as_str() {
                "black" => "\x1b[30m".to_string(),
                "red" => "\x1b[31m".to_string(),
                "green" => "\x1b[32m".to_string(),
                "yellow" => "\x1b[33m".to_string(),
                "blue" => "\x1b[34m".to_string(),
                "magenta" | "purple" => "\x1b[35m".to_string(),
                "cyan" => "\x1b[36m".to_string(),
                "white" => "\x1b[37m".to_string(),
                "bright-black" | "brightblack" | "gray" | "grey" => "\x1b[90m".to_string(),
                "bright-red" | "brightred" => "\x1b[91m".to_string(),
                "bright-green" | "brightgreen" => "\x1b[92m".to_string(),
                "bright-yellow" | "brightyellow" => "\x1b[93m".to_string(),
                "bright-blue" | "brightblue" => "\x1b[94m".to_string(),
                "bright-magenta" | "brightmagenta" | "bright-purple" | "brightpurple" => {
                    "\x1b[95m".to_string()
                }
                "bright-cyan" | "brightcyan" => "\x1b[96m".to_string(),
                "bright-white" | "brightwhite" => "\x1b[97m".to_string(),
                _ => "\x1b[37m".to_string(),
            },
            crate::Color::Rgb(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
            crate::Color::Hex(hex) => {
                let hex = hex.trim_start_matches('#');
                if hex.len() == 6 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&hex[0..2], 16),
                        u8::from_str_radix(&hex[2..4], 16),
                        u8::from_str_radix(&hex[4..6], 16),
                    ) {
                        return format!("\x1b[38;2;{};{};{}m", r, g, b);
                    }
                }
                "\x1b[37m".to_string()
            }
        }
    }

    fn color_to_ansi_bg(&self, color: &crate::Color) -> String {
        match color {
            crate::Color::Named(name) => match name.to_lowercase().as_str() {
                "black" => "\x1b[40m".to_string(),
                "red" => "\x1b[41m".to_string(),
                "green" => "\x1b[42m".to_string(),
                "yellow" => "\x1b[43m".to_string(),
                "blue" => "\x1b[44m".to_string(),
                "magenta" | "purple" => "\x1b[45m".to_string(),
                "cyan" => "\x1b[46m".to_string(),
                "white" => "\x1b[47m".to_string(),
                "bright-black" | "brightblack" | "gray" | "grey" => "\x1b[100m".to_string(),
                "bright-red" | "brightred" => "\x1b[101m".to_string(),
                "bright-green" | "brightgreen" => "\x1b[102m".to_string(),
                "bright-yellow" | "brightyellow" => "\x1b[103m".to_string(),
                "bright-blue" | "brightblue" => "\x1b[104m".to_string(),
                "bright-magenta" | "brightmagenta" | "bright-purple" | "brightpurple" => {
                    "\x1b[105m".to_string()
                }
                "bright-cyan" | "brightcyan" => "\x1b[106m".to_string(),
                "bright-white" | "brightwhite" => "\x1b[107m".to_string(),
                _ => "\x1b[40m".to_string(),
            },
            crate::Color::Rgb(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
            crate::Color::Hex(hex) => {
                let hex = hex.trim_start_matches('#');
                if hex.len() == 6 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&hex[0..2], 16),
                        u8::from_str_radix(&hex[2..4], 16),
                        u8::from_str_radix(&hex[4..6], 16),
                    ) {
                        return format!("\x1b[48;2;{};{};{}m", r, g, b);
                    }
                }
                "\x1b[40m".to_string()
            }
        }
    }
}

#[derive(Default)]
struct RenderState {
    in_heading: Option<HeadingLevel>,
    in_blockquote: bool,
    blockquote_depth: usize,
    in_code_block: bool,
    in_list_item: bool,
    list_depth: usize,
    list_item_number: Option<u64>,
    link_url: Option<String>,
    in_definition_list: bool,
    in_definition_title: bool,
    in_definition_definition: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_text() {
        let renderer = MarkdownRenderer::new(MarkdownStyle::default(), MarkdownConfig::default());
        let result = renderer.render("Hello world");
        assert!(result.contains("Hello world"));
    }

    #[test]
    fn test_bold_text() {
        let renderer = MarkdownRenderer::new(MarkdownStyle::default(), MarkdownConfig::default());
        let result = renderer.render("**bold**");
        assert!(result.contains("bold"));
        assert!(result.contains("\x1b[1m"));
    }

    #[test]
    fn test_inline_code() {
        let renderer = MarkdownRenderer::new(MarkdownStyle::default(), MarkdownConfig::default());
        let result = renderer.render("`code`");
        assert!(result.contains("code"));
    }

    #[test]
    fn test_heading() {
        let renderer = MarkdownRenderer::new(MarkdownStyle::default(), MarkdownConfig::default());
        let result = renderer.render("# Heading");
        assert!(result.contains("Heading"));
    }
}
