use ::boxen::{BorderStyle, BoxenOptions, Color, Spacing, TextAlignment, TitleAlignment, boxen};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Boxen Color Integration Demo ===\n");

    // Basic border color
    println!("1. Border Color (Red):");
    let result = boxen(
        "Hello, World!",
        Some(BoxenOptions {
            border_color: Some(Color::Named("red".to_string())),
            ..Default::default()
        }),
    )?;
    println!("{}\n", result);

    // Background color
    println!("2. Background Color (Blue):");
    let result = boxen(
        "Hello, World!",
        Some(BoxenOptions {
            background_color: Some(Color::Named("blue".to_string())),
            ..Default::default()
        }),
    )?;
    println!("{}\n", result);

    // Both border and background colors
    println!("3. Border (Green) + Background (Yellow):");
    let result = boxen(
        "Hello, World!",
        Some(BoxenOptions {
            border_color: Some(Color::Named("green".to_string())),
            background_color: Some(Color::Named("yellow".to_string())),
            ..Default::default()
        }),
    )?;
    println!("{}\n", result);

    // Dim border
    println!("4. Dim Border:");
    let result = boxen(
        "Hello, World!",
        Some(BoxenOptions {
            border_color: Some(Color::Named("white".to_string())),
            dim_border: true,
            ..Default::default()
        }),
    )?;
    println!("{}\n", result);

    // Hex colors
    println!("5. Hex Colors (Border: #FF0000, Background: #00FF00):");
    let result = boxen(
        "Hello, World!",
        Some(BoxenOptions {
            border_color: Some(Color::Hex("#FF0000".to_string())),
            background_color: Some(Color::Hex("#00FF00".to_string())),
            ..Default::default()
        }),
    )?;
    println!("{}\n", result);

    // RGB colors
    println!("6. RGB Colors (Border: 255,0,255, Background: 0,255,255):");
    let result = boxen(
        "Hello, World!",
        Some(BoxenOptions {
            border_color: Some(Color::Rgb(255, 0, 255)),
            background_color: Some(Color::Rgb(0, 255, 255)),
            ..Default::default()
        }),
    )?;
    println!("{}\n", result);

    // Colors with title
    println!("7. Colors with Title:");
    let result = boxen(
        "Content goes here",
        Some(BoxenOptions {
            title: Some("Colored Title".to_string()),
            title_alignment: TitleAlignment::Center,
            border_color: Some(Color::Named("cyan".to_string())),
            background_color: Some(Color::Named("magenta".to_string())),
            width: Some(25),
            ..Default::default()
        }),
    )?;
    println!("{}\n", result);

    // Colors with padding
    println!("8. Colors with Padding:");
    let result = boxen(
        "Padded content",
        Some(BoxenOptions {
            border_color: Some(Color::Named("blue".to_string())),
            background_color: Some(Color::Named("white".to_string())),
            padding: Spacing::from(1),
            ..Default::default()
        }),
    )?;
    println!("{}\n", result);

    // Colors with different border styles
    println!("9. Colors with Double Border:");
    let result = boxen(
        "Double border",
        Some(BoxenOptions {
            border_style: BorderStyle::Double,
            border_color: Some(Color::Named("red".to_string())),
            background_color: Some(Color::Named("black".to_string())),
            ..Default::default()
        }),
    )?;
    println!("{}\n", result);

    // Colors with no border (background only)
    println!("10. Background Color Only (No Border):");
    let result = boxen(
        "No border, just background",
        Some(BoxenOptions {
            border_style: BorderStyle::None,
            background_color: Some(Color::Named("green".to_string())),
            padding: Spacing::from(1),
            ..Default::default()
        }),
    )?;
    println!("{}\n", result);

    // Multiline content with colors
    println!("11. Multiline Content with Colors:");
    let result = boxen(
        "Line 1\nLine 2\nLine 3",
        Some(BoxenOptions {
            border_color: Some(Color::Named("yellow".to_string())),
            background_color: Some(Color::Named("blue".to_string())),
            text_alignment: TextAlignment::Center,
            width: Some(20),
            ..Default::default()
        }),
    )?;
    println!("{}\n", result);

    println!("=== Demo Complete ===");
    Ok(())
}
