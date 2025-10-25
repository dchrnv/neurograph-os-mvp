"""
NeuroGraph OS - Python Wrapper for Rust Core

This module provides a high-level Python interface to the Rust core implementation
of NeuroGraph OS, including Token V2.0 and Connection V1.0 structures.

Usage:
    from neurograph import Token, Connection, CoordinateSpace, ConnectionType

    # Create a token
    token = Token(42)
    token.set_coordinates(CoordinateSpace.L1Physical(), 10.50, 20.30, 5.20)
    token.set_active(True)

    # Create a connection
    conn = Connection(1, 2, ConnectionType.Synonym())
    conn.pull_strength = 0.70
    conn.activate()

Performance:
    The Rust implementation is typically 10-100x faster than pure Python
    for Token/Connection operations, with zero-copy serialization.
"""

from typing import Tuple, List, Optional
import neurograph_core as _core

# Re-export core classes
Token = _core.Token
Connection = _core.Connection
CoordinateSpace = _core.CoordinateSpace
EntityType = _core.EntityType
ConnectionType = _core.ConnectionType

__all__ = [
    'Token',
    'Connection',
    'CoordinateSpace',
    'EntityType',
    'ConnectionType',
    'create_example_token',
    'create_emotional_token',
    'create_semantic_connection',
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


# Version info
__version__ = "0.14.0"
__author__ = "NeuroGraph OS Team"
