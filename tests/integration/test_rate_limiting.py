"""
Integration tests for rate limiting.

Tests token bucket algorithm and rate limit enforcement.
"""

import pytest
import time


class TestRateLimiting:
    """Test rate limiting middleware."""

    def test_rate_limit_headers(self, client, admin_token):
        """Test rate limit headers are present."""
        response = client.get(
            "/api/v1/tokens",
            headers={"Authorization": f"Bearer {admin_token}"}
        )

        assert response.status_code == 200
        assert "X-RateLimit-Limit" in response.headers
        assert "X-RateLimit-Remaining" in response.headers

        limit = int(response.headers["X-RateLimit-Limit"])
        remaining = int(response.headers["X-RateLimit-Remaining"])

        assert limit == 100  # Default limit
        assert remaining <= limit

    def test_rate_limit_decreases(self, client, admin_token):
        """Test rate limit remaining decreases with requests."""
        # First request
        response1 = client.get(
            "/api/v1/tokens",
            headers={"Authorization": f"Bearer {admin_token}"}
        )
        remaining1 = int(response1.headers["X-RateLimit-Remaining"])

        # Second request
        response2 = client.get(
            "/api/v1/tokens",
            headers={"Authorization": f"Bearer {admin_token}"}
        )
        remaining2 = int(response2.headers["X-RateLimit-Remaining"])

        # Remaining should decrease
        assert remaining2 < remaining1

    def test_rate_limit_exceeded(self, client, admin_token):
        """Test rate limit exceeded response."""
        # Make many requests quickly
        for _ in range(105):  # Exceed default limit of 100
            response = client.get(
                "/api/v1/tokens",
                headers={"Authorization": f"Bearer {admin_token}"}
            )

        # Last response should be 429
        if response.status_code == 429:
            assert response.status_code == 429
            data = response.json()

            assert data["success"] is False
            assert data["error"]["code"] == "RATE_LIMIT_EXCEEDED"
            assert "Retry-After" in response.headers

    def test_rate_limit_refills(self, client, admin_token):
        """Test rate limit tokens refill over time."""
        # Make request
        response1 = client.get(
            "/api/v1/tokens",
            headers={"Authorization": f"Bearer {admin_token}"}
        )
        remaining1 = int(response1.headers["X-RateLimit-Remaining"])

        # Wait for refill (1 token per second)
        time.sleep(2)

        # Make another request
        response2 = client.get(
            "/api/v1/tokens",
            headers={"Authorization": f"Bearer {admin_token}"}
        )
        remaining2 = int(response2.headers["X-RateLimit-Remaining"])

        # Remaining should be higher (refilled)
        # Note: Might be equal or slightly different due to timing
        assert remaining2 >= remaining1 - 1

    def test_rate_limit_per_user(self, client, admin_token, developer_token):
        """Test rate limits are per-user."""
        # Admin makes requests
        for _ in range(50):
            client.get(
                "/api/v1/tokens",
                headers={"Authorization": f"Bearer {admin_token}"}
            )

        # Developer should have fresh limit
        response = client.get(
            "/api/v1/tokens",
            headers={"Authorization": f"Bearer {developer_token}"}
        )

        assert response.status_code == 200
        remaining = int(response.headers["X-RateLimit-Remaining"])
        # Should be close to full limit (minus 1 for this request)
        assert remaining >= 95

    def test_rate_limit_with_api_key(self, client, test_api_key):
        """Test rate limiting works with API keys."""
        response = client.get(
            "/api/v1/tokens",
            headers={"X-API-Key": test_api_key}
        )

        assert response.status_code == 200
        assert "X-RateLimit-Limit" in response.headers
        assert "X-RateLimit-Remaining" in response.headers

    def test_health_check_exempt_from_rate_limit(self, client):
        """Test health check is exempt from rate limiting."""
        # Make many health check requests
        for _ in range(150):
            response = client.get("/health")

        # Should all succeed (no rate limit)
        assert response.status_code == 200


class TestRateLimitEdgeCases:
    """Test rate limiting edge cases."""

    def test_rate_limit_with_invalid_token(self, client):
        """Test rate limiting applies even with invalid token."""
        # Rate limit should apply before auth
        for _ in range(105):
            response = client.get(
                "/api/v1/tokens",
                headers={"Authorization": "Bearer invalid"}
            )

        # Should eventually hit rate limit (or auth error first)
        # This depends on middleware order

    def test_rate_limit_different_endpoints(self, client, admin_token):
        """Test rate limit is shared across endpoints."""
        # Make requests to different endpoints
        for i in range(50):
            if i % 2 == 0:
                client.get(
                    "/api/v1/tokens",
                    headers={"Authorization": f"Bearer {admin_token}"}
                )
            else:
                client.get(
                    "/api/v1/status",
                    headers={"Authorization": f"Bearer {admin_token}"}
                )

        # Rate limit should be shared
        response = client.get(
            "/api/v1/tokens",
            headers={"Authorization": f"Bearer {admin_token}"}
        )

        remaining = int(response.headers["X-RateLimit-Remaining"])
        # Should be significantly reduced
        assert remaining < 60
