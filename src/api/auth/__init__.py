"""
Authentication and Authorization module for NeuroGraph API.

Provides:
- JWT token generation and validation
- RBAC (Role-Based Access Control)
- API keys management
- Permission system
"""

from .jwt import JWTManager, TokenPayload
from .permissions import Permission, require_permission
from .rbac import Role, get_user_role
from .dependencies import (
    get_current_user,
    get_current_active_user,
    get_user_from_api_key,
    get_user_jwt_or_api_key
)

__all__ = [
    "JWTManager",
    "TokenPayload",
    "Permission",
    "require_permission",
    "Role",
    "get_user_role",
    "get_current_user",
    "get_current_active_user",
    "get_user_from_api_key",
    "get_user_jwt_or_api_key",
]
