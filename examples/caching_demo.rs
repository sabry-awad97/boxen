/// Caching features demonstration
///
/// This example demonstrates the performance benefits of the width and terminal
/// size caching features. Run with and without features to compare performance.
///
/// Run with caching:
/// cargo run --example `caching_demo` --features width-cache,terminal-cache
///
/// Run without caching:
/// cargo run --example `caching_demo`
use ::boxen::{BorderStyle, boxen, builder};
use std::time::Instant;

fn main() {
    println!("Boxen Caching Features Demo");
    println!("===========================\n");

    // Check which features are enabled
    #[cfg(feature = "width-cache")]
    println!("✓ Width caching enabled");
    #[cfg(not(feature = "width-cache"))]
    println!("✗ Width caching disabled");

    #[cfg(feature = "terminal-cache")]
    println!("✓ Terminal size caching enabled");
    #[cfg(not(feature = "terminal-cache"))]
    println!("✗ Terminal size caching disabled");

    println!();

    // Benchmark 1: Simple boxes
    println!("Benchmark 1: Simple boxes (1000 iterations)");
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = boxen("Hello, World!", None);
    }
    let duration = start.elapsed();
    println!("Time: {duration:?}");
    println!("Avg: {:?} per box\n", duration / 1000);

    // Benchmark 2: Unicode content
    println!("Benchmark 2: Unicode content (1000 iterations)");
    let unicode = "Unicode: 🌍🌎🌏 你好世界 🚀✨🎉 Émojis: àáâãäåæçèéêë";
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = boxen(unicode, None);
    }
    let duration = start.elapsed();
    println!("Time: {duration:?}");
    println!("Avg: {:?} per box\n", duration / 1000);

    // Benchmark 3: Multiple boxes with same content
    println!("Benchmark 3: Repeated content (1000 iterations)");
    let content = "This is repeated content that should benefit from caching";
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = boxen(content, None);
    }
    let duration = start.elapsed();
    println!("Time: {duration:?}");
    println!("Avg: {:?} per box\n", duration / 1000);

    // Benchmark 4: Complex boxes
    println!("Benchmark 4: Complex boxes (1000 iterations)");
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = builder()
            .border_style(BorderStyle::Double)
            .padding(2)
            .title("Test")
            .render("Complex box with title and padding");
    }
    let duration = start.elapsed();
    println!("Time: {duration:?}");
    println!("Avg: {:?} per box\n", duration / 1000);

    // Display cache statistics if available
    #[cfg(feature = "width-cache")]
    {
        println!("\nWidth Cache Statistics:");
        let stats = boxen::text::width_cache::cache_stats();
        println!("  Hits: {}", stats.hits);
        println!("  Misses: {}", stats.misses);
        println!("  Hit rate: {:.2}%", stats.hit_rate());
        println!("  Evictions: {}", stats.evictions);
    }

    #[cfg(feature = "terminal-cache")]
    {
        println!("\nTerminal Size Cache Statistics:");
        let stats = boxen::terminal::size_cache::cache_stats();
        println!("  Hits: {}", stats.hits);
        println!("  Misses: {}", stats.misses);
        println!("  Hit rate: {:.2}%", stats.hit_rate());
        println!("  Expirations: {}", stats.expirations);
        println!("  Invalidations: {}", stats.invalidations);
    }

    println!("\n=== Performance Tips ===");
    println!("1. Enable width-cache for Unicode-heavy content (2-3x speedup)");
    println!("2. Enable terminal-cache for batch rendering (10-20% speedup)");
    println!("3. Use both features together for maximum performance");
    println!("\nTo enable features:");
    println!("  cargo build --features width-cache,terminal-cache");
}
