//! Title Color Examples
//!
//! This example demonstrates various creative uses of the title_color feature,
//! showing how to combine title colors with border colors and background colors
//! to create visually appealing terminal boxes.

use boxen::{BorderStyle, builder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Force enable colors (the colored crate disables them in non-TTY environments)
    colored::control::set_override(true);

    println!("=== Boxen Title Color Examples ===\n");

    // Example 1: Error message with red title
    println!("1. Error Message:");
    let error = builder()
        .title("ERROR")
        .title_color("red")
        .border_color("bright_red")
        .border_style(BorderStyle::Bold)
        .padding(1)
        .render("File not found: config.toml")?;
    println!("{}\n", error);

    // Example 2: Success message with green title
    println!("2. Success Message:");
    let success = builder()
        .title("SUCCESS")
        .title_color("bright_green")
        .border_color("green")
        .border_style(BorderStyle::Double)
        .padding(1)
        .render("Build completed successfully!")?;
    println!("{}\n", success);

    // Example 3: Info with contrasting colors
    println!("3. Info Box:");
    let info = builder()
        .title("INFO")
        .title_color("white")
        .border_color("blue")
        .background_color("bright_blue")
        .border_style(BorderStyle::Round)
        .padding(1)
        .render("System update available")?;
    println!("{}\n", info);

    // Example 4: Warning with yellow/orange theme
    println!("4. Warning:");
    let warning = builder()
        .title("WARNING")
        .title_color("#FFA500") // Orange hex
        .border_color("yellow")
        .border_style(BorderStyle::Bold)
        .padding(1)
        .render("Disk space running low")?;
    println!("{}\n", warning);

    // Example 5: Fancy announcement with RGB colors
    println!("5. Fancy Box:");
    let fancy = builder()
        .title("ANNOUNCEMENT")
        .title_color((255, 255, 255)) // White RGB
        .border_color((128, 0, 128)) // Purple RGB
        .border_style(BorderStyle::Double)
        .padding(2)
        .render("New feature released!")?;
    println!("{}\n", fancy);

    // Example 6: Subtle design with dim border
    println!("6. Subtle Design:");
    let subtle = builder()
        .title("Note")
        .title_color("cyan")
        .border_color("blue")
        .dim_border(true)
        .border_style(BorderStyle::Single)
        .padding(1)
        .render("This is a subtle note with a dimmed border")?;
    println!("{}\n", subtle);

    // Example 7: Colorful status indicators
    println!("7. Status Indicators:");

    let statuses = [
        ("ONLINE", "green", "All systems operational"),
        ("DEGRADED", "yellow", "Some services experiencing issues"),
        ("OFFLINE", "red", "System maintenance in progress"),
    ];

    for (status, color, message) in &statuses {
        let box_output = builder()
            .title(*status)
            .title_color(*color)
            .border_color(*color)
            .border_style(BorderStyle::Round)
            .padding(1)
            .width(45)
            .render(message)?;
        println!("{}", box_output);
    }

    println!("\n8. Mixed Color Formats:");
    let mixed = builder()
        .title("Mixed Formats")
        .title_color("#FF1493") // Hex: Deep Pink
        .border_color((0, 191, 255)) // RGB: Deep Sky Blue
        .border_style(BorderStyle::Double)
        .padding(1)
        .render("Title uses hex, border uses RGB tuple")?;
    println!("{}\n", mixed);

    Ok(())
}
