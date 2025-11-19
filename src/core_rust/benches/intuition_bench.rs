// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2025 Chernov Denys

//! IntuitionEngine v3.0 Performance Benchmarks
//!
//! Measures performance of Fast Path (Reflex Layer):
//! - GridHash computation: Target <10ns
//! - AssociativeMemory lookup: Target <30ns
//! - Fast Path total (hash + lookup): Target <50ns
//! - Speedup vs Slow Path: Target >10,000x
//!
//! Run with: cargo bench --bench intuition_bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use neurograph_core::token::Token;
use neurograph_core::reflex_layer::{
    ShiftConfig, AssociativeMemory, compute_grid_hash,
};

// ================================================================================================
// GridHash Benchmarks
// ================================================================================================

/// Benchmark: GridHash computation (target: <10ns)
fn bench_grid_hash(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_hash");

    let token = Token::new(100);
    let config = ShiftConfig::default();

    group.bench_function("compute_hash", |b| {
        b.iter(|| {
            black_box(compute_grid_hash(
                black_box(&token),
                black_box(&config)
            ))
        })
    });

    group.finish();
}

/// Benchmark: GridHash with different shift configs
fn bench_grid_hash_shift_variations(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_hash_shift");

    let token = Token::new(100);

    for shift in [4, 6, 8, 10].iter() {
        let config = ShiftConfig::uniform(*shift);
        group.bench_with_input(
            BenchmarkId::from_parameter(shift),
            shift,
            |b, _| {
                b.iter(|| {
                    black_box(compute_grid_hash(
                        black_box(&token),
                        black_box(&config)
                    ))
                })
            },
        );
    }

    group.finish();
}

// ================================================================================================
// AssociativeMemory Benchmarks
// ================================================================================================

/// Benchmark: AssociativeMemory lookup (target: <30ns)
fn bench_associative_memory_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("associative_memory");

    // Setup: Create memory with varying sizes
    for size in [10, 100, 1_000, 10_000].iter() {
        let memory = AssociativeMemory::new();

        // Populate with reflexes
        for i in 0..*size {
            memory.insert(i as u64, i as u64 + 1000);
        }

        let test_hash = (size / 2) as u64;  // Middle entry

        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, _| {
                b.iter(|| {
                    black_box(memory.lookup(black_box(test_hash)))
                })
            },
        );
    }

    group.finish();
}

/// Benchmark: AssociativeMemory insert (background operation)
fn bench_associative_memory_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("associative_memory_insert");

    let memory = AssociativeMemory::new();

    group.bench_function("insert", |b| {
        let mut counter = 0u64;
        b.iter(|| {
            memory.insert(black_box(counter), black_box(counter + 1000));
            counter += 1;
        })
    });

    group.finish();
}

/// Benchmark: Collision handling (multiple candidates)
fn bench_associative_memory_collisions(c: &mut Criterion) {
    let mut group = c.benchmark_group("associative_memory_collision");

    for num_candidates in [1, 2, 4, 8].iter() {
        let memory = AssociativeMemory::new();

        // Create collision: same hash, multiple connections
        let hash = 12345u64;
        for i in 0..*num_candidates {
            memory.insert(hash, hash + i as u64);
        }

        group.bench_with_input(
            BenchmarkId::from_parameter(num_candidates),
            num_candidates,
            |b, _| {
                b.iter(|| {
                    let candidates = black_box(memory.lookup(black_box(hash))).unwrap();
                    // Simulate collision resolution (iterate candidates)
                    for &_conn_id in candidates.iter() {
                        black_box(_conn_id);
                    }
                })
            },
        );
    }

    group.finish();
}

// ================================================================================================
// Fast Path E2E Benchmarks
// ================================================================================================

/// Benchmark: Complete Fast Path (hash + lookup + eligibility check)
fn bench_fast_path_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("fast_path_e2e");

    // Setup
    let shift_config = ShiftConfig::default();
    let memory = AssociativeMemory::new();

    // Create 1000 reflexes
    let mut tokens = Vec::new();
    for i in 0..1000 {
        let mut token = Token::new(i);
        token.coordinates[0] = [(i * 100) as i16, (i * 200) as i16, (i * 300) as i16];

        let hash = compute_grid_hash(&token, &shift_config);
        memory.insert(hash, i as u64);

        tokens.push(token);
    }

    // Test token (known reflex)
    let test_token = &tokens[500];

    group.bench_function("fast_path_hit", |b| {
        b.iter(|| {
            // 1. Compute hash
            let hash = compute_grid_hash(black_box(test_token), &shift_config);

            // 2. Lookup
            let candidates = memory.lookup(hash);

            // 3. Return first candidate (simplified)
            black_box(candidates)
        })
    });

    // Test unknown token (miss)
    let mut unknown_token = Token::new(9999);
    unknown_token.coordinates[0] = [30000, 30000, 30000];

    group.bench_function("fast_path_miss", |b| {
        b.iter(|| {
            let hash = compute_grid_hash(black_box(&unknown_token), &shift_config);
            let candidates = memory.lookup(hash);
            black_box(candidates)
        })
    });

    group.finish();
}

/// Benchmark: Batch operations (simulate real-world usage)
fn bench_fast_path_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("fast_path_batch");

    let shift_config = ShiftConfig::default();
    let memory = AssociativeMemory::new();

    // Create reflexes
    for i in 0..1000 {
        let mut token = Token::new(i);
        token.coordinates[0] = [(i * 100) as i16, 0, 0];
        let hash = compute_grid_hash(&token, &shift_config);
        memory.insert(hash, i as u64);
    }

    // Batch of queries (mix of hits and misses)
    let mut query_tokens = Vec::new();
    for i in 0..100 {
        let mut token = Token::new(i);
        if i < 80 {
            // 80% hits
            token.coordinates[0] = [(i * 100) as i16, 0, 0];
        } else {
            // 20% misses
            token.coordinates[0] = [30000, 30000, 30000];
        }
        query_tokens.push(token);
    }

    group.throughput(Throughput::Elements(100));
    group.bench_function("batch_100_queries", |b| {
        b.iter(|| {
            for token in &query_tokens {
                let hash = compute_grid_hash(token, &shift_config);
                let _result = memory.lookup(hash);
                black_box(_result);
            }
        })
    });

    group.finish();
}

// ================================================================================================
// Comparison: Fast Path vs Slow Path
// ================================================================================================

/// Benchmark: Fast Path vs Slow Path speedup
fn bench_fast_vs_slow_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("fast_vs_slow");

    let token = Token::new(100);
    let shift_config = ShiftConfig::default();
    let memory = AssociativeMemory::new();

    // Add reflex
    let hash = compute_grid_hash(&token, &shift_config);
    memory.insert(hash, 999);

    // Fast Path
    group.bench_function("fast_path", |b| {
        b.iter(|| {
            let hash = compute_grid_hash(black_box(&token), &shift_config);
            black_box(memory.lookup(hash))
        })
    });

    // Slow Path (simulated: just a delay representing ADNA computation)
    group.bench_function("slow_path_simulated", |b| {
        b.iter(|| {
            // Simulate ADNA forward pass (~1-10ms)
            // For benchmark purposes, use busy loop to represent computation
            let mut sum = 0u64;
            for i in 0..10_000 {  // ~10μs on modern CPU
                sum = sum.wrapping_add(i);
            }
            black_box(sum)
        })
    });

    group.finish();
}

// ================================================================================================
// Criterion Groups
// ================================================================================================

criterion_group!(
    benches,
    bench_grid_hash,
    bench_grid_hash_shift_variations,
    bench_associative_memory_lookup,
    bench_associative_memory_insert,
    bench_associative_memory_collisions,
    bench_fast_path_e2e,
    bench_fast_path_batch,
    bench_fast_vs_slow_path,
);

criterion_main!(benches);
