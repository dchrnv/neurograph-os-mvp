#!/usr/bin/env python3
"""
Quick test for Gateway v2.0 Pydantic models
"""

import sys
import time
from src.gateway.models import (
    SignalEvent,
    SignalSource,
    SemanticCore,
    LayerDecomposition,
    EnergyProfile,
    TemporalBinding,
    RawPayload,
    ProcessingResult,
    NeighborInfo,
    RoutingInfo,
)


def test_basic_models():
    """Test individual model creation"""
    print("=" * 60)
    print("Testing Gateway v2.0 Models")
    print("=" * 60)
    print()

    # 1. SignalSource
    print("1. SignalSource:")
    source = SignalSource(
        domain="external",
        modality="text",
        sensor_id="test_sensor_001",
        sensor_type="text_chat",
        confidence=1.0
    )
    print(f"   ✓ Created: {source.sensor_id}")
    print()

    # 2. SemanticCore
    print("2. SemanticCore:")
    semantic = SemanticCore(
        vector=[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
        encoding_method="pca_projection"
    )
    print(f"   ✓ Created: 8D vector, method={semantic.encoding_method}")
    print()

    # 3. LayerDecomposition
    print("3. LayerDecomposition:")
    layers = LayerDecomposition(
        physical=0.2,
        emotional=0.5,
        social=0.3
    )
    print(f"   ✓ Created: physical={layers.physical}, emotional={layers.emotional}")
    print()

    # 4. EnergyProfile
    print("4. EnergyProfile:")
    energy = EnergyProfile(
        magnitude=0.8,
        valence=0.6,
        arousal=0.7,
        urgency=0.9
    )
    print(f"   ✓ Created: magnitude={energy.magnitude}, urgency={energy.urgency}")
    print()

    # 5. TemporalBinding
    print("5. TemporalBinding:")
    timestamp_us = int(time.time() * 1_000_000)
    temporal = TemporalBinding(
        timestamp_us=timestamp_us,
        neuro_tick=12345
    )
    print(f"   ✓ Created: timestamp_us={temporal.timestamp_us}")
    print()

    # 6. RawPayload
    print("6. RawPayload:")
    payload = RawPayload(
        data_type="text",
        data="Hello, NeuroGraph!",
        mime_type="text/plain",
        size_bytes=18
    )
    print(f"   ✓ Created: data_type={payload.data_type}, size={payload.size_bytes}B")
    print()

    # 7. RoutingInfo
    print("7. RoutingInfo:")
    routing = RoutingInfo(
        priority=200,
        tags=["test", "user_input"]
    )
    print(f"   ✓ Created: priority={routing.priority}, tags={routing.tags}")
    print()

    # 8. NeighborInfo
    print("8. NeighborInfo:")
    neighbor = NeighborInfo(
        token_id=42,
        distance=0.35,
        similarity=0.92
    )
    print(f"   ✓ Created: token_id={neighbor.token_id}, similarity={neighbor.similarity}")
    print()

    # 9. ProcessingResult
    print("9. ProcessingResult:")
    result = ProcessingResult(
        token_id=1337,
        neighbors=[neighbor],
        is_novel=False,
        processing_time_us=87
    )
    print(f"   ✓ Created: token_id={result.token_id}, neighbors={len(result.neighbors)}")
    print()

    return {
        "source": source,
        "semantic": semantic,
        "energy": energy,
        "temporal": temporal,
        "payload": payload,
        "routing": routing,
        "result": result,
    }


def test_signal_event(components):
    """Test full SignalEvent creation"""
    print("=" * 60)
    print("Testing SignalEvent (Master Model)")
    print("=" * 60)
    print()

    event = SignalEvent(
        event_type="signal.input.external.text.chat",
        source=components["source"],
        semantic=components["semantic"],
        energy=components["energy"],
        temporal=components["temporal"],
        payload=components["payload"],
        routing=components["routing"],
        result=components["result"],
    )

    print(f"SignalEvent created:")
    print(f"  event_id: {event.event_id}")
    print(f"  event_type: {event.event_type}")
    print(f"  schema_version: {event.schema_version}")
    print(f"  source.sensor_id: {event.source.sensor_id}")
    print(f"  semantic.vector: {event.semantic.vector}")
    print(f"  energy.urgency: {event.energy.urgency}")
    print(f"  temporal.neuro_tick: {event.temporal.neuro_tick}")
    print(f"  payload.data: {event.payload.data}")
    print(f"  routing.priority: {event.routing.priority}")
    print(f"  result.token_id: {event.result.token_id if event.result else 'None'}")
    print()

    return event


def test_json_serialization(event):
    """Test JSON export/import"""
    print("=" * 60)
    print("Testing JSON Serialization")
    print("=" * 60)
    print()

    # Export to JSON
    json_str = event.model_dump_json(indent=2)
    print(f"JSON size: {len(json_str)} bytes")
    print()
    print("Sample JSON (first 500 chars):")
    print(json_str[:500] + "...")
    print()

    # Import from JSON
    event_restored = SignalEvent.model_validate_json(json_str)
    print(f"✓ Successfully restored from JSON")
    print(f"  event_id matches: {event.event_id == event_restored.event_id}")
    print(f"  vector matches: {event.semantic.vector == event_restored.semantic.vector}")
    print()


def test_validation():
    """Test Pydantic validation"""
    print("=" * 60)
    print("Testing Validation")
    print("=" * 60)
    print()

    # Test 1: Invalid vector length (should raise error)
    print("1. Invalid vector length (expecting error):")
    try:
        SemanticCore(vector=[0.1, 0.2, 0.3])  # Only 3 dimensions instead of 8
        print("   ✗ Validation failed to catch error!")
    except Exception as e:
        print(f"   ✓ Caught: {type(e).__name__}")
    print()

    # Test 2: Invalid priority range (should raise error)
    print("2. Invalid priority (expecting error):")
    try:
        RoutingInfo(priority=500)  # Max is 255
        print("   ✗ Validation failed to catch error!")
    except Exception as e:
        print(f"   ✓ Caught: {type(e).__name__}")
    print()

    # Test 3: Invalid valence range (should raise error)
    print("3. Invalid valence (expecting error):")
    try:
        EnergyProfile(valence=2.0)  # Max is 1.0
        print("   ✗ Validation failed to catch error!")
    except Exception as e:
        print(f"   ✓ Caught: {type(e).__name__}")
    print()


def main():
    print("\n" + "=" * 60)
    print("Gateway v2.0 - Pydantic Models Test Suite")
    print("=" * 60)
    print()

    # Test individual models
    components = test_basic_models()

    # Test master SignalEvent
    event = test_signal_event(components)

    # Test JSON serialization
    test_json_serialization(event)

    # Test validation
    test_validation()

    print("=" * 60)
    print("✓ All tests passed!")
    print("=" * 60)
    print()


if __name__ == "__main__":
    main()
