#!/usr/bin/env python3
"""
Quick performance test for /status endpoint optimization

Tests that /status endpoint responds in <10ms after v0.52.0 optimization
(previously 108ms due to psutil blocking calls)
"""

import time
import requests
import statistics

BASE_URL = "http://localhost:8000/api/v1"
NUM_REQUESTS = 50


def test_status_performance():
    """Test /status endpoint performance."""
    print("Testing /status endpoint performance...")
    print(f"Making {NUM_REQUESTS} requests...")

    latencies = []

    for i in range(NUM_REQUESTS):
        start = time.perf_counter()
        response = requests.get(f"{BASE_URL}/status", timeout=5)
        latency = (time.perf_counter() - start) * 1000  # Convert to ms

        assert response.status_code == 200, f"Expected 200, got {response.status_code}"
        latencies.append(latency)

        if (i + 1) % 10 == 0:
            print(f"  Progress: {i+1}/{NUM_REQUESTS}")

    # Calculate statistics
    mean_latency = statistics.mean(latencies)
    median_latency = statistics.median(latencies)
    min_latency = min(latencies)
    max_latency = max(latencies)
    p95_latency = sorted(latencies)[int(len(latencies) * 0.95)]
    p99_latency = sorted(latencies)[int(len(latencies) * 0.99)]

    print("\n" + "=" * 60)
    print("RESULTS")
    print("=" * 60)
    print(f"Requests:      {NUM_REQUESTS}")
    print(f"Mean latency:  {mean_latency:.2f}ms")
    print(f"Median:        {median_latency:.2f}ms")
    print(f"Min:           {min_latency:.2f}ms")
    print(f"Max:           {max_latency:.2f}ms")
    print(f"P95:           {p95_latency:.2f}ms")
    print(f"P99:           {p99_latency:.2f}ms")
    print("=" * 60)

    # Check optimization target
    target_latency = 10.0  # ms
    if p95_latency < target_latency:
        print(f"✅ SUCCESS: P95 latency ({p95_latency:.2f}ms) < {target_latency}ms")
        print(f"   Improvement: {108 / p95_latency:.1f}x faster than v0.51.0 (108ms)")
    else:
        print(f"❌ FAILED: P95 latency ({p95_latency:.2f}ms) >= {target_latency}ms")
        print(f"   Target: <{target_latency}ms")

    return p95_latency < target_latency


if __name__ == "__main__":
    try:
        success = test_status_performance()
        exit(0 if success else 1)
    except Exception as e:
        print(f"\n❌ ERROR: {e}")
        print("\nMake sure the API server is running:")
        print("  python -m src.api.main")
        exit(1)
