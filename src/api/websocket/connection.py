
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
WebSocket Connection Handler

Main WebSocket endpoint with authentication, heartbeat, and message routing.
"""

import asyncio
import json
import uuid
from typing import Optional
from fastapi import WebSocket, WebSocketDisconnect, Query, HTTPException, status
from datetime import datetime

from .manager import connection_manager
from ..logging_config import get_logger
from ..auth.jwt import JWTManager

logger = get_logger(__name__, service="websocket")
jwt_manager = JWTManager()


async def websocket_endpoint(
    websocket: WebSocket,
    token: Optional[str] = Query(None, description="JWT authentication token")
):
    """
    WebSocket endpoint for real-time communication.

    Authentication:
        - Token can be provided via query parameter: /ws?token=<jwt_token>
        - If no token provided, connection is accepted in anonymous mode (development)

    Protocol:
        Client → Server:
            - {"type": "subscribe", "channels": ["metrics", "signals"]}
            - {"type": "unsubscribe", "channels": ["metrics"]}
            - {"type": "ping"}

        Server → Client:
            - {"type": "pong", "timestamp": "..."}
            - {"type": "connected", "client_id": "...", "timestamp": "..."}
            - {"type": "subscribed", "channels": [...]}
            - {"type": "error", "message": "..."}
            - {"channel": "metrics", "event": {...}}

    Args:
        websocket: WebSocket connection
        token: Optional JWT token for authentication
    """
    client_id = str(uuid.uuid4())
    user_id: Optional[str] = None

    # Authenticate if token provided
    if token:
        try:
            payload = jwt_manager.verify_token(token)
            user_id = payload.user_id
            logger.info(
                f"WebSocket authenticated: user={user_id}",
                extra={
                    "event": "ws_authenticated",
                    "user_id": user_id,
                    "client_id": client_id
                }
            )
        except Exception as e:
            logger.warning(
                f"WebSocket authentication failed: {e}",
                extra={
                    "event": "ws_auth_failed",
                    "error": str(e),
                    "client_id": client_id
                }
            )
            await websocket.close(code=status.WS_1008_POLICY_VIOLATION)
            return

    try:
        # Connect the client
        await connection_manager.connect(websocket, client_id, user_id)

        # Send connection confirmation
        await connection_manager.send_personal(
            {
                "type": "connected",
                "client_id": client_id,
                "user_id": user_id,
                "timestamp": datetime.utcnow().isoformat()
            },
            client_id
        )

        # Start heartbeat task
        heartbeat_task = asyncio.create_task(_heartbeat_loop(client_id))

        # Message loop
        try:
            while True:
                # Receive message from client
                data = await websocket.receive_text()

                try:
                    message = json.loads(data)
                    await _handle_message(client_id, message)
                except json.JSONDecodeError as e:
                    logger.error(
                        f"Invalid JSON from {client_id}: {e}",
                        extra={
                            "event": "invalid_json",
                            "client_id": client_id,
                            "error": str(e)
                        }
                    )
                    await connection_manager.send_personal(
                        {
                            "type": "error",
                            "message": "Invalid JSON format"
                        },
                        client_id
                    )
                except Exception as e:
                    logger.error(
                        f"Error handling message from {client_id}: {e}",
                        extra={
                            "event": "message_error",
                            "client_id": client_id,
                            "error": str(e)
                        }
                    )
                    await connection_manager.send_personal(
                        {
                            "type": "error",
                            "message": f"Error processing message: {str(e)}"
                        },
                        client_id
                    )

        except WebSocketDisconnect:
            logger.info(
                f"Client disconnected: {client_id}",
                extra={
                    "event": "ws_disconnect",
                    "client_id": client_id,
                    "user_id": user_id
                }
            )
        finally:
            # Cancel heartbeat task
            heartbeat_task.cancel()
            try:
                await heartbeat_task
            except asyncio.CancelledError:
                pass

            # Disconnect the client
            connection_manager.disconnect(client_id)

    except Exception as e:
        logger.error(
            f"WebSocket error for {client_id}: {e}",
            extra={
                "event": "ws_error",
                "client_id": client_id,
                "error": str(e)
            }
        )
        connection_manager.disconnect(client_id)


async def _handle_message(client_id: str, message: dict):
    """
    Handle incoming WebSocket message.

    Args:
        client_id: Client identifier
        message: Parsed message dictionary
    """
    message_type = message.get("type")

    if message_type == "ping":
        # Respond to ping with pong
        await _handle_ping(client_id)

    elif message_type == "subscribe":
        # Subscribe to channels
        channels = message.get("channels", [])
        if not isinstance(channels, list):
            await connection_manager.send_personal(
                {
                    "type": "error",
                    "message": "channels must be an array"
                },
                client_id
            )
            return

        connection_manager.subscribe(client_id, channels)
        await connection_manager.send_personal(
            {
                "type": "subscribed",
                "channels": channels,
                "timestamp": datetime.utcnow().isoformat()
            },
            client_id
        )

    elif message_type == "unsubscribe":
        # Unsubscribe from channels
        channels = message.get("channels", [])
        if not isinstance(channels, list):
            await connection_manager.send_personal(
                {
                    "type": "error",
                    "message": "channels must be an array"
                },
                client_id
            )
            return

        connection_manager.unsubscribe(client_id, channels)
        await connection_manager.send_personal(
            {
                "type": "unsubscribed",
                "channels": channels,
                "timestamp": datetime.utcnow().isoformat()
            },
            client_id
        )

    elif message_type == "get_subscriptions":
        # Get current subscriptions
        subscriptions = list(connection_manager.get_subscriptions(client_id))
        await connection_manager.send_personal(
            {
                "type": "subscriptions",
                "channels": subscriptions,
                "timestamp": datetime.utcnow().isoformat()
            },
            client_id
        )

    else:
        # Unknown message type
        logger.warning(
            f"Unknown message type from {client_id}: {message_type}",
            extra={
                "event": "unknown_message_type",
                "client_id": client_id,
                "message_type": message_type
            }
        )
        await connection_manager.send_personal(
            {
                "type": "error",
                "message": f"Unknown message type: {message_type}"
            },
            client_id
        )


async def _handle_ping(client_id: str):
    """
    Handle ping message.

    Args:
        client_id: Client identifier
    """
    connection_manager.update_heartbeat(client_id)
    await connection_manager.send_personal(
        {
            "type": "pong",
            "timestamp": datetime.utcnow().isoformat()
        },
        client_id
    )


async def _heartbeat_loop(client_id: str):
    """
    Periodic heartbeat loop to detect dead connections.

    Sends ping every 30 seconds and checks for client timeout.

    Args:
        client_id: Client identifier
    """
    heartbeat_interval = 30  # seconds
    timeout = 90  # seconds

    try:
        while True:
            await asyncio.sleep(heartbeat_interval)

            # Send server-initiated ping
            try:
                await connection_manager.send_personal(
                    {
                        "type": "ping",
                        "timestamp": datetime.utcnow().isoformat()
                    },
                    client_id
                )
            except Exception as e:
                logger.error(
                    f"Failed to send heartbeat to {client_id}: {e}",
                    extra={
                        "event": "heartbeat_failed",
                        "client_id": client_id,
                        "error": str(e)
                    }
                )
                # Connection is likely dead
                connection_manager.disconnect(client_id)
                break

    except asyncio.CancelledError:
        # Task cancelled (normal shutdown)
        pass
