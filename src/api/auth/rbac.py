"""
Role-Based Access Control (RBAC) for NeuroGraph API.

Defines roles and maps them to permission sets.
"""

from enum import Enum
from typing import List

from .permissions import Permission, PERMISSION_SETS


class Role(str, Enum):
    """
    User roles with hierarchical permissions.

    Hierarchy: viewer < developer < admin
    """

    ADMIN = "admin"  # Full access to everything
    DEVELOPER = "developer"  # Read + Write (except system config)
    VIEWER = "viewer"  # Read-only access
    BOT = "bot"  # Limited access for bots/integrations


# Role → Permissions mapping
ROLE_PERMISSIONS: dict[Role, List[Permission]] = {
    Role.ADMIN: PERMISSION_SETS["admin"],  # All permissions
    Role.DEVELOPER: PERMISSION_SETS["read_write"],  # Read + Write
    Role.VIEWER: PERMISSION_SETS["read_only"],  # Read only
    Role.BOT: [
        # Bots can only emit signals and read basic info
        Permission.WRITE_TOKENS,  # Can create tokens (emit signals)
        Permission.READ_HEALTH,  # Can check health
        Permission.READ_STATUS,  # Can check status
    ],
}


def get_permissions_for_role(role: Role) -> List[str]:
    """
    Get all permissions for a role.

    Args:
        role: User role

    Returns:
        List of permission strings

    Example:
        >>> get_permissions_for_role(Role.DEVELOPER)
        ['tokens:read', 'tokens:write', ...]
    """
    permissions = ROLE_PERMISSIONS.get(role, [])
    return [p.value for p in permissions]


def get_user_role(user_id: str) -> Role:
    """
    Get role for user ID.

    In production, this would query a database.
    For MVP, use simple hardcoded mapping.

    Args:
        user_id: User identifier

    Returns:
        User's role

    Example:
        >>> get_user_role("admin")
        Role.ADMIN
    """
    # Hardcoded mapping for MVP
    # In production: query from database/LDAP/OAuth
    role_mapping = {
        "admin": Role.ADMIN,
        "developer": Role.DEVELOPER,
        "viewer": Role.VIEWER,
        "bot": Role.BOT,
    }

    return role_mapping.get(user_id, Role.VIEWER)  # Default to viewer


def has_role(user_role: Role, required_role: Role) -> bool:
    """
    Check if user's role meets minimum requirement.

    Checks role hierarchy: admin > developer > viewer > bot

    Args:
        user_role: User's current role
        required_role: Minimum required role

    Returns:
        True if user meets requirement

    Example:
        >>> has_role(Role.ADMIN, Role.DEVELOPER)
        True  # Admin can do everything Developer can
    """
    # Define role hierarchy (higher number = more permissions)
    hierarchy = {
        Role.BOT: 1,
        Role.VIEWER: 2,
        Role.DEVELOPER: 3,
        Role.ADMIN: 4,
    }

    return hierarchy.get(user_role, 0) >= hierarchy.get(required_role, 0)


def can_perform_action(user_role: Role, permission: Permission) -> bool:
    """
    Check if role has specific permission.

    Args:
        user_role: User's role
        permission: Required permission

    Returns:
        True if role has permission

    Example:
        >>> can_perform_action(Role.DEVELOPER, Permission.WRITE_TOKENS)
        True
    """
    role_permissions = ROLE_PERMISSIONS.get(user_role, [])
    return permission in role_permissions
