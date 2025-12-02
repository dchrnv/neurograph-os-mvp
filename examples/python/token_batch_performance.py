#!/usr/bin/env python3
"""
NeuroGraph OS - Token Batch Performance Demo

Demonstrates the 4x performance improvement from batch operations.

Results (from stress test):
- Without batch: 708ms for 1M tokens
- With batch:    175ms for 1M tokens
- Speedup:       4.03x
"""

import neurograph
import time


def demo_slow_method():
    """SLOW: Creating tokens in Python loop"""
    print("=" * 60)
    print("SLOW METHOD: Python loop")
    print("=" * 60)

    count = 100_000  # Reduced from 1M for demo

    start = time.time()
    tokens = []
    for i in range(count):
        tokens.append(neurograph.Token(i))
    duration = time.time() - start

    print(f"✗ Created {len(tokens):,} tokens in {duration:.3f}s")
    print(f"  Rate: {count/duration:,.0f} tokens/sec")
    print(f"  Avg: {duration/count*1e9:.1f} ns/token")
    print()

    return duration


def demo_fast_method():
    """FAST: Batch creation in Rust"""
    print("=" * 60)
    print("FAST METHOD: Batch creation (Rust)")
    print("=" * 60)

    count = 100_000  # Same count for comparison

    start = time.time()
    tokens = neurograph.Token.create_batch(count)
    duration = time.time() - start

    print(f"✓ Created {len(tokens):,} tokens in {duration:.3f}s")
    print(f"  Rate: {count/duration:,.0f} tokens/sec")
    print(f"  Avg: {duration/count*1e9:.1f} ns/token")
    print()

    return duration


def main():
    print("""
╔══════════════════════════════════════════════════════════════╗
║    NeuroGraph OS - Batch API Performance Demo               ║
╚══════════════════════════════════════════════════════════════╝

This demo shows why you should ALWAYS use batch operations
when creating large numbers of tokens.
""")

    # Run slow method
    slow_time = demo_slow_method()

    # Run fast method
    fast_time = demo_fast_method()

    # Compare
    print("=" * 60)
    print("COMPARISON")
    print("=" * 60)
    speedup = slow_time / fast_time
    print(f"Slow method: {slow_time:.3f}s")
    print(f"Fast method: {fast_time:.3f}s")
    print(f"Speedup:     {speedup:.2f}x faster! 🚀")
    print()

    # Recommendation
    print("📝 RECOMMENDATION:")
    print()
    print("✓ GOOD - Use batch API:")
    print("  tokens = neurograph.Token.create_batch(1_000_000)")
    print()
    print("✗ BAD - Avoid Python loops:")
    print("  tokens = [neurograph.Token(i) for i in range(1_000_000)]")
    print()

    # Memory info
    print("💾 MEMORY:")
    print("  - 1M tokens = 61 MB (Rust) or ~107 MB (Python)")
    print("  - Python adds ~48 bytes/object overhead")
    print()


if __name__ == "__main__":
    main()
