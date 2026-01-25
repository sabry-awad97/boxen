/// Memory profiling example using dhat
///
/// This example demonstrates memory allocation patterns and helps identify
/// optimization opportunities using the dhat heap profiler.
///
/// Run with: cargo run --example memory_profiling --features dhat-heap
/// Then view the output with: dh_view.py dhat-heap.json

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use ::boxen::{BorderStyle, BoxenOptions, Spacing, TextAlignment, TitleAlignment, boxen, builder};

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!("Memory Profiling with dhat");
    println!("==========================\n");

    // Profile simple box
    println!("1. Simple box:");
    for _ in 0..100 {
        let _ = boxen("Hello, World!", None);
    }

    // Profile box with options
    println!("2. Box with options:");
    for _ in 0..100 {
        let _ = boxen(
            "Hello, World!",
            Some(BoxenOptions {
                border_style: BorderStyle::Double,
                padding: Spacing::from(2),
                ..Default::default()
            }),
        );
    }

    // Profile box with title
    println!("3. Box with title:");
    for _ in 0..100 {
        let _ = builder()
            .border_style(BorderStyle::Round)
            .padding(2)
            .title("Test Title")
            .title_alignment(TitleAlignment::Center)
            .render("Content with title");
    }

    // Profile multi-line content
    println!("4. Multi-line content:");
    let multiline = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
    for _ in 0..100 {
        let _ = boxen(multiline, None);
    }

    // Profile large content
    println!("5. Large content:");
    let large_content = "Large content line. ".repeat(50);
    for _ in 0..100 {
        let _ = boxen(
            &large_content,
            Some(BoxenOptions {
                width: Some(60),
                ..Default::default()
            }),
        );
    }

    // Profile Unicode content
    println!("6. Unicode content:");
    let unicode = "Unicode: 🌍🌎🌏 你好世界 🚀✨🎉 Émojis: àáâãäåæçèéêë";
    for _ in 0..100 {
        let _ = boxen(unicode, None);
    }

    // Profile complex configuration
    println!("7. Complex configuration:");
    for _ in 0..100 {
        let _ = builder()
            .border_style(BorderStyle::Double)
            .padding(3)
            .margin(2)
            .text_alignment(TextAlignment::Center)
            .title("Performance Test")
            .title_alignment(TitleAlignment::Center)
            .width(60)
            .border_color("red")
            .dim_border(true)
            .render("Complex box configuration");
    }

    println!("\nProfiling complete!");

    #[cfg(feature = "dhat-heap")]
    println!("\nMemory profile saved to dhat-heap.json");

    #[cfg(not(feature = "dhat-heap"))]
    println!("\nTo enable memory profiling, run with:");
    println!("cargo run --example memory_profiling --features dhat-heap");
}
