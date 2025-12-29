/**
 * Performance benchmark for NeuroGraph TypeScript client.
 *
 * Compares performance of different operation patterns.
 */

import { NeuroGraphClient } from '../../typescript-client/src';

interface BenchmarkResult {
  mean: number;
  median: number;
  stdev: number;
}

/**
 * Benchmark sequential operations.
 */
async function benchmarkSequential(
  client: NeuroGraphClient,
  count: number
): Promise<number> {
  const start = Date.now();

  const tokens = [];
  for (let i = 0; i < count; i++) {
    const token = await client.tokens.create({ text: `benchmark token ${i}` });
    tokens.push(token);
  }

  // Cleanup
  for (const token of tokens) {
    await client.tokens.delete(token.id);
  }

  return (Date.now() - start) / 1000;
}

/**
 * Benchmark concurrent operations with Promise.all.
 */
async function benchmarkConcurrent(
  client: NeuroGraphClient,
  count: number
): Promise<number> {
  const start = Date.now();

  // Create tokens concurrently
  const createPromises = Array.from({ length: count }, (_, i) =>
    client.tokens.create({ text: `benchmark token ${i}` })
  );
  const tokens = await Promise.all(createPromises);

  // Delete concurrently
  const deletePromises = tokens.map((token) => client.tokens.delete(token.id));
  await Promise.all(deletePromises);

  return (Date.now() - start) / 1000;
}

/**
 * Benchmark batched operations.
 */
async function benchmarkBatched(
  client: NeuroGraphClient,
  count: number,
  batchSize: number
): Promise<number> {
  const start = Date.now();

  const tokens = [];

  // Create in batches
  for (let i = 0; i < count; i += batchSize) {
    const batch = Math.min(batchSize, count - i);
    const promises = Array.from({ length: batch }, (_, j) =>
      client.tokens.create({ text: `benchmark token ${i + j}` })
    );
    const batchTokens = await Promise.all(promises);
    tokens.push(...batchTokens);
  }

  // Delete in batches
  for (let i = 0; i < tokens.length; i += batchSize) {
    const batch = tokens.slice(i, i + batchSize);
    const promises = batch.map((token) => client.tokens.delete(token.id));
    await Promise.all(promises);
  }

  return (Date.now() - start) / 1000;
}

/**
 * Calculate statistics.
 */
function calculateStats(times: number[]): BenchmarkResult {
  const mean = times.reduce((a, b) => a + b, 0) / times.length;
  const sorted = [...times].sort((a, b) => a - b);
  const median = sorted[Math.floor(sorted.length / 2)];

  const variance =
    times.reduce((sum, time) => sum + Math.pow(time - mean, 2), 0) / times.length;
  const stdev = Math.sqrt(variance);

  return { mean, median, stdev };
}

/**
 * Run benchmark suite.
 */
async function runBenchmarkSuite() {
  console.log('='.repeat(60));
  console.log('NeuroGraph TypeScript Client Performance Benchmark');
  console.log('='.repeat(60));
  console.log();

  // Configuration
  const baseUrl = process.env.NEUROGRAPH_API_URL || 'http://localhost:8000';
  const username = process.env.NEUROGRAPH_USERNAME || 'developer';
  const password = process.env.NEUROGRAPH_PASSWORD || 'developer123';
  const iterations = 3;
  const tokenCounts = [10, 50, 100];
  const batchSize = 10;

  console.log('Configuration:');
  console.log(`  Base URL: ${baseUrl}`);
  console.log(`  Iterations: ${iterations}`);
  console.log(`  Token counts: ${tokenCounts.join(', ')}`);
  console.log(`  Batch size: ${batchSize}`);
  console.log();

  const client = new NeuroGraphClient({
    baseUrl,
    username,
    password,
  });

  const results: Record<string, BenchmarkResult> = {};

  // Sequential benchmark
  console.log('Running sequential benchmarks...');
  console.log('-'.repeat(60));

  for (const count of tokenCounts) {
    const times: number[] = [];
    for (let i = 0; i < iterations; i++) {
      const elapsed = await benchmarkSequential(client, count);
      times.push(elapsed);
      console.log(`  Sequential (${count} tokens), iter ${i + 1}: ${elapsed.toFixed(3)}s`);
    }
    results[`sequential_${count}`] = calculateStats(times);
  }

  console.log();

  // Concurrent benchmark
  console.log('Running concurrent benchmarks...');
  console.log('-'.repeat(60));

  for (const count of tokenCounts) {
    const times: number[] = [];
    for (let i = 0; i < iterations; i++) {
      const elapsed = await benchmarkConcurrent(client, count);
      times.push(elapsed);
      console.log(`  Concurrent (${count} tokens), iter ${i + 1}: ${elapsed.toFixed(3)}s`);
    }
    results[`concurrent_${count}`] = calculateStats(times);
  }

  console.log();

  // Batched benchmark
  console.log('Running batched benchmarks...');
  console.log('-'.repeat(60));

  for (const count of tokenCounts) {
    const times: number[] = [];
    for (let i = 0; i < iterations; i++) {
      const elapsed = await benchmarkBatched(client, count, batchSize);
      times.push(elapsed);
      console.log(
        `  Batched (${count} tokens, batch=${batchSize}), iter ${i + 1}: ${elapsed.toFixed(3)}s`
      );
    }
    results[`batched_${count}`] = calculateStats(times);
  }

  // Summary
  console.log();
  console.log('='.repeat(60));
  console.log('Summary (mean ± stdev)');
  console.log('='.repeat(60));
  console.log();

  for (const count of tokenCounts) {
    console.log(`Token count: ${count}`);

    const seq = results[`sequential_${count}`];
    const conc = results[`concurrent_${count}`];
    const batch = results[`batched_${count}`];

    console.log(`  Sequential:  ${seq.mean.toFixed(3)}s ± ${seq.stdev.toFixed(3)}s`);
    console.log(`  Concurrent:  ${conc.mean.toFixed(3)}s ± ${conc.stdev.toFixed(3)}s`);
    console.log(`  Batched:     ${batch.mean.toFixed(3)}s ± ${batch.stdev.toFixed(3)}s`);

    // Calculate speedups
    const speedupConc = seq.mean / conc.mean;
    const speedupBatch = seq.mean / batch.mean;

    console.log(`  Speedup (concurrent): ${speedupConc.toFixed(2)}x`);
    console.log(`  Speedup (batched):    ${speedupBatch.toFixed(2)}x`);
    console.log();
  }

  console.log('Benchmark complete!');
}

// Run benchmark
runBenchmarkSuite().catch(console.error);
