
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
WebSocket module for NeuroGraph API

Version: v0.60.1
Provides real-time communication for live updates with advanced features
"""

from .manager import ConnectionManager
from .connection import websocket_endpoint
from .channels import Channel
from .metrics import metrics, WebSocketMetricsCollector
from .permissions import can_subscribe, can_broadcast, get_accessible_channels
from .rate_limit import rate_limiter, WebSocketRateLimiter
from .reconnection import reconnection_manager, ReconnectionManager
from .binary import binary_handler, BinaryMessageHandler, BinaryMessageType
from .compression import default_compressor, adaptive_compressor, MessageCompressor

__all__ = [
    # Core
    "ConnectionManager",
    "websocket_endpoint",
    "Channel",

    # Metrics
    "metrics",
    "WebSocketMetricsCollector",

    # Permissions
    "can_subscribe",
    "can_broadcast",
    "get_accessible_channels",

    # Rate Limiting
    "rate_limiter",
    "WebSocketRateLimiter",

    # Reconnection
    "reconnection_manager",
    "ReconnectionManager",

    # Binary Messages
    "binary_handler",
    "BinaryMessageHandler",
    "BinaryMessageType",

    # Compression
    "default_compressor",
    "adaptive_compressor",
    "MessageCompressor",
]
