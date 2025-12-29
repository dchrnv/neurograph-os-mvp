
    # NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
    # Copyright (C) 2024-2025 Chernov Denys

    # This program is free software: you can redistribute it and/or modify
    # it under the terms of the GNU Affero General Public License as published by
    # the Free Software Foundation, either version 3 of the License, or
    # (at your option) any later version.

    # This program is distributed in the hope that it will be useful,
    # but WITHOUT ANY WARRANTY; without even the implied warranty of
    # MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    # GNU Affero General Public License for more details.

    # You should have received a copy of the GNU Affero General Public License
    # along with this program. If not, see <https://www.gnu.org/licenses/>.

"""
WebSocket Integration Tests

Tests for WebSocket connection, subscription, and event broadcasting.
"""

import pytest
import json
from fastapi.testclient import TestClient
from fastapi.websockets import WebSocketDisconnect

from src.api.main import app
from src.api.websocket.manager import ConnectionManager
from src.api.websocket.channels import Channel


@pytest.fixture
def client():
    """Create test client."""
    return TestClient(app)


@pytest.fixture
def connection_manager():
    """Create fresh connection manager for each test."""
    return ConnectionManager()


class TestWebSocketConnection:
    """Test WebSocket connection establishment and lifecycle."""

    def test_websocket_connect_without_auth(self, client):
        """Test connecting to WebSocket without authentication."""
        with client.websocket_connect("/ws") as websocket:
            # Should receive connection confirmation
            data = websocket.receive_json()

            assert data["type"] == "connected"
            assert "client_id" in data
            assert data["user_id"] is None
            assert "timestamp" in data

    def test_websocket_connect_with_token(self, client):
        """Test connecting to WebSocket with JWT token."""
        # Invalid token should close connection
        try:
            with client.websocket_connect("/ws?token=test_token") as websocket:
                # Should not reach here with invalid token
                assert False, "Expected connection to be rejected"
        except WebSocketDisconnect as e:
            # Expected: connection closed due to invalid token
            assert e.code == 1008  # Policy Violation

    def test_websocket_ping_pong(self, client):
        """Test ping-pong heartbeat mechanism."""
        with client.websocket_connect("/ws") as websocket:
            # Skip connection message
            websocket.receive_json()

            # Send ping
            websocket.send_json({"type": "ping"})

            # Should receive pong
            pong = websocket.receive_json()
            assert pong["type"] == "pong"
            assert "timestamp" in pong

    def test_websocket_invalid_json(self, client):
        """Test sending invalid JSON."""
        with client.websocket_connect("/ws") as websocket:
            # Skip connection message
            websocket.receive_json()

            # Send invalid JSON
            websocket.send_text("invalid json{")

            # Should receive error
            error = websocket.receive_json()
            assert error["type"] == "error"
            assert "Invalid JSON" in error["message"]


class TestWebSocketSubscriptions:
    """Test channel subscription functionality."""

    def test_subscribe_to_channel(self, client):
        """Test subscribing to a channel."""
        with client.websocket_connect("/ws") as websocket:
            # Skip connection message
            websocket.receive_json()

            # Subscribe to metrics channel
            websocket.send_json({
                "type": "subscribe",
                "channels": ["metrics"]
            })

            # Should receive subscription confirmation
            response = websocket.receive_json()
            assert response["type"] == "subscribed"
            assert response["channels"] == ["metrics"]

    def test_subscribe_multiple_channels(self, client):
        """Test subscribing to multiple channels at once."""
        with client.websocket_connect("/ws") as websocket:
            # Skip connection message
            websocket.receive_json()

            # Subscribe to multiple channels
            channels = ["metrics", "signals", "actions"]
            websocket.send_json({
                "type": "subscribe",
                "channels": channels
            })

            # Should receive subscription confirmation
            response = websocket.receive_json()
            assert response["type"] == "subscribed"
            assert set(response["channels"]) == set(channels)

    def test_unsubscribe_from_channel(self, client):
        """Test unsubscribing from a channel."""
        with client.websocket_connect("/ws") as websocket:
            # Skip connection message
            websocket.receive_json()

            # First subscribe
            websocket.send_json({
                "type": "subscribe",
                "channels": ["metrics", "signals"]
            })
            websocket.receive_json()  # Skip confirmation

            # Then unsubscribe from one
            websocket.send_json({
                "type": "unsubscribe",
                "channels": ["metrics"]
            })

            # Should receive unsubscription confirmation
            response = websocket.receive_json()
            assert response["type"] == "unsubscribed"
            assert response["channels"] == ["metrics"]

    def test_get_subscriptions(self, client):
        """Test getting current subscriptions."""
        with client.websocket_connect("/ws") as websocket:
            # Skip connection message
            websocket.receive_json()

            # Subscribe to channels
            websocket.send_json({
                "type": "subscribe",
                "channels": ["metrics", "signals"]
            })
            websocket.receive_json()  # Skip confirmation

            # Get subscriptions
            websocket.send_json({"type": "get_subscriptions"})

            # Should receive subscriptions list
            response = websocket.receive_json()
            assert response["type"] == "subscriptions"
            assert set(response["channels"]) == {"metrics", "signals"}

    def test_subscribe_invalid_channels(self, client):
        """Test subscribing with invalid channels parameter."""
        with client.websocket_connect("/ws") as websocket:
            # Skip connection message
            websocket.receive_json()

            # Send invalid channels (not an array)
            websocket.send_json({
                "type": "subscribe",
                "channels": "not_an_array"
            })

            # Should receive error
            error = websocket.receive_json()
            assert error["type"] == "error"
            assert "must be an array" in error["message"]


class TestConnectionManager:
    """Test ConnectionManager functionality."""

    @pytest.mark.asyncio
    async def test_connection_stats(self, connection_manager):
        """Test getting connection statistics."""
        stats = connection_manager.get_connection_stats()

        assert "total_connections" in stats
        assert "total_channels" in stats
        assert "total_subscriptions" in stats
        assert "buffered_events" in stats
        assert "clients" in stats
        assert isinstance(stats["clients"], list)

    def test_subscribe_and_get_subscribers(self, connection_manager):
        """Test subscribing clients and getting subscriber list."""
        client_id = "test_client_1"

        # Subscribe to channel
        connection_manager.subscribe(client_id, ["metrics"])

        # Get subscribers
        subscribers = connection_manager.get_channel_subscribers("metrics")
        assert client_id in subscribers

    def test_unsubscribe_removes_from_channel(self, connection_manager):
        """Test that unsubscribe removes client from channel."""
        client_id = "test_client_1"

        # Subscribe then unsubscribe
        connection_manager.subscribe(client_id, ["metrics"])
        connection_manager.unsubscribe(client_id, ["metrics"])

        # Should not be in subscribers
        subscribers = connection_manager.get_channel_subscribers("metrics")
        assert client_id not in subscribers

    def test_disconnect_cleans_up_subscriptions(self, connection_manager):
        """Test that disconnect removes all subscriptions."""
        client_id = "test_client_1"

        # Subscribe to multiple channels
        connection_manager.subscribe(client_id, ["metrics", "signals"])

        # Disconnect
        connection_manager.disconnect(client_id)

        # Should not be in any channel
        assert client_id not in connection_manager.get_channel_subscribers("metrics")
        assert client_id not in connection_manager.get_channel_subscribers("signals")
        assert len(connection_manager.get_subscriptions(client_id)) == 0


class TestWebSocketAPI:
    """Test WebSocket REST API endpoints."""

    def test_get_websocket_status(self, client):
        """Test GET /api/v1/websocket/status endpoint."""
        response = client.get("/api/v1/websocket/status")

        assert response.status_code == 200
        data = response.json()

        assert data["status"] == "ok"
        assert "websocket" in data
        assert data["websocket"]["endpoint"] == "/ws"
        assert "total_connections" in data["websocket"]

    def test_get_channels(self, client):
        """Test GET /api/v1/websocket/channels endpoint."""
        response = client.get("/api/v1/websocket/channels")

        assert response.status_code == 200
        data = response.json()

        assert "channels" in data
        assert "descriptions" in data
        assert "metrics" in data["channels"]
        assert "signals" in data["channels"]
        assert "actions" in data["channels"]

    def test_get_channel_status(self, client):
        """Test GET /api/v1/websocket/channels/{channel} endpoint."""
        response = client.get("/api/v1/websocket/channels/metrics")

        assert response.status_code == 200
        data = response.json()

        assert data["channel"] == "metrics"
        assert "subscribers" in data
        assert "subscriber_ids" in data


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
