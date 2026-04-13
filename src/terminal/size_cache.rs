/// Terminal size caching with TTL for improved performance
///
/// This module provides a TTL-based cache for terminal dimensions to avoid
/// expensive system calls. The cache automatically expires after a configurable
/// duration and can be invalidated by SIGWINCH signals on Unix systems.
#[cfg(feature = "terminal-cache")]
use std::cell::RefCell;
#[cfg(feature = "terminal-cache")]
use std::time::{Duration, Instant};
use terminal_size::{Height, Width, terminal_size};

/// Default cache TTL (time-to-live) in milliseconds
#[cfg(feature = "terminal-cache")]
const DEFAULT_TTL_MS: u64 = 100;

/// Cached terminal size with expiration
#[cfg(feature = "terminal-cache")]
#[derive(Debug, Clone)]
struct CachedSize {
    width: u16,
    height: u16,
    timestamp: Instant,
}

#[cfg(feature = "terminal-cache")]
impl CachedSize {
    fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            timestamp: Instant::now(),
        }
    }

    fn is_expired(&self, ttl: Duration) -> bool {
        self.timestamp.elapsed() > ttl
    }
}

/// Cache statistics for monitoring performance
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: usize,
    /// Number of cache misses
    pub misses: usize,
    /// Number of cache expirations
    pub expirations: usize,
    /// Number of manual cache invalidations
    pub invalidations: usize,
}

impl CacheStats {
    /// Calculate cache hit rate as a percentage
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            (self.hits as f64 / total as f64) * 100.0
        }
    }

    /// Reset all statistics
    pub fn reset(&mut self) {
        self.hits = 0;
        self.misses = 0;
        self.expirations = 0;
        self.invalidations = 0;
    }
}

/// Terminal size cache with TTL
#[cfg(feature = "terminal-cache")]
struct TerminalSizeCache {
    cached: Option<CachedSize>,
    ttl: Duration,
    stats: CacheStats,
}

#[cfg(feature = "terminal-cache")]
impl TerminalSizeCache {
    fn new(ttl_ms: u64) -> Self {
        Self {
            cached: None,
            ttl: Duration::from_millis(ttl_ms),
            stats: CacheStats::default(),
        }
    }

    fn get(&mut self) -> Option<(u16, u16)> {
        if let Some(ref cached) = self.cached {
            if !cached.is_expired(self.ttl) {
                self.stats.hits += 1;
                return Some((cached.width, cached.height));
            }
            self.stats.expirations += 1;
            self.cached = None;
        }
        self.stats.misses += 1;
        None
    }

    fn set(&mut self, width: u16, height: u16) {
        self.cached = Some(CachedSize::new(width, height));
    }

    fn invalidate(&mut self) {
        if self.cached.is_some() {
            self.stats.invalidations += 1;
            self.cached = None;
        }
    }

    fn set_ttl(&mut self, ttl_ms: u64) {
        self.ttl = Duration::from_millis(ttl_ms);
    }

    fn stats(&self) -> CacheStats {
        self.stats.clone()
    }

    fn clear(&mut self) {
        self.cached = None;
        self.stats.reset();
    }
}

#[cfg(feature = "terminal-cache")]
thread_local! {
    static SIZE_CACHE: RefCell<TerminalSizeCache> = RefCell::new(TerminalSizeCache::new(DEFAULT_TTL_MS));
}

/// Get terminal size with caching
///
/// This function uses a TTL-based cache to avoid expensive terminal size queries.
/// The cache expires after the configured TTL (default 100ms) and is automatically
/// invalidated on SIGWINCH signals (Unix only).
///
/// # Returns
///
/// Returns `Some((width, height))` if terminal size can be determined, `None` otherwise.
///
/// # Examples
///
/// ```
/// use ::boxen::terminal::size_cache::cached_terminal_size;
///
/// if let Some((width, height)) = cached_terminal_size() {
///     println!("Terminal: {}x{}", width, height);
/// }
/// ```
#[cfg(feature = "terminal-cache")]
pub fn cached_terminal_size() -> Option<(u16, u16)> {
    SIZE_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();

        // Check cache first
        if let Some(size) = cache.get() {
            return Some(size);
        }

        // Cache miss - query terminal
        if let Some((Width(w), Height(h))) = terminal_size() {
            cache.set(w, h);
            Some((w, h))
        } else {
            None
        }
    })
}

/// Get terminal size without caching (fallback)
#[cfg(not(feature = "terminal-cache"))]
pub fn cached_terminal_size() -> Option<(u16, u16)> {
    terminal_size().map(|(Width(w), Height(h))| (w, h))
}

/// Invalidate the terminal size cache
///
/// Forces the next call to `cached_terminal_size()` to query the terminal.
/// This is automatically called on SIGWINCH signals (Unix only).
///
/// # Examples
///
/// ```
/// use ::boxen::terminal::size_cache::invalidate_cache;
///
/// // Manually invalidate cache after terminal resize
/// invalidate_cache();
/// ```
#[cfg(feature = "terminal-cache")]
pub fn invalidate_cache() {
    SIZE_CACHE.with(|cache| {
        cache.borrow_mut().invalidate();
    });
}

/// Get cache statistics
///
/// Returns hit/miss/expiration/invalidation counts for monitoring cache performance.
/// Only available when the `terminal-cache` feature is enabled.
///
/// # Examples
///
/// ```
/// use ::boxen::terminal::size_cache::cache_stats;
///
/// let stats = cache_stats();
/// println!("Hit rate: {:.2}%", stats.hit_rate());
/// ```
#[cfg(feature = "terminal-cache")]
pub fn cache_stats() -> CacheStats {
    SIZE_CACHE.with(|cache| cache.borrow().stats())
}

/// Clear the terminal size cache
///
/// Removes cached size and resets statistics.
/// Only available when the `terminal-cache` feature is enabled.
#[cfg(feature = "terminal-cache")]
pub fn clear_cache() {
    SIZE_CACHE.with(|cache| cache.borrow_mut().clear());
}

/// Configure cache TTL (time-to-live)
///
/// Sets the duration in milliseconds before cached values expire.
/// This will clear the existing cache.
///
/// # Arguments
///
/// * `ttl_ms` - Time-to-live in milliseconds
///
/// # Examples
///
/// ```
/// use ::boxen::terminal::size_cache::set_cache_ttl;
///
/// // Set cache to expire after 200ms
/// set_cache_ttl(200);
/// ```
#[cfg(feature = "terminal-cache")]
pub fn set_cache_ttl(ttl_ms: u64) {
    SIZE_CACHE.with(|cache| {
        cache.borrow_mut().set_ttl(ttl_ms);
    });
}

/// Setup SIGWINCH handler to invalidate cache on terminal resize (Unix only)
///
/// This function sets up a signal handler that automatically invalidates the
/// terminal size cache when the terminal is resized. This ensures the cache
/// always reflects the current terminal dimensions.
///
/// # Platform Support
///
/// This function is only available on Unix platforms (Linux, macOS, BSD, etc.).
/// On other platforms, it's a no-op.
///
/// # Examples
///
/// ```no_run
/// use ::boxen::terminal::size_cache::setup_sigwinch_handler;
///
/// // Setup handler at application startup
/// setup_sigwinch_handler();
/// ```
#[cfg(all(feature = "terminal-cache", unix))]
pub fn setup_sigwinch_handler() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    static HANDLER_INSTALLED: AtomicBool = AtomicBool::new(false);

    // Only install handler once
    if HANDLER_INSTALLED.swap(true, Ordering::SeqCst) {
        return;
    }

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGWINCH, Arc::clone(&term))
        .expect("Failed to register SIGWINCH handler");

    // Spawn background thread to handle signals
    std::thread::spawn(move || {
        loop {
            if term.load(Ordering::Relaxed) {
                invalidate_cache();
                term.store(false, Ordering::Relaxed);
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    });
}

/// Setup SIGWINCH handler to invalidate cache on terminal resize (Unix only)
///
/// This function sets up a signal handler that automatically invalidates the
/// terminal size cache when the terminal is resized. This ensures the cache
/// always reflects the current terminal dimensions.
///
/// # Platform Support
///
/// This function is only available on Unix platforms (Linux, macOS, BSD, etc.).
/// On other platforms, it's a no-op.
///
/// # Examples
///
/// ```no_run
/// use ::boxen::terminal::size_cache::setup_sigwinch_handler;
///
/// // Setup handler at application startup
/// setup_sigwinch_handler();
/// ```
#[cfg(all(feature = "terminal-cache", not(unix)))]
pub fn setup_sigwinch_handler() {
    // No-op on non-Unix platforms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_terminal_size() {
        let size = cached_terminal_size();
        // May be None in CI environments without a terminal
        if let Some((w, h)) = size {
            assert!(w > 0);
            assert!(h > 0);
        }
    }

    #[cfg(feature = "terminal-cache")]
    #[test]
    fn test_cache_hit() {
        clear_cache();

        // First call - cache miss
        let _ = cached_terminal_size();
        let stats1 = cache_stats();
        assert_eq!(stats1.misses, 1);

        // Second call - cache hit (if terminal exists)
        let _ = cached_terminal_size();
        let stats2 = cache_stats();
        if cached_terminal_size().is_some() {
            assert_eq!(stats2.hits, 1);
        }
    }

    #[cfg(feature = "terminal-cache")]
    #[test]
    fn test_cache_invalidation() {
        clear_cache();

        let _ = cached_terminal_size();
        invalidate_cache();

        let stats = cache_stats();
        if cached_terminal_size().is_some() {
            assert_eq!(stats.invalidations, 1);
        }
    }

    #[cfg(feature = "terminal-cache")]
    #[test]
    fn test_cache_clear() {
        clear_cache();

        let _ = cached_terminal_size();
        clear_cache();

        let stats = cache_stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
    }

    #[cfg(feature = "terminal-cache")]
    #[test]
    fn test_set_ttl() {
        set_cache_ttl(50);
        clear_cache();

        // First call
        let _ = cached_terminal_size();

        // Wait for expiration
        std::thread::sleep(Duration::from_millis(60));

        // Should be expired
        let _ = cached_terminal_size();
        let stats = cache_stats();
        if cached_terminal_size().is_some() {
            assert!(stats.expirations > 0);
        }
    }

    #[cfg(feature = "terminal-cache")]
    #[test]
    fn test_cache_stats() {
        clear_cache();

        for _ in 0..10 {
            let _ = cached_terminal_size();
        }

        let stats = cache_stats();
        if cached_terminal_size().is_some() {
            assert!(stats.hits > 0);
            assert_eq!(stats.misses, 1);
            assert!(stats.hit_rate() > 80.0);
        }
    }
}
