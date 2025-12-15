
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
Storage Abstraction Layer

Provides interfaces and implementations for token, grid, and CDNA storage.
Supports multiple backends: in-memory (current) and runtime (future).
"""

from abc import ABC, abstractmethod
from typing import List, Optional, Dict, Tuple, Any
import sys
from pathlib import Path

# Add src/core to path for Token import
src_path = Path(__file__).parent.parent.parent
sys.path.insert(0, str(src_path))

from core.token.token_v2 import Token


class TokenStorageInterface(ABC):
    """Abstract interface for token storage."""

    @abstractmethod
    def get(self, token_id: int) -> Optional[Token]:
        """Get token by ID."""
        pass

    @abstractmethod
    def create(self, token_data: Dict[str, Any]) -> Token:
        """Create new token."""
        pass

    @abstractmethod
    def update(self, token_id: int, token_data: Dict[str, Any]) -> Optional[Token]:
        """Update existing token."""
        pass

    @abstractmethod
    def delete(self, token_id: int) -> bool:
        """Delete token. Returns True if deleted, False if not found."""
        pass

    @abstractmethod
    def list(self, limit: int = 100, offset: int = 0) -> List[Token]:
        """List tokens with pagination."""
        pass

    @abstractmethod
    def clear(self) -> int:
        """Clear all tokens. Returns count of deleted tokens."""
        pass

    @abstractmethod
    def count(self) -> int:
        """Get total count of tokens."""
        pass


class GridStorageInterface(ABC):
    """Abstract interface for grid storage."""

    @abstractmethod
    def create_grid(self, config: Optional[Dict[str, Any]] = None) -> int:
        """Create new grid instance. Returns grid_id."""
        pass

    @abstractmethod
    def get_grid(self, grid_id: int) -> Optional[Any]:
        """Get grid instance by ID."""
        pass

    @abstractmethod
    def delete_grid(self, grid_id: int) -> bool:
        """Delete grid instance."""
        pass

    @abstractmethod
    def list_grids(self) -> List[int]:
        """List all grid IDs."""
        pass

    @abstractmethod
    def add_token(self, grid_id: int, token: Token) -> bool:
        """Add token to grid."""
        pass

    @abstractmethod
    def remove_token(self, grid_id: int, token_id: int) -> bool:
        """Remove token from grid."""
        pass

    @abstractmethod
    def find_neighbors(
        self, grid_id: int, token_id: int, space: int, radius: float, max_results: int
    ) -> List[Tuple[int, float]]:
        """Find neighbors of token. Returns list of (token_id, distance) tuples."""
        pass

    @abstractmethod
    def range_query(
        self, grid_id: int, space: int, x: float, y: float, z: float, radius: float
    ) -> List[Tuple[int, float]]:
        """Find all tokens within radius of point."""
        pass

    @abstractmethod
    def calculate_field_influence(
        self, grid_id: int, space: int, x: float, y: float, z: float, radius: float
    ) -> float:
        """Calculate field influence at point."""
        pass

    @abstractmethod
    def calculate_density(
        self, grid_id: int, space: int, x: float, y: float, z: float, radius: float
    ) -> float:
        """Calculate token density in region."""
        pass


class CDNAStorageInterface(ABC):
    """Abstract interface for CDNA storage."""

    @abstractmethod
    def get_config(self) -> Dict[str, Any]:
        """Get current CDNA configuration."""
        pass

    @abstractmethod
    def update_config(self, config: Dict[str, Any]) -> bool:
        """Update CDNA configuration."""
        pass

    @abstractmethod
    def get_profile(self, profile_id: str) -> Optional[Dict[str, Any]]:
        """Get profile by ID."""
        pass

    @abstractmethod
    def list_profiles(self) -> Dict[str, Dict[str, Any]]:
        """List all available profiles."""
        pass

    @abstractmethod
    def switch_profile(self, profile_id: str) -> bool:
        """Switch to different profile."""
        pass

    @abstractmethod
    def get_history(self, limit: int = 10) -> List[Dict[str, Any]]:
        """Get configuration change history."""
        pass

    @abstractmethod
    def add_history(self, entry: Dict[str, Any]) -> None:
        """Add entry to history."""
        pass

    @abstractmethod
    def validate_scales(self, scales: List[float]) -> Tuple[bool, List[str], List[str]]:
        """Validate dimension scales. Returns (valid, warnings, errors)."""
        pass

    @abstractmethod
    def get_quarantine_status(self) -> Dict[str, Any]:
        """Get quarantine status."""
        pass

    @abstractmethod
    def start_quarantine(self) -> bool:
        """Start quarantine mode."""
        pass

    @abstractmethod
    def stop_quarantine(self, apply: bool = False) -> bool:
        """Stop quarantine mode."""
        pass
