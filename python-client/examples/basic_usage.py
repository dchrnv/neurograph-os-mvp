"""
Basic usage examples for NeuroGraph Python client.

This example demonstrates:
- Authentication with JWT
- Creating and querying tokens
- Managing API keys
- Error handling
"""

from neurograph import NeuroGraphClient, AuthenticationError, NotFoundError


def main():
    # Initialize client with JWT authentication
    client = NeuroGraphClient(
        base_url="http://localhost:8000",
        username="developer",
        password="developer123",
    )

    # Or use API key authentication
    # client = NeuroGraphClient(
    #     base_url="http://localhost:8000",
    #     api_key="ng_1234567890abcdef",
    # )

    # ========================================================================
    # Health Check
    # ========================================================================
    print("=== Health Check ===")
    health = client.health.check()
    print(f"Status: {health.status}")
    print(f"Version: {health.version}")

    # ========================================================================
    # Create Tokens
    # ========================================================================
    print("\n=== Creating Tokens ===")

    # Create a token
    token1 = client.tokens.create(
        text="hello world",
        metadata={"category": "greeting"}
    )
    print(f"Created token {token1.id}: {token1.text}")
    print(f"Embedding: {token1.embedding[:5]}...")  # Show first 5 dimensions

    # Create another token
    token2 = client.tokens.create(
        text="goodbye world",
        metadata={"category": "farewell"}
    )
    print(f"Created token {token2.id}: {token2.text}")

    # ========================================================================
    # Get Token
    # ========================================================================
    print("\n=== Get Token ===")

    retrieved = client.tokens.get(token1.id)
    print(f"Retrieved token {retrieved.id}: {retrieved.text}")

    # ========================================================================
    # List Tokens
    # ========================================================================
    print("\n=== List Tokens ===")

    tokens = client.tokens.list(limit=10)
    print(f"Total tokens: {len(tokens)}")
    for token in tokens[:5]:  # Show first 5
        print(f"  - Token {token.id}: {token.text}")

    # ========================================================================
    # Query Similar Tokens
    # ========================================================================
    print("\n=== Query Tokens ===")

    # Query using a token's embedding
    results = client.tokens.query(
        query_vector=token1.embedding,
        top_k=5,
    )
    print(f"Found {len(results)} similar tokens:")
    for result in results:
        print(f"  - Token {result.token.id}: {result.token.text} (similarity: {result.similarity:.4f})")

    # ========================================================================
    # Update Token
    # ========================================================================
    print("\n=== Update Token ===")

    updated = client.tokens.update(
        token1.id,
        metadata={"category": "greeting", "language": "en"}
    )
    print(f"Updated token {updated.id}: {updated.metadata}")

    # ========================================================================
    # API Keys Management
    # ========================================================================
    print("\n=== API Keys ===")

    # Create API key
    api_key = client.api_keys.create(
        name="My Integration Key",
        scopes=["tokens:read", "tokens:write"],
        expires_in_days=30,
    )
    print(f"Created API key: {api_key.api_key}")
    print(f"Key ID: {api_key.key_id}")
    print("⚠️  Save this key! It won't be shown again.")

    # List API keys
    keys = client.api_keys.list()
    print(f"\nTotal API keys: {len(keys)}")
    for key in keys:
        print(f"  - {key.name} ({key.key_prefix}...) - Active: {key.is_active}")

    # ========================================================================
    # Error Handling
    # ========================================================================
    print("\n=== Error Handling ===")

    try:
        # Try to get non-existent token
        client.tokens.get(999999)
    except NotFoundError as e:
        print(f"Token not found: {e.message}")

    try:
        # Try with wrong credentials
        bad_client = NeuroGraphClient(
            base_url="http://localhost:8000",
            username="wrong",
            password="wrong",
        )
        bad_client.tokens.list()
    except AuthenticationError as e:
        print(f"Authentication failed: {e.message}")

    # ========================================================================
    # Cleanup
    # ========================================================================
    print("\n=== Cleanup ===")

    # Delete tokens
    client.tokens.delete(token1.id)
    client.tokens.delete(token2.id)
    print(f"Deleted tokens")

    # Revoke API key
    client.api_keys.revoke(api_key.key_id)
    print(f"Revoked API key")

    # Close client
    client.close()
    print("\nDone!")


if __name__ == "__main__":
    main()
