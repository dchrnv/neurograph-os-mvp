
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
WebSocket Connection Manager

Manages WebSocket connections, broadcasting, and channel subscriptions.
"""

import asyncio
import json
import time
from typing import Dict, Set, Optional, Any, List
from datetime import datetime
from fastapi import WebSocket
from collections import defaultdict

from ..logging_config import get_logger

logger = get_logger(__name__, service="websocket")


class ConnectionManager:
    """
    Manages WebSocket connections and message broadcasting.

    Features:
    - Connection lifecycle management
    - Personal and broadcast messaging
    - Channel-based subscriptions
    - Heartbeat/ping-pong mechanism
    - Connection statistics
    """

    def __init__(self):
        # Active connections: client_id -> WebSocket
        self._connections: Dict[str, WebSocket] = {}

        # Client metadata: client_id -> metadata dict
        self._metadata: Dict[str, Dict[str, Any]] = {}

        # Channel subscriptions: channel_name -> set of client_ids
        self._subscriptions: Dict[str, Set[str]] = defaultdict(set)

        # Client subscriptions: client_id -> set of channel_names
        self._client_channels: Dict[str, Set[str]] = defaultdict(set)

        # Event buffers: client_id -> list of buffered events
        self._event_buffers: Dict[str, List[Dict[str, Any]]] = defaultdict(list)

        # Max buffer size per client
        self._max_buffer_size = 1000

        # Heartbeat tracking
        self._last_heartbeat: Dict[str, float] = {}

        logger.info("ConnectionManager initialized")

    async def connect(self, websocket: WebSocket, client_id: str, user_id: Optional[str] = None):
        """
        Register a new WebSocket connection.

        Args:
            websocket: WebSocket connection
            client_id: Unique client identifier
            user_id: Optional authenticated user ID
        """
        await websocket.accept()

        self._connections[client_id] = websocket
        self._metadata[client_id] = {
            "user_id": user_id,
            "connected_at": datetime.utcnow().isoformat(),
            "client_id": client_id
        }
        self._last_heartbeat[client_id] = time.time()

        logger.info(
            f"Client connected: {client_id}",
            extra={
                "event": "client_connected",
                "client_id": client_id,
                "user_id": user_id,
                "total_connections": len(self._connections)
            }
        )

        # Send buffered events if any
        await self._flush_buffer(client_id)

    def disconnect(self, client_id: str):
        """
        Remove a WebSocket connection.

        Args:
            client_id: Client identifier to disconnect
        """
        if client_id in self._connections:
            del self._connections[client_id]

        if client_id in self._metadata:
            metadata = self._metadata.pop(client_id)
            logger.info(
                f"Client disconnected: {client_id}",
                extra={
                    "event": "client_disconnected",
                    "client_id": client_id,
                    "user_id": metadata.get("user_id"),
                    "total_connections": len(self._connections)
                }
            )

        # Clean up subscriptions
        if client_id in self._client_channels:
            channels = self._client_channels.pop(client_id)
            for channel in channels:
                if channel in self._subscriptions:
                    self._subscriptions[channel].discard(client_id)
                    if not self._subscriptions[channel]:
                        del self._subscriptions[channel]

        # Clean up heartbeat tracking
        if client_id in self._last_heartbeat:
            del self._last_heartbeat[client_id]

        # Keep event buffer for potential reconnection (will be cleared later)

    async def send_personal(self, message: Dict[str, Any], client_id: str):
        """
        Send a message to a specific client.

        Args:
            message: Message dictionary to send
            client_id: Target client identifier
        """
        if client_id in self._connections:
            websocket = self._connections[client_id]
            try:
                await websocket.send_json(message)
                logger.debug(
                    f"Message sent to {client_id}",
                    extra={
                        "event": "message_sent",
                        "client_id": client_id,
                        "message_type": message.get("type")
                    }
                )
            except Exception as e:
                logger.error(
                    f"Failed to send message to {client_id}: {e}",
                    extra={
                        "event": "send_failed",
                        "client_id": client_id,
                        "error": str(e)
                    }
                )
                # Connection might be dead, disconnect it
                self.disconnect(client_id)
        else:
            # Client not connected, buffer the message
            self._buffer_event(client_id, message)

    async def broadcast(self, message: Dict[str, Any], exclude: Optional[Set[str]] = None):
        """
        Broadcast a message to all connected clients.

        Args:
            message: Message dictionary to broadcast
            exclude: Optional set of client IDs to exclude from broadcast
        """
        exclude = exclude or set()

        disconnected = []
        for client_id, websocket in self._connections.items():
            if client_id in exclude:
                continue

            try:
                await websocket.send_json(message)
            except Exception as e:
                logger.error(
                    f"Failed to broadcast to {client_id}: {e}",
                    extra={
                        "event": "broadcast_failed",
                        "client_id": client_id,
                        "error": str(e)
                    }
                )
                disconnected.append(client_id)

        # Clean up disconnected clients
        for client_id in disconnected:
            self.disconnect(client_id)

        logger.debug(
            f"Message broadcasted to {len(self._connections) - len(exclude) - len(disconnected)} clients",
            extra={
                "event": "message_broadcasted",
                "recipients": len(self._connections) - len(exclude) - len(disconnected),
                "message_type": message.get("type")
            }
        )

    async def broadcast_to_channel(self, channel: str, message: Dict[str, Any]):
        """
        Broadcast a message to all clients subscribed to a channel.

        Args:
            channel: Channel name
            message: Message dictionary to send
        """
        if channel not in self._subscriptions:
            logger.debug(f"No subscribers for channel: {channel}")
            return

        subscribers = self._subscriptions[channel].copy()
        disconnected = []

        for client_id in subscribers:
            if client_id in self._connections:
                try:
                    await self._connections[client_id].send_json(message)
                except Exception as e:
                    logger.error(
                        f"Failed to send to {client_id} on channel {channel}: {e}",
                        extra={
                            "event": "channel_send_failed",
                            "client_id": client_id,
                            "channel": channel,
                            "error": str(e)
                        }
                    )
                    disconnected.append(client_id)
            else:
                # Client not connected, buffer the event
                self._buffer_event(client_id, message)

        # Clean up disconnected clients
        for client_id in disconnected:
            self.disconnect(client_id)

        logger.debug(
            f"Message sent to {len(subscribers) - len(disconnected)} subscribers on channel '{channel}'",
            extra={
                "event": "channel_broadcast",
                "channel": channel,
                "subscribers": len(subscribers) - len(disconnected),
                "message_type": message.get("type")
            }
        )

    def subscribe(self, client_id: str, channels: List[str]):
        """
        Subscribe a client to one or more channels.

        Args:
            client_id: Client identifier
            channels: List of channel names to subscribe to
        """
        for channel in channels:
            self._subscriptions[channel].add(client_id)
            self._client_channels[client_id].add(channel)

        logger.info(
            f"Client {client_id} subscribed to {len(channels)} channel(s)",
            extra={
                "event": "client_subscribed",
                "client_id": client_id,
                "channels": channels,
                "total_channels": len(self._client_channels[client_id])
            }
        )

    def unsubscribe(self, client_id: str, channels: List[str]):
        """
        Unsubscribe a client from one or more channels.

        Args:
            client_id: Client identifier
            channels: List of channel names to unsubscribe from
        """
        for channel in channels:
            if channel in self._subscriptions:
                self._subscriptions[channel].discard(client_id)
                if not self._subscriptions[channel]:
                    del self._subscriptions[channel]

            if client_id in self._client_channels:
                self._client_channels[client_id].discard(channel)

        logger.info(
            f"Client {client_id} unsubscribed from {len(channels)} channel(s)",
            extra={
                "event": "client_unsubscribed",
                "client_id": client_id,
                "channels": channels
            }
        )

    def get_subscriptions(self, client_id: str) -> Set[str]:
        """
        Get all channels a client is subscribed to.

        Args:
            client_id: Client identifier

        Returns:
            Set of channel names
        """
        return self._client_channels.get(client_id, set())

    def get_channel_subscribers(self, channel: str) -> Set[str]:
        """
        Get all clients subscribed to a channel.

        Args:
            channel: Channel name

        Returns:
            Set of client IDs
        """
        return self._subscriptions.get(channel, set())

    def update_heartbeat(self, client_id: str):
        """
        Update the last heartbeat time for a client.

        Args:
            client_id: Client identifier
        """
        self._last_heartbeat[client_id] = time.time()

    def get_connection_stats(self) -> Dict[str, Any]:
        """
        Get connection statistics.

        Returns:
            Dictionary with connection statistics
        """
        return {
            "total_connections": len(self._connections),
            "total_channels": len(self._subscriptions),
            "total_subscriptions": sum(len(subs) for subs in self._subscriptions.values()),
            "buffered_events": sum(len(buf) for buf in self._event_buffers.values()),
            "clients": [
                {
                    "client_id": client_id,
                    "user_id": self._metadata[client_id].get("user_id"),
                    "connected_at": self._metadata[client_id].get("connected_at"),
                    "channels": list(self._client_channels.get(client_id, [])),
                    "last_heartbeat": self._last_heartbeat.get(client_id, 0)
                }
                for client_id in self._connections.keys()
            ]
        }

    def _buffer_event(self, client_id: str, event: Dict[str, Any]):
        """
        Buffer an event for a disconnected client.

        Args:
            client_id: Client identifier
            event: Event to buffer
        """
        buffer = self._event_buffers[client_id]

        # Add event with timestamp
        buffered_event = {
            **event,
            "buffered_at": datetime.utcnow().isoformat()
        }
        buffer.append(buffered_event)

        # Trim buffer if too large (keep most recent events)
        if len(buffer) > self._max_buffer_size:
            self._event_buffers[client_id] = buffer[-self._max_buffer_size:]
            logger.warning(
                f"Event buffer for {client_id} exceeded limit, trimmed to {self._max_buffer_size}",
                extra={
                    "event": "buffer_trimmed",
                    "client_id": client_id,
                    "buffer_size": self._max_buffer_size
                }
            )

    async def _flush_buffer(self, client_id: str):
        """
        Flush buffered events to a newly connected client.

        Args:
            client_id: Client identifier
        """
        if client_id in self._event_buffers and self._event_buffers[client_id]:
            buffer = self._event_buffers[client_id]

            logger.info(
                f"Flushing {len(buffer)} buffered events to {client_id}",
                extra={
                    "event": "buffer_flushed",
                    "client_id": client_id,
                    "event_count": len(buffer)
                }
            )

            # Send all buffered events
            for event in buffer:
                await self.send_personal(event, client_id)

            # Clear buffer
            self._event_buffers[client_id] = []

    def clear_buffer(self, client_id: str):
        """
        Clear buffered events for a client.

        Args:
            client_id: Client identifier
        """
        if client_id in self._event_buffers:
            del self._event_buffers[client_id]


# Global connection manager instance
connection_manager = ConnectionManager()
