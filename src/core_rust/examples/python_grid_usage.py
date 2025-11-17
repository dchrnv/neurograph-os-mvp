
    # NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
    # Copyright (C) 2024-2025 Chernov Denys

    # This program is free software: you can redistribute it and/or modify
    # it under the terms of the GNU Affero General Public License as published by
    # the Free Software Foundation, either version 3 of the License, or
    # (at your option) any later version.

    # This program is distributed in the hope that it will be useful,
    # but WITHOUT ANY WARRANTY; without even the implied warranty of
    # MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    # GNU Affero General Public License for more details.

    # You should have received a copy of the GNU Affero General Public License
    # along with this program. If not, see <https://www.gnu.org/licenses/>.
    

#!/usr/bin/env python3
"""
NeuroGraph Grid - Python Usage Examples

Demonstrates using the Rust-powered Grid V2.0 from Python.

Prerequisites:
    cd src/core_rust
    maturin develop --release --features python
"""

import sys
sys.path.insert(0, 'python')

from neurograph import (
    Token, Grid, GridConfig,
    create_example_token, create_grid_with_tokens
)


def example_basic_grid():
    """Basic grid creation and token management"""
    print("=== Basic Grid Usage ===")

    # Create grid with default config
    grid = Grid()
    print(f"Created empty grid: {grid}")
    print(f"Grid size: {len(grid)}")

    # Create and add tokens
    token1 = create_example_token(1, 0.00, 0.00, 0.00)
    token2 = create_example_token(2, 10.00, 0.00, 0.00)
    token3 = create_example_token(3, 0.00, 10.00, 0.00)

    grid.add(token1)
    grid.add(token2)
    grid.add(token3)

    print(f"Grid size after adding: {len(grid)}")

    # Retrieve token
    retrieved = grid.get(1)
    print(f"Retrieved token: {retrieved}")

    # Remove token
    removed = grid.remove(2)
    print(f"Removed token: {removed}")
    print(f"Grid size after removal: {len(grid)}")
    print()


def example_grid_config():
    """Grid with custom configuration"""
    print("=== Custom Grid Configuration ===")

    # Create custom config
    config = GridConfig(
        bucket_size=5.0,
        density_threshold=0.8,
        min_field_nodes=5
    )
    print(f"Config: {config}")

    # Create grid with custom config
    grid = Grid(config)
    print(f"Grid created with custom config")
    print()


def example_find_neighbors():
    """Finding neighbors in grid"""
    print("=== Find Neighbors ===")

    grid = Grid()

    # Add tokens in a pattern
    grid.add(create_example_token(0, 0.00, 0.00, 0.00))  # Center
    grid.add(create_example_token(1, 1.00, 0.00, 0.00))  # Close
    grid.add(create_example_token(2, 2.00, 0.00, 0.00))  # Medium
    grid.add(create_example_token(3, 5.00, 0.00, 0.00))  # Far
    grid.add(create_example_token(4, 50.00, 0.00, 0.00))  # Very far

    # Find neighbors of token 0 within radius 10
    neighbors = grid.find_neighbors(
        center_token_id=0,
        space=0,  # L1Physical
        radius=10.0,
        max_results=10
    )

    print(f"Neighbors of token 0 within radius 10.0:")
    for token_id, distance in neighbors:
        print(f"  Token {token_id}: distance = {distance:.2f}")
    print()


def example_range_query():
    """Range query around a point"""
    print("=== Range Query ===")

    # Create grid with many tokens
    grid, tokens = create_grid_with_tokens(50, space=0, spread=50.0)

    # Query around point (0, 0, 0) with radius 15
    results = grid.range_query(
        space=0,  # L1Physical
        x=0.00,
        y=0.00,
        z=0.00,
        radius=15.0
    )

    print(f"Tokens within radius 15.0 of origin:")
    print(f"  Found {len(results)} tokens")
    for token_id, distance in results[:5]:  # Show first 5
        print(f"  Token {token_id}: distance = {distance:.2f}")
    print()


def example_field_influence():
    """Calculate field influence at a point"""
    print("=== Field Influence ===")

    grid = Grid()

    # Add token with strong field at origin
    token = create_example_token(1, 0.00, 0.00, 0.00)
    token.field_radius = 200  # 2.0 decoded
    token.field_strength = 255  # 1.0 decoded
    grid.add(token)

    # Add token with weak field nearby
    token2 = create_example_token(2, 5.00, 0.00, 0.00)
    token2.field_radius = 100  # 1.0 decoded
    token2.field_strength = 128  # 0.5 decoded
    grid.add(token2)

    # Calculate influence at different points
    points = [
        (0.00, 0.00, 0.00, "Origin (on strong field)"),
        (1.00, 0.00, 0.00, "1m from strong field"),
        (5.00, 0.00, 0.00, "On weak field"),
        (10.00, 0.00, 0.00, "Far from both")
    ]

    for x, y, z, desc in points:
        influence = grid.calculate_field_influence(
            space=0,  # L1Physical
            x=x, y=y, z=z,
            radius=10.0
        )
        print(f"  {desc}: influence = {influence:.3f}")
    print()


def example_density_calculation():
    """Calculate node density"""
    print("=== Density Calculation ===")

    # Create grid with clusters
    grid = Grid()

    # Cluster 1: Dense cluster at origin
    for i in range(10):
        x = i * 0.5
        token = create_example_token(i, x, 0.00, 0.00)
        grid.add(token)

    # Cluster 2: Sparse area far away
    for i in range(10, 12):
        x = 50.00 + (i - 10) * 10.0
        token = create_example_token(i, x, 0.00, 0.00)
        grid.add(token)

    # Calculate density in different areas
    areas = [
        (0.00, 0.00, 0.00, 5.0, "Dense cluster"),
        (50.00, 0.00, 0.00, 5.0, "Sparse area"),
        (25.00, 0.00, 0.00, 5.0, "Empty space")
    ]

    for x, y, z, radius, desc in areas:
        density = grid.calculate_density(
            space=0,
            x=x, y=y, z=z,
            radius=radius
        )
        print(f"  {desc}: density = {density:.6f} tokens/unit³")
    print()


def example_multi_dimensional():
    """Tokens in multiple coordinate spaces"""
    print("=== Multi-dimensional Grid ===")

    grid = Grid()

    # Create token with coordinates in multiple spaces
    from neurograph import CoordinateSpace

    token = Token(1)
    token.set_coordinates(CoordinateSpace.L1Physical(), 10.00, 20.00, 5.00)
    token.set_coordinates(CoordinateSpace.L4Emotional(), 0.80, 0.60, 0.50)
    token.set_coordinates(CoordinateSpace.L8Abstract(), 0.70, 0.30, 0.40)
    grid.add(token)

    # Add more tokens
    for i in range(2, 6):
        t = Token(i)
        t.set_coordinates(CoordinateSpace.L1Physical(), 10.0 + i, 20.00, 5.00)
        t.set_coordinates(CoordinateSpace.L4Emotional(), 0.80, 0.60 + i * 0.05, 0.50)
        grid.add(t)

    # Find neighbors in different spaces
    print("Neighbors in L1Physical (space 0):")
    neighbors_physical = grid.find_neighbors(1, space=0, radius=10.0, max_results=5)
    for tid, dist in neighbors_physical:
        print(f"  Token {tid}: distance = {dist:.2f}")

    print("\nNeighbors in L4Emotional (space 3):")
    neighbors_emotional = grid.find_neighbors(1, space=3, radius=0.50, max_results=5)
    for tid, dist in neighbors_emotional:
        print(f"  Token {tid}: distance = {dist:.3f}")
    print()


def example_large_grid():
    """Performance with larger grid"""
    print("=== Large Grid Performance ===")
    import time

    # Create grid with 1000 tokens
    start = time.time()
    grid, tokens = create_grid_with_tokens(1000, space=0, spread=100.0)
    creation_time = time.time() - start

    print(f"Created grid with {len(grid)} tokens in {creation_time:.3f}s")
    print(f"  (~{1000/creation_time:.0f} tokens/sec)")

    # Perform neighbor searches
    start = time.time()
    for i in range(100):
        neighbors = grid.find_neighbors(
            tokens[i].id,
            space=0,
            radius=20.0,
            max_results=10
        )
    search_time = time.time() - start

    print(f"Performed 100 neighbor searches in {search_time:.3f}s")
    print(f"  (~{100/search_time:.0f} searches/sec)")
    print()


def main():
    """Run all examples"""
    print("NeuroGraph Grid - Python Usage Examples")
    print("=" * 60)
    print()

    try:
        example_basic_grid()
        example_grid_config()
        example_find_neighbors()
        example_range_query()
        example_field_influence()
        example_density_calculation()
        example_multi_dimensional()
        example_large_grid()

        print("=" * 60)
        print("All examples completed successfully!")

    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
