#!/usr/bin/env python3
"""
SignalSystem v1.1 - Basic Usage Example

Demonstrates:
- Creating a SignalSystem
- Emitting events
- Subscribing with filters
- Processing callbacks
- Statistics monitoring

NOTE: Requires neurograph._core to be built with python-bindings feature:
    cd src/core_rust
    maturin develop --features python-bindings
"""

import _core

def example_basic_emit():
    """Example 1: Basic event emission"""
    print("=" * 60)
    print("Example 1: Basic Event Emission")
    print("=" * 60)

    system = _core.SignalSystem()

    # Emit a simple event
    result = system.emit(
        event_type="signal.input.text",
        vector=[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
        priority=200
    )

    print(f"Event processed:")
    print(f"  Token ID: {result['token_id']}")
    print(f"  Novel: {result['is_novel']}")
    print(f"  Processing time: {result['processing_time_us']}μs")
    print(f"  Neighbors found: {len(result['neighbors'])}")
    print()


def example_subscription_with_filter():
    """Example 2: Subscribe with wildcard filter"""
    print("=" * 60)
    print("Example 2: Subscription with Filter")
    print("=" * 60)

    # TODO: Uncomment when ready
    # system = SignalSystem()
    #
    # events_received = []
    #
    # def handler(event):
    #     events_received.append(event)
    #     print(f"  Received: {event['event']['event_type_id']}")
    #
    # # Subscribe to all input.external.* events
    # sub_id = system.subscribe(
    #     name="external_monitor",
    #     filter_dict={
    #         "event_type": {"$wildcard": "signal.input.external.*"},
    #         "priority": {"$gte": 150}
    #     },
    #     callback=handler
    # )
    #
    # print(f"Subscribed with ID: {sub_id}")
    #
    # # Emit matching event
    # system.emit(
    #     event_type="signal.input.external.text.chat",
    #     vector=[0.5] * 8,
    #     priority=200
    # )
    #
    # # Emit non-matching event (wrong type)
    # system.emit(
    #     event_type="signal.input.system.timer",
    #     vector=[0.3] * 8,
    #     priority=200
    # )
    #
    # # Emit non-matching event (low priority)
    # system.emit(
    #     event_type="signal.input.external.audio",
    #     vector=[0.4] * 8,
    #     priority=100
    # )
    #
    # print(f"\nTotal events received: {len(events_received)}")
    # print(f"Expected: 1 (only matching filter)")
    #
    # system.unsubscribe(sub_id)

    print("TODO: Build _core with python-bindings feature")
    print()


def example_statistics():
    """Example 3: Monitoring statistics"""
    print("=" * 60)
    print("Example 3: Statistics Monitoring")
    print("=" * 60)

    # TODO: Uncomment when ready
    # system = SignalSystem()
    #
    # # Emit several events
    # for i in range(10):
    #     system.emit(
    #         event_type=f"signal.test.event_{i % 3}",
    #         vector=[0.1 * i] * 8,
    #         priority=100 + i * 10
    #     )
    #
    # # Get statistics
    # stats = system.get_stats()
    #
    # print(f"System Statistics:")
    # print(f"  Total events: {stats['total_events']}")
    # print(f"  Avg processing time: {stats['avg_processing_time_us']:.2f}μs")
    # print(f"  Subscriber notifications: {stats['subscriber_notifications']}")
    # print(f"  Filter matches: {stats['filter_matches']}")
    # print(f"  Filter misses: {stats['filter_misses']}")
    # print(f"\nEvents by type:")
    # for type_id, count in stats['events_by_type'].items():
    #     print(f"    Type {type_id}: {count} events")
    #
    # # Reset stats
    # system.reset_stats()
    # print("\nStatistics reset")

    print("TODO: Build _core with python-bindings feature")
    print()


def example_multiple_subscribers():
    """Example 4: Multiple subscribers with different filters"""
    print("=" * 60)
    print("Example 4: Multiple Subscribers")
    print("=" * 60)

    # TODO: Uncomment when ready
    # system = SignalSystem()
    #
    # high_priority_events = []
    # text_events = []
    # all_events = []
    #
    # # Subscriber 1: High priority events
    # sub1 = system.subscribe(
    #     name="high_priority_monitor",
    #     filter_dict={"priority": {"$gte": 200}},
    #     callback=lambda e: high_priority_events.append(e)
    # )
    #
    # # Subscriber 2: Text events only
    # sub2 = system.subscribe(
    #     name="text_monitor",
    #     filter_dict={"event_type": {"$wildcard": "*.text.*"}},
    #     callback=lambda e: text_events.append(e)
    # )
    #
    # # Subscriber 3: All events
    # sub3 = system.subscribe(
    #     name="all_monitor",
    #     filter_dict={},  # No filter = match all
    #     callback=lambda e: all_events.append(e)
    # )
    #
    # print(f"Active subscribers: {system.subscriber_count()}")
    #
    # # Emit various events
    # events = [
    #     ("signal.input.text.chat", 250),      # Matches: all 3
    #     ("signal.input.audio.mic", 180),      # Matches: sub3
    #     ("signal.output.text.reply", 210),    # Matches: sub1, sub2, sub3
    #     ("signal.system.timer", 50),          # Matches: sub3
    # ]
    #
    # for event_type, priority in events:
    #     system.emit(
    #         event_type=event_type,
    #         vector=[0.5] * 8,
    #         priority=priority
    #     )
    #
    # print(f"\nEvents received:")
    # print(f"  High priority (≥200): {len(high_priority_events)}")
    # print(f"  Text events: {len(text_events)}")
    # print(f"  All events: {len(all_events)}")
    #
    # # Cleanup
    # system.unsubscribe(sub1)
    # system.unsubscribe(sub2)
    # system.unsubscribe(sub3)

    print("TODO: Build _core with python-bindings feature")
    print()


def main():
    """Run all examples"""
    print("\n" + "=" * 60)
    print("SignalSystem v1.1 - Python Examples")
    print("=" * 60)
    print()

    example_basic_emit()
    example_subscription_with_filter()
    example_statistics()
    example_multiple_subscribers()

    print("=" * 60)
    print("Examples complete!")
    print("=" * 60)
    print()
    print("To build the Python module:")
    print("  cd src/core_rust")
    print("  maturin develop --features python-bindings")
    print()


if __name__ == "__main__":
    main()
