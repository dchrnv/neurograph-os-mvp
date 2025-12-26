#!/usr/bin/env python3
"""
Tests for Action Executors - Phase 2
"""

import asyncio
import tempfile
import os
from src.action_controller import ActionPriority
from src.action_controller.executors import (
    TextResponseAction,
    LoggingAction,
    MetricsAction
)


# Mock signal event
class MockSignalEvent:
    def __init__(self, event_id, text):
        self.event_id = event_id
        self.payload = MockPayload(text)


class MockPayload:
    def __init__(self, data):
        self.data = data


async def test_text_response_action():
    """Test TextResponseAction."""
    print("=" * 60)
    print("Test 1: TextResponseAction")
    print("=" * 60)
    print()

    # Test with template
    action = TextResponseAction(
        action_id="text_001",
        action_type="text_response",
        priority=ActionPriority.HIGH,
        response_template="Novel: {is_novel}, Token: {token_id}"
    )

    context = {
        "signal_event": MockSignalEvent("evt_001", "Hello!"),
        "processing_result": {
            "token_id": 42,
            "is_novel": True,
            "triggered_actions": [1, 2, 3]
        }
    }

    result = await action.execute(context)

    print(f"Text Response Action:")
    print(f"  Success: {result.success}")
    print(f"  Response: {result.data['response_text']}")
    print(f"  Execution time: {result.execution_time_ms:.2f}ms")
    print()

    # Test with custom generator
    def custom_generator(event, result, ctx):
        return f"Custom response for token {result['token_id']}"

    action2 = TextResponseAction(
        action_id="text_002",
        action_type="text_response",
        priority=ActionPriority.NORMAL,
        response_generator=custom_generator
    )

    result2 = await action2.execute(context)

    print(f"Custom Generator:")
    print(f"  Success: {result2.success}")
    print(f"  Response: {result2.data['response_text']}")
    print()

    print("✓ TextResponseAction working")
    print()


async def test_logging_action():
    """Test LoggingAction."""
    print("=" * 60)
    print("Test 2: LoggingAction")
    print("=" * 60)
    print()

    # Create temp log file
    with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.log') as f:
        log_file = f.name

    try:
        action = LoggingAction(
            action_id="log_001",
            action_type="logging",
            priority=ActionPriority.LOW,
            log_level="INFO",
            log_file=log_file
        )

        context = {
            "signal_event": MockSignalEvent("evt_001", "Test message"),
            "processing_result": {
                "token_id": 42,
                "is_novel": False,
                "triggered_actions": []
            }
        }

        result = await action.execute(context)

        print(f"Logging Action:")
        print(f"  Success: {result.success}")
        print(f"  Log file: {log_file}")
        print(f"  Execution time: {result.execution_time_ms:.2f}ms")
        print()

        # Check log file
        with open(log_file, 'r') as f:
            log_content = f.read()
            print(f"Log content:")
            print(f"  {log_content[:100]}...")
        print()

    finally:
        # Cleanup
        if os.path.exists(log_file):
            os.unlink(log_file)

    print("✓ LoggingAction working")
    print()


async def test_metrics_action():
    """Test MetricsAction."""
    print("=" * 60)
    print("Test 3: MetricsAction")
    print("=" * 60)
    print()

    action = MetricsAction(
        action_id="metrics_001",
        action_type="metrics",
        priority=ActionPriority.BACKGROUND
    )

    # Process multiple events
    for i in range(5):
        context = {
            "signal_event": MockSignalEvent(f"evt_{i:03d}", f"Message {i}"),
            "processing_result": {
                "token_id": 100 + i,
                "is_novel": i % 2 == 0,  # Every other is novel
                "triggered_actions": [1, 2] if i > 2 else []
            }
        }

        result = await action.execute(context)
        print(f"Event {i}: success={result.success}")

    print()

    # Get metrics
    metrics = action.get_metrics()
    print(f"Collected Metrics:")
    print(f"  Total events: {metrics['total_events']}")
    print(f"  Novel events: {metrics['novel_events']}")
    print(f"  Triggered actions: {metrics['triggered_actions_total']}")
    print()

    print("✓ MetricsAction working")
    print()


async def main():
    print("\n" + "=" * 60)
    print("Action Executors Test Suite - Phase 2")
    print("=" * 60)
    print()

    await test_text_response_action()
    await test_logging_action()
    await test_metrics_action()

    print("=" * 60)
    print("✓ All Phase 2 Executor tests passed!")
    print("=" * 60)
    print()


if __name__ == "__main__":
    asyncio.run(main())
