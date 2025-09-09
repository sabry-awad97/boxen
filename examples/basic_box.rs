/// Basic boxen usage example
/// Run with: cargo run --example basic_box
use ::boxen::{boxen, double_box, round_box, simple_box};

fn main() {
    println!("=== Basic Boxen Examples ===\n");

    // Simplest usage
    println!("1. Default box:");
    let basic = boxen("Hello, World!", None).unwrap();
    println!("{}\n", basic);

    // Convenience functions
    println!("2. Simple box (convenience function):");
    println!("{}\n", simple_box("This is a simple box"));

    println!("3. Double border box:");
    println!("{}\n", double_box("Double border style"));

    println!("4. Round corner box:");
    println!("{}\n", round_box("Rounded corners"));

    // Multi-line text
    println!("5. Multi-line content:");
    let multiline = simple_box("Line 1\nLine 2\nLine 3");
    println!("{}\n", multiline);

    // Longer text that demonstrates wrapping
    println!("6. Text wrapping:");
    let long_text = "This is a longer piece of text that will demonstrate how boxen handles text wrapping when the content is wider than the default terminal width.";
    println!("{}\n", simple_box(long_text));

    println!("=== End of Basic Examples ===");
}
