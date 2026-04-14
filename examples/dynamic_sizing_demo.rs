/// Demonstration of dynamic width and height sizing
///
/// This example shows how to use the new Width and Height types to create
/// boxes that adapt to available terminal space.
use boxen::{BorderStyle, TextAlignment, builder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Dynamic Width/Height Demo ===\n");

    // Example 1: Fixed width (traditional approach)
    println!("1. Fixed width (50 columns):");
    let result = builder()
        .border_style(BorderStyle::Round)
        .padding(1)
        .width(50)
        .text_alignment(TextAlignment::Center)
        .title("Fixed Width")
        .render("This box has a fixed width of 50 columns")?;
    println!("{}\n", result);

    // Example 2: Dynamic width - use 80% of available space
    println!("2. Dynamic width (80% of terminal):");
    let result = builder()
        .border_style(BorderStyle::Double)
        .padding(1)
        .width(|available: usize| (available * 4 / 5).max(30))
        .text_alignment(TextAlignment::Center)
        .title("Dynamic Width")
        .render("This box uses 80% of available terminal width (minimum 30 columns)")?;
    println!("{}\n", result);

    // Example 3: Dynamic width with maximum limit
    println!("3. Dynamic width (up to 60 columns):");
    let result = builder()
        .border_style(BorderStyle::Bold)
        .padding(1)
        .width(|available: usize| available.min(60))
        .text_alignment(TextAlignment::Center)
        .title("Capped Width")
        .render("This box uses available width but never exceeds 60 columns")?;
    println!("{}\n", result);

    // Example 4: Dynamic height
    println!("4. Dynamic height (50% of terminal):");
    let content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8";
    let result = builder()
        .border_style(BorderStyle::Round)
        .padding(1)
        .width(50)
        .height(|available: usize| (available / 2).max(10))
        .title("Dynamic Height")
        .render(content)?;
    println!("{}\n", result);

    // Example 5: Both dynamic width and height
    println!("5. Both dynamic (responsive box):");
    let result = builder()
        .border_style(BorderStyle::Single)
        .padding(1)
        .width(|available: usize| (available * 3 / 4).max(40))
        .height(|available: usize| (available / 3).max(8))
        .text_alignment(TextAlignment::Center)
        .title("Fully Responsive")
        .render("This box adapts both width and height to terminal size")?;
    println!("{}\n", result);

    // Example 6: Mix fixed and dynamic
    println!("6. Mix fixed and dynamic:");
    let result = builder()
        .border_style(BorderStyle::Round)
        .padding(1)
        .width(|available: usize| available.max(30))
        .height(15) // Fixed height
        .text_alignment(TextAlignment::Center)
        .title("Mixed Sizing")
        .render("Dynamic width, fixed height - best of both worlds!")?;
    println!("{}\n", result);

    Ok(())
}
