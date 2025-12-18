#!/usr/bin/env python3
"""
Example: Using RuntimeStorage in NeuroGraph v0.50.0

Demonstrates the new RuntimeStorage interface for managing tokens,
connections, spatial queries, and CDNA configuration.
"""

from neurograph import Runtime, Config


def example_token_operations(runtime: Runtime):
    """Demonstrate token CRUD operations."""
    print("=" * 60)
    print("TOKEN OPERATIONS")
    print("=" * 60)

    # Create tokens
    print("\n1. Creating tokens...")
    token1 = runtime.tokens.create(weight=1.0)
    token2 = runtime.tokens.create(weight=0.8)
    token3 = runtime.tokens.create(weight=1.2)
    print(f"   Created tokens: {token1}, {token2}, {token3}")

    # Get token
    print("\n2. Getting token...")
    token_data = runtime.tokens.get(token1)
    if token_data:
        print(f"   Token {token1}: weight={token_data['weight']}")

    # Update token
    print("\n3. Updating token...")
    success = runtime.tokens.update(token1, weight=0.9)
    print(f"   Update {'successful' if success else 'failed'}")

    # List tokens
    print("\n4. Listing tokens...")
    token_ids = runtime.tokens.list(limit=10)
    print(f"   Found {len(token_ids)} tokens: {token_ids}")

    # Count tokens
    print("\n5. Counting tokens...")
    total = runtime.tokens.count()
    print(f"   Total tokens: {total}")

    # Delete token
    print("\n6. Deleting token...")
    deleted = runtime.tokens.delete(token3)
    print(f"   Token {token3} {'deleted' if deleted else 'not found'}")
    print(f"   Remaining tokens: {runtime.tokens.count()}")

    return token1, token2


def example_connection_operations(runtime: Runtime, token1: int, token2: int):
    """Demonstrate connection operations."""
    print("\n" + "=" * 60)
    print("CONNECTION OPERATIONS")
    print("=" * 60)

    # Create connection
    print("\n1. Creating connection...")
    conn_id = runtime.connections.create(token_a=token1, token_b=token2)
    print(f"   Created connection {conn_id}: {token1} <-> {token2}")

    # Get connection
    print("\n2. Getting connection...")
    conn_data = runtime.connections.get(conn_id)
    if conn_data:
        print(f"   Connection: {conn_data['token_a_id']} <-> {conn_data['token_b_id']}")

    # Count connections
    print("\n3. Counting connections...")
    total = runtime.connections.count()
    print(f"   Total connections: {total}")

    # List connections
    print("\n4. Listing connections...")
    conn_ids = runtime.connections.list(limit=10)
    print(f"   Found {len(conn_ids)} connections")

    return conn_id


def example_grid_operations(runtime: Runtime, token1: int):
    """Demonstrate spatial grid operations."""
    print("\n" + "=" * 60)
    print("GRID OPERATIONS")
    print("=" * 60)

    # Grid info
    print("\n1. Getting grid info...")
    info = runtime.grid.info()
    print(f"   Tokens in grid: {info['count']}")

    # Find neighbors
    print("\n2. Finding neighbors...")
    try:
        neighbors = runtime.grid.find_neighbors(token_id=token1, radius=10.0)
        print(f"   Found {len(neighbors)} neighbors within radius 10.0")
        for neighbor_id, distance in neighbors[:5]:  # Show first 5
            print(f"     - Token {neighbor_id} at distance {distance:.2f}")
    except Exception as e:
        print(f"   Error: {e}")

    # Range query
    print("\n3. Range query around origin...")
    try:
        results = runtime.grid.range_query(center=(0.0, 0.0, 0.0), radius=5.0)
        print(f"   Found {len(results)} tokens near origin")
        for token_id, distance in results[:3]:  # Show first 3
            print(f"     - Token {token_id} at distance {distance:.2f}")
    except Exception as e:
        print(f"   Error: {e}")


def example_cdna_operations(runtime: Runtime):
    """Demonstrate CDNA configuration operations."""
    print("\n" + "=" * 60)
    print("CDNA CONFIGURATION")
    print("=" * 60)

    # Get config
    print("\n1. Getting CDNA config...")
    config = runtime.cdna.get_config()
    print(f"   Profile ID: {config['profile_id']}")
    print(f"   Flags: 0x{config['flags']:X}")

    # Validate
    print("\n2. Validating CDNA...")
    is_valid = runtime.cdna.validate()
    print(f"   CDNA is {'valid' if is_valid else 'invalid'}")

    # Update scales
    print("\n3. Updating dimension scales...")
    try:
        # Set custom scales for L1-L8 dimensions
        scales = [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5]
        success = runtime.cdna.update_scales(scales)
        print(f"   Scales update {'successful' if success else 'failed'}")
        print(f"   New scales: {scales}")
    except Exception as e:
        print(f"   Error: {e}")

    # Get/Set profile
    print("\n4. Managing profile...")
    profile_id = runtime.cdna.get_profile()
    print(f"   Current profile: {profile_id}")

    runtime.cdna.set_profile(1)  # Set Explorer profile
    new_profile = runtime.cdna.get_profile()
    print(f"   New profile: {new_profile}")

    # Get/Set flags
    print("\n5. Managing flags...")
    flags = runtime.cdna.get_flags()
    print(f"   Current flags: 0x{flags:X}")

    runtime.cdna.set_flags(0xFF)
    new_flags = runtime.cdna.get_flags()
    print(f"   New flags: 0x{new_flags:X}")


def example_cleanup(runtime: Runtime):
    """Cleanup demonstration."""
    print("\n" + "=" * 60)
    print("CLEANUP")
    print("=" * 60)

    print("\n1. Clearing all tokens...")
    removed = runtime.tokens.clear()
    print(f"   Removed {removed} tokens")

    print("\n2. Final counts:")
    print(f"   Tokens: {runtime.tokens.count()}")
    print(f"   Connections: {runtime.connections.count()}")


def main():
    """Run all examples."""
    print("\n" + "=" * 60)
    print("NeuroGraph v0.50.0 - Runtime Storage Example")
    print("=" * 60)

    # Initialize runtime
    print("\nInitializing runtime...")
    config = Config(
        grid_size=1000,
        dimensions=50,
    )
    runtime = Runtime(config)
    print("✓ Runtime initialized")

    # Check if FFI is available
    if runtime.tokens is None:
        print("\n⚠️  FFI module not available!")
        print("Please build the module first:")
        print("  cd src/core_rust")
        print("  maturin develop --release")
        return

    try:
        # Run examples
        token1, token2 = example_token_operations(runtime)
        conn_id = example_connection_operations(runtime, token1, token2)
        example_grid_operations(runtime, token1)
        example_cdna_operations(runtime)
        example_cleanup(runtime)

        print("\n" + "=" * 60)
        print("✓ All examples completed successfully!")
        print("=" * 60)

    except Exception as e:
        print(f"\n❌ Error: {e}")
        import traceback
        traceback.print_exc()


if __name__ == "__main__":
    main()
