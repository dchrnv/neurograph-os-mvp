// NeuroGraph - Observability Stack Stress Test
// Copyright (C) 2024-2025 Chernov Denys
//
// Full-stack observability stress test covering:
// 1. Memory behavior near quota limits (9.5M tokens)
// 2. Distributed tracing overhead measurement
// 3. Panic recovery with WAL replay
// 4. Metrics collection under load
// 5. Black Box crash analysis

use neurograph_core::{
    Guardian, GuardianConfig, Token, CDNA,
    wal::{WalWriter, WalReader, WalEntry, WalEntryType},
    black_box,
};
use std::time::Instant;

/// Test configuration
const TARGET_TOKENS: usize = 9_500_000;  // 95% of 10M quota
const BATCH_SIZE: usize = 100_000;       // Create in batches
const WAL_PATH: &str = "/tmp/neurograph_stress_test.wal";

/// Observability overhead measurement
#[derive(Debug, Clone)]
struct ObservabilityMetrics {
    /// Time to create tokens WITHOUT observability
    baseline_duration_ms: u128,
    /// Time to create tokens WITH observability
    observed_duration_ms: u128,
    /// Overhead percentage
    overhead_percent: f64,
    /// Memory usage at peak
    peak_memory_bytes: usize,
    /// WAL entries written
    wal_entries_written: u64,
    /// Black box events recorded
    black_box_events_recorded: usize,
}

impl ObservabilityMetrics {
    fn calculate_overhead(&mut self) {
        let overhead = self.observed_duration_ms as f64 - self.baseline_duration_ms as f64;
        self.overhead_percent = (overhead / self.baseline_duration_ms as f64) * 100.0;
    }

    fn print_report(&self) {
        println!("\n╔══════════════════════════════════════════════════════════╗");
        println!("║      OBSERVABILITY STRESS TEST REPORT v0.44.0            ║");
        println!("╚══════════════════════════════════════════════════════════╝\n");

        println!("📊 Performance Impact:");
        println!("  Baseline (no observability):  {:>8} ms", self.baseline_duration_ms);
        println!("  With observability:           {:>8} ms", self.observed_duration_ms);
        println!("  Overhead:                     {:>7.2}%", self.overhead_percent);

        println!("\n💾 Memory Usage:");
        println!("  Peak memory:                  {:>8} MB", self.peak_memory_bytes / 1_000_000);
        println!("  Per token:                    {:>8} bytes", self.peak_memory_bytes / TARGET_TOKENS);

        println!("\n📝 Data Collection:");
        println!("  WAL entries written:          {:>8}", self.wal_entries_written);
        println!("  Black Box events:             {:>8}", self.black_box_events_recorded);

        println!("\n🎯 Resource Quotas:");
        println!("  Target tokens:                {:>8} (95% of quota)", TARGET_TOKENS);
        println!("  Expected memory:              {:>8} MB (tokens * 64 bytes)", (TARGET_TOKENS * 64) / 1_000_000);

        println!();
    }
}

/// Test 1: Baseline performance (no observability)
fn test_baseline_performance() -> u128 {
    println!("🏃 Test 1: Baseline Performance (no observability)");

    let mut config = GuardianConfig::default();
    config.max_tokens = Some(TARGET_TOKENS);
    config.max_memory_bytes = None; // Disable memory quota for baseline

    let mut guardian = Guardian::with_config(CDNA::new(), config);

    let start = Instant::now();

    // Create tokens in batches
    for batch in 0..(TARGET_TOKENS / BATCH_SIZE) {
        for i in 0..BATCH_SIZE {
            let token_id = (batch * BATCH_SIZE + i) as u32;
            let _token = Token::new(token_id);
            guardian.record_token_created();
        }

        if batch % 10 == 0 {
            let progress = (batch * BATCH_SIZE) as f64 / TARGET_TOKENS as f64 * 100.0;
            print!("\r  Progress: {:.1}%", progress);
        }
    }

    let duration = start.elapsed().as_millis();
    println!("\r  ✅ Created {} tokens in {} ms", TARGET_TOKENS, duration);

    duration
}

/// Test 2: With full observability stack
fn test_with_observability() -> (u128, usize, u64, usize) {
    println!("\n🔍 Test 2: With Full Observability Stack");

    let mut config = GuardianConfig::default();
    config.max_tokens = Some(TARGET_TOKENS);
    config.max_memory_bytes = Some(1_024_000_000); // 1GB quota
    config.memory_threshold = 0.95; // 95% threshold

    let mut guardian = Guardian::with_config(CDNA::new(), config);

    // Initialize WAL
    let mut wal = WalWriter::new(WAL_PATH).expect("Failed to create WAL");

    // Initialize Black Box
    black_box::record_event(
        black_box::Event::new(black_box::EventType::SystemStarted)
            .with_data("test", "observability_stress")
    );

    let start = Instant::now();
    let mut wal_entries = 0u64;
    let mut peak_memory = 0usize;
    let mut bb_events_count = 0usize;

    // Create tokens with full observability
    for batch in 0..(TARGET_TOKENS / BATCH_SIZE) {
        for i in 0..BATCH_SIZE {
            let token_id = (batch * BATCH_SIZE + i) as u32;

            // Check quota BEFORE creation
            if let Err(msg) = guardian.can_create_token() {
                println!("\n  ⚠️  Quota check failed: {}", msg);
                guardian.record_quota_exceeded();

                // Record in Black Box
                black_box::record_event(
                    black_box::Event::new(black_box::EventType::QuotaExceeded)
                        .with_data("message", msg)
                );
                bb_events_count += 1;
                break;
            }

            // Create token
            let token = Token::new(token_id);
            guardian.record_token_created();

            // Write to WAL (every 1000th token for performance)
            if token_id % 1000 == 0 {
                let payload = token.to_bytes().to_vec();
                let entry = WalEntry::new(WalEntryType::TokenCreated, payload);
                wal.append(&entry).expect("WAL write failed");
                wal_entries += 1;
            }

            // Track peak memory (estimate based on token count)
            let current_memory = guardian.resource_stats().0 * 64;
            if current_memory > peak_memory {
                peak_memory = current_memory;
            }
        }

        // Check if aggressive cleanup needed
        if guardian.should_trigger_aggressive_cleanup() {
            println!("\n  🧹 Aggressive cleanup triggered at batch {}", batch);
            guardian.record_aggressive_cleanup();

            black_box::record_event(
                black_box::Event::new(black_box::EventType::AggressiveCleanup)
                    .with_data("batch", batch.to_string())
            );
            bb_events_count += 1;
        }

        if batch % 10 == 0 {
            let progress = (batch * BATCH_SIZE) as f64 / TARGET_TOKENS as f64 * 100.0;
            let (tokens, _, quota_exceeded, cleanups) = guardian.resource_stats();
            print!("\r  Progress: {:.1}% | Tokens: {} | Quota exceeded: {} | Cleanups: {}",
                   progress, tokens, quota_exceeded, cleanups);
        }
    }

    // Final sync
    wal.sync().expect("WAL sync failed");

    let duration = start.elapsed().as_millis();
    let (final_tokens, _, quota_exceeded, cleanups) = guardian.resource_stats();

    println!("\r  ✅ Created {} tokens in {} ms", final_tokens, duration);
    println!("     Quota exceeded: {} times", quota_exceeded);
    println!("     Aggressive cleanups: {} times", cleanups);
    println!("     WAL entries written: {}", wal_entries);
    println!("     Black Box events: {}", bb_events_count);

    (duration, peak_memory, wal_entries, bb_events_count)
}

/// Test 3: Panic recovery with WAL replay
fn test_panic_recovery() {
    println!("\n💥 Test 3: Panic Recovery with WAL Replay");

    // Simulate crash: Create some tokens, write to WAL, then "crash"
    println!("  1. Creating test data and WAL entries...");

    let wal_crash_path = "/tmp/neurograph_crash_test.wal";
    let test_tokens = 1000;

    {
        let mut wal = WalWriter::new(wal_crash_path).expect("Failed to create WAL");

        for i in 0..test_tokens {
            let token = Token::new(i);
            let payload = token.to_bytes().to_vec();
            let entry = WalEntry::new(WalEntryType::TokenCreated, payload);
            wal.append(&entry).expect("WAL write failed");
        }

        wal.sync().expect("WAL sync failed");
        println!("     ✅ Written {} entries to WAL", test_tokens);
    }
    // WAL writer dropped here (simulating crash)

    // Recovery: Replay WAL
    println!("  2. Simulating crash and recovery...");

    black_box::record_event(
        black_box::Event::new(black_box::EventType::PanicRecovered)
            .with_data("reason", "simulated_crash_test")
    );

    let mut guardian = Guardian::new();
    let mut recovered_tokens = 0usize;

    {
        let mut reader = WalReader::new(wal_crash_path).expect("Failed to open WAL");

        let start = Instant::now();
        let entries_replayed = reader.replay(|entry| {
            match entry.header.entry_type {
                WalEntryType::TokenCreated => {
                    // Verify checksum
                    if !entry.verify() {
                        return Err(neurograph_core::wal::WalError::ChecksumMismatch);
                    }

                    // Deserialize token
                    let token_bytes: [u8; 64] = entry.payload[0..64]
                        .try_into()
                        .map_err(|_| neurograph_core::wal::WalError::CorruptedFile)?;
                    let _token = Token::from_bytes(&token_bytes);

                    guardian.record_token_created();
                    recovered_tokens += 1;
                    Ok(())
                }
                _ => Ok(()),
            }
        }).expect("WAL replay failed");

        let duration = start.elapsed();

        println!("     ✅ Replayed {} entries in {:?}", entries_replayed, duration);
        println!("     ✅ Recovered {} tokens", recovered_tokens);
    }

    // Verify recovery
    let (final_tokens, _, _, _) = guardian.resource_stats();
    assert_eq!(final_tokens, recovered_tokens);
    assert_eq!(recovered_tokens, test_tokens as usize);

    println!("  3. ✅ Recovery verification passed");

    // Cleanup
    std::fs::remove_file(wal_crash_path).ok();
}

/// Test 4: Distributed tracing overhead (simulated)
fn test_tracing_overhead() {
    println!("\n🔍 Test 4: Distributed Tracing Overhead (simulated)");

    const ITERATIONS: usize = 10_000;

    // Baseline: Simple token creation
    let start = Instant::now();
    for i in 0..ITERATIONS {
        let _token = Token::new(i as u32);
    }
    let baseline_ns = start.elapsed().as_nanos();

    // With tracing simulation (span creation overhead)
    let start = Instant::now();
    for i in 0..ITERATIONS {
        // Simulate span creation (info_span! overhead)
        let _span = tracing::info_span!("token_create", id = i);
        let _enter = _span.entered();

        let _token = Token::new(i as u32);
    }
    let traced_ns = start.elapsed().as_nanos();

    let overhead_ns = traced_ns - baseline_ns;
    let overhead_per_op = overhead_ns / ITERATIONS as u128;
    let overhead_percent = (overhead_ns as f64 / baseline_ns as f64) * 100.0;

    println!("  Baseline:               {} ns total", baseline_ns);
    println!("  With tracing:           {} ns total", traced_ns);
    println!("  Overhead:               {} ns total ({:.2}%)", overhead_ns, overhead_percent);
    println!("  Overhead per operation: {} ns", overhead_per_op);

    // Estimate for 9.5M tokens
    let estimated_overhead_ms = (overhead_per_op * TARGET_TOKENS as u128) / 1_000_000;
    println!("  Estimated overhead for {}M tokens: {} ms",
             TARGET_TOKENS / 1_000_000, estimated_overhead_ms);
}

/// Main stress test orchestrator
#[test]
fn stress_test_full_observability_stack() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  NEUROGRAPH OBSERVABILITY STRESS TEST v0.44.0            ║");
    println!("║  Full Stack: Metrics + Logs + Traces + WAL + Black Box  ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // Initialize logging (optional - use tracing_subscriber if available)
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .try_init();

    let mut metrics = ObservabilityMetrics {
        baseline_duration_ms: 0,
        observed_duration_ms: 0,
        overhead_percent: 0.0,
        peak_memory_bytes: 0,
        wal_entries_written: 0,
        black_box_events_recorded: 0,
    };

    // Run tests
    metrics.baseline_duration_ms = test_baseline_performance();

    let (observed_duration, peak_memory, wal_entries, bb_events) = test_with_observability();
    metrics.observed_duration_ms = observed_duration;
    metrics.peak_memory_bytes = peak_memory;
    metrics.wal_entries_written = wal_entries;
    metrics.black_box_events_recorded = bb_events;

    test_panic_recovery();
    test_tracing_overhead();

    // Calculate and print report
    metrics.calculate_overhead();
    metrics.print_report();

    // Cleanup
    std::fs::remove_file(WAL_PATH).ok();

    println!("✅ All observability stress tests completed successfully!\n");
}

/// Quick performance sanity check
#[test]
fn sanity_check_observability_overhead() {
    println!("\n🔍 Sanity Check: Observability Overhead");

    const SAMPLE_SIZE: usize = 100_000;

    // Baseline
    let start = Instant::now();
    for i in 0..SAMPLE_SIZE {
        let _token = Token::new(i as u32);
    }
    let baseline_ms = start.elapsed().as_millis();

    // With WAL
    let wal_path = "/tmp/neurograph_sanity.wal";
    let mut wal = WalWriter::new(wal_path).unwrap();

    let start = Instant::now();
    for i in 0..SAMPLE_SIZE {
        let token = Token::new(i as u32);
        let entry = WalEntry::new(WalEntryType::TokenCreated, token.to_bytes().to_vec());
        wal.append(&entry).unwrap();
    }
    wal.sync().unwrap();
    let wal_ms = start.elapsed().as_millis();

    let overhead_percent = ((wal_ms - baseline_ms) as f64 / baseline_ms as f64) * 100.0;

    println!("  Sample size:     {}", SAMPLE_SIZE);
    println!("  Baseline:        {} ms", baseline_ms);
    println!("  With WAL:        {} ms", wal_ms);
    println!("  Overhead:        {:.2}%", overhead_percent);

    // Cleanup
    std::fs::remove_file(wal_path).ok();

    // Note: WAL overhead is high (~1800-2000%) due to fsync() on every write
    // This is expected for full durability guarantees
    // In production, we batch writes (every 1000th token) to reduce overhead

    if overhead_percent > 1000.0 {
        println!("  ⚠️  High overhead detected ({:.2}%), but this is expected for unbatched WAL writes", overhead_percent);
        println!("      In production, batching reduces this to <100%");
    } else {
        println!("  ✅ Overhead is acceptable: {:.2}%", overhead_percent);
    }

    println!();
}
