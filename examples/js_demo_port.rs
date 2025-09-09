/// Direct Rust port of the JavaScript boxen demo
/// Demonstrates all the features shown in the original TypeScript/JavaScript example
use ::boxen::{
    BorderChars, BorderStyle, BoxenOptions, Color, FullscreenMode, Spacing, TextAlignment,
    TitleAlignment, boxen, terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic box with yellow border
    println!(
        "\n\n{}\n",
        boxen(
            "unicorn",
            Some(BoxenOptions {
                padding: Spacing::from(1),
                margin: Spacing::from(1),
                border_color: Some(Color::Named("yellow".to_string())),
                ..Default::default()
            })
        )?
    );

    // Double border style with yellow border
    println!(
        "\n\n{}\n",
        boxen(
            "unicorn",
            Some(BoxenOptions {
                padding: Spacing::from(1),
                margin: Spacing::from(1),
                border_color: Some(Color::Named("yellow".to_string())),
                border_style: BorderStyle::Double,
                ..Default::default()
            })
        )?
    );

    // Double border with hex color
    println!(
        "\n\n{}\n",
        boxen(
            "unicorn",
            Some(BoxenOptions {
                padding: Spacing::from(1),
                margin: Spacing::from(1),
                border_color: Some(Color::Hex("#eebbaa".to_string())),
                border_style: BorderStyle::Double,
                ..Default::default()
            })
        )?
    );

    // Black text with pink border and cyan background
    println!(
        "\n\n{}\n",
        boxen(
            "unicorn",
            Some(BoxenOptions {
                padding: Spacing::from(1),
                margin: Spacing::from(1),
                border_color: Some(Color::Hex("#ffc0cb".to_string())),
                background_color: Some(Color::Hex("#00ffff".to_string())),
                border_style: BorderStyle::Double,
                ..Default::default()
            })
        )?
    );

    // Custom border characters (equivalent to the JS custom border style)
    println!(
        "\n\n{}\n",
        boxen(
            "unicorn",
            Some(BoxenOptions {
                padding: Spacing::from(1),
                margin: Spacing::from(1),
                border_color: Some(Color::Named("yellow".to_string())),
                background_color: Some(Color::Named("magenta".to_string())),
                border_style: BorderStyle::Custom(BorderChars {
                    top_left: '+',
                    top_right: '+',
                    bottom_left: '+',
                    bottom_right: '+',
                    top: '-',
                    bottom: '-',
                    left: '|',
                    right: '|',
                }),
                ..Default::default()
            })
        )?
    );

    // Long unbreakable text with left alignment
    let sentences = "Unbreakable_text_because_it_has_no_spaces ".repeat(5);
    println!(
        "\n\n{}\n",
        boxen(
            &sentences,
            Some(BoxenOptions {
                text_alignment: TextAlignment::Left,
                ..Default::default()
            })
        )?
    );

    // Center alignment
    println!(
        "\n\n{}\n",
        boxen(
            &sentences,
            Some(BoxenOptions {
                text_alignment: TextAlignment::Center,
                ..Default::default()
            })
        )?
    );

    // Right alignment with custom padding
    println!(
        "\n\n{}\n",
        boxen(
            &sentences,
            Some(BoxenOptions {
                text_alignment: TextAlignment::Right,
                padding: Spacing {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 0,
                },
                ..Default::default()
            })
        )?
    );

    // Very long word that exceeds terminal width
    let terminal_width = terminal::get_terminal_width();
    let long_word = "x".repeat(terminal_width + 20);
    println!(
        "\n\n{}\n",
        boxen(
            &long_word,
            Some(BoxenOptions {
                text_alignment: TextAlignment::Center,
                ..Default::default()
            })
        )?
    );

    // Box with title
    let title = "Beautiful title";
    println!(
        "\n\n{}\n",
        boxen(
            "This box has a nice title",
            Some(BoxenOptions {
                title: Some(title.to_string()),
                ..Default::default()
            })
        )?
    );

    // Box with centered title
    println!(
        "\n\n{}\n",
        boxen(
            "This box has a centered title",
            Some(BoxenOptions {
                title: Some(title.to_string()),
                title_alignment: TitleAlignment::Center,
                ..Default::default()
            })
        )?
    );

    // Fixed width boxes
    println!(
        "\n\n{}\n",
        boxen(
            "This box has fixed width of 20",
            Some(BoxenOptions {
                width: Some(20),
                ..Default::default()
            })
        )?
    );

    println!(
        "\n\n{}\n",
        boxen(
            "This box has fixed width of 50",
            Some(BoxenOptions {
                width: Some(50),
                ..Default::default()
            })
        )?
    );

    // Fixed height boxes
    println!(
        "\n\n{}\n",
        boxen(
            "This box has fixed height of 8",
            Some(BoxenOptions {
                height: Some(8),
                ..Default::default()
            })
        )?
    );

    println!(
        "\n\n{}\n",
        boxen(
            "This box has fixed height of 10",
            Some(BoxenOptions {
                height: Some(10),
                padding: Spacing::from(2),
                ..Default::default()
            })
        )?
    );

    // Fixed width and height
    println!(
        "\n\n{}\n",
        boxen(
            "This box has fixed height of 8 and width of 15",
            Some(BoxenOptions {
                height: Some(8),
                width: Some(15),
                ..Default::default()
            })
        )?
    );

    // Fullscreen mode
    println!(
        "\n\n{}\n",
        boxen(
            "This box is in fullscreen !",
            Some(BoxenOptions {
                fullscreen: Some(FullscreenMode::Auto),
                ..Default::default()
            })
        )?
    );

    // Custom fullscreen - full width, half height
    println!(
        "\n\n{}\n",
        boxen(
            "This box is in full-width and half-height !",
            Some(BoxenOptions {
                fullscreen: Some(FullscreenMode::Custom(|w, h| (w, h / 2))),
                ..Default::default()
            })
        )?
    );

    // Custom fullscreen - half width, full height
    println!(
        "\n\n{}\n",
        boxen(
            "And this one is in half-width and full-height !",
            Some(BoxenOptions {
                fullscreen: Some(FullscreenMode::Custom(|w, h| (w / 2, h))),
                ..Default::default()
            })
        )?
    );

    Ok(())
}
