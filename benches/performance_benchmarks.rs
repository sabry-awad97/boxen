/// Comprehensive performance benchmarks for boxen library
///
/// This benchmark suite measures overall performance improvements across
/// various scenarios and configurations.
///
/// Run with: cargo bench --bench performance_benchmarks
use ::boxen::{BorderStyle, BoxenOptions, Spacing, TextAlignment, TitleAlignment, boxen, builder};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

// Test content samples
const SIMPLE_TEXT: &str = "Hello, World!";
const MULTILINE_TEXT: &str = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
const UNICODE_TEXT: &str = "Unicode: 🌍🌎🌏 你好世界 🚀✨🎉 Émojis: àáâãäåæçèéêë";
const LARGE_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

fn bench_simple_boxes(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_boxes");

    group.bench_function("default_style", |b| {
        b.iter(|| black_box(boxen(SIMPLE_TEXT, None).unwrap()));
    });

    group.bench_function("double_border", |b| {
        b.iter(|| {
            black_box(
                boxen(
                    SIMPLE_TEXT,
                    Some(BoxenOptions {
                        border_style: BorderStyle::Double,
                        ..Default::default()
                    }),
                )
                .unwrap(),
            )
        });
    });

    group.bench_function("round_border", |b| {
        b.iter(|| {
            black_box(
                boxen(
                    SIMPLE_TEXT,
                    Some(BoxenOptions {
                        border_style: BorderStyle::Round,
                        ..Default::default()
                    }),
                )
                .unwrap(),
            )
        });
    });

    group.finish();
}

fn bench_with_padding(c: &mut Criterion) {
    let mut group = c.benchmark_group("padding");

    for padding in [1, 2, 3, 5].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(padding),
            padding,
            |b, &padding| {
                b.iter(|| {
                    black_box(
                        boxen(
                            SIMPLE_TEXT,
                            Some(BoxenOptions {
                                padding: Spacing::from(padding),
                                ..Default::default()
                            }),
                        )
                        .unwrap(),
                    )
                });
            },
        );
    }

    group.finish();
}

fn bench_text_alignment(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_alignment");

    group.bench_function("left", |b| {
        b.iter(|| {
            black_box(
                boxen(
                    SIMPLE_TEXT,
                    Some(BoxenOptions {
                        text_alignment: TextAlignment::Left,
                        width: Some(30),
                        ..Default::default()
                    }),
                )
                .unwrap(),
            )
        });
    });

    group.bench_function("center", |b| {
        b.iter(|| {
            black_box(
                boxen(
                    SIMPLE_TEXT,
                    Some(BoxenOptions {
                        text_alignment: TextAlignment::Center,
                        width: Some(30),
                        ..Default::default()
                    }),
                )
                .unwrap(),
            )
        });
    });

    group.bench_function("right", |b| {
        b.iter(|| {
            black_box(
                boxen(
                    SIMPLE_TEXT,
                    Some(BoxenOptions {
                        text_alignment: TextAlignment::Right,
                        width: Some(30),
                        ..Default::default()
                    }),
                )
                .unwrap(),
            )
        });
    });

    group.finish();
}

fn bench_multiline_content(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiline");

    group.bench_function("5_lines", |b| {
        b.iter(|| black_box(boxen(MULTILINE_TEXT, None).unwrap()));
    });

    group.bench_function("5_lines_with_padding", |b| {
        b.iter(|| {
            black_box(
                boxen(
                    MULTILINE_TEXT,
                    Some(BoxenOptions {
                        padding: Spacing::from(2),
                        ..Default::default()
                    }),
                )
                .unwrap(),
            )
        });
    });

    group.bench_function("5_lines_centered", |b| {
        b.iter(|| {
            black_box(
                boxen(
                    MULTILINE_TEXT,
                    Some(BoxenOptions {
                        text_alignment: TextAlignment::Center,
                        width: Some(40),
                        ..Default::default()
                    }),
                )
                .unwrap(),
            )
        });
    });

    group.finish();
}

fn bench_unicode_content(c: &mut Criterion) {
    let mut group = c.benchmark_group("unicode");

    group.bench_function("basic", |b| {
        b.iter(|| black_box(boxen(UNICODE_TEXT, None).unwrap()));
    });

    group.bench_function("with_padding", |b| {
        b.iter(|| {
            black_box(
                boxen(
                    UNICODE_TEXT,
                    Some(BoxenOptions {
                        padding: Spacing::from(2),
                        ..Default::default()
                    }),
                )
                .unwrap(),
            )
        });
    });

    group.bench_function("centered", |b| {
        b.iter(|| {
            black_box(
                boxen(
                    UNICODE_TEXT,
                    Some(BoxenOptions {
                        text_alignment: TextAlignment::Center,
                        width: Some(60),
                        ..Default::default()
                    }),
                )
                .unwrap(),
            )
        });
    });

    group.finish();
}

fn bench_large_content(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_content");

    group.bench_function("wrapped_40", |b| {
        b.iter(|| {
            black_box(
                boxen(
                    LARGE_TEXT,
                    Some(BoxenOptions {
                        width: Some(40),
                        ..Default::default()
                    }),
                )
                .unwrap(),
            )
        });
    });

    group.bench_function("wrapped_60", |b| {
        b.iter(|| {
            black_box(
                boxen(
                    LARGE_TEXT,
                    Some(BoxenOptions {
                        width: Some(60),
                        ..Default::default()
                    }),
                )
                .unwrap(),
            )
        });
    });

    group.bench_function("wrapped_80", |b| {
        b.iter(|| {
            black_box(
                boxen(
                    LARGE_TEXT,
                    Some(BoxenOptions {
                        width: Some(80),
                        ..Default::default()
                    }),
                )
                .unwrap(),
            )
        });
    });

    group.finish();
}

fn bench_with_title(c: &mut Criterion) {
    let mut group = c.benchmark_group("with_title");

    group.bench_function("left_aligned", |b| {
        b.iter(|| {
            black_box(
                builder()
                    .title("Test Title")
                    .title_alignment(TitleAlignment::Left)
                    .render(SIMPLE_TEXT)
                    .unwrap(),
            )
        });
    });

    group.bench_function("center_aligned", |b| {
        b.iter(|| {
            black_box(
                builder()
                    .title("Test Title")
                    .title_alignment(TitleAlignment::Center)
                    .render(SIMPLE_TEXT)
                    .unwrap(),
            )
        });
    });

    group.bench_function("right_aligned", |b| {
        b.iter(|| {
            black_box(
                builder()
                    .title("Test Title")
                    .title_alignment(TitleAlignment::Right)
                    .render(SIMPLE_TEXT)
                    .unwrap(),
            )
        });
    });

    group.finish();
}

fn bench_complex_configuration(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex");

    group.bench_function("full_featured", |b| {
        b.iter(|| {
            black_box(
                builder()
                    .border_style(BorderStyle::Double)
                    .padding(3)
                    .margin(2)
                    .text_alignment(TextAlignment::Center)
                    .title("Performance Test")
                    .title_alignment(TitleAlignment::Center)
                    .width(60)
                    .border_color("blue")
                    .dim_border(true)
                    .render("Complex box configuration with all features enabled")
                    .unwrap(),
            )
        });
    });

    group.finish();
}

fn bench_batch_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch");

    group.bench_function("10_boxes", |b| {
        b.iter(|| {
            for _ in 0..10 {
                black_box(boxen(SIMPLE_TEXT, None).unwrap());
            }
        });
    });

    group.bench_function("100_boxes", |b| {
        b.iter(|| {
            for _ in 0..100 {
                black_box(boxen(SIMPLE_TEXT, None).unwrap());
            }
        });
    });

    group.bench_function("1000_boxes", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(boxen(SIMPLE_TEXT, None).unwrap());
            }
        });
    });

    group.finish();
}

fn bench_border_styles(c: &mut Criterion) {
    let mut group = c.benchmark_group("border_styles");

    let styles = [
        ("single", BorderStyle::Single),
        ("double", BorderStyle::Double),
        ("round", BorderStyle::Round),
        ("bold", BorderStyle::Bold),
        ("single_double", BorderStyle::SingleDouble),
        ("double_single", BorderStyle::DoubleSingle),
        ("classic", BorderStyle::Classic),
    ];

    for (name, style) in styles.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), style, |b, style| {
            b.iter(|| {
                black_box(
                    boxen(
                        SIMPLE_TEXT,
                        Some(BoxenOptions {
                            border_style: style.clone(),
                            ..Default::default()
                        }),
                    )
                    .unwrap(),
                )
            });
        });
    }

    group.finish();
}

fn bench_width_variations(c: &mut Criterion) {
    let mut group = c.benchmark_group("width_variations");

    for width in [20, 40, 60, 80, 100].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(width), width, |b, &width| {
            b.iter(|| {
                black_box(
                    boxen(
                        LARGE_TEXT,
                        Some(BoxenOptions {
                            width: Some(width),
                            ..Default::default()
                        }),
                    )
                    .unwrap(),
                )
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_simple_boxes,
    bench_with_padding,
    bench_text_alignment,
    bench_multiline_content,
    bench_unicode_content,
    bench_large_content,
    bench_with_title,
    bench_complex_configuration,
    bench_batch_rendering,
    bench_border_styles,
    bench_width_variations,
);

criterion_main!(benches);
