"""Runtime Storage - Python interface for RuntimeStorage FFI.

Provides high-level Python classes for working with tokens, connections,
grid operations and CDNA configuration in the neurograph runtime.
"""

from typing import Optional, Dict, Any, List, Tuple
import logging

logger = logging.getLogger(__name__)


class RuntimeTokenStorage:
    """Interface for runtime token storage operations.

    This class wraps FFI methods from PyRuntime to provide convenient
    Python API for token CRUD operations.

    Example:
        >>> runtime = Runtime()
        >>> tokens = RuntimeTokenStorage(runtime._core)
        >>> token_id = tokens.create(weight=1.0)
        >>> token = tokens.get(token_id)
        >>> print(f"Token {token['id']} weight: {token['weight']}")
    """

    def __init__(self, core):
        """Initialize token storage.

        Args:
            core: PyRuntime FFI instance
        """
        self._core = core

    def create(self, weight: float = 1.0, **kwargs) -> int:
        """Create a new token.

        Args:
            weight: Token weight/intensity (default: 1.0)
            **kwargs: Additional token properties

        Returns:
            Assigned token ID

        Example:
            >>> token_id = tokens.create(weight=0.8)
            >>> print(f"Created token: {token_id}")
        """
        token_dict = {"weight": weight}
        token_dict.update(kwargs)

        token_id = self._core.create_token(token_dict)
        logger.debug(f"Created token {token_id} with weight={weight}")
        return token_id

    def get(self, token_id: int) -> Optional[Dict[str, Any]]:
        """Get token by ID.

        Args:
            token_id: Token ID

        Returns:
            Token data dict or None if not found

        Example:
            >>> token = tokens.get(42)
            >>> if token:
            >>>     print(f"Weight: {token['weight']}")
        """
        return self._core.get_token(token_id)

    def update(self, token_id: int, **kwargs) -> bool:
        """Update token properties.

        Args:
            token_id: Token ID
            **kwargs: Properties to update

        Returns:
            True if successful

        Example:
            >>> success = tokens.update(42, weight=0.5)
        """
        return self._core.update_token(token_id, kwargs)

    def delete(self, token_id: int) -> bool:
        """Delete token.

        Args:
            token_id: Token ID

        Returns:
            True if deleted

        Example:
            >>> tokens.delete(42)
        """
        result = self._core.delete_token(token_id)
        if result:
            logger.debug(f"Deleted token {token_id}")
        return result

    def list(self, limit: int = 100, offset: int = 0) -> List[int]:
        """List token IDs with pagination.

        Args:
            limit: Maximum number of tokens (default: 100)
            offset: Number to skip (default: 0)

        Returns:
            List of token IDs

        Example:
            >>> # Get first page
            >>> page1 = tokens.list(limit=10, offset=0)
            >>> # Get second page
            >>> page2 = tokens.list(limit=10, offset=10)
        """
        return self._core.list_tokens(limit, offset)

    def count(self) -> int:
        """Get total number of tokens.

        Returns:
            Total token count

        Example:
            >>> total = tokens.count()
            >>> print(f"Total tokens: {total}")
        """
        return self._core.count_tokens()

    def clear(self) -> int:
        """Clear all tokens.

        Returns:
            Number of tokens removed

        Example:
            >>> removed = tokens.clear()
            >>> print(f"Cleared {removed} tokens")
        """
        count = self._core.clear_tokens()
        logger.info(f"Cleared {count} tokens")
        return count


class RuntimeConnectionStorage:
    """Interface for runtime connection storage operations.

    Manages connections (edges) between tokens in the runtime graph.

    Example:
        >>> runtime = Runtime()
        >>> connections = RuntimeConnectionStorage(runtime._core)
        >>> conn_id = connections.create(token_a=1, token_b=2)
        >>> conn = connections.get(conn_id)
    """

    def __init__(self, core):
        """Initialize connection storage.

        Args:
            core: PyRuntime FFI instance
        """
        self._core = core

    def create(self, token_a: int, token_b: int) -> int:
        """Create connection between two tokens.

        Args:
            token_a: First token ID
            token_b: Second token ID

        Returns:
            Connection ID

        Example:
            >>> conn_id = connections.create(1, 2)
        """
        conn_id = self._core.create_connection(token_a, token_b)
        logger.debug(f"Created connection {conn_id}: {token_a} <-> {token_b}")
        return conn_id

    def get(self, connection_id: int) -> Optional[Dict[str, Any]]:
        """Get connection by ID.

        Args:
            connection_id: Connection ID

        Returns:
            Connection data dict or None

        Example:
            >>> conn = connections.get(123)
            >>> if conn:
            >>>     print(f"{conn['token_a_id']} <-> {conn['token_b_id']}")
        """
        return self._core.get_connection(connection_id)

    def delete(self, connection_id: int) -> bool:
        """Delete connection.

        Args:
            connection_id: Connection ID

        Returns:
            True if deleted

        Example:
            >>> connections.delete(123)
        """
        result = self._core.delete_connection(connection_id)
        if result:
            logger.debug(f"Deleted connection {connection_id}")
        return result

    def list(self, limit: int = 100, offset: int = 0) -> List[int]:
        """List connection IDs with pagination.

        Args:
            limit: Maximum number (default: 100)
            offset: Number to skip (default: 0)

        Returns:
            List of connection IDs

        Example:
            >>> page = connections.list(limit=50)
        """
        return self._core.list_connections(limit, offset)

    def count(self) -> int:
        """Get total number of connections.

        Returns:
            Total connection count

        Example:
            >>> total = connections.count()
        """
        return self._core.count_connections()


class RuntimeGridStorage:
    """Interface for runtime spatial grid operations.

    Provides spatial indexing and neighbor queries for tokens.

    Example:
        >>> runtime = Runtime()
        >>> grid = RuntimeGridStorage(runtime._core)
        >>> info = grid.info()
        >>> neighbors = grid.find_neighbors(token_id=1, radius=5.0)
    """

    def __init__(self, core):
        """Initialize grid storage.

        Args:
            core: PyRuntime FFI instance
        """
        self._core = core

    def info(self) -> Dict[str, Any]:
        """Get grid information.

        Returns:
            Dict with grid statistics (count, bounds, etc.)

        Example:
            >>> info = grid.info()
            >>> print(f"Tokens in grid: {info['count']}")
        """
        return self._core.get_grid_info()

    def find_neighbors(
        self,
        token_id: int,
        radius: float,
    ) -> List[Tuple[int, float]]:
        """Find neighbors of a token within radius.

        Args:
            token_id: Center token ID
            radius: Search radius

        Returns:
            List of (token_id, distance) tuples, sorted by distance

        Example:
            >>> # Find tokens within radius 10.0
            >>> neighbors = grid.find_neighbors(token_id=42, radius=10.0)
            >>> for neighbor_id, distance in neighbors:
            >>>     print(f"Token {neighbor_id} at distance {distance:.2f}")
        """
        return self._core.find_neighbors(token_id, radius)

    def range_query(
        self,
        center: Tuple[float, float, float],
        radius: float,
    ) -> List[Tuple[int, float]]:
        """Query tokens in spatial range around a point.

        Args:
            center: Center coordinates (x, y, z)
            radius: Search radius

        Returns:
            List of (token_id, distance) tuples

        Example:
            >>> # Find all tokens near origin
            >>> results = grid.range_query(center=(0, 0, 0), radius=5.0)
            >>> print(f"Found {len(results)} tokens near origin")
        """
        # Convert tuple to list for FFI
        center_list = [center[0], center[1], center[2]]
        return self._core.range_query(center_list, radius)


class RuntimeCDNAStorage:
    """Interface for runtime CDNA (Constitution DNA) configuration.

    Manages system-wide configuration parameters including dimension
    scales, profiles, and feature flags.

    Example:
        >>> runtime = Runtime()
        >>> cdna = RuntimeCDNAStorage(runtime._core)
        >>> config = cdna.get_config()
        >>> cdna.update_scales([1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5])
    """

    def __init__(self, core):
        """Initialize CDNA storage.

        Args:
            core: PyRuntime FFI instance
        """
        self._core = core

    def get_config(self) -> Dict[str, Any]:
        """Get current CDNA configuration.

        Returns:
            Dict with profile_id, flags, etc.

        Example:
            >>> config = cdna.get_config()
            >>> print(f"Profile: {config['profile_id']}")
        """
        return self._core.get_cdna_config()

    def update_scales(self, scales: List[float]) -> bool:
        """Update dimension scales (L1-L8).

        Args:
            scales: List of 8 scale values, one per dimension

        Returns:
            True if successful

        Raises:
            ValueError: If scales are invalid (must be positive)

        Example:
            >>> # Set equal scales for all dimensions
            >>> cdna.update_scales([1.0] * 8)
            >>> # Set custom scales
            >>> cdna.update_scales([1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5])
        """
        if len(scales) != 8:
            raise ValueError(f"Expected 8 scales, got {len(scales)}")

        # Convert to array for FFI
        scales_array = scales
        result = self._core.update_cdna_scales(scales_array)

        if result:
            logger.debug(f"Updated CDNA scales: {scales}")
        return result

    def get_profile(self) -> int:
        """Get current profile ID.

        Returns:
            Profile ID (0=Default, 1=Explorer, etc.)

        Example:
            >>> profile_id = cdna.get_profile()
        """
        return self._core.get_cdna_profile()

    def set_profile(self, profile_id: int) -> None:
        """Set profile ID.

        Args:
            profile_id: New profile ID

        Example:
            >>> cdna.set_profile(1)  # Set Explorer profile
        """
        self._core.set_cdna_profile(profile_id)
        logger.debug(f"Set CDNA profile: {profile_id}")

    def get_flags(self) -> int:
        """Get current feature flags.

        Returns:
            Flags value (bitmask)

        Example:
            >>> flags = cdna.get_flags()
        """
        return self._core.get_cdna_flags()

    def set_flags(self, flags: int) -> None:
        """Set feature flags.

        Args:
            flags: New flags value

        Example:
            >>> cdna.set_flags(0xFF)
        """
        self._core.set_cdna_flags(flags)
        logger.debug(f"Set CDNA flags: 0x{flags:X}")

    def validate(self) -> bool:
        """Validate CDNA configuration.

        Returns:
            True if valid

        Example:
            >>> if not cdna.validate():
            >>>     print("Warning: Invalid CDNA configuration!")
        """
        return self._core.validate_cdna()
