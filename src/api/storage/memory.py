
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
In-Memory Storage Implementation

Thread-safe in-memory storage for tokens, grids, and CDNA configuration.
Ported from MVP API with enhancements for production use.
"""

import threading
import time
from typing import Dict, List, Optional, Tuple, Any
from datetime import datetime
from . import TokenStorageInterface, GridStorageInterface, CDNAStorageInterface
import sys
from pathlib import Path

# Add src/core to path
src_path = Path(__file__).parent.parent.parent
sys.path.insert(0, str(src_path))

from core.token.token_v2 import Token, create_token_id, FLAG_ACTIVE, FLAG_PERSISTENT

# Try to import Rust Grid (optional)
try:
    from neurograph import Grid, GridConfig, CoordinateSpace
    GRID_AVAILABLE = True
except ImportError:
    GRID_AVAILABLE = False
    Grid = None
    GridConfig = None
    CoordinateSpace = None


# =============================================================================
# Token Storage
# =============================================================================

class InMemoryTokenStorage(TokenStorageInterface):
    """Thread-safe in-memory token storage."""

    _instance = None
    _lock = threading.Lock()

    def __new__(cls):
        if cls._instance is None:
            with cls._lock:
                if cls._instance is None:
                    cls._instance = super().__new__(cls)
                    cls._instance._initialized = False
        return cls._instance

    def __init__(self):
        if self._initialized:
            return

        self._storage: Dict[int, Token] = {}
        self._next_local_id = 1
        self._lock = threading.RLock()
        self._initialized = True

    def get(self, token_id: int) -> Optional[Token]:
        with self._lock:
            return self._storage.get(token_id)

    def create(self, token_data: Dict[str, Any]) -> Token:
        with self._lock:
            # Extract parameters
            entity_type = token_data.get('entity_type', 0)
            domain = token_data.get('domain', 0)
            weight = token_data.get('weight', 0.5)
            field_radius = token_data.get('field_radius', 1.0)
            field_strength = token_data.get('field_strength', 1.0)
            persistent = token_data.get('persistent', False)

            # Generate token ID
            token_id = create_token_id(self._next_local_id, entity_type, domain)
            self._next_local_id += 1

            # Create token
            token = Token(id=token_id)
            token.weight = weight
            token.field_radius = field_radius
            token.field_strength = field_strength

            if persistent:
                token.set_flag(FLAG_PERSISTENT)

            # Set coordinates for all 8 levels
            for level in range(8):
                coord_key = f'l{level+1}_{"physical sensory motor emotional cognitive social temporal abstract".split()[level]}'
                coords = token_data.get(coord_key)
                if coords:
                    x = coords.get('x')
                    y = coords.get('y')
                    z = coords.get('z')
                    if x is not None or y is not None or z is not None:
                        token.set_coordinates(level, x, y, z)

            # Store token
            self._storage[token_id] = token
            return token

    def update(self, token_id: int, token_data: Dict[str, Any]) -> Optional[Token]:
        with self._lock:
            token = self._storage.get(token_id)
            if not token:
                return None

            # Update fields
            if 'weight' in token_data:
                token.weight = token_data['weight']
            if 'field_radius' in token_data:
                token.field_radius = token_data['field_radius']
            if 'field_strength' in token_data:
                token.field_strength = token_data['field_strength']

            # Update coordinates
            for level in range(8):
                coord_key = f'l{level+1}_{"physical sensory motor emotional cognitive social temporal abstract".split()[level]}'
                coords = token_data.get(coord_key)
                if coords:
                    x = coords.get('x')
                    y = coords.get('y')
                    z = coords.get('z')
                    if x is not None or y is not None or z is not None:
                        token.set_coordinates(level, x, y, z)

            return token

    def delete(self, token_id: int) -> bool:
        with self._lock:
            if token_id in self._storage:
                del self._storage[token_id]
                return True
            return False

    def list(self, limit: int = 100, offset: int = 0) -> List[Token]:
        with self._lock:
            tokens = list(self._storage.values())
            if limit == 0:  # Special case: return all for counting
                return tokens
            return tokens[offset:offset + limit]

    def clear(self) -> int:
        with self._lock:
            count = len(self._storage)
            self._storage.clear()
            self._next_local_id = 1
            return count

    def count(self) -> int:
        with self._lock:
            return len(self._storage)


# =============================================================================
# Grid Storage
# =============================================================================

class InMemoryGridStorage(GridStorageInterface):
    """Thread-safe in-memory grid storage."""

    _instance = None
    _lock = threading.Lock()

    def __new__(cls):
        if cls._instance is None:
            with cls._lock:
                if cls._instance is None:
                    cls._instance = super().__new__(cls)
                    cls._instance._initialized = False
        return cls._instance

    def __init__(self):
        if self._initialized:
            return

        self._grids: Dict[int, Any] = {}
        self._next_grid_id = 1
        self._lock = threading.RLock()
        self._initialized = True

    def create_grid(self, config: Optional[Dict[str, Any]] = None) -> int:
        if not GRID_AVAILABLE:
            raise RuntimeError("Grid not available. Install Rust bindings.")

        with self._lock:
            # Create grid with optional config
            if config:
                grid_config = GridConfig()
                grid_config.bucket_size = config.get('bucket_size', 10.0)
                grid_config.density_threshold = config.get('density_threshold', 0.5)
                grid_config.min_field_nodes = config.get('min_field_nodes', 3)
                grid = Grid(grid_config)
            else:
                grid = Grid()

            grid_id = self._next_grid_id
            self._next_grid_id += 1
            self._grids[grid_id] = grid
            return grid_id

    def get_grid(self, grid_id: int) -> Optional[Any]:
        with self._lock:
            return self._grids.get(grid_id)

    def delete_grid(self, grid_id: int) -> bool:
        with self._lock:
            if grid_id in self._grids:
                del self._grids[grid_id]
                return True
            return False

    def list_grids(self) -> List[int]:
        with self._lock:
            return list(self._grids.keys())

    def add_token(self, grid_id: int, token: Token) -> bool:
        if not GRID_AVAILABLE:
            raise RuntimeError("Grid not available.")

        with self._lock:
            grid = self._grids.get(grid_id)
            if not grid:
                return False

            # Convert Python Token to Rust Token
            rust_token = __import__('neurograph').Token(token.id)

            # Copy coordinates from all spaces
            space_map = [
                (0, CoordinateSpace.L1Physical()),
                (1, CoordinateSpace.L2Sensory()),
                (2, CoordinateSpace.L3Motor()),
                (3, CoordinateSpace.L4Emotional()),
                (4, CoordinateSpace.L5Cognitive()),
                (5, CoordinateSpace.L6Social()),
                (6, CoordinateSpace.L7Temporal()),
                (7, CoordinateSpace.L8Abstract()),
            ]

            for level, space_enum in space_map:
                coords = token.get_coordinates(level)
                if coords:
                    rust_token.set_coordinates(space_enum, coords[0], coords[1], coords[2])

            # Copy properties
            rust_token.weight = token.weight
            rust_token.field_radius = int(token.field_radius * 100)
            rust_token.field_strength = int(token.field_strength * 255)
            rust_token.set_active(True)

            # Add to grid
            grid.add(rust_token)
            return True

    def remove_token(self, grid_id: int, token_id: int) -> bool:
        with self._lock:
            grid = self._grids.get(grid_id)
            if not grid:
                return False

            removed = grid.remove(token_id)
            return removed is not None

    def find_neighbors(
        self, grid_id: int, token_id: int, space: int, radius: float, max_results: int
    ) -> List[Tuple[int, float]]:
        with self._lock:
            grid = self._grids.get(grid_id)
            if not grid:
                return []

            return grid.find_neighbors(token_id, space, radius, max_results)

    def range_query(
        self, grid_id: int, space: int, x: float, y: float, z: float, radius: float
    ) -> List[Tuple[int, float]]:
        with self._lock:
            grid = self._grids.get(grid_id)
            if not grid:
                return []

            return grid.range_query(space, x, y, z, radius)

    def calculate_field_influence(
        self, grid_id: int, space: int, x: float, y: float, z: float, radius: float
    ) -> float:
        with self._lock:
            grid = self._grids.get(grid_id)
            if not grid:
                return 0.0

            return grid.calculate_field_influence(space, x, y, z, radius)

    def calculate_density(
        self, grid_id: int, space: int, x: float, y: float, z: float, radius: float
    ) -> float:
        with self._lock:
            grid = self._grids.get(grid_id)
            if not grid:
                return 0.0

            return grid.calculate_density(space, x, y, z, radius)


# =============================================================================
# CDNA Storage
# =============================================================================

class InMemoryCDNAStorage(CDNAStorageInterface):
    """Thread-safe in-memory CDNA storage."""

    _instance = None
    _lock = threading.Lock()

    # Available profiles (from MVP)
    PROFILES = {
        "explorer": {
            "name": "Explorer",
            "scales": [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 5.0],
            "description": "Свободная структура, высокая пластичность",
            "plasticity": 0.8,
            "evolution_rate": 0.5
        },
        "analyzer": {
            "name": "Analyzer",
            "scales": [1.0, 1.0, 1.0, 1.5, 10.0, 5.0, 3.0, 20.0],
            "description": "Строгие правила, низкая эволюция",
            "plasticity": 0.2,
            "evolution_rate": 0.1
        },
        "creative": {
            "name": "Creative",
            "scales": [1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0],
            "description": "Экспериментальный режим",
            "plasticity": 0.95,
            "evolution_rate": 0.8
        },
        "quarantine": {
            "name": "Quarantine",
            "scales": [1.0, 1.0, 1.0, 1.0, 2.0, 1.5, 1.0, 3.0],
            "description": "Изолированный режим тестирования",
            "plasticity": 0.1,
            "evolution_rate": 0.0,
            "restricted": True,
            "max_change": 0.5
        }
    }

    def __new__(cls):
        if cls._instance is None:
            with cls._lock:
                if cls._instance is None:
                    cls._instance = super().__new__(cls)
                    cls._instance._initialized = False
        return cls._instance

    def __init__(self):
        if self._initialized:
            return

        self._config = {
            "version": "2.1.0",
            "profile": "explorer",
            "dimension_scales": self.PROFILES["explorer"]["scales"].copy(),
            "timestamp": datetime.now().isoformat()
        }
        self._history: List[Dict[str, Any]] = []
        self._quarantine = {
            "active": False,
            "time_left": 300,
            "metrics": {
                "memory_growth": 0,
                "connection_breaks": 0,
                "token_churn": 0
            }
        }
        self._lock = threading.RLock()
        self._initialized = True

    def get_config(self) -> Dict[str, Any]:
        with self._lock:
            return self._config.copy()

    def update_config(self, config: Dict[str, Any]) -> bool:
        with self._lock:
            if 'profile' in config:
                self._config['profile'] = config['profile']
            if 'dimension_scales' in config:
                self._config['dimension_scales'] = config['dimension_scales']
            self._config['timestamp'] = datetime.now().isoformat()
            return True

    def get_profile(self, profile_id: str) -> Optional[Dict[str, Any]]:
        return self.PROFILES.get(profile_id)

    def list_profiles(self) -> Dict[str, Dict[str, Any]]:
        return self.PROFILES.copy()

    def switch_profile(self, profile_id: str) -> bool:
        with self._lock:
            if profile_id not in self.PROFILES:
                return False

            profile = self.PROFILES[profile_id]
            self._config['profile'] = profile_id
            self._config['dimension_scales'] = profile['scales'].copy()
            self._config['timestamp'] = datetime.now().isoformat()
            return True

    def get_history(self, limit: int = 10) -> List[Dict[str, Any]]:
        with self._lock:
            return self._history[:limit]

    def add_history(self, entry: Dict[str, Any]) -> None:
        with self._lock:
            entry['timestamp'] = datetime.now().isoformat()
            self._history.insert(0, entry)

    def validate_scales(self, scales: List[float]) -> Tuple[bool, List[str], List[str]]:
        warnings = []
        errors = []

        # Dimension limits
        dimension_limits = [
            (0, 20), (0, 20), (0, 20), (0, 20),  # Physical, Sensory, Motor, Emotional
            (0, 30), (0, 20), (0, 20), (0, 50)   # Cognitive, Social, Temporal, Abstract
        ]

        for i, (scale, (min_val, max_val)) in enumerate(zip(scales, dimension_limits)):
            if scale < min_val or scale > max_val:
                errors.append(f"Dimension {i} value {scale} out of range [{min_val}, {max_val}]")
            elif scale > max_val * 0.75:
                warnings.append(f"Dimension {i} value {scale} in danger zone (>{max_val * 0.75})")
            elif scale > max_val * 0.5:
                warnings.append(f"Dimension {i} value {scale} in caution zone (>{max_val * 0.5})")

        return (len(errors) == 0, warnings, errors)

    def get_quarantine_status(self) -> Dict[str, Any]:
        with self._lock:
            return self._quarantine.copy()

    def start_quarantine(self) -> bool:
        with self._lock:
            if self._quarantine["active"]:
                return False

            self._quarantine["active"] = True
            self._quarantine["time_left"] = 300
            self._quarantine["metrics"] = {
                "memory_growth": 0,
                "connection_breaks": 0,
                "token_churn": 0
            }
            return True

    def stop_quarantine(self, apply: bool = False) -> bool:
        with self._lock:
            if not self._quarantine["active"]:
                return False

            self._quarantine["active"] = False

            if apply:
                self.add_history({
                    "action": "quarantine_applied",
                    "metrics": self._quarantine["metrics"].copy()
                })

            return True


# =============================================================================
# Singleton Accessors
# =============================================================================

_token_storage = None
_grid_storage = None
_cdna_storage = None


def get_memory_token_storage() -> InMemoryTokenStorage:
    """Get singleton token storage instance."""
    global _token_storage
    if _token_storage is None:
        _token_storage = InMemoryTokenStorage()
    return _token_storage


def get_memory_grid_storage() -> InMemoryGridStorage:
    """Get singleton grid storage instance."""
    global _grid_storage
    if _grid_storage is None:
        _grid_storage = InMemoryGridStorage()
    return _grid_storage


def get_memory_cdna_storage() -> InMemoryCDNAStorage:
    """Get singleton CDNA storage instance."""
    global _cdna_storage
    if _cdna_storage is None:
        _cdna_storage = InMemoryCDNAStorage()
    return _cdna_storage


def initialize_memory_storage() -> None:
    """Initialize all storage singletons."""
    get_memory_token_storage()
    get_memory_grid_storage()
    get_memory_cdna_storage()


def cleanup_memory_storage() -> None:
    """Cleanup storage (for shutdown)."""
    global _token_storage, _grid_storage, _cdna_storage
    if _token_storage:
        _token_storage.clear()
    _token_storage = None
    _grid_storage = None
    _cdna_storage = None


def is_grid_available() -> bool:
    """Check if Rust Grid is available."""
    return GRID_AVAILABLE
