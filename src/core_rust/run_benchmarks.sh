#!/bin/bash
# Benchmark runner for v0.27.0
# Runs all benchmarks sequentially

set -e

echo "=== NeuroGraph OS v0.27.0 Benchmark Suite ==="
echo ""

BENCHMARKS=(
    "token_bench"
    "grid_bench"
    "graph_bench"
    "experience_stream_bench"
    "intuition_bench"
)

for bench in "${BENCHMARKS[@]}"; do
    echo "Running benchmark: $bench"
    echo "---"
    cargo bench --bench "$bench" -- --nocapture || {
        echo "❌ Failed to run $bench"
        exit 1
    }
    echo ""
done

echo "✅ All benchmarks completed!"
echo "Results saved in target/criterion/"
echo ""
echo "To view HTML reports:"
echo "  open target/criterion/report/index.html"