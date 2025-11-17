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

//! Grid Benchmarks for v0.27.0
//!
//! Performance measurements for Grid operations:
//! - grid_insert: <100 ns target
//! - grid_knn_search: <5 μs target (k=10 from 10k tokens)
//! - grid_range_query: <10 μs target
//! - grid_batch_insert: <100 μs target (1k tokens)

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use neurograph_core::{Grid, Token, CoordinateSpace};

/// Benchmark: Single token insert (target: <100 ns)
fn bench_grid_insert(c: &mut Criterion) {
    let mut grid = Grid::new();

    c.bench_function("grid_insert", |b| {
        let mut id = 0u32;
        b.iter(|| {
            id = id.wrapping_add(1);
            let mut token = Token::new(id);
            token.set_coordinates(CoordinateSpace::L8Abstract,
                (id as f32 * 0.001) % 10.0,
                (id as f32 * 0.002) % 10.0,
                (id as f32 * 0.003) % 10.0
            );
            grid.add(black_box(token)).ok();
        })
    });
}

/// Benchmark: KNN search (target: <5 μs for k=10 from 10k tokens)
fn bench_grid_knn_search(c: &mut Criterion) {
    let mut grid = Grid::new();

    // Populate grid with 10,000 tokens
    for i in 0..10_000 {
        let mut token = Token::new(i);
        token.set_coordinates(
            CoordinateSpace::L8Abstract,
            (i as f32 * 0.001) % 10.0,
            (i as f32 * 0.002) % 10.0,
            (i as f32 * 0.003) % 10.0,
        );
        grid.add(token).ok();
    }

    let mut group = c.benchmark_group("grid_knn_search");

    for k in [1, 5, 10, 20].iter() {
        group.bench_with_input(BenchmarkId::new("k", k), k, |b, &k| {
            b.iter(|| {
                // Search for k neighbors around token 5000
                grid.find_neighbors(
                    black_box(5000),
                    black_box(CoordinateSpace::L8Abstract),
                    black_box(5.0),
                )
            })
        });
    }

    group.finish();
}

/// Benchmark: Range query (target: <10 μs)
fn bench_grid_range_query(c: &mut Criterion) {
    let mut grid = Grid::new();

    // Populate grid with 10,000 tokens
    for i in 0..10_000 {
        let mut token = Token::new(i);
        token.set_coordinates(
            CoordinateSpace::L8Abstract,
            (i as f32 * 0.001) % 10.0,
            (i as f32 * 0.002) % 10.0,
            (i as f32 * 0.003) % 10.0,
        );
        grid.add(token).ok();
    }

    let mut group = c.benchmark_group("grid_range_query");

    for radius in [1.0, 2.0, 5.0, 10.0].iter() {
        group.bench_with_input(BenchmarkId::new("radius", radius), radius, |b, &radius| {
            b.iter(|| {
                grid.find_neighbors(
                    black_box(5000),
                    black_box(CoordinateSpace::L8Abstract),
                    black_box(radius),
                )
            })
        });
    }

    group.finish();
}

/// Benchmark: Batch insert (target: <100 μs for 1k tokens)
fn bench_grid_batch_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_batch_insert");

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut grid = Grid::new();
                for i in 0..size {
                    let mut token = Token::new(i as u32);
                    token.set_coordinates(
                        CoordinateSpace::L8Abstract,
                        (i as f32 * 0.001) % 10.0,
                        (i as f32 * 0.002) % 10.0,
                        (i as f32 * 0.003) % 10.0,
                    );
                    grid.add(token).ok();
                }
                black_box(grid)
            })
        });
    }

    group.finish();
}

/// Benchmark: Token removal
fn bench_grid_remove(c: &mut Criterion) {
    let mut grid = Grid::new();

    // Populate grid with 1,000 tokens
    for i in 0..1_000 {
        let mut token = Token::new(i);
        token.set_coordinates(
            CoordinateSpace::L8Abstract,
            (i as f32 * 0.001) % 10.0,
            (i as f32 * 0.002) % 10.0,
            (i as f32 * 0.003) % 10.0,
        );
        grid.add(token).ok();
    }

    c.bench_function("grid_remove", |b| {
        let mut id = 0u32;
        b.iter(|| {
            id = (id + 1) % 1000;
            grid.remove(black_box(id))
        })
    });
}

criterion_group!(
    benches,
    bench_grid_insert,
    bench_grid_knn_search,
    bench_grid_range_query,
    bench_grid_batch_insert,
    bench_grid_remove
);
criterion_main!(benches);