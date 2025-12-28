"""
JWT token management for NeuroGraph API.

Provides secure token generation, validation, and refresh functionality.
"""

import os
from datetime import datetime, timedelta
from typing import Optional, Dict, Any, List

import jwt
from pydantic import BaseModel, Field


class TokenPayload(BaseModel):
    """JWT token payload structure."""

    sub: str = Field(..., description="Subject (user ID)")
    scopes: List[str] = Field(default_factory=list, description="Permission scopes")
    exp: datetime = Field(..., description="Expiration time")
    iat: datetime = Field(..., description="Issued at")
    jti: Optional[str] = Field(None, description="JWT ID (for revocation)")
    token_type: str = Field("access", description="Token type (access or refresh)")


class JWTManager:
    """
    Manages JWT token lifecycle.

    Features:
    - Access token generation (short-lived, 15 min)
    - Refresh token generation (long-lived, 7 days)
    - Token validation and decoding
    - Token revocation support (via JTI)
    """

    def __init__(
        self,
        secret_key: Optional[str] = None,
        algorithm: str = "HS256",
        access_token_expire_minutes: int = 15,
        refresh_token_expire_days: int = 7,
    ):
        """
        Initialize JWT manager.

        Args:
            secret_key: Secret key for signing tokens (from env if None)
            algorithm: JWT algorithm (default: HS256)
            access_token_expire_minutes: Access token lifetime (default: 15 min)
            refresh_token_expire_days: Refresh token lifetime (default: 7 days)
        """
        self.secret_key = secret_key or os.getenv(
            "JWT_SECRET_KEY", "CHANGE_THIS_SECRET_KEY_IN_PRODUCTION"
        )
        self.algorithm = algorithm
        self.access_token_expire_minutes = access_token_expire_minutes
        self.refresh_token_expire_days = refresh_token_expire_days

        # Revoked tokens storage (in production use Redis/database)
        self._revoked_tokens: set = set()

    def create_access_token(
        self,
        user_id: str,
        scopes: Optional[List[str]] = None,
        expires_delta: Optional[timedelta] = None,
    ) -> str:
        """
        Create access token.

        Args:
            user_id: User identifier
            scopes: List of permission scopes
            expires_delta: Custom expiration time (overrides default)

        Returns:
            Encoded JWT access token

        Example:
            >>> manager = JWTManager()
            >>> token = manager.create_access_token("user123", ["tokens:read"])
        """
        if scopes is None:
            scopes = []

        now = datetime.utcnow()
        if expires_delta:
            expire = now + expires_delta
        else:
            expire = now + timedelta(minutes=self.access_token_expire_minutes)

        payload = {
            "sub": user_id,
            "scopes": scopes,
            "exp": expire,
            "iat": now,
            "token_type": "access",
        }

        return jwt.encode(payload, self.secret_key, algorithm=self.algorithm)

    def create_refresh_token(
        self,
        user_id: str,
        expires_delta: Optional[timedelta] = None,
    ) -> str:
        """
        Create refresh token.

        Args:
            user_id: User identifier
            expires_delta: Custom expiration time (overrides default)

        Returns:
            Encoded JWT refresh token

        Example:
            >>> manager = JWTManager()
            >>> refresh = manager.create_refresh_token("user123")
        """
        now = datetime.utcnow()
        if expires_delta:
            expire = now + expires_delta
        else:
            expire = now + timedelta(days=self.refresh_token_expire_days)

        payload = {
            "sub": user_id,
            "scopes": [],  # Refresh tokens don't have scopes
            "exp": expire,
            "iat": now,
            "token_type": "refresh",
        }

        return jwt.encode(payload, self.secret_key, algorithm=self.algorithm)

    def verify_token(self, token: str, expected_type: str = "access") -> TokenPayload:
        """
        Verify and decode token.

        Args:
            token: JWT token string
            expected_type: Expected token type ("access" or "refresh")

        Returns:
            Decoded token payload

        Raises:
            jwt.ExpiredSignatureError: Token expired
            jwt.InvalidTokenError: Token invalid
            ValueError: Token type mismatch or token revoked

        Example:
            >>> manager = JWTManager()
            >>> payload = manager.verify_token(token)
            >>> print(payload.sub)  # user ID
        """
        try:
            payload = jwt.decode(
                token, self.secret_key, algorithms=[self.algorithm]
            )
        except jwt.ExpiredSignatureError:
            raise jwt.ExpiredSignatureError("Token expired")
        except jwt.InvalidTokenError as e:
            raise jwt.InvalidTokenError(f"Invalid token: {e}")

        # Check token type
        token_type = payload.get("token_type", "access")
        if token_type != expected_type:
            raise ValueError(
                f"Invalid token type: expected {expected_type}, got {token_type}"
            )

        # Check if revoked
        jti = payload.get("jti")
        if jti and jti in self._revoked_tokens:
            raise ValueError("Token has been revoked")

        # Convert to TokenPayload
        return TokenPayload(
            sub=payload["sub"],
            scopes=payload.get("scopes", []),
            exp=datetime.fromtimestamp(payload["exp"]),
            iat=datetime.fromtimestamp(payload["iat"]),
            jti=jti,
            token_type=token_type,
        )

    def refresh_access_token(self, refresh_token: str) -> Dict[str, str]:
        """
        Create new access token from refresh token.

        Args:
            refresh_token: Valid refresh token

        Returns:
            Dictionary with new access and refresh tokens

        Raises:
            ValueError: Invalid refresh token

        Example:
            >>> manager = JWTManager()
            >>> tokens = manager.refresh_access_token(refresh_token)
            >>> print(tokens["access_token"])
        """
        # Verify refresh token
        payload = self.verify_token(refresh_token, expected_type="refresh")

        # Create new access token
        access_token = self.create_access_token(
            user_id=payload.sub,
            scopes=[]  # Get scopes from user profile in production
        )

        # Create new refresh token (rotate refresh tokens)
        new_refresh_token = self.create_refresh_token(user_id=payload.sub)

        return {
            "access_token": access_token,
            "refresh_token": new_refresh_token,
            "token_type": "bearer",
        }

    def revoke_token(self, token: str) -> None:
        """
        Revoke token (add to blacklist).

        Args:
            token: JWT token to revoke

        Note:
            In production, use Redis or database for revocation storage.

        Example:
            >>> manager = JWTManager()
            >>> manager.revoke_token(token)
        """
        try:
            payload = jwt.decode(
                token, self.secret_key, algorithms=[self.algorithm]
            )
            jti = payload.get("jti")
            if jti:
                self._revoked_tokens.add(jti)
        except jwt.InvalidTokenError:
            # Ignore invalid tokens
            pass

    def decode_token_unsafe(self, token: str) -> Dict[str, Any]:
        """
        Decode token without verification (for debugging only).

        Args:
            token: JWT token

        Returns:
            Decoded payload (not verified!)

        Warning:
            Do NOT use for authentication! Only for debugging.

        Example:
            >>> manager = JWTManager()
            >>> payload = manager.decode_token_unsafe(token)
        """
        return jwt.decode(
            token,
            options={"verify_signature": False, "verify_exp": False},
            algorithms=[self.algorithm],
        )


# Global JWT manager instance
jwt_manager = JWTManager()
