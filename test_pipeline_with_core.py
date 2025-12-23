#!/usr/bin/env python3
"""
Test SignalPipeline with Rust Core - v0.57.0

Tests full end-to-end integration:
Gateway → Rust Core → ActionController
"""

import sys
sys.path.insert(0, 'src/core_rust/target/release')

import asyncio
import _core
from src.integration import SignalPipeline
from src.action_controller import ActionPriority
from src.action_controller.executors import TextResponseAction, LoggingAction


async def test_pipeline_with_core():
    """Test pipeline WITH Rust Core."""
    print("=" * 60)
    print("Test: Pipeline WITH Rust Core")
    print("=" * 60)
    print()

    # Create Rust Core
    core = _core.SignalSystem()
    print("✓ Rust Core created")

    # Create pipeline with Core
    pipeline = SignalPipeline(core_system=core)
    print("✓ Pipeline created with Core")

    # Register actions
    pipeline.register_action("text_response", TextResponseAction)
    pipeline.register_action("logging", LoggingAction)

    pipeline.configure_actions(
        hot_path=["text_response"],
        cold_path=["logging"]
    )
    print("✓ Actions configured")
    print()

    # Process text through full pipeline
    print("Processing: 'Hello, NeuroGraph with Core!'")
    result = await pipeline.process_text(
        text="Hello, NeuroGraph with Core!",
        user_id="user_123",
        chat_id="chat_456",
        priority=200
    )

    print()
    print("=" * 60)
    print("Results:")
    print("=" * 60)
    print()

    # Signal Event
    print("Signal Event:")
    print(f"  ID: {result['signal_event'].event_id}")
    print(f"  Type: {result['signal_event'].event_type}")
    print(f"  8D Vector: {result['signal_event'].semantic.vector[:4]}...")
    print(f"  Priority: {result['signal_event'].routing.priority}")
    print()

    # Core Processing
    print("Core Processing:")
    print(f"  From Core: {result['processing_result']['from_core']}")
    print(f"  Token ID: {result['processing_result']['token_id']}")
    print(f"  Is Novel: {result['processing_result']['is_novel']}")
    print(f"  Neighbors: {len(result['processing_result']['neighbors'])}")
    print(f"  Triggered Actions: {result['processing_result']['triggered_actions']}")
    print(f"  Processing Time: {result['processing_result']['processing_time_us']}μs")
    print(f"  Core Time: {result['stats']['core_time_ms']:.2f}ms")
    print()

    # Actions
    print("Actions:")
    print(f"  Hot path executed: {result['action_results']['stats']['hot_path_executed']}")
    print(f"  Cold path queued: {result['action_results']['stats']['cold_path_queued']}")
    print()

    # Timing
    print("Performance:")
    print(f"  Total time: {result['stats']['total_time_ms']:.2f}ms")
    print(f"  Gateway time: ~{result['stats']['total_time_ms'] - result['stats']['core_time_ms']:.2f}ms")
    print(f"  Core time: {result['stats']['core_time_ms']:.2f}ms")
    print(f"  Action time: {result['stats']['action_time_ms']:.2f}ms")
    print()

    await pipeline.shutdown()

    print("✓ Pipeline with Core working!")
    print()


async def test_multiple_with_core():
    """Test processing multiple messages through Core."""
    print("=" * 60)
    print("Test: Multiple Messages Through Core")
    print("=" * 60)
    print()

    # Setup
    core = _core.SignalSystem()
    pipeline = SignalPipeline(core_system=core)

    pipeline.register_action("text_response", TextResponseAction)
    pipeline.register_action("logging", LoggingAction)
    pipeline.configure_actions(hot_path=["text_response"], cold_path=["logging"])

    # Process 5 messages
    messages = [
        "First message",
        "Second message",
        "Third message",
        "Fourth message",
        "Fifth message"
    ]

    for i, msg in enumerate(messages):
        result = await pipeline.process_text(
            text=msg,
            user_id="user_123",
            priority=200
        )

        print(f"Message {i+1}: \"{msg}\"")
        print(f"  Core: token_id={result['processing_result']['token_id']}, novel={result['processing_result']['is_novel']}")
        print(f"  Time: {result['stats']['total_time_ms']:.2f}ms (core: {result['stats']['core_time_ms']:.2f}ms)")

    print()

    # Get stats
    stats = pipeline.get_stats()
    core_stats = core.get_stats()

    print("Pipeline Stats:")
    print(f"  Total processed: {stats['pipeline']['total_processed']}")
    print(f"  With Core: {stats['pipeline']['with_core']}")
    print(f"  Without Core: {stats['pipeline']['without_core']}")
    print()

    print("Core Stats:")
    print(f"  Total events: {core_stats['total_events']}")
    print(f"  Avg processing time: {core_stats['avg_processing_time_us']:.2f}μs")
    print()

    await pipeline.shutdown()

    print("✓ Multiple messages processed through Core!")
    print()


async def test_core_performance():
    """Test performance with Core."""
    print("=" * 60)
    print("Test: Performance with Core")
    print("=" * 60)
    print()

    core = _core.SignalSystem()
    pipeline = SignalPipeline(core_system=core)

    pipeline.register_action("text_response", TextResponseAction)
    pipeline.configure_actions(hot_path=["text_response"], cold_path=[])

    import time
    start = time.time()

    # Process 100 messages
    for i in range(100):
        await pipeline.process_text(
            text=f"Message {i}",
            priority=200
        )

    elapsed = (time.time() - start) * 1000

    # Stats
    stats = pipeline.get_stats()
    core_stats = core.get_stats()

    print(f"✓ Processed 100 messages in {elapsed:.2f}ms")
    print(f"  Throughput: {100 / (elapsed/1000):.0f} messages/sec")
    print(f"  Avg total time: {elapsed/100:.2f}ms per message")
    print(f"  Core avg time: {core_stats['avg_processing_time_us']:.2f}μs")
    print()

    await pipeline.shutdown()


async def main():
    print("\n" + "=" * 60)
    print("SignalPipeline + Rust Core Integration Tests - v0.57.0")
    print("=" * 60)
    print()

    try:
        await test_pipeline_with_core()
        await test_multiple_with_core()
        await test_core_performance()

        print("=" * 60)
        print("✓ All integration tests passed!")
        print("=" * 60)
        print()

    except Exception as e:
        print(f"\n❌ Test failed: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    asyncio.run(main())
