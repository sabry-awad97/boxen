/// Performance demonstration and benchmarking for boxen
use ::boxen::{
    BorderStyle, BoxenOptions, Color, Spacing, TextAlignment, TitleAlignment, boxen, builder,
};
use std::time::Instant;

fn main() {
    println!("=== Boxen Performance Demo ===\n");

    // 1. Large text performance
    benchmark_large_text();

    // 2. Many lines performance
    benchmark_many_lines();

    // 3. Complex configuration performance
    benchmark_complex_configuration();

    // 4. Unicode handling performance
    benchmark_unicode_handling();

    // 5. Repeated rendering performance
    benchmark_repeated_rendering();

    // 6. Memory usage demonstration
    demonstrate_memory_efficiency();

    // 7. Scaling tests
    benchmark_scaling();

    println!("\n=== Performance Demo Complete ===");
}

fn benchmark_large_text() {
    println!("1. Large Text Performance:");

    let sizes = [100, 500, 1000, 2000];

    for &size in &sizes {
        let large_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(size);

        let start = Instant::now();
        let result = boxen(
            &large_text,
            Some(BoxenOptions {
                width: Some(80),
                text_alignment: TextAlignment::Left,
                ..Default::default()
            }),
        );
        let duration = start.elapsed();

        match result {
            Ok(output) => {
                let lines = output.lines().count();
                println!("  {size} repetitions: {duration:?} ({lines} lines)");
            }
            Err(e) => {
                println!("  {size} repetitions: Error - {e}");
            }
        }
    }
    println!();
}

fn benchmark_many_lines() {
    println!("2. Many Lines Performance:");

    let line_counts = [100, 500, 1000, 2000];

    for &count in &line_counts {
        let many_lines = (0..count)
            .map(|i| format!("Line number {i} with some content"))
            .collect::<Vec<_>>()
            .join("\n");

        let start = Instant::now();
        let result = boxen(
            &many_lines,
            Some(BoxenOptions {
                width: Some(80),
                height: Some(50), // Limit output height
                text_alignment: TextAlignment::Left,
                ..Default::default()
            }),
        );
        let duration = start.elapsed();

        match result {
            Ok(output) => {
                let output_lines = output.lines().count();
                println!("  {count} input lines: {duration:?} ({output_lines} output lines)");
            }
            Err(e) => {
                println!("  {count} input lines: Error - {e}");
            }
        }
    }
    println!();
}

fn benchmark_complex_configuration() {
    println!("3. Complex Configuration Performance:");

    let iterations = [10, 50, 100, 200];

    for &iter_count in &iterations {
        let start = Instant::now();

        for i in 0..iter_count {
            if let Err(err) = builder()
                .border_style(BorderStyle::Double)
                .padding(2)
                .margin(1)
                .text_alignment(TextAlignment::Center)
                .title(format!("Complex Config {i}"))
                .title_alignment(TitleAlignment::Center)
                .width(60)
                .height(12)
                .border_color("red")
                .background_color("#ffffff")
                .dim_border(true)
                .render("This is a complex configuration test\nwith multiple features enabled\nto measure performance impact")
            {
                println!("  Error in iteration {i}: {err}");
                break;
            }
        }

        let duration = start.elapsed();
        let avg_duration = duration / u32::try_from(iter_count).unwrap_or(1);
        println!("  {iter_count} iterations: {duration:?} total, {avg_duration:?} average");
    }
    println!();
}

fn benchmark_unicode_handling() {
    println!("4. Unicode Handling Performance:");

    let unicode_texts = [
        ("Basic ASCII", "Hello World! ".repeat(100)),
        ("Mixed Unicode", "Hello 世界! 🌍🚀✨ ".repeat(100)),
        ("Heavy Unicode", "🌍🌎🌏🚀✨🎉🎊🎈🎁🎀 你好世界 مرحبا بالعالم ".repeat(100)),
        ("Emoji Heavy", "😀😃😄😁😆😅😂🤣😊😇🙂🙃😉😌😍🥰😘😗😙😚😋😛😝😜🤪🤨🧐🤓😎🤩🥳😏😒😞😔😟😕🙁☹️😣😖😫😩🥺😢😭😤😠😡🤬🤯😳🥵🥶😱😨😰😥😓🤗🤔🤭🤫🤥😶😐😑😬🙄😯😦😧😮😲🥱😴🤤😪😵🤐🥴🤢🤮🤧😷🤒🤕🤑🤠😈👿👹👺🤡💩👻💀☠️👽👾🤖🎃😺😸😹😻😼😽🙀😿😾".repeat(20)),
    ];

    for (desc, text) in &unicode_texts {
        let start = Instant::now();
        let result = boxen(
            text,
            Some(BoxenOptions {
                width: Some(80),
                text_alignment: TextAlignment::Center,
                border_style: BorderStyle::Round,
                ..Default::default()
            }),
        );
        let duration = start.elapsed();

        match result {
            Ok(output) => {
                let lines = output.lines().count();
                println!("  {desc}: {duration:?} ({lines} lines)");
            }
            Err(e) => {
                println!("  {desc}: Error - {e}");
            }
        }
    }
    println!();
}

fn benchmark_repeated_rendering() {
    println!("5. Repeated Rendering Performance:");

    let content = "Repeated rendering performance test";
    let configurations = [
        ("Simple", BoxenOptions::default()),
        (
            "With Padding",
            BoxenOptions {
                padding: Spacing::from(2),
                ..Default::default()
            },
        ),
        (
            "With Colors",
            BoxenOptions {
                border_color: Some(Color::Named("red".to_string())),
                background_color: Some(Color::Named("yellow".to_string())),
                ..Default::default()
            },
        ),
        (
            "Complex",
            BoxenOptions {
                border_style: BorderStyle::Double,
                padding: Spacing::from(2),
                margin: Spacing::from(1),
                text_alignment: TextAlignment::Center,
                title: Some("Performance Test".to_string()),
                width: Some(50),
                border_color: Some(Color::Hex("#ff0000".to_string())),
                ..Default::default()
            },
        ),
    ];

    for (desc, config) in &configurations {
        let start = Instant::now();

        for _ in 0..100 {
            if let Err(err) = boxen(content, Some(config.clone())) {
                println!("  {desc}: Error - {err}");
                break;
            }
        }

        let duration = start.elapsed();
        let avg_duration = duration / 100;
        println!("  {desc} (100x): {duration:?} total, {avg_duration:?} average");
    }
    println!();
}

fn demonstrate_memory_efficiency() {
    println!("6. Memory Efficiency Demonstration:");

    // Test memory usage with different text sizes
    let text_sizes = [
        ("Small", "Short text"),
        ("Medium", &"Medium length text. ".repeat(50)),
        ("Large", &"Large text content. ".repeat(500)),
        ("Very Large", &"Very large text content. ".repeat(2000)),
    ];

    for (desc, text) in &text_sizes {
        let start = Instant::now();

        // Create and drop multiple boxes to test memory efficiency
        for _ in 0..10 {
            let result = boxen(
                text,
                Some(BoxenOptions {
                    width: Some(80),
                    padding: Spacing::from(1),
                    ..Default::default()
                }),
            );

            if let Ok(output) = result {
                // Force the output to be used to prevent optimization
                let _length = output.len();
            }
        }

        let duration = start.elapsed();
        println!("  {desc} text (10x): {duration:?}");
    }
    println!();
}

fn benchmark_scaling() {
    println!("7. Scaling Performance Tests:");

    // Test how performance scales with different parameters
    println!("  Width scaling:");
    let widths = [20, 40, 80, 120, 160];
    for &width in &widths {
        let start = Instant::now();
        let result = boxen(
            "Width scaling test content",
            Some(BoxenOptions {
                width: Some(width),
                ..Default::default()
            }),
        );
        let duration = start.elapsed();

        match result {
            Ok(_) => println!("    Width {width}: {duration:?}"),
            Err(e) => println!("    Width {width}: Error - {e}"),
        }
    }

    println!("  Height scaling:");
    let heights = [5, 10, 20, 30, 50];
    let long_content = (0..100)
        .map(|i| format!("Line {i}"))
        .collect::<Vec<_>>()
        .join("\n");

    for &height in &heights {
        let start = Instant::now();
        let result = boxen(
            &long_content,
            Some(BoxenOptions {
                height: Some(height),
                width: Some(80),
                ..Default::default()
            }),
        );
        let duration = start.elapsed();

        match result {
            Ok(_) => println!("    Height {height}: {duration:?}"),
            Err(e) => println!("    Height {height}: Error - {e}"),
        }
    }

    println!("  Padding scaling:");
    let paddings = [0, 1, 2, 5, 10];
    for &padding in &paddings {
        let start = Instant::now();
        let result = boxen(
            "Padding scaling test",
            Some(BoxenOptions {
                padding: Spacing::from(padding),
                width: Some(80),
                ..Default::default()
            }),
        );
        let duration = start.elapsed();

        match result {
            Ok(_) => println!("    Padding {padding}: {duration:?}"),
            Err(e) => println!("    Padding {padding}: Error - {e}"),
        }
    }

    println!();
}
