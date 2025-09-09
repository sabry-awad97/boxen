use ::boxen::{BorderStyle, BoxenOptions, Color as BoxenColor, Spacing, boxen};
use chrono::prelude::*;
use colored::Colorize;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// Spinner frames for the dots animation
const DOTS_FRAMES: &[&str] = &[
    "🕛 ", "🕚 ", "🕙 ", "🕘 ", "🕗 ", "🕖 ", "🕕 ", "🕔 ", "🕓 ", "🕒 ", "🕑 ", "🕐 ",
];
const DOTS_INTERVAL: u64 = 100; // milliseconds

/// ANSI escape sequences
const ESC: &str = "\x1b[";
const CLEAR_SCREEN: &str = "2J";
const CURSOR_HOME: &str = "H";
const HIDE_CURSOR: &str = "?25l";
const SHOW_CURSOR: &str = "?25h";

/// Initialize terminal - hide cursor and clear screen once
fn init_terminal() {
    print!(
        "{}{}{}{}{}{}",
        ESC, CLEAR_SCREEN, ESC, CURSOR_HOME, ESC, HIDE_CURSOR
    );
    io::stdout().flush().unwrap();
}

/// Move cursor to home position without clearing
fn move_to_home() {
    print!("{}{}", ESC, CURSOR_HOME);
    io::stdout().flush().unwrap();
}

/// Show cursor on exit
fn show_cursor() {
    print!("{}{}", ESC, SHOW_CURSOR);
    io::stdout().flush().unwrap();
}

/// Get current time formatted with colors
fn get_colored_time(frame_index: usize) -> String {
    let now = Local::now();

    // Format with colors using colored crate and chrono
    let mut time = String::new();
    time.push_str(&format!("{:02}:", now.hour()).bright_yellow().to_string());
    time.push_str(&format!("{:02}:", now.minute()).bright_green().to_string());
    time.push_str(&format!("{:02} ", now.second()).bright_red().to_string());
    time.push_str(
        &now.format("%a %b %d %Y ")
            .to_string()
            .bright_blue()
            .to_string(),
    );
    time.push_str(&DOTS_FRAMES[frame_index].magenta().to_string());

    time
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🕐 Boxen-rs Clock Demo - Press Ctrl+C to exit");
    thread::sleep(Duration::from_millis(1000));

    // Initialize terminal once
    init_terminal();

    // Set up signal handler to show cursor on exit
    ctrlc::set_handler(move || {
        show_cursor();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // Configure boxen options to match TypeScript example
    let options = BoxenOptions {
        padding: Spacing::from(1),
        margin: Spacing::from(1),
        border_style: BorderStyle::Round,
        border_color: Some(BoxenColor::Named("yellow".to_string())),
        ..Default::default()
    };

    let mut frame_index = 0;

    loop {
        // Get colored time string
        let time_display = get_colored_time(frame_index);

        // Create boxen output
        let boxed_time = boxen(&time_display, Some(options.clone()))?;

        // Move cursor to home and overwrite previous content
        move_to_home();
        print!("{}", boxed_time);
        io::stdout().flush().unwrap();

        // Update frame index
        frame_index = (frame_index + 1) % DOTS_FRAMES.len();

        // Wait for next frame
        thread::sleep(Duration::from_millis(DOTS_INTERVAL));
    }
}
