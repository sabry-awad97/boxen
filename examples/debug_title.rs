use boxen::builder;

fn main() {
    let input = "# Content\n**bold**";

    let result = builder()
        .title("Box Title")
        .markdown()
        .render(input)
        .unwrap();

    println!("Result:\n{}", result);
    println!("\n\nContains 'Box Title': {}", result.contains("Box Title"));
    println!("Contains 'Title': {}", result.contains("Title"));
}
