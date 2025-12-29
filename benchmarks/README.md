# NeuroGraph Client Performance Benchmarks

Performance benchmarks for NeuroGraph Python and TypeScript clients.

## Python Benchmarks

### Sync vs Async Comparison

Compares synchronous and asynchronous client performance:

```bash
cd benchmarks/python
python benchmark_sync_vs_async.py
```

**What it measures:**
- Sync client sequential operations
- Async client sequential operations
- Async client concurrent operations

**Expected results:**
- Async concurrent: ~10-20x faster than sync sequential
- Async sequential: Similar to sync sequential

## TypeScript Benchmarks

### Sequential vs Concurrent

Compares different operation patterns:

```bash
cd benchmarks/typescript
npx tsx benchmark.ts
```

**What it measures:**
- Sequential operations (one at a time)
- Concurrent operations (Promise.all)
- Batched operations (controlled concurrency)

**Expected results:**
- Concurrent: ~5-15x faster than sequential
- Batched: Good balance of speed and resource usage

## Running Benchmarks

### Prerequisites

1. NeuroGraph API running at `http://localhost:8000`
2. Test user credentials (developer/developer123)

### Python Setup

```bash
pip install neurograph-python
```

### TypeScript Setup

```bash
cd typescript-client
npm install
npm run build
```

## Interpreting Results

### Key Metrics

- **Mean**: Average time across iterations
- **Median**: Middle value (less affected by outliers)
- **Stdev**: Standard deviation (consistency indicator)
- **Speedup**: Performance improvement ratio

### Recommendations

**For I/O-bound workloads:**
- Use async client with concurrent operations (Python)
- Use Promise.all for concurrent operations (TypeScript)

**For CPU-bound workloads:**
- Sequential operations may perform similarly
- Consider batching to balance concurrency

**For production:**
- Monitor API rate limits
- Use batching to control load
- Implement retry logic for resilience

## Sample Output

```
NeuroGraph Python Client Performance Benchmark
================================================================

Configuration:
  Base URL: http://localhost:8000
  Iterations: 3
  Token counts: [10, 50, 100]

Running synchronous client benchmarks...
----------------------------------------------------------------
  Sync sequential (10 tokens), iter 1: 1.234s
  Sync sequential (10 tokens), iter 2: 1.198s
  Sync sequential (10 tokens), iter 3: 1.215s

Running asynchronous client benchmarks...
----------------------------------------------------------------
  Async sequential (10 tokens), iter 1: 1.187s
  Async sequential (10 tokens), iter 2: 1.203s
  Async sequential (10 tokens), iter 3: 1.195s
  Async concurrent (10 tokens), iter 1: 0.142s
  Async concurrent (10 tokens), iter 2: 0.138s
  Async concurrent (10 tokens), iter 3: 0.145s

================================================================
Summary (mean ± stdev)
================================================================

Token count: 10
  Sync sequential:   1.216s ± 0.015s
  Async sequential:  1.195s ± 0.007s
  Async concurrent:  0.142s ± 0.003s
  Speedup (async seq):   1.02x
  Speedup (async conc):  8.56x

Benchmark complete!
```

## Best Practices

1. **Run multiple iterations** to account for variance
2. **Warm up** the API before benchmarking
3. **Use consistent network conditions**
4. **Monitor system resources** during benchmarks
5. **Test with realistic data sizes**

## Contributing

To add new benchmarks:

1. Create new benchmark script
2. Follow existing naming conventions
3. Document what is being measured
4. Include expected results
5. Update this README
