"""
Tests for NeuroGraph synchronous client.

Run with: pytest tests/
"""

import pytest
from neurograph import NeuroGraphClient, AuthenticationError, NotFoundError
from neurograph.models import Token


# Skip tests if API is not running
pytestmark = pytest.mark.skipif(
    reason="API server not running (start with: uvicorn src.api.main:app)",
)


@pytest.fixture
def client():
    """Create test client."""
    client = NeuroGraphClient(
        base_url="http://localhost:8000",
        username="developer",
        password="developer123",
    )
    yield client
    client.close()


def test_health_check(client):
    """Test health check endpoint."""
    health = client.health.check()
    assert health.status == "healthy"
    assert health.version is not None


def test_create_token(client):
    """Test token creation."""
    token = client.tokens.create(text="test token")

    assert isinstance(token, Token)
    assert token.id is not None
    assert token.text == "test token"
    assert len(token.embedding) > 0

    # Cleanup
    client.tokens.delete(token.id)


def test_get_token(client):
    """Test get token."""
    # Create token
    created = client.tokens.create(text="get test")

    # Get token
    retrieved = client.tokens.get(created.id)

    assert retrieved.id == created.id
    assert retrieved.text == created.text

    # Cleanup
    client.tokens.delete(created.id)


def test_get_nonexistent_token(client):
    """Test get non-existent token raises NotFoundError."""
    with pytest.raises(NotFoundError):
        client.tokens.get(token_id=999999)


def test_list_tokens(client):
    """Test list tokens."""
    # Create some tokens
    tokens = [
        client.tokens.create(text=f"test {i}")
        for i in range(3)
    ]

    # List tokens
    listed = client.tokens.list(limit=10)

    assert len(listed) >= 3

    # Cleanup
    for token in tokens:
        client.tokens.delete(token.id)


def test_update_token(client):
    """Test update token."""
    # Create token
    token = client.tokens.create(text="original")

    # Update
    updated = client.tokens.update(
        token.id,
        metadata={"updated": True}
    )

    assert updated.id == token.id
    assert updated.metadata == {"updated": True}

    # Cleanup
    client.tokens.delete(token.id)


def test_delete_token(client):
    """Test delete token."""
    # Create token
    token = client.tokens.create(text="to delete")

    # Delete
    result = client.tokens.delete(token.id)
    assert result is True

    # Verify deleted
    with pytest.raises(NotFoundError):
        client.tokens.get(token.id)


def test_query_tokens(client):
    """Test query tokens."""
    # Create token
    token = client.tokens.create(text="query test")

    # Query
    results = client.tokens.query(
        query_vector=token.embedding,
        top_k=5
    )

    assert len(results) > 0
    assert results[0].token.id == token.id  # Should match itself
    assert results[0].similarity > 0.99  # Very similar to itself

    # Cleanup
    client.tokens.delete(token.id)


def test_authentication_error():
    """Test authentication with wrong credentials."""
    client = NeuroGraphClient(
        base_url="http://localhost:8000",
        username="wrong",
        password="wrong",
    )

    with pytest.raises(AuthenticationError):
        client.tokens.list()

    client.close()


def test_context_manager():
    """Test client as context manager."""
    with NeuroGraphClient(
        base_url="http://localhost:8000",
        username="developer",
        password="developer123",
    ) as client:
        health = client.health.check()
        assert health.status == "healthy"
    # Client should be closed after exit


def test_api_keys(client):
    """Test API key management."""
    # Create API key
    api_key = client.api_keys.create(
        name="Test Key",
        scopes=["tokens:read"],
    )

    assert api_key.api_key is not None
    assert api_key.key_id is not None
    assert api_key.name == "Test Key"

    # List keys
    keys = client.api_keys.list()
    assert len(keys) > 0

    # Get key
    retrieved = client.api_keys.get(api_key.key_id)
    assert retrieved.key_id == api_key.key_id

    # Revoke key
    client.api_keys.revoke(api_key.key_id)

    # Delete key
    client.api_keys.delete(api_key.key_id)
