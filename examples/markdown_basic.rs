use boxen::builder;

fn main() {
    // Enable markdown parsing
    let help = builder()
        .title("Help")
        .markdown() // Enable markdown parsing
        .render(
            r#"
# Commands

**create** - Create a new item
**delete** - Remove an item

Use `--help` for more info
"#,
        )
        .unwrap();

    println!("{}", help);
}
