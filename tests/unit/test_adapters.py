#!/usr/bin/env python3
"""
Tests for Gateway Adapters - Phase 2
"""

import time
from src.gateway import SignalGateway
from src.gateway.adapters import TextAdapter, SystemAdapter, TimerAdapter


def test_text_adapter_basic():
    """Test TextAdapter basic message handling"""
    print("=" * 60)
    print("Test 1: TextAdapter - Basic Messages")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    adapter = TextAdapter(gateway, source_name="telegram")

    # Handle simple message
    event = adapter.handle_message(
        text="Hello, bot!",
        user_id="user_123",
        chat_id="chat_456"
    )

    print(f"Message sent:")
    print(f"  Text: {event.payload.data}")
    print(f"  User ID: {event.payload.metadata.get('user_id')}")
    print(f"  Chat ID: {event.payload.metadata.get('chat_id')}")
    print(f"  Source: {event.payload.metadata.get('source')}")
    print(f"  Tags: {event.routing.tags}")
    print(f"  Sequence ID: {event.temporal.sequence_id}")
    print()

    # Verify
    assert "user_123" in str(event.payload.metadata)
    assert "telegram" in event.routing.tags
    assert event.temporal.sequence_id is not None
    print("✓ TextAdapter basic message working")
    print()


def test_text_adapter_commands():
    """Test TextAdapter command handling"""
    print("=" * 60)
    print("Test 2: TextAdapter - Commands")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    adapter = TextAdapter(gateway, source_name="telegram")

    # Handle command
    event = adapter.handle_command(
        command="start",
        args=["arg1", "arg2"],
        user_id="user_123",
        chat_id="chat_456"
    )

    print(f"Command sent:")
    print(f"  Text: {event.payload.data}")
    print(f"  Command: {event.payload.metadata.get('command')}")
    print(f"  Args: {event.payload.metadata.get('args')}")
    print(f"  Priority: {event.routing.priority}")
    print(f"  Tags: {event.routing.tags}")
    print()

    # Verify
    assert event.payload.data == "/start arg1 arg2"
    assert event.payload.metadata["command"] == "start"
    assert event.payload.metadata["is_command"] == True
    assert "command" in event.routing.tags
    assert event.routing.priority == 220  # Commands have higher priority
    print("✓ TextAdapter command handling working")
    print()


def test_text_adapter_conversations():
    """Test TextAdapter conversation tracking"""
    print("=" * 60)
    print("Test 3: TextAdapter - Conversations")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    adapter = TextAdapter(gateway, source_name="telegram")

    chat_id = "chat_001"

    # Send multiple messages in same chat
    events = []
    for i, text in enumerate(["Hello", "How are you?", "Goodbye"], 1):
        event = adapter.handle_message(
            text=text,
            user_id="user_123",
            chat_id=chat_id
        )
        events.append(event)

    print(f"Conversation in chat_id={chat_id}:")
    for i, event in enumerate(events, 1):
        print(f"  {i}. \"{event.payload.data}\" (seq={event.temporal.sequence_id})")

    # Verify all have same sequence_id
    sequence_ids = [e.temporal.sequence_id for e in events]
    assert len(set(sequence_ids)) == 1, "All messages should have same sequence_id"
    print()
    print(f"✓ All {len(events)} messages in same conversation")
    print(f"  Sequence ID: {sequence_ids[0]}")
    print()

    # Reset conversation
    adapter.reset_conversation(chat_id)
    new_event = adapter.handle_message("New conversation", user_id="user_123", chat_id=chat_id)
    assert new_event.temporal.sequence_id != sequence_ids[0]
    print(f"✓ Conversation reset working")
    print(f"  New sequence ID: {new_event.temporal.sequence_id}")
    print()


def test_system_adapter_basic():
    """Test SystemAdapter basic metrics"""
    print("=" * 60)
    print("Test 4: SystemAdapter - Basic Metrics")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    adapter = SystemAdapter(gateway)

    # Send single metric
    event = adapter.send_metric("cpu_percent", 45.7)

    print(f"Metric sent:")
    print(f"  Name: cpu_percent")
    print(f"  Value: 45.7")
    print(f"  Priority: {event.routing.priority}")
    print(f"  Sensor: {event.source.sensor_id}")
    print()

    assert event.payload.data["metric"] == "cpu_percent"
    assert event.payload.data["value"] == 45.7
    print("✓ SystemAdapter basic metric working")
    print()


def test_system_adapter_alerts():
    """Test SystemAdapter alert system"""
    print("=" * 60)
    print("Test 5: SystemAdapter - Alerts")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    adapter = SystemAdapter(gateway)

    # Send alerts with different severities
    alerts = [
        ("disk_full", "critical", "Disk usage > 95%"),
        ("high_memory", "warning", "Memory usage > 80%"),
        ("service_started", "info", "Service XYZ started"),
    ]

    for alert_name, severity, message in alerts:
        event = adapter.send_alert(alert_name, severity, message)
        print(f"{severity.upper()}: {alert_name}")
        print(f"  Priority: {event.routing.priority}")
        print(f"  Message: {message}")

    print()
    print("✓ SystemAdapter alerts working")
    print()


def test_system_adapter_custom_metrics():
    """Test SystemAdapter custom metrics"""
    print("=" * 60)
    print("Test 6: SystemAdapter - Custom Metrics")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    adapter = SystemAdapter(gateway)

    # Register custom metrics
    queue_size = [10]  # Mutable so we can modify in collector

    def get_queue_size():
        return float(queue_size[0])

    def get_active_users():
        return 42.0

    adapter.register_custom_metric("queue_size", get_queue_size)
    adapter.register_custom_metric("active_users", get_active_users)

    # Send custom metrics
    events = adapter.send_custom_metrics()

    print(f"Custom metrics sent:")
    for name, event in events.items():
        value = event.payload.data["value"]
        print(f"  {name}: {value}")

    assert "queue_size" in events
    assert "active_users" in events
    print()
    print("✓ SystemAdapter custom metrics working")
    print()


def test_timer_adapter_single_event():
    """Test TimerAdapter single event"""
    print("=" * 60)
    print("Test 7: TimerAdapter - Single Event")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    adapter = TimerAdapter(gateway)

    # Send single timer event
    event = adapter.send_timer_event(
        timer_name="test_timer",
        tick_count=5,
        priority=75
    )

    print(f"Timer event sent:")
    print(f"  Timer name: {event.payload.metadata['timer_name']}")
    print(f"  Tick count: {event.payload.metadata['tick_count']}")
    print(f"  Priority: {event.routing.priority}")
    print()

    assert event.payload.metadata["timer_name"] == "test_timer"
    assert event.payload.metadata["tick_count"] == 5
    print("✓ TimerAdapter single event working")
    print()


def test_timer_adapter_periodic():
    """Test TimerAdapter periodic timer"""
    print("=" * 60)
    print("Test 8: TimerAdapter - Periodic Timer")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    adapter = TimerAdapter(gateway)

    # Track ticks
    tick_events = []

    def on_tick(event):
        tick_events.append(event)
        print(f"  Tick {len(tick_events)}: neuro_tick={event.temporal.neuro_tick}")

    # Start periodic timer (5 ticks, 0.2s interval)
    print("Starting periodic timer (5 ticks, 0.2s interval):")
    timer_id = adapter.start_periodic(
        interval_seconds=0.2,
        callback=on_tick,
        timer_name="test_periodic",
        max_ticks=5,
        priority=50
    )

    # Wait for completion
    time.sleep(1.5)

    print()
    print(f"✓ Periodic timer completed")
    print(f"  Total ticks: {len(tick_events)}")
    assert len(tick_events) == 5
    print()


def test_timer_adapter_countdown():
    """Test TimerAdapter countdown"""
    print("=" * 60)
    print("Test 9: TimerAdapter - Countdown")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    adapter = TimerAdapter(gateway)

    # Track countdown
    countdown_ticks = []

    def on_countdown(event):
        tick = event.payload.metadata["tick_count"]
        countdown_ticks.append(tick)
        print(f"  Countdown tick {tick}")

    # Start 1-second countdown with 0.3s ticks
    print("Starting 1-second countdown (0.3s interval):")
    timer_id = adapter.start_countdown(
        duration_seconds=1.0,
        callback=on_countdown,
        timer_name="test_countdown",
        tick_interval=0.3,
        priority=100
    )

    # Wait for completion
    time.sleep(1.5)

    print()
    print(f"✓ Countdown completed")
    print(f"  Total ticks: {len(countdown_ticks)}")
    assert len(countdown_ticks) == 3  # 1.0 / 0.3 = ~3 ticks
    print()


def test_timer_adapter_stop():
    """Test TimerAdapter stop functionality"""
    print("=" * 60)
    print("Test 10: TimerAdapter - Stop Timer")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    adapter = TimerAdapter(gateway)

    tick_count = [0]

    def on_tick(event):
        tick_count[0] += 1

    # Start periodic timer
    timer_id = adapter.start_periodic(
        interval_seconds=0.1,
        callback=on_tick,
        timer_name="test_stop"
    )

    print(f"Timer started: {timer_id}")

    # Let it run for a bit
    time.sleep(0.35)
    ticks_before_stop = tick_count[0]
    print(f"  Ticks before stop: {ticks_before_stop}")

    # Stop timer
    result = adapter.stop_timer(timer_id)
    assert result == True
    print(f"✓ Timer stopped")

    # Wait a bit more
    time.sleep(0.3)
    ticks_after_stop = tick_count[0]
    print(f"  Ticks after stop: {ticks_after_stop}")

    # Should not have increased much
    assert ticks_after_stop == ticks_before_stop, "Timer should have stopped"
    print(f"✓ Timer correctly stopped (no new ticks)")
    print()


def main():
    print("\n" + "=" * 60)
    print("Gateway Adapters Test Suite")
    print("=" * 60)
    print()

    test_text_adapter_basic()
    test_text_adapter_commands()
    test_text_adapter_conversations()
    test_system_adapter_basic()
    test_system_adapter_alerts()
    test_system_adapter_custom_metrics()
    test_timer_adapter_single_event()
    test_timer_adapter_periodic()
    test_timer_adapter_countdown()
    test_timer_adapter_stop()

    print("=" * 60)
    print("✓ All Adapter tests passed!")
    print("=" * 60)
    print()


if __name__ == "__main__":
    main()
