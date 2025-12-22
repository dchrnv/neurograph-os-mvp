#!/usr/bin/env python3
"""
Tests for SubscriptionFilter - Phase 1
"""

from src.gateway.filters import SubscriptionFilter
from src.gateway.filters.examples import (
    telegram_user_messages_filter,
    telegram_high_priority_filter,
    dashboard_all_events_filter,
    action_selector_novel_signals_filter,
    anomaly_detector_high_score_filter,
    sentiment_positive_filter,
    tag_contains_filter,
)
from src.gateway import SignalGateway


def test_simple_equality():
    """Test simple field equality"""
    print("=" * 60)
    print("Test 1: Simple Equality")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    # Create event
    event = gateway.push_text("Test message", priority=200)

    # Test exact match
    filter1 = SubscriptionFilter({
        "source.domain": "external"
    })
    assert filter1.matches(event) == True
    print(f"✓ Exact match: source.domain = 'external'")

    # Test non-match
    filter2 = SubscriptionFilter({
        "source.domain": "internal"
    })
    assert filter2.matches(event) == False
    print(f"✓ Non-match: source.domain != 'internal'")
    print()


def test_comparison_operators():
    """Test numeric comparison operators"""
    print("=" * 60)
    print("Test 2: Comparison Operators")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    event = gateway.push_text("Test", priority=150)

    # $eq
    filter_eq = SubscriptionFilter({"routing.priority": {"$eq": 150}})
    assert filter_eq.matches(event) == True
    print(f"✓ $eq: priority = 150")

    # $ne
    filter_ne = SubscriptionFilter({"routing.priority": {"$ne": 100}})
    assert filter_ne.matches(event) == True
    print(f"✓ $ne: priority != 100")

    # $gt
    filter_gt = SubscriptionFilter({"routing.priority": {"$gt": 100}})
    assert filter_gt.matches(event) == True
    print(f"✓ $gt: priority > 100")

    # $gte
    filter_gte = SubscriptionFilter({"routing.priority": {"$gte": 150}})
    assert filter_gte.matches(event) == True
    print(f"✓ $gte: priority >= 150")

    # $lt
    filter_lt = SubscriptionFilter({"routing.priority": {"$lt": 200}})
    assert filter_lt.matches(event) == True
    print(f"✓ $lt: priority < 200")

    # $lte
    filter_lte = SubscriptionFilter({"routing.priority": {"$lte": 150}})
    assert filter_lte.matches(event) == True
    print(f"✓ $lte: priority <= 150")
    print()


def test_wildcard_operator():
    """Test wildcard pattern matching"""
    print("=" * 60)
    print("Test 3: Wildcard Operator")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    event = gateway.push_text("Test")

    # Wildcard match
    filter1 = SubscriptionFilter({
        "event_type": {"$wildcard": "signal.input.*"}
    })
    assert filter1.matches(event) == True
    print(f"✓ Wildcard: event_type matches 'signal.input.*'")
    print(f"  Actual: {event.event_type}")

    # More specific wildcard
    filter2 = SubscriptionFilter({
        "event_type": {"$wildcard": "signal.input.external.text.*"}
    })
    assert filter2.matches(event) == True
    print(f"✓ Wildcard: matches 'signal.input.external.text.*'")

    # Non-matching wildcard
    filter3 = SubscriptionFilter({
        "event_type": {"$wildcard": "signal.output.*"}
    })
    assert filter3.matches(event) == False
    print(f"✓ Wildcard: doesn't match 'signal.output.*'")
    print()


def test_collection_operators():
    """Test $in, $nin, $contains operators"""
    print("=" * 60)
    print("Test 4: Collection Operators")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    event = gateway.push_text("Test", priority=150)

    # $in
    filter_in = SubscriptionFilter({
        "routing.priority": {"$in": [100, 150, 200]}
    })
    assert filter_in.matches(event) == True
    print(f"✓ $in: priority in [100, 150, 200]")

    # $nin
    filter_nin = SubscriptionFilter({
        "routing.priority": {"$nin": [50, 75, 100]}
    })
    assert filter_nin.matches(event) == True
    print(f"✓ $nin: priority not in [50, 75, 100]")

    # $contains (for tags)
    filter_contains = SubscriptionFilter({
        "routing.tags": {"$contains": "text_chat"}
    })
    assert filter_contains.matches(event) == True
    print(f"✓ $contains: tags contains 'text_chat'")
    print(f"  Actual tags: {event.routing.tags}")
    print()


def test_logical_operators():
    """Test $and, $or, $not operators"""
    print("=" * 60)
    print("Test 5: Logical Operators")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    event = gateway.push_text("Test", priority=180)

    # $and
    filter_and = SubscriptionFilter({
        "$and": [
            {"source.domain": "external"},
            {"routing.priority": {"$gte": 150}}
        ]
    })
    assert filter_and.matches(event) == True
    print(f"✓ $and: domain=external AND priority>=150")

    # $or
    filter_or = SubscriptionFilter({
        "$or": [
            {"source.domain": "internal"},
            {"routing.priority": {"$gte": 150}}
        ]
    })
    assert filter_or.matches(event) == True
    print(f"✓ $or: domain=internal OR priority>=150")

    # $not
    filter_not = SubscriptionFilter({
        "$not": {"source.domain": "system"}
    })
    assert filter_not.matches(event) == True
    print(f"✓ $not: NOT domain=system")

    # Complex: ($and with $or)
    filter_complex = SubscriptionFilter({
        "$and": [
            {
                "$or": [
                    {"source.domain": "external"},
                    {"source.domain": "internal"}
                ]
            },
            {"routing.priority": {"$gte": 150}}
        ]
    })
    assert filter_complex.matches(event) == True
    print(f"✓ Complex: (external OR internal) AND priority>=150")
    print()


def test_regex_operator():
    """Test regex pattern matching"""
    print("=" * 60)
    print("Test 6: Regex Operator")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    event = gateway.push_text("Test")

    # Regex match
    filter1 = SubscriptionFilter({
        "event_type": {"$regex": r"^signal\.input\..*\.text\..*$"}
    })
    assert filter1.matches(event) == True
    print(f"✓ Regex: matches pattern")
    print(f"  Pattern: ^signal\\.input\\..*\\.text\\..*$")
    print(f"  Actual: {event.event_type}")

    # Partial regex match
    filter2 = SubscriptionFilter({
        "event_type": {"$regex": r"external"}
    })
    assert filter2.matches(event) == True
    print(f"✓ Regex: partial match 'external'")
    print()


def test_filter_examples():
    """Test pre-built filter examples"""
    print("=" * 60)
    print("Test 7: Filter Examples")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    # Create test events
    text_event = gateway.push_text("Hello!", priority=200)
    system_event = gateway.push_system("cpu_percent", 45.7, priority=100)

    # Test Telegram filter
    telegram_filter = telegram_user_messages_filter()
    assert telegram_filter.matches(text_event) == True
    assert telegram_filter.matches(system_event) == False
    print(f"✓ telegram_user_messages_filter:")
    print(f"  Text event: MATCH")
    print(f"  System event: NO MATCH")

    # Test dashboard filter
    dashboard_filter = dashboard_all_events_filter()
    assert dashboard_filter.matches(text_event) == True
    assert dashboard_filter.matches(system_event) == True
    print(f"✓ dashboard_all_events_filter:")
    print(f"  Both events: MATCH")

    # Test tag filter
    tag_filter = tag_contains_filter("text_chat")
    assert tag_filter.matches(text_event) == True
    assert tag_filter.matches(system_event) == False
    print(f"✓ tag_contains_filter('text_chat'):")
    print(f"  Text event: MATCH")
    print(f"  System event: NO MATCH")
    print()


def test_nested_field_access():
    """Test accessing deeply nested fields"""
    print("=" * 60)
    print("Test 8: Nested Field Access")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    event = gateway.push_text("Test", priority=175)

    # Access nested fields
    tests = [
        ("event_type", event.event_type),
        ("source.domain", event.source.domain),
        ("source.sensor_id", event.source.sensor_id),
        ("semantic.encoding_method", event.semantic.encoding_method),
        ("energy.urgency", event.energy.urgency),
        ("temporal.neuro_tick", event.temporal.neuro_tick),
        ("routing.priority", event.routing.priority),
    ]

    for field_path, expected_value in tests:
        filter_obj = SubscriptionFilter({field_path: expected_value})
        assert filter_obj.matches(event) == True
        print(f"✓ Access {field_path}: {expected_value}")

    print()


def test_filter_performance():
    """Test filter matching performance"""
    print("=" * 60)
    print("Test 9: Filter Performance")
    print("=" * 60)
    print()

    import time

    gateway = SignalGateway()
    gateway.initialize()

    # Create test event
    event = gateway.push_text("Performance test", priority=200)

    # Create complex filter
    complex_filter = SubscriptionFilter({
        "$and": [
            {"event_type": {"$wildcard": "signal.input.*"}},
            {"routing.priority": {"$gte": 150}},
            {
                "$or": [
                    {"source.domain": "external"},
                    {"source.domain": "internal"}
                ]
            },
            {"routing.tags": {"$contains": "text_chat"}}
        ]
    })

    # Benchmark
    iterations = 10000
    start = time.time()
    for _ in range(iterations):
        complex_filter.matches(event)
    elapsed = time.time() - start

    avg_time_us = (elapsed / iterations) * 1_000_000
    print(f"Complex filter matching:")
    print(f"  Iterations: {iterations}")
    print(f"  Total time: {elapsed*1000:.2f}ms")
    print(f"  Avg per match: {avg_time_us:.2f}μs")
    print(f"  Throughput: {iterations/elapsed:.0f} matches/sec")
    print()

    # Target: <100μs per match for complex filters
    assert avg_time_us < 100, f"Filter too slow: {avg_time_us:.2f}μs > 100μs"
    print(f"✓ Performance target met (<100μs per match)")
    print()


def main():
    print("\n" + "=" * 60)
    print("SubscriptionFilter Test Suite")
    print("=" * 60)
    print()

    test_simple_equality()
    test_comparison_operators()
    test_wildcard_operator()
    test_collection_operators()
    test_logical_operators()
    test_regex_operator()
    test_filter_examples()
    test_nested_field_access()
    test_filter_performance()

    print("=" * 60)
    print("✓ All SubscriptionFilter tests passed!")
    print("=" * 60)
    print()


if __name__ == "__main__":
    main()
