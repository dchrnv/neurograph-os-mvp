"""
Tests for mock NeuroGraph client.
"""

import pytest
from neurograph.testing import MockNeuroGraphClient, mock_token, mock_api_key
from neurograph import NotFoundError


def test_mock_client_create_token():
    """Test creating token with mock client."""
    client = MockNeuroGraphClient()

    token = client.tokens.create(text="test token")

    assert token.id == 1
    assert token.text == "test token"
    assert len(token.embedding) == 768
    assert token.metadata == {}


def test_mock_client_get_token():
    """Test getting token with mock client."""
    client = MockNeuroGraphClient()

    created = client.tokens.create(text="test")
    retrieved = client.tokens.get(created.id)

    assert retrieved.id == created.id
    assert retrieved.text == created.text


def test_mock_client_get_nonexistent_token():
    """Test getting non-existent token raises NotFoundError."""
    client = MockNeuroGraphClient()

    with pytest.raises(NotFoundError):
        client.tokens.get(999)


def test_mock_client_list_tokens():
    """Test listing tokens with mock client."""
    client = MockNeuroGraphClient()

    # Create some tokens
    for i in range(5):
        client.tokens.create(text=f"token {i}")

    tokens = client.tokens.list()

    assert len(tokens) == 5


def test_mock_client_update_token():
    """Test updating token with mock client."""
    client = MockNeuroGraphClient()

    token = client.tokens.create(text="original")
    updated = client.tokens.update(token.id, metadata={"updated": True})

    assert updated.id == token.id
    assert updated.metadata == {"updated": True}


def test_mock_client_delete_token():
    """Test deleting token with mock client."""
    client = MockNeuroGraphClient()

    token = client.tokens.create(text="to delete")
    result = client.tokens.delete(token.id)

    assert result is True

    with pytest.raises(NotFoundError):
        client.tokens.get(token.id)


def test_mock_client_query_tokens():
    """Test querying tokens with mock client."""
    client = MockNeuroGraphClient()

    # Create some tokens
    for i in range(10):
        client.tokens.create(text=f"token {i}")

    # Query
    results = client.tokens.query(
        query_vector=[0.1] * 768,
        top_k=5
    )

    assert len(results) <= 5
    assert all(hasattr(r, 'token') for r in results)
    assert all(hasattr(r, 'similarity') for r in results)
    assert all(0.0 <= r.similarity <= 1.0 for r in results)


def test_mock_client_create_api_key():
    """Test creating API key with mock client."""
    client = MockNeuroGraphClient()

    api_key = client.api_keys.create(
        name="test key",
        scopes=["tokens:read"]
    )

    assert api_key.key_id.startswith("key_")
    assert api_key.api_key.startswith("ng_mock_")
    assert api_key.name == "test key"
    assert api_key.scopes == ["tokens:read"]
    assert api_key.is_active is True


def test_mock_client_list_api_keys():
    """Test listing API keys with mock client."""
    client = MockNeuroGraphClient()

    # Create some keys
    for i in range(3):
        client.api_keys.create(
            name=f"key {i}",
            scopes=["tokens:read"]
        )

    keys = client.api_keys.list()

    assert len(keys) == 3
    # api_key field should not be included in list
    assert all(k.api_key is None for k in keys)


def test_mock_client_revoke_api_key():
    """Test revoking API key with mock client."""
    client = MockNeuroGraphClient()

    api_key = client.api_keys.create(
        name="test",
        scopes=["tokens:read"]
    )

    client.api_keys.revoke(api_key.key_id)

    retrieved = client.api_keys.get(api_key.key_id)
    assert retrieved.is_active is False


def test_mock_client_delete_api_key():
    """Test deleting API key with mock client."""
    client = MockNeuroGraphClient()

    api_key = client.api_keys.create(
        name="test",
        scopes=["tokens:read"]
    )

    client.api_keys.delete(api_key.key_id)

    with pytest.raises(NotFoundError):
        client.api_keys.get(api_key.key_id)


def test_mock_client_health_check():
    """Test health check with mock client."""
    client = MockNeuroGraphClient()

    health = client.health.check()

    assert health.status == "healthy"
    assert health.version == "0.59.0-mock"


def test_mock_client_system_status():
    """Test system status with mock client."""
    client = MockNeuroGraphClient()

    # Create some data
    client.tokens.create(text="test")
    client.api_keys.create(name="test", scopes=["tokens:read"])

    status = client.health.status()

    assert status.status == "healthy"
    assert status.tokens_count == 1
    assert status.api_keys_count == 1


def test_mock_client_context_manager():
    """Test mock client as context manager."""
    with MockNeuroGraphClient() as client:
        token = client.tokens.create(text="test")
        assert token.id == 1


def test_mock_token_factory():
    """Test mock token factory function."""
    token = mock_token(text="custom", id=123)

    assert token.id == 123
    assert token.text == "custom"
    assert len(token.embedding) == 768


def test_mock_api_key_factory():
    """Test mock API key factory function."""
    api_key = mock_api_key(name="custom key", scopes=["tokens:write"])

    assert api_key.name == "custom key"
    assert api_key.scopes == ["tokens:write"]
    assert api_key.api_key.startswith("ng_mock_")
