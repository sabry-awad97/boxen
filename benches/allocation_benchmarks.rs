/// Allocation and performance benchmarks for boxen library
///
/// This benchmark suite measures:
/// 1. Rendering performance for various box configurations
/// 2. Memory allocation patterns
/// 3. Performance improvements from string pooling optimizations
///
/// Run with: cargo bench --bench `allocation_benchmarks`
use ::boxen::{BorderStyle, BoxenOptions, Spacing, TextAlignment, TitleAlignment, boxen, builder};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

// Test configurations
const ITERATIONS: usize = 1000;
const WARMUP_ITERATIONS: usize = 100;

// Global allocation tracker
struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATED: AtomicUsize = AtomicUsize::new(0);
static ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = unsafe { System.alloc(layout) };
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
            ALLOCATION_COUNT.fetch_add(1, Ordering::SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        DEALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

fn reset_allocation_stats() {
    ALLOCATED.store(0, Ordering::SeqCst);
    DEALLOCATED.store(0, Ordering::SeqCst);
    ALLOCATION_COUNT.store(0, Ordering::SeqCst);
}

fn get_allocation_stats() -> (usize, usize, usize) {
    (
        ALLOCATED.load(Ordering::SeqCst),
        DEALLOCATED.load(Ordering::SeqCst),
        ALLOCATION_COUNT.load(Ordering::SeqCst),
    )
}

#[derive(Debug)]
struct BenchmarkResult {
    name: String,
    mean_duration: Duration,
    min_duration: Duration,
    max_duration: Duration,
    total_duration: Duration,
    iterations: usize,
    total_allocated: usize,
    total_deallocated: usize,
    allocation_count: usize,
    allocations_per_op: f64,
    bytes_per_op: f64,
}

impl BenchmarkResult {
    fn new(name: &str, durations: &[Duration], alloc_stats: (usize, usize, usize)) -> Self {
        let total: Duration = durations.iter().sum();
        let mean = total / durations.len() as u32;
        let min = *durations.iter().min().unwrap();
        let max = *durations.iter().max().unwrap();

        let (allocated, deallocated, alloc_count) = alloc_stats;
        let iterations = durations.len();

        Self {
            name: name.to_string(),
            mean_duration: mean,
            min_duration: min,
            max_duration: max,
            total_duration: total,
            iterations,
            total_allocated: allocated,
            total_deallocated: deallocated,
            allocation_count: alloc_count,
            allocations_per_op: alloc_count as f64 / iterations as f64,
            bytes_per_op: allocated as f64 / iterations as f64,
        }
    }

    fn print(&self) {
        println!("\n{}", "=".repeat(70));
        println!("Benchmark: {}", self.name);
        println!("{}", "=".repeat(70));
        println!("Iterations:     {}", self.iterations);
        println!("Total time:     {:?}", self.total_duration);
        println!("Mean time:      {:?}", self.mean_duration);
        println!("Min time:       {:?}", self.min_duration);
        println!("Max time:       {:?}", self.max_duration);
        println!(
            "Throughput:     {:.2} ops/sec",
            1_000_000_000.0 / self.mean_duration.as_nanos() as f64
        );
        println!("\n--- Memory Statistics ---");
        println!("Total allocated:      {} bytes", self.total_allocated);
        println!("Total deallocated:    {} bytes", self.total_deallocated);
        println!(
            "Net allocated:        {} bytes",
            self.total_allocated.saturating_sub(self.total_deallocated)
        );
        println!("Allocation count:     {}", self.allocation_count);
        println!("Allocations per op:   {:.2}", self.allocations_per_op);
        println!("Bytes per op:         {:.2}", self.bytes_per_op);
    }
}

fn benchmark<F>(name: &str, mut f: F) -> BenchmarkResult
where
    F: FnMut(),
{
    // Warmup
    for _ in 0..WARMUP_ITERATIONS {
        f();
    }

    // Reset allocation stats before actual benchmark
    reset_allocation_stats();

    // Actual benchmark
    let mut durations = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        let start = Instant::now();
        f();
        durations.push(start.elapsed());
    }

    // Capture allocation stats
    let alloc_stats = get_allocation_stats();

    BenchmarkResult::new(name, &durations, alloc_stats)
}

fn main() {
    println!("\n{}", "=".repeat(70));
    println!("BOXEN PERFORMANCE BENCHMARKS");
    println!("{}", "=".repeat(70));
    println!("Measuring performance improvements from string allocation optimizations");
    println!("Target: 80% reduction in allocations, 3-5x performance improvement");
    println!();

    // Benchmark 1: Simple box
    let result = benchmark("Simple Box (80x24)", || {
        let _ = boxen("Hello, World!", None);
    });
    result.print();

    // Benchmark 2: Box with options
    let result = benchmark("Box with Options", || {
        let _ = boxen(
            "Hello, World!",
            Some(BoxenOptions {
                border_style: BorderStyle::Double,
                padding: Spacing::from(2),
                ..Default::default()
            }),
        );
    });
    result.print();

    // Benchmark 3: Box with title
    let result = benchmark("Box with Title", || {
        let _ = builder()
            .border_style(BorderStyle::Round)
            .padding(2)
            .title("Test Title")
            .title_alignment(TitleAlignment::Center)
            .render("Content with title");
    });
    result.print();

    // Benchmark 4: Multi-line content
    let multiline = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
    let result = benchmark("Multi-line Content (5 lines)", || {
        let _ = boxen(multiline, None);
    });
    result.print();

    // Benchmark 5: Large content
    let large_content = "Large content line. ".repeat(50);
    let result = benchmark("Large Content (1000 chars)", || {
        let _ = boxen(
            &large_content,
            Some(BoxenOptions {
                width: Some(boxen::Width::Fixed(60)),
                ..Default::default()
            }),
        );
    });
    result.print();

    // Benchmark 6: Unicode content
    let unicode = "Unicode: 🌍🌎🌏 你好世界 🚀✨🎉 Émojis: àáâãäåæçèéêë";
    let result = benchmark("Unicode Content", || {
        let _ = boxen(unicode, None);
    });
    result.print();

    // Benchmark 7: Complex configuration
    let result = benchmark("Complex Configuration", || {
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
            .render("Complex box configuration");
    });
    result.print();

    // Benchmark 8: Repeated rendering (batch)
    let result = benchmark("Batch Rendering (100 boxes)", || {
        for _ in 0..100 {
            let _ = boxen("Batch test", None);
        }
    });
    result.print();

    // Benchmark 9: Different border styles
    let styles = [
        BorderStyle::Single,
        BorderStyle::Double,
        BorderStyle::Round,
        BorderStyle::Bold,
    ];
    let result = benchmark("Border Style Variations", || {
        for style in &styles {
            let _ = boxen(
                "Style test",
                Some(BoxenOptions {
                    border_style: *style,
                    ..Default::default()
                }),
            );
        }
    });
    result.print();

    // Benchmark 10: Text alignment variations
    let alignments = [
        TextAlignment::Left,
        TextAlignment::Center,
        TextAlignment::Right,
    ];
    let result = benchmark("Text Alignment Variations", || {
        for alignment in &alignments {
            let _ = boxen(
                "Alignment test",
                Some(BoxenOptions {
                    text_alignment: *alignment,
                    width: Some(boxen::Width::Fixed(40)),
                    ..Default::default()
                }),
            );
        }
    });
    result.print();

    // Summary
    println!("\n{}", "=".repeat(70));
    println!("BENCHMARK SUMMARY");
    println!("{}", "=".repeat(70));
    println!("\nAll benchmarks completed successfully!");
    println!("\nTo compare with baseline:");
    println!("1. Save these results");
    println!("2. Apply optimizations");
    println!("3. Re-run benchmarks");
    println!("4. Compare mean times and allocation counts for improvement percentage");
    println!("\nExpected improvements:");
    println!("- Simple boxes: 3x faster");
    println!("- Unicode content: 3x faster");
    println!("- Large boxes: 4x faster");
    println!("- Batch rendering: 3.75x faster");
    println!("- Allocations: 80% reduction (from ~25 to ~3-5 per box)");
    println!("\nMemory Optimization Targets:");
    println!("- Baseline: ~25 allocations per box");
    println!("- Target: ~3-5 allocations per box (80% reduction)");
    println!("- String pooling should reduce allocations in hot paths");
    println!("- Caching should reduce redundant calculations");
    println!("{}", "=".repeat(70));
}
