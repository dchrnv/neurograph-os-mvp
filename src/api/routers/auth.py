"""
Authentication and authorization endpoints for NeuroGraph API.

Provides:
- User login/logout
- Token refresh
- User profile
- Password management
"""

import hashlib
import secrets
import time
from datetime import datetime
from typing import Dict

from fastapi import APIRouter, Depends, HTTPException, status
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials

from ..models.auth import (
    LoginRequest,
    LoginResponse,
    RefreshTokenRequest,
    RefreshTokenResponse,
    LogoutRequest,
    User,
    UserInDB,
    ChangePasswordRequest,
)
from ..models.response import SuccessResponse, ErrorResponse
from ..auth.jwt import jwt_manager
from ..auth.dependencies import get_current_user, get_current_active_user
from ..auth.rbac import get_user_role, get_permissions_for_role
from ..metrics_prometheus import (
    track_auth_login,
    track_auth_token_operation,
    track_auth_password_change
)
from ..logging_config import get_logger

logger = get_logger(__name__, component="auth")


router = APIRouter(
    prefix="/auth",
    tags=["authentication"],
)

# Security scheme
security = HTTPBearer()


# Simple in-memory user database for MVP
# In production: use proper database (PostgreSQL, MongoDB, etc.)
USERS_DB: Dict[str, UserInDB] = {
    "admin": UserInDB(
        user_id="admin",
        username="admin",
        email="admin@neurograph.dev",
        full_name="Administrator",
        role="admin",
        scopes=[],  # Populated dynamically
        hashed_password=hashlib.sha256("admin123".encode()).hexdigest(),
    ),
    "developer": UserInDB(
        user_id="developer",
        username="developer",
        email="dev@neurograph.dev",
        full_name="Developer User",
        role="developer",
        scopes=[],
        hashed_password=hashlib.sha256("developer123".encode()).hexdigest(),
    ),
    "viewer": UserInDB(
        user_id="viewer",
        username="viewer",
        email="viewer@neurograph.dev",
        full_name="Viewer User",
        role="viewer",
        scopes=[],
        hashed_password=hashlib.sha256("viewer123".encode()).hexdigest(),
    ),
}


def verify_password(plain_password: str, hashed_password: str) -> bool:
    """Verify password against hash."""
    return hashlib.sha256(plain_password.encode()).hexdigest() == hashed_password


def authenticate_user(username: str, password: str) -> UserInDB | None:
    """Authenticate user by username and password."""
    user = USERS_DB.get(username)
    if not user:
        return None
    if not verify_password(password, user.hashed_password):
        return None
    return user


@router.post(
    "/login",
    response_model=LoginResponse,
    status_code=status.HTTP_200_OK,
    summary="User login",
    description="Authenticate user and receive JWT tokens",
    responses={
        200: {"description": "Login successful"},
        401: {"model": ErrorResponse, "description": "Invalid credentials"},
    },
)
async def login(request: LoginRequest) -> LoginResponse:
    """
    Authenticate user and return JWT tokens.

    **Flow:**
    1. Validate username and password
    2. Generate access token (15 min) and refresh token (7 days)
    3. Return tokens with user profile

    **Example:**
    ```bash
    curl -X POST http://localhost:8000/api/v1/auth/login \\
      -H "Content-Type: application/json" \\
      -d '{"username": "developer", "password": "developer123"}'
    ```

    **Default users:**
    - `admin` / `admin123` - Full access
    - `developer` / `developer123` - Read/Write access
    - `viewer` / `viewer123` - Read-only access
    """
    start_time = time.perf_counter()

    # Authenticate user
    user = authenticate_user(request.username, request.password)
    if not user:
        duration = time.perf_counter() - start_time
        track_auth_login("invalid_credentials", duration)
        logger.warning(
            f"Failed login attempt for user: {request.username}",
            extra={"event": "login_failed", "username": request.username}
        )
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Incorrect username or password",
            headers={"WWW-Authenticate": "Bearer"},
        )

    # Check if user disabled
    if user.disabled:
        duration = time.perf_counter() - start_time
        track_auth_login("user_disabled", duration)
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="User account is disabled",
        )

    # Get user role and permissions
    role = get_user_role(user.user_id)
    scopes = get_permissions_for_role(role)

    # Generate tokens
    track_auth_token_operation("generate")
    access_token = jwt_manager.create_access_token(
        user_id=user.user_id,
        scopes=scopes,
    )
    refresh_token = jwt_manager.create_refresh_token(
        user_id=user.user_id,
    )

    # Track successful login
    duration = time.perf_counter() - start_time
    track_auth_login("success", duration)
    logger.info(
        f"Successful login for user: {user.username}",
        extra={"event": "login_success", "user_id": user.user_id, "role": role.value}
    )

    # Create user response (without password)
    user_response = User(
        user_id=user.user_id,
        username=user.username,
        email=user.email,
        full_name=user.full_name,
        role=role.value,
        scopes=scopes,
        disabled=user.disabled,
        created_at=user.created_at,
    )

    return LoginResponse(
        access_token=access_token,
        refresh_token=refresh_token,
        token_type="bearer",
        expires_in=jwt_manager.access_token_expire_minutes * 60,
        user=user_response,
    )


@router.post(
    "/refresh",
    response_model=RefreshTokenResponse,
    status_code=status.HTTP_200_OK,
    summary="Refresh access token",
    description="Get new access token using refresh token",
    responses={
        200: {"description": "Token refreshed"},
        401: {"model": ErrorResponse, "description": "Invalid refresh token"},
    },
)
async def refresh_token(request: RefreshTokenRequest) -> RefreshTokenResponse:
    """
    Refresh access token using refresh token.

    **Flow:**
    1. Validate refresh token
    2. Generate new access token and refresh token
    3. Return new tokens

    **Note:** Refresh tokens are rotated for security.

    **Example:**
    ```bash
    curl -X POST http://localhost:8000/api/v1/auth/refresh \\
      -H "Content-Type: application/json" \\
      -d '{"refresh_token": "eyJ0eXAi..."}'
    ```
    """
    try:
        tokens = jwt_manager.refresh_access_token(request.refresh_token)
        return RefreshTokenResponse(
            access_token=tokens["access_token"],
            refresh_token=tokens["refresh_token"],
            token_type="bearer",
            expires_in=jwt_manager.access_token_expire_minutes * 60,
        )
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail=f"Invalid refresh token: {str(e)}",
            headers={"WWW-Authenticate": "Bearer"},
        )


@router.post(
    "/logout",
    response_model=SuccessResponse,
    status_code=status.HTTP_200_OK,
    summary="User logout",
    description="Logout user and revoke tokens",
    responses={
        200: {"description": "Logout successful"},
    },
)
async def logout(
    request: LogoutRequest,
    current_user: User = Depends(get_current_active_user),
) -> SuccessResponse:
    """
    Logout user and revoke tokens.

    **Flow:**
    1. Revoke access token (add to blacklist)
    2. Optionally revoke refresh token

    **Note:** Client should delete tokens from storage.

    **Example:**
    ```bash
    curl -X POST http://localhost:8000/api/v1/auth/logout \\
      -H "Authorization: Bearer eyJ0eXAi..." \\
      -H "Content-Type: application/json" \\
      -d '{}'
    ```
    """
    # Revoke token if provided
    if request.token:
        jwt_manager.revoke_token(request.token)

    return SuccessResponse(
        success=True,
        message=f"User {current_user.username} logged out successfully",
    )


@router.get(
    "/me",
    response_model=User,
    status_code=status.HTTP_200_OK,
    summary="Get current user",
    description="Get current user profile",
    responses={
        200: {"description": "User profile"},
        401: {"model": ErrorResponse, "description": "Not authenticated"},
    },
)
async def read_users_me(
    current_user: User = Depends(get_current_active_user),
) -> User:
    """
    Get current user profile.

    **Returns user information:**
    - User ID
    - Username
    - Email
    - Role
    - Permissions (scopes)

    **Example:**
    ```bash
    curl -X GET http://localhost:8000/api/v1/auth/me \\
      -H "Authorization: Bearer eyJ0eXAi..."
    ```
    """
    return current_user


@router.post(
    "/change-password",
    response_model=SuccessResponse,
    status_code=status.HTTP_200_OK,
    summary="Change password",
    description="Change user password",
    responses={
        200: {"description": "Password changed"},
        400: {"model": ErrorResponse, "description": "Invalid old password"},
        401: {"model": ErrorResponse, "description": "Not authenticated"},
    },
)
async def change_password(
    request: ChangePasswordRequest,
    current_user: User = Depends(get_current_active_user),
) -> SuccessResponse:
    """
    Change user password.

    **Flow:**
    1. Verify old password
    2. Hash new password
    3. Update password in database

    **Example:**
    ```bash
    curl -X POST http://localhost:8000/api/v1/auth/change-password \\
      -H "Authorization: Bearer eyJ0eXAi..." \\
      -H "Content-Type: application/json" \\
      -d '{"old_password": "old123", "new_password": "new456"}'
    ```
    """
    # Get user from database
    user_in_db = USERS_DB.get(current_user.username)
    if not user_in_db:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="User not found",
        )

    # Verify old password
    if not verify_password(request.old_password, user_in_db.hashed_password):
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Incorrect old password",
        )

    # Hash new password
    new_hashed = hashlib.sha256(request.new_password.encode()).hexdigest()

    # Update password
    user_in_db.hashed_password = new_hashed
    USERS_DB[current_user.username] = user_in_db

    return SuccessResponse(
        success=True,
        message="Password changed successfully",
    )
