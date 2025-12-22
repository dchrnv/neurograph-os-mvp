#!/usr/bin/env python3
"""
Tests for Integration Pipeline - Phase 3
"""

import asyncio
from src.integration import SignalPipeline
from src.action_controller import ActionPriority
from src.action_controller.executors import TextResponseAction, LoggingAction


async def test_pipeline_without_core():
    """Test pipeline without Rust Core."""
    print("=" * 60)
    print("Test 1: Pipeline Without Core")
    print("=" * 60)
    print()

    # Create pipeline (no Core)
    pipeline = SignalPipeline()

    # Register actions
    pipeline.register_action(
        action_type="text_response",
        action_class=TextResponseAction,
        priority=ActionPriority.HIGH
    )

    pipeline.register_action(
        action_type="logging",
        action_class=LoggingAction,
        priority=ActionPriority.LOW
    )

    # Configure action paths
    pipeline.configure_actions(
        hot_path=["text_response"],
        cold_path=["logging"]
    )

    # Process text input
    result = await pipeline.process_text(
        text="Hello, NeuroGraph!",
        user_id="user_123",
        chat_id="chat_456",
        priority=200
    )

    print("Processing Result:")
    print(f"  Signal Event ID: {result['signal_event'].event_id}")
    print(f"  8D Vector: {result['signal_event'].semantic.vector[:4]}...")
    print(f"  Priority: {result['signal_event'].routing.priority}")
    print()

    print("Processing (without Core):")
    print(f"  From Core: {result['processing_result']['from_core']}")
    print(f"  Is Novel: {result['processing_result']['is_novel']}")
    print()

    print("Actions:")
    print(f"  Hot path executed: {result['action_results']['stats']['hot_path_executed']}")
    print(f"  Cold path queued: {result['action_results']['stats']['cold_path_queued']}")
    print()

    print("Hot Path Results:")
    for action_result in result['action_results']['hot_path_results']:
        print(f"  {action_result.action_id}: {action_result.status.value}")
        if action_result.success and action_result.data:
            response = action_result.data.get('response_text', 'N/A')
            print(f"    Response: {response}")
    print()

    print("Timing:")
    print(f"  Total time: {result['stats']['total_time_ms']:.2f}ms")
    print(f"  Action time: {result['stats']['action_time_ms']:.2f}ms")
    print()

    # Get stats
    stats = pipeline.get_stats()
    print("Pipeline Stats:")
    print(f"  Total processed: {stats['pipeline']['total_processed']}")
    print(f"  With Core: {stats['pipeline']['with_core']}")
    print(f"  Without Core: {stats['pipeline']['without_core']}")
    print()

    await pipeline.shutdown()

    print("✓ Pipeline without Core working")
    print()


async def test_multiple_messages():
    """Test processing multiple messages."""
    print("=" * 60)
    print("Test 2: Multiple Messages")
    print("=" * 60)
    print()

    pipeline = SignalPipeline()

    # Register actions
    pipeline.register_action("text_response", TextResponseAction)
    pipeline.register_action("logging", LoggingAction)

    pipeline.configure_actions(
        hot_path=["text_response"],
        cold_path=["logging"]
    )

    # Process multiple messages
    messages = [
        "Hello!",
        "How are you?",
        "What is NeuroGraph?",
        "Thank you!",
        "Goodbye!"
    ]

    for i, msg in enumerate(messages):
        result = await pipeline.process_text(
            text=msg,
            user_id="user_123",
            chat_id="chat_456",
            priority=200
        )

        print(f"Message {i+1}: \"{msg}\"")
        print(f"  Event ID: {result['signal_event'].event_id}")
        print(f"  Actions executed: {result['action_results']['stats']['hot_path_executed']}")
        print(f"  Time: {result['stats']['total_time_ms']:.2f}ms")

    print()

    # Final stats
    stats = pipeline.get_stats()
    print(f"Total processed: {stats['pipeline']['total_processed']}")
    print(f"Gateway events: {stats['gateway']['total_events']}")
    print()

    await pipeline.shutdown()

    print("✓ Multiple messages processed")
    print()


async def main():
    print("\n" + "=" * 60)
    print("Integration Pipeline Test Suite - Phase 3")
    print("=" * 60)
    print()

    await test_pipeline_without_core()
    await test_multiple_messages()

    print("=" * 60)
    print("✓ All Phase 3 Integration tests passed!")
    print("=" * 60)
    print()


if __name__ == "__main__":
    asyncio.run(main())
