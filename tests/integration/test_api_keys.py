"""
Integration tests for API keys management.

Tests API key creation, usage, revocation, and deletion.
"""

import pytest


class TestAPIKeyManagement:
    """Test API key CRUD operations."""

    def test_create_api_key(self, client, admin_token):
        """Test API key creation."""
        response = client.post(
            "/api/v1/api-keys",
            headers={"Authorization": f"Bearer {admin_token}"},
            json={
                "name": "Test Bot",
                "scopes": ["tokens:read", "tokens:write"],
                "rate_limit": 500,
                "expires_in_days": 30
            }
        )

        assert response.status_code == 201
        data = response.json()["data"]

        assert "api_key" in data
        assert data["api_key"].startswith("ng_live_")
        assert "key_id" in data
        assert data["name"] == "Test Bot"
        assert "warning" in data
        assert "Save this key securely" in data["warning"]

        # Cleanup
        client.delete(
            f"/api/v1/api-keys/{data['key_id']}",
            headers={"Authorization": f"Bearer {admin_token}"}
        )

    def test_create_api_key_non_admin(self, client, developer_token):
        """Test non-admin cannot create API keys."""
        response = client.post(
            "/api/v1/api-keys",
            headers={"Authorization": f"Bearer {developer_token}"},
            json={
                "name": "Test Bot",
                "scopes": ["tokens:read"],
                "rate_limit": 100
            }
        )

        assert response.status_code == 403
        assert "Permission denied" in response.json()["detail"]

    def test_list_api_keys(self, client, admin_token, test_api_key):
        """Test listing API keys."""
        response = client.get(
            "/api/v1/api-keys",
            headers={"Authorization": f"Bearer {admin_token}"}
        )

        assert response.status_code == 200
        data = response.json()["data"]

        assert "keys" in data
        assert "count" in data
        assert data["count"] >= 1

        # Check key structure
        if data["keys"]:
            key = data["keys"][0]
            assert "key_id" in key
            assert "key_prefix" in key
            assert "name" in key
            assert "scopes" in key
            assert "api_key" not in key  # Full key should not be returned

    def test_get_api_key(self, client, admin_token, test_api_key):
        """Test getting specific API key details."""
        # Get key_id from test_api_key fixture
        list_response = client.get(
            "/api/v1/api-keys",
            headers={"Authorization": f"Bearer {admin_token}"}
        )
        keys = list_response.json()["data"]["keys"]
        test_key = next(k for k in keys if "Integration" in k["name"])
        key_id = test_key["key_id"]

        # Get specific key
        response = client.get(
            f"/api/v1/api-keys/{key_id}",
            headers={"Authorization": f"Bearer {admin_token}"}
        )

        assert response.status_code == 200
        data = response.json()["data"]

        assert data["key_id"] == key_id
        assert "api_key" not in data  # Full key should not be returned

    def test_revoke_api_key(self, client, admin_token, test_api_key):
        """Test API key revocation."""
        # Get key_id
        list_response = client.get(
            "/api/v1/api-keys",
            headers={"Authorization": f"Bearer {admin_token}"}
        )
        keys = list_response.json()["data"]["keys"]
        test_key = next(k for k in keys if "Integration" in k["name"])
        key_id = test_key["key_id"]

        # Revoke
        response = client.post(
            f"/api/v1/api-keys/{key_id}/revoke",
            headers={"Authorization": f"Bearer {admin_token}"}
        )

        assert response.status_code == 200
        assert response.json()["data"]["status"] == "revoked"

        # Verify cannot use revoked key
        tokens_response = client.get(
            "/api/v1/tokens",
            headers={"X-API-Key": test_api_key}
        )
        assert tokens_response.status_code == 401

    def test_delete_api_key(self, client, admin_token):
        """Test API key deletion."""
        # Create key
        create_response = client.post(
            "/api/v1/api-keys",
            headers={"Authorization": f"Bearer {admin_token}"},
            json={
                "name": "Delete Test Key",
                "scopes": ["tokens:read"],
                "rate_limit": 100
            }
        )
        key_id = create_response.json()["data"]["key_id"]

        # Delete
        response = client.delete(
            f"/api/v1/api-keys/{key_id}",
            headers={"Authorization": f"Bearer {admin_token}"}
        )

        assert response.status_code == 204

        # Verify key is deleted
        get_response = client.get(
            f"/api/v1/api-keys/{key_id}",
            headers={"Authorization": f"Bearer {admin_token}"}
        )
        assert get_response.status_code == 404


class TestAPIKeyAuthentication:
    """Test authentication with API keys."""

    def test_use_api_key(self, client, test_api_key):
        """Test using API key for authentication."""
        response = client.get(
            "/api/v1/tokens",
            headers={"X-API-Key": test_api_key}
        )

        assert response.status_code == 200

    def test_api_key_with_limited_scopes(self, client, admin_token):
        """Test API key with limited scopes."""
        # Create key with only read permission
        create_response = client.post(
            "/api/v1/api-keys",
            headers={"Authorization": f"Bearer {admin_token}"},
            json={
                "name": "Read Only Key",
                "scopes": ["tokens:read"],  # Only read
                "rate_limit": 100
            }
        )
        api_key = create_response.json()["data"]["api_key"]
        key_id = create_response.json()["data"]["key_id"]

        # Can read
        read_response = client.get(
            "/api/v1/tokens",
            headers={"X-API-Key": api_key}
        )
        assert read_response.status_code == 200

        # Cannot write
        write_response = client.post(
            "/api/v1/tokens",
            headers={"X-API-Key": api_key},
            json={"data": "test", "context": "test"}
        )
        assert write_response.status_code == 403
        assert "Permission denied" in write_response.json()["detail"]

        # Cleanup
        client.delete(
            f"/api/v1/api-keys/{key_id}",
            headers={"Authorization": f"Bearer {admin_token}"}
        )

    def test_invalid_api_key(self, client):
        """Test using invalid API key."""
        response = client.get(
            "/api/v1/tokens",
            headers={"X-API-Key": "ng_live_invalid_key"}
        )

        assert response.status_code == 401

    def test_api_key_and_jwt_both_work(self, client, admin_token, test_api_key):
        """Test that both JWT and API key work on same endpoint."""
        # JWT should work
        jwt_response = client.get(
            "/api/v1/tokens",
            headers={"Authorization": f"Bearer {admin_token}"}
        )
        assert jwt_response.status_code == 200

        # API key should work
        api_key_response = client.get(
            "/api/v1/tokens",
            headers={"X-API-Key": test_api_key}
        )
        assert api_key_response.status_code == 200


class TestAPIKeyExpiration:
    """Test API key expiration."""

    def test_create_key_with_expiration(self, client, admin_token):
        """Test creating key with expiration."""
        response = client.post(
            "/api/v1/api-keys",
            headers={"Authorization": f"Bearer {admin_token}"},
            json={
                "name": "Expiring Key",
                "scopes": ["tokens:read"],
                "rate_limit": 100,
                "expires_in_days": 1  # 1 day
            }
        )

        assert response.status_code == 201
        data = response.json()["data"]

        assert data["expires_at"] is not None

        # Cleanup
        client.delete(
            f"/api/v1/api-keys/{data['key_id']}",
            headers={"Authorization": f"Bearer {admin_token}"}
        )

    def test_create_key_without_expiration(self, client, admin_token):
        """Test creating key without expiration."""
        response = client.post(
            "/api/v1/api-keys",
            headers={"Authorization": f"Bearer {admin_token}"},
            json={
                "name": "Permanent Key",
                "scopes": ["tokens:read"],
                "rate_limit": 100
                # No expires_in_days
            }
        )

        assert response.status_code == 201
        data = response.json()["data"]

        assert data["expires_at"] is None

        # Cleanup
        client.delete(
            f"/api/v1/api-keys/{data['key_id']}",
            headers={"Authorization": f"Bearer {admin_token}"}
        )
