"""
Testing utilities and mock clients for NeuroGraph.

Provides mock clients and test fixtures for unit testing applications
that use NeuroGraph without requiring a live API server.
"""

from typing import List, Optional, Dict, Any
from datetime import datetime
import random

from .models import Token, TokenCreate, APIKey, APIKeyCreate, HealthCheck, SystemStatus, QueryResult


class MockNeuroGraphClient:
    """
    Mock NeuroGraph client for testing.

    Simulates NeuroGraph API responses without requiring a live server.
    Useful for unit tests and CI/CD pipelines.

    Example:
        >>> from neurograph.testing import MockNeuroGraphClient
        >>> client = MockNeuroGraphClient()
        >>> token = client.tokens.create(text="test")
        >>> assert token.id > 0
        >>> assert token.text == "test"
    """

    def __init__(self, **kwargs):
        """Initialize mock client."""
        self._tokens_store: Dict[int, Token] = {}
        self._api_keys_store: Dict[str, APIKey] = {}
        self._next_token_id = 1
        self._next_key_id = 1

        self.tokens = self.MockTokensClient(self)
        self.api_keys = self.MockAPIKeysClient(self)
        self.health = self.MockHealthClient(self)

    def close(self):
        """Close client (no-op for mock)."""
        pass

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()

    class MockTokensClient:
        """Mock tokens resource."""

        def __init__(self, parent):
            self.parent = parent

        def create(self, text: str, metadata: Optional[Dict[str, Any]] = None) -> Token:
            """Create mock token."""
            token_id = self.parent._next_token_id
            self.parent._next_token_id += 1

            # Generate fake embedding
            embedding = [random.random() for _ in range(768)]

            token = Token(
                id=token_id,
                text=text,
                embedding=embedding,
                metadata=metadata or {},
                created_at=datetime.utcnow().isoformat(),
                updated_at=datetime.utcnow().isoformat()
            )

            self.parent._tokens_store[token_id] = token
            return token

        def get(self, token_id: int) -> Token:
            """Get mock token."""
            from .exceptions import NotFoundError

            if token_id not in self.parent._tokens_store:
                raise NotFoundError(
                    f"Token {token_id} not found",
                    "TOKEN_NOT_FOUND"
                )

            return self.parent._tokens_store[token_id]

        def list(self, limit: int = 100, offset: int = 0) -> List[Token]:
            """List mock tokens."""
            tokens = list(self.parent._tokens_store.values())
            return tokens[offset:offset + limit]

        def update(
            self,
            token_id: int,
            text: Optional[str] = None,
            metadata: Optional[Dict[str, Any]] = None
        ) -> Token:
            """Update mock token."""
            token = self.get(token_id)

            if text is not None:
                token.text = text
                # Re-generate embedding for new text
                token.embedding = [random.random() for _ in range(768)]

            if metadata is not None:
                token.metadata = metadata

            token.updated_at = datetime.utcnow().isoformat()
            self.parent._tokens_store[token_id] = token
            return token

        def delete(self, token_id: int) -> bool:
            """Delete mock token."""
            from .exceptions import NotFoundError

            if token_id not in self.parent._tokens_store:
                raise NotFoundError(
                    f"Token {token_id} not found",
                    "TOKEN_NOT_FOUND"
                )

            del self.parent._tokens_store[token_id]
            return True

        def query(
            self,
            query_vector: List[float],
            top_k: int = 10,
            threshold: Optional[float] = None
        ) -> List[QueryResult]:
            """Query mock tokens."""
            # Simple mock: return random tokens with random similarities
            tokens = list(self.parent._tokens_store.values())

            # Limit to top_k
            tokens = tokens[:top_k]

            # Generate results with mock similarities
            results = []
            for token in tokens:
                similarity = random.uniform(0.7, 1.0)

                if threshold is None or similarity >= threshold:
                    results.append(QueryResult(
                        token=token,
                        similarity=similarity
                    ))

            # Sort by similarity descending
            results.sort(key=lambda r: r.similarity, reverse=True)
            return results

    class MockAPIKeysClient:
        """Mock API keys resource."""

        def __init__(self, parent):
            self.parent = parent

        def create(
            self,
            name: str,
            scopes: List[str],
            expires_in_days: Optional[int] = None
        ) -> APIKey:
            """Create mock API key."""
            key_id = f"key_{self.parent._next_key_id:08d}"
            self.parent._next_key_id += 1

            api_key_str = f"ng_mock_{key_id}_{random.randint(10000, 99999)}"

            expires_at = None
            if expires_in_days:
                from datetime import timedelta
                expires_at = (datetime.utcnow() + timedelta(days=expires_in_days)).isoformat()

            api_key = APIKey(
                key_id=key_id,
                name=name,
                scopes=scopes,
                created_at=datetime.utcnow().isoformat(),
                expires_at=expires_at,
                last_used_at=None,
                is_active=True,
                api_key=api_key_str  # Only returned on creation
            )

            self.parent._api_keys_store[key_id] = api_key
            return api_key

        def list(self) -> List[APIKey]:
            """List mock API keys."""
            # Return copies without api_key field
            return [
                APIKey(
                    key_id=k.key_id,
                    name=k.name,
                    scopes=k.scopes,
                    created_at=k.created_at,
                    expires_at=k.expires_at,
                    last_used_at=k.last_used_at,
                    is_active=k.is_active,
                    api_key=None
                )
                for k in self.parent._api_keys_store.values()
            ]

        def get(self, key_id: str) -> APIKey:
            """Get mock API key."""
            from .exceptions import NotFoundError

            if key_id not in self.parent._api_keys_store:
                raise NotFoundError(
                    f"API key {key_id} not found",
                    "API_KEY_NOT_FOUND"
                )

            key = self.parent._api_keys_store[key_id]
            # Return without api_key field
            return APIKey(
                key_id=key.key_id,
                name=key.name,
                scopes=key.scopes,
                created_at=key.created_at,
                expires_at=key.expires_at,
                last_used_at=key.last_used_at,
                is_active=key.is_active,
                api_key=None
            )

        def revoke(self, key_id: str) -> None:
            """Revoke mock API key."""
            key = self.get(key_id)
            self.parent._api_keys_store[key_id].is_active = False

        def delete(self, key_id: str) -> None:
            """Delete mock API key."""
            from .exceptions import NotFoundError

            if key_id not in self.parent._api_keys_store:
                raise NotFoundError(
                    f"API key {key_id} not found",
                    "API_KEY_NOT_FOUND"
                )

            del self.parent._api_keys_store[key_id]

    class MockHealthClient:
        """Mock health resource."""

        def __init__(self, parent):
            self.parent = parent

        def check(self) -> HealthCheck:
            """Mock health check."""
            return HealthCheck(
                status="healthy",
                version="0.59.0-mock",
                timestamp=datetime.utcnow().isoformat()
            )

        def status(self) -> SystemStatus:
            """Mock system status."""
            return SystemStatus(
                status="healthy",
                version="0.59.0-mock",
                timestamp=datetime.utcnow().isoformat(),
                uptime_seconds=12345,
                tokens_count=len(self.parent._tokens_store),
                api_keys_count=len(self.parent._api_keys_store)
            )


# Convenience fixtures for pytest
def mock_neurograph_client(**kwargs):
    """
    Create a mock NeuroGraph client for testing.

    Example:
        >>> def test_create_token():
        ...     client = mock_neurograph_client()
        ...     token = client.tokens.create(text="test")
        ...     assert token.text == "test"
    """
    return MockNeuroGraphClient(**kwargs)


def mock_token(**kwargs) -> Token:
    """
    Create a mock token for testing.

    Args:
        **kwargs: Override default token fields

    Example:
        >>> token = mock_token(text="custom text", id=123)
        >>> assert token.id == 123
        >>> assert token.text == "custom text"
    """
    defaults = {
        "id": 1,
        "text": "mock token text",
        "embedding": [random.random() for _ in range(768)],
        "metadata": {},
        "created_at": datetime.utcnow().isoformat(),
        "updated_at": datetime.utcnow().isoformat()
    }
    defaults.update(kwargs)
    return Token(**defaults)


def mock_api_key(**kwargs) -> APIKey:
    """
    Create a mock API key for testing.

    Args:
        **kwargs: Override default API key fields

    Example:
        >>> api_key = mock_api_key(name="test key", scopes=["tokens:read"])
        >>> assert api_key.name == "test key"
    """
    defaults = {
        "key_id": "key_00000001",
        "name": "mock api key",
        "scopes": ["tokens:read", "tokens:write"],
        "created_at": datetime.utcnow().isoformat(),
        "expires_at": None,
        "last_used_at": None,
        "is_active": True,
        "api_key": "ng_mock_key_12345"
    }
    defaults.update(kwargs)
    return APIKey(**defaults)
