
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
WebSocket Management API

REST endpoints for WebSocket status and management.
"""

from fastapi import APIRouter, Depends
from typing import Dict, Any, List

from ..websocket.manager import connection_manager
from ..websocket.channels import get_all_channels, get_channel_info, Channel
from ..auth.dependencies import get_optional_user
from ..logging_config import get_logger

logger = get_logger(__name__, service="api")
router = APIRouter()


@router.get("/websocket/status")
async def get_websocket_status(
    user=Depends(get_optional_user)
) -> Dict[str, Any]:
    """
    Get WebSocket connection status and statistics.

    Returns:
        - total_connections: Number of active WebSocket connections
        - total_channels: Number of active channels
        - total_subscriptions: Total number of channel subscriptions
        - buffered_events: Number of buffered events across all clients
        - clients: List of connected clients with their details
    """
    stats = connection_manager.get_connection_stats()

    return {
        "status": "ok",
        "websocket": {
            "endpoint": "/ws",
            "protocol": "ws",
            **stats
        }
    }


@router.get("/websocket/channels")
async def get_channels(
    user=Depends(get_optional_user)
) -> Dict[str, Any]:
    """
    Get list of available WebSocket channels.

    Returns:
        - channels: List of available channel names
        - descriptions: Channel descriptions
    """
    return {
        "channels": get_all_channels(),
        "descriptions": get_channel_info()
    }


@router.get("/websocket/channels/{channel}")
async def get_channel_status(
    channel: str,
    user=Depends(get_optional_user)
) -> Dict[str, Any]:
    """
    Get status of a specific channel.

    Args:
        channel: Channel name

    Returns:
        - channel: Channel name
        - subscribers: Number of subscribers
        - subscriber_ids: List of client IDs subscribed to this channel
    """
    subscribers = connection_manager.get_channel_subscribers(channel)

    return {
        "channel": channel,
        "subscribers": len(subscribers),
        "subscriber_ids": list(subscribers)
    }


@router.post("/websocket/broadcast")
async def broadcast_message(
    channel: str,
    message: Dict[str, Any],
    user=Depends(get_optional_user)
) -> Dict[str, Any]:
    """
    Broadcast a message to a channel (admin only in production).

    Args:
        channel: Target channel
        message: Message to broadcast

    Returns:
        - status: "ok" if successful
        - channel: Target channel
        - subscribers: Number of recipients
    """
    subscribers = connection_manager.get_channel_subscribers(channel)

    # Format message with channel
    event = {
        "channel": channel,
        **message
    }

    # Broadcast to channel
    await connection_manager.broadcast_to_channel(channel, event)

    return {
        "status": "ok",
        "channel": channel,
        "subscribers": len(subscribers),
        "message_sent": True
    }
