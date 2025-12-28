"""
Integration tests configuration and fixtures.
"""

import pytest
from fastapi.testclient import TestClient
import sys
from pathlib import Path

# Add src to path
src_path = Path(__file__).parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

from api.main import app
from api.storage.api_keys import APIKeyStorage


@pytest.fixture
def client():
    """FastAPI test client."""
    return TestClient(app)


@pytest.fixture
def admin_token(client):
    """Get admin access token."""
    response = client.post(
        "/api/v1/auth/login",
        json={"username": "admin", "password": "admin123"}
    )
    assert response.status_code == 200
    return response.json()["access_token"]


@pytest.fixture
def developer_token(client):
    """Get developer access token."""
    response = client.post(
        "/api/v1/auth/login",
        json={"username": "developer", "password": "developer123"}
    )
    assert response.status_code == 200
    return response.json()["access_token"]


@pytest.fixture
def viewer_token(client):
    """Get viewer access token."""
    response = client.post(
        "/api/v1/auth/login",
        json={"username": "viewer", "password": "viewer123"}
    )
    assert response.status_code == 200
    return response.json()["access_token"]


@pytest.fixture
def test_api_key(client, admin_token):
    """Create test API key."""
    response = client.post(
        "/api/v1/api-keys",
        headers={"Authorization": f"Bearer {admin_token}"},
        json={
            "name": "Test Integration Key",
            "scopes": ["tokens:read", "tokens:write"],
            "rate_limit": 100,
            "expires_in_days": 30
        }
    )
    assert response.status_code == 201
    data = response.json()["data"]

    yield data["api_key"]

    # Cleanup
    try:
        client.delete(
            f"/api/v1/api-keys/{data['key_id']}",
            headers={"Authorization": f"Bearer {admin_token}"}
        )
    except:
        pass


@pytest.fixture(autouse=True)
def cleanup_api_keys():
    """Cleanup test API keys after each test."""
    yield
    # Clean up test keys
    storage = APIKeyStorage(storage_path="data/test_api_keys.json")
    for key in storage.list_keys():
        if "Test" in key.name or "Integration" in key.name:
            storage.delete_key(key.key_id)
