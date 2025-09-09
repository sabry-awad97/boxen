//! # Core Boxen Rendering Engine
//!
//! This module contains the main boxen rendering functionality, providing the core
//! `boxen` function that transforms text content into beautifully formatted terminal
//! boxes with comprehensive styling, layout, and error handling capabilities.
//!
//! ## Overview
//!
//! The boxen rendering engine is the heart of the library, orchestrating text processing,
//! layout calculation, border rendering, and color application to create visually
//! appealing terminal output. It handles complex scenarios including Unicode text,
//! ANSI escape sequences, dynamic sizing, and terminal constraints.
//!
//! ## Core Components
//!
//! ### Main Rendering Function (`boxen`)
//! - **Text Processing**: Handles Unicode, ANSI sequences, and text wrapping
//! - **Layout Calculation**: Determines optimal box dimensions and positioning
//! - **Border Rendering**: Draws borders with various styles and decorations
//! - **Color Application**: Applies colors and styling with terminal compatibility
//!
//! ### Content Processing (`process_content`)
//! - **Text Wrapping**: Intelligent line breaking and width constraint handling
//! - **Alignment Processing**: Horizontal and vertical text alignment
//! - **Dimension Calculation**: Accurate content measurement and sizing
//! - **Unicode Handling**: Proper width calculation for complex characters
//!
//! ### Box Rendering (`render_box`)
//! - **Border Construction**: Creates top, middle, and bottom border sections
//! - **Content Integration**: Combines text content with border elements
//! - **Title Rendering**: Positions and styles optional box titles
//! - **Final Assembly**: Combines all elements into the final output string
//!
//! ## Quick Start
//!
//! ```rust
//! # fn main() {
//! // Simple box with default styling
//! if let Ok(result) = boxen::boxen("Hello, World!", None) {
//!     println!("{}", result);
//! }
//! # }
//! ```
//!
//! ## Advanced Usage
//!
//! ### Custom Styling
//! ```rust
//! use boxen::{BoxenOptions, BorderStyle, TextAlignment, Spacing, Color};
//!
//! # fn main() {
//! let options = BoxenOptions {
//!     border_style: BorderStyle::Double,
//!     padding: Spacing::from(2),
//!     text_alignment: TextAlignment::Center,
//!     border_color: Some(Color::Named("blue".to_string())),
//!     title: Some("Important Message".to_string()),
//!     ..Default::default()
//! };
//!
//! if let Ok(result) = boxen::boxen("Your content here", Some(options)) {
//!     println!("{}", result);
//! }
//! # }
//! ```
//!
//! ### Dynamic Sizing
//! ```rust
//! use boxen::{BoxenOptions, Spacing};
//!
//! # fn main() {
//! // Auto-size to terminal width with margins
//! let options = BoxenOptions {
//!     margin: Spacing::from(4),
//!     ..Default::default()
//! };
//!
//! if let Ok(result) = boxen::boxen("This will size to fit the terminal", Some(options)) {
//!     println!("{}", result);
//! }
//! # }
//! ```
//!
//! ### Layout Control
//! ```rust
//! use boxen::{BoxenOptions, Float, Spacing};
//!
//! # fn main() {
//! let options = BoxenOptions {
//!     width: Some(60),
//!     height: Some(10),
//!     float: Float::Center,
//!     margin: Spacing::from(3),
//!     ..Default::default()
//! };
//!
//! if let Ok(result) = boxen::boxen("Centered box with fixed dimensions", Some(options)) {
//!     println!("{}", result);
//! }
//! # }
//! ```
//!
//! ### Multiline Content
//! ```rust
//! use boxen::{BoxenOptions, TextAlignment, Spacing};
//!
//! # fn main() {
//! let content = "Line 1\nLine 2\nLine 3\nLonger line of text";
//! let options = BoxenOptions {
//!     text_alignment: TextAlignment::Center,
//!     padding: Spacing::from(1),
//!     ..Default::default()
//! };
//!
//! if let Ok(result) = boxen::boxen(content, Some(options)) {
//!     println!("{}", result);
//! }
//! # }
//! ```
//!
//! ## Error Handling and Recovery
//!
//! The rendering engine includes comprehensive error handling with actionable recommendations:
//!
//! ### Validation Errors
//! ```rust
//! use boxen::{BoxenOptions, Spacing};
//!
//! # fn main() {
//! let options = BoxenOptions {
//!     width: Some(5),  // Too small for content
//!     padding: Spacing::from(10), // Excessive padding
//!     ..Default::default()
//! };
//!
//! match boxen::boxen("Long content that won't fit", Some(options)) {
//!     Ok(result) => println!("{}", result),
//!     Err(error) => {
//!         println!("Configuration error: {}", error);
//!         
//!         // Access specific recommendations
//!         for recommendation in error.recommendations() {
//!             println!("Suggestion: {}", recommendation.suggestion);
//!             if let Some(fix) = &recommendation.auto_fix {
//!                 println!("Auto-fix: {}", fix);
//!             }
//!         }
//!     }
//! }
//! # }
//! ```
//!
//! ### Automatic Recovery
//! ```rust
//! use boxen::BoxenOptions;
//!
//! # fn main() {
//! let options = BoxenOptions {
//!     width: Some(3), // Invalid - too small
//!     ..Default::default()
//! };
//! let content = "Content that doesn't fit";
//!
//! // This will return an error with recommendations
//! match boxen::boxen(content, Some(options)) {
//!     Ok(result) => println!("{}", result),
//!     Err(e) => println!("Error: {}", e),
//! }
//! # }
//! ```
//!
//! ## Performance Characteristics
//!
//! The rendering engine is optimized for performance across various scenarios:
//!
//! ### Memory Efficiency
//! - **Pre-allocation**: Estimates output size to minimize reallocations
//! - **String Reuse**: Reuses buffers where possible during processing
//! - **Lazy Processing**: Only processes content when necessary
//! - **Minimal Copying**: Avoids unnecessary string duplication
//!
//! ### Processing Speed
//! - **Fast ASCII Path**: Optimized handling for ASCII-only content
//! - **Unicode Optimization**: Efficient width calculation for complex characters
//! - **Cached Calculations**: Memoizes expensive operations like terminal size
//! - **Linear Complexity**: Most operations scale linearly with content size
//!
//! ### Benchmark Results
//! ```rust
//! use std::time::Instant;
//!
//! # fn main() {
//! let start = Instant::now();
//! if let Ok(result) = boxen::boxen("Sample content", None) {
//!     let duration = start.elapsed();
//!     // Typical performance: < 1ms for normal content
//!     println!("Rendered in {:?}", duration);
//! }
//! # }
//! ```
//!
//! ## Unicode and International Support
//!
//! The rendering engine provides comprehensive Unicode support:
//!
//! ### Character Width Handling
//! ```rust
//! # fn main() {
//! // Properly handles wide characters (CJK)
//! let _ = boxen::boxen("你好世界", None); // Chinese characters
//! let _ = boxen::boxen("こんにちは", None); // Japanese characters
//! let _ = boxen::boxen("안녕하세요", None); // Korean characters
//!
//! // Handles combining characters
//! let _ = boxen::boxen("café naïve résumé", None);
//!
//! // Supports emoji
//! let _ = boxen::boxen("🎉 Celebration! 🎊", None);
//! # }
//! ```
//!
//! ### ANSI Sequence Preservation
//! ```rust
//! # fn main() {
//! // Colors and formatting are preserved
//! let colored_text = "\x1b[31mRed\x1b[0m \x1b[32mGreen\x1b[0m \x1b[34mBlue\x1b[0m";
//! let _ = boxen::boxen(colored_text, None);
//!
//! // Bold and italic formatting preserved
//! let formatted = "\x1b[1mBold\x1b[0m and \x1b[3mItalic\x1b[0m text";
//! let _ = boxen::boxen(formatted, None);
//! # }
//! ```
//!
//! ## Integration with Terminal Features
//!
//! The rendering engine integrates seamlessly with terminal capabilities:
//!
//! ### Terminal Size Detection
//! ```rust
//! use boxen::{BoxenOptions, Spacing};
//!
//! # fn main() {
//! // Automatically respects terminal dimensions
//! let options = BoxenOptions {
//!     margin: Spacing::from(2),
//!     ..Default::default()
//! };
//!
//! let _ = boxen::boxen("Content that adapts to terminal size", Some(options));
//! # }
//! ```
//!
//! ### Color Support Detection
//! ```rust
//! use boxen::{BoxenOptions, Color};
//!
//! # fn main() {
//! let options = BoxenOptions {
//!     border_color: Some(Color::Named("blue".to_string())),
//!     background_color: Some(Color::Named("lightgray".to_string())),
//!     ..Default::default()
//! };
//!
//! let _ = boxen::boxen("Colored box content", Some(options));
//! # }
//! ```
//!
//! ## Thread Safety and Concurrency
//!
//! All rendering functions are thread-safe and can be used in concurrent environments:
//!
//! ```rust
//! use std::thread;
//!
//! # fn main() {
//! let handles: Vec<_> = (0..10)
//!     .map(|i| {
//!         thread::spawn(move || {
//!             let content = format!("Thread {} content", i);
//!             boxen::boxen(&content, None)
//!         })
//!     })
//!     .collect();
//!
//! for handle in handles {
//!     if let Ok(result) = handle.join().unwrap() {
//!         println!("{}", result);
//!     }
//! }
//! # }
//! ```
//!
//! ## Testing and Quality Assurance
//!
//! The rendering engine includes comprehensive test coverage:
//! - **Unit Tests**: Individual function validation
//! - **Integration Tests**: End-to-end rendering scenarios
//! - **Unicode Tests**: Complex character handling validation
//! - **Performance Tests**: Speed and memory usage benchmarks
//! - **Error Handling Tests**: Comprehensive error scenario coverage

use crate::color::{apply_color_with_dim, apply_colors};
use crate::error::BoxenResult;
use crate::options::{BoxenOptions, TitleAlignment};
use crate::text::text_width;
use crate::text::wrapping::wrap_text;
use std::fmt::Write;

/// Main boxen function that renders text within a styled box.
///
/// This is the core function of the boxen library. It takes text and optional configuration
/// and returns a formatted string with the text enclosed in a styled box.
///
/// # Arguments
///
/// * `text` - The text content to be boxed. Can be a `&str`, `String`, or any type that implements `AsRef<str>`
/// * `options` - Optional configuration for the box. If `None`, default settings are used
///
/// # Returns
///
/// Returns `Result<String, BoxenError>` where:
/// - `Ok(String)` contains the formatted box as a string ready for terminal output
/// - `Err(BoxenError)` contains detailed error information with helpful recommendations
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use ::boxen::boxen;
///
/// // Simple box with default settings
/// let result = boxen("Hello, World!", None).unwrap();
/// println!("{}", result);
/// // ┌─────────────┐
/// // │Hello, World!│
/// // └─────────────┘
/// ```
///
/// ## With Custom Options
///
/// ```rust
/// use ::boxen::{boxen, BoxenOptions, BorderStyle, TextAlignment, Spacing};
///
/// let options = BoxenOptions {
///     border_style: BorderStyle::Double,
///     padding: Spacing::from(1),
///     text_alignment: TextAlignment::Center,
///     title: Some("Greeting".to_string()),
///     width: Some(20),
///     ..Default::default()
/// };
///
/// let result = boxen("Hello!", Some(options)).unwrap();
/// ```
///
/// ## Multiline Text
///
/// ```rust
/// use ::boxen::boxen;
///
/// let multiline = "Line 1\nLine 2\nLine 3";
/// let result = boxen(multiline, None).unwrap();
/// // ┌──────┐
/// // │Line 1│
/// // │Line 2│
/// // │Line 3│
/// // └──────┘
/// ```
///
/// # Error Handling
///
/// The function validates all input parameters and configuration options.
/// Common error scenarios include:
///
/// - Invalid dimensions (width/height too small for content + padding + borders)
/// - Invalid color specifications
/// - Configuration conflicts (e.g., fullscreen mode with explicit dimensions)
///
/// All errors include detailed messages and actionable recommendations:
///
/// ```rust
/// use ::boxen::{boxen, BoxenOptions, Spacing};
///
/// let options = BoxenOptions {
///     width: Some(5),  // Too small
///     padding: Spacing::from(10),  // Too large
///     ..Default::default()
/// };
///
/// match boxen("Hello", Some(options)) {
///     Ok(result) => println!("{}", result),
///     Err(e) => {
///         println!("Error: {}", e);
///         // Error includes recommendations for fixing the issue
///         for rec in e.recommendations() {
///             println!("Suggestion: {}", rec.suggestion);
///         }
///     }
/// }
/// ```
///
/// # Performance
///
/// This function is optimized for performance:
/// - Pre-allocates string buffers based on estimated output size
/// - Uses efficient Unicode width calculation with fast paths for ASCII text
/// - Minimizes memory allocations in text processing
/// - Caches terminal dimensions to avoid repeated system calls
///
/// For repeated rendering with the same options, consider using the builder pattern
/// which can reuse validated configuration.
///
/// # Unicode and ANSI Support
///
/// The function properly handles:
/// - Unicode characters with correct width calculation (CJK characters count as 2 width)
/// - ANSI escape sequences for colors and formatting (preserved in output)
/// - Combining characters and zero-width characters
///
/// ```rust
/// use ::boxen::boxen;
///
/// // Unicode text
/// let result = boxen("你好世界", None).unwrap();
///
/// // Text with ANSI colors (colors are preserved)
/// let colored = "\x1b[31mRed text\x1b[0m";
/// let result = boxen(colored, None).unwrap();
/// ```
pub fn boxen<S: AsRef<str>>(text: S, options: Option<BoxenOptions>) -> BoxenResult<String> {
    let text = text.as_ref();
    let options = options.unwrap_or_default();

    // Comprehensive input validation
    crate::error::validation::validate_all_options(text, &options).map_err(|e| {
        crate::error::BoxenError::rendering_error(
            format!("Input validation failed: {}", e),
            e.recommendations(),
        )
    })?;

    // Process the text content
    let processed_content = process_content(text, &options).map_err(|e| {
        crate::error::BoxenError::rendering_error(
            format!("Text processing failed: {}", e),
            vec![crate::error::ErrorRecommendation::suggestion_only(
                "Text processing error".to_string(),
                "Check your text content and box dimensions".to_string(),
            )],
        )
    })?;

    // Calculate final layout dimensions
    let layout = options
        .calculate_layout_dimensions(
            processed_content.content_width,
            processed_content.content_height,
        )
        .map_err(|e| {
            crate::error::BoxenError::rendering_error(
                format!("Layout calculation failed: {}", e),
                e.recommendations(),
            )
        })?;

    // Render the box
    render_box(&processed_content, &options, &layout).map_err(|e| {
        crate::error::BoxenError::rendering_error(
            format!("Box rendering failed: {}", e),
            vec![crate::error::ErrorRecommendation::suggestion_only(
                "Rendering error".to_string(),
                "Check your configuration and try reducing complexity".to_string(),
            )],
        )
    })
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

    // Calculate maximum content height available
    let max_content_height = options.calculate_max_content_height()?;

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

    // Determine target width based on options
    let target_width = if options.fullscreen.is_some() || options.width.is_some() {
        // For fullscreen mode or specified width, use the maximum available content width
        max_content_width
    } else {
        // For normal mode, use the smaller of natural width or max width for better layout
        natural_content_width.min(max_content_width)
    };

    // Apply height constraints if specified
    let height_constrained_lines = if let Some(max_height) = max_content_height {
        crate::text::apply_height_constraints(&wrapped_lines, max_height)
    } else {
        wrapped_lines
    };

    // Apply text alignment without padding (padding will be applied during rendering)
    let aligned_lines = crate::text::align_lines(
        &height_constrained_lines,
        options.text_alignment.clone(),
        target_width,
    );

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

    // Pre-allocate string capacity to reduce reallocations
    let estimated_chars_per_line = layout.total_width + 10; // +10 for ANSI codes
    let estimated_lines = layout.total_height + options.margin.vertical();
    let estimated_capacity = estimated_chars_per_line * estimated_lines;
    let mut result = String::with_capacity(estimated_capacity);

    // Add top margins
    for _ in 0..options.margin.top {
        writeln!(result).unwrap();
    }

    if has_border {
        // Render top border with title
        let top_border = render_top_border(&border_chars, options, layout.inner_width)?;
        add_line_with_float_positioning(&mut result, &top_border, options, layout);

        // Render content lines with borders and padding
        render_content_with_borders(&mut result, content, options, layout, &border_chars)?;

        // Render bottom border
        let bottom_border = render_bottom_border(&border_chars, layout.inner_width, options)?;
        add_line_with_float_positioning(&mut result, &bottom_border, options, layout);
    } else {
        // No border - just render content with padding and margins
        render_content_without_borders(&mut result, content, options, layout)?;
    }

    // Add bottom margins as empty lines
    for _ in 0..options.margin.bottom {
        result.push('\n');
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

    // Apply border color and dim styling
    let styled_border =
        apply_color_with_dim(&border, options.border_color.as_ref(), options.dim_border)?;

    Ok(styled_border.to_string())
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
fn render_bottom_border(
    border_chars: &crate::options::BorderChars,
    inner_width: usize,
    options: &BoxenOptions,
) -> BoxenResult<String> {
    let mut border = String::new();

    border.push(border_chars.bottom_left);
    for _ in 0..inner_width {
        border.push(border_chars.bottom);
    }
    border.push(border_chars.bottom_right);

    // Apply border color and dim styling
    let styled_border =
        apply_color_with_dim(&border, options.border_color.as_ref(), options.dim_border)?;

    Ok(styled_border.to_string())
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
        let padded_line = render_padded_empty_line(border_chars, layout.inner_width, options)?;
        add_line_with_float_positioning(result, &padded_line, options, layout);
    }

    // Render content lines
    for line in &content.lines {
        let content_line = render_content_line(line, border_chars, options, layout.inner_width)?;
        add_line_with_float_positioning(result, &content_line, options, layout);
    }

    // Add bottom padding
    for _ in 0..options.padding.bottom {
        let padded_line = render_padded_empty_line(border_chars, layout.inner_width, options)?;
        add_line_with_float_positioning(result, &padded_line, options, layout);
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
        add_line_with_float_positioning(result, &title_line, options, layout);
    }

    // Add top padding
    for _ in 0..options.padding.top {
        let empty_line = " ".repeat(layout.inner_width);
        let styled_line = if let Some(bg_color) = &options.background_color {
            apply_colors(&empty_line, None, Some(bg_color))?.to_string()
        } else {
            empty_line
        };
        add_line_with_float_positioning(result, &styled_line, options, layout);
    }

    // Render content lines with padding
    for line in &content.lines {
        let padded_line = format!(
            "{}{}{}",
            " ".repeat(options.padding.left),
            line,
            " ".repeat(options.padding.right)
        );
        let styled_line = if let Some(bg_color) = &options.background_color {
            apply_colors(&padded_line, None, Some(bg_color))?.to_string()
        } else {
            padded_line
        };
        add_line_with_float_positioning(result, &styled_line, options, layout);
    }

    // Add bottom padding
    for _ in 0..options.padding.bottom {
        let empty_line = " ".repeat(layout.inner_width);
        let styled_line = if let Some(bg_color) = &options.background_color {
            apply_colors(&empty_line, None, Some(bg_color))?.to_string()
        } else {
            empty_line
        };
        add_line_with_float_positioning(result, &styled_line, options, layout);
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
    // Build the content area (padding + content)
    let mut content_area = String::new();

    // Left padding
    for _ in 0..options.padding.left {
        content_area.push(' ');
    }

    // Content
    content_area.push_str(line);

    // Right padding (fill to inner width)
    let current_content_width = text_width(&content_area);
    let remaining_width = inner_width - current_content_width;
    for _ in 0..remaining_width {
        content_area.push(' ');
    }

    // Apply background color to content area if specified
    let styled_content = if let Some(bg_color) = &options.background_color {
        apply_colors(&content_area, None, Some(bg_color))?.to_string()
    } else {
        content_area
    };

    // Build borders separately and apply border styling
    let left_border = apply_color_with_dim(
        &border_chars.left.to_string(),
        options.border_color.as_ref(),
        options.dim_border,
    )?
    .to_string();

    let right_border = apply_color_with_dim(
        &border_chars.right.to_string(),
        options.border_color.as_ref(),
        options.dim_border,
    )?
    .to_string();

    // Combine borders and content
    let content_line = format!("{}{}{}", left_border, styled_content, right_border);

    Ok(content_line)
}

/// Render an empty line with borders and padding (for top/bottom padding)
fn render_padded_empty_line(
    border_chars: &crate::options::BorderChars,
    inner_width: usize,
    options: &BoxenOptions,
) -> BoxenResult<String> {
    // Build the content area (all spaces)
    let content_area = " ".repeat(inner_width);

    // Apply background color to content area if specified
    let styled_content = if let Some(bg_color) = &options.background_color {
        apply_colors(&content_area, None, Some(bg_color))?.to_string()
    } else {
        content_area
    };

    // Build borders separately and apply border styling
    let left_border = apply_color_with_dim(
        &border_chars.left.to_string(),
        options.border_color.as_ref(),
        options.dim_border,
    )?
    .to_string();

    let right_border = apply_color_with_dim(
        &border_chars.right.to_string(),
        options.border_color.as_ref(),
        options.dim_border,
    )?
    .to_string();

    // Combine borders and content
    let line = format!("{}{}{}", left_border, styled_content, right_border);

    Ok(line)
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

    // Apply background color if specified
    let styled_title = if let Some(bg_color) = &options.background_color {
        apply_colors(&title_line, None, Some(bg_color))?.to_string()
    } else {
        title_line
    };

    Ok(styled_title)
}

/// Add a line to the result with float positioning applied
fn add_line_with_float_positioning(
    result: &mut String,
    line: &str,
    options: &BoxenOptions,
    layout: &crate::options::LayoutDimensions,
) {
    use crate::options::Float;
    use crate::terminal::get_terminal_width;

    let terminal_width = get_terminal_width();
    let box_width_without_margins = layout.total_width - options.margin.horizontal();

    // Calculate positioning based on float mode
    let left_spacing = match options.float {
        Float::Left => {
            // Left float: use the specified left margin
            options.margin.left
        }
        Float::Center => {
            // Center float: center the box within terminal width
            let available_space = terminal_width.saturating_sub(layout.total_width);
            available_space / 2 + options.margin.left
        }
        Float::Right => {
            // Right float: align to right edge minus right margin
            if terminal_width > box_width_without_margins + options.margin.right {
                terminal_width - box_width_without_margins - options.margin.right
            } else {
                options.margin.left // Fallback to left margin if not enough space
            }
        }
    };

    // Add calculated left spacing
    for _ in 0..left_spacing {
        result.push(' ');
    }

    // Add the line content
    result.push_str(line);

    // For right margin, we only add it for left float mode
    // For center and right float, the positioning handles the spacing
    if matches!(options.float, Float::Left) {
        for _ in 0..options.margin.right {
            result.push(' ');
        }
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
        let options = BoxenOptions::default();
        let result = render_bottom_border(&border_chars, 10, &options).unwrap();

        assert!(result.contains("└"));
        assert!(result.contains("┘"));
        // Note: We can't test exact equality due to potential ANSI codes
        // but we can verify the border characters are present
    }

    #[test]
    fn test_add_line_with_float_positioning() {
        use crate::options::Float;

        let mut result = String::new();
        let options = BoxenOptions {
            float: Float::Left,
            margin: Spacing {
                top: 0,
                right: 2,
                bottom: 0,
                left: 3,
            },
            ..Default::default()
        };
        let layout = crate::options::LayoutDimensions {
            content_width: 4,
            content_height: 1,
            total_width: 10,
            total_height: 3,
            inner_width: 4,
            inner_height: 1,
        };

        add_line_with_float_positioning(&mut result, "test", &options, &layout);

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
        assert_eq!(result.lines().count(), 5); // 1 top margin + 1 top border + 1 content + 1 bottom border + 1 bottom margin
    }

    #[test]
    fn test_fullscreen_mode_auto() {
        let options = BoxenOptions {
            fullscreen: Some(crate::options::FullscreenMode::Auto),
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Should have borders
        assert!(result.contains("┌"));
        assert!(result.contains("┐"));
        assert!(result.contains("└"));
        assert!(result.contains("┘"));

        // Should fill the terminal dimensions
        let terminal_width = crate::terminal::get_terminal_width();
        let terminal_height = crate::terminal::get_terminal_height().unwrap_or(24);

        // Should have terminal height number of lines
        assert_eq!(result.lines().count(), terminal_height);

        // Each line should be the terminal width
        for line in result.lines() {
            assert_eq!(text_width(line), terminal_width);
        }
    }

    #[test]
    fn test_fullscreen_mode_with_margins() {
        let options = BoxenOptions {
            fullscreen: Some(crate::options::FullscreenMode::Auto),
            margin: Spacing {
                top: 1,
                right: 2,
                bottom: 1,
                left: 2,
            },
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        let terminal_width = crate::terminal::get_terminal_width();
        let terminal_height = crate::terminal::get_terminal_height().unwrap_or(24);

        // Should fill terminal height
        assert_eq!(result.lines().count(), terminal_height);

        let lines: Vec<&str> = result.lines().collect();

        // First and last lines should be empty (margins)
        assert_eq!(lines[0], "");
        assert_eq!(lines[terminal_height - 1], "");

        // Box lines should have left margin (2 spaces) and total terminal width
        for line in lines.iter().take(terminal_height - 1).skip(1) {
            assert!(line.starts_with("  "));
            assert_eq!(text_width(line), terminal_width);
        }
    }

    #[test]
    fn test_fullscreen_mode_with_padding() {
        let options = BoxenOptions {
            fullscreen: Some(crate::options::FullscreenMode::Auto),
            padding: Spacing {
                top: 1,
                right: 1,
                bottom: 1,
                left: 1,
            },
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        let terminal_width = crate::terminal::get_terminal_width();
        let terminal_height = crate::terminal::get_terminal_height().unwrap_or(24);

        // Should fill terminal height
        assert_eq!(result.lines().count(), terminal_height);

        // Should contain the text
        assert!(result.contains("Hello"));

        // Each line should be the terminal width
        for line in result.lines() {
            assert_eq!(text_width(line), terminal_width);
        }
    }

    #[test]
    fn test_fullscreen_mode_custom() {
        let custom_func =
            |width: usize, height: usize| -> (usize, usize) { (width * 3 / 4, height * 3 / 4) };

        let options = BoxenOptions {
            fullscreen: Some(crate::options::FullscreenMode::Custom(custom_func)),
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Should have borders
        assert!(result.contains("┌"));
        assert!(result.contains("┐"));

        // Each line should be 3/4 the terminal width
        let terminal_width = crate::terminal::get_terminal_width();
        let terminal_height = crate::terminal::get_terminal_height().unwrap_or(24);
        let expected_width = terminal_width * 3 / 4;
        let expected_height = terminal_height * 3 / 4;

        assert_eq!(result.lines().count(), expected_height);
        for line in result.lines() {
            assert_eq!(text_width(line), expected_width);
        }
    }

    #[test]
    fn test_fullscreen_mode_with_title() {
        let options = BoxenOptions {
            fullscreen: Some(crate::options::FullscreenMode::Auto),
            title: Some("Test Title".to_string()),
            title_alignment: crate::options::TitleAlignment::Center,
            ..Default::default()
        };

        let result = boxen("Content", Some(options)).unwrap();

        // Should contain both title and content
        assert!(result.contains("Test Title"));
        assert!(result.contains("Content"));

        // Title should be in the top border
        let lines: Vec<&str> = result.lines().collect();
        let top_border = lines[0];
        assert!(top_border.contains("Test Title"));

        // Each line should be the terminal width
        let terminal_width = crate::terminal::get_terminal_width();
        for line in result.lines() {
            assert_eq!(text_width(line), terminal_width);
        }
    }

    #[test]
    fn test_fullscreen_mode_multiline_content() {
        let options = BoxenOptions {
            fullscreen: Some(crate::options::FullscreenMode::Auto),
            text_alignment: crate::options::TextAlignment::Center,
            ..Default::default()
        };

        let result = boxen("Line 1\nLine 2\nLine 3", Some(options)).unwrap();

        // Should contain all lines
        assert!(result.contains("Line 1"));
        assert!(result.contains("Line 2"));
        assert!(result.contains("Line 3"));

        let terminal_width = crate::terminal::get_terminal_width();
        let terminal_height = crate::terminal::get_terminal_height().unwrap_or(24);

        // Should fill terminal height
        assert_eq!(result.lines().count(), terminal_height);

        // Each line should be the terminal width
        for line in result.lines() {
            assert_eq!(text_width(line), terminal_width);
        }
    }

    #[test]
    fn test_fullscreen_mode_overrides_width() {
        let options = BoxenOptions {
            fullscreen: Some(crate::options::FullscreenMode::Auto),
            width: Some(50), // Should be ignored
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        let terminal_width = crate::terminal::get_terminal_width();
        let terminal_height = crate::terminal::get_terminal_height().unwrap_or(24);

        // Should use terminal dimensions, not specified width
        assert_eq!(result.lines().count(), terminal_height);
        for line in result.lines() {
            assert_eq!(text_width(line), terminal_width);
        }
    }

    #[test]
    fn test_fullscreen_mode_no_border() {
        let options = BoxenOptions {
            fullscreen: Some(crate::options::FullscreenMode::Auto),
            border_style: BorderStyle::None,
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Should not contain border characters
        assert!(!result.contains("┌"));
        assert!(!result.contains("│"));

        let terminal_width = crate::terminal::get_terminal_width();
        let terminal_height = crate::terminal::get_terminal_height().unwrap_or(24);

        // Should fill terminal height
        assert_eq!(result.lines().count(), terminal_height);

        // Each line should be the terminal width
        for line in result.lines() {
            assert_eq!(text_width(line), terminal_width);
        }
    }

    #[test]
    fn test_fullscreen_mode_with_height_constraint() {
        let terminal_height = crate::terminal::get_terminal_height().unwrap_or(24);

        let options = BoxenOptions {
            fullscreen: Some(crate::options::FullscreenMode::Auto),
            padding: Spacing::from(1), // 6 horizontal, 2 vertical
            ..Default::default()
        };

        let result = boxen("Small content", Some(options)).unwrap();

        // Should expand to fill terminal height
        assert_eq!(result.lines().count(), terminal_height);

        // Each line should be the terminal width
        let terminal_width = crate::terminal::get_terminal_width();
        for line in result.lines() {
            assert_eq!(text_width(line), terminal_width);
        }
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
        assert!(title_line.starts_with("This is a very"));
    }

    #[test]
    fn test_height_constraint_padding() {
        let options = BoxenOptions {
            height: Some(10), // Specify height larger than content
            ..Default::default()
        };

        let result = boxen("Hello\nWorld", Some(options)).unwrap();

        // Should be exactly 10 lines total (including borders)
        assert_eq!(result.lines().count(), 10);

        // Should contain the content
        assert!(result.contains("Hello"));
        assert!(result.contains("World"));

        // Should have borders
        assert!(result.contains("┌"));
        assert!(result.contains("└"));
    }

    #[test]
    fn test_height_constraint_truncation() {
        let long_text = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8";
        let options = BoxenOptions {
            height: Some(5), // Small height to force truncation
            ..Default::default()
        };

        let result = boxen(long_text, Some(options)).unwrap();

        // Should be exactly 5 lines total
        assert_eq!(result.lines().count(), 5);

        // Should contain some of the content (truncated)
        assert!(result.contains("Line 1"));
        assert!(result.contains("Line 2"));

        // Should not contain later lines due to truncation
        assert!(!result.contains("Line 7"));
        assert!(!result.contains("Line 8"));

        // Should still have borders
        assert!(result.contains("┌"));
        assert!(result.contains("└"));
    }

    #[test]
    fn test_height_constraint_exact_fit() {
        let options = BoxenOptions {
            height: Some(3), // Exact fit for single line + borders
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should be exactly 3 lines (top border, content, bottom border)
        assert_eq!(result.lines().count(), 3);

        // Should contain the content
        assert!(result.contains("Hello"));

        // Should have borders
        assert!(result.contains("┌"));
        assert!(result.contains("└"));
    }

    #[test]
    fn test_height_constraint_with_padding() {
        let options = BoxenOptions {
            height: Some(8),
            padding: Spacing {
                top: 1,
                right: 1,
                bottom: 1,
                left: 1,
            },
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should be exactly 8 lines total
        assert_eq!(result.lines().count(), 8);

        // Should contain the content
        assert!(result.contains("Hello"));

        // Should have proper padding structure
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines[0].contains("┌")); // Top border
        assert!(lines[7].contains("└")); // Bottom border
    }

    #[test]
    fn test_height_constraint_with_margins() {
        let options = BoxenOptions {
            height: Some(10), // Total height including margins (need more space)
            margin: Spacing {
                top: 1,
                right: 0,
                bottom: 1,
                left: 0,
            },
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should be 10 lines total (1 top margin + 8 box + 1 bottom margin)
        assert_eq!(result.lines().count(), 10);

        // Should contain the content
        assert!(result.contains("Hello"));

        // First and last lines should be empty (margins)
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines[0], ""); // Top margin
        assert_eq!(lines[9], ""); // Bottom margin
    }

    #[test]
    fn test_height_constraint_no_border() {
        let options = BoxenOptions {
            border_style: BorderStyle::None,
            height: Some(4), // Height constraint without borders
            ..Default::default()
        };

        let result = boxen("Line 1\nLine 2\nLine 3\nLine 4\nLine 5", Some(options)).unwrap();

        // Should be exactly 4 lines (no borders)
        assert_eq!(result.lines().count(), 4);

        // Should contain truncated content
        assert!(result.contains("Line 1"));
        assert!(result.contains("Line 4"));
        assert!(!result.contains("Line 5")); // Truncated

        // Should not have border characters
        assert!(!result.contains("┌"));
        assert!(!result.contains("└"));
    }

    #[test]
    fn test_height_constraint_minimum_height() {
        let options = BoxenOptions {
            height: Some(2), // Very small height
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should be exactly 2 lines (just borders, no content space)
        assert_eq!(result.lines().count(), 2);

        // Should have borders but no visible content due to space constraints
        assert!(result.contains("┌"));
        assert!(result.contains("└"));
    }

    #[test]
    fn test_height_constraint_with_title() {
        let options = BoxenOptions {
            title: Some("Title".to_string()),
            height: Some(5),
            ..Default::default()
        };

        let result = boxen("Content\nMore content\nEven more", Some(options)).unwrap();

        // Should be exactly 5 lines
        assert_eq!(result.lines().count(), 5);

        // Should contain title and some content
        assert!(result.contains("Title"));
        assert!(result.contains("Content"));

        // Title should be in the top border
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines[0].contains("Title"));
    }

    #[test]
    fn test_height_constraint_error_handling() {
        // Test with height too small for borders and padding
        let options = BoxenOptions {
            height: Some(1), // Too small for borders
            padding: Spacing::from(1),
            ..Default::default()
        };

        let result = boxen("Hello", Some(options));

        // Should return an error due to invalid dimensions
        assert!(result.is_err());
    }

    #[test]
    fn test_process_content_with_height_constraints() {
        let options = BoxenOptions {
            height: Some(8), // Total height including borders
            padding: Spacing {
                top: 1,
                right: 1,
                bottom: 1,
                left: 1,
            },
            ..Default::default()
        };

        let content = process_content("Line 1\nLine 2\nLine 3\nLine 4\nLine 5", &options).unwrap();

        // Content height should be constrained by available space
        // Total height: 8, borders: 2, padding: 2 vertical = 4 available for content
        assert_eq!(content.content_height, 4);
        assert_eq!(content.lines.len(), 4);

        // Should contain first 4 lines
        assert_eq!(content.lines[0], "Line 1");
        assert_eq!(content.lines[1], "Line 2");
        assert_eq!(content.lines[2], "Line 3");
        assert_eq!(content.lines[3], "Line 4");
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

        // Should still render properly with empty title
        assert!(result.contains("Content"));
        assert!(result.contains("┌"));
        assert!(result.contains("┐"));
    }

    #[test]
    fn test_float_left_positioning() {
        use crate::options::Float;

        let options = BoxenOptions {
            float: Float::Left,
            margin: Spacing {
                top: 0,
                right: 0,
                bottom: 0,
                left: 5, // 5 spaces from left
            },
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Each line should start with 5 spaces (left margin)
        for line in result.lines() {
            if !line.trim().is_empty() {
                assert!(line.starts_with("     ")); // 5 spaces
            }
        }
    }

    #[test]
    fn test_float_center_positioning() {
        use crate::options::Float;
        use crate::terminal::get_terminal_width;

        let options = BoxenOptions {
            float: Float::Center,
            width: Some(20), // Fixed width for predictable centering
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        let terminal_width = get_terminal_width();
        let expected_left_spacing = (terminal_width - 20) / 2;

        // Each line should be centered
        for line in result.lines() {
            if !line.trim().is_empty() {
                // Count leading spaces
                let leading_spaces = line.len() - line.trim_start().len();
                // Should be approximately centered (within 1 space due to rounding)
                assert!(
                    leading_spaces >= expected_left_spacing.saturating_sub(1)
                        && leading_spaces <= expected_left_spacing + 1,
                    "Expected ~{} leading spaces, got {}",
                    expected_left_spacing,
                    leading_spaces
                );
            }
        }
    }

    #[test]
    fn test_float_right_positioning() {
        use crate::options::Float;
        use crate::terminal::get_terminal_width;

        let options = BoxenOptions {
            float: Float::Right,
            width: Some(20), // Fixed width for predictable positioning
            margin: Spacing {
                top: 0,
                right: 3, // 3 spaces from right
                bottom: 0,
                left: 0,
            },
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Each line should be right-aligned - just verify it's positioned significantly to the right
        for line in result.lines() {
            if !line.trim().is_empty() {
                let leading_spaces = line.len() - line.trim_start().len();
                // Should be positioned significantly to the right (more than center)
                let terminal_width = get_terminal_width();
                let center_position = terminal_width / 2;
                assert!(
                    leading_spaces > center_position,
                    "Right float should position box to the right of center. Got {} leading spaces, center is at {}",
                    leading_spaces,
                    center_position
                );
            }
        }
    }

    #[test]
    fn test_float_positioning_with_different_terminal_widths() {
        use crate::options::Float;

        // Test that float positioning adapts to terminal width constraints
        let options = BoxenOptions {
            float: Float::Center,
            width: Some(10),
            ..Default::default()
        };

        let result = boxen("Test", Some(options)).unwrap();

        // Should render successfully regardless of terminal width
        assert!(result.contains("Test"));
        assert_eq!(result.lines().count(), 3); // top border, content, bottom border

        // Each content line should be exactly 10 characters wide
        for line in result.lines() {
            if line.contains("Test") || line.contains("─") {
                // Find the actual box content (excluding leading spaces)
                let trimmed = line.trim_start();
                if !trimmed.is_empty() {
                    assert_eq!(crate::text::text_width(trimmed), 10);
                }
            }
        }
    }

    #[test]
    fn test_float_positioning_with_no_border() {
        use crate::options::Float;

        let options = BoxenOptions {
            float: Float::Center,
            border_style: BorderStyle::None,
            width: Some(15),
            ..Default::default()
        };

        let result = boxen("No Border", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("No Border"));

        // Should not contain border characters
        assert!(!result.contains("┌"));
        assert!(!result.contains("│"));

        // Should still be positioned according to float
        assert_eq!(result.lines().count(), 1); // Just content line
    }

    #[test]
    fn test_float_positioning_with_padding_and_margins() {
        use crate::options::Float;

        let options = BoxenOptions {
            float: Float::Left,
            padding: Spacing {
                top: 1,
                right: 2,
                bottom: 1,
                left: 2,
            },
            margin: Spacing {
                top: 1,
                right: 1,
                bottom: 1,
                left: 3, // 3 spaces from left
            },
            ..Default::default()
        };

        let result = boxen("Padded", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Padded"));

        // Should have correct number of lines (top margin, top border, top padding, content, bottom padding, bottom border, bottom margin)
        assert_eq!(result.lines().count(), 7);

        // Border and content lines should start with 3 spaces (left margin)
        let lines: Vec<&str> = result.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if i != 0 && i != lines.len() - 1 && !line.trim().is_empty() {
                // Skip first (top margin) and last (bottom margin) lines
                assert!(line.starts_with("   ")); // 3 spaces
            }
        }
    }

    #[test]
    fn test_float_positioning_edge_cases() {
        use crate::options::Float;

        // Test with very small terminal width simulation
        let options = BoxenOptions {
            float: Float::Right,
            width: Some(50), // Larger than typical small terminal
            ..Default::default()
        };

        let result = boxen("Edge Case", Some(options));

        // Should handle gracefully even if positioning is challenging
        assert!(result.is_ok());
        let box_str = result.unwrap();
        assert!(box_str.contains("Edge Case"));
    }

    #[test]
    fn test_float_positioning_with_multiline_content() {
        use crate::options::Float;

        let options = BoxenOptions {
            float: Float::Center,
            width: Some(25),
            ..Default::default()
        };

        let result = boxen("Line 1\nLine 2\nLine 3", Some(options)).unwrap();

        // Should contain all lines
        assert!(result.contains("Line 1"));
        assert!(result.contains("Line 2"));
        assert!(result.contains("Line 3"));

        // All content lines should be consistently positioned
        let lines: Vec<&str> = result.lines().collect();
        let mut content_line_positions = Vec::new();

        for line in &lines {
            if line.contains("Line") || line.contains("─") {
                let leading_spaces = line.len() - line.trim_start().len();
                content_line_positions.push(leading_spaces);
            }
        }

        // All content lines should have the same positioning
        if content_line_positions.len() > 1 {
            let first_pos = content_line_positions[0];
            for pos in &content_line_positions {
                assert_eq!(
                    *pos, first_pos,
                    "All content lines should be aligned consistently"
                );
            }
        }
    }

    #[test]
    fn test_float_positioning_with_title() {
        use crate::options::Float;

        let options = BoxenOptions {
            float: Float::Right,
            title: Some("My Title".to_string()),
            width: Some(20),
            margin: Spacing {
                top: 0,
                right: 2,
                bottom: 0,
                left: 0,
            },
            ..Default::default()
        };

        let result = boxen("Content", Some(options)).unwrap();

        // Should contain both title and content
        assert!(result.contains("My Title"));
        assert!(result.contains("Content"));

        // Title should be in the top border and positioned consistently with content
        let lines: Vec<&str> = result.lines().collect();
        let top_border = lines[0];
        let content_line = lines[1];

        // Both should have the same leading space positioning
        let top_leading = top_border.len() - top_border.trim_start().len();
        let content_leading = content_line.len() - content_line.trim_start().len();
        assert_eq!(
            top_leading, content_leading,
            "Title and content should be aligned"
        );
    }

    #[test]
    fn test_border_color_integration() {
        use crate::options::Color;

        let options = BoxenOptions {
            border_color: Some(Color::Named("red".to_string())),
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Should contain ANSI color codes (we can't test exact codes due to colored crate implementation)
        // but we can verify the result is longer than uncolored version
        let uncolored_result = boxen("Hello", None).unwrap();
        assert!(result.len() > uncolored_result.len());
    }

    #[test]
    fn test_background_color_integration() {
        use crate::options::Color;

        let options = BoxenOptions {
            background_color: Some(Color::Named("blue".to_string())),
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Should contain ANSI color codes
        let uncolored_result = boxen("Hello", None).unwrap();
        assert!(result.len() > uncolored_result.len());
    }

    #[test]
    fn test_dim_border_integration() {
        let options = BoxenOptions {
            dim_border: true,
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Should contain ANSI dim codes
        let normal_result = boxen("Hello", None).unwrap();
        assert!(result.len() > normal_result.len());
    }

    #[test]
    fn test_border_and_background_color_combination() {
        use crate::options::Color;

        let options = BoxenOptions {
            border_color: Some(Color::Named("red".to_string())),
            background_color: Some(Color::Named("blue".to_string())),
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Should be significantly longer due to multiple color codes
        let uncolored_result = boxen("Hello", None).unwrap();
        assert!(result.len() > uncolored_result.len() + 20); // Should have at least 20 extra characters from ANSI codes
    }

    #[test]
    fn test_color_with_padding() {
        use crate::options::Color;

        let options = BoxenOptions {
            background_color: Some(Color::Named("green".to_string())),
            padding: Spacing::from(1),
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Should have multiple lines due to padding
        assert!(result.lines().count() > 3);
    }

    #[test]
    fn test_color_with_no_border() {
        use crate::options::Color;

        let options = BoxenOptions {
            border_style: BorderStyle::None,
            background_color: Some(Color::Named("yellow".to_string())),
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Should not contain border characters
        assert!(!result.contains("┌"));
        assert!(!result.contains("│"));

        // Should still have color codes
        let uncolored_result = boxen(
            "Hello",
            Some(BoxenOptions {
                border_style: BorderStyle::None,
                ..Default::default()
            }),
        )
        .unwrap();
        assert!(result.len() > uncolored_result.len());
    }

    #[test]
    fn test_hex_color_integration() {
        use crate::options::Color;

        let options = BoxenOptions {
            border_color: Some(Color::Hex("#FF0000".to_string())),
            background_color: Some(Color::Hex("#00FF00".to_string())),
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Should contain color codes
        let uncolored_result = boxen("Hello", None).unwrap();
        assert!(result.len() > uncolored_result.len());
    }

    #[test]
    fn test_rgb_color_integration() {
        use crate::options::Color;

        let options = BoxenOptions {
            border_color: Some(Color::Rgb(255, 0, 0)),
            background_color: Some(Color::Rgb(0, 255, 0)),
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Should contain color codes
        let uncolored_result = boxen("Hello", None).unwrap();
        assert!(result.len() > uncolored_result.len());
    }

    #[test]
    fn test_invalid_color_error_handling() {
        use crate::options::Color;

        let options = BoxenOptions {
            border_color: Some(Color::Named("invalid_color".to_string())),
            ..Default::default()
        };

        let result = boxen("Hello", Some(options));
        assert!(result.is_err());

        // Should be a color-related error
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.to_lowercase().contains("color"));
    }

    #[test]
    fn test_color_with_multiline_content() {
        use crate::options::Color;

        let options = BoxenOptions {
            border_color: Some(Color::Named("cyan".to_string())),
            background_color: Some(Color::Named("magenta".to_string())),
            ..Default::default()
        };

        let result = boxen("Line 1\nLine 2\nLine 3", Some(options)).unwrap();

        // Should contain all lines
        assert!(result.contains("Line 1"));
        assert!(result.contains("Line 2"));
        assert!(result.contains("Line 3"));

        // Should have multiple content lines
        assert!(result.lines().count() >= 5); // top border + 3 content + bottom border
    }

    #[test]
    fn test_color_with_title() {
        use crate::options::Color;

        let options = BoxenOptions {
            title: Some("Title".to_string()),
            border_color: Some(Color::Named("blue".to_string())),
            background_color: Some(Color::Named("white".to_string())),
            width: Some(20), // Ensure enough width for the title
            ..Default::default()
        };

        let result = boxen("Content", Some(options)).unwrap();

        // Should contain both title and content
        assert!(result.contains("Title"));
        assert!(result.contains("Content"));

        // Title should be in the top border which should be colored
        let lines: Vec<&str> = result.lines().collect();
        let top_border = lines[0];
        assert!(top_border.contains("Title"));
    }

    #[test]
    fn test_color_with_different_border_styles() {
        use crate::options::Color;

        let border_styles = vec![
            BorderStyle::Single,
            BorderStyle::Double,
            BorderStyle::Round,
            BorderStyle::Bold,
            BorderStyle::Classic,
        ];

        for style in border_styles {
            let options = BoxenOptions {
                border_style: style.clone(),
                border_color: Some(Color::Named("green".to_string())),
                background_color: Some(Color::Named("black".to_string())),
                ..Default::default()
            };

            let result = boxen("Test", Some(options)).unwrap();

            // Should contain the text
            assert!(result.contains("Test"));

            // Should contain color codes
            let uncolored_result = boxen(
                "Test",
                Some(BoxenOptions {
                    border_style: style,
                    ..Default::default()
                }),
            )
            .unwrap();
            assert!(result.len() > uncolored_result.len());
        }
    }

    #[test]
    fn test_dim_border_with_color() {
        use crate::options::Color;

        let options = BoxenOptions {
            border_color: Some(Color::Named("red".to_string())),
            dim_border: true,
            ..Default::default()
        };

        let result = boxen("Hello", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Hello"));

        // Should contain both color and dim codes
        let normal_colored_result = boxen(
            "Hello",
            Some(BoxenOptions {
                border_color: Some(Color::Named("red".to_string())),
                dim_border: false,
                ..Default::default()
            }),
        )
        .unwrap();

        // Dim version should be different from normal colored version
        assert_ne!(result, normal_colored_result);
    }

    #[test]
    fn test_color_with_float_positioning() {
        use crate::options::{Color, Float};

        let options = BoxenOptions {
            float: Float::Center,
            border_color: Some(Color::Named("blue".to_string())),
            background_color: Some(Color::Named("yellow".to_string())),
            width: Some(20),
            ..Default::default()
        };

        let result = boxen("Centered", Some(options)).unwrap();

        // Should contain the text
        assert!(result.contains("Centered"));

        // Should be positioned and colored
        let uncolored_result = boxen(
            "Centered",
            Some(BoxenOptions {
                float: Float::Center,
                width: Some(20),
                ..Default::default()
            }),
        )
        .unwrap();
        assert!(result.len() > uncolored_result.len());
    }
}
