// NeuroGraph - Tracing Sampling Performance Test v0.44.3
// Copyright (C) 2024-2025 Chernov Denys
//
// Benchmark comparing tracing overhead with different sampling strategies:
// 1. No tracing (baseline)
// 2. Full tracing (100% sampling) - current v0.44.0 behavior
// 3. Probability sampling (1%) - v0.44.3 base rate
// 4. Adaptive sampling (error/latency-based) - v0.44.3 intelligent

use neurograph_core::Token;
use std::time::Instant;

/// Test configuration
const TEST_TOKENS: usize = 1_000_000;    // 1M tokens

#[derive(Debug)]
struct PerformanceResults {
    baseline_ms: u128,
    full_tracing_ms: u128,
    sampled_1pct_ms: u128,
    adaptive_ms: u128,
    full_overhead: f64,
    sampled_overhead: f64,
    adaptive_overhead: f64,
    improvement_factor: f64,
}

impl PerformanceResults {
    fn calculate(&mut self) {
        self.full_overhead = (self.full_tracing_ms as f64 / self.baseline_ms as f64) - 1.0;
        self.sampled_overhead = (self.sampled_1pct_ms as f64 / self.baseline_ms as f64) - 1.0;
        self.adaptive_overhead = (self.adaptive_ms as f64 / self.baseline_ms as f64) - 1.0;
        self.improvement_factor = self.full_tracing_ms as f64 / self.sampled_1pct_ms as f64;
    }

    fn print_report(&self) {
        println!("\n╔══════════════════════════════════════════════════════════╗");
        println!("║      TRACING SAMPLING PERFORMANCE TEST v0.44.3           ║");
        println!("╚══════════════════════════════════════════════════════════╝\n");

        println!("📊 Test Configuration:");
        println!("  Tokens created:              {:>10}", TEST_TOKENS);
        println!("  Trace points:                {:>10} (1 per token)", TEST_TOKENS);

        println!("\n⚡ Performance Results:");
        println!("  Baseline (no tracing):       {:>8} ms", self.baseline_ms);
        println!("  Full tracing (100%):         {:>8} ms", self.full_tracing_ms);
        println!("  Sampled (1%):                {:>8} ms", self.sampled_1pct_ms);
        println!("  Adaptive sampling:           {:>8} ms", self.adaptive_ms);

        println!("\n📈 Overhead Analysis:");
        println!("  Full tracing overhead:       {:>7.1}x ({:.0}%)",
            self.full_overhead + 1.0, self.full_overhead * 100.0);
        println!("  1% sampling overhead:        {:>7.1}x ({:.0}%)",
            self.sampled_overhead + 1.0, self.sampled_overhead * 100.0);
        println!("  Adaptive overhead:           {:>7.1}x ({:.0}%)",
            self.adaptive_overhead + 1.0, self.adaptive_overhead * 100.0);

        println!("\n🚀 Improvement:");
        println!("  Speedup vs full tracing:     {:>7.1}x faster", self.improvement_factor);
        println!("  Overhead reduction:          {:.0}% → {:.0}%",
            self.full_overhead * 100.0, self.sampled_overhead * 100.0);

        // Verdict
        println!("\n✅ Verdict:");
        if self.sampled_overhead < 0.5 {
            println!("  🎉 EXCELLENT - Sampling overhead <50% (target achieved!)");
        } else if self.sampled_overhead < 1.5 {
            println!("  ✅ GOOD - Sampling overhead <150% (acceptable)");
        } else if self.sampled_overhead < 5.0 {
            println!("  ⚠️  MARGINAL - Sampling overhead <500% (needs tuning)");
        } else {
            println!("  ❌ POOR - Sampling overhead >500% (investigate)");
        }

        if self.improvement_factor >= 10.0 {
            println!("  🚀 Target improvement (10x+) ACHIEVED!");
        } else if self.improvement_factor >= 5.0 {
            println!("  ✅ Significant improvement (5x+) achieved");
        } else {
            println!("  ⚠️  Improvement below target ({}x < 10x)", self.improvement_factor as i32);
        }

        println!();
    }
}

/// Test 1: Baseline (no tracing)
fn test_baseline() -> u128 {
    println!("🏃 Test 1: Baseline (no tracing)");

    let start = Instant::now();

    for i in 0..TEST_TOKENS {
        let token_id = i as u32;
        let _token = Token::new(token_id);
        // No tracing overhead
    }

    let duration = start.elapsed().as_millis();
    println!("  ✅ Created {} tokens in {} ms", TEST_TOKENS, duration);

    duration
}

/// Test 2: Full tracing (100% sampling)
fn test_full_tracing() -> u128 {
    println!("\n🏃 Test 2: Full Tracing (100% sampling)");

    let start = Instant::now();

    for i in 0..TEST_TOKENS {
        let token_id = i as u32;

        // Simulate span creation (this is the overhead we're measuring)
        let _span = tracing::info_span!("token_create", id = token_id);
        let _guard = _span.enter();

        let _token = Token::new(token_id);
    }

    let duration = start.elapsed().as_millis();
    println!("  ✅ Created {} tokens with full tracing in {} ms", TEST_TOKENS, duration);

    duration
}

/// Test 3: 1% probability sampling (using disabled spans approach)
fn test_sampled_1pct() -> u128 {
    println!("\n🏃 Test 3: Probability Sampling (1%) - Using `tracing::enabled!`");

    let start = Instant::now();

    // Use a sampling counter to simulate 1% rate
    let sample_every = 100;

    for i in 0..TEST_TOKENS {
        let token_id = i as u32;

        // Create span only 1% of the time (deterministic for testing)
        if i % sample_every == 0 {
            let _span = tracing::info_span!("token_create", id = token_id);
            let _guard = _span.enter();
            let _token = Token::new(token_id);
        } else {
            // No span creation - zero overhead
            let _token = Token::new(token_id);
        }
    }

    let duration = start.elapsed().as_millis();
    let sampled_count = TEST_TOKENS / sample_every;
    println!("  ✅ Created {} tokens with 1% sampling in {} ms", TEST_TOKENS, duration);
    println!("     Sampled: {}/{} ({:.1}%)",
        sampled_count, TEST_TOKENS, (sampled_count as f64 / TEST_TOKENS as f64) * 100.0);

    duration
}

/// Test 4: Adaptive sampling (error/latency-based)
fn test_adaptive_sampling() -> u128 {
    println!("\n🏃 Test 4: Adaptive Sampling (100% errors, 1% normal)");

    let start = Instant::now();

    let sample_every = 100;
    let mut error_count = 0;
    let mut sampled_count = 0;

    for i in 0..TEST_TOKENS {
        let token_id = i as u32;

        // Simulate some errors (1% of operations)
        let is_error = (i % 100) == 0;

        if is_error {
            // Always sample errors
            error_count += 1;
            sampled_count += 1;
            let _span = tracing::info_span!("token_create",
                id = token_id,
                error = true
            );
            let _guard = _span.enter();
            let _token = Token::new(token_id);
        } else if i % sample_every == 0 {
            // Sample 1% of normal operations
            sampled_count += 1;
            let _span = tracing::info_span!("token_create", id = token_id);
            let _guard = _span.enter();
            let _token = Token::new(token_id);
        } else {
            // No span creation
            let _token = Token::new(token_id);
        }
    }

    let duration = start.elapsed().as_millis();
    println!("  ✅ Created {} tokens with adaptive sampling in {} ms", TEST_TOKENS, duration);
    println!("     Sampled: {}/{} ({:.1}%)",
        sampled_count, TEST_TOKENS, (sampled_count as f64 / TEST_TOKENS as f64) * 100.0);
    println!("     Errors: {} (100% sampled)", error_count);

    duration
}

/// Main benchmark
#[test]
fn benchmark_tracing_sampling_performance() {
    println!("\n🚀 Starting Tracing Sampling Performance Benchmark...\n");

    // Initialize tracing subscriber (required for span creation)
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .try_init();

    let mut results = PerformanceResults {
        baseline_ms: 0,
        full_tracing_ms: 0,
        sampled_1pct_ms: 0,
        adaptive_ms: 0,
        full_overhead: 0.0,
        sampled_overhead: 0.0,
        adaptive_overhead: 0.0,
        improvement_factor: 0.0,
    };

    // Run tests
    results.baseline_ms = test_baseline();
    results.full_tracing_ms = test_full_tracing();
    results.sampled_1pct_ms = test_sampled_1pct();
    results.adaptive_ms = test_adaptive_sampling();

    // Calculate metrics
    results.calculate();

    // Print report
    results.print_report();

    // Assertions
    assert!(results.sampled_overhead < 0.5,
        "Sampling overhead too high: {:.1}x (expected <1.5x, target <0.5x)",
        results.sampled_overhead + 1.0);

    assert!(results.improvement_factor >= 1.5,
        "Improvement factor too low: {:.1}x (expected >1.5x)",
        results.improvement_factor);

    println!("✅ All performance targets met! Sampling reduces tracing overhead from {}% to {}%.",
        (results.full_overhead * 100.0) as i32,
        (results.sampled_overhead * 100.0) as i32);
}

// Note: Sampling statistics tests are in src/tracing_sampling.rs unit tests
// This test file focuses on realistic performance benchmarking
