#!/usr/bin/env python3
"""
Gateway v2.0 - Complete Demo

Demonstrates all Gateway v2.0 features:
- Sensor registration
- Signal encoding (all 4 encoders)
- Event creation and routing
- Sensor management
- Statistics tracking

This is the main example for Gateway v2.0.
"""

from src.gateway import SignalGateway, EncoderType
import json


def demo_text_processing():
    """Demo 1: Text signal processing"""
    print("=" * 70)
    print("Demo 1: Text Signal Processing (TEXT_TFIDF encoder)")
    print("=" * 70)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    # Send text message
    event = gateway.push_text(
        text="Hello, NeuroGraph! This is a test message.",
        priority=200,
        metadata={"source": "demo", "language": "en"},
        sequence_id="demo_conversation_001"
    )

    print(f"Text Event Created:")
    print(f"  Event ID: {event.event_id}")
    print(f"  Event Type: {event.event_type}")
    print(f"  Sensor: {event.source.sensor_id} ({event.source.sensor_type})")
    print(f"  Modality: {event.source.modality}")
    print(f"  Encoding: {event.semantic.encoding_method}")
    print(f"  8D Vector: {[f'{v:.3f}' for v in event.semantic.vector]}")
    print(f"  Priority: {event.routing.priority}")
    print(f"  Urgency: {event.energy.urgency:.2f}")
    print(f"  NeuroTick: {event.temporal.neuro_tick}")
    print(f"  Sequence: {event.temporal.sequence_id}")
    print(f"  Payload: \"{event.payload.data}\"")
    print()


def demo_system_metrics():
    """Demo 2: System metric monitoring"""
    print("=" * 70)
    print("Demo 2: System Metrics (NUMERIC_DIRECT encoder)")
    print("=" * 70)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    # Send system metrics
    metrics = [
        ("cpu_percent", 45.7),
        ("memory_percent", 67.3),
        ("disk_io", 23.1),
    ]

    events = []
    for metric_name, value in metrics:
        event = gateway.push_system(
            metric_name=metric_name,
            metric_value=value,
            priority=100
        )
        events.append(event)

    print(f"Sent {len(events)} system metrics:")
    for event in events:
        metric_data = event.payload.data
        vector = event.semantic.vector
        print(f"  {metric_data['metric']}: {metric_data['value']}")
        print(f"    Vector: {[f'{v:.3f}' for v in vector[:3]]}...")
        print(f"    NeuroTick: {event.temporal.neuro_tick}")
    print()


def demo_custom_sensor():
    """Demo 3: Custom sensor with sentiment analysis"""
    print("=" * 70)
    print("Demo 3: Custom Sensor with Sentiment Analysis")
    print("=" * 70)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    # Register custom sensor with SENTIMENT encoder
    gateway.register_sensor(
        sensor_id="custom.sentiment_analyzer",
        sensor_type="sentiment_feed",
        domain="external",
        modality="text",
        encoder_type=EncoderType.SENTIMENT_SIMPLE,
        description="Sentiment analysis for user feedback",
        default_priority=180
    )

    # Send messages with different sentiments
    messages = [
        ("I am very happy with this product!", "positive"),
        ("This is terrible and disappointing", "negative"),
        ("The system is functioning normally", "neutral"),
    ]

    print(f"Analyzing {len(messages)} messages:")
    print()

    for text, expected_sentiment in messages:
        # Push through custom sensor (using push_text but with custom sensor_id)
        event = gateway._push_signal(
            data=text,
            data_type="text",
            sensor_id="custom.sentiment_analyzer",
            priority=180
        )

        vector = event.semantic.vector
        polarity = vector[0]
        joy = vector[3]
        sadness = vector[4]

        print(f'Text: "{text}"')
        print(f'  Expected: {expected_sentiment}')
        print(f'  Polarity: {polarity:.2f} (0=negative, 1=positive)')
        print(f'  Joy: {joy:.2f}, Sadness: {sadness:.2f}')
        print(f'  Full vector: {[f"{v:.2f}" for v in vector]}')
        print()


def demo_conversation_sequence():
    """Demo 4: Multi-turn conversation tracking"""
    print("=" * 70)
    print("Demo 4: Conversation Sequence Tracking")
    print("=" * 70)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    conversation_id = "conv_demo_001"

    messages = [
        "Hello, how can I help you?",
        "I need help with the system",
        "Sure, what issue are you experiencing?",
        "The gateway is not responding",
        "Let me check the logs...",
    ]

    print(f"Conversation ID: {conversation_id}")
    print(f"Processing {len(messages)} messages:")
    print()

    events = []
    for i, text in enumerate(messages):
        event = gateway.push_text(
            text=text,
            sequence_id=conversation_id,
            priority=200,
            metadata={"turn": i + 1}
        )
        events.append(event)

    # Display conversation
    for event in events:
        turn = event.payload.metadata.get("turn", "?")
        tick = event.temporal.neuro_tick
        text = event.payload.data
        vector_summary = f"[{event.semantic.vector[0]:.2f}, {event.semantic.vector[1]:.2f}, ...]"

        print(f"  Turn {turn} (tick={tick}): \"{text}\"")
        print(f"    Vector: {vector_summary}")

    print()
    print(f"✓ Conversation tracked across {len(events)} turns")
    print()


def demo_statistics():
    """Demo 5: Gateway statistics"""
    print("=" * 70)
    print("Demo 5: Gateway Statistics")
    print("=" * 70)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    # Generate some traffic
    for i in range(10):
        gateway.push_text(f"Message {i+1}", priority=150 + i * 10)

    for i in range(5):
        gateway.push_system(f"metric_{i}", i * 10.0, priority=100)

    # Get statistics
    stats = gateway.get_stats()

    print(f"Gateway Statistics:")
    print(f"  Total events processed: {stats['total_events']}")
    print(f"  Current NeuroTick: {stats['neuro_tick']}")
    print(f"  Registered sensors: {stats['registered_sensors']}")
    print(f"  Enabled sensors: {stats['enabled_sensors']}")
    print()

    # List sensors
    sensors = gateway.list_sensors()
    print(f"Active Sensors:")
    for sensor in sensors:
        print(f"  - {sensor.sensor_id}")
        print(f"      Type: {sensor.sensor_type}")
        print(f"      Domain: {sensor.domain}")
        print(f"      Modality: {sensor.modality}")
        print(f"      Encoder: {sensor.encoder_type}")
    print()


def demo_json_export():
    """Demo 6: JSON export for persistence/API"""
    print("=" * 70)
    print("Demo 6: JSON Export/Import")
    print("=" * 70)
    print()

    gateway = SignalGateway()
    gateway.initialize()

    # Create event
    event = gateway.push_text(
        text="Export test message",
        priority=180,
        metadata={"export": True}
    )

    # Export to JSON
    json_str = event.model_dump_json(indent=2)

    print(f"Event exported to JSON:")
    print(f"  Size: {len(json_str)} bytes")
    print()
    print("JSON structure (first 800 chars):")
    print(json_str[:800] + "...")
    print()

    # Verify it's valid
    parsed = json.loads(json_str)
    print(f"✓ Valid JSON")
    print(f"  Event ID: {parsed['event_id']}")
    print(f"  Event Type: {parsed['event_type']}")
    print(f"  Vector dimensions: {len(parsed['semantic']['vector'])}")
    print()


def main():
    """Run all demos"""
    print("\n" + "=" * 70)
    print("Gateway v2.0 - Complete Feature Demo")
    print("=" * 70)
    print()

    demo_text_processing()
    demo_system_metrics()
    demo_custom_sensor()
    demo_conversation_sequence()
    demo_statistics()
    demo_json_export()

    print("=" * 70)
    print("✓ All demos completed successfully!")
    print("=" * 70)
    print()
    print("Gateway v2.0 Features Demonstrated:")
    print("  ✓ Text encoding (TF-IDF)")
    print("  ✓ Numeric encoding (direct scaling)")
    print("  ✓ Sentiment analysis")
    print("  ✓ Custom sensor registration")
    print("  ✓ Conversation tracking")
    print("  ✓ System metrics monitoring")
    print("  ✓ Statistics & sensor management")
    print("  ✓ JSON serialization")
    print()


if __name__ == "__main__":
    main()
