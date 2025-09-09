//! # Configuration System
//!
//! This module provides comprehensive configuration options for customizing the appearance
//! and behavior of terminal boxes. The configuration system is built around flexible,
//! composable options that allow fine-grained control over every aspect of box rendering.
//!
//! ## Overview
//!
//! The configuration system consists of several key components:
//! - **`BoxenOptions`**: Main configuration struct containing all styling options
//! - **`BoxenBuilder`**: Ergonomic builder pattern for constructing configurations
//! - **Style Enums**: Type-safe options for borders, alignment, positioning, and colors
//! - **Spacing Types**: Flexible spacing configuration for padding and margins
//!
//! ## Quick Start
//!
//! ```rust
//! use boxen::{BoxenBuilder, BorderStyle, TextAlignment, Color, TitleAlignment};
//!
//! // Simple box with basic styling
//! let result = BoxenBuilder::new()
//!     .border_style(BorderStyle::Double)
//!     .padding(2)
//!     .text_alignment(TextAlignment::Center)
//!     .render("Hello, World!")
//!     .unwrap();
//!
//! // Advanced box with colors and title
//! let result = BoxenBuilder::new()
//!     .border_style(BorderStyle::Round)
//!     .border_color(Color::Named("blue".to_string()))
//!     .background_color(Color::Named("white".to_string()))
//!     .title("Status Report")
//!     .title_alignment(TitleAlignment::Center)
//!     .width(50)
//!     .margin(1)
//!     .render("System operational")
//!     .unwrap();
//! ```
//!
//! ## Configuration Categories
//!
//! ### Border Configuration
//! - **Style**: Choose from 9 predefined border styles (Single, Double, Rounded, etc.)
//! - **Color**: Optional border coloring with full color palette support
//! - **Dimming**: Reduce border intensity for subtle presentation
//!
//! ### Layout Configuration
//! - **Dimensions**: Fixed width/height or automatic sizing
//! - **Positioning**: Left, center, or right alignment within terminal
//! - **Spacing**: Independent padding and margin control
//! - **Fullscreen**: Optional fullscreen mode with various behaviors
//!
//! ### Content Configuration
//! - **Text Alignment**: Left, center, or right alignment within the box
//! - **Background**: Optional background coloring for content area
//! - **Title**: Optional title with independent alignment control
//!
//! ## Builder Pattern
//!
//! The recommended way to create configurations is through the builder pattern:
//!
//! ```rust
//! use boxen::{BoxenBuilder, BorderStyle, TextAlignment, Float};
//!
//! let config = BoxenBuilder::new()
//!     .border_style(BorderStyle::Bold)
//!     .padding(3)
//!     .margin(1)
//!     .text_alignment(TextAlignment::Center)
//!     .float(Float::Center)
//!     .width(60)
//!     .title("Configuration Example")
//!     .build();
//! ```
//!
//! ## Direct Configuration
//!
//! For advanced use cases, you can construct `BoxenOptions` directly:
//!
//! ```rust
//! use boxen::{BoxenOptions, BorderStyle, TextAlignment, Spacing, Color};
//!
//! let options = BoxenOptions {
//!     border_style: BorderStyle::Double,
//!     padding: Spacing::from((2, 4, 2, 4)), // top, right, bottom, left
//!     margin: Spacing::from(1),
//!     text_alignment: TextAlignment::Center,
//!     border_color: Some(Color::Named("green".to_string())),
//!     title: Some("Direct Config".to_string()),
//!     width: Some(50),
//!     ..Default::default()
//! };
//! ```
//!
//! ## Spacing System
//!
//! The spacing system provides flexible control over padding and margins:
//!
//! ### Uniform Spacing
//! ```rust
//! use boxen::Spacing;
//! let spacing = Spacing::from(2); // 2 units on all sides
//! ```
//!
//! ### Individual Control
//! ```rust
//! use boxen::Spacing;
//! let spacing = Spacing::from((1, 2, 1, 2)); // top, right, bottom, left
//! ```
//!
//! ### Direct Field Access
//! ```rust
//! use boxen::Spacing;
//! let spacing = Spacing {
//!     top: 2,
//!     right: 3,
//!     bottom: 1,
//!     left: 3,
//! };
//! ```
//!
//! ## Color System
//!
//! Colors can be specified in multiple formats:
//!
//! ### Named Colors
//! ```rust
//! use boxen::{BoxenBuilder, Color};
//!
//! let result = BoxenBuilder::new().border_color(Color::Named("red".to_string()));
//! ```
//!
//! ### RGB Values
//! ```rust
//! use boxen::{BoxenBuilder, Color};
//!
//! let result = BoxenBuilder::new().border_color(Color::Rgb(255, 128, 0));
//! ```
//!
//! ### Hex Strings
//! ```rust
//! use boxen::{BoxenBuilder, Color};
//!
//! let result = BoxenBuilder::new().border_color(Color::Hex("#FF8000".to_string()));
//! ```
//!
//! ## Fullscreen Mode
//!
//! Fullscreen mode provides several behaviors for terminal-wide boxes:
//!
//! ```rust
//! use boxen::{BoxenBuilder, FullscreenMode};
//!
//! // Automatically use terminal dimensions
//! let result = BoxenBuilder::new().fullscreen(FullscreenMode::Auto);
//!
//! // Use custom function to calculate dimensions
//! let result = BoxenBuilder::new().fullscreen(FullscreenMode::Custom(|width, height| {
//!     (width - 4, height - 2)
//! }));
//! ```
//!
//! ## Validation and Error Handling
//!
//! All configuration options are validated before rendering:
//!
//! ```rust
//! use boxen::BoxenBuilder;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     match BoxenBuilder::new().width(0).render("Invalid") {
//!         Ok(result) => println!("{}", result),
//!         Err(e) => {
//!             println!("Configuration error: {}", e);
//!             // Error includes helpful suggestions for fixing the issue
//!             for recommendation in e.recommendations() {
//!                 println!("💡 {}: {}", recommendation.issue, recommendation.suggestion);
//!             }
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ## Performance Considerations
//!
//! - Configuration structs are lightweight and can be cloned efficiently
//! - Builder operations are zero-cost until `build()` or `render()` is called
//! - Default values are optimized for common use cases
//! - Validation is performed only when necessary to avoid overhead
//!
//! ## Thread Safety
//!
//! All configuration types are thread-safe and can be shared between threads
//! or used in concurrent rendering operations.

use crate::error::{BoxenError, BoxenResult};
use crate::terminal::{calculate_border_width, get_terminal_height, get_terminal_width};

/// Main configuration struct for boxen styling options.
///
/// `BoxenOptions` contains all the configuration parameters for customizing
/// the appearance and layout of terminal boxes. This struct is typically
/// created using the [`BoxenBuilder`] for a more ergonomic API.
///
/// # Examples
///
/// ## Direct Construction
///
/// ```rust
/// use ::boxen::{BoxenOptions, BorderStyle, TextAlignment, Spacing};
///
/// let options = BoxenOptions {
///     border_style: BorderStyle::Double,
///     padding: Spacing::from(2),
///     text_alignment: TextAlignment::Center,
///     title: Some("My Box".to_string()),
///     width: Some(40),
///     ..Default::default()
/// };
/// ```
///
/// ## Using Builder (Recommended)
///
/// ```rust
/// use ::boxen::{builder, BorderStyle, TextAlignment};
///
/// let result = builder()
///     .border_style(BorderStyle::Double)
///     .padding(2)
///     .text_alignment(TextAlignment::Center)
///     .title("My Box")
///     .width(40)
///     .render("Hello, World!")
///     .unwrap();
/// ```
///
/// # Field Documentation
///
/// - `border_style`: The style of border to draw around the box
/// - `padding`: Internal spacing between the border and content
/// - `margin`: External spacing around the entire box
/// - `text_alignment`: How to align text within the box
/// - `title`: Optional title to display in the top border
/// - `title_alignment`: How to align the title within the top border
/// - `float`: How to position the box within the terminal
/// - `width`: Optional fixed width for the box
/// - `height`: Optional fixed height for the box
/// - `border_color`: Optional color for the border
/// - `background_color`: Optional background color for the content area
/// - `dim_border`: Whether to render the border with reduced intensity
/// - `fullscreen`: Optional fullscreen mode configuration
#[derive(Debug, Clone)]
pub struct BoxenOptions {
    /// The visual style of the border (Single, Double, Rounded, etc.)
    pub border_style: BorderStyle,
    /// Internal spacing between the border and content
    pub padding: Spacing,
    /// External spacing around the entire box
    pub margin: Spacing,
    /// How to align text within the box content area
    pub text_alignment: TextAlignment,
    /// Optional title to display in the top border
    pub title: Option<String>,
    /// How to align the title within the top border
    pub title_alignment: TitleAlignment,
    /// How to position the box within the terminal width
    pub float: Float,
    /// Optional fixed width for the box (overrides automatic sizing)
    pub width: Option<usize>,
    /// Optional fixed height for the box (overrides automatic sizing)
    pub height: Option<usize>,
    /// Optional color for the border characters
    pub border_color: Option<Color>,
    /// Optional background color for the content area
    pub background_color: Option<Color>,
    /// Whether to render the border with reduced intensity
    pub dim_border: bool,
    /// Optional fullscreen mode configuration
    pub fullscreen: Option<FullscreenMode>,
}

impl Default for BoxenOptions {
    fn default() -> Self {
        Self {
            border_style: BorderStyle::Single,
            padding: Spacing::default(),
            margin: Spacing::default(),
            text_alignment: TextAlignment::Left,
            title: None,
            title_alignment: TitleAlignment::Left,
            float: Float::Left,
            width: None,
            height: None,
            border_color: None,
            background_color: None,
            dim_border: false,
            fullscreen: None,
        }
    }
}

/// Border style definition for box rendering.
///
/// Defines the visual style of the border drawn around the box content.
/// Each style uses different Unicode characters to create distinct visual effects.
///
/// # Examples
///
/// ```rust
/// use ::boxen::{builder, BorderStyle};
///
/// // Single line border (default)
/// let single = builder().border_style(BorderStyle::Single);
///
/// // Double line border for emphasis
/// let double = builder().border_style(BorderStyle::Double);
///
/// // Rounded corners for a softer look
/// let round = builder().border_style(BorderStyle::Round);
///
/// // No border for minimal styling
/// let none = builder().border_style(BorderStyle::None);
/// ```
///
/// # Visual Examples
///
/// ## Single
/// ```text
/// ┌─────┐
/// │Hello│
/// └─────┘
/// ```
///
/// ## Double
/// ```text
/// ╔═════╗
/// ║Hello║
/// ╚═════╝
/// ```
///
/// ## Round
/// ```text
/// ╭─────╮
/// │Hello│
/// ╰─────╯
/// ```
///
/// ## Bold
/// ```text
/// ┏━━━━━┓
/// ┃Hello┃
/// ┗━━━━━┛
/// ```
#[derive(Debug, Clone)]
pub enum BorderStyle {
    /// No border - content only
    None,
    /// Single line border (default)
    Single,
    /// Double line border
    Double,
    /// Rounded corners
    Round,
    /// Bold/thick lines
    Bold,
    /// Single horizontal, double vertical
    SingleDouble,
    /// Double horizontal, single vertical
    DoubleSingle,
    /// Classic ASCII-style border using +, -, |
    Classic,
    /// Custom border using specified characters
    Custom(BorderChars),
}

/// Border character set for custom borders
#[derive(Debug, Clone)]
pub struct BorderChars {
    /// Character for the top-left corner of the border
    pub top_left: char,
    /// Character for the top-right corner of the border
    pub top_right: char,
    /// Character for the bottom-left corner of the border
    pub bottom_left: char,
    /// Character for the bottom-right corner of the border
    pub bottom_right: char,
    /// Character for the left vertical edge of the border
    pub left: char,
    /// Character for the right vertical edge of the border
    pub right: char,
    /// Character for the top horizontal edge of the border
    pub top: char,
    /// Character for the bottom horizontal edge of the border
    pub bottom: char,
}

/// Spacing configuration for padding and margins.
///
/// Represents spacing values for all four sides of a box. Used for both
/// padding (internal spacing) and margins (external spacing).
///
/// # Examples
///
/// ## Direct Construction
///
/// ```rust
/// use ::boxen::Spacing;
///
/// let spacing = Spacing {
///     top: 1,
///     right: 2,
///     bottom: 1,
///     left: 2,
/// };
/// ```
///
/// ## Using From Implementations
///
/// ```rust
/// use ::boxen::Spacing;
///
/// // Asymmetric spacing (TypeScript-compatible)
/// let asymmetric = Spacing::from(2);  // top: 2, right: 6, bottom: 2, left: 6
///
/// // Explicit values
/// let explicit = Spacing::from((1, 2, 3, 4));  // top, right, bottom, left
///
/// // Horizontal and vertical
/// let symmetric = Spacing::from((3, 1));  // horizontal: 3, vertical: 1
///
/// // Array syntax
/// let array = Spacing::from([2, 4]);  // [horizontal, vertical]
/// let full_array = Spacing::from([1, 2, 3, 4]);  // [top, right, bottom, left]
/// ```
///
/// # TypeScript Compatibility
///
/// When created from a single `usize` value, this struct follows the TypeScript
/// boxen behavior of creating asymmetric spacing with 3x horizontal padding
/// to account for typical terminal character aspect ratios.
#[derive(Debug, Default, Clone, Copy)]
pub struct Spacing {
    /// Top spacing
    pub top: usize,
    /// Right spacing
    pub right: usize,
    /// Bottom spacing
    pub bottom: usize,
    /// Left spacing
    pub left: usize,
}

impl From<usize> for Spacing {
    /// Creates asymmetric spacing (3x horizontal, 1x vertical) to match TypeScript behavior
    fn from(value: usize) -> Self {
        Self {
            top: value,
            right: value * 3,
            bottom: value,
            left: value * 3,
        }
    }
}

impl From<(usize, usize, usize, usize)> for Spacing {
    /// Creates spacing from (top, right, bottom, left) tuple
    fn from((top, right, bottom, left): (usize, usize, usize, usize)) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

/// Text alignment within the box
#[derive(Debug, Clone)]
pub enum TextAlignment {
    /// Align text to the left side of the box
    Left,
    /// Center text within the box
    Center,
    /// Align text to the right side of the box
    Right,
}

/// Title alignment within the top border
#[derive(Debug, Clone)]
pub enum TitleAlignment {
    /// Align title to the left side of the top border
    Left,
    /// Center title within the top border
    Center,
    /// Align title to the right side of the top border
    Right,
}

/// Box positioning relative to terminal
#[derive(Debug, Clone)]
pub enum Float {
    /// Position box on the left side of the terminal
    Left,
    /// Center box horizontally in the terminal
    Center,
    /// Position box on the right side of the terminal
    Right,
}

/// Color specification for borders and backgrounds
#[derive(Debug, Clone)]
pub enum Color {
    /// Named color (e.g., "red", "blue", "green")
    Named(String),
    /// Hexadecimal color code (e.g., "#FF0000", "#00FF00")
    Hex(String),
    /// RGB color values (red, green, blue components 0-255)
    Rgb(u8, u8, u8),
}

/// Fullscreen mode configuration
#[derive(Debug, Clone)]
pub enum FullscreenMode {
    /// Automatically use terminal dimensions
    Auto,
    /// Use custom function to calculate dimensions from terminal size
    Custom(fn(usize, usize) -> (usize, usize)),
}

/// Builder pattern for creating BoxenOptions with a fluent interface.
///
/// The `BoxenBuilder` provides a convenient and type-safe way to configure box styling
/// options using method chaining. This is the recommended approach for creating boxes
/// with custom styling.
///
/// # Examples
///
/// ## Basic Builder Usage
///
/// ```rust
/// use ::boxen::{builder, BorderStyle, TextAlignment};
///
/// let result = builder()
///     .border_style(BorderStyle::Double)
///     .padding(2)
///     .text_alignment(TextAlignment::Center)
///     .render("Hello, World!")
///     .unwrap();
/// ```
///
/// ## Advanced Configuration
///
/// ```rust
/// use ::boxen::{builder, BorderStyle, TextAlignment, TitleAlignment, Float, Color};
///
/// let result = builder()
///     .border_style(BorderStyle::Round)
///     .padding((2, 4, 2, 4))  // top, right, bottom, left
///     .margin(1)
///     .text_alignment(TextAlignment::Center)
///     .title("Status Report")
///     .title_alignment(TitleAlignment::Center)
///     .float(Float::Center)
///     .width(50)
///     .height(10)
///     .border_color("green")
///     .background_color("#f0f0f0")
///     .dim_border(false)
///     .render("All systems operational")
///     .unwrap();
/// ```
///
/// ## Convenience Methods
///
/// ```rust
/// use ::boxen::builder;
///
/// let result = builder()
///     .spacing(1)              // Sets both padding and margin
///     .colors("red", "white")  // Sets border and background colors
///     .size(40, 8)            // Sets width and height
///     .center_all()           // Centers text, title, and float
///     .title("Centered Box")
///     .render("This box is centered in every way")
///     .unwrap();
/// ```
///
/// # Validation and Error Handling
///
/// The builder validates configuration when `render()` is called, not during method chaining.
/// This allows for efficient configuration building without intermediate validations.
///
/// ```rust
/// use ::boxen::builder;
///
/// let result = builder()
///     .width(10)
///     .padding(20)  // This will cause an error - padding too large for width
///     .render("Test");
///
/// match result {
///     Ok(output) => println!("{}", output),
///     Err(e) => println!("Configuration error: {}", e),
/// }
/// ```
///
/// # Performance
///
/// - Method chaining is zero-cost - no allocations until `render()` is called
/// - Configuration validation is performed once at render time
/// - The builder can be reused by calling `render()` multiple times with different text
///
/// # Auto-Adjustment
///
/// The builder provides methods for automatic configuration adjustment:
///
/// ```rust
/// use ::boxen::builder;
///
/// // Automatically adjust configuration to fix common issues
/// let result = builder()
///     .width(5)     // Too small
///     .padding(10)  // Too large
///     .auto_adjust("Hello, World!")  // Fixes the configuration
///     .render("Hello, World!")
///     .unwrap();
///
/// // Or render with automatic adjustment if needed
/// let result = builder()
///     .width(5)
///     .padding(10)
///     .render_or_adjust("Hello, World!")  // Tries render, auto-adjusts if it fails
///     .unwrap();
/// ```
pub struct BoxenBuilder {
    options: BoxenOptions,
}

impl BoxenBuilder {
    /// Create a new builder with default options.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ::boxen::BoxenBuilder;
    ///
    /// let builder = BoxenBuilder::new();
    /// let result = builder.render("Hello").unwrap();
    /// ```
    pub fn new() -> Self {
        Self {
            options: BoxenOptions::default(),
        }
    }

    /// Set the border style for the box.
    ///
    /// # Arguments
    ///
    /// * `style` - The border style to use (Single, Double, Round, Bold, etc.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ::boxen::{builder, BorderStyle};
    ///
    /// let result = builder()
    ///     .border_style(BorderStyle::Double)
    ///     .render("Double border")
    ///     .unwrap();
    /// ```
    pub fn border_style(mut self, style: BorderStyle) -> Self {
        self.options.border_style = style;
        self
    }

    /// Set padding around the text content.
    ///
    /// Padding is the space between the text and the border. Accepts various formats:
    /// - `usize`: Creates asymmetric padding (3x horizontal, 1x vertical) to match TypeScript behavior
    /// - `(usize, usize, usize, usize)`: Explicit (top, right, bottom, left) values
    /// - `(usize, usize)`: Horizontal and vertical values
    /// - `Spacing`: Direct spacing struct
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ::boxen::{builder, Spacing};
    ///
    /// // Asymmetric padding (TypeScript-compatible)
    /// let result1 = builder().padding(1).render("Text").unwrap();
    ///
    /// // Explicit padding values
    /// let result2 = builder().padding((1, 2, 1, 2)).render("Text").unwrap();
    ///
    /// // Horizontal and vertical
    /// let result3 = builder().padding((3, 1)).render("Text").unwrap();
    /// ```
    pub fn padding<T: Into<Spacing>>(mut self, padding: T) -> Self {
        self.options.padding = padding.into();
        self
    }

    /// Set margin around the entire box.
    ///
    /// Margin is the space outside the border. Accepts the same formats as padding.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ::boxen::builder;
    ///
    /// let result = builder()
    ///     .margin(2)
    ///     .render("Text with margin")
    ///     .unwrap();
    /// ```
    pub fn margin<T: Into<Spacing>>(mut self, margin: T) -> Self {
        self.options.margin = margin.into();
        self
    }

    /// Set text alignment
    pub fn text_alignment(mut self, alignment: TextAlignment) -> Self {
        self.options.text_alignment = alignment;
        self
    }

    /// Set title text
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.options.title = Some(title.into());
        self
    }

    /// Set title alignment
    pub fn title_alignment(mut self, alignment: TitleAlignment) -> Self {
        self.options.title_alignment = alignment;
        self
    }

    /// Set float positioning
    pub fn float(mut self, float: Float) -> Self {
        self.options.float = float;
        self
    }

    /// Set box width
    pub fn width(mut self, width: usize) -> Self {
        self.options.width = Some(width);
        self
    }

    /// Set box height
    pub fn height(mut self, height: usize) -> Self {
        self.options.height = Some(height);
        self
    }

    /// Set border color
    pub fn border_color<C: Into<Color>>(mut self, color: C) -> Self {
        self.options.border_color = Some(color.into());
        self
    }

    /// Set background color
    pub fn background_color<C: Into<Color>>(mut self, color: C) -> Self {
        self.options.background_color = Some(color.into());
        self
    }

    /// Enable dim border
    pub fn dim_border(mut self, dim: bool) -> Self {
        self.options.dim_border = dim;
        self
    }

    /// Set fullscreen mode
    pub fn fullscreen(mut self, mode: FullscreenMode) -> Self {
        self.options.fullscreen = Some(mode);
        self
    }

    /// Build the final options
    pub fn build(self) -> BoxenOptions {
        self.options
    }

    /// Build and render box with the given text.
    ///
    /// This is the final method in the builder chain that validates the configuration
    /// and renders the box. All validation is performed at this stage.
    ///
    /// # Arguments
    ///
    /// * `text` - The text content to render in the box
    ///
    /// # Returns
    ///
    /// Returns `Result<String, BoxenError>` with the rendered box or detailed error information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ::boxen::{builder, BorderStyle};
    ///
    /// let result = builder()
    ///     .border_style(BorderStyle::Round)
    ///     .padding(1)
    ///     .render("Hello, World!")
    ///     .unwrap();
    ///
    /// println!("{}", result);
    /// ```
    ///
    /// # Error Handling
    ///
    /// ```rust
    /// use ::boxen::builder;
    ///
    /// let result = builder()
    ///     .width(5)     // Too small
    ///     .padding(10)  // Too large
    ///     .render("Hello");
    ///
    /// match result {
    ///     Ok(output) => println!("{}", output),
    ///     Err(e) => {
    ///         println!("Error: {}", e);
    ///         for rec in e.recommendations() {
    ///             println!("Try: {}", rec.suggestion);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn render<S: AsRef<str>>(self, text: S) -> BoxenResult<String> {
        let text_ref = text.as_ref();

        // Comprehensive input validation
        crate::error::validation::validate_all_options(text_ref, &self.options)?;

        // Validate configuration constraints
        self.options.validate_constraints()?;

        crate::boxen(text_ref, Some(self.options))
    }

    /// Validate the current builder configuration without building
    pub fn validate(&self) -> BoxenResult<()> {
        self.options.validate_constraints()
    }

    /// Validate configuration with intelligent recommendations
    pub fn validate_with_suggestions(&self, text: &str) -> crate::validation::ValidationResult {
        crate::validation::validate_configuration(text, &self.options)
    }

    /// Calculate minimum dimensions required for the given text
    pub fn minimum_dimensions(&self, text: &str) -> crate::validation::MinimumDimensions {
        crate::validation::calculate_minimum_dimensions(text, &self.options)
    }

    /// Suggest optimal dimensions for the given text
    pub fn suggest_dimensions(&self, text: &str) -> (usize, usize) {
        crate::validation::suggest_optimal_dimensions(text, &self.options)
    }

    /// Auto-adjust configuration to fix common issues
    pub fn auto_adjust(mut self, text: &str) -> Self {
        self.options = crate::validation::auto_adjust_options(text, self.options);
        self
    }

    /// Render with auto-adjustment if the current configuration fails
    pub fn render_or_adjust<S: AsRef<str>>(mut self, text: S) -> BoxenResult<String> {
        let text_ref = text.as_ref();

        // Try comprehensive validation first
        match crate::error::validation::validate_all_options(text_ref, &self.options) {
            Ok(_) => {
                // Input validation passed, try configuration validation
                let validation = crate::validation::validate_configuration(text_ref, &self.options);
                if validation.is_valid {
                    // Configuration is valid, proceed with normal render
                    self.render(text_ref)
                } else {
                    // Auto-adjust and try again
                    self.options =
                        crate::validation::recovery::smart_recovery(text_ref, self.options);
                    self.render(text_ref)
                }
            }
            Err(_) => {
                // Input validation failed, try smart recovery
                self.options = crate::validation::recovery::smart_recovery(text_ref, self.options);
                self.render(text_ref)
            }
        }
    }

    /// Get detailed validation information with recommendations
    pub fn check_configuration(&self, text: &str) -> String {
        let validation = self.validate_with_suggestions(text);
        let min_dims = self.minimum_dimensions(text);
        let (opt_width, opt_height) = self.suggest_dimensions(text);

        let mut message = format!("Configuration Analysis for text: {:?}\n", text);
        message.push_str(&format!(
            "Minimum required: {}×{}\n",
            min_dims.width, min_dims.height
        ));
        message.push_str(&format!(
            "Suggested optimal: {}×{}\n",
            opt_width, opt_height
        ));

        if validation.is_valid {
            message.push_str("✓ Configuration is valid\n");
        } else {
            message.push_str("✗ Configuration has errors:\n");
            for error in &validation.errors {
                message.push_str(&format!("  - {}\n", error.detailed_message()));
            }
        }

        if !validation.warnings.is_empty() {
            message.push_str("⚠ Warnings:\n");
            for warning in &validation.warnings {
                message.push_str(&format!("  - {}: {}\n", warning.issue, warning.suggestion));
            }
        }

        message
    }

    /// Convenience method to set both padding and margin to the same value
    pub fn spacing<T: Into<Spacing>>(mut self, spacing: T) -> Self {
        let spacing_value = spacing.into();
        self.options.padding = spacing_value;
        self.options.margin = spacing_value;
        self
    }

    /// Convenience method to set both border and background color
    pub fn colors<C1: Into<Color>, C2: Into<Color>>(mut self, border: C1, background: C2) -> Self {
        self.options.border_color = Some(border.into());
        self.options.background_color = Some(background.into());
        self
    }

    /// Convenience method to set both width and height
    pub fn size(mut self, width: usize, height: usize) -> Self {
        self.options.width = Some(width);
        self.options.height = Some(height);
        self
    }

    /// Convenience method to center align both text and title
    pub fn center_all(mut self) -> Self {
        self.options.text_alignment = TextAlignment::Center;
        self.options.title_alignment = TitleAlignment::Center;
        self.options.float = Float::Center;
        self
    }
}

impl Default for BoxenBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for Color {
    fn from(value: String) -> Self {
        if value.starts_with('#') {
            Color::Hex(value)
        } else {
            Color::Named(value)
        }
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        Color::from(value.to_string())
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color::Rgb(r, g, b)
    }
}

// Additional convenient From implementations
impl From<(usize, usize)> for Spacing {
    /// Creates spacing from (horizontal, vertical) tuple
    /// Horizontal value is applied to left and right, vertical to top and bottom
    fn from((horizontal, vertical): (usize, usize)) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
}

impl From<[usize; 4]> for Spacing {
    /// Creates spacing from [top, right, bottom, left] array
    fn from([top, right, bottom, left]: [usize; 4]) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

impl From<[usize; 2]> for Spacing {
    /// Creates spacing from [horizontal, vertical] array
    fn from([horizontal, vertical]: [usize; 2]) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_horizontal_vertical() {
        let spacing = Spacing {
            top: 1,
            right: 2,
            bottom: 3,
            left: 4,
        };

        assert_eq!(spacing.horizontal(), 6); // left + right = 4 + 2
        assert_eq!(spacing.vertical(), 4); // top + bottom = 1 + 3
    }

    #[test]
    fn test_spacing_is_empty() {
        let empty_spacing = Spacing::default();
        assert!(empty_spacing.is_empty());

        let non_empty_spacing = Spacing {
            top: 1,
            right: 0,
            bottom: 0,
            left: 0,
        };
        assert!(!non_empty_spacing.is_empty());
    }

    #[test]
    fn test_spacing_from_usize() {
        let spacing = Spacing::from(2);
        assert_eq!(spacing.top, 2);
        assert_eq!(spacing.right, 6); // 3x horizontal
        assert_eq!(spacing.bottom, 2);
        assert_eq!(spacing.left, 6); // 3x horizontal
        assert_eq!(spacing.horizontal(), 12);
        assert_eq!(spacing.vertical(), 4);
    }

    #[test]
    fn test_spacing_from_tuple() {
        let spacing = Spacing::from((1, 2, 3, 4));
        assert_eq!(spacing.top, 1);
        assert_eq!(spacing.right, 2);
        assert_eq!(spacing.bottom, 3);
        assert_eq!(spacing.left, 4);
        assert_eq!(spacing.horizontal(), 6);
        assert_eq!(spacing.vertical(), 4);
    }

    #[test]
    fn test_calculate_constraints_default() {
        let options = BoxenOptions::default();
        let constraints = options.calculate_constraints().unwrap();

        assert!(constraints.terminal_width >= 80); // Should have fallback
        assert_eq!(constraints.border_width, 2); // Single border style
        assert!(constraints.max_width <= constraints.terminal_width);
    }

    #[test]
    fn test_calculate_constraints_with_specified_width() {
        let options = BoxenOptions {
            width: Some(50),
            ..Default::default()
        };
        let constraints = options.calculate_constraints().unwrap();

        assert_eq!(constraints.max_width, 50);
    }

    #[test]
    fn test_calculate_constraints_invalid_width() {
        let options = BoxenOptions {
            width: Some(1),            // Too small for borders + padding
            padding: Spacing::from(2), // 6 horizontal padding + 2 border = 8 total
            ..Default::default()
        };

        let result = options.calculate_constraints();
        assert!(result.is_err());
        matches!(result.unwrap_err(), BoxenError::InvalidDimensions { .. });
    }

    #[test]
    fn test_calculate_layout_dimensions_basic() {
        let options = BoxenOptions::default();
        let layout = options.calculate_layout_dimensions(10, 3).unwrap();

        assert_eq!(layout.content_width, 10);
        assert_eq!(layout.content_height, 3);
        assert_eq!(layout.inner_width, 10); // no padding
        assert_eq!(layout.inner_height, 3); // no padding
        assert_eq!(layout.total_width, 12); // 10 + 2 borders
        assert_eq!(layout.total_height, 5); // 3 + 2 borders
    }

    #[test]
    fn test_calculate_layout_dimensions_with_padding() {
        let options = BoxenOptions {
            padding: Spacing::from(1), // 3 horizontal, 1 vertical each side
            ..Default::default()
        };
        let layout = options.calculate_layout_dimensions(10, 3).unwrap();

        assert_eq!(layout.content_width, 10);
        assert_eq!(layout.content_height, 3);
        assert_eq!(layout.inner_width, 16); // 10 + 6 horizontal padding
        assert_eq!(layout.inner_height, 5); // 3 + 2 vertical padding
        assert_eq!(layout.total_width, 18); // 16 + 2 borders
        assert_eq!(layout.total_height, 7); // 5 + 2 borders
    }

    #[test]
    fn test_calculate_layout_dimensions_with_margins() {
        let options = BoxenOptions {
            margin: Spacing::from(1), // 3 horizontal, 1 vertical each side
            ..Default::default()
        };
        let layout = options.calculate_layout_dimensions(10, 3).unwrap();

        assert_eq!(layout.total_width, 18); // 10 + 2 borders + 6 margins
        assert_eq!(layout.total_height, 7); // 3 + 2 borders + 2 margins
    }

    #[test]
    fn test_calculate_layout_dimensions_no_border() {
        let options = BoxenOptions {
            border_style: BorderStyle::None,
            ..Default::default()
        };
        let layout = options.calculate_layout_dimensions(10, 3).unwrap();

        assert_eq!(layout.total_width, 10); // no borders
        assert_eq!(layout.total_height, 3); // no borders
    }

    #[test]
    fn test_calculate_max_content_width() {
        let options = BoxenOptions {
            width: Some(50),
            padding: Spacing::from(1), // 6 horizontal padding
            ..Default::default()
        };

        let max_width = options.calculate_max_content_width().unwrap();
        assert_eq!(max_width, 42); // 50 - 2 borders - 6 padding
    }

    #[test]
    fn test_calculate_max_content_height() {
        let options = BoxenOptions {
            height: Some(20),
            padding: Spacing::from(1), // 2 vertical padding
            ..Default::default()
        };

        let max_height = options.calculate_max_content_height().unwrap();
        assert_eq!(max_height, Some(16)); // 20 - 2 borders - 2 padding
    }

    #[test]
    fn test_calculate_max_content_height_no_constraint() {
        let options = BoxenOptions::default(); // No height specified

        let max_height = options.calculate_max_content_height().unwrap();
        // Should be None or Some value depending on terminal height detection
        // We can't assert a specific value since it depends on the actual terminal
        assert!(max_height.is_none() || max_height.unwrap() > 0);
    }

    #[test]
    fn test_validate_constraints_valid() {
        let options = BoxenOptions {
            width: Some(50),
            height: Some(20),
            padding: Spacing::from(1),
            ..Default::default()
        };

        assert!(options.validate_constraints().is_ok());
    }

    #[test]
    fn test_validate_constraints_invalid() {
        let options = BoxenOptions {
            width: Some(5),            // Too small
            padding: Spacing::from(2), // Large padding
            ..Default::default()
        };

        assert!(options.validate_constraints().is_err());
    }

    #[test]
    fn test_dimension_constraints_with_fullscreen() {
        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Auto),
            ..Default::default()
        };

        let constraints = options.calculate_constraints().unwrap();
        // Should use terminal dimensions
        assert!(constraints.max_width > 0);
        assert_eq!(constraints.max_width, constraints.terminal_width);
    }

    #[test]
    fn test_layout_dimensions_edge_cases() {
        // Test with zero content
        let options = BoxenOptions::default();
        let layout = options.calculate_layout_dimensions(0, 0).unwrap();

        assert_eq!(layout.content_width, 0);
        assert_eq!(layout.content_height, 0);
        assert_eq!(layout.total_width, 2); // Just borders
        assert_eq!(layout.total_height, 2); // Just borders
    }

    #[test]
    fn test_spacing_calculations_comprehensive() {
        let spacing = Spacing {
            top: 2,
            right: 4,
            bottom: 1,
            left: 3,
        };

        assert_eq!(spacing.horizontal(), 7); // 4 + 3
        assert_eq!(spacing.vertical(), 3); // 2 + 1
        assert!(!spacing.is_empty());

        let zero_spacing = Spacing::default();
        assert_eq!(zero_spacing.horizontal(), 0);
        assert_eq!(zero_spacing.vertical(), 0);
        assert!(zero_spacing.is_empty());
    }

    #[test]
    fn test_complex_layout_calculation() {
        let options = BoxenOptions {
            border_style: BorderStyle::Double,
            padding: Spacing {
                top: 1,
                right: 2,
                bottom: 1,
                left: 2,
            },
            margin: Spacing {
                top: 0,
                right: 1,
                bottom: 0,
                left: 1,
            },
            width: Some(60),
            height: Some(50), // Use a larger height that should work on most terminals
            ..Default::default()
        };

        let layout = options.calculate_layout_dimensions(40, 5).unwrap(); // Use smaller content height

        // Content: 40x5
        // Inner (content + padding): 44x7 (40+4, 5+2)
        // Total (inner + borders + margins): 48x9 (44+2+2, 7+2+0)
        assert_eq!(layout.content_width, 40);
        assert_eq!(layout.content_height, 5);
        assert_eq!(layout.inner_width, 44);
        assert_eq!(layout.inner_height, 7);
        assert_eq!(layout.total_width, 48);
        assert_eq!(layout.total_height, 9);
    }

    #[test]
    fn test_overflow_handling() {
        // Test when content would exceed terminal width
        let terminal_width = get_terminal_width();
        let options = BoxenOptions {
            padding: Spacing::from(1),
            ..Default::default()
        };

        // Try to create content that would exceed terminal
        let excessive_width = terminal_width + 100;
        let result = options.calculate_layout_dimensions(excessive_width, 5);

        // Should fail with configuration error
        assert!(result.is_err());
        matches!(result.unwrap_err(), BoxenError::ConfigurationError { .. });
    }

    #[test]
    fn test_fullscreen_mode_auto() {
        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Auto),
            ..Default::default()
        };

        let constraints = options.calculate_constraints().unwrap();

        // Should use terminal dimensions
        assert_eq!(constraints.max_width, constraints.terminal_width);
        assert_eq!(constraints.max_height, constraints.terminal_height);
    }

    #[test]
    fn test_fullscreen_mode_auto_with_margins() {
        let margin = Spacing::from(2);
        println!(
            "Margin: top={}, right={}, bottom={}, left={}",
            margin.top, margin.right, margin.bottom, margin.left
        );
        println!("Horizontal margin: {}", margin.horizontal());

        let expected_horizontal = margin.horizontal();
        let expected_vertical = margin.vertical();

        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Auto),
            margin,
            ..Default::default()
        };

        let constraints = options.calculate_constraints().unwrap();

        // Should account for margins - Spacing::from(2) creates 12 horizontal (left=6, right=6), 4 vertical
        assert_eq!(
            constraints.max_width,
            constraints.terminal_width - expected_horizontal
        );
        if let Some(terminal_height) = constraints.terminal_height {
            assert_eq!(
                constraints.max_height,
                Some(terminal_height - expected_vertical)
            );
        }
    }

    #[test]
    fn test_fullscreen_mode_custom() {
        let custom_func = |width: usize, height: usize| -> (usize, usize) {
            // Use 3/4 of dimensions to ensure we have enough space for borders and padding
            (width * 3 / 4, height * 3 / 4)
        };

        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Custom(custom_func)),
            ..Default::default()
        };

        let constraints = options.calculate_constraints().unwrap();

        // Should use custom dimensions
        assert_eq!(constraints.max_width, constraints.terminal_width * 3 / 4);
        if let Some(terminal_height) = constraints.terminal_height {
            assert_eq!(constraints.max_height, Some(terminal_height * 3 / 4));
        }
    }

    #[test]
    fn test_fullscreen_mode_with_padding() {
        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Auto),
            padding: Spacing::from(1), // 6 horizontal, 2 vertical
            ..Default::default()
        };

        let max_content_width = options.calculate_max_content_width().unwrap();
        let max_content_height = options.calculate_max_content_height().unwrap();

        // Should account for borders and padding
        let constraints = options.calculate_constraints().unwrap();
        let expected_width = constraints.max_width - 2 - 6; // borders + padding horizontal
        assert_eq!(max_content_width, expected_width);

        if let Some(height) = max_content_height {
            let expected_height = constraints.max_height.unwrap() - 2 - 2; // borders + padding vertical
            assert_eq!(height, expected_height);
        }
    }

    #[test]
    fn test_fullscreen_mode_insufficient_space() {
        // Create a scenario where fullscreen mode doesn't have enough space
        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Custom(|_, _| (5, 3))), // Very small dimensions
            padding: Spacing::from(3), // Large padding: 18 horizontal (9*2), 6 vertical (3*2)
            ..Default::default()
        };

        let result = options.calculate_constraints();

        // Should fail due to insufficient space (5 total width < 18 padding + 2 borders = 20)
        assert!(result.is_err());
        matches!(result.unwrap_err(), BoxenError::InvalidDimensions { .. });
    }

    #[test]
    fn test_fullscreen_mode_overrides_width_height() {
        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Auto),
            width: Some(50),  // Should be ignored in fullscreen mode
            height: Some(20), // Should be ignored in fullscreen mode
            ..Default::default()
        };

        let constraints = options.calculate_constraints().unwrap();

        // Should use terminal dimensions, not specified width/height
        assert_eq!(constraints.max_width, constraints.terminal_width);
        assert_eq!(constraints.max_height, constraints.terminal_height);
    }

    #[test]
    fn test_fullscreen_layout_dimensions() {
        let options = BoxenOptions {
            fullscreen: Some(FullscreenMode::Auto),
            padding: Spacing::from(1), // 6 horizontal, 2 vertical
            margin: Spacing::from(1),  // 6 horizontal, 2 vertical
            ..Default::default()
        };

        // Use small content that should be expanded to fill fullscreen
        let layout = options.calculate_layout_dimensions(10, 3).unwrap();

        // Content should be expanded to fill available space
        let max_content_width = options.calculate_max_content_width().unwrap();
        let max_content_height = options.calculate_max_content_height().unwrap();

        assert_eq!(layout.content_width, max_content_width);
        if let Some(expected_height) = max_content_height {
            assert_eq!(layout.content_height, expected_height);
        }
    }

    // Builder pattern tests
    #[test]
    fn test_builder_new() {
        let builder = BoxenBuilder::new();
        let options = builder.build();

        // Should have default values
        assert!(matches!(options.border_style, BorderStyle::Single));
        assert!(options.padding.is_empty());
        assert!(options.margin.is_empty());
        assert!(matches!(options.text_alignment, TextAlignment::Left));
        assert!(options.title.is_none());
        assert!(matches!(options.title_alignment, TitleAlignment::Left));
        assert!(matches!(options.float, Float::Left));
        assert!(options.width.is_none());
        assert!(options.height.is_none());
        assert!(options.border_color.is_none());
        assert!(options.background_color.is_none());
        assert!(!options.dim_border);
        assert!(options.fullscreen.is_none());
    }

    #[test]
    fn test_builder_default() {
        let builder = BoxenBuilder::default();
        let options = builder.build();

        // Should be same as new()
        assert!(matches!(options.border_style, BorderStyle::Single));
        assert!(options.padding.is_empty());
    }

    #[test]
    fn test_builder_method_chaining() {
        let result = BoxenBuilder::new()
            .border_style(BorderStyle::Double)
            .padding(2)
            .margin((1, 2, 3, 4))
            .text_alignment(TextAlignment::Center)
            .title("Test Title")
            .title_alignment(TitleAlignment::Right)
            .float(Float::Center)
            .width(50)
            .height(20)
            .border_color("red")
            .background_color("#ff0000")
            .dim_border(true)
            .fullscreen(FullscreenMode::Auto)
            .render("Hello World");

        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_border_style() {
        let options = BoxenBuilder::new().border_style(BorderStyle::Round).build();

        assert!(matches!(options.border_style, BorderStyle::Round));
    }

    #[test]
    fn test_builder_padding_from_usize() {
        let options = BoxenBuilder::new().padding(3).build();

        assert_eq!(options.padding.top, 3);
        assert_eq!(options.padding.right, 9); // 3x horizontal
        assert_eq!(options.padding.bottom, 3);
        assert_eq!(options.padding.left, 9); // 3x horizontal
    }

    #[test]
    fn test_builder_padding_from_tuple() {
        let options = BoxenBuilder::new().padding((1, 2, 3, 4)).build();

        assert_eq!(options.padding.top, 1);
        assert_eq!(options.padding.right, 2);
        assert_eq!(options.padding.bottom, 3);
        assert_eq!(options.padding.left, 4);
    }

    #[test]
    fn test_builder_margin_from_usize() {
        let options = BoxenBuilder::new().margin(2).build();

        assert_eq!(options.margin.top, 2);
        assert_eq!(options.margin.right, 6); // 3x horizontal
        assert_eq!(options.margin.bottom, 2);
        assert_eq!(options.margin.left, 6); // 3x horizontal
    }

    #[test]
    fn test_builder_margin_from_tuple() {
        let options = BoxenBuilder::new().margin((5, 6, 7, 8)).build();

        assert_eq!(options.margin.top, 5);
        assert_eq!(options.margin.right, 6);
        assert_eq!(options.margin.bottom, 7);
        assert_eq!(options.margin.left, 8);
    }

    #[test]
    fn test_builder_text_alignment() {
        let options = BoxenBuilder::new()
            .text_alignment(TextAlignment::Right)
            .build();

        assert!(matches!(options.text_alignment, TextAlignment::Right));
    }

    #[test]
    fn test_builder_title() {
        let options = BoxenBuilder::new().title("My Title").build();

        assert_eq!(options.title, Some("My Title".to_string()));
    }

    #[test]
    fn test_builder_title_string() {
        let title = String::from("Dynamic Title");
        let options = BoxenBuilder::new().title(title.clone()).build();

        assert_eq!(options.title, Some(title));
    }

    #[test]
    fn test_builder_title_alignment() {
        let options = BoxenBuilder::new()
            .title_alignment(TitleAlignment::Center)
            .build();

        assert!(matches!(options.title_alignment, TitleAlignment::Center));
    }

    #[test]
    fn test_builder_float() {
        let options = BoxenBuilder::new().float(Float::Right).build();

        assert!(matches!(options.float, Float::Right));
    }

    #[test]
    fn test_builder_dimensions() {
        let options = BoxenBuilder::new().width(80).height(25).build();

        assert_eq!(options.width, Some(80));
        assert_eq!(options.height, Some(25));
    }

    #[test]
    fn test_builder_border_color_string() {
        let options = BoxenBuilder::new().border_color("blue").build();

        if let Some(Color::Named(name)) = options.border_color {
            assert_eq!(name, "blue");
        } else {
            panic!("Expected named color");
        }
    }

    #[test]
    fn test_builder_border_color_hex() {
        let options = BoxenBuilder::new().border_color("#00ff00").build();

        if let Some(Color::Hex(hex)) = options.border_color {
            assert_eq!(hex, "#00ff00");
        } else {
            panic!("Expected hex color");
        }
    }

    #[test]
    fn test_builder_border_color_rgb() {
        let options = BoxenBuilder::new().border_color((255, 128, 0)).build();

        if let Some(Color::Rgb(r, g, b)) = options.border_color {
            assert_eq!((r, g, b), (255, 128, 0));
        } else {
            panic!("Expected RGB color");
        }
    }

    #[test]
    fn test_builder_background_color() {
        let options = BoxenBuilder::new().background_color("yellow").build();

        if let Some(Color::Named(name)) = options.background_color {
            assert_eq!(name, "yellow");
        } else {
            panic!("Expected named color");
        }
    }

    #[test]
    fn test_builder_dim_border() {
        let options = BoxenBuilder::new().dim_border(true).build();

        assert!(options.dim_border);
    }

    #[test]
    fn test_builder_fullscreen_auto() {
        let options = BoxenBuilder::new().fullscreen(FullscreenMode::Auto).build();

        assert!(matches!(options.fullscreen, Some(FullscreenMode::Auto)));
    }

    #[test]
    fn test_builder_fullscreen_custom() {
        let custom_func = |w: usize, h: usize| (w / 2, h / 2);
        let options = BoxenBuilder::new()
            .fullscreen(FullscreenMode::Custom(custom_func))
            .build();

        assert!(matches!(
            options.fullscreen,
            Some(FullscreenMode::Custom(_))
        ));
    }

    #[test]
    fn test_builder_render_success() {
        let result = BoxenBuilder::new()
            .border_style(BorderStyle::Single)
            .padding(1)
            .render("Test content");

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Test content"));
    }

    #[test]
    fn test_builder_render_with_validation_error() {
        let result = BoxenBuilder::new()
            .width(5) // Too small
            .padding(10) // Too large padding
            .render("Test");

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            BoxenError::InvalidDimensions { .. }
        ));
    }

    #[test]
    fn test_builder_validate_success() {
        let builder = BoxenBuilder::new().width(50).padding(2);

        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_builder_validate_error() {
        let builder = BoxenBuilder::new()
            .width(3) // Too small
            .padding(5); // Too large padding

        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_builder_complex_configuration() {
        let result = BoxenBuilder::new()
            .border_style(BorderStyle::Double)
            .padding((2, 4, 2, 4))
            .margin(1)
            .text_alignment(TextAlignment::Center)
            .title("Complex Box")
            .title_alignment(TitleAlignment::Center)
            .float(Float::Center)
            .width(60)
            .border_color("#0066cc")
            .background_color("white")
            .dim_border(false)
            .render("This is a complex box configuration test");

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("This is a complex box configuration test"));
    }

    #[test]
    fn test_builder_empty_text() {
        let result = BoxenBuilder::new().render("");

        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_multiline_text() {
        let text = "Line 1\nLine 2\nLine 3";
        let result = BoxenBuilder::new().padding(1).render(text);

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Line 1"));
        assert!(output.contains("Line 2"));
        assert!(output.contains("Line 3"));
    }

    #[test]
    fn test_builder_method_chaining_order_independence() {
        // Test that method order doesn't matter
        let options1 = BoxenBuilder::new()
            .width(50)
            .padding(2)
            .border_style(BorderStyle::Round)
            .build();

        let options2 = BoxenBuilder::new()
            .border_style(BorderStyle::Round)
            .padding(2)
            .width(50)
            .build();

        // Both should produce equivalent options
        assert!(matches!(options1.border_style, BorderStyle::Round));
        assert!(matches!(options2.border_style, BorderStyle::Round));
        assert_eq!(options1.width, Some(50));
        assert_eq!(options2.width, Some(50));
        assert_eq!(options1.padding.top, 2);
        assert_eq!(options2.padding.top, 2);
    }

    #[test]
    fn test_builder_overwrite_values() {
        // Test that later calls overwrite earlier ones
        let options = BoxenBuilder::new()
            .width(30)
            .width(50) // Should overwrite the 30
            .padding(1)
            .padding(3) // Should overwrite the 1
            .build();

        assert_eq!(options.width, Some(50));
        assert_eq!(options.padding.top, 3);
        assert_eq!(options.padding.right, 9); // 3x horizontal
    }

    // Test additional From implementations
    #[test]
    fn test_spacing_from_horizontal_vertical_tuple() {
        let spacing = Spacing::from((4, 2));
        assert_eq!(spacing.top, 2);
        assert_eq!(spacing.right, 4);
        assert_eq!(spacing.bottom, 2);
        assert_eq!(spacing.left, 4);
    }

    #[test]
    fn test_spacing_from_array_4() {
        let spacing = Spacing::from([1, 2, 3, 4]);
        assert_eq!(spacing.top, 1);
        assert_eq!(spacing.right, 2);
        assert_eq!(spacing.bottom, 3);
        assert_eq!(spacing.left, 4);
    }

    #[test]
    fn test_spacing_from_array_2() {
        let spacing = Spacing::from([6, 3]);
        assert_eq!(spacing.top, 3);
        assert_eq!(spacing.right, 6);
        assert_eq!(spacing.bottom, 3);
        assert_eq!(spacing.left, 6);
    }

    // Test convenience builder methods
    #[test]
    fn test_builder_spacing_convenience() {
        let options = BoxenBuilder::new().spacing(2).build();

        // Both padding and margin should be set
        assert_eq!(options.padding.top, 2);
        assert_eq!(options.padding.right, 6); // 3x horizontal
        assert_eq!(options.margin.top, 2);
        assert_eq!(options.margin.right, 6); // 3x horizontal
    }

    #[test]
    fn test_builder_colors_convenience() {
        let options = BoxenBuilder::new().colors("red", "#00ff00").build();

        if let Some(Color::Named(border_name)) = options.border_color {
            assert_eq!(border_name, "red");
        } else {
            panic!("Expected named border color");
        }

        if let Some(Color::Hex(bg_hex)) = options.background_color {
            assert_eq!(bg_hex, "#00ff00");
        } else {
            panic!("Expected hex background color");
        }
    }

    #[test]
    fn test_builder_size_convenience() {
        let options = BoxenBuilder::new().size(80, 25).build();

        assert_eq!(options.width, Some(80));
        assert_eq!(options.height, Some(25));
    }

    #[test]
    fn test_builder_center_all_convenience() {
        let options = BoxenBuilder::new().center_all().build();

        assert!(matches!(options.text_alignment, TextAlignment::Center));
        assert!(matches!(options.title_alignment, TitleAlignment::Center));
        assert!(matches!(options.float, Float::Center));
    }

    #[test]
    fn test_builder_with_array_spacing() {
        let options = BoxenBuilder::new()
            .padding([1, 2, 3, 4])
            .margin([5, 6])
            .build();

        assert_eq!(options.padding.top, 1);
        assert_eq!(options.padding.right, 2);
        assert_eq!(options.padding.bottom, 3);
        assert_eq!(options.padding.left, 4);

        assert_eq!(options.margin.top, 6);
        assert_eq!(options.margin.right, 5);
        assert_eq!(options.margin.bottom, 6);
        assert_eq!(options.margin.left, 5);
    }

    #[test]
    fn test_builder_with_tuple_spacing() {
        let options = BoxenBuilder::new()
            .padding((8, 4)) // horizontal, vertical
            .build();

        assert_eq!(options.padding.top, 4);
        assert_eq!(options.padding.right, 8);
        assert_eq!(options.padding.bottom, 4);
        assert_eq!(options.padding.left, 8);
    }

    #[test]
    fn test_builder_convenience_methods_chaining() {
        let result = BoxenBuilder::new()
            .spacing(1)
            .colors("blue", "white")
            .size(60, 10) // Use smaller height to avoid terminal size issues
            .center_all()
            .title("Centered Box")
            .render("This box uses all convenience methods");

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("This box uses all convenience methods"));
    }
}

/// Dimension constraints for box calculation
#[derive(Debug, Clone)]
pub struct DimensionConstraints {
    /// Maximum allowed width for the box
    pub max_width: usize,
    /// Maximum allowed height for the box (None if unlimited)
    pub max_height: Option<usize>,
    /// Current terminal width in columns
    pub terminal_width: usize,
    /// Current terminal height in rows (None if unknown)
    pub terminal_height: Option<usize>,
    /// Width consumed by borders (0 for no border, 2 for visible borders)
    pub border_width: usize,
}

/// Final calculated layout dimensions
#[derive(Debug, Clone)]
pub struct LayoutDimensions {
    /// Width of the actual text content area
    pub content_width: usize,
    /// Height of the actual text content area
    pub content_height: usize,
    /// Total width including borders, padding, and margins
    pub total_width: usize,
    /// Total height including borders, padding, and margins
    pub total_height: usize,
    /// Width of content area plus padding (excludes borders and margins)
    pub inner_width: usize,
    /// Height of content area plus padding (excludes borders and margins)
    pub inner_height: usize,
}

impl Spacing {
    /// Calculate total horizontal spacing (left + right).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ::boxen::Spacing;
    ///
    /// let spacing = Spacing::from((1, 2, 3, 4));  // top, right, bottom, left
    /// assert_eq!(spacing.horizontal(), 6);  // right + left = 2 + 4
    /// ```
    pub fn horizontal(&self) -> usize {
        self.left + self.right
    }

    /// Calculate total vertical spacing (top + bottom).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ::boxen::Spacing;
    ///
    /// let spacing = Spacing::from((1, 2, 3, 4));  // top, right, bottom, left
    /// assert_eq!(spacing.vertical(), 4);  // top + bottom = 1 + 3
    /// ```
    pub fn vertical(&self) -> usize {
        self.top + self.bottom
    }

    /// Check if all spacing values are zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ::boxen::Spacing;
    ///
    /// let empty = Spacing::default();
    /// assert!(empty.is_empty());
    ///
    /// let non_empty = Spacing::from(1);
    /// assert!(!non_empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.top == 0 && self.right == 0 && self.bottom == 0 && self.left == 0
    }

    /// Create uniform spacing with the same value for all sides.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ::boxen::Spacing;
    ///
    /// let uniform = Spacing::uniform(2);
    /// assert_eq!(uniform.top, 2);
    /// assert_eq!(uniform.right, 2);
    /// assert_eq!(uniform.bottom, 2);
    /// assert_eq!(uniform.left, 2);
    /// ```
    pub fn uniform(value: usize) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Create spacing with separate horizontal and vertical values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ::boxen::Spacing;
    ///
    /// let spacing = Spacing::symmetric(3, 1);  // horizontal: 3, vertical: 1
    /// assert_eq!(spacing.top, 1);
    /// assert_eq!(spacing.right, 3);
    /// assert_eq!(spacing.bottom, 1);
    /// assert_eq!(spacing.left, 3);
    /// ```
    pub fn symmetric(horizontal: usize, vertical: usize) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
}

impl BoxenOptions {
    /// Helper to create InvalidDimensions error with basic recommendations
    fn invalid_dimensions_error(
        message: String,
        width: Option<usize>,
        height: Option<usize>,
    ) -> crate::error::BoxenError {
        use crate::error::{BoxenError, ErrorRecommendation};

        let mut recommendations = vec![];

        if let Some(w) = width {
            recommendations.push(ErrorRecommendation::suggestion_only(
                "Width too small".to_string(),
                format!("Consider increasing width from {}", w),
            ));
        }

        if let Some(h) = height {
            recommendations.push(ErrorRecommendation::suggestion_only(
                "Height too small".to_string(),
                format!("Consider increasing height from {}", h),
            ));
        }

        BoxenError::invalid_dimensions(message, width, height, recommendations)
    }

    /// Helper to create ConfigurationError with basic recommendations
    fn configuration_error(message: String) -> crate::error::BoxenError {
        use crate::error::{BoxenError, ErrorRecommendation};

        let recommendations = vec![ErrorRecommendation::suggestion_only(
            "Configuration conflict".to_string(),
            "Check your width, height, padding, and margin settings".to_string(),
        )];

        BoxenError::configuration_error(message, recommendations)
    }
    /// Calculate dimension constraints based on terminal size and options
    pub fn calculate_constraints(&self) -> BoxenResult<DimensionConstraints> {
        let terminal_width = get_terminal_width();
        let terminal_height = get_terminal_height();
        let border_width = calculate_border_width(&self.border_style);

        // Handle fullscreen mode first
        if let Some(fullscreen_mode) = &self.fullscreen {
            return self.calculate_fullscreen_constraints(
                fullscreen_mode,
                terminal_width,
                terminal_height,
                border_width,
            );
        }

        let _total_horizontal_overhead =
            border_width + self.padding.horizontal() + self.margin.horizontal();

        // Calculate maximum available width
        let max_width = if let Some(specified_width) = self.width {
            // When width is specified, it represents the total box width including margins
            // So we need to subtract margins to get the available width for content + borders + padding
            let available_width_for_content = if specified_width > self.margin.horizontal() {
                specified_width - self.margin.horizontal()
            } else {
                return Err(Self::invalid_dimensions_error(
                    format!(
                        "Width {} is too small for margins {}",
                        specified_width,
                        self.margin.horizontal()
                    ),
                    Some(specified_width),
                    self.height,
                ));
            };

            // Validate that we have enough space for borders and padding
            if available_width_for_content < border_width + self.padding.horizontal() {
                return Err(Self::invalid_dimensions_error(
                    format!(
                        "Width {} is too small for borders and padding",
                        specified_width
                    ),
                    Some(specified_width),
                    self.height,
                ));
            }

            available_width_for_content
        } else {
            // Use terminal width minus margins (borders and padding will be subtracted later)
            if terminal_width < self.margin.horizontal() {
                return Err(BoxenError::terminal_size_error(
                    "Failed to detect terminal dimensions".to_string(),
                    vec![
                        crate::error::ErrorRecommendation::suggestion_only(
                            "Terminal detection failed".to_string(),
                            "Specify explicit width and height instead of using fullscreen mode"
                                .to_string(),
                        ),
                        crate::error::ErrorRecommendation::with_auto_fix(
                            "Use fixed dimensions".to_string(),
                            "Set explicit dimensions".to_string(),
                            ".width(80).height(24)".to_string(),
                        ),
                    ],
                ));
            }
            terminal_width - self.margin.horizontal()
        };

        // Calculate maximum available height
        let max_height = if let Some(specified_height) = self.height {
            // When height is specified, it represents the total box height including margins
            // So we need to subtract margins to get the available height for content + borders + padding
            if specified_height > self.margin.vertical() {
                Some(specified_height - self.margin.vertical())
            } else {
                return Err(Self::invalid_dimensions_error(
                    format!(
                        "Height {} is too small for margins {}",
                        specified_height,
                        self.margin.vertical()
                    ),
                    None,
                    Some(specified_height),
                ));
            }
        } else {
            // Don't apply height constraints unless explicitly specified by user
            None
        };

        Ok(DimensionConstraints {
            max_width,
            max_height,
            terminal_width,
            terminal_height,
            border_width,
        })
    }

    /// Calculate final layout dimensions for given content
    pub fn calculate_layout_dimensions(
        &self,
        content_width: usize,
        content_height: usize,
    ) -> BoxenResult<LayoutDimensions> {
        let constraints = self.calculate_constraints()?;

        // In fullscreen mode, expand content to fill available space
        let (final_content_width, final_content_height) = if self.fullscreen.is_some() {
            let max_content_width = self.calculate_max_content_width()?;
            let max_content_height = self.calculate_max_content_height()?;

            let expanded_width = max_content_width;
            let expanded_height = if let Some(max_height) = max_content_height {
                max_height
            } else {
                content_height // Use original height if no height constraint
            };

            (expanded_width, expanded_height)
        } else {
            (content_width, content_height)
        };

        // Calculate inner dimensions (content + padding)
        let inner_width = final_content_width + self.padding.horizontal();
        let inner_height = final_content_height + self.padding.vertical();

        // Calculate box dimensions without margins (for constraint validation)
        let box_width = inner_width + constraints.border_width;
        let box_height = inner_height
            + (if matches!(self.border_style, BorderStyle::None) {
                0
            } else {
                2
            }); // top and bottom borders

        // Calculate total dimensions (box + margins)
        let total_width = box_width + self.margin.horizontal();
        let total_height = box_height + self.margin.vertical();

        // Validate against constraints
        // In fullscreen mode, ignore specified width and use terminal width as limit
        let width_limit = if self.fullscreen.is_some() {
            constraints.terminal_width // In fullscreen mode, use terminal width as limit
        } else if let Some(specified_width) = self.width {
            specified_width
        } else {
            constraints.max_width + self.margin.horizontal()
        };

        if total_width > width_limit {
            return Err(Self::configuration_error(format!(
                "Calculated box width ({}) exceeds maximum available width ({})",
                total_width, width_limit
            )));
        }

        // For height validation, compare box height (without margins) against max_height (which already has margins subtracted)
        if let Some(max_height) = constraints.max_height {
            if box_height > max_height {
                return Err(Self::configuration_error(format!(
                    "Calculated box height ({}) exceeds maximum available height ({})",
                    box_height, max_height
                )));
            }
        }

        // Validate against terminal constraints
        if total_width > constraints.terminal_width {
            return Err(Self::configuration_error(format!(
                "Box width ({}) exceeds terminal width ({})",
                total_width, constraints.terminal_width
            )));
        }

        if let Some(terminal_height) = constraints.terminal_height {
            if total_height > terminal_height {
                return Err(Self::configuration_error(format!(
                    "Box height ({}) exceeds terminal height ({})",
                    total_height, terminal_height
                )));
            }
        }

        Ok(LayoutDimensions {
            content_width: final_content_width,
            content_height: final_content_height,
            total_width,
            total_height,
            inner_width,
            inner_height,
        })
    }

    /// Calculate the maximum content width available given the current options
    pub fn calculate_max_content_width(&self) -> BoxenResult<usize> {
        let constraints = self.calculate_constraints()?;
        let total_overhead = constraints.border_width + self.padding.horizontal();

        if constraints.max_width < total_overhead {
            return Err(Self::invalid_dimensions_error(
                format!(
                    "Width {} is too small for borders and padding",
                    constraints.max_width
                ),
                Some(constraints.max_width),
                None,
            ));
        }

        Ok(constraints.max_width - total_overhead)
    }

    /// Calculate the maximum content height available given the current options
    pub fn calculate_max_content_height(&self) -> BoxenResult<Option<usize>> {
        let constraints = self.calculate_constraints()?;

        if let Some(max_height) = constraints.max_height {
            let vertical_overhead = self.padding.vertical()
                + (if matches!(self.border_style, BorderStyle::None) {
                    0
                } else {
                    2
                });

            if max_height < vertical_overhead {
                return Err(Self::invalid_dimensions_error(
                    format!("Height {} is too small for borders and padding", max_height),
                    None,
                    Some(max_height),
                ));
            }

            Ok(Some(max_height - vertical_overhead))
        } else {
            Ok(None)
        }
    }

    /// Calculate constraints for fullscreen mode
    fn calculate_fullscreen_constraints(
        &self,
        fullscreen_mode: &FullscreenMode,
        terminal_width: usize,
        terminal_height: Option<usize>,
        border_width: usize,
    ) -> BoxenResult<DimensionConstraints> {
        let (target_width, target_height) = match fullscreen_mode {
            FullscreenMode::Auto => {
                // Use full terminal dimensions
                (terminal_width, terminal_height)
            }
            FullscreenMode::Custom(func) => {
                // Use custom function to calculate dimensions
                let height = terminal_height.unwrap_or(24); // Fallback height
                let (custom_width, custom_height) = func(terminal_width, height);
                (custom_width, Some(custom_height))
            }
        };

        // In fullscreen mode, the target dimensions represent the total box size
        // We need to calculate the available space for content
        let max_width = if target_width > self.margin.horizontal() {
            target_width - self.margin.horizontal()
        } else {
            return Err(Self::invalid_dimensions_error(
                format!("Target width {} is too small for margins", target_width),
                Some(target_width),
                target_height,
            ));
        };

        let max_height = if let Some(height) = target_height {
            if height > self.margin.vertical() {
                Some(height - self.margin.vertical())
            } else {
                return Err(Self::invalid_dimensions_error(
                    format!("Target height {} is too small for margins", height),
                    Some(target_width),
                    Some(height),
                ));
            }
        } else {
            None
        };

        // Validate that we have enough space for borders and padding
        if max_width < border_width + self.padding.horizontal() {
            return Err(Self::invalid_dimensions_error(
                "Insufficient space for borders and padding".to_string(),
                Some(target_width),
                target_height,
            ));
        }

        if let Some(height) = max_height {
            let vertical_border_overhead = if matches!(self.border_style, BorderStyle::None) {
                0
            } else {
                2
            };
            if height < vertical_border_overhead + self.padding.vertical() {
                return Err(Self::invalid_dimensions_error(
                    "Insufficient space for vertical borders and padding".to_string(),
                    Some(target_width),
                    Some(height + self.margin.vertical()),
                ));
            }
        }

        Ok(DimensionConstraints {
            max_width,
            max_height,
            terminal_width,
            terminal_height,
            border_width,
        })
    }

    /// Validate that the current options are compatible with terminal constraints
    pub fn validate_constraints(&self) -> BoxenResult<()> {
        let _constraints = self.calculate_constraints()?;
        let _max_content_width = self.calculate_max_content_width()?;
        let _max_content_height = self.calculate_max_content_height()?;
        Ok(())
    }
}
