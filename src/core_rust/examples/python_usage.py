#!/usr/bin/env python3
"""
NeuroGraph Core - Python Usage Examples

This demonstrates using the Rust-powered Token and Connection structures from Python.

Prerequisites:
    pip install maturin
    cd src/core_rust
    maturin develop --release --features python
"""

import sys
sys.path.insert(0, 'python')

from neurograph import (
    Token, Connection,
    CoordinateSpace, EntityType, ConnectionType,
    create_example_token, create_emotional_token, create_semantic_connection
)


def example_basic_token():
    """Basic token creation and manipulation"""
    print("=== Basic Token Usage ===")

    # Create a token
    token = Token(42)
    print(f"Created: {token}")

    # Set coordinates in physical space
    token.set_coordinates(CoordinateSpace.L1Physical(), 10.50, 20.30, 5.20)

    # Set properties
    token.weight = 2.50
    token.field_radius = 10
    token.field_strength = 128
    token.set_entity_type(EntityType.Concept())
    token.set_active(True)

    # Get coordinates
    x, y, z = token.get_coordinates(CoordinateSpace.L1Physical())
    print(f"Physical position: ({x:.2f}, {y:.2f}, {z:.2f})")
    print(f"Weight: {token.weight:.2f}")
    print(f"Active: {token.is_active()}")
    print(f"Entity type: {token.get_entity_type()}")
    print()


def example_multidimensional_token():
    """Token with coordinates in multiple semantic spaces"""
    print("=== Multi-dimensional Token ===")

    token = Token(100)

    # Set coordinates across multiple spaces
    token.set_coordinates(CoordinateSpace.L1Physical(), 10.50, 20.30, 5.20)
    token.set_coordinates(CoordinateSpace.L4Emotional(), 0.80, 0.60, 0.50)  # VAD
    token.set_coordinates(CoordinateSpace.L5Cognitive(), 0.70, 0.90, 0.85)
    token.set_coordinates(CoordinateSpace.L8Abstract(), 0.60, 0.40, 0.30)

    # Display all coordinates
    print("All coordinates:")
    for space, coords in token.all_coordinates():
        print(f"  {space}: ({coords[0]:.2f}, {coords[1]:.2f}, {coords[2]:.2f})")
    print()


def example_token_distance():
    """Calculate distance between tokens in semantic space"""
    print("=== Token Distance ===")

    # Create two tokens
    token1 = create_example_token(1, 10.00, 20.00, 5.00)
    token2 = create_example_token(2, 15.00, 25.00, 8.00)

    # Calculate physical distance
    distance = token1.distance_to(token2, CoordinateSpace.L1Physical())
    print(f"Physical distance: {distance:.2f}")

    # Add emotional coordinates
    token1.set_coordinates(CoordinateSpace.L4Emotional(), 0.80, 0.60, 0.50)
    token2.set_coordinates(CoordinateSpace.L4Emotional(), -0.50, 0.30, 0.40)

    # Calculate emotional distance
    emotional_dist = token1.distance_to(token2, CoordinateSpace.L4Emotional())
    print(f"Emotional distance: {emotional_dist:.2f}")
    print()


def example_token_serialization():
    """Serialize and deserialize tokens"""
    print("=== Token Serialization ===")

    # Create and configure token
    original = create_example_token(42, 10.50, 20.30, 5.20, weight=2.50)
    original.set_entity_type(EntityType.Concept())

    # Serialize to bytes
    data = original.to_bytes()
    print(f"Serialized size: {len(data)} bytes (expected: {Token.size()})")

    # Deserialize
    restored = Token.from_bytes(data)
    print(f"Original:  {original}")
    print(f"Restored:  {restored}")

    # Verify coordinates match
    orig_coords = original.get_coordinates(CoordinateSpace.L1Physical())
    rest_coords = restored.get_coordinates(CoordinateSpace.L1Physical())
    print(f"Coordinates match: {orig_coords == rest_coords}")
    print()


def example_basic_connection():
    """Basic connection creation and usage"""
    print("=== Basic Connection ===")

    # Create connection
    conn = Connection(1, 2, ConnectionType.Synonym())
    print(f"Created: {conn}")

    # Set properties
    conn.pull_strength = 0.70  # Attraction
    conn.preferred_distance = 1.50
    conn.rigidity = 0.80
    conn.set_bidirectional(True)
    conn.set_active(True)

    # Activate several times
    for _ in range(5):
        conn.activate()

    print(f"Type: {conn.get_connection_type()}")
    print(f"Pull strength: {conn.pull_strength:.2f}")
    print(f"Rigidity: {conn.rigidity:.2f}")
    print(f"Activations: {conn.activation_count}")
    print(f"Bidirectional: {conn.is_bidirectional()}")
    print()


def example_connection_types():
    """Different connection types"""
    print("=== Connection Types ===")

    connections = [
        ("Semantic", Connection(1, 2, ConnectionType.Synonym())),
        ("Causal", Connection(3, 4, ConnectionType.Cause())),
        ("Temporal", Connection(5, 6, ConnectionType.Before())),
        ("Spatial", Connection(7, 8, ConnectionType.Near())),
        ("Logical", Connection(9, 10, ConnectionType.Implies())),
        ("Emotional", Connection(11, 12, ConnectionType.Desires())),
    ]

    for category, conn in connections:
        print(f"{category:12} -> {conn}")
    print()


def example_connection_force():
    """Physical force model"""
    print("=== Physical Force Model ===")

    conn = create_semantic_connection(1, 2, ConnectionType.Related(), strength=0.70)
    conn.preferred_distance = 2.00
    conn.rigidity = 0.80

    # Calculate forces at different distances
    print("Distance  | Force    | Effect")
    print("----------|----------|--------")
    for distance in [0.50, 1.00, 2.00, 3.00, 4.00]:
        force = conn.calculate_force(distance)
        effect = "Pull" if force > 0 else "Push" if force < 0 else "Equilibrium"
        print(f"{distance:8.2f}  | {force:8.3f} | {effect}")
    print()


def example_connection_levels():
    """Selective activation across levels"""
    print("=== Selective Activation ===")

    conn = Connection(1, 2, ConnectionType.Related())

    # Activate specific levels (L1, L4, L8)
    conn.set_level_active(0, True)  # L1
    conn.set_level_active(3, True)  # L4
    conn.set_level_active(7, True)  # L8

    # Check active levels
    active_levels = conn.get_active_levels()
    print(f"Active levels: L{', L'.join(str(l+1) for l in active_levels)}")

    # Check individual levels
    for level in range(8):
        status = "active" if conn.is_level_active(level) else "inactive"
        print(f"  L{level+1}: {status}")
    print()


def example_connection_serialization():
    """Serialize and deserialize connections"""
    print("=== Connection Serialization ===")

    # Create and configure connection
    original = create_semantic_connection(42, 100, ConnectionType.Cause(), strength=0.85)
    original.rigidity = 0.75
    original.preferred_distance = 2.50
    original.activate()
    original.activate()

    # Serialize
    data = original.to_bytes()
    print(f"Serialized size: {len(data)} bytes (expected: {Connection.size()})")

    # Deserialize
    restored = Connection.from_bytes(data)
    print(f"Original:  {original}")
    print(f"Restored:  {restored}")
    print(f"Activations match: {original.activation_count == restored.activation_count}")
    print()


def example_helper_functions():
    """Using convenience helper functions"""
    print("=== Helper Functions ===")

    # Create emotional token
    emotional = create_emotional_token(1, valence=0.80, arousal=0.60, dominance=0.50)
    print(f"Emotional token: {emotional}")

    # Get emotional coordinates
    v, a, d = emotional.get_coordinates(CoordinateSpace.L4Emotional())
    print(f"  VAD: ({v:.2f}, {a:.2f}, {d:.2f})")

    # Create semantic connection
    semantic = create_semantic_connection(
        1, 2,
        ConnectionType.Hypernym(),
        strength=0.90,
        bidirectional=False
    )
    print(f"Semantic connection: {semantic}")
    print()


def main():
    """Run all examples"""
    print("NeuroGraph Core - Python Usage Examples")
    print("=" * 50)
    print()

    try:
        example_basic_token()
        example_multidimensional_token()
        example_token_distance()
        example_token_serialization()
        example_basic_connection()
        example_connection_types()
        example_connection_force()
        example_connection_levels()
        example_connection_serialization()
        example_helper_functions()

        print("=" * 50)
        print("All examples completed successfully!")

    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
