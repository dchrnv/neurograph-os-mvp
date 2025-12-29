"""
Tests for NeuroGraph asynchronous client.

Run with: pytest tests/test_async_client.py
"""

import pytest
from neurograph import AsyncNeuroGraphClient, AuthenticationError, NotFoundError
from neurograph.models import Token


# Skip tests if API is not running
pytestmark = pytest.mark.skipif(
    reason="API server not running (start with: uvicorn src.api.main:app)",
)


@pytest.fixture
async def client():
    """Create async test client."""
    client = AsyncNeuroGraphClient(
        base_url="http://localhost:8000",
        username="developer",
        password="developer123",
    )
    yield client
    await client.close()


@pytest.mark.asyncio
async def test_health_check(client):
    """Test health check endpoint."""
    health = await client.health.check()
    assert health.status == "healthy"
    assert health.version is not None


@pytest.mark.asyncio
async def test_create_token(client):
    """Test token creation."""
    token = await client.tokens.create(text="async test token")

    assert isinstance(token, Token)
    assert token.id is not None
    assert token.text == "async test token"
    assert len(token.embedding) > 0

    # Cleanup
    await client.tokens.delete(token.id)


@pytest.mark.asyncio
async def test_get_token(client):
    """Test get token."""
    # Create token
    created = await client.tokens.create(text="async get test")

    # Get token
    retrieved = await client.tokens.get(created.id)

    assert retrieved.id == created.id
    assert retrieved.text == created.text

    # Cleanup
    await client.tokens.delete(created.id)


@pytest.mark.asyncio
async def test_get_nonexistent_token(client):
    """Test get non-existent token raises NotFoundError."""
    with pytest.raises(NotFoundError):
        await client.tokens.get(token_id=999999)


@pytest.mark.asyncio
async def test_list_tokens(client):
    """Test list tokens."""
    # Create some tokens
    tokens = []
    for i in range(3):
        token = await client.tokens.create(text=f"async test {i}")
        tokens.append(token)

    # List tokens
    listed = await client.tokens.list(limit=10)

    assert len(listed) >= 3

    # Cleanup
    for token in tokens:
        await client.tokens.delete(token.id)


@pytest.mark.asyncio
async def test_update_token(client):
    """Test update token."""
    # Create token
    token = await client.tokens.create(text="async original")

    # Update
    updated = await client.tokens.update(
        token.id,
        metadata={"async_updated": True}
    )

    assert updated.id == token.id
    assert updated.metadata == {"async_updated": True}

    # Cleanup
    await client.tokens.delete(token.id)


@pytest.mark.asyncio
async def test_delete_token(client):
    """Test delete token."""
    # Create token
    token = await client.tokens.create(text="async to delete")

    # Delete
    result = await client.tokens.delete(token.id)
    assert result is True

    # Verify deleted
    with pytest.raises(NotFoundError):
        await client.tokens.get(token.id)


@pytest.mark.asyncio
async def test_query_tokens(client):
    """Test query tokens."""
    # Create token
    token = await client.tokens.create(text="async query test")

    # Query
    results = await client.tokens.query(
        query_vector=token.embedding,
        top_k=5
    )

    assert len(results) > 0
    assert results[0].token.id == token.id
    assert results[0].similarity > 0.99

    # Cleanup
    await client.tokens.delete(token.id)


@pytest.mark.asyncio
async def test_concurrent_operations(client):
    """Test concurrent token operations."""
    import asyncio

    # Create tokens concurrently
    tasks = [
        client.tokens.create(text=f"concurrent {i}")
        for i in range(5)
    ]
    tokens = await asyncio.gather(*tasks)

    assert len(tokens) == 5
    assert all(isinstance(t, Token) for t in tokens)

    # Delete concurrently
    delete_tasks = [
        client.tokens.delete(token.id)
        for token in tokens
    ]
    await asyncio.gather(*delete_tasks)


@pytest.mark.asyncio
async def test_authentication_error():
    """Test authentication with wrong credentials."""
    client = AsyncNeuroGraphClient(
        base_url="http://localhost:8000",
        username="wrong",
        password="wrong",
    )

    with pytest.raises(AuthenticationError):
        await client.tokens.list()

    await client.close()


@pytest.mark.asyncio
async def test_context_manager():
    """Test async client as context manager."""
    async with AsyncNeuroGraphClient(
        base_url="http://localhost:8000",
        username="developer",
        password="developer123",
    ) as client:
        health = await client.health.check()
        assert health.status == "healthy"
    # Client should be closed after exit


@pytest.mark.asyncio
async def test_api_keys(client):
    """Test async API key management."""
    # Create API key
    api_key = await client.api_keys.create(
        name="Async Test Key",
        scopes=["tokens:read"],
    )

    assert api_key.api_key is not None
    assert api_key.key_id is not None
    assert api_key.name == "Async Test Key"

    # List keys
    keys = await client.api_keys.list()
    assert len(keys) > 0

    # Get key
    retrieved = await client.api_keys.get(api_key.key_id)
    assert retrieved.key_id == api_key.key_id

    # Revoke key
    await client.api_keys.revoke(api_key.key_id)

    # Delete key
    await client.api_keys.delete(api_key.key_id)
