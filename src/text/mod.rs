/// Text processing functionality
pub mod alignment;
pub mod measurement;
pub mod wrapping;

pub use alignment::{
    align_line, align_lines, apply_padding, calculate_content_height, calculate_content_width,
    process_text_alignment,
};
pub use measurement::*;
pub use wrapping::*;
