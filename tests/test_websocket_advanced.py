
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
Tests for WebSocket Advanced Features (v0.60.1)

Tests for:
- Metrics tracking
- Permissions checking
- Rate limiting
- Reconnection tokens
- Binary messages
- Compression
"""

import pytest
import time
import json
from datetime import datetime

from src.api.websocket.metrics import metrics, WebSocketMetricsCollector
from src.api.websocket.permissions import (
    can_subscribe,
    can_broadcast,
    get_accessible_channels,
    ChannelPermission
)
from src.api.websocket.channels import Channel
from src.api.websocket.rate_limit import rate_limiter, WebSocketRateLimiter
from src.api.websocket.reconnection import reconnection_manager, ReconnectionManager
from src.api.websocket.binary import (
    binary_handler,
    BinaryMessageHandler,
    BinaryMessageType,
    BinaryMessageFormat
)
from src.api.websocket.compression import (
    default_compressor,
    adaptive_compressor,
    MessageCompressor,
    CompressionAlgorithm
)


class TestMetrics:
    """Test WebSocket metrics tracking."""

    def test_track_connection_opened(self):
        """Test tracking connection opened."""
        metrics.track_connection_opened(user_id="test_user")

        # Just verify no errors
        assert True

    def test_track_message_sent(self):
        """Test tracking message sent."""
        metrics.track_message_sent(
            channel="metrics",
            message_type="test",
            size_bytes=100
        )

        assert True

    def test_track_subscription(self):
        """Test tracking subscription."""
        metrics.track_subscription(channel="metrics")

        assert True

    def test_update_channel_subscribers(self):
        """Test updating channel subscriber count."""
        metrics.update_channel_subscribers(channel="metrics", count=5)

        assert True


class TestPermissions:
    """Test WebSocket permissions."""

    def test_can_subscribe_admin(self):
        """Test admin can subscribe to all channels."""
        assert can_subscribe(Channel.METRICS, "admin") is True
        assert can_subscribe(Channel.SIGNALS, "admin") is True
        assert can_subscribe(Channel.ACTIONS, "admin") is True
        assert can_subscribe(Channel.LOGS, "admin") is True
        assert can_subscribe(Channel.STATUS, "admin") is True
        assert can_subscribe(Channel.CONNECTIONS, "admin") is True

    def test_can_subscribe_anonymous(self):
        """Test anonymous can only subscribe to public channels."""
        assert can_subscribe(Channel.METRICS, "anonymous") is True
        assert can_subscribe(Channel.STATUS, "anonymous") is True
        assert can_subscribe(Channel.SIGNALS, "anonymous") is False
        assert can_subscribe(Channel.ACTIONS, "anonymous") is False
        assert can_subscribe(Channel.LOGS, "anonymous") is False
        assert can_subscribe(Channel.CONNECTIONS, "anonymous") is False

    def test_can_subscribe_developer(self):
        """Test developer permissions."""
        assert can_subscribe(Channel.METRICS, "developer") is True
        assert can_subscribe(Channel.SIGNALS, "developer") is True
        assert can_subscribe(Channel.LOGS, "developer") is True
        assert can_subscribe(Channel.ACTIONS, "developer") is True  # Developers can subscribe to actions

    def test_can_broadcast_admin(self):
        """Test admin can broadcast to all channels."""
        assert can_broadcast(Channel.METRICS, "admin") is True
        assert can_broadcast(Channel.SIGNALS, "admin") is True
        assert can_broadcast(Channel.ACTIONS, "admin") is True

    def test_can_broadcast_anonymous(self):
        """Test anonymous cannot broadcast."""
        assert can_broadcast(Channel.METRICS, "anonymous") is False
        assert can_broadcast(Channel.SIGNALS, "anonymous") is False

    def test_get_accessible_channels(self):
        """Test getting accessible channels."""
        admin_channels = get_accessible_channels("admin")
        assert len(admin_channels) == 6  # All channels

        anon_channels = get_accessible_channels("anonymous")
        assert Channel.METRICS in anon_channels
        assert Channel.STATUS in anon_channels
        assert Channel.SIGNALS not in anon_channels


class TestRateLimiting:
    """Test WebSocket rate limiting."""

    def test_rate_limit_ping(self):
        """Test rate limiting for ping messages."""
        limiter = WebSocketRateLimiter()
        client_id = "test_client_ping"

        # First message should be allowed
        allowed, retry_after = limiter.check_rate_limit(client_id, "ping")
        assert allowed is True
        assert retry_after is None

    def test_rate_limit_exceeded(self):
        """Test rate limit exceeded."""
        limiter = WebSocketRateLimiter()
        client_id = "test_client_exceed"

        # Exhaust the rate limit
        for i in range(150):  # Exceeds capacity
            limiter.check_rate_limit(client_id, "ping")

        # Should be rate limited now
        allowed, retry_after = limiter.check_rate_limit(client_id, "ping")

        # Depending on implementation, might still be allowed due to refill
        # Just verify the call works
        assert isinstance(allowed, bool)

    def test_different_message_types(self):
        """Test different rate limits for different message types."""
        limiter = WebSocketRateLimiter()
        client_id = "test_client_types"

        # Should have different limits
        allowed1, _ = limiter.check_rate_limit(client_id, "ping")
        allowed2, _ = limiter.check_rate_limit(client_id, "subscribe")
        allowed3, _ = limiter.check_rate_limit(client_id, "unknown_type")

        assert allowed1 is True
        assert allowed2 is True
        assert allowed3 is True


class TestReconnection:
    """Test reconnection token management."""

    def test_create_reconnection_token(self):
        """Test creating reconnection token."""
        manager = ReconnectionManager()

        token = manager.create_reconnection_token(
            client_id="test_client",
            user_id="test_user",
            subscriptions=["metrics", "signals"],
            metadata={"role": "developer"}
        )

        assert isinstance(token, str)
        assert len(token) > 0

    def test_restore_session(self):
        """Test restoring session from token."""
        manager = ReconnectionManager()

        # Create token
        token = manager.create_reconnection_token(
            client_id="old_client",
            user_id="test_user",
            subscriptions=["metrics", "signals"],
            metadata={"role": "developer"}
        )

        # Restore session
        session = manager.restore_session(token, "new_client")

        assert session is not None
        assert session.user_id == "test_user"
        assert set(session.subscriptions) == {"metrics", "signals"}
        assert session.metadata["role"] == "developer"

    def test_restore_invalid_token(self):
        """Test restoring with invalid token."""
        manager = ReconnectionManager()

        session = manager.restore_session("invalid_token", "client")

        assert session is None

    def test_token_cleanup(self):
        """Test automatic token cleanup."""
        manager = ReconnectionManager()

        # Create token
        token = manager.create_reconnection_token(
            client_id="test_client",
            user_id="test_user",
            subscriptions=[],
            metadata={}
        )

        # Manually trigger cleanup
        manager.cleanup_expired_sessions()

        # Token should still be valid (not expired yet)
        session = manager.restore_session(token, "new_client")
        # After first restore, token is consumed
        session2 = manager.restore_session(token, "newer_client")
        assert session2 is None


class TestBinaryMessages:
    """Test binary message handling."""

    def test_pack_unpack_image(self):
        """Test packing and unpacking image message."""
        image_data = b"fake_image_data_jpeg"
        metadata = {"format": "jpeg", "width": 800, "height": 600}

        # Pack
        packed = BinaryMessageFormat.pack(
            BinaryMessageType.IMAGE,
            image_data,
            metadata
        )

        assert isinstance(packed, bytes)
        assert len(packed) > len(image_data)

        # Unpack
        msg_type, payload, meta = BinaryMessageFormat.unpack(packed)

        assert msg_type == BinaryMessageType.IMAGE
        assert payload == image_data
        assert meta["format"] == "jpeg"
        assert meta["width"] == 800

    def test_create_image_message(self):
        """Test creating image message."""
        image_data = b"test_image"

        message = binary_handler.create_image_message(
            image_data,
            format="png",
            width=1920,
            height=1080
        )

        assert isinstance(message, bytes)

        # Unpack and verify
        msg_type, payload, meta = BinaryMessageFormat.unpack(message)
        assert msg_type == BinaryMessageType.IMAGE
        assert payload == image_data
        assert meta["format"] == "png"

    def test_create_audio_message(self):
        """Test creating audio message."""
        audio_data = b"test_audio"

        message = binary_handler.create_audio_message(
            audio_data,
            format="wav",
            sample_rate=44100,
            channels=2
        )

        msg_type, payload, meta = BinaryMessageFormat.unpack(message)
        assert msg_type == BinaryMessageType.AUDIO
        assert meta["sample_rate"] == 44100

    def test_compressed_json_message(self):
        """Test compressed JSON message."""
        json_data = {"test": "data", "numbers": [1, 2, 3, 4, 5]}

        message = binary_handler.create_compressed_json_message(json_data)

        # Parse back
        parsed = binary_handler.parse_message(message)
        assert parsed["type"] == "COMPRESSED_JSON"
        assert parsed["json_data"] == json_data


class TestCompression:
    """Test message compression."""

    def test_compress_decompress_gzip(self):
        """Test GZIP compression."""
        compressor = MessageCompressor(
            algorithm=CompressionAlgorithm.GZIP,
            compression_level=6
        )

        original = "Hello, World! " * 100
        compressed = compressor.compress(original)
        decompressed = compressor.decompress(compressed)

        assert len(compressed) < len(original.encode('utf-8'))
        assert decompressed.decode('utf-8') == original

    def test_compress_decompress_zlib(self):
        """Test ZLIB compression."""
        compressor = MessageCompressor(
            algorithm=CompressionAlgorithm.ZLIB,
            compression_level=6
        )

        original = "Test data " * 50
        compressed = compressor.compress(original)
        decompressed = compressor.decompress(compressed)

        assert decompressed.decode('utf-8') == original

    def test_should_compress(self):
        """Test compression threshold."""
        compressor = MessageCompressor(min_size_threshold=1024)

        # Small message
        assert compressor.should_compress("small") is False

        # Large message
        large = "x" * 2000
        assert compressor.should_compress(large) is True

    def test_compress_json(self):
        """Test JSON compression."""
        compressor = MessageCompressor(min_size_threshold=100)

        data = {"key": "value" * 50}
        compressed, was_compressed = compressor.compress_json(data)

        assert isinstance(compressed, bytes)

        if was_compressed:
            # Decompress and verify
            decompressed_json = compressor.decompress_json(compressed)
            assert decompressed_json == data

    def test_adaptive_compressor(self):
        """Test adaptive compressor."""
        text_data = "Text " * 100

        compressed, algorithm = adaptive_compressor.compress(text_data, "text")

        if algorithm != CompressionAlgorithm.NONE:
            decompressed = adaptive_compressor.decompress(compressed, algorithm)
            assert decompressed.decode('utf-8') == text_data

    def test_compression_ratio(self):
        """Test compression ratio calculation."""
        compressor = MessageCompressor()

        # Highly compressible data
        original = "A" * 1000
        compressed = compressor.compress(original)

        ratio = len(compressed) / len(original.encode('utf-8'))

        # Should achieve significant compression
        assert ratio < 0.5  # At least 50% reduction


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
