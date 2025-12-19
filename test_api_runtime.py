#!/usr/bin/env python3
"""
Test REST API with RuntimeStorage (v0.51.0)

Quick smoke test for REST API endpoints with Runtime backend.
"""

import requests
import json
import time

BASE_URL = "http://localhost:8000/api/v1"


def test_health():
    """Test /health endpoint."""
    print("\n[1/6] Testing /health...")
    response = requests.get(f"{BASE_URL}/health")
    data = response.json()

    assert response.status_code == 200
    assert data["success"] is True
    assert "runtime_metrics" in data["data"]
    print(f"  ✓ Health check passed: {data['data']['status']}")
    print(f"  ✓ Tokens count: {data['data']['runtime_metrics']['tokens_count']}")
    print(f"  ✓ Storage backend: {data['data']['runtime_metrics']['storage_backend']}")


def test_ready():
    """Test /health/ready endpoint."""
    print("\n[2/6] Testing /health/ready...")
    response = requests.get(f"{BASE_URL}/health/ready")
    data = response.json()

    assert response.status_code == 200
    assert data["success"] is True
    print(f"  ✓ Readiness check: {data['data']['ready']}")
    print(f"  ✓ Checks: {data['data']['checks']}")


def test_status():
    """Test /status endpoint."""
    print("\n[3/6] Testing /status...")
    response = requests.get(f"{BASE_URL}/status")
    data = response.json()

    assert response.status_code == 200
    assert data["success"] is True
    print(f"  ✓ Status: {data['data']['state']}")
    print(f"  ✓ Tokens: {data['data']['tokens']}")
    print(f"  ✓ Memory: {data['data']['memory_usage_mb']:.2f} MB")
    print(f"  ✓ CDNA profile: {data['data'].get('cdna_profile', 'N/A')}")


def test_tokens_crud():
    """Test token CRUD operations."""
    print("\n[4/6] Testing Token CRUD...")

    # Create token
    response = requests.post(
        f"{BASE_URL}/tokens",
        json={"weight": 0.75}
    )
    data = response.json()
    assert response.status_code == 200
    assert data["success"] is True
    token_id = data["data"]["id"]
    print(f"  ✓ Created token: {token_id}")

    # Get token
    response = requests.get(f"{BASE_URL}/tokens/{token_id}")
    data = response.json()
    assert response.status_code == 200
    assert data["data"]["weight"] == 0.75
    print(f"  ✓ Retrieved token: weight={data['data']['weight']}")

    # List tokens
    response = requests.get(f"{BASE_URL}/tokens")
    data = response.json()
    assert response.status_code == 200
    assert data["data"]["total"] >= 1
    print(f"  ✓ Listed tokens: total={data['data']['total']}")

    # Update token
    response = requests.put(
        f"{BASE_URL}/tokens/{token_id}",
        json={"weight": 0.9}
    )
    data = response.json()
    assert response.status_code == 200
    assert data["data"]["weight"] == 0.9
    print(f"  ✓ Updated token: new_weight={data['data']['weight']}")

    # Delete token
    response = requests.delete(f"{BASE_URL}/tokens/{token_id}")
    data = response.json()
    assert response.status_code == 200
    print(f"  ✓ Deleted token: {token_id}")


def test_grid_queries():
    """Test grid spatial queries."""
    print("\n[5/6] Testing Grid queries...")

    # Create some tokens first
    token_ids = []
    for i in range(5):
        response = requests.post(
            f"{BASE_URL}/tokens",
            json={"weight": 1.0 + i * 0.1}
        )
        data = response.json()
        token_ids.append(data["data"]["id"])

    print(f"  ✓ Created {len(token_ids)} tokens for grid test")

    # Grid info
    response = requests.get(f"{BASE_URL}/grid/0")
    data = response.json()
    assert response.status_code == 200
    print(f"  ✓ Grid info retrieved")

    # Find neighbors (if we have tokens)
    if token_ids:
        response = requests.get(
            f"{BASE_URL}/grid/0/neighbors/{token_ids[0]}",
            params={"radius": 10.0, "max_results": 5}
        )
        data = response.json()
        assert response.status_code == 200
        print(f"  ✓ Found {len(data['data']['neighbors'])} neighbors")

    # Cleanup
    for token_id in token_ids:
        requests.delete(f"{BASE_URL}/tokens/{token_id}")
    print(f"  ✓ Cleaned up test tokens")


def test_cdna():
    """Test CDNA configuration."""
    print("\n[6/6] Testing CDNA...")

    # Get config
    response = requests.get(f"{BASE_URL}/cdna/config")
    data = response.json()
    assert response.status_code == 200
    print(f"  ✓ CDNA config: profile={data['data']['profile_id']}")

    # Update scales
    new_scales = [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5]
    response = requests.put(
        f"{BASE_URL}/cdna/config",
        json={"scales": new_scales}
    )
    data = response.json()
    assert response.status_code == 200
    print(f"  ✓ Updated CDNA scales")

    # Validate
    response = requests.post(
        f"{BASE_URL}/cdna/validate",
        json={"scales": new_scales}
    )
    data = response.json()
    assert response.status_code == 200
    print(f"  ✓ CDNA validation: valid={data['data']['valid']}")


def main():
    """Run all tests."""
    print("=" * 60)
    print("REST API v0.51.0 - RuntimeStorage Integration Test")
    print("=" * 60)

    try:
        test_health()
        test_ready()
        test_status()
        test_tokens_crud()
        test_grid_queries()
        test_cdna()

        print("\n" + "=" * 60)
        print("✓ All tests passed! REST API with RuntimeStorage works!")
        print("=" * 60)

    except requests.exceptions.ConnectionError:
        print("\n✗ Error: Cannot connect to API server.")
        print("  Please start the server with: python -m src.api.main")
    except AssertionError as e:
        print(f"\n✗ Test failed: {e}")
        import traceback
        traceback.print_exc()
    except Exception as e:
        print(f"\n✗ Unexpected error: {e}")
        import traceback
        traceback.print_exc()


if __name__ == "__main__":
    main()
