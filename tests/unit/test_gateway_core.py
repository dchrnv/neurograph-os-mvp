#!/usr/bin/env python3
"""
Test for SignalGateway - Phase 3
"""

from src.gateway import SignalGateway, EncoderType


def test_gateway_initialization():
    """Test Gateway initialization"""
    print("=" * 60)
    print("Test 1: Gateway Initialization")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    stats = gateway.get_stats()
    print(f"Gateway initialized:")
    print(f"  Registered sensors: {stats['registered_sensors']}")
    print(f"  Enabled sensors: {stats['enabled_sensors']}")
    print(f"  Total events: {stats['total_events']}")
    print(f"  NeuroTick: {stats['neuro_tick']}")
    print()

    # Check built-in sensors
    sensors = gateway.list_sensors()
    print(f"Built-in sensors:")
    for sensor in sensors:
        print(f"  - {sensor.sensor_id} ({sensor.sensor_type})")
    print()


def test_push_text():
    """Test push_text method"""
    print("=" * 60)
    print("Test 2: Push Text Signal")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    # Push text signal
    event = gateway.push_text(
        text="Hello, NeuroGraph!",
        priority=200,
        metadata={"language": "en"}
    )

    print(f"Text signal created:")
    print(f"  event_id: {event.event_id}")
    print(f"  event_type: {event.event_type}")
    print(f"  source.sensor_id: {event.source.sensor_id}")
    print(f"  source.modality: {event.source.modality}")
    print(f"  semantic.vector: {event.semantic.vector}")
    print(f"  semantic.encoding_method: {event.semantic.encoding_method}")
    print(f"  energy.urgency: {event.energy.urgency:.2f}")
    print(f"  temporal.neuro_tick: {event.temporal.neuro_tick}")
    print(f"  payload.data: {event.payload.data}")
    print(f"  payload.size_bytes: {event.payload.size_bytes}")
    print(f"  routing.priority: {event.routing.priority}")
    print(f"  routing.tags: {event.routing.tags}")
    print()

    # Verify stats updated
    stats = gateway.get_stats()
    assert stats['total_events'] == 1
    assert stats['neuro_tick'] == 1
    print(f"✓ Stats updated: {stats['total_events']} events, tick={stats['neuro_tick']}")
    print()


def test_push_system():
    """Test push_system method"""
    print("=" * 60)
    print("Test 3: Push System Metric")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    # Push system metric
    event = gateway.push_system(
        metric_name="cpu_percent",
        metric_value=45.7,
        priority=100,
        metadata={"host": "localhost"}
    )

    print(f"System metric signal created:")
    print(f"  event_id: {event.event_id}")
    print(f"  event_type: {event.event_type}")
    print(f"  source.sensor_id: {event.source.sensor_id}")
    print(f"  source.domain: {event.source.domain}")
    print(f"  payload.data_type: {event.payload.data_type}")
    print(f"  payload.data: {event.payload.data}")
    print(f"  routing.priority: {event.routing.priority}")
    print()


def test_custom_sensor():
    """Test custom sensor registration and usage"""
    print("=" * 60)
    print("Test 4: Custom Sensor")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    # Register custom sensor
    gateway.register_sensor(
        sensor_id="custom.weather_api",
        sensor_type="weather_feed",
        domain="external",
        modality="json",
        encoder_type=EncoderType.NUMERIC_DIRECT,
        description="Weather API data feed",
        default_priority=120,
        metadata={"api_version": "v2", "provider": "openweather"}
    )

    print(f"✓ Custom sensor registered")
    print()

    # Use custom sensor
    weather_data = {
        "temperature": 22.5,
        "humidity": 65,
        "pressure": 1013
    }

    event = gateway.push_system(
        metric_name="weather",
        metric_value=22.5,
        sensor_id="custom.weather_api",
        priority=120
    )

    print(f"Custom sensor event created:")
    print(f"  event_type: {event.event_type}")
    print(f"  source.sensor_id: {event.source.sensor_id}")
    print(f"  source.sensor_type: {event.source.sensor_type}")
    print(f"  source.sensor_meta: {event.source.sensor_meta}")
    print()


def test_sequence_tracking():
    """Test sequence ID tracking for conversations"""
    print("=" * 60)
    print("Test 5: Sequence Tracking")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    sequence_id = "conv_test_001"

    # Push multiple messages in sequence
    messages = [
        "Hello!",
        "How are you?",
        "What's the weather like?",
    ]

    events = []
    for i, msg in enumerate(messages):
        event = gateway.push_text(
            text=msg,
            sequence_id=sequence_id,
            priority=200
        )
        events.append(event)

    print(f"Created sequence of {len(events)} events:")
    for i, event in enumerate(events):
        print(f"  {i+1}. tick={event.temporal.neuro_tick}, "
              f"seq_id={event.temporal.sequence_id}, "
              f"text=\"{event.payload.data}\"")
    print()

    # Verify neuro_tick increments
    assert events[0].temporal.neuro_tick == 1
    assert events[1].temporal.neuro_tick == 2
    assert events[2].temporal.neuro_tick == 3
    print(f"✓ NeuroTick increments correctly")
    print()

    # Verify sequence_id preserved
    for event in events:
        assert event.temporal.sequence_id == sequence_id
    print(f"✓ Sequence ID preserved: {sequence_id}")
    print()


def test_error_handling():
    """Test error handling"""
    print("=" * 60)
    print("Test 6: Error Handling")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    # Test 1: Unknown sensor
    print("1. Unknown sensor (expecting error):")
    try:
        gateway.push_text(
            text="Test",
            sensor_id="nonexistent_sensor"
        )
        print("   ✗ Failed to catch error!")
    except ValueError as e:
        print(f"   ✓ Caught: {e}")
    print()

    # Test 2: Disabled sensor
    print("2. Disabled sensor (expecting error):")
    gateway.register_sensor(
        sensor_id="disabled_test",
        sensor_type="test",
        domain="system",
        modality="text",
        encoder_type=EncoderType.PASSTHROUGH
    )
    # Disable it after registration
    gateway.registry.disable_sensor("disabled_test")

    try:
        gateway.push_text(
            text="Test",
            sensor_id="disabled_test"
        )
        print("   ✗ Failed to catch error!")
    except ValueError as e:
        print(f"   ✓ Caught: {e}")
    print()


def test_json_export():
    """Test JSON export of SignalEvent"""
    print("=" * 60)
    print("Test 7: JSON Export")
    print("=" * 60)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    event = gateway.push_text(
        text="Test message",
        priority=180,
        sequence_id="test_seq"
    )

    # Export to JSON
    json_str = event.model_dump_json(indent=2)
    print(f"JSON export size: {len(json_str)} bytes")
    print()
    print("JSON sample (first 600 chars):")
    print(json_str[:600] + "...")
    print()

    # Verify it's valid JSON
    import json
    parsed = json.loads(json_str)
    assert parsed["event_type"] == event.event_type
    assert parsed["payload"]["data"] == "Test message"
    print(f"✓ JSON is valid and parseable")
    print()


def main():
    print("\n" + "=" * 60)
    print("SignalGateway Test Suite")
    print("=" * 60)
    print()

    test_gateway_initialization()
    test_push_text()
    test_push_system()
    test_custom_sensor()
    test_sequence_tracking()
    test_error_handling()
    test_json_export()

    print("=" * 60)
    print("✓ All SignalGateway tests passed!")
    print("=" * 60)
    print()


if __name__ == "__main__":
    main()
