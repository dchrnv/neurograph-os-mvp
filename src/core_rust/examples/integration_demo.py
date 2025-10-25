#!/usr/bin/env python3
"""
Integration Demo: Token + Connection + Grid

Demonstrates how all three core components work together in NeuroGraph OS.

Scenario: Building a semantic knowledge graph with spatial positioning
- Tokens represent concepts in multi-dimensional space
- Connections define semantic relationships
- Grid enables efficient spatial queries and field calculations
"""

import random
from neurograph import (
    Token, Connection, Grid, GridConfig,
    CoordinateSpace, EntityType, ConnectionType,
    create_emotional_token
)


def print_section(title):
    """Print a formatted section header."""
    print(f"\n{'=' * 70}")
    print(f"  {title}")
    print(f"{'=' * 70}\n")


def demo_1_semantic_network():
    """Demo 1: Building a semantic network of concepts."""
    print_section("DEMO 1: Semantic Network with Spatial Positioning")

    # Create grid for spatial indexing
    config = GridConfig()
    config.bucket_size = 5.0  # 5-unit buckets
    grid = Grid(config)

    # Define concepts with their semantic positions
    concepts = {
        "dog": (0.0, 0.0, 0.0, EntityType.Concept()),
        "cat": (3.0, 2.0, 0.0, EntityType.Concept()),
        "animal": (1.5, 5.0, 0.0, EntityType.Concept()),
        "pet": (1.5, 1.0, 0.0, EntityType.Concept()),
        "mammal": (1.5, 3.5, 0.0, EntityType.Concept()),
    }

    # Create tokens and add to grid
    tokens = {}
    print("Creating concept tokens:")
    for i, (name, (x, y, z, entity_type)) in enumerate(concepts.items(), start=1):
        token = Token(i)
        token.set_coordinates(CoordinateSpace.L8Abstract(), x, y, z)
        token.set_entity_type(entity_type)
        token.weight = 1.0
        token.set_active(True)

        grid.add(token)
        tokens[name] = token

        print(f"  [{i}] {name:8s} at ({x:4.1f}, {y:4.1f}, {z:4.1f}) - weight: {token.weight:.2f}")

    print(f"\nGrid contains {len(grid)} tokens")

    # Create semantic connections
    print("\nCreating semantic relationships:")
    connections = []

    # Hypernym relationships (is-a)
    hypernym_pairs = [
        ("dog", "animal", 0.90),
        ("cat", "animal", 0.90),
        ("dog", "mammal", 0.95),
        ("cat", "mammal", 0.95),
        ("dog", "pet", 0.85),
        ("cat", "pet", 0.85),
        ("mammal", "animal", 0.95),
        ("pet", "animal", 0.80),
    ]

    for source, target, strength in hypernym_pairs:
        conn = Connection(
            tokens[source].id,
            tokens[target].id,
            ConnectionType.Hypernym()
        )
        conn.pull_strength = int(strength * 255)  # Convert to u8
        conn.rigidity = 200  # 0.80 * 255
        conn.set_active(True)
        conn.set_bidirectional(False)  # Directed edge

        connections.append(conn)
        print(f"  {source:8s} --[Hypernym]-> {target:8s} (strength: {strength:.2f})")

    # Similar relationships
    similar_pairs = [
        ("dog", "cat", 0.70),
    ]

    for a, b, strength in similar_pairs:
        conn = Connection(
            tokens[a].id,
            tokens[b].id,
            ConnectionType.Similar()
        )
        conn.pull_strength = int(strength * 255)
        conn.rigidity = 150  # 0.60 * 255
        conn.set_active(True)
        conn.set_bidirectional(True)

        connections.append(conn)
        print(f"  {a:8s} <-[Similar]-> {b:8s} (strength: {strength:.2f})")

    print(f"\nCreated {len(connections)} semantic connections")

    # Spatial queries using Grid
    print("\nSpatial Queries in Abstract Space (L8):")

    # Find neighbors of "dog"
    dog_id = tokens["dog"].id
    neighbors = grid.find_neighbors(
        center_token_id=dog_id,
        space=7,  # L8Abstract (0-indexed)
        radius=5.0,
        max_results=10
    )

    print(f"\n  Neighbors of 'dog' within radius 5.0:")
    concept_names = {v.id: k for k, v in tokens.items()}
    for token_id, distance in neighbors:
        if token_id != dog_id:  # Skip self
            name = concept_names.get(token_id, f"Token{token_id}")
            print(f"    - {name:8s}: distance = {distance:.2f}")

    # Range query - find all concepts near "animal"
    animal_coords = tokens["animal"].get_coordinates(CoordinateSpace.L8Abstract())
    nearby = grid.range_query(
        space=7,  # L8Abstract
        x=animal_coords[0],
        y=animal_coords[1],
        z=animal_coords[2],
        radius=3.0
    )

    print(f"\n  Concepts within 3.0 units of 'animal':")
    for token_id, distance in nearby:
        name = concept_names.get(token_id, f"Token{token_id}")
        print(f"    - {name:8s}: distance = {distance:.2f}")

    print("\n✓ Demo 1 complete")
    return tokens, connections, grid


def demo_2_emotional_landscape():
    """Demo 2: Emotional tokens with field influence."""
    print_section("DEMO 2: Emotional Landscape with Field Physics")

    # Create grid
    grid = Grid()

    # Create emotional tokens (VAD model)
    emotions = {
        "joy": (0.8, 0.7, 0.6, 1.5, 0.8),
        "sadness": (-0.7, 0.3, 0.3, 1.2, 0.6),
        "anger": (-0.5, 0.8, 0.7, 1.0, 0.7),
        "fear": (-0.6, 0.7, 0.2, 1.3, 0.5),
        "calm": (0.3, 0.2, 0.6, 2.0, 0.4),
    }

    tokens = {}
    print("Creating emotional tokens with fields:")

    for i, (name, (v, a, d, field_r, field_s)) in enumerate(emotions.items(), start=1):
        token = create_emotional_token(i, v, a, d)
        token.field_radius = int(field_r * 100)  # Encode as u8 (×100)
        token.field_strength = int(field_s * 255)  # Encode as u8
        token.set_active(True)

        grid.add(token)
        tokens[name] = token

        print(f"  [{i}] {name:8s} VAD=({v:5.2f}, {a:5.2f}, {d:5.2f}) "
              f"field_radius={field_r:.2f} field_strength={field_s:.2f}")

    # Calculate field influence at different points
    print("\nField Influence Analysis:")

    test_points = [
        ("Near joy", 0.8, 0.7, 0.6),
        ("Near sadness", -0.7, 0.3, 0.3),
        ("Center", 0.0, 0.0, 0.0),
        ("High arousal", 0.0, 0.8, 0.5),
    ]

    for name, x, y, z in test_points:
        influence = grid.calculate_field_influence(
            space=3,  # L4Emotional (0-indexed)
            x=x, y=y, z=z,
            radius=3.0  # Search radius
        )

        # Also check density
        density = grid.calculate_density(
            space=3,
            x=x, y=y, z=z,
            radius=1.0
        )

        print(f"  {name:15s} ({x:5.2f}, {y:5.2f}, {z:5.2f}): "
              f"influence={influence:.3f}, density={density:.4f}")

    # Find emotionally similar states
    print("\nEmotionally Similar States (neighbors of 'joy'):")
    joy_neighbors = grid.find_neighbors(
        center_token_id=tokens["joy"].id,
        space=3,  # L4Emotional
        radius=2.0,
        max_results=5
    )

    emotion_names = {v.id: k for k, v in tokens.items()}
    for token_id, distance in joy_neighbors:
        if token_id != tokens["joy"].id:
            name = emotion_names.get(token_id, f"Emotion{token_id}")
            print(f"    - {name:8s}: emotional distance = {distance:.3f}")

    print("\n✓ Demo 2 complete")
    return tokens, grid


def demo_3_spatial_network():
    """Demo 3: Physical network with distance-based connections."""
    print_section("DEMO 3: Spatial Network with Dynamic Forces")

    # Create grid
    config = GridConfig()
    config.bucket_size = 10.0
    grid = Grid(config)

    # Create tokens in physical space (L1)
    print("Creating spatial nodes:")
    nodes = []
    for i in range(1, 11):
        token = Token(i)

        # Random position in 3D space
        x = random.uniform(-20.0, 20.0)
        y = random.uniform(-20.0, 20.0)
        z = random.uniform(-5.0, 5.0)

        token.set_coordinates(CoordinateSpace.L1Physical(), x, y, z)
        token.set_entity_type(EntityType.Object())
        token.weight = random.uniform(0.5, 2.0)
        token.set_active(True)

        grid.add(token)
        nodes.append(token)

        print(f"  Node {i:2d}: ({x:6.2f}, {y:6.2f}, {z:6.2f}) weight={token.weight:.2f}")

    # Create connections based on proximity
    print("\nCreating proximity-based connections:")
    connections = []

    for i, token_a in enumerate(nodes):
        # Find nearby nodes
        neighbors = grid.find_neighbors(
            center_token_id=token_a.id,
            space=0,  # L1Physical
            radius=15.0,
            max_results=3
        )

        for neighbor_id, distance in neighbors:
            if neighbor_id != token_a.id:
                # Create proximity connection
                conn = Connection(
                    token_a.id,
                    neighbor_id,
                    ConnectionType.Proximity()
                )

                # Configure force model
                conn.preferred_distance = int(distance * 100)  # Current distance as preferred
                conn.pull_strength = 127  # 0.5 normalized
                conn.rigidity = 178  # 0.7 * 255
                conn.set_active(True)

                connections.append(conn)

                print(f"  Node {token_a.id} <-> Node {neighbor_id} "
                      f"(distance={distance:.2f}, preferred={distance:.2f})")

    print(f"\nCreated {len(connections)} proximity connections")

    # Calculate forces
    print("\nForce Analysis:")

    # Simulate slight displacement and calculate forces
    test_distances = [5.0, 10.0, 15.0, 20.0]
    sample_conn = connections[0] if connections else None

    if sample_conn:
        preferred = sample_conn.preferred_distance / 100.0
        print(f"\n  Sample connection (preferred distance: {preferred:.2f}m):")

        for d in test_distances:
            force = sample_conn.calculate_force(d)
            force_decoded = force / 255.0  # Decode from u8

            direction = "pull" if force_decoded > 0 else "push" if force_decoded < 0 else "equilibrium"
            print(f"    At {d:5.2f}m: force = {force_decoded:+.3f} ({direction})")

    # Density analysis
    print("\nDensity Analysis:")

    # Find center of mass
    total_x = sum(t.get_coordinates(CoordinateSpace.L1Physical())[0] for t in nodes)
    total_y = sum(t.get_coordinates(CoordinateSpace.L1Physical())[1] for t in nodes)
    total_z = sum(t.get_coordinates(CoordinateSpace.L1Physical())[2] for t in nodes)

    center_x = total_x / len(nodes)
    center_y = total_y / len(nodes)
    center_z = total_z / len(nodes)

    print(f"  Center of mass: ({center_x:.2f}, {center_y:.2f}, {center_z:.2f})")

    for radius in [5.0, 10.0, 15.0, 20.0]:
        density = grid.calculate_density(
            space=0,  # L1Physical
            x=center_x, y=center_y, z=center_z,
            radius=radius
        )
        print(f"    Density within {radius:5.2f}m radius: {density:.6f} nodes/m³")

    print("\n✓ Demo 3 complete")
    return nodes, connections, grid


def demo_4_multi_dimensional():
    """Demo 4: Multi-dimensional token with relationships across spaces."""
    print_section("DEMO 4: Multi-Dimensional Positioning")

    # Create grid
    grid = Grid()

    # Create a token that exists in multiple spaces
    token = Token(1)

    # Physical position
    token.set_coordinates(CoordinateSpace.L1Physical(), 10.0, 20.0, 5.0)

    # Emotional state (VAD)
    token.set_coordinates(CoordinateSpace.L4Emotional(), 0.6, 0.5, 0.7)

    # Abstract semantic position
    token.set_coordinates(CoordinateSpace.L8Abstract(), 2.0, 3.0, 1.5)

    # Temporal position
    token.set_coordinates(CoordinateSpace.L7Temporal(), 0.0, 10.0, 1.0)

    token.set_entity_type(EntityType.Agent())
    token.weight = 1.5
    token.set_active(True)

    grid.add(token)

    print("Multi-dimensional token created:")
    print(f"  ID: {token.id}")
    print(f"  Entity Type: Agent")
    print(f"  Weight: {token.weight:.2f}")

    # Display all coordinates
    spaces = [
        (CoordinateSpace.L1Physical(), 0, "L1 Physical"),
        (CoordinateSpace.L4Emotional(), 3, "L4 Emotional (VAD)"),
        (CoordinateSpace.L7Temporal(), 6, "L7 Temporal"),
        (CoordinateSpace.L8Abstract(), 7, "L8 Abstract"),
    ]

    print("\n  Coordinates across spaces:")
    for space_enum, space_idx, name in spaces:
        coords = token.get_coordinates(space_enum)
        print(f"    {name:20s}: ({coords[0]:6.2f}, {coords[1]:6.2f}, {coords[2]:6.2f})")

    # Create companion tokens in different spaces
    print("\nCreating companion tokens:")
    companions = []

    # Physical neighbor
    t2 = Token(2)
    t2.set_coordinates(CoordinateSpace.L1Physical(), 12.0, 21.0, 5.5)
    t2.set_entity_type(EntityType.Object())
    grid.add(t2)
    companions.append((t2, "physical neighbor"))

    # Emotional neighbor
    t3 = Token(3)
    t3.set_coordinates(CoordinateSpace.L4Emotional(), 0.65, 0.48, 0.72)
    t3.set_entity_type(EntityType.Event())
    grid.add(t3)
    companions.append((t3, "emotional neighbor"))

    # Abstract neighbor
    t4 = Token(4)
    t4.set_coordinates(CoordinateSpace.L8Abstract(), 2.1, 2.9, 1.6)
    t4.set_entity_type(EntityType.Concept())
    grid.add(t4)
    companions.append((t4, "semantic neighbor"))

    for comp_token, description in companions:
        print(f"  Token {comp_token.id}: {description}")

    # Query neighbors in each space
    print("\nNeighbor Analysis Across Dimensions:")

    for space_enum, space_idx, name in spaces:
        neighbors = grid.find_neighbors(
            center_token_id=token.id,
            space=space_idx,
            radius=5.0,
            max_results=5
        )

        print(f"\n  {name}:")
        if len(neighbors) > 1:  # More than just self
            for neighbor_id, distance in neighbors:
                if neighbor_id != token.id:
                    print(f"    - Token {neighbor_id}: distance = {distance:.3f}")
        else:
            print(f"    - No neighbors within radius 5.0")

    print("\n✓ Demo 4 complete")
    return token, companions, grid


def main():
    """Run all integration demos."""
    print("\n" + "=" * 70)
    print("  NeuroGraph OS - Integration Demo")
    print("  Token + Connection + Grid Working Together")
    print("=" * 70)

    # Run demos
    demo_1_semantic_network()
    demo_2_emotional_landscape()
    demo_3_spatial_network()
    demo_4_multi_dimensional()

    # Summary
    print("\n" + "=" * 70)
    print("  Summary: All Integration Demos Complete")
    print("=" * 70)
    print("\nDemonstrated capabilities:")
    print("  ✓ Token creation with multi-dimensional coordinates")
    print("  ✓ Connection creation with semantic types and force models")
    print("  ✓ Grid spatial indexing for fast neighbor queries")
    print("  ✓ Field influence and density calculations")
    print("  ✓ Multi-dimensional positioning (8 coordinate spaces)")
    print("  ✓ Integration of all three core components")
    print("\nNeuroGraph OS v0.15.0 - Production Ready")
    print("=" * 70 + "\n")


if __name__ == "__main__":
    main()
