#!/usr/bin/env python3
"""
Test SignalSystem Rust Core - v0.57.0

Tests the Rust SignalSystem through Python bindings.
"""

import sys
sys.path.insert(0, 'src/core_rust/target/release')

import _core


def test_signal_system_creation():
    """Test SignalSystem creation."""
    print("=" * 60)
    print("Test 1: SignalSystem Creation")
    print("=" * 60)
    print()

    system = _core.SignalSystem()
    print(f"✓ SignalSystem created")
    print(f"  Available methods: {[m for m in dir(system) if not m.startswith('_')]}")
    print()


def test_emit_event():
    """Test emitting events."""
    print("=" * 60)
    print("Test 2: Emit Event")
    print("=" * 60)
    print()

    system = _core.SignalSystem()

    # Emit a signal event
    result = system.emit(
        event_type="signal.input.text",
        vector=[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
        priority=200
    )

    print(f"✓ Event emitted")
    print(f"  Result keys: {list(result.keys())}")
    print(f"  Token ID: {result.get('token_id')}")
    print(f"  Is novel: {result.get('is_novel')}")
    print(f"  Triggered actions: {result.get('triggered_actions', [])}")
    print(f"  Processing time: {result.get('processing_time_us')}μs")
    print()

    return system, result


def test_multiple_events():
    """Test processing multiple events."""
    print("=" * 60)
    print("Test 3: Multiple Events")
    print("=" * 60)
    print()

    system = _core.SignalSystem()

    # Emit 5 events
    results = []
    for i in range(5):
        result = system.emit(
            event_type=f"signal.input.test_{i}",
            vector=[i * 0.1] * 8,
            priority=200
        )
        results.append(result)
        print(f"  Event {i+1}: token_id={result['token_id']}, novel={result['is_novel']}")

    print()
    print(f"✓ Processed {len(results)} events")
    print()

    return system, results


def test_statistics():
    """Test getting statistics."""
    print("=" * 60)
    print("Test 4: Statistics")
    print("=" * 60)
    print()

    system = _core.SignalSystem()

    # Process some events
    for i in range(10):
        system.emit(
            event_type="signal.test",
            vector=[0.5] * 8,
            priority=200
        )

    # Get stats
    stats = system.get_stats()
    print(f"✓ Statistics retrieved")
    print(f"  Total events: {stats.get('total_events')}")
    print(f"  Avg processing time: {stats.get('avg_processing_time_us')}μs")
    print(f"  Novel tokens: {stats.get('novel_tokens')}")
    print()

    return stats


def test_subscriptions():
    """Test subscription system."""
    print("=" * 60)
    print("Test 5: Subscriptions")
    print("=" * 60)
    print()

    system = _core.SignalSystem()

    # Create a subscriber with filter
    subscriber_id = system.subscribe(
        name="test_subscriber",
        filter_dict={
            "event_type": {"$wildcard": "signal.input.*"},
            "priority": {"$gte": 150}
        },
        callback=lambda event: print(f"  → Received: {event.get('event_type')}")
    )

    print(f"✓ Subscriber created: {subscriber_id}")
    print(f"  Total subscribers: {system.subscriber_count()}")
    print()

    # Emit matching event
    print("  Emitting matching event...")
    system.emit(
        event_type="signal.input.text",
        vector=[0.5] * 8,
        priority=200
    )

    # Emit non-matching event
    print("  Emitting non-matching event (low priority)...")
    system.emit(
        event_type="signal.input.text",
        vector=[0.5] * 8,
        priority=100  # Below threshold
    )

    print()

    # Unsubscribe
    success = system.unsubscribe(subscriber_id)
    print(f"✓ Unsubscribed: {success}")
    print(f"  Total subscribers: {system.subscriber_count()}")
    print()


def test_performance():
    """Test performance."""
    print("=" * 60)
    print("Test 6: Performance")
    print("=" * 60)
    print()

    system = _core.SignalSystem()

    import time
    start = time.time()

    # Emit 1000 events
    for i in range(1000):
        system.emit(
            event_type="signal.perf.test",
            vector=[0.5] * 8,
            priority=200
        )

    elapsed = (time.time() - start) * 1000  # ms

    stats = system.get_stats()

    print(f"✓ Processed 1000 events in {elapsed:.2f}ms")
    print(f"  Throughput: {1000 / (elapsed/1000):.0f} events/sec")
    print(f"  Avg processing time: {stats.get('avg_processing_time_us')}μs")
    print(f"  Total events: {stats.get('total_events')}")
    print()


def main():
    print("\n" + "=" * 60)
    print("SignalSystem Rust Core Test Suite - v0.57.0")
    print("=" * 60)
    print()

    try:
        test_signal_system_creation()
        test_emit_event()
        test_multiple_events()
        test_statistics()
        test_subscriptions()
        test_performance()

        print("=" * 60)
        print("✓ All tests passed!")
        print("=" * 60)
        print()

    except Exception as e:
        print(f"\n❌ Test failed: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
