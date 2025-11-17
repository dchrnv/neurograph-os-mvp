// NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
// Copyright (C) 2024-2025 Chernov Denys

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Token Benchmarks for v0.27.0
//!
//! Performance measurements for Token operations:
//! - token_creation: <10 ns target
//! - token_similarity: <50 ns target
//! - token_serialization: <5 ns target (zero-copy)
//! - token_batch_creation: <100 μs target (10k tokens)

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use neurograph_core::{Token, CoordinateSpace, EntityType};
use neurograph_core::token::flags;

/// Benchmark: Token creation (target: <10 ns)
fn bench_token_creation(c: &mut Criterion) {
    c.bench_function("token_creation", |b| {
        let mut id = 0u32;
        b.iter(|| {
            id = id.wrapping_add(1);
            black_box(Token::new(black_box(id)))
        })
    });
}

/// Benchmark: Cosine similarity between two tokens (target: <50 ns)
/// Uses L8 Abstract space for semantic similarity
fn bench_token_similarity(c: &mut Criterion) {
    let mut token1 = Token::new(1);
    token1.set_coordinates(CoordinateSpace::L8Abstract, 1.0, 0.5, 0.3);

    let mut token2 = Token::new(2);
    token2.set_coordinates(CoordinateSpace::L8Abstract, 0.8, 0.6, 0.4);

    c.bench_function("token_similarity", |b| {
        b.iter(|| {
            let coords1 = black_box(token1.get_coordinates(CoordinateSpace::L8Abstract));
            let coords2 = black_box(token2.get_coordinates(CoordinateSpace::L8Abstract));

            // Cosine similarity calculation
            let mut dot_product = 0.0f32;
            let mut mag1 = 0.0f32;
            let mut mag2 = 0.0f32;

            for i in 0..3 {
                dot_product += coords1[i] * coords2[i];
                mag1 += coords1[i] * coords1[i];
                mag2 += coords2[i] * coords2[i];
            }

            let similarity = dot_product / (mag1.sqrt() * mag2.sqrt());
            black_box(similarity)
        })
    });
}

/// Benchmark: Serialization and deserialization (target: <5 ns for zero-copy)
fn bench_token_serialization(c: &mut Criterion) {
    let mut token = Token::new(42);
    token.set_coordinates(CoordinateSpace::L1Physical, 1.0, 2.0, 3.0);
    token.set_entity_type(EntityType::Object);
    token.weight = 0.5;
    token.set_field_radius(1.5);
    token.set_field_strength(0.8);

    let mut group = c.benchmark_group("token_serialization");

    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(black_box(&token).to_bytes())
        })
    });

    let bytes = token.to_bytes();
    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(Token::from_bytes(black_box(&bytes)))
        })
    });

    group.finish();
}

/// Benchmark: Batch token creation (target: <100 μs for 10k tokens)
fn bench_token_batch_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("token_batch_creation");

    for size in [100, 1000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut tokens = Vec::with_capacity(size);
                for i in 0..size {
                    tokens.push(Token::new(i as u32));
                }
                black_box(tokens)
            })
        });
    }

    group.finish();
}

/// Benchmark: Coordinate encoding/decoding (measuring fixed-point conversion overhead)
fn bench_coordinate_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("coordinate_encoding");

    group.bench_function("encode", |b| {
        b.iter(|| {
            black_box(Token::encode_coordinate(
                black_box(10.5),
                black_box(CoordinateSpace::L1Physical)
            ))
        })
    });

    group.bench_function("decode", |b| {
        b.iter(|| {
            black_box(Token::decode_coordinate(
                black_box(1050),
                black_box(CoordinateSpace::L1Physical)
            ))
        })
    });

    group.finish();
}

/// Benchmark: Flag operations (bit manipulation)
fn bench_flag_operations(c: &mut Criterion) {
    let mut token = Token::new(1);
    let mut group = c.benchmark_group("flag_operations");

    group.bench_function("has_flag", |b| {
        b.iter(|| {
            black_box(black_box(&token).has_flag(flags::ACTIVE))
        })
    });

    group.bench_function("set_flag", |b| {
        b.iter(|| {
            black_box(&mut token).set_flag(flags::PERSISTENT)
        })
    });

    group.bench_function("clear_flag", |b| {
        b.iter(|| {
            black_box(&mut token).clear_flag(flags::PERSISTENT)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_token_creation,
    bench_token_similarity,
    bench_token_serialization,
    bench_token_batch_creation,
    bench_coordinate_encoding,
    bench_flag_operations
);
criterion_main!(benches);