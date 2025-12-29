
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
from .metrics import metrics
from .permissions import can_subscribe, can_broadcast
from .rate_limit import rate_limiter
from .reconnection import reconnection_manager
from ..logging_config import get_logger
from ..auth.jwt import JWTManager

logger = get_logger(__name__, service="websocket")
jwt_manager = JWTManager()


async def websocket_endpoint(
    websocket: WebSocket,
    token: Optional[str] = Query(None, description="JWT authentication token"),
    reconnection_token: Optional[str] = Query(None, description="Reconnection token for session restoration")
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
    user_role: Optional[str] = "anonymous"
    restored_session = None

    # Try to restore session from reconnection token
    if reconnection_token:
        try:
            restored_session = reconnection_manager.restore_session(reconnection_token, client_id)
            if restored_session:
                user_id = restored_session["user_id"]
                user_role = restored_session.get("metadata", {}).get("role", "viewer")
                logger.info(
                    f"Session restored from reconnection token: user={user_id}",
                    extra={
                        "event": "session_restored",
                        "user_id": user_id,
                        "client_id": client_id,
                        "subscriptions": restored_session.get("subscriptions", [])
                    }
                )
        except Exception as e:
            logger.warning(
                f"Failed to restore session: {e}",
                extra={
                    "event": "session_restore_failed",
                    "error": str(e),
                    "client_id": client_id
                }
            )

    # Authenticate if token provided (and no session restored)
    if token and not restored_session:
        try:
            payload = jwt_manager.verify_token(token)
            user_id = payload.user_id
            user_role = payload.role if hasattr(payload, 'role') else "viewer"
            logger.info(
                f"WebSocket authenticated: user={user_id}, role={user_role}",
                extra={
                    "event": "ws_authenticated",
                    "user_id": user_id,
                    "role": user_role,
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

        # Restore subscriptions if session was restored
        if restored_session and restored_session.get("subscriptions"):
            restored_channels = restored_session["subscriptions"]
            connection_manager.subscribe(client_id, restored_channels)
            logger.info(
                f"Restored {len(restored_channels)} channel subscriptions",
                extra={
                    "event": "subscriptions_restored",
                    "client_id": client_id,
                    "channels": restored_channels
                }
            )

        # Send connection confirmation
        await connection_manager.send_personal(
            {
                "type": "connected",
                "client_id": client_id,
                "user_id": user_id,
                "reconnected": restored_session is not None,
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

                    # Track message received
                    message_type = message.get("type", "unknown")
                    metrics.track_message_received(message_type, len(data.encode('utf-8')))

                    await _handle_message(client_id, message, user_role)
                except json.JSONDecodeError as e:
                    metrics.track_error("invalid_json")
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
                    metrics.track_error("message_handling")
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

            # Create reconnection token before disconnecting
            if user_id:
                subscriptions = list(connection_manager.get_subscriptions(client_id))
                metadata = {"role": user_role}

                try:
                    recon_token = reconnection_manager.create_reconnection_token(
                        client_id=client_id,
                        user_id=user_id,
                        subscriptions=subscriptions,
                        metadata=metadata
                    )
                    logger.info(
                        f"Created reconnection token for {client_id}",
                        extra={
                            "event": "reconnection_token_created",
                            "client_id": client_id,
                            "user_id": user_id,
                            "subscriptions_count": len(subscriptions)
                        }
                    )
                except Exception as e:
                    logger.error(
                        f"Failed to create reconnection token: {e}",
                        extra={
                            "event": "reconnection_token_failed",
                            "client_id": client_id,
                            "error": str(e)
                        }
                    )

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


async def _handle_message(client_id: str, message: dict, user_role: Optional[str] = "anonymous"):
    """
    Handle incoming WebSocket message.

    Args:
        client_id: Client identifier
        message: Parsed message dictionary
        user_role: User's role for permission checking
    """
    message_type = message.get("type")

    # Check rate limit
    allowed, retry_after = rate_limiter.check_rate_limit(client_id, message_type)
    if not allowed:
        logger.warning(
            f"Rate limit exceeded for {client_id}, message type: {message_type}",
            extra={
                "event": "rate_limit_exceeded",
                "client_id": client_id,
                "message_type": message_type,
                "retry_after": retry_after
            }
        )
        await connection_manager.send_personal(
            {
                "type": "error",
                "message": "Rate limit exceeded",
                "retry_after": retry_after
            },
            client_id
        )
        return

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

        # Check permissions for each channel
        allowed_channels = []
        denied_channels = []

        for channel in channels:
            if can_subscribe(channel, user_role):
                allowed_channels.append(channel)
            else:
                denied_channels.append(channel)
                logger.warning(
                    f"Permission denied: {client_id} (role={user_role}) cannot subscribe to {channel}",
                    extra={
                        "event": "permission_denied",
                        "client_id": client_id,
                        "channel": channel,
                        "role": user_role
                    }
                )

        # Subscribe to allowed channels
        if allowed_channels:
            connection_manager.subscribe(client_id, allowed_channels)

        # Send response
        response = {
            "type": "subscribed",
            "channels": allowed_channels,
            "timestamp": datetime.utcnow().isoformat()
        }

        if denied_channels:
            response["denied"] = denied_channels
            response["message"] = f"Permission denied for channels: {', '.join(denied_channels)}"

        await connection_manager.send_personal(response, client_id)

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

    elif message_type == "get_reconnection_token":
        # Create and return a reconnection token
        metadata_obj = connection_manager._metadata.get(client_id, {})
        user_id_from_meta = metadata_obj.get("user_id")

        if not user_id_from_meta:
            await connection_manager.send_personal(
                {
                    "type": "error",
                    "message": "Reconnection tokens are only available for authenticated users"
                },
                client_id
            )
            return

        subscriptions = list(connection_manager.get_subscriptions(client_id))
        metadata = {"role": user_role}

        try:
            recon_token = reconnection_manager.create_reconnection_token(
                client_id=client_id,
                user_id=user_id_from_meta,
                subscriptions=subscriptions,
                metadata=metadata
            )

            await connection_manager.send_personal(
                {
                    "type": "reconnection_token",
                    "token": recon_token,
                    "expires_in": 300,  # 5 minutes
                    "timestamp": datetime.utcnow().isoformat()
                },
                client_id
            )

            logger.info(
                f"Reconnection token issued to {client_id}",
                extra={
                    "event": "reconnection_token_issued",
                    "client_id": client_id,
                    "user_id": user_id_from_meta
                }
            )
        except Exception as e:
            logger.error(
                f"Failed to create reconnection token: {e}",
                extra={
                    "event": "reconnection_token_error",
                    "client_id": client_id,
                    "error": str(e)
                }
            )
            await connection_manager.send_personal(
                {
                    "type": "error",
                    "message": "Failed to create reconnection token"
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
