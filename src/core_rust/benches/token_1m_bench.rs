// NeuroGraph OS - 1M Tokens Benchmark
// Performance benchmarks for 1 million token operations

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use neurograph_core::{Token, Graph, BootstrapLibrary, BootstrapConfig};
use std::sync::{Arc, RwLock};

/// Benchmark token creation at scale
fn bench_token_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("token_creation_scale");

    for size in [1_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let tokens: Vec<Token> = (0..size)
                    .map(|i| Token::new(black_box(i as u64)))
                    .collect();
                black_box(tokens);
            });
        });
    }

    group.finish();
}

/// Benchmark graph insertion at scale
fn bench_graph_insertion(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_insertion_scale");
    group.sample_size(10); // Reduce samples for large benchmarks

    for size in [1_000, 10_000, 50_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let graph = Arc::new(RwLock::new(Graph::new()));
                for i in 0..size {
                    let token = Token::new(black_box(i as u64));
                    graph.write().unwrap().add_token(token);
                }
                black_box(graph);
            });
        });
    }

    group.finish();
}

/// Benchmark similarity calculations at scale
fn bench_similarity_scale(c: &mut Criterion) {
    let mut group = c.benchmark_group("similarity_scale");

    let reference = Token::new(0);

    for size in [1_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut sum = 0.0f32;
                for i in 0..size {
                    let token = Token::new(black_box(i as u64));
                    sum += reference.similarity(&token);
                }
                black_box(sum);
            });
        });
    }

    group.finish();
}

/// Benchmark Bootstrap semantic search with growing dataset
fn bench_bootstrap_search_scale(c: &mut Criterion) {
    let mut group = c.benchmark_group("bootstrap_search_scale");
    group.sample_size(10);

    for size in [1_000, 10_000, 50_000, 100_000].iter() {
        // Pre-populate bootstrap
        let config = BootstrapConfig::default();
        let mut bootstrap = BootstrapLibrary::new(config);

        for i in 0..*size {
            bootstrap.add_token(Token::new(i as u64));
        }

        let query = Token::new(size / 2); // Middle token

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, _size| {
                b.iter(|| {
                    let results = bootstrap.find_nearest(black_box(&query), 10);
                    black_box(results);
                });
            }
        );
    }

    group.finish();
}

/// Benchmark concurrent read performance with 1M tokens
fn bench_concurrent_reads(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_reads_1m");
    group.sample_size(10);

    // Pre-populate with 100k tokens (1M too slow for repeated bench)
    let graph = Arc::new(RwLock::new(Graph::new()));
    for i in 0..100_000 {
        graph.write().unwrap().add_token(Token::new(i));
    }

    group.bench_function("graph_read_100k", |b| {
        b.iter(|| {
            let g = graph.read().unwrap();
            black_box(g.token_count());
        });
    });

    group.finish();
}

/// Benchmark memory allocation patterns
fn bench_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");

    group.bench_function("vec_1m_tokens", |b| {
        b.iter(|| {
            let tokens: Vec<Token> = (0..1_000_000)
                .map(|i| Token::new(black_box(i as u64)))
                .collect();
            black_box(tokens);
        });
    });

    group.bench_function("vec_with_capacity_1m", |b| {
        b.iter(|| {
            let mut tokens = Vec::with_capacity(1_000_000);
            for i in 0..1_000_000 {
                tokens.push(Token::new(black_box(i as u64)));
            }
            black_box(tokens);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_token_creation,
    bench_similarity_scale,
    bench_graph_insertion,
    bench_bootstrap_search_scale,
    bench_concurrent_reads,
    bench_memory_allocation,
);

criterion_main!(benches);
