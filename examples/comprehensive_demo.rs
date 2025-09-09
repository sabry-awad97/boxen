/// Comprehensive demonstration of all boxen features and usage patterns
use ::boxen::{
    BorderStyle, BoxenOptions, Color, Float, FullscreenMode, Spacing, TextAlignment,
    TitleAlignment, boxen, builder, double_box, round_box, simple_box,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Comprehensive Boxen Demo ===\n");

    // 1. Basic usage patterns
    demonstrate_basic_usage()?;

    // 2. Border styles showcase
    demonstrate_border_styles()?;

    // 3. Text alignment and formatting
    demonstrate_text_formatting()?;

    // 4. Spacing and dimensions
    demonstrate_spacing_and_dimensions()?;

    // 5. Colors and styling
    demonstrate_colors_and_styling()?;

    // 6. Titles and positioning
    demonstrate_titles_and_positioning()?;

    // 7. Advanced features
    demonstrate_advanced_features()?;

    // 8. Builder pattern showcase
    demonstrate_builder_patterns()?;

    // 9. Real-world examples
    demonstrate_real_world_examples()?;

    println!("\n=== Demo Complete ===");
    Ok(())
}

fn demonstrate_basic_usage() -> Result<(), Box<dyn std::error::Error>> {
    println!("1. Basic Usage Patterns:");

    // Simple box
    println!("Simple box:");
    println!("{}", simple_box("Hello, World!"));

    // Double border
    println!("Double border:");
    println!("{}", double_box("Double border style"));

    // Round corners
    println!("Round corners:");
    println!("{}", round_box("Rounded corners"));

    // Using main function with options
    println!("With custom options:");
    let options = BoxenOptions {
        border_style: BorderStyle::Bold,
        padding: Spacing::from(1),
        ..Default::default()
    };
    println!("{}", boxen("Bold border with padding", Some(options))?);

    println!();
    Ok(())
}

fn demonstrate_border_styles() -> Result<(), Box<dyn std::error::Error>> {
    println!("2. Border Styles Showcase:");

    let styles = [
        ("Single", BorderStyle::Single),
        ("Double", BorderStyle::Double),
        ("Round", BorderStyle::Round),
        ("Bold", BorderStyle::Bold),
        ("Single-Double", BorderStyle::SingleDouble),
        ("Double-Single", BorderStyle::DoubleSingle),
        ("Classic", BorderStyle::Classic),
        ("None", BorderStyle::None),
    ];

    for (name, style) in styles.iter() {
        println!("{} border:", name);
        let options = BoxenOptions {
            border_style: style.clone(),
            ..Default::default()
        };
        println!(
            "{}",
            boxen(format!("{} style example", name), Some(options))?
        );
    }

    println!();
    Ok(())
}

fn demonstrate_text_formatting() -> Result<(), Box<dyn std::error::Error>> {
    println!("3. Text Alignment and Formatting:");

    let multiline_content = "Left aligned text\nCenter aligned text\nRight aligned text";

    // Left alignment
    println!("Left alignment:");
    let options = BoxenOptions {
        text_alignment: TextAlignment::Left,
        width: Some(40),
        ..Default::default()
    };
    println!("{}", boxen(multiline_content, Some(options))?);

    // Center alignment
    println!("Center alignment:");
    let options = BoxenOptions {
        text_alignment: TextAlignment::Center,
        width: Some(40),
        ..Default::default()
    };
    println!("{}", boxen(multiline_content, Some(options))?);

    // Right alignment
    println!("Right alignment:");
    let options = BoxenOptions {
        text_alignment: TextAlignment::Right,
        width: Some(40),
        ..Default::default()
    };
    println!("{}", boxen(multiline_content, Some(options))?);

    println!();
    Ok(())
}

fn demonstrate_spacing_and_dimensions() -> Result<(), Box<dyn std::error::Error>> {
    println!("4. Spacing and Dimensions:");

    // Padding examples
    println!("Different padding values:");
    let padding_examples = [
        ("No padding", Spacing::from(0)),
        ("Uniform padding (1)", Spacing::from(1)),
        ("Asymmetric padding", Spacing::from((1, 2, 3, 4))),
        ("Horizontal/Vertical", Spacing::from([2, 4])),
    ];

    for (desc, padding) in padding_examples.iter() {
        println!("{}:", desc);
        let options = BoxenOptions {
            padding: *padding,
            width: Some(35),
            ..Default::default()
        };
        println!("{}", boxen("Padding example", Some(options))?);
    }

    // Margin examples
    println!("With margins:");
    let options = BoxenOptions {
        padding: Spacing::from(1),
        margin: Spacing::from(2),
        ..Default::default()
    };
    println!("{}", boxen("Padding and margin example", Some(options))?);

    // Dimension constraints
    println!("Dimension constraints:");
    let options = BoxenOptions {
        width: Some(30),
        height: Some(8),
        text_alignment: TextAlignment::Center,
        ..Default::default()
    };
    println!(
        "{}",
        boxen(
            "This text will be constrained\nto specific dimensions\nwith center alignment",
            Some(options)
        )?
    );

    println!();
    Ok(())
}

fn demonstrate_colors_and_styling() -> Result<(), Box<dyn std::error::Error>> {
    println!("5. Colors and Styling:");

    // Named colors
    println!("Named colors:");
    let options = BoxenOptions {
        border_color: Some(Color::Named("red".to_string())),
        background_color: Some(Color::Named("yellow".to_string())),
        ..Default::default()
    };
    println!("{}", boxen("Red border, yellow background", Some(options))?);

    // Hex colors
    println!("Hex colors:");
    let options = BoxenOptions {
        border_color: Some(Color::Hex("#00ff00".to_string())),
        background_color: Some(Color::Hex("#0000ff".to_string())),
        ..Default::default()
    };
    println!("{}", boxen("Green border, blue background", Some(options))?);

    // RGB colors
    println!("RGB colors:");
    let options = BoxenOptions {
        border_color: Some(Color::Rgb(255, 0, 255)),
        background_color: Some(Color::Rgb(0, 255, 255)),
        ..Default::default()
    };
    println!(
        "{}",
        boxen("Magenta border, cyan background", Some(options))?
    );

    // Dim border
    println!("Dim border:");
    let options = BoxenOptions {
        border_color: Some(Color::Named("white".to_string())),
        dim_border: true,
        ..Default::default()
    };
    println!("{}", boxen("Dimmed white border", Some(options))?);

    println!();
    Ok(())
}

fn demonstrate_titles_and_positioning() -> Result<(), Box<dyn std::error::Error>> {
    println!("6. Titles and Positioning:");

    // Title alignments
    let title_alignments = [
        ("Left", TitleAlignment::Left),
        ("Center", TitleAlignment::Center),
        ("Right", TitleAlignment::Right),
    ];

    for (desc, alignment) in title_alignments.iter() {
        println!("{} title alignment:", desc);
        let options = BoxenOptions {
            title: Some(format!("{} Title", desc)),
            title_alignment: alignment.clone(),
            width: Some(40),
            ..Default::default()
        };
        println!("{}", boxen("Content with title", Some(options))?);
    }

    // Float positioning
    let float_positions = [
        ("Left", Float::Left),
        ("Center", Float::Center),
        ("Right", Float::Right),
    ];

    for (desc, float_pos) in float_positions.iter() {
        println!("{} float positioning:", desc);
        let options = BoxenOptions {
            float: float_pos.clone(),
            width: Some(30),
            title: Some(format!("{} Float", desc)),
            ..Default::default()
        };
        println!("{}", boxen("Positioned content", Some(options))?);
    }

    println!();
    Ok(())
}

fn demonstrate_advanced_features() -> Result<(), Box<dyn std::error::Error>> {
    println!("7. Advanced Features:");

    // Fullscreen mode
    println!("Fullscreen mode (auto):");
    let options = BoxenOptions {
        fullscreen: Some(FullscreenMode::Auto),
        title: Some("Fullscreen Demo".to_string()),
        text_alignment: TextAlignment::Center,
        ..Default::default()
    };
    println!("{}", boxen("This box fills the terminal", Some(options))?);

    // Custom fullscreen function
    println!("Custom fullscreen (75% size):");
    let custom_func =
        |width: usize, height: usize| -> (usize, usize) { (width * 3 / 4, height * 3 / 4) };
    let options = BoxenOptions {
        fullscreen: Some(FullscreenMode::Custom(custom_func)),
        title: Some("Custom Size".to_string()),
        ..Default::default()
    };
    println!("{}", boxen("Custom sized fullscreen box", Some(options))?);

    // Complex combination
    println!("Complex feature combination:");
    let options = BoxenOptions {
        border_style: BorderStyle::Double,
        padding: Spacing::from(2),
        margin: Spacing::from(1),
        text_alignment: TextAlignment::Center,
        title: Some("Complex Example".to_string()),
        title_alignment: TitleAlignment::Center,
        width: Some(50),
        height: Some(10),
        border_color: Some(Color::Named("cyan".to_string())),
        background_color: Some(Color::Named("black".to_string())),
        float: Float::Center,
        ..Default::default()
    };
    println!(
        "{}",
        boxen(
            "This demonstrates multiple\nfeatures working together\nin a complex configuration",
            Some(options)
        )?
    );

    println!();
    Ok(())
}

fn demonstrate_builder_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("8. Builder Pattern Showcase:");

    // Basic builder usage
    println!("Basic builder:");
    let result = builder()
        .border_style(BorderStyle::Round)
        .padding(1)
        .title("Builder Demo")
        .render("Built with fluent interface")?;
    println!("{}", result);

    // Convenience methods
    println!("Builder convenience methods:");
    let result = builder()
        .spacing(2) // Sets both padding and margin
        .colors("green", "yellow") // Sets both colors
        .size(45, 8) // Sets both dimensions
        .center_all() // Centers everything
        .title("Convenience Methods")
        .render("Using convenience methods\nfor easier configuration")?;
    println!("{}", result);

    // Method chaining showcase
    println!("Complex builder chain:");
    let result = builder()
        .border_style(BorderStyle::Bold)
        .padding(3)
        .margin(2)
        .text_alignment(TextAlignment::Right)
        .title("Method Chaining")
        .title_alignment(TitleAlignment::Left)
        .width(55)
        .border_color("red")
        .background_color("#f0f0f0")
        .dim_border(false)
        .float(Float::Center)
        .render("This demonstrates extensive\nmethod chaining with the\nbuilder pattern")?;
    println!("{}", result);

    println!();
    Ok(())
}

fn demonstrate_real_world_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("9. Real-World Usage Examples:");

    // CLI tool output
    println!("CLI tool success message:");
    let result = builder()
        .border_style(BorderStyle::Round)
        .border_color("green")
        .padding(1)
        .title("✓ Success")
        .title_alignment(TitleAlignment::Left)
        .render("Operation completed successfully!\nAll files have been processed.")?;
    println!("{}", result);

    // Error message
    println!("CLI tool error message:");
    let result = builder()
        .border_style(BorderStyle::Bold)
        .border_color("red")
        .padding(1)
        .title("✗ Error")
        .title_alignment(TitleAlignment::Left)
        .render("Failed to process file: permission denied\nPlease check file permissions and try again.")?;
    println!("{}", result);

    // Information panel
    println!("Information panel:");
    let result = builder()
        .border_style(BorderStyle::Double)
        .border_color("blue")
        .background_color("white")
        .padding(2)
        .title("ℹ Information")
        .title_alignment(TitleAlignment::Center)
        .width(60)
        .text_alignment(TextAlignment::Center)
        .render("System Status: Online\nUptime: 24 days, 3 hours\nMemory Usage: 45%\nDisk Space: 78% available")?;
    println!("{}", result);

    // Configuration display
    println!("Configuration display:");
    let result = builder()
        .border_style(BorderStyle::Single)
        .border_color("cyan")
        .padding(1)
        .margin(1)
        .title("Configuration")
        .title_alignment(TitleAlignment::Center)
        .width(50)
        .render("Server: localhost:8080\nDatabase: postgresql://localhost:5432\nEnvironment: development\nDebug Mode: enabled")?;
    println!("{}", result);

    // Progress indicator
    println!("Progress indicator:");
    let result = builder()
        .border_style(BorderStyle::Round)
        .border_color("yellow")
        .padding(1)
        .title("⏳ Processing")
        .title_alignment(TitleAlignment::Left)
        .width(40)
        .render("Progress: ████████░░ 80%\nETA: 2 minutes remaining")?;
    println!("{}", result);

    println!();
    Ok(())
}
