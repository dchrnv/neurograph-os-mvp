"""
Pydantic models for authentication and authorization.
"""

from datetime import datetime
from typing import Optional, List

from pydantic import BaseModel, Field, EmailStr


class User(BaseModel):
    """User model."""

    user_id: str = Field(..., description="Unique user identifier")
    username: str = Field(..., description="Username")
    email: Optional[EmailStr] = Field(None, description="User email")
    full_name: Optional[str] = Field(None, description="Full name")
    role: str = Field("viewer", description="User role (admin, developer, viewer, bot)")
    scopes: List[str] = Field(default_factory=list, description="Permission scopes")
    disabled: bool = Field(False, description="Whether user is disabled")
    created_at: datetime = Field(default_factory=datetime.utcnow)


class UserInDB(User):
    """User model with hashed password (for database storage)."""

    hashed_password: str = Field(..., description="Hashed password")


class LoginRequest(BaseModel):
    """Login request body."""

    username: str = Field(..., description="Username", min_length=3, max_length=50)
    password: str = Field(..., description="Password", min_length=8)

    class Config:
        json_schema_extra = {
            "example": {
                "username": "developer",
                "password": "securepassword123",
            }
        }


class LoginResponse(BaseModel):
    """Login response with tokens."""

    access_token: str = Field(..., description="JWT access token")
    refresh_token: str = Field(..., description="JWT refresh token")
    token_type: str = Field("bearer", description="Token type")
    expires_in: int = Field(..., description="Access token expiration (seconds)")
    user: User = Field(..., description="User information")

    class Config:
        json_schema_extra = {
            "example": {
                "access_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
                "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
                "token_type": "bearer",
                "expires_in": 900,
                "user": {
                    "user_id": "user123",
                    "username": "developer",
                    "role": "developer",
                    "scopes": ["tokens:read", "tokens:write"],
                },
            }
        }


class RefreshTokenRequest(BaseModel):
    """Refresh token request."""

    refresh_token: str = Field(..., description="JWT refresh token")

    class Config:
        json_schema_extra = {
            "example": {
                "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
            }
        }


class RefreshTokenResponse(BaseModel):
    """Refresh token response."""

    access_token: str = Field(..., description="New JWT access token")
    refresh_token: str = Field(..., description="New JWT refresh token")
    token_type: str = Field("bearer", description="Token type")
    expires_in: int = Field(..., description="Access token expiration (seconds)")

    class Config:
        json_schema_extra = {
            "example": {
                "access_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
                "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
                "token_type": "bearer",
                "expires_in": 900,
            }
        }


class LogoutRequest(BaseModel):
    """Logout request (revoke token)."""

    token: Optional[str] = Field(None, description="Token to revoke (optional)")

    class Config:
        json_schema_extra = {
            "example": {
                "token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
            }
        }


class APIKey(BaseModel):
    """API key model."""

    key_id: str = Field(..., description="API key ID")
    key_prefix: str = Field(..., description="First 8 characters of key (for display)")
    name: str = Field(..., description="Human-readable key name")
    scopes: List[str] = Field(default_factory=list, description="Permission scopes")
    rate_limit: int = Field(1000, description="Requests per minute limit")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    expires_at: Optional[datetime] = Field(None, description="Expiration time")
    last_used_at: Optional[datetime] = Field(None, description="Last usage time")
    disabled: bool = Field(False, description="Whether key is disabled")


class APIKeyCreate(BaseModel):
    """API key creation request."""

    name: str = Field(..., description="Human-readable key name", min_length=3)
    scopes: List[str] = Field(default_factory=list, description="Permission scopes")
    rate_limit: int = Field(1000, description="Requests per minute limit", gt=0, le=10000)
    expires_in_days: Optional[int] = Field(None, description="Expiration in days (None = no expiration)", gt=0, le=365)

    class Config:
        json_schema_extra = {
            "example": {
                "name": "Production Bot",
                "scopes": ["tokens:write", "health:read"],
                "rate_limit": 500,
                "expires_in_days": 90,
            }
        }


class APIKeyCreateResponse(BaseModel):
    """API key creation response (includes secret only once)."""

    key_id: str = Field(..., description="API key ID")
    key_secret: str = Field(..., description="API key secret (SAVE THIS - shown only once!)")
    api_key: APIKey = Field(..., description="API key metadata")

    class Config:
        json_schema_extra = {
            "example": {
                "key_id": "ak_123456",
                "key_secret": "sk_abc123def456ghi789jkl012mno345pqr678",
                "api_key": {
                    "key_id": "ak_123456",
                    "key_prefix": "sk_abc12",
                    "name": "Production Bot",
                    "scopes": ["tokens:write"],
                    "rate_limit": 500,
                },
            }
        }


class ChangePasswordRequest(BaseModel):
    """Change password request."""

    old_password: str = Field(..., description="Current password")
    new_password: str = Field(..., description="New password", min_length=8)

    class Config:
        json_schema_extra = {
            "example": {
                "old_password": "oldpassword123",
                "new_password": "newsecurepassword456",
            }
        }
