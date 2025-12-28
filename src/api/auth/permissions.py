"""
Permission system for NeuroGraph API.

Defines fine-grained permissions for resources and operations.
"""

from enum import Enum
from typing import List, Callable
from functools import wraps

from fastapi import HTTPException, status


class Permission(str, Enum):
    """
    Fine-grained permissions for API resources.

    Naming convention: <resource>:<operation>
    """

    # Tokens
    READ_TOKENS = "tokens:read"
    WRITE_TOKENS = "tokens:write"
    DELETE_TOKENS = "tokens:delete"

    # Connections
    READ_CONNECTIONS = "connections:read"
    WRITE_CONNECTIONS = "connections:write"
    DELETE_CONNECTIONS = "connections:delete"

    # Grid
    READ_GRID = "grid:read"
    WRITE_GRID = "grid:write"

    # CDNA
    READ_CDNA = "cdna:read"
    WRITE_CDNA = "cdna:write"

    # Metrics
    READ_METRICS = "metrics:read"

    # Health
    READ_HEALTH = "health:read"

    # Status
    READ_STATUS = "status:read"

    # Admin operations
    ADMIN_CONFIG = "config:admin"
    ADMIN_BOOTSTRAP = "bootstrap:admin"
    ADMIN_LOGS = "logs:admin"
    ADMIN_USERS = "users:admin"

    # API Keys
    READ_API_KEYS = "api_keys:read"
    WRITE_API_KEYS = "api_keys:write"
    DELETE_API_KEYS = "api_keys:delete"


# Permission sets for common use cases
PERMISSION_SETS = {
    "read_only": [
        Permission.READ_TOKENS,
        Permission.READ_CONNECTIONS,
        Permission.READ_GRID,
        Permission.READ_CDNA,
        Permission.READ_METRICS,
        Permission.READ_HEALTH,
        Permission.READ_STATUS,
    ],
    "read_write": [
        Permission.READ_TOKENS,
        Permission.WRITE_TOKENS,
        Permission.READ_CONNECTIONS,
        Permission.WRITE_CONNECTIONS,
        Permission.READ_GRID,
        Permission.WRITE_GRID,
        Permission.READ_CDNA,
        Permission.WRITE_CDNA,
        Permission.READ_METRICS,
        Permission.READ_HEALTH,
        Permission.READ_STATUS,
    ],
    "admin": [
        # All permissions
        *[p for p in Permission],
    ],
}


def check_permission(user_scopes: List[str], required_permission: Permission) -> bool:
    """
    Check if user has required permission.

    Args:
        user_scopes: List of user's permission scopes
        required_permission: Required permission

    Returns:
        True if user has permission, False otherwise

    Example:
        >>> check_permission(["tokens:read", "tokens:write"], Permission.READ_TOKENS)
        True
    """
    return required_permission.value in user_scopes


def require_permission(permission: Permission):
    """
    Decorator to require specific permission for endpoint.

    Args:
        permission: Required permission

    Raises:
        HTTPException: 403 Forbidden if permission denied

    Example:
        >>> @app.get("/api/v1/tokens")
        >>> @require_permission(Permission.READ_TOKENS)
        >>> async def get_tokens(current_user: User = Depends(get_current_user)):
        >>>     ...
    """

    def decorator(func: Callable):
        @wraps(func)
        async def wrapper(*args, **kwargs):
            # Extract current_user from kwargs
            current_user = kwargs.get("current_user")
            if not current_user:
                raise HTTPException(
                    status_code=status.HTTP_401_UNAUTHORIZED,
                    detail="Authentication required",
                )

            # Check permission
            user_scopes = getattr(current_user, "scopes", [])
            if not check_permission(user_scopes, permission):
                raise HTTPException(
                    status_code=status.HTTP_403_FORBIDDEN,
                    detail=f"Permission denied: {permission.value} required",
                )

            return await func(*args, **kwargs)

        return wrapper

    return decorator


def require_any_permission(*permissions: Permission):
    """
    Decorator to require any of the specified permissions.

    Args:
        permissions: List of acceptable permissions

    Raises:
        HTTPException: 403 Forbidden if no matching permission

    Example:
        >>> @require_any_permission(Permission.READ_TOKENS, Permission.ADMIN_CONFIG)
        >>> async def get_resource(...):
        >>>     ...
    """

    def decorator(func: Callable):
        @wraps(func)
        async def wrapper(*args, **kwargs):
            current_user = kwargs.get("current_user")
            if not current_user:
                raise HTTPException(
                    status_code=status.HTTP_401_UNAUTHORIZED,
                    detail="Authentication required",
                )

            user_scopes = getattr(current_user, "scopes", [])
            has_permission = any(
                check_permission(user_scopes, perm) for perm in permissions
            )

            if not has_permission:
                perms_str = ", ".join(p.value for p in permissions)
                raise HTTPException(
                    status_code=status.HTTP_403_FORBIDDEN,
                    detail=f"Permission denied: one of [{perms_str}] required",
                )

            return await func(*args, **kwargs)

        return wrapper

    return decorator


def require_all_permissions(*permissions: Permission):
    """
    Decorator to require all of the specified permissions.

    Args:
        permissions: List of required permissions

    Raises:
        HTTPException: 403 Forbidden if any permission missing

    Example:
        >>> @require_all_permissions(Permission.READ_TOKENS, Permission.WRITE_TOKENS)
        >>> async def modify_tokens(...):
        >>>     ...
    """

    def decorator(func: Callable):
        @wraps(func)
        async def wrapper(*args, **kwargs):
            current_user = kwargs.get("current_user")
            if not current_user:
                raise HTTPException(
                    status_code=status.HTTP_401_UNAUTHORIZED,
                    detail="Authentication required",
                )

            user_scopes = getattr(current_user, "scopes", [])
            missing_permissions = [
                perm
                for perm in permissions
                if not check_permission(user_scopes, perm)
            ]

            if missing_permissions:
                perms_str = ", ".join(p.value for p in missing_permissions)
                raise HTTPException(
                    status_code=status.HTTP_403_FORBIDDEN,
                    detail=f"Permission denied: missing [{perms_str}]",
                )

            return await func(*args, **kwargs)

        return wrapper

    return decorator
