
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
    

"""
NeuroGraph OS - Python Wrapper for Rust Core

This module provides a high-level Python interface to the Rust core implementation
of NeuroGraph OS, including Token V2.0, Connection V1.0, and Grid V2.0 structures.

Usage:
    from neurograph import Token, Connection, Grid, CoordinateSpace, ConnectionType

    # Create a token
    token = Token(42)
    token.set_coordinates(CoordinateSpace.L1Physical(), 10.50, 20.30, 5.20)
    token.set_active(True)

    # Create a connection
    conn = Connection(1, 2, ConnectionType.Synonym())
    conn.pull_strength = 0.70
    conn.activate()

    # Create a grid and add tokens
    grid = Grid()
    grid.add(token)

    # Find neighbors
    neighbors = grid.find_neighbors(42, 0, radius=10.0, max_results=5)

Performance:
    The Rust implementation is typically 10-100x faster than pure Python
    for Token/Connection/Grid operations, with zero-copy serialization.
"""

from typing import Tuple, List, Optional
import neurograph_core as _core

# Re-export core classes
Token = _core.Token
Connection = _core.Connection
Grid = _core.Grid
GridConfig = _core.GridConfig
CoordinateSpace = _core.CoordinateSpace
EntityType = _core.EntityType
ConnectionType = _core.ConnectionType

__all__ = [
    'Token',
    'Connection',
    'Grid',
    'GridConfig',
    'CoordinateSpace',
    'EntityType',
    'ConnectionType',
    'create_example_token',
    'create_emotional_token',
    'create_semantic_connection',
    'create_grid_with_tokens',
]

# Helper functions for common patterns

def create_example_token(
    token_id: int,
    x: float = 0.0,
    y: float = 0.0,
    z: float = 0.0,
    weight: float = 1.0,
    entity_type: Optional[EntityType] = None
) -> Token:
    """
    Create a token with common defaults.

    Args:
        token_id: Unique token identifier
        x, y, z: Physical coordinates (L1Physical space)
        weight: Token weight (default 1.0)
        entity_type: Entity type (default: Concept)

    Returns:
        Configured Token instance

    Example:
        >>> token = create_example_token(42, 10.50, 20.30, 5.20)
        >>> token.is_active()
        True
    """
    token = Token(token_id)
    token.set_coordinates(CoordinateSpace.L1Physical(), x, y, z)
    token.weight = weight

    if entity_type is None:
        entity_type = EntityType.Concept()
    token.set_entity_type(entity_type)

    token.set_active(True)
    return token


def create_emotional_token(
    token_id: int,
    valence: float,
    arousal: float,
    dominance: float,
    weight: float = 1.0
) -> Token:
    """
    Create a token in emotional (VAD) space.

    Args:
        token_id: Unique token identifier
        valence: Emotional valence (-1.0 to 1.0)
        arousal: Emotional arousal (-1.0 to 1.0)
        dominance: Emotional dominance (-1.0 to 1.0)
        weight: Token weight

    Returns:
        Token positioned in L4Emotional space

    Example:
        >>> token = create_emotional_token(1, 0.80, 0.60, 0.50)
        >>> coords = token.get_coordinates(CoordinateSpace.L4Emotional())
        >>> print(f"Valence: {coords[0]:.2f}")
        Valence: 0.80
    """
    token = Token(token_id)
    token.set_coordinates(CoordinateSpace.L4Emotional(), valence, arousal, dominance)
    token.weight = weight
    token.set_entity_type(EntityType.State())
    token.set_active(True)
    return token


def create_semantic_connection(
    token_a_id: int,
    token_b_id: int,
    connection_type: ConnectionType,
    strength: float = 1.0,
    bidirectional: bool = False
) -> Connection:
    """
    Create a semantic connection between two tokens.

    Args:
        token_a_id: Source token ID
        token_b_id: Target token ID
        connection_type: Type of semantic connection
        strength: Pull strength (default 1.0)
        bidirectional: Whether connection works both ways

    Returns:
        Configured Connection instance

    Example:
        >>> conn = create_semantic_connection(
        ...     1, 2,
        ...     ConnectionType.Synonym(),
        ...     strength=0.80,
        ...     bidirectional=True
        ... )
        >>> conn.is_bidirectional()
        True
    """
    conn = Connection(token_a_id, token_b_id, connection_type)
    conn.pull_strength = strength
    conn.set_bidirectional(bidirectional)
    conn.set_active(True)
    return conn


def create_grid_with_tokens(
    num_tokens: int,
    space: int = 0,
    spread: float = 100.0,
    config: Optional[GridConfig] = None
) -> Tuple[Grid, List[Token]]:
    """
    Create a grid populated with random tokens for testing/demos.

    Args:
        num_tokens: Number of tokens to create
        space: Coordinate space to populate (0-7, default 0 = L1Physical)
        spread: Maximum coordinate value (default 100.0)
        config: Grid configuration (default: None, uses default config)

    Returns:
        Tuple of (Grid, list of created Tokens)

    Example:
        >>> grid, tokens = create_grid_with_tokens(100, space=0, spread=50.0)
        >>> len(grid)
        100
        >>> neighbors = grid.find_neighbors(tokens[0].id, 0, radius=10.0)
    """
    import random

    # Create grid
    grid = Grid(config) if config else Grid()

    # Create tokens
    tokens = []
    for i in range(num_tokens):
        token = Token(i)

        # Set random coordinates
        x = random.uniform(-spread, spread)
        y = random.uniform(-spread, spread)
        z = random.uniform(-spread, spread)

        coord_space = [
            CoordinateSpace.L1Physical(),
            CoordinateSpace.L2Sensory(),
            CoordinateSpace.L3Motor(),
            CoordinateSpace.L4Emotional(),
            CoordinateSpace.L5Cognitive(),
            CoordinateSpace.L6Social(),
            CoordinateSpace.L7Temporal(),
            CoordinateSpace.L8Abstract(),
        ][space]

        token.set_coordinates(coord_space, x, y, z)
        token.set_active(True)

        # Random field properties
        token.field_radius = random.randint(50, 150)  # 0.5 - 1.5 decoded
        token.field_strength = random.randint(100, 255)  # 0.4 - 1.0 decoded

        tokens.append(token)
        grid.add(token)

    return grid, tokens


# Version info
__version__ = "0.15.0"
__author__ = "NeuroGraph OS Team"
