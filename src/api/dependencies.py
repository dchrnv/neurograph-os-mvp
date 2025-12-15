
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
FastAPI Dependencies

Dependency injection for runtime, storage, authentication, etc.
"""

from fastapi import Depends, HTTPException, status
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials
from typing import Optional
import logging

from .storage import (
    TokenStorageInterface,
    GridStorageInterface,
    CDNAStorageInterface
)
from .storage.memory import (
    InMemoryTokenStorage,
    InMemoryGridStorage,
    InMemoryCDNAStorage
)
from .storage.runtime import (
    RuntimeTokenStorage,
    RuntimeGridStorage,
    RuntimeCDNAStorage
)
from .config import settings

logger = logging.getLogger(__name__)

# Security scheme
security = HTTPBearer(auto_error=False)

# Global runtime instance (will be initialized on startup)
_runtime_instance: Optional[object] = None

# Global storage instances (singletons)
_token_storage: Optional[TokenStorageInterface] = None
_grid_storage: Optional[GridStorageInterface] = None
_cdna_storage: Optional[CDNAStorageInterface] = None


def get_runtime():
    """
    Dependency to get the NeuroGraph runtime instance.

    Returns:
        Runtime instance

    Raises:
        HTTPException: If runtime is not initialized
    """
    global _runtime_instance

    if _runtime_instance is None:
        # For now, return None - will initialize in Phase 2.2
        # TODO: Initialize runtime on first call
        logger.warning("Runtime not initialized yet")
        return None

    return _runtime_instance


def set_runtime(runtime):
    """Set the global runtime instance."""
    global _runtime_instance
    _runtime_instance = runtime


async def verify_token(
    credentials: Optional[HTTPAuthorizationCredentials] = Depends(security)
) -> Optional[str]:
    """
    Verify JWT token (optional for development mode).

    Args:
        credentials: HTTP Bearer token

    Returns:
        User ID if authenticated, None if no token provided

    Raises:
        HTTPException: If token is invalid
    """
    if credentials is None:
        # Development mode: allow unauthenticated access
        return None

    token = credentials.credentials

    # TODO: Implement JWT verification
    # For now, accept any token in development
    logger.info(f"Token provided: {token[:20]}...")

    return "anonymous"


async def require_admin(user_id: Optional[str] = Depends(verify_token)) -> str:
    """
    Require admin authentication.

    Args:
        user_id: User ID from token verification

    Returns:
        User ID

    Raises:
        HTTPException: If not authenticated as admin
    """
    if user_id is None:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Admin authentication required"
        )

    # TODO: Check if user is admin
    return user_id


# =============================================================================
# Storage Dependencies
# =============================================================================

def get_token_storage() -> TokenStorageInterface:
    """
    Get token storage instance.

    Returns appropriate storage backend based on settings.
    Uses singleton pattern for lifecycle management.
    """
    global _token_storage

    if _token_storage is None:
        backend = settings.STORAGE_BACKEND.lower()

        if backend == "memory":
            _token_storage = InMemoryTokenStorage()
            logger.info("Initialized InMemory token storage")
        elif backend == "runtime":
            _token_storage = RuntimeTokenStorage()
            logger.info("Initialized Runtime token storage")
        else:
            logger.warning(f"Unknown storage backend '{backend}', defaulting to memory")
            _token_storage = InMemoryTokenStorage()

    return _token_storage


def get_grid_storage() -> GridStorageInterface:
    """
    Get grid storage instance.

    Returns appropriate storage backend based on settings.
    Uses singleton pattern for lifecycle management.
    """
    global _grid_storage

    if _grid_storage is None:
        backend = settings.STORAGE_BACKEND.lower()

        if backend == "memory":
            _grid_storage = InMemoryGridStorage()
            logger.info("Initialized InMemory grid storage")
        elif backend == "runtime":
            _grid_storage = RuntimeGridStorage()
            logger.info("Initialized Runtime grid storage")
        else:
            logger.warning(f"Unknown storage backend '{backend}', defaulting to memory")
            _grid_storage = InMemoryGridStorage()

    return _grid_storage


def get_cdna_storage() -> CDNAStorageInterface:
    """
    Get CDNA storage instance.

    Returns appropriate storage backend based on settings.
    Uses singleton pattern for lifecycle management.
    """
    global _cdna_storage

    if _cdna_storage is None:
        backend = settings.STORAGE_BACKEND.lower()

        if backend == "memory":
            _cdna_storage = InMemoryCDNAStorage()
            logger.info("Initialized InMemory CDNA storage")
        elif backend == "runtime":
            _cdna_storage = RuntimeCDNAStorage()
            logger.info("Initialized Runtime CDNA storage")
        else:
            logger.warning(f"Unknown storage backend '{backend}', defaulting to memory")
            _cdna_storage = InMemoryCDNAStorage()

    return _cdna_storage


def reset_storage():
    """
    Reset all storage instances.

    Useful for testing or when changing storage backend at runtime.
    """
    global _token_storage, _grid_storage, _cdna_storage

    _token_storage = None
    _grid_storage = None
    _cdna_storage = None

    logger.info("Storage instances reset")


# =============================================================================
# Grid Availability Check
# =============================================================================

def check_grid_available() -> bool:
    """
    Check if Grid V2.0 (Rust FFI) is available.

    Returns:
        True if Grid is available, False otherwise
    """
    try:
        # Try to import Rust Grid V2.0
        import neurograph_grid_v2
        return True
    except ImportError:
        return False
