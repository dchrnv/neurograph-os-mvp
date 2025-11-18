// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2025 Chernov Denys

//! Connection v3.0 Performance Benchmarks
//!
//! Comprehensive benchmarks comparing v3.0 (64 bytes, learning-capable)
//! vs v1.0 (32 bytes, static) across various operations.
//!
//! Run with: cargo bench --bench connection_v3_bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use neurograph_core::connection_v3::{
    ConnectionV3, ConnectionType, ConnectionMutability, ConnectionProposal, ConnectionField,
    guardian_validation, learning_stats::*,
};

// ========================================
// Baseline: Connection v1.0 (32 bytes)
// ========================================

/// Simplified Connection v1.0 structure for comparison
#[repr(C)]
#[derive(Clone, Copy)]
struct ConnectionV1 {
    token_a_id: u32,
    token_b_id: u32,
    connection_type: u8,
    rigidity: u8,
    pull_strength: f32,
    preferred_distance: f32,
    last_activation: u32,
    activation_count: u16,
    flags: u8,
    _padding: [u8; 3],
}

impl ConnectionV1 {
    fn new(token_a_id: u32, token_b_id: u32) -> Self {
        Self {
            token_a_id,
            token_b_id,
            connection_type: 0,
            rigidity: 128,
            pull_strength: 1.0,
            preferred_distance: 5.0,
            last_activation: 0,
            activation_count: 0,
            flags: 0,
            _padding: [0; 3],
        }
    }

    fn activate(&mut self) {
        self.activation_count = self.activation_count.saturating_add(1);
        self.rigidity = self.rigidity.saturating_add(1);
        self.flags |= 0x01; // ACTIVE flag
    }
}

// ========================================
// Benchmark 1: Creation Performance
// ========================================

fn bench_connection_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("connection_creation");

    group.bench_function("v1.0_new", |b| {
        b.iter(|| {
            let conn = ConnectionV1::new(black_box(100), black_box(200));
            black_box(conn)
        })
    });

    group.bench_function("v3.0_new", |b| {
        b.iter(|| {
            let conn = ConnectionV3::new(black_box(100), black_box(200));
            black_box(conn)
        })
    });

    group.bench_function("v3.0_with_type", |b| {
        b.iter(|| {
            let mut conn = ConnectionV3::new(black_box(100), black_box(200));
            conn.set_connection_type(ConnectionType::Cause);
            black_box(conn)
        })
    });

    group.finish();
}

// ========================================
// Benchmark 2: Activation Performance
// ========================================

fn bench_connection_activation(c: &mut Criterion) {
    let mut group = c.benchmark_group("connection_activation");
    group.throughput(Throughput::Elements(1));

    group.bench_function("v1.0_activate", |b| {
        let mut conn_v1 = ConnectionV1::new(100, 200);
        b.iter(|| {
            conn_v1.activate();
            black_box(conn_v1)
        })
    });

    group.bench_function("v3.0_activate", |b| {
        let mut conn_v3 = ConnectionV3::new(100, 200);
        conn_v3.set_connection_type(ConnectionType::Cause);
        b.iter(|| {
            conn_v3.activate();
            black_box(conn_v3)
        })
    });

    group.finish();
}

// ========================================
// Benchmark 3: Learning Operations
// ========================================

fn bench_learning_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("learning_operations");
    group.throughput(Throughput::Elements(1));

    group.bench_function("update_confidence_success", |b| {
        let mut conn_learnable = ConnectionV3::new(100, 200);
        conn_learnable.set_connection_type(ConnectionType::Cause);
        conn_learnable.mutability = ConnectionMutability::Learnable as u8;
        conn_learnable.confidence = 128;
        b.iter(|| {
            conn_learnable.update_confidence(true);
            black_box(conn_learnable)
        })
    });

    group.bench_function("update_confidence_failure", |b| {
        let mut conn_learnable = ConnectionV3::new(100, 200);
        conn_learnable.set_connection_type(ConnectionType::Cause);
        conn_learnable.mutability = ConnectionMutability::Learnable as u8;
        conn_learnable.confidence = 128;
        b.iter(|| {
            conn_learnable.update_confidence(false);
            black_box(conn_learnable)
        })
    });

    group.bench_function("hypothesis_fast_learning", |b| {
        let mut conn_hypothesis = ConnectionV3::new(100, 200);
        conn_hypothesis.mutability = ConnectionMutability::Hypothesis as u8;
        conn_hypothesis.learning_rate = 128;
        conn_hypothesis.confidence = 128;
        b.iter(|| {
            conn_hypothesis.update_confidence(true);
            black_box(conn_hypothesis)
        })
    });

    group.bench_function("apply_decay", |b| {
        let mut conn_hypothesis = ConnectionV3::new(100, 200);
        conn_hypothesis.mutability = ConnectionMutability::Hypothesis as u8;
        conn_hypothesis.learning_rate = 128;
        conn_hypothesis.confidence = 128;
        b.iter(|| {
            conn_hypothesis.apply_decay();
            black_box(conn_hypothesis)
        })
    });

    group.finish();
}

// ========================================
// Benchmark 4: Proposal System
// ========================================

fn bench_proposal_system(c: &mut Criterion) {
    let mut group = c.benchmark_group("proposal_system");
    group.throughput(Throughput::Elements(1));

    let mut conn = ConnectionV3::new(100, 200);
    conn.mutability = ConnectionMutability::Learnable as u8;
    conn.confidence = 128;

    let modify_proposal = ConnectionProposal::Modify {
        connection_id: 0,
        field: ConnectionField::Confidence,
        old_value: 0.5,
        new_value: 0.75,
        justification: "Benchmark test".to_string(),
        evidence_count: 10,
    };

    group.bench_function("apply_proposal_basic", |b| {
        b.iter(|| {
            let mut conn_copy = conn;
            conn_copy.apply_proposal(&modify_proposal).unwrap();
            black_box(conn_copy)
        })
    });

    group.bench_function("apply_proposal_with_guardian", |b| {
        b.iter(|| {
            let mut conn_copy = conn;
            conn_copy.apply_proposal_with_guardian(&modify_proposal).unwrap();
            black_box(conn_copy)
        })
    });

    let promote_proposal = ConnectionProposal::Promote {
        connection_id: 0,
        evidence_count: 25,
        justification: "Benchmark promotion".to_string(),
    };

    let mut conn_hypo = ConnectionV3::new(100, 200);
    conn_hypo.mutability = ConnectionMutability::Hypothesis as u8;

    group.bench_function("promote_hypothesis_to_learnable", |b| {
        b.iter(|| {
            let mut conn_copy = conn_hypo;
            conn_copy.apply_proposal_with_guardian(&promote_proposal).unwrap();
            black_box(conn_copy)
        })
    });

    let create_proposal = ConnectionProposal::Create {
        token_a_id: 100,
        token_b_id: 200,
        connection_type: ConnectionType::Cause as u8,
        initial_strength: 3.0,
        initial_confidence: 128,
        justification: "Benchmark creation".to_string(),
    };

    group.bench_function("create_from_proposal", |b| {
        b.iter(|| {
            let conn = ConnectionV3::from_proposal(&create_proposal).unwrap();
            black_box(conn)
        })
    });

    group.bench_function("create_from_proposal_with_guardian", |b| {
        b.iter(|| {
            let conn = ConnectionV3::from_proposal_with_guardian(&create_proposal).unwrap();
            black_box(conn)
        })
    });

    group.finish();
}

// ========================================
// Benchmark 5: Guardian Validation
// ========================================

fn bench_guardian_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("guardian_validation");
    group.throughput(Throughput::Elements(1));

    let conn = ConnectionV3::new(100, 200);

    let valid_proposal = ConnectionProposal::Modify {
        connection_id: 0,
        field: ConnectionField::PullStrength,
        old_value: 1.0,
        new_value: 5.0,
        justification: "Valid strength".to_string(),
        evidence_count: 10,
    };

    group.bench_function("validate_proposal", |b| {
        b.iter(|| {
            guardian_validation::validate_proposal(&conn, &valid_proposal).unwrap();
            black_box(())
        })
    });

    group.bench_function("validate_connection_state", |b| {
        b.iter(|| {
            guardian_validation::validate_connection_state(&conn).unwrap();
            black_box(())
        })
    });

    let invalid_proposal = ConnectionProposal::Modify {
        connection_id: 0,
        field: ConnectionField::PullStrength,
        old_value: 1.0,
        new_value: 15.0, // Invalid
        justification: "Too strong".to_string(),
        evidence_count: 10,
    };

    group.bench_function("reject_invalid_proposal", |b| {
        b.iter(|| {
            let result = guardian_validation::validate_proposal(&conn, &invalid_proposal);
            black_box(result)
        })
    });

    group.finish();
}

// ========================================
// Benchmark 6: Learning Statistics
// ========================================

fn bench_learning_statistics(c: &mut Criterion) {
    let mut group = c.benchmark_group("learning_statistics");
    group.throughput(Throughput::Elements(1));

    group.bench_function("record_success", |b| {
        let mut stats = ConnectionLearningStats::new();
        b.iter(|| {
            stats.record_success();
            black_box(stats.success_rate)
        })
    });

    group.bench_function("record_failure", |b| {
        let mut stats = ConnectionLearningStats::new();
        b.iter(|| {
            stats.record_failure();
            black_box(stats.success_rate)
        })
    });

    group.bench_function("record_cooccurrence", |b| {
        let mut stats = ConnectionLearningStats::new();
        b.iter(|| {
            stats.record_cooccurrence(black_box(100));
            black_box(stats.avg_time_delta_ms)
        })
    });

    group.bench_function("generate_confidence_proposal", |b| {
        let mut stats = ConnectionLearningStats::new();
        for _ in 0..20 {
            stats.record_success();
        }
        let conn = ConnectionV3::new(100, 200);
        b.iter(|| {
            let proposal = stats.generate_confidence_proposal(&conn, 10);
            black_box(proposal)
        })
    });

    group.bench_function("generate_promote_proposal", |b| {
        let mut stats = ConnectionLearningStats::new();
        for _ in 0..20 {
            stats.record_success();
        }
        let mut conn_hypo = ConnectionV3::new(100, 200);
        conn_hypo.mutability = ConnectionMutability::Hypothesis as u8;
        b.iter(|| {
            let proposal = stats.generate_promote_proposal(&conn_hypo, 20, 0.8);
            black_box(proposal)
        })
    });

    group.finish();
}

// ========================================
// Benchmark 7: Temporal Pattern Detection
// ========================================

fn bench_temporal_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("temporal_patterns");

    // Small dataset (10 observations)
    let observations_small: Vec<(u32, u32, i64)> = (0..10)
        .map(|i| (100, 200, 100 + i * 10))
        .collect();

    group.throughput(Throughput::Elements(observations_small.len() as u64));
    group.bench_function("detect_pattern_10_obs", |b| {
        b.iter(|| {
            let pattern = detect_temporal_pattern(
                black_box(100),
                black_box(200),
                &observations_small,
                5,
            );
            black_box(pattern)
        })
    });

    // Medium dataset (100 observations)
    let observations_medium: Vec<(u32, u32, i64)> = (0..100)
        .map(|i| (100, 200, 100 + i * 10))
        .collect();

    group.throughput(Throughput::Elements(observations_medium.len() as u64));
    group.bench_function("detect_pattern_100_obs", |b| {
        b.iter(|| {
            let pattern = detect_temporal_pattern(
                black_box(100),
                black_box(200),
                &observations_medium,
                5,
            );
            black_box(pattern)
        })
    });

    // Large dataset (1000 observations)
    let observations_large: Vec<(u32, u32, i64)> = (0..1000)
        .map(|i| (100, 200, 100 + i * 10))
        .collect();

    group.throughput(Throughput::Elements(observations_large.len() as u64));
    group.bench_function("detect_pattern_1000_obs", |b| {
        b.iter(|| {
            let pattern = detect_temporal_pattern(
                black_box(100),
                black_box(200),
                &observations_large,
                5,
            );
            black_box(pattern)
        })
    });

    // Pattern → proposal generation
    let pattern = TemporalPattern {
        token_a_id: 100,
        token_b_id: 200,
        connection_type: ConnectionType::After as u8,
        cooccurrence_count: 10,
        confidence: 0.7,
        avg_time_delta_ms: 150,
    };

    group.bench_function("pattern_to_proposal", |b| {
        b.iter(|| {
            let proposal = pattern.generate_create_proposal();
            black_box(proposal)
        })
    });

    group.finish();
}

// ========================================
// Benchmark 8: Batch Operations
// ========================================

fn bench_batch_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_operations");

    for batch_size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*batch_size));

        group.bench_with_input(
            BenchmarkId::new("v1.0_batch_create", batch_size),
            batch_size,
            |b, &size| {
                b.iter(|| {
                    let connections: Vec<ConnectionV1> = (0..size)
                        .map(|i| ConnectionV1::new(i as u32, (i + 1) as u32))
                        .collect();
                    black_box(connections)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("v3.0_batch_create", batch_size),
            batch_size,
            |b, &size| {
                b.iter(|| {
                    let connections: Vec<ConnectionV3> = (0..size)
                        .map(|i| ConnectionV3::new(i as u32, (i + 1) as u32))
                        .collect();
                    black_box(connections)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("v3.0_batch_activate", batch_size),
            batch_size,
            |b, &size| {
                b.iter(|| {
                    let mut connections: Vec<ConnectionV3> = (0..size)
                        .map(|i| ConnectionV3::new(i as u32, (i + 1) as u32))
                        .collect();
                    for conn in connections.iter_mut() {
                        conn.activate();
                    }
                    black_box(connections)
                })
            },
        );
    }

    group.finish();
}

// ========================================
// Benchmark 9: Memory Layout
// ========================================

fn bench_memory_layout(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_layout");

    group.bench_function("v1.0_size_check", |b| {
        b.iter(|| {
            let size = std::mem::size_of::<ConnectionV1>();
            black_box(size)
        })
    });

    group.bench_function("v3.0_size_check", |b| {
        b.iter(|| {
            let size = std::mem::size_of::<ConnectionV3>();
            black_box(size)
        })
    });

    // Cache line alignment test
    let v1_array: Vec<ConnectionV1> = (0..1000)
        .map(|i| ConnectionV1::new(i, i + 1))
        .collect();

    group.bench_function("v1.0_cache_iteration", |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for conn in &v1_array {
                sum += conn.activation_count as u64;
            }
            black_box(sum)
        })
    });

    let v3_array: Vec<ConnectionV3> = (0..1000)
        .map(|i| ConnectionV3::new(i, i + 1))
        .collect();

    group.bench_function("v3.0_cache_iteration", |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for conn in &v3_array {
                sum += conn.activation_count as u64;
            }
            black_box(sum)
        })
    });

    group.finish();
}

// ========================================
// Benchmark 10: E2E Learning Cycle
// ========================================

fn bench_e2e_learning_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e_learning_cycle");
    group.throughput(Throughput::Elements(1));

    // Full cycle: Pattern detection → Creation → Learning → Promotion
    group.bench_function("complete_learning_cycle", |b| {
        b.iter(|| {
            // 1. Detect pattern
            let observations = vec![
                (100, 200, 100),
                (100, 200, 120),
                (100, 200, 110),
                (100, 200, 105),
                (100, 200, 115),
            ];
            let pattern = detect_temporal_pattern(100, 200, &observations, 5).unwrap();

            // 2. Create connection
            let create_proposal = pattern.generate_create_proposal().unwrap();
            let mut conn = ConnectionV3::from_proposal_with_guardian(&create_proposal).unwrap();

            // 3. Learning
            let mut stats = ConnectionLearningStats::new();
            for _ in 0..25 {
                stats.record_success();
            }

            // 4. Adjust confidence
            if let Some(conf_proposal) = stats.generate_confidence_proposal(&conn, 20) {
                conn.apply_proposal_with_guardian(&conf_proposal).unwrap();
            }

            // 5. Promote
            if let Some(promote_proposal) = stats.generate_promote_proposal(&conn, 20, 0.8) {
                conn.apply_proposal_with_guardian(&promote_proposal).unwrap();
            }

            black_box(conn)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_connection_creation,
    bench_connection_activation,
    bench_learning_operations,
    bench_proposal_system,
    bench_guardian_validation,
    bench_learning_statistics,
    bench_temporal_patterns,
    bench_batch_operations,
    bench_memory_layout,
    bench_e2e_learning_cycle,
);

criterion_main!(benches);
