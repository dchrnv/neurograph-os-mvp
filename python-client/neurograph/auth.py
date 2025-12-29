"""
Authentication handling for NeuroGraph client.
"""

from typing import Optional, Dict
from datetime import datetime, timedelta
import httpx

from .exceptions import AuthenticationError
from .models import LoginResponse


class AuthManager:
    """
    Manages authentication for NeuroGraph client.

    Supports two authentication methods:
    1. JWT (username/password) - with automatic token refresh
    2. API Key - long-lived authentication
    """

    def __init__(
        self,
        base_url: str,
        username: Optional[str] = None,
        password: Optional[str] = None,
        api_key: Optional[str] = None,
        client: Optional[httpx.Client] = None,
    ):
        self.base_url = base_url.rstrip("/")
        self.username = username
        self.password = password
        self.api_key = api_key
        self._client = client

        # JWT state
        self._access_token: Optional[str] = None
        self._refresh_token: Optional[str] = None
        self._token_expires_at: Optional[datetime] = None

    def get_auth_headers(self) -> Dict[str, str]:
        """
        Get authentication headers for requests.

        Returns:
            Dict with Authorization header
        """
        if self.api_key:
            # API Key authentication
            return {"Authorization": f"Bearer {self.api_key}"}

        if self._access_token:
            # JWT authentication
            # Check if token needs refresh
            if self._should_refresh_token():
                self._refresh_access_token()

            return {"Authorization": f"Bearer {self._access_token}"}

        # No authentication yet - need to login
        if self.username and self.password:
            self._login()
            return {"Authorization": f"Bearer {self._access_token}"}

        raise AuthenticationError("No authentication credentials provided")

    def _should_refresh_token(self) -> bool:
        """Check if access token should be refreshed."""
        if not self._token_expires_at:
            return False

        # Refresh 30 seconds before expiry
        return datetime.now() + timedelta(seconds=30) >= self._token_expires_at

    def _login(self):
        """Perform JWT login."""
        if not self._client:
            raise AuthenticationError("HTTP client not initialized")

        response = self._client.post(
            f"{self.base_url}/api/v1/auth/login",
            json={"username": self.username, "password": self.password},
        )

        if response.status_code != 200:
            raise AuthenticationError(
                f"Login failed: {response.text}",
                status_code=response.status_code,
            )

        data = LoginResponse(**response.json())
        self._access_token = data.access_token
        self._refresh_token = data.refresh_token
        self._token_expires_at = datetime.now() + timedelta(seconds=data.expires_in)

    def _refresh_access_token(self):
        """Refresh the access token using refresh token."""
        if not self._client or not self._refresh_token:
            # Fall back to re-login
            self._login()
            return

        try:
            response = self._client.post(
                f"{self.base_url}/api/v1/auth/refresh",
                json={"refresh_token": self._refresh_token},
            )

            if response.status_code != 200:
                # Refresh failed, re-login
                self._login()
                return

            data = response.json()
            self._access_token = data["access_token"]
            self._token_expires_at = datetime.now() + timedelta(seconds=data["expires_in"])

        except Exception:
            # Any error during refresh - re-login
            self._login()

    def logout(self):
        """Logout and invalidate tokens."""
        if self._client and self._access_token:
            try:
                self._client.post(
                    f"{self.base_url}/api/v1/auth/logout",
                    headers={"Authorization": f"Bearer {self._access_token}"},
                    json={"access_token": self._access_token},
                )
            except Exception:
                pass  # Ignore logout errors

        # Clear tokens
        self._access_token = None
        self._refresh_token = None
        self._token_expires_at = None
