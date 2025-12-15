
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

Dependency injection for runtime, authentication, etc.
"""

from fastapi import Depends, HTTPException, status
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials
from typing import Optional
import logging

logger = logging.getLogger(__name__)

# Security scheme
security = HTTPBearer(auto_error=False)

# Global runtime instance (will be initialized on startup)
_runtime_instance: Optional[object] = None


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
