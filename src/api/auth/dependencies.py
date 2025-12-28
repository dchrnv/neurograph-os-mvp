"""
FastAPI dependencies for authentication and authorization.
"""

from typing import Optional, Union
from fastapi import Depends, HTTPException, status, Header
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials

from ..models.auth import User
from .jwt import jwt_manager
from .rbac import get_user_role, get_permissions_for_role
from ..storage.api_keys import get_api_key_storage


# Security scheme for Swagger UI
security = HTTPBearer(
    scheme_name="JWT Bearer Token",
    description="Enter your JWT access token",
    bearerFormat="JWT",
)


async def get_current_user(
    credentials: HTTPAuthorizationCredentials = Depends(security),
) -> User:
    """
    Dependency to get current authenticated user from JWT token.

    Args:
        credentials: HTTP Authorization credentials

    Returns:
        Current user

    Raises:
        HTTPException: 401 if token invalid or expired

    Example:
        >>> @app.get("/api/v1/protected")
        >>> async def protected_route(user: User = Depends(get_current_user)):
        >>>     return {"user_id": user.user_id}
    """
    token = credentials.credentials

    try:
        # Verify token
        payload = jwt_manager.verify_token(token, expected_type="access")

        # Get user role and permissions
        user_id = payload.sub
        role = get_user_role(user_id)
        scopes = get_permissions_for_role(role)

        # Create user object
        user = User(
            user_id=user_id,
            username=user_id,  # In production, query from database
            role=role.value,
            scopes=scopes,
            disabled=False,
        )

        return user

    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail=f"Invalid authentication credentials: {str(e)}",
            headers={"WWW-Authenticate": "Bearer"},
        )


async def get_current_active_user(
    current_user: User = Depends(get_current_user),
) -> User:
    """
    Dependency to get current active (non-disabled) user.

    Args:
        current_user: Current user from get_current_user

    Returns:
        Current active user

    Raises:
        HTTPException: 400 if user is disabled

    Example:
        >>> @app.get("/api/v1/me")
        >>> async def read_users_me(user: User = Depends(get_current_active_user)):
        >>>     return user
    """
    if current_user.disabled:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Inactive user",
        )
    return current_user


async def get_optional_user(
    credentials: Optional[HTTPAuthorizationCredentials] = Depends(
        HTTPBearer(auto_error=False)
    ),
) -> Optional[User]:
    """
    Dependency to get current user if authenticated, None otherwise.

    Useful for endpoints that are public but have enhanced features for authenticated users.

    Args:
        credentials: HTTP Authorization credentials (optional)

    Returns:
        Current user or None

    Example:
        >>> @app.get("/api/v1/public")
        >>> async def public_route(user: Optional[User] = Depends(get_optional_user)):
        >>>     if user:
        >>>         return {"message": f"Hello, {user.username}"}
        >>>     return {"message": "Hello, anonymous"}
    """
    if not credentials:
        return None

    try:
        return await get_current_user(credentials)
    except HTTPException:
        return None


async def get_user_from_api_key(
    x_api_key: Optional[str] = Header(None, description="API Key")
) -> Optional[User]:
    """
    Authenticate user via API key.

    Args:
        x_api_key: API key from X-API-Key header

    Returns:
        User object if valid key, None otherwise

    Example:
        >>> @app.get("/api/v1/data")
        >>> async def get_data(user: User = Depends(get_user_from_api_key)):
        >>>     return {"data": "sensitive"}
    """
    if not x_api_key:
        return None

    # Verify API key
    api_key_storage = get_api_key_storage()
    api_key = api_key_storage.verify_key(x_api_key)

    if not api_key:
        return None

    # Create user object from API key
    # API keys act as "bot" users with custom scopes
    user = User(
        user_id=f"apikey_{api_key.key_id}",
        username=api_key.name,
        role="bot",  # API keys are treated as bot users
        scopes=api_key.scopes,  # Use key's specific scopes
        disabled=api_key.disabled,
    )

    return user


async def get_user_jwt_or_api_key(
    credentials: Optional[HTTPAuthorizationCredentials] = Depends(
        HTTPBearer(auto_error=False)
    ),
    x_api_key: Optional[str] = Header(None, description="API Key")
) -> User:
    """
    Authenticate user via JWT token OR API key.

    Tries JWT first, then falls back to API key.
    This allows flexible authentication for clients.

    Args:
        credentials: HTTP Authorization credentials (JWT)
        x_api_key: API key from X-API-Key header

    Returns:
        Current user

    Raises:
        HTTPException: 401 if both authentication methods fail

    Example:
        >>> @app.get("/api/v1/protected")
        >>> async def protected(user: User = Depends(get_user_jwt_or_api_key)):
        >>>     return {"user_id": user.user_id}
    """
    # Try JWT first
    if credentials:
        try:
            user = await get_current_user(credentials)
            if user and not user.disabled:
                return user
        except HTTPException:
            pass  # Fall through to API key

    # Try API key
    if x_api_key:
        user = await get_user_from_api_key(x_api_key)
        if user and not user.disabled:
            return user

    # Both failed
    raise HTTPException(
        status_code=status.HTTP_401_UNAUTHORIZED,
        detail="Invalid authentication credentials (JWT or API key required)",
        headers={"WWW-Authenticate": "Bearer"},
    )
