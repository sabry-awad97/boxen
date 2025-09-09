/// Demonstration of the main boxen API functions
use ::boxen::{
    BorderStyle, BoxenOptions, Color, Float, Spacing, TextAlignment, TitleAlignment, boxen,
    builder, double_box, round_box, simple_box,
};

fn main() {
    println!("=== Main boxen() function ===");

    // Basic usage
    let basic = boxen("Hello, World!", None).unwrap();
    println!("{}", basic);

    // With options
    let options = BoxenOptions {
        border_style: BorderStyle::Double,
        padding: Spacing::from(1),
        text_alignment: TextAlignment::Center,
        title: Some("Greeting".to_string()),
        border_color: Some(Color::Named("blue".to_string())),
        ..Default::default()
    };
    let with_options = boxen("Hello with options!", Some(options)).unwrap();
    println!("{}", with_options);

    println!("\n=== Builder pattern ===");

    let builder_result = builder()
        .border_style(BorderStyle::Round)
        .padding(2)
        .margin(1)
        .text_alignment(TextAlignment::Center)
        .title("Builder Demo")
        .title_alignment(TitleAlignment::Center)
        .width(40)
        .border_color("green")
        .background_color("#f0f0f0")
        .float(Float::Center)
        .render("Built with fluent interface!")
        .unwrap();
    println!("{}", builder_result);

    println!("\n=== Convenience functions ===");

    println!("Simple box:");
    println!("{}", simple_box("Simple and clean"));

    println!("\nDouble box:");
    println!("{}", double_box("Double borders"));

    println!("\nRound box:");
    println!("{}", round_box("Rounded corners"));

    println!("\n=== Complex example ===");

    let complex = builder()
        .border_style(BorderStyle::Bold)
        .spacing(2) // Sets both padding and margin
        .colors("red", "yellow") // Sets both border and background
        .size(50, 12) // Sets both width and height
        .center_all() // Centers everything
        .title("Complex Configuration")
        .render("This demonstrates a complex\nconfiguration with multiple\nlines of text and various\nstyling options.")
        .unwrap();
    println!("{}", complex);
}
