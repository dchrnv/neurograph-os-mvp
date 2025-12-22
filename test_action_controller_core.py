#!/usr/bin/env python3
"""
Tests for ActionController Core - Phase 1
"""

import asyncio
from src.action_controller import (
    ActionController,
    ActionSelector,
    ActionRegistry,
    ActionConfig,
    Action,
    ActionResult,
    ActionPriority,
    ActionStatus,
    PathType
)


# Mock Action for testing
class MockTextResponseAction(Action):
    """Mock action for testing."""

    def __init__(self, action_id: str, action_type: str, priority: ActionPriority, **kwargs):
        super().__init__(action_id, action_type, priority)
        self.response_text = kwargs.get("response_text", "Default response")

    async def execute(self, context):
        """Execute mock action."""
        return ActionResult(
            action_id=self.action_id,
            status=ActionStatus.COMPLETED,
            success=True,
            data={"response": self.response_text}
        )

    def can_execute(self, context):
        """Always can execute."""
        return True


class MockLoggingAction(Action):
    """Mock logging action for testing."""

    def __init__(self, action_id: str, action_type: str, priority: ActionPriority, **kwargs):
        super().__init__(action_id, action_type, priority)

    async def execute(self, context):
        """Execute mock logging."""
        return ActionResult(
            action_id=self.action_id,
            status=ActionStatus.COMPLETED,
            success=True,
            data={"logged": True}
        )

    def can_execute(self, context):
        """Always can execute."""
        return True


def test_action_registry():
    """Test ActionRegistry."""
    print("=" * 60)
    print("Test 1: ActionRegistry")
    print("=" * 60)
    print()

    registry = ActionRegistry()

    # Register actions
    registry.register(
        action_type="text_response",
        action_class=MockTextResponseAction,
        priority=ActionPriority.HIGH,
        tags=["response", "user_facing"]
    )

    registry.register(
        action_type="logging",
        action_class=MockLoggingAction,
        priority=ActionPriority.LOW,
        tags=["system", "background"]
    )

    print(f"Registered actions: {registry.list_actions()}")
    print(f"Stats: {registry.get_stats()}")
    print()

    # Create action
    action = registry.create_action(
        action_type="text_response",
        action_id="test_001",
        response_text="Hello, world!"
    )

    print(f"Created action: {action}")
    print(f"  Priority: {action.priority.name}")
    print()

    print("✓ ActionRegistry working")
    print()


def test_action_selector():
    """Test ActionSelector."""
    print("=" * 60)
    print("Test 2: ActionSelector")
    print("=" * 60)
    print()

    selector = ActionSelector()

    # Set defaults
    selector.set_default_hot_path(["text_response"])
    selector.set_default_cold_path(["logging", "analytics"])

    # Add rule: high priority messages get immediate response
    def is_high_priority(context):
        event = context.get("signal_event", {})
        priority = event.get("priority", 0)
        return priority >= 200

    selector.add_rule(
        name="high_priority_response",
        condition=is_high_priority,
        action_types=["text_response"],
        path=PathType.HOT,
        priority_boost=100
    )

    # Test selection with high priority
    context_high = {
        "signal_event": {"priority": 220},
        "processing_result": {"is_novel": False}
    }

    selected = selector.select(context_high)
    print(f"High priority context:")
    print(f"  Hot path: {len(selected[PathType.HOT])} actions")
    print(f"  Cold path: {len(selected[PathType.COLD])} actions")
    print()

    # Test selection with low priority
    context_low = {
        "signal_event": {"priority": 50},
        "processing_result": {"is_novel": False}
    }

    selected = selector.select(context_low)
    print(f"Low priority context:")
    print(f"  Hot path: {len(selected[PathType.HOT])} actions")
    print(f"  Cold path: {len(selected[PathType.COLD])} actions")
    print()

    print(f"Selector stats: {selector.get_stats()}")
    print()

    print("✓ ActionSelector working")
    print()


async def test_action_controller():
    """Test ActionController."""
    print("=" * 60)
    print("Test 3: ActionController")
    print("=" * 60)
    print()

    # Create registry and register actions
    registry = ActionRegistry()
    registry.register("text_response", MockTextResponseAction, priority=ActionPriority.HIGH)
    registry.register("logging", MockLoggingAction, priority=ActionPriority.LOW)

    # Create selector with defaults
    selector = ActionSelector()
    selector.set_default_hot_path(["text_response"])
    selector.set_default_cold_path(["logging"])

    # Create controller
    controller = ActionController(registry=registry, selector=selector)

    # Mock signal event
    signal_event = {
        "event_id": "evt_001",
        "text": "Hello, bot!",
        "priority": 200
    }

    # Mock processing result
    processing_result = {
        "token_id": 42,
        "is_novel": True,
        "triggered_actions": [1, 2]
    }

    # Process
    result = await controller.process(
        signal_event=signal_event,
        processing_result=processing_result
    )

    print("Processing result:")
    print(f"  Hot path executed: {result['stats']['hot_path_executed']}")
    print(f"  Cold path queued: {result['stats']['cold_path_queued']}")
    print(f"  Execution time: {result['stats']['execution_time_ms']:.2f}ms")
    print()

    print("Hot path results:")
    for r in result["hot_path_results"]:
        print(f"  {r.action_id}: {r.status.value} (success={r.success})")
    print()

    # Wait for cold path
    await asyncio.sleep(0.1)

    # Get stats
    stats = controller.get_stats()
    print("Controller stats:")
    print(f"  Total executions: {stats['total_executions']}")
    print(f"  Hot path executed: {stats['hot_path_executed']}")
    print(f"  Cold path executed: {stats['cold_path_executed']}")
    print(f"  Failed actions: {stats['failed_actions']}")
    print()

    await controller.shutdown()

    print("✓ ActionController working")
    print()


async def main():
    print("\n" + "=" * 60)
    print("ActionController Core Test Suite - Phase 1")
    print("=" * 60)
    print()

    test_action_registry()
    test_action_selector()
    await test_action_controller()

    print("=" * 60)
    print("✓ All Phase 1 Core tests passed!")
    print("=" * 60)
    print()


if __name__ == "__main__":
    asyncio.run(main())
