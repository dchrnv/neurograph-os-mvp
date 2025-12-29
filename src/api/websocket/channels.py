
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
WebSocket Event Channels

Defines available channels and their event types for real-time streaming.
"""

from enum import Enum
from typing import Dict, Any
from pydantic import BaseModel


class Channel(str, Enum):
    """Available WebSocket channels."""

    METRICS = "metrics"          # System metrics stream
    SIGNALS = "signals"          # Signal events stream
    ACTIONS = "actions"          # Action execution stream
    LOGS = "logs"                # System logs stream
    STATUS = "status"            # System status changes
    CONNECTIONS = "connections"  # WebSocket connection events


# Channel descriptions
CHANNEL_DESCRIPTIONS: Dict[str, str] = {
    Channel.METRICS: "System metrics and performance data (CPU, memory, events/sec)",
    Channel.SIGNALS: "Real-time signal events from the SignalSystem",
    Channel.ACTIONS: "Action execution notifications and results",
    Channel.LOGS: "System logs and debug information",
    Channel.STATUS: "System status changes (started, stopped, errors)",
    Channel.CONNECTIONS: "WebSocket connection and subscription events"
}


class ChannelEvent(BaseModel):
    """Base class for channel events."""

    channel: str
    timestamp: str
    event_type: str


class MetricsEvent(ChannelEvent):
    """
    Metrics channel event.

    Example:
        {
            "channel": "metrics",
            "timestamp": "2024-12-26T12:00:00Z",
            "event_type": "system_metrics",
            "data": {
                "cpu_percent": 45.2,
                "memory_mb": 1024,
                "events_per_sec": 5000,
                "total_tokens": 50000
            }
        }
    """

    channel: str = Channel.METRICS
    data: Dict[str, Any]


class SignalEvent(ChannelEvent):
    """
    Signal channel event.

    Example:
        {
            "channel": "signals",
            "timestamp": "2024-12-26T12:00:00Z",
            "event_type": "signal.input.text",
            "data": {
                "event_id": "evt_123",
                "event_type": "signal.input.text",
                "is_novel": true,
                "priority": 128,
                "resonance_score": 0.85,
                "neighbors_count": 5
            }
        }
    """

    channel: str = Channel.SIGNALS
    data: Dict[str, Any]


class ActionEvent(ChannelEvent):
    """
    Action channel event.

    Example:
        {
            "channel": "actions",
            "timestamp": "2024-12-26T12:00:00Z",
            "event_type": "action.executed",
            "data": {
                "action_id": "act_456",
                "action_type": "TextResponseAction",
                "status": "completed",
                "result": {"response": "Hello!"},
                "execution_time_ms": 15
            }
        }
    """

    channel: str = Channel.ACTIONS
    data: Dict[str, Any]


class LogEvent(ChannelEvent):
    """
    Log channel event.

    Example:
        {
            "channel": "logs",
            "timestamp": "2024-12-26T12:00:00Z",
            "event_type": "log.info",
            "data": {
                "level": "INFO",
                "message": "System started",
                "service": "api",
                "correlation_id": "req_789"
            }
        }
    """

    channel: str = Channel.LOGS
    data: Dict[str, Any]


class StatusEvent(ChannelEvent):
    """
    Status channel event.

    Example:
        {
            "channel": "status",
            "timestamp": "2024-12-26T12:00:00Z",
            "event_type": "status.running",
            "data": {
                "status": "running",
                "uptime_seconds": 3600,
                "version": "v0.60.0"
            }
        }
    """

    channel: str = Channel.STATUS
    data: Dict[str, Any]


class ConnectionEvent(ChannelEvent):
    """
    Connection channel event.

    Example:
        {
            "channel": "connections",
            "timestamp": "2024-12-26T12:00:00Z",
            "event_type": "client.connected",
            "data": {
                "client_id": "abc123",
                "user_id": "user_456",
                "total_connections": 42
            }
        }
    """

    channel: str = Channel.CONNECTIONS
    data: Dict[str, Any]


def get_all_channels() -> list[str]:
    """
    Get list of all available channels.

    Returns:
        List of channel names
    """
    return [channel.value for channel in Channel]


def get_channel_info() -> Dict[str, str]:
    """
    Get information about all channels.

    Returns:
        Dictionary mapping channel names to descriptions
    """
    return CHANNEL_DESCRIPTIONS.copy()


def validate_channel(channel: str) -> bool:
    """
    Validate if a channel name is valid.

    Args:
        channel: Channel name to validate

    Returns:
        True if valid, False otherwise
    """
    return channel in [c.value for c in Channel]
