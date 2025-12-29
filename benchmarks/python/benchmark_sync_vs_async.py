"""
Performance benchmark: Sync vs Async clients.

Compares performance of synchronous and asynchronous NeuroGraph clients
for various operations.
"""

import time
import asyncio
from typing import List
import statistics

from neurograph import NeuroGraphClient, AsyncNeuroGraphClient


def benchmark_sync_sequential(client: NeuroGraphClient, count: int) -> float:
    """Benchmark sync client with sequential operations."""
    start = time.time()

    tokens = []
    for i in range(count):
        token = client.tokens.create(text=f"benchmark token {i}")
        tokens.append(token)

    # Cleanup
    for token in tokens:
        client.tokens.delete(token.id)

    return time.time() - start


async def benchmark_async_concurrent(client: AsyncNeuroGraphClient, count: int) -> float:
    """Benchmark async client with concurrent operations."""
    start = time.time()

    # Create tokens concurrently
    tasks = [client.tokens.create(text=f"benchmark token {i}") for i in range(count)]
    tokens = await asyncio.gather(*tasks)

    # Delete concurrently
    delete_tasks = [client.tokens.delete(token.id) for token in tokens]
    await asyncio.gather(*delete_tasks)

    return time.time() - start


async def benchmark_async_sequential(client: AsyncNeuroGraphClient, count: int) -> float:
    """Benchmark async client with sequential operations."""
    start = time.time()

    tokens = []
    for i in range(count):
        token = await client.tokens.create(text=f"benchmark token {i}")
        tokens.append(token)

    # Cleanup
    for token in tokens:
        await client.tokens.delete(token.id)

    return time.time() - start


def run_benchmark_suite():
    """Run complete benchmark suite."""
    print("=" * 60)
    print("NeuroGraph Python Client Performance Benchmark")
    print("=" * 60)
    print()

    # Configuration
    base_url = "http://localhost:8000"
    username = "developer"
    password = "developer123"
    iterations = 3
    token_counts = [10, 50, 100]

    print(f"Configuration:")
    print(f"  Base URL: {base_url}")
    print(f"  Iterations: {iterations}")
    print(f"  Token counts: {token_counts}")
    print()

    results = {}

    # Sync client benchmarks
    print("Running synchronous client benchmarks...")
    print("-" * 60)

    with NeuroGraphClient(
        base_url=base_url, username=username, password=password
    ) as client:
        for count in token_counts:
            times = []
            for i in range(iterations):
                elapsed = benchmark_sync_sequential(client, count)
                times.append(elapsed)
                print(f"  Sync sequential ({count} tokens), iter {i+1}: {elapsed:.3f}s")

            results[f"sync_seq_{count}"] = {
                "mean": statistics.mean(times),
                "median": statistics.median(times),
                "stdev": statistics.stdev(times) if len(times) > 1 else 0,
            }

    print()

    # Async client benchmarks
    print("Running asynchronous client benchmarks...")
    print("-" * 60)

    async def run_async_benchmarks():
        async with AsyncNeuroGraphClient(
            base_url=base_url, username=username, password=password
        ) as client:
            # Sequential
            for count in token_counts:
                times = []
                for i in range(iterations):
                    elapsed = await benchmark_async_sequential(client, count)
                    times.append(elapsed)
                    print(f"  Async sequential ({count} tokens), iter {i+1}: {elapsed:.3f}s")

                results[f"async_seq_{count}"] = {
                    "mean": statistics.mean(times),
                    "median": statistics.median(times),
                    "stdev": statistics.stdev(times) if len(times) > 1 else 0,
                }

            # Concurrent
            for count in token_counts:
                times = []
                for i in range(iterations):
                    elapsed = await benchmark_async_concurrent(client, count)
                    times.append(elapsed)
                    print(f"  Async concurrent ({count} tokens), iter {i+1}: {elapsed:.3f}s")

                results[f"async_conc_{count}"] = {
                    "mean": statistics.mean(times),
                    "median": statistics.median(times),
                    "stdev": statistics.stdev(times) if len(times) > 1 else 0,
                }

    asyncio.run(run_async_benchmarks())

    # Summary
    print()
    print("=" * 60)
    print("Summary (mean ± stdev)")
    print("=" * 60)
    print()

    for count in token_counts:
        print(f"Token count: {count}")
        print(f"  Sync sequential:   {results[f'sync_seq_{count}']['mean']:.3f}s ± {results[f'sync_seq_{count}']['stdev']:.3f}s")
        print(f"  Async sequential:  {results[f'async_seq_{count}']['mean']:.3f}s ± {results[f'async_seq_{count}']['stdev']:.3f}s")
        print(f"  Async concurrent:  {results[f'async_conc_{count}']['mean']:.3f}s ± {results[f'async_conc_{count}']['stdev']:.3f}s")

        # Calculate speedups
        sync_mean = results[f'sync_seq_{count}']['mean']
        async_seq_mean = results[f'async_seq_{count}']['mean']
        async_conc_mean = results[f'async_conc_{count}']['mean']

        speedup_async_seq = sync_mean / async_seq_mean
        speedup_async_conc = sync_mean / async_conc_mean

        print(f"  Speedup (async seq):   {speedup_async_seq:.2f}x")
        print(f"  Speedup (async conc):  {speedup_async_conc:.2f}x")
        print()

    print("Benchmark complete!")


if __name__ == "__main__":
    run_benchmark_suite()
