#!/usr/bin/env python3
"""
Test for SensorRegistry - Phase 2
"""

from src.gateway.registry import SensorRegistry, SensorConfig
from src.gateway.encoders import EncoderType


def test_basic_registration():
    """Test basic sensor registration"""
    print("=" * 60)
    print("Test 1: Basic Registration")
    print("=" * 60)
    print()

    registry = SensorRegistry()

    # Register a custom sensor
    config = registry.register_sensor(
        sensor_id="test_sensor_001",
        sensor_type="custom_text",
        domain="external",
        modality="text",
        encoder_type=EncoderType.TEXT_TFIDF,
        description="Test text sensor",
        default_priority=150,
        metadata={"test_key": "test_value"}
    )

    print(f"✓ Registered sensor: {config.sensor_id}")
    print(f"  Type: {config.sensor_type}")
    print(f"  Domain: {config.domain}")
    print(f"  Modality: {config.modality}")
    print(f"  Encoder: {config.encoder_type}")
    print(f"  Priority: {config.default_priority}")
    print(f"  Metadata: {config.metadata}")
    print()

    # Check existence
    assert registry.sensor_exists("test_sensor_001")
    print("✓ Sensor exists in registry")
    print()

    # Retrieve sensor
    retrieved = registry.get_sensor("test_sensor_001")
    assert retrieved is not None
    assert retrieved.sensor_id == "test_sensor_001"
    print("✓ Sensor retrieved successfully")
    print()


def test_builtin_sensors():
    """Test built-in sensors registration"""
    print("=" * 60)
    print("Test 2: Built-in Sensors")
    print("=" * 60)
    print()

    registry = SensorRegistry()
    registry.register_builtin_sensors()

    # Check all built-in sensors
    builtin_ids = [
        "builtin.text_chat",
        "builtin.system_monitor",
        "builtin.timer"
    ]

    print(f"Checking {len(builtin_ids)} built-in sensors:")
    for sensor_id in builtin_ids:
        config = registry.get_sensor(sensor_id)
        assert config is not None
        print(f"  ✓ {sensor_id}")
        print(f"      Type: {config.sensor_type}")
        print(f"      Domain: {config.domain}")
        print(f"      Modality: {config.modality}")
        print(f"      Encoder: {config.encoder_type}")

    print()
    print(f"✓ All {len(builtin_ids)} built-in sensors registered")
    print()


def test_query_filtering():
    """Test sensor querying and filtering"""
    print("=" * 60)
    print("Test 3: Query & Filtering")
    print("=" * 60)
    print()

    registry = SensorRegistry()
    registry.register_builtin_sensors()

    # Filter by domain
    external_sensors = registry.list_sensors(domain="external")
    print(f"External sensors: {len(external_sensors)}")
    for s in external_sensors:
        print(f"  - {s.sensor_id} ({s.sensor_type})")
    print()

    system_sensors = registry.list_sensors(domain="system")
    print(f"System sensors: {len(system_sensors)}")
    for s in system_sensors:
        print(f"  - {s.sensor_id} ({s.sensor_type})")
    print()

    # Filter by modality
    text_sensors = registry.list_sensors(modality="text")
    print(f"Text modality sensors: {len(text_sensors)}")
    for s in text_sensors:
        print(f"  - {s.sensor_id} ({s.modality})")
    print()

    # Count sensors
    total = registry.count_sensors()
    print(f"Total active sensors: {total}")
    print()


def test_state_management():
    """Test sensor enable/disable"""
    print("=" * 60)
    print("Test 4: State Management")
    print("=" * 60)
    print()

    registry = SensorRegistry()
    registry.register_sensor(
        sensor_id="test_state",
        sensor_type="test",
        domain="system",
        modality="numeric",
        encoder_type=EncoderType.NUMERIC_DIRECT
    )

    # Initially enabled
    config = registry.get_sensor("test_state")
    assert config.enabled == True
    print(f"✓ Initial state: enabled={config.enabled}")

    # Disable
    registry.disable_sensor("test_state")
    config = registry.get_sensor("test_state")
    assert config.enabled == False
    print(f"✓ After disable: enabled={config.enabled}")

    # Enable
    registry.enable_sensor("test_state")
    config = registry.get_sensor("test_state")
    assert config.enabled == True
    print(f"✓ After enable: enabled={config.enabled}")
    print()

    # Check filtering by enabled state
    all_count = registry.count_sensors(enabled_only=False)
    enabled_count = registry.count_sensors(enabled_only=True)
    print(f"Total sensors: {all_count}")
    print(f"Enabled sensors: {enabled_count}")
    print()


def test_update_and_unregister():
    """Test sensor updates and removal"""
    print("=" * 60)
    print("Test 5: Update & Unregister")
    print("=" * 60)
    print()

    registry = SensorRegistry()
    registry.register_sensor(
        sensor_id="test_update",
        sensor_type="test",
        domain="external",
        modality="text",
        encoder_type=EncoderType.PASSTHROUGH,
        default_priority=100
    )

    # Update sensor
    registry.update_sensor(
        "test_update",
        default_priority=200,
        description="Updated description"
    )

    config = registry.get_sensor("test_update")
    assert config.default_priority == 200
    assert config.description == "Updated description"
    print(f"✓ Updated sensor:")
    print(f"  Priority: {config.default_priority}")
    print(f"  Description: {config.description}")
    print()

    # Unregister
    assert registry.sensor_exists("test_update")
    result = registry.unregister_sensor("test_update")
    assert result == True
    assert not registry.sensor_exists("test_update")
    print(f"✓ Sensor unregistered successfully")
    print()


def test_error_handling():
    """Test error cases"""
    print("=" * 60)
    print("Test 6: Error Handling")
    print("=" * 60)
    print()

    registry = SensorRegistry()

    # Test duplicate registration
    registry.register_sensor(
        sensor_id="duplicate_test",
        sensor_type="test",
        domain="system",
        modality="numeric",
        encoder_type=EncoderType.NUMERIC_DIRECT
    )

    print("1. Duplicate registration (expecting error):")
    try:
        registry.register_sensor(
            sensor_id="duplicate_test",
            sensor_type="test",
            domain="system",
            modality="numeric",
            encoder_type=EncoderType.NUMERIC_DIRECT
        )
        print("   ✗ Failed to catch duplicate registration!")
    except ValueError as e:
        print(f"   ✓ Caught: {e}")
    print()

    # Test update non-existent sensor
    print("2. Update non-existent sensor (expecting error):")
    try:
        registry.update_sensor("nonexistent", default_priority=200)
        print("   ✗ Failed to catch missing sensor!")
    except KeyError as e:
        print(f"   ✓ Caught: {e}")
    print()


def main():
    print("\n" + "=" * 60)
    print("SensorRegistry Test Suite")
    print("=" * 60)
    print()

    test_basic_registration()
    test_builtin_sensors()
    test_query_filtering()
    test_state_management()
    test_update_and_unregister()
    test_error_handling()

    print("=" * 60)
    print("✓ All SensorRegistry tests passed!")
    print("=" * 60)
    print()


if __name__ == "__main__":
    main()
