"""
Synchronous NeuroGraph Client

Main client class for interacting with NeuroGraph REST API.
"""

from typing import Optional, List, Dict, Any
import httpx

from .auth import AuthManager
from .exceptions import (
    NeuroGraphError,
    AuthenticationError,
    AuthorizationError,
    NotFoundError,
    ValidationError,
    RateLimitError,
    ServerError,
    ConnectionError as ClientConnectionError,
    TimeoutError as ClientTimeoutError,
)
from .models import (
    Token,
    TokenCreate,
    TokenUpdate,
    TokenQuery,
    TokenQueryResult,
    APIKey,
    APIKeyCreate,
    APIKeyCreated,
    User,
    HealthStatus,
    SystemStatus,
)


class NeuroGraphClient:
    """
    Synchronous client for NeuroGraph REST API.

    Supports two authentication methods:
    1. JWT (username/password)
    2. API Key

    Example with JWT:
        >>> client = NeuroGraphClient(
        ...     base_url="http://localhost:8000",
        ...     username="developer",
        ...     password="developer123"
        ... )
        >>> token = client.tokens.create(text="hello world")

    Example with API Key:
        >>> client = NeuroGraphClient(
        ...     base_url="http://localhost:8000",
        ...     api_key="ng_1234567890abcdef"
        ... )
        >>> tokens = client.tokens.list()

    Args:
        base_url: Base URL of NeuroGraph API
        username: Username for JWT authentication
        password: Password for JWT authentication
        api_key: API key for authentication
        timeout: Request timeout in seconds (default: 30)
        verify_ssl: Verify SSL certificates (default: True)
    """

    def __init__(
        self,
        base_url: str,
        username: Optional[str] = None,
        password: Optional[str] = None,
        api_key: Optional[str] = None,
        timeout: float = 30.0,
        verify_ssl: bool = True,
    ):
        self.base_url = base_url.rstrip("/")
        self.timeout = timeout

        # Initialize HTTP client
        self._client = httpx.Client(
            timeout=timeout,
            verify=verify_ssl,
            follow_redirects=True,
        )

        # Initialize auth manager
        self._auth = AuthManager(
            base_url=base_url,
            username=username,
            password=password,
            api_key=api_key,
            client=self._client,
        )

        # Initialize resource clients
        self.tokens = TokensClient(self)
        self.api_keys = APIKeysClient(self)
        self.health = HealthClient(self)

    def _request(
        self,
        method: str,
        path: str,
        **kwargs,
    ) -> httpx.Response:
        """
        Make HTTP request with authentication.

        Args:
            method: HTTP method (GET, POST, etc.)
            path: API path
            **kwargs: Additional arguments for httpx.request

        Returns:
            httpx.Response object

        Raises:
            NeuroGraphError: On API errors
        """
        url = f"{self.base_url}{path}"

        # Add authentication headers
        headers = kwargs.pop("headers", {})
        headers.update(self._auth.get_auth_headers())

        try:
            response = self._client.request(
                method=method,
                url=url,
                headers=headers,
                **kwargs,
            )

            # Handle errors
            if response.status_code >= 400:
                self._handle_error_response(response)

            return response

        except httpx.TimeoutException as e:
            raise ClientTimeoutError(f"Request timeout: {e}")
        except httpx.ConnectError as e:
            raise ClientConnectionError(f"Connection failed: {e}")
        except Exception as e:
            if isinstance(e, NeuroGraphError):
                raise
            raise NeuroGraphError(f"Request failed: {e}")

    def _handle_error_response(self, response: httpx.Response):
        """Handle error responses from API."""
        try:
            data = response.json()
            error = data.get("error", {})
            message = error.get("message", response.text)
            error_code = error.get("code")
            details = error.get("details", {})
        except Exception:
            message = response.text
            error_code = None
            details = {}

        status_code = response.status_code

        if status_code == 401:
            raise AuthenticationError(message, status_code, error_code, details)
        elif status_code == 403:
            raise AuthorizationError(message, status_code, error_code, details)
        elif status_code == 404:
            raise NotFoundError(message, status_code, error_code, details)
        elif status_code == 422:
            raise ValidationError(message, status_code, error_code, details)
        elif status_code == 429:
            retry_after = response.headers.get("Retry-After")
            raise RateLimitError(
                message,
                status_code,
                error_code,
                details,
                retry_after=int(retry_after) if retry_after else None,
            )
        elif status_code >= 500:
            raise ServerError(message, status_code, error_code, details)
        else:
            raise NeuroGraphError(message, status_code, error_code, details)

    def close(self):
        """Close the HTTP client."""
        self._client.close()

    def __enter__(self):
        """Context manager entry."""
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit."""
        self.close()


class TokensClient:
    """Client for token operations."""

    def __init__(self, client: NeuroGraphClient):
        self._client = client

    def create(self, text: str, metadata: Optional[Dict[str, Any]] = None) -> Token:
        """
        Create a new token.

        Args:
            text: Text to create token from
            metadata: Optional metadata

        Returns:
            Created token

        Example:
            >>> token = client.tokens.create(text="hello world")
            >>> print(token.id, token.embedding)
        """
        response = self._client._request(
            "POST",
            "/api/v1/tokens",
            json={"text": text, "metadata": metadata},
        )
        return Token(**response.json())

    def get(self, token_id: int) -> Token:
        """
        Get token by ID.

        Args:
            token_id: Token ID

        Returns:
            Token object

        Raises:
            NotFoundError: If token doesn't exist
        """
        response = self._client._request("GET", f"/api/v1/tokens/{token_id}")
        return Token(**response.json())

    def list(
        self,
        limit: int = 100,
        offset: int = 0,
    ) -> List[Token]:
        """
        List tokens.

        Args:
            limit: Maximum number of tokens to return
            offset: Offset for pagination

        Returns:
            List of tokens
        """
        response = self._client._request(
            "GET",
            "/api/v1/tokens",
            params={"limit": limit, "offset": offset},
        )
        data = response.json()
        return [Token(**item) for item in data.get("tokens", [])]

    def update(
        self,
        token_id: int,
        text: Optional[str] = None,
        embedding: Optional[List[float]] = None,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> Token:
        """
        Update token.

        Args:
            token_id: Token ID
            text: New text
            embedding: New embedding
            metadata: New metadata

        Returns:
            Updated token
        """
        update_data = {}
        if text is not None:
            update_data["text"] = text
        if embedding is not None:
            update_data["embedding"] = embedding
        if metadata is not None:
            update_data["metadata"] = metadata

        response = self._client._request(
            "PUT",
            f"/api/v1/tokens/{token_id}",
            json=update_data,
        )
        return Token(**response.json())

    def delete(self, token_id: int) -> bool:
        """
        Delete token.

        Args:
            token_id: Token ID

        Returns:
            True if deleted successfully
        """
        self._client._request("DELETE", f"/api/v1/tokens/{token_id}")
        return True

    def query(
        self,
        query_vector: List[float],
        top_k: int = 10,
        threshold: Optional[float] = None,
    ) -> List[TokenQueryResult]:
        """
        Query similar tokens.

        Args:
            query_vector: Query embedding vector
            top_k: Number of results to return
            threshold: Similarity threshold

        Returns:
            List of query results with similarity scores
        """
        query_data = {
            "query_vector": query_vector,
            "top_k": top_k,
        }
        if threshold is not None:
            query_data["threshold"] = threshold

        response = self._client._request(
            "POST",
            "/api/v1/tokens/query",
            json=query_data,
        )
        data = response.json()
        return [TokenQueryResult(**item) for item in data.get("results", [])]


class APIKeysClient:
    """Client for API key operations."""

    def __init__(self, client: NeuroGraphClient):
        self._client = client

    def create(
        self,
        name: str,
        scopes: Optional[List[str]] = None,
        expires_in_days: Optional[int] = None,
    ) -> APIKeyCreated:
        """
        Create a new API key.

        WARNING: The full API key is only returned once!
        Make sure to save it securely.

        Args:
            name: Descriptive name for the key
            scopes: List of scopes/permissions
            expires_in_days: Expiration in days (None = no expiration)

        Returns:
            Created API key (includes full key)

        Example:
            >>> key = client.api_keys.create(
            ...     name="My Integration",
            ...     scopes=["tokens:read", "tokens:write"]
            ... )
            >>> print(f"Save this key: {key.api_key}")
        """
        response = self._client._request(
            "POST",
            "/api/v1/api-keys",
            json={
                "name": name,
                "scopes": scopes,
                "expires_in_days": expires_in_days,
            },
        )
        return APIKeyCreated(**response.json())

    def list(self) -> List[APIKey]:
        """
        List all API keys for current user.

        Returns:
            List of API keys (without full key values)
        """
        response = self._client._request("GET", "/api/v1/api-keys")
        data = response.json()
        return [APIKey(**item) for item in data.get("api_keys", [])]

    def get(self, key_id: str) -> APIKey:
        """
        Get API key by ID.

        Args:
            key_id: API key ID

        Returns:
            API key (without full key value)
        """
        response = self._client._request("GET", f"/api/v1/api-keys/{key_id}")
        return APIKey(**response.json())

    def revoke(self, key_id: str) -> bool:
        """
        Revoke API key.

        Args:
            key_id: API key ID

        Returns:
            True if revoked successfully
        """
        self._client._request("POST", f"/api/v1/api-keys/{key_id}/revoke")
        return True

    def delete(self, key_id: str) -> bool:
        """
        Delete API key.

        Args:
            key_id: API key ID

        Returns:
            True if deleted successfully
        """
        self._client._request("DELETE", f"/api/v1/api-keys/{key_id}")
        return True


class HealthClient:
    """Client for health and status checks."""

    def __init__(self, client: NeuroGraphClient):
        self._client = client

    def check(self) -> HealthStatus:
        """
        Check API health.

        Returns:
            Health status

        Example:
            >>> health = client.health.check()
            >>> print(health.status, health.version)
        """
        response = self._client._request("GET", "/api/v1/health")
        return HealthStatus(**response.json())

    def status(self) -> SystemStatus:
        """
        Get system status.

        Returns:
            System status with metrics

        Example:
            >>> status = client.health.status()
            >>> print(f"Tokens: {status.tokens_count}")
        """
        response = self._client._request("GET", "/api/v1/status")
        return SystemStatus(**response.json())
