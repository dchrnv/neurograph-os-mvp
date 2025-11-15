//! Graph Benchmarks for v0.27.0
//!
//! Performance measurements for Graph operations:
//! - graph_add_node: <50 ns target
//! - graph_add_connection: <100 ns target
//! - graph_bfs: <500 μs target (1k nodes)
//! - graph_dfs: <500 μs target (1k nodes)
//! - graph_shortest_path: <1 ms target

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use neurograph_core::{Graph, Direction};

/// Benchmark: Add node (target: <50 ns)
fn bench_graph_add_node(c: &mut Criterion) {
    let mut graph = Graph::new();

    c.bench_function("graph_add_node", |b| {
        let mut id = 0u32;
        b.iter(|| {
            id = id.wrapping_add(1);
            graph.add_node(black_box(id))
        })
    });
}

/// Benchmark: Add edge/connection (target: <100 ns)
fn bench_graph_add_connection(c: &mut Criterion) {
    let mut graph = Graph::new();

    // Pre-create 10,000 nodes
    for i in 0..10_000 {
        graph.add_node(i);
    }

    c.bench_function("graph_add_connection", |b| {
        let mut counter = 0u32;
        b.iter(|| {
            counter = counter.wrapping_add(1);
            let from = counter % 10_000;
            let to = (counter + 1) % 10_000;
            let edge_id = Graph::compute_edge_id(from, to, 0);
            graph.add_edge(black_box(edge_id), black_box(from), black_box(to), 0, 1.0, false).ok()
        })
    });
}

/// Benchmark: BFS traversal (target: <500 μs for 1k nodes)
fn bench_graph_bfs(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_bfs");

    for size in [100, 500, 1000].iter() {
        let mut graph = Graph::new();

        // Create a connected graph with 'size' nodes in a linear chain
        for i in 0..*size {
            graph.add_node(i as u32);
        }

        for i in 0..(*size - 1) {
            let edge_id = Graph::compute_edge_id(i as u32, (i + 1) as u32, 0);
            graph.add_edge(edge_id, i as u32, (i + 1) as u32, 0, 1.0, false).ok();
        }

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                graph.bfs(black_box(0), black_box(None))
            })
        });
    }

    group.finish();
}

/// Benchmark: DFS traversal (target: <500 μs for 1k nodes)
fn bench_graph_dfs(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_dfs");

    for size in [100, 500, 1000].iter() {
        let mut graph = Graph::new();

        // Create a connected graph with 'size' nodes in a linear chain
        for i in 0..*size {
            graph.add_node(i as u32);
        }

        for i in 0..(*size - 1) {
            let edge_id = Graph::compute_edge_id(i as u32, (i + 1) as u32, 0);
            graph.add_edge(edge_id, i as u32, (i + 1) as u32, 0, 1.0, false).ok();
        }

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                graph.dfs(black_box(0), black_box(None))
            })
        });
    }

    group.finish();
}

/// Benchmark: Shortest path (target: <1 ms)
fn bench_graph_shortest_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_shortest_path");

    for size in [100, 500, 1000].iter() {
        let mut graph = Graph::new();

        // Create a connected graph with 'size' nodes in a grid pattern
        for i in 0..*size {
            graph.add_node(i as u32);
        }

        // Add edges in a grid pattern (each node connects to next and +10)
        for i in 0..*size {
            if (i + 1) < *size {
                let edge_id = Graph::compute_edge_id(i as u32, (i + 1) as u32, 0);
                graph.add_edge(edge_id, i as u32, (i + 1) as u32, 0, 1.0, false).ok();
            }
            if (i + 10) < *size {
                let edge_id = Graph::compute_edge_id(i as u32, (i + 10) as u32, 0);
                graph.add_edge(edge_id, i as u32, (i + 10) as u32, 0, 1.0, false).ok();
            }
        }

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                graph.shortest_path(black_box(0), black_box((*size - 1) as u32))
            })
        });
    }

    group.finish();
}

/// Benchmark: Get neighbors (adjacency lookup)
fn bench_graph_get_neighbors(c: &mut Criterion) {
    let mut graph = Graph::new();

    // Create a graph where each node has 10 neighbors
    for i in 0..1000 {
        graph.add_node(i);
    }

    for i in 0..1000 {
        for j in 1..=10 {
            let to = (i + j) % 1000;
            let edge_id = Graph::compute_edge_id(i, to, 0);
            graph.add_edge(edge_id, i, to, 0, 1.0, false).ok();
        }
    }

    let mut group = c.benchmark_group("graph_get_neighbors");

    group.bench_function("outgoing", |b| {
        b.iter(|| {
            graph.get_neighbors(black_box(500), black_box(Direction::Outgoing))
        })
    });

    group.bench_function("incoming", |b| {
        b.iter(|| {
            graph.get_neighbors(black_box(500), black_box(Direction::Incoming))
        })
    });

    group.bench_function("both", |b| {
        b.iter(|| {
            graph.get_neighbors(black_box(500), black_box(Direction::Both))
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_graph_add_node,
    bench_graph_add_connection,
    bench_graph_bfs,
    bench_graph_dfs,
    bench_graph_shortest_path,
    bench_graph_get_neighbors
);
criterion_main!(benches);