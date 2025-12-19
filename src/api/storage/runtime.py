
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
Runtime Storage Implementation

Integration with neurograph.Runtime v0.50.0 for persistent storage
through the Rust core RuntimeStorage.
"""

from typing import List, Optional, Dict, Tuple, Any
from . import TokenStorageInterface, GridStorageInterface, CDNAStorageInterface
import sys
from pathlib import Path
import logging

# Add src/core to path
src_path = Path(__file__).parent.parent.parent
sys.path.insert(0, str(src_path))

from core.token.token_v2 import Token

logger = logging.getLogger(__name__)


class RuntimeTokenStorage(TokenStorageInterface):
    """
    Runtime-based token storage (v0.51.0).

    Integrates with neurograph.Runtime v0.50.0 for persistent storage
    through Rust RuntimeStorage with Arc<RwLock<T>> thread-safe architecture.
    """

    def __init__(self, runtime=None):
        """
        Initialize RuntimeTokenStorage.

        Args:
            runtime: neurograph.Runtime instance. If None, creates new instance.
        """
        if runtime is None:
            try:
                from neurograph import Runtime, Config
                config = Config(grid_size=1000, dimensions=50)
                runtime = Runtime(config)
                logger.info("Created new Runtime instance for TokenStorage")
            except ImportError as e:
                raise ImportError(
                    "neurograph package not found. "
                    "Please build with: cd src/core_rust && maturin develop --release --features python-bindings"
                ) from e

        self._runtime = runtime
        logger.info("RuntimeTokenStorage initialized with Runtime v0.50.0")

    def get(self, token_id: int) -> Optional[Token]:
        """Get token by ID from RuntimeStorage."""
        token_dict = self._runtime.tokens.get(token_id)

        if token_dict is None:
            return None

        # Convert dict to Token object
        return Token(
            id=token_dict['id'],
            weight=token_dict['weight'],
            coordinates=token_dict.get('coordinates', [[0.0, 0.0, 0.0]] * 8)
        )

    def create(self, token_data: Dict[str, Any]) -> Token:
        """Create new token in RuntimeStorage."""
        weight = token_data.get('weight', 1.0)

        # Create token in Rust runtime
        token_id = self._runtime.tokens.create(weight=weight)

        # Get the created token
        return self.get(token_id)

    def update(self, token_id: int, token_data: Dict[str, Any]) -> Optional[Token]:
        """Update existing token in RuntimeStorage."""
        # Check if token exists
        if self.get(token_id) is None:
            return None

        # Update weight if provided
        if 'weight' in token_data:
            self._runtime.tokens.update(token_id, weight=token_data['weight'])

        # Return updated token
        return self.get(token_id)

    def delete(self, token_id: int) -> bool:
        """Delete token from RuntimeStorage."""
        return self._runtime.tokens.delete(token_id)

    def list(self, limit: int = 100, offset: int = 0) -> List[Token]:
        """List tokens with pagination from RuntimeStorage."""
        # Get all token IDs
        all_tokens = self._runtime.tokens.list()

        # Apply pagination
        paginated = all_tokens[offset:offset + limit]

        # Convert to Token objects
        result = []
        for token_dict in paginated:
            token = Token(
                id=token_dict['id'],
                weight=token_dict['weight'],
                coordinates=token_dict.get('coordinates', [[0.0, 0.0, 0.0]] * 8)
            )
            result.append(token)

        return result

    def clear(self) -> int:
        """Clear all tokens from RuntimeStorage."""
        count = self._runtime.tokens.count()
        self._runtime.tokens.clear()
        return count

    def count(self) -> int:
        """Get total count of tokens in RuntimeStorage."""
        return self._runtime.tokens.count()


class RuntimeGridStorage(GridStorageInterface):
    """
    Runtime-based grid storage (v0.51.0).

    Uses Grid from neurograph.Runtime v0.50.0 for spatial queries.
    Note: RuntimeStorage has a single global Grid, so grid_id is ignored.
    """

    def __init__(self, runtime=None):
        """
        Initialize RuntimeGridStorage.

        Args:
            runtime: neurograph.Runtime instance. If None, creates new instance.
        """
        if runtime is None:
            try:
                from neurograph import Runtime, Config
                config = Config(grid_size=1000, dimensions=50)
                runtime = Runtime(config)
                logger.info("Created new Runtime instance for GridStorage")
            except ImportError as e:
                raise ImportError(
                    "neurograph package not found. "
                    "Please build with: cd src/core_rust && maturin develop --release --features python-bindings"
                ) from e

        self._runtime = runtime
        logger.info("RuntimeGridStorage initialized with Runtime v0.50.0")

    def create_grid(self, config: Optional[Dict[str, Any]] = None) -> int:
        """Create grid (no-op, single global grid). Returns grid_id=0."""
        logger.info("Grid already exists in RuntimeStorage (single global grid)")
        return 0

    def get_grid(self, grid_id: int) -> Optional[Any]:
        """Get grid info from RuntimeStorage."""
        return self._runtime.grid.info()

    def delete_grid(self, grid_id: int) -> bool:
        """Delete grid (no-op, cannot delete global grid)."""
        logger.warning("Cannot delete global grid in RuntimeStorage")
        return False

    def list_grids(self) -> List[int]:
        """List grids. Returns [0] for single global grid."""
        return [0]

    def add_token(self, grid_id: int, token: Token) -> bool:
        """Add token to grid (handled automatically by RuntimeStorage)."""
        logger.info(f"Token {token.token_id} automatically indexed in grid")
        return True

    def remove_token(self, grid_id: int, token_id: int) -> bool:
        """Remove token from grid (handled by token deletion)."""
        logger.info(f"Token {token_id} removal from grid handled by delete")
        return True

    def find_neighbors(
        self, grid_id: int, token_id: int, space: int, radius: float, max_results: int
    ) -> List[Tuple[int, float]]:
        """Find neighbors of token in grid."""
        neighbors = self._runtime.grid.find_neighbors(
            token_id=token_id,
            radius=radius
        )

        # neighbors is list of dicts: [{'token_id': int, 'distance': float}, ...]
        result = [(n['token_id'], n['distance']) for n in neighbors[:max_results]]
        return result

    def range_query(
        self, grid_id: int, space: int, x: float, y: float, z: float, radius: float
    ) -> List[Tuple[int, float]]:
        """Find all tokens within radius of point."""
        results = self._runtime.grid.range_query(
            center=[x, y, z],
            radius=radius
        )

        # results is list of dicts: [{'token_id': int, 'distance': float}, ...]
        return [(r['token_id'], r['distance']) for r in results]

    def calculate_field_influence(
        self, grid_id: int, space: int, x: float, y: float, z: float, radius: float
    ) -> float:
        """Calculate field influence at point (not yet in FFI)."""
        # TODO: Add to FFI in future version
        logger.warning("calculate_field_influence not yet in RuntimeStorage FFI")
        return 0.0

    def calculate_density(
        self, grid_id: int, space: int, x: float, y: float, z: float, radius: float
    ) -> float:
        """Calculate token density in region."""
        # Use range_query and count results
        results = self.range_query(grid_id, space, x, y, z, radius)
        volume = (4.0 / 3.0) * 3.14159 * (radius ** 3)  # Sphere volume
        return len(results) / volume if volume > 0 else 0.0


class RuntimeCDNAStorage(CDNAStorageInterface):
    """
    Runtime-based CDNA storage (v0.51.0).

    Persists CDNA configuration through neurograph.Runtime v0.50.0.
    """

    def __init__(self, runtime=None):
        """
        Initialize RuntimeCDNAStorage.

        Args:
            runtime: neurograph.Runtime instance. If None, creates new instance.
        """
        if runtime is None:
            try:
                from neurograph import Runtime, Config
                config = Config(grid_size=1000, dimensions=50)
                runtime = Runtime(config)
                logger.info("Created new Runtime instance for CDNAStorage")
            except ImportError as e:
                raise ImportError(
                    "neurograph package not found. "
                    "Please build with: cd src/core_rust && maturin develop --release --features python-bindings"
                ) from e

        self._runtime = runtime
        self._history: List[Dict[str, Any]] = []
        self._quarantine_active = False
        logger.info("RuntimeCDNAStorage initialized with Runtime v0.50.0")

    def get_config(self) -> Dict[str, Any]:
        """Get current CDNA configuration from RuntimeStorage."""
        config = self._runtime.cdna.get_config()
        scales = self._runtime.cdna.get_scales()

        return {
            "profile_id": config["profile_id"],
            "flags": config["flags"],
            "scales": scales,
            "version": "2.1"
        }

    def update_config(self, config: Dict[str, Any]) -> bool:
        """Update CDNA configuration in RuntimeStorage."""
        try:
            # Update scales if provided
            if 'scales' in config:
                self._runtime.cdna.update_scales(config['scales'])

            # Update profile if provided
            if 'profile_id' in config:
                self._runtime.cdna.set_profile(config['profile_id'])

            # Update flags if provided
            if 'flags' in config:
                self._runtime.cdna.set_flags(config['flags'])

            # Add to history
            self.add_history({
                "action": "update_config",
                "config": config,
                "timestamp": __import__('time').time()
            })

            return True
        except Exception as e:
            logger.error(f"Failed to update CDNA config: {e}")
            return False

    def get_profile(self, profile_id: str) -> Optional[Dict[str, Any]]:
        """Get profile by ID."""
        # Get current profile ID
        current_profile = self._runtime.cdna.get_profile()

        if current_profile == profile_id:
            return {
                "profile_id": profile_id,
                "scales": self._runtime.cdna.get_scales(),
                "flags": self._runtime.cdna.get_flags()
            }

        return None

    def list_profiles(self) -> Dict[str, Dict[str, Any]]:
        """List all available profiles (predefined in Rust)."""
        # CDNA profiles are hardcoded in Rust
        # Return current profile for now
        current_profile = self._runtime.cdna.get_profile()

        return {
            current_profile: {
                "profile_id": current_profile,
                "scales": self._runtime.cdna.get_scales(),
                "flags": self._runtime.cdna.get_flags()
            }
        }

    def switch_profile(self, profile_id: str) -> bool:
        """Switch to different profile."""
        try:
            self._runtime.cdna.set_profile(profile_id)

            # Add to history
            self.add_history({
                "action": "switch_profile",
                "profile_id": profile_id,
                "timestamp": __import__('time').time()
            })

            return True
        except Exception as e:
            logger.error(f"Failed to switch profile: {e}")
            return False

    def get_history(self, limit: int = 10) -> List[Dict[str, Any]]:
        """Get configuration change history."""
        return self._history[-limit:]

    def add_history(self, entry: Dict[str, Any]) -> None:
        """Add entry to history."""
        self._history.append(entry)

        # Keep only last 1000 entries
        if len(self._history) > 1000:
            self._history = self._history[-1000:]

    def validate_scales(self, scales: List[float]) -> Tuple[bool, List[str], List[str]]:
        """Validate dimension scales using Runtime."""
        warnings = []
        errors = []

        # Check count
        if len(scales) != 8:
            errors.append(f"Expected 8 scales, got {len(scales)}")
            return (False, warnings, errors)

        # Check range
        for i, scale in enumerate(scales):
            if scale < 0.1 or scale > 10.0:
                warnings.append(f"Scale {i} ({scale}) is outside recommended range [0.1, 10.0]")

        # Validate in Rust
        is_valid = self._runtime.cdna.validate()

        return (is_valid and len(errors) == 0, warnings, errors)

    def get_quarantine_status(self) -> Dict[str, Any]:
        """Get quarantine status (not in RuntimeStorage FFI yet)."""
        return {
            "active": self._quarantine_active,
            "pending_changes": {},
            "message": "Quarantine not yet in RuntimeStorage FFI"
        }

    def start_quarantine(self) -> bool:
        """Start quarantine mode (stub)."""
        self._quarantine_active = True
        logger.info("Quarantine mode started (local only, not in FFI)")
        return True

    def stop_quarantine(self, apply: bool = False) -> bool:
        """Stop quarantine mode (stub)."""
        self._quarantine_active = False
        logger.info(f"Quarantine mode stopped (apply={apply})")
        return True
