use ::boxen::{BoxenOptions, FullscreenMode, Spacing, TextAlignment, TitleAlignment, boxen};

fn main() {
    println!("=== Fullscreen Mode Demo ===\n");

    // Basic fullscreen mode
    println!("1. Basic fullscreen mode:");
    let options = BoxenOptions {
        fullscreen: Some(FullscreenMode::Auto),
        ..Default::default()
    };
    let result = boxen("This box fills the entire terminal!", Some(options)).unwrap();
    println!("{}\n", result);

    // Fullscreen with title and center alignment
    println!("2. Fullscreen with title and center alignment:");
    let options = BoxenOptions {
        fullscreen: Some(FullscreenMode::Auto),
        title: Some("Fullscreen Demo".to_string()),
        title_alignment: TitleAlignment::Center,
        text_alignment: TextAlignment::Center,
        ..Default::default()
    };
    let result = boxen(
        "Content is centered\nwithin the fullscreen box",
        Some(options),
    )
    .unwrap();
    println!("{}\n", result);

    // Custom fullscreen mode (half size)
    println!("3. Custom fullscreen mode (3/4 size):");
    let custom_func =
        |width: usize, height: usize| -> (usize, usize) { (width * 3 / 4, height * 3 / 4) };
    let options = BoxenOptions {
        fullscreen: Some(FullscreenMode::Custom(custom_func)),
        title: Some("Custom Size".to_string()),
        padding: Spacing::from(1),
        ..Default::default()
    };
    let result = boxen("This box is 3/4 the size of the terminal", Some(options)).unwrap();
    println!("{}\n", result);

    // Fullscreen with margins
    println!("4. Fullscreen with margins:");
    let options = BoxenOptions {
        fullscreen: Some(FullscreenMode::Auto),
        margin: Spacing {
            top: 2,
            right: 4,
            bottom: 2,
            left: 4,
        },
        title: Some("With Margins".to_string()),
        ..Default::default()
    };
    let result = boxen("This fullscreen box has margins", Some(options)).unwrap();
    println!("{}", result);
}
