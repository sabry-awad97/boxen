/// Criterion-based benchmarks for boxen library
///
/// This benchmark suite uses the Criterion framework for sophisticated
/// statistical analysis of performance. It complements the custom allocation
/// benchmarks by providing:
/// - Statistical analysis with confidence intervals
/// - Regression detection
/// - HTML reports with plots
/// - Comparison across runs
///
/// Run with: cargo bench --bench criterion_benchmarks
///
/// View HTML reports in: target/criterion/
use ::boxen::{BorderStyle, BoxenOptions, Spacing, TextAlignment, TitleAlignment, boxen, builder};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

// ============================================================================
// Basic Rendering Benchmarks
// ============================================================================

fn bench_simple_box(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_box");

    group.bench_function("minimal", |b| {
        b.iter(|| {
            let _ = boxen(black_box("Hello, World!"), None);
        });
    });

    group.bench_function("with_default_options", |b| {
        b.iter(|| {
            let _ = boxen(black_box("Hello, World!"), Some(BoxenOptions::default()));
        });
    });

    group.finish();
}

fn bench_border_styles(c: &mut Criterion) {
    let mut group = c.benchmark_group("border_styles");

    let styles = vec![
        ("single", BorderStyle::Single),
        ("double", BorderStyle::Double),
        ("round", BorderStyle::Round),
        ("bold", BorderStyle::Bold),
        ("single_double", BorderStyle::SingleDouble),
        ("double_single", BorderStyle::DoubleSingle),
        ("classic", BorderStyle::Classic),
    ];

    for (name, style) in styles {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &style,
            |b, style: &BorderStyle| {
                b.iter(|| {
                    let _ = boxen(
                        black_box("Test content"),
                        Some(BoxenOptions {
                            border_style: style.clone(),
                            ..Default::default()
                        }),
                    );
                });
            },
        );
    }

    group.finish();
}

fn bench_padding_variations(c: &mut Criterion) {
    let mut group = c.benchmark_group("padding");

    for padding in [0, 1, 2, 3, 5, 10] {
        group.bench_with_input(
            BenchmarkId::from_parameter(padding),
            &padding,
            |b, &padding| {
                b.iter(|| {
                    let _ = boxen(
                        black_box("Test content"),
                        Some(BoxenOptions {
                            padding: Spacing::from(padding),
                            ..Default::default()
                        }),
                    );
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Content Size Benchmarks
// ============================================================================

fn bench_content_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("content_size");

    // Single line with varying lengths
    let sizes = vec![10, 50, 100, 200, 500, 1000];

    for size in sizes {
        let content = "x".repeat(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::new("single_line", size),
            &content,
            |b, content| {
                b.iter(|| {
                    let _ = boxen(black_box(content), None);
                });
            },
        );
    }

    group.finish();
}

fn bench_multiline_content(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiline");

    let line_counts = vec![1, 5, 10, 20, 50, 100];

    for line_count in line_counts {
        let content = vec!["Line of text"; line_count].join("\n");
        group.throughput(Throughput::Elements(line_count as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(line_count),
            &content,
            |b, content| {
                b.iter(|| {
                    let _ = boxen(black_box(content), None);
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Unicode and Special Characters
// ============================================================================

fn bench_unicode_content(c: &mut Criterion) {
    let mut group = c.benchmark_group("unicode");

    let test_cases = vec![
        ("ascii", "Hello, World! This is ASCII text."),
        ("emoji", "🌍🌎🌏 Hello 🚀✨🎉 World! 🎈🎊🎁"),
        ("mixed", "Unicode: 你好世界 Émojis: àáâãäåæçèéêë 🌟"),
        ("cjk", "你好世界 こんにちは世界 안녕하세요 세계"),
        ("rtl", "مرحبا بالعالم שלום עולם"),
    ];

    for (name, content) in test_cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), &content, |b, content| {
            b.iter(|| {
                let _ = boxen(black_box(content), None);
            });
        });
    }

    group.finish();
}

// ============================================================================
// Text Alignment Benchmarks
// ============================================================================

fn bench_text_alignment(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_alignment");

    let alignments = vec![
        ("left", TextAlignment::Left),
        ("center", TextAlignment::Center),
        ("right", TextAlignment::Right),
    ];

    let content = "Test content for alignment";

    for (name, alignment) in alignments {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &alignment,
            |b, alignment: &TextAlignment| {
                b.iter(|| {
                    let _ = boxen(
                        black_box(content),
                        Some(BoxenOptions {
                            text_alignment: alignment.clone(),
                            width: Some(50),
                            ..Default::default()
                        }),
                    );
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Builder Pattern Benchmarks
// ============================================================================

fn bench_builder_pattern(c: &mut Criterion) {
    let mut group = c.benchmark_group("builder");

    group.bench_function("minimal_builder", |b| {
        b.iter(|| {
            let _ = builder().render(black_box("Test content"));
        });
    });

    group.bench_function("full_builder", |b| {
        b.iter(|| {
            let _ = builder()
                .border_style(BorderStyle::Double)
                .padding(2)
                .margin(1)
                .text_alignment(TextAlignment::Center)
                .title("Test Title")
                .title_alignment(TitleAlignment::Center)
                .width(60)
                .border_color("blue")
                .render(black_box("Test content"));
        });
    });

    group.finish();
}

// ============================================================================
// Title Benchmarks
// ============================================================================

fn bench_titles(c: &mut Criterion) {
    let mut group = c.benchmark_group("titles");

    group.bench_function("no_title", |b| {
        b.iter(|| {
            let _ = boxen(black_box("Content"), None);
        });
    });

    group.bench_function("with_title", |b| {
        b.iter(|| {
            let _ = builder().title("Title").render(black_box("Content"));
        });
    });

    let title_alignments = vec![
        ("left", TitleAlignment::Left),
        ("center", TitleAlignment::Center),
        ("right", TitleAlignment::Right),
    ];

    for (name, alignment) in title_alignments {
        group.bench_with_input(
            BenchmarkId::new("title_alignment", name),
            &alignment,
            |b, alignment: &TitleAlignment| {
                b.iter(|| {
                    let _ = builder()
                        .title("Test Title")
                        .title_alignment(alignment.clone())
                        .render(black_box("Content"));
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Width Configuration Benchmarks
// ============================================================================

fn bench_width_configurations(c: &mut Criterion) {
    let mut group = c.benchmark_group("width");

    group.bench_function("auto_width", |b| {
        b.iter(|| {
            let _ = boxen(black_box("Test content"), None);
        });
    });

    let widths = vec![20, 40, 60, 80, 100, 120];

    for width in widths {
        group.bench_with_input(
            BenchmarkId::new("fixed_width", width),
            &width,
            |b, &width| {
                b.iter(|| {
                    let _ = boxen(
                        black_box("Test content"),
                        Some(BoxenOptions {
                            width: Some(width),
                            ..Default::default()
                        }),
                    );
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Batch Rendering Benchmarks
// ============================================================================

fn bench_batch_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch");

    let batch_sizes = vec![1, 10, 50, 100, 500];

    for batch_size in batch_sizes {
        group.throughput(Throughput::Elements(batch_size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(batch_size),
            &batch_size,
            |b, &batch_size| {
                b.iter(|| {
                    for _ in 0..batch_size {
                        let _ = boxen(black_box("Batch test"), None);
                    }
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Complex Configuration Benchmarks
// ============================================================================

fn bench_complex_configurations(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex");

    group.bench_function("all_features", |b| {
        b.iter(|| {
            let _ = builder()
                .border_style(BorderStyle::Double)
                .padding(3)
                .margin(2)
                .text_alignment(TextAlignment::Center)
                .title("Performance Test")
                .title_alignment(TitleAlignment::Center)
                .width(60)
                .border_color("red")
                .dim_border(true)
                .render(black_box(
                    "Complex box configuration with all features enabled",
                ));
        });
    });

    group.bench_function("multiline_with_features", |b| {
        let content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        b.iter(|| {
            let _ = builder()
                .border_style(BorderStyle::Round)
                .padding(2)
                .title("Multi-line Test")
                .text_alignment(TextAlignment::Center)
                .width(50)
                .render(black_box(content));
        });
    });

    group.finish();
}

// ============================================================================
// Realistic Workload Benchmarks
// ============================================================================

fn bench_realistic_workloads(c: &mut Criterion) {
    let mut group = c.benchmark_group("realistic");

    // Simulating a CLI help message
    group.bench_function("cli_help_message", |b| {
        let help_text = "Usage: myapp [OPTIONS] <COMMAND>\n\n\
                         Commands:\n  \
                         start    Start the application\n  \
                         stop     Stop the application\n  \
                         status   Check status\n\n\
                         Options:\n  \
                         -h, --help     Print help\n  \
                         -v, --version  Print version";

        b.iter(|| {
            let _ = builder()
                .border_style(BorderStyle::Round)
                .padding(1)
                .title("Help")
                .title_alignment(TitleAlignment::Center)
                .render(black_box(help_text));
        });
    });

    // Simulating a status message
    group.bench_function("status_message", |b| {
        b.iter(|| {
            let _ = builder()
                .border_style(BorderStyle::Double)
                .padding(2)
                .title("Status")
                .border_color("green")
                .render(black_box("✓ All systems operational"));
        });
    });

    // Simulating an error message
    group.bench_function("error_message", |b| {
        let error_text = "Error: Connection failed\n\n\
                          Details:\n  \
                          - Host: example.com\n  \
                          - Port: 8080\n  \
                          - Timeout: 30s\n\n\
                          Please check your network connection.";

        b.iter(|| {
            let _ = builder()
                .border_style(BorderStyle::Bold)
                .padding(2)
                .title("Error")
                .title_alignment(TitleAlignment::Left)
                .border_color("red")
                .render(black_box(error_text));
        });
    });

    group.finish();
}

// ============================================================================
// Criterion Configuration and Groups
// ============================================================================

criterion_group!(
    basic_benches,
    bench_simple_box,
    bench_border_styles,
    bench_padding_variations
);

criterion_group!(
    content_benches,
    bench_content_sizes,
    bench_multiline_content,
    bench_unicode_content
);

criterion_group!(
    formatting_benches,
    bench_text_alignment,
    bench_titles,
    bench_width_configurations
);

criterion_group!(
    builder_benches,
    bench_builder_pattern,
    bench_complex_configurations
);

criterion_group!(
    workload_benches,
    bench_batch_rendering,
    bench_realistic_workloads
);

criterion_main!(
    basic_benches,
    content_benches,
    formatting_benches,
    builder_benches,
    workload_benches
);
