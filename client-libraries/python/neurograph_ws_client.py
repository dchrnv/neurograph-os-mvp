
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
NeuroGraph WebSocket Client (Python)

Python client for NeuroGraph WebSocket API
Version: v0.60.0

Example:
    >>> from neurograph_ws_client import NeurographWSClient
    >>>
    >>> client = NeurographWSClient("ws://localhost:8000/ws")
    >>> await client.connect()
    >>>
    >>> def on_metrics(data):
    ...     print(f"Metrics: {data}")
    >>>
    >>> client.subscribe("metrics", on_metrics)
    >>>
    >>> # Keep running
    >>> await client.run_forever()
"""

import asyncio
import json
import logging
from typing import Dict, Any, Callable, Optional, Set, List
from dataclasses import dataclass
from urllib.parse import urlencode

try:
    import websockets
    from websockets.client import WebSocketClientProtocol
except ImportError:
    raise ImportError("websockets library required: pip install websockets")


logger = logging.getLogger(__name__)


@dataclass
class ConnectionInfo:
    """WebSocket connection information."""

    client_id: str
    user_id: Optional[str]
    timestamp: str


class Channel:
    """Available WebSocket channels."""

    METRICS = "metrics"
    SIGNALS = "signals"
    ACTIONS = "actions"
    LOGS = "logs"
    STATUS = "status"
    CONNECTIONS = "connections"


EventHandler = Callable[[Dict[str, Any]], None]


class NeurographWSClient:
    """
    Python WebSocket client for NeuroGraph real-time events.

    Args:
        url: WebSocket URL (default: ws://localhost:8000/ws)
        token: Optional JWT authentication token
        auto_reconnect: Auto-reconnect on disconnect (default: True)
        reconnect_delay: Reconnect delay in seconds (default: 3)
        max_reconnect_attempts: Maximum reconnection attempts (default: 10)
        debug: Enable debug logging (default: False)

    Example:
        >>> client = NeurographWSClient(
        ...     url="ws://localhost:8000/ws",
        ...     token="your-jwt-token"
        ... )
        >>> await client.connect()
        >>> client.subscribe("metrics", lambda data: print(data))
    """

    def __init__(
        self,
        url: str = "ws://localhost:8000/ws",
        token: Optional[str] = None,
        auto_reconnect: bool = True,
        reconnect_delay: float = 3.0,
        max_reconnect_attempts: int = 10,
        debug: bool = False
    ):
        self.url = url
        self.token = token
        self.auto_reconnect = auto_reconnect
        self.reconnect_delay = reconnect_delay
        self.max_reconnect_attempts = max_reconnect_attempts
        self.debug = debug

        self.ws: Optional[WebSocketClientProtocol] = None
        self.subscriptions: Dict[str, Set[EventHandler]] = {}
        self.event_handlers: Dict[str, Set[EventHandler]] = {}
        self.connection_info: Optional[ConnectionInfo] = None
        self.connected = False
        self.reconnect_attempts = 0
        self._running = False
        self._receive_task: Optional[asyncio.Task] = None

        # Configure logging
        if debug:
            logging.basicConfig(level=logging.DEBUG)
        else:
            logging.basicConfig(level=logging.INFO)

    async def connect(self) -> None:
        """
        Connect to WebSocket server.

        Raises:
            Exception: If connection fails
        """
        # Build URL with token if provided
        url = self.url
        if self.token:
            params = {"token": self.token}
            url += "?" + urlencode(params)

        logger.info(f"Connecting to {url}")

        try:
            self.ws = await websockets.connect(url)
            logger.info("WebSocket connection established")

            # Wait for connection confirmation
            message = await self.ws.recv()
            data = json.loads(message)

            if data.get("type") == "connected":
                self.connected = True
                self.connection_info = ConnectionInfo(
                    client_id=data["client_id"],
                    user_id=data.get("user_id"),
                    timestamp=data["timestamp"]
                )
                self.reconnect_attempts = 0

                logger.info(f"Connected with client_id: {self.connection_info.client_id}")
                self._emit("connected", self.connection_info)

                # Start receiving messages
                if not self._receive_task or self._receive_task.done():
                    self._receive_task = asyncio.create_task(self._receive_loop())

        except Exception as e:
            logger.error(f"Connection failed: {e}")
            raise

    async def disconnect(self) -> None:
        """Disconnect from WebSocket server."""
        self._running = False

        if self._receive_task and not self._receive_task.done():
            self._receive_task.cancel()
            try:
                await self._receive_task
            except asyncio.CancelledError:
                pass

        if self.ws:
            await self.ws.close()
            self.ws = None

        self.connected = False
        self.connection_info = None
        logger.info("Disconnected")

    def subscribe(self, channel: str, handler: EventHandler) -> None:
        """
        Subscribe to a channel.

        Args:
            channel: Channel name (e.g., "metrics", "signals")
            handler: Event handler function
        """
        if channel not in self.subscriptions:
            self.subscriptions[channel] = set()

        self.subscriptions[channel].add(handler)

        # Send subscription request
        asyncio.create_task(self._send({
            "type": "subscribe",
            "channels": [channel]
        }))

        logger.info(f"Subscribed to channel: {channel}")

    def subscribe_multiple(self, channels: List[str], handler: EventHandler) -> None:
        """
        Subscribe to multiple channels at once.

        Args:
            channels: List of channel names
            handler: Event handler function (receives events from all channels)
        """
        for channel in channels:
            if channel not in self.subscriptions:
                self.subscriptions[channel] = set()
            self.subscriptions[channel].add(handler)

        # Send subscription request
        asyncio.create_task(self._send({
            "type": "subscribe",
            "channels": channels
        }))

        logger.info(f"Subscribed to channels: {channels}")

    def unsubscribe(self, channel: str, handler: Optional[EventHandler] = None) -> None:
        """
        Unsubscribe from a channel.

        Args:
            channel: Channel name
            handler: Optional specific handler to remove
        """
        if handler:
            # Remove specific handler
            if channel in self.subscriptions:
                self.subscriptions[channel].discard(handler)
                if not self.subscriptions[channel]:
                    del self.subscriptions[channel]
        else:
            # Remove all handlers
            if channel in self.subscriptions:
                del self.subscriptions[channel]

        # Send unsubscription request
        asyncio.create_task(self._send({
            "type": "unsubscribe",
            "channels": [channel]
        }))

        logger.info(f"Unsubscribed from channel: {channel}")

    def on(self, event: str, handler: EventHandler) -> None:
        """
        Register an event handler for internal events.

        Events:
            - "connected" - Connection established
            - "disconnected" - Connection closed
            - "error" - Error occurred
            - "pong" - Pong response received

        Args:
            event: Event name
            handler: Event handler function
        """
        if event not in self.event_handlers:
            self.event_handlers[event] = set()

        self.event_handlers[event].add(handler)

    def off(self, event: str, handler: EventHandler) -> None:
        """
        Remove an event handler.

        Args:
            event: Event name
            handler: Event handler function
        """
        if event in self.event_handlers:
            self.event_handlers[event].discard(handler)

    async def ping(self) -> None:
        """Send a ping to the server."""
        await self._send({"type": "ping"})

    async def get_subscriptions(self) -> List[str]:
        """
        Get current subscriptions.

        Returns:
            List of channel names
        """
        # Send request
        await self._send({"type": "get_subscriptions"})

        # Wait for response (simplified - should use future/promise pattern)
        # For now, just return local subscriptions
        return list(self.subscriptions.keys())

    def is_connected(self) -> bool:
        """Check if client is connected."""
        return self.connected

    def get_connection_info(self) -> Optional[ConnectionInfo]:
        """Get connection information."""
        return self.connection_info

    async def run_forever(self) -> None:
        """
        Run the client forever with auto-reconnect.

        This is a blocking call that will keep the client running.
        """
        self._running = True

        while self._running:
            try:
                if not self.connected:
                    await self.connect()

                # Keep connection alive
                await asyncio.sleep(1)

            except Exception as e:
                logger.error(f"Error in run loop: {e}")

                if self.auto_reconnect and self.reconnect_attempts < self.max_reconnect_attempts:
                    self.reconnect_attempts += 1
                    logger.info(f"Reconnecting in {self.reconnect_delay}s (attempt {self.reconnect_attempts}/{self.max_reconnect_attempts})")
                    await asyncio.sleep(self.reconnect_delay)
                else:
                    logger.error("Max reconnection attempts reached")
                    break

    # Private methods

    async def _send(self, data: Dict[str, Any]) -> None:
        """Send a message to the server."""
        if not self.ws:
            logger.warning("Cannot send message: not connected")
            return

        try:
            await self.ws.send(json.dumps(data))
        except Exception as e:
            logger.error(f"Failed to send message: {e}")

    async def _receive_loop(self) -> None:
        """Receive messages from server."""
        try:
            async for message in self.ws:
                await self._handle_message(message)
        except websockets.exceptions.ConnectionClosed:
            logger.info("Connection closed by server")
            self.connected = False
            self._emit("disconnected", {})

            if self.auto_reconnect and self.reconnect_attempts < self.max_reconnect_attempts:
                await asyncio.sleep(self.reconnect_delay)
                try:
                    await self.connect()
                except Exception as e:
                    logger.error(f"Reconnection failed: {e}")
        except Exception as e:
            logger.error(f"Error in receive loop: {e}")
            self._emit("error", {"error": str(e)})

    async def _handle_message(self, message: str) -> None:
        """Handle incoming message."""
        try:
            data = json.loads(message)

            # Handle system messages
            if data.get("type") == "pong":
                self._emit("pong", data)
                return

            if data.get("type") == "error":
                logger.error(f"Server error: {data.get('message')}")
                self._emit("error", data)
                return

            if data.get("type") == "subscribed":
                logger.debug(f"Subscription confirmed: {data.get('channels')}")
                return

            if data.get("type") == "unsubscribed":
                logger.debug(f"Unsubscription confirmed: {data.get('channels')}")
                return

            # Handle channel events
            if "channel" in data:
                channel = data["channel"]
                if channel in self.subscriptions:
                    for handler in self.subscriptions[channel]:
                        try:
                            handler(data)
                        except Exception as e:
                            logger.error(f"Error in event handler: {e}")

        except json.JSONDecodeError as e:
            logger.error(f"Failed to parse message: {e}")
        except Exception as e:
            logger.error(f"Error handling message: {e}")

    def _emit(self, event: str, data: Any) -> None:
        """Emit an internal event."""
        if event in self.event_handlers:
            for handler in self.event_handlers[event]:
                try:
                    handler(data)
                except Exception as e:
                    logger.error(f"Error in event handler: {e}")


# Example usage
async def main():
    """Example usage of the WebSocket client."""
    client = NeurographWSClient(
        url="ws://localhost:8000/ws",
        debug=True
    )

    # Connect
    await client.connect()

    # Subscribe to metrics
    def on_metrics(data):
        print(f"Metrics received: {data}")

    client.subscribe("metrics", on_metrics)

    # Subscribe to signals
    client.subscribe("signals", lambda data: print(f"Signal: {data}"))

    # Run forever
    await client.run_forever()


if __name__ == "__main__":
    asyncio.run(main())
