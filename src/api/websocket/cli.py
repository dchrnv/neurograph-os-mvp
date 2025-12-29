#!/usr/bin/env python3

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
WebSocket CLI Tool

Command-line interface for testing NeuroGraph WebSocket connections.

Usage:
    python -m src.api.websocket.cli --url ws://localhost:8000/ws --subscribe metrics,signals
"""

import asyncio
import argparse
import json
import sys
from typing import List, Optional
from datetime import datetime

try:
    import websockets
    from websockets.client import WebSocketClientProtocol
except ImportError:
    print("Error: websockets library required")
    print("Install: pip install websockets")
    sys.exit(1)


class Colors:
    """ANSI color codes for terminal output."""

    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'


class WebSocketCLI:
    """WebSocket CLI client."""

    def __init__(
        self,
        url: str,
        token: Optional[str] = None,
        subscribe: Optional[List[str]] = None,
        format_output: str = "pretty",
        verbose: bool = False,
    ):
        """
        Initialize WebSocket CLI.

        Args:
            url: WebSocket URL
            token: Optional JWT token
            subscribe: List of channels to subscribe to
            format_output: Output format (pretty, json, compact)
            verbose: Enable verbose logging
        """
        self.url = url
        self.token = token
        self.subscribe_channels = subscribe or []
        self.format_output = format_output
        self.verbose = verbose
        self.ws: Optional[WebSocketClientProtocol] = None
        self.connected = False

    def log(self, message: str, color: str = ""):
        """Print log message."""
        timestamp = datetime.now().strftime("%H:%M:%S.%f")[:-3]
        if color:
            print(f"{color}[{timestamp}] {message}{Colors.ENDC}")
        else:
            print(f"[{timestamp}] {message}")

    def format_message(self, data: dict) -> str:
        """Format message for display."""
        if self.format_output == "json":
            return json.dumps(data, indent=2)
        elif self.format_output == "compact":
            return json.dumps(data)
        else:  # pretty
            msg_type = data.get("type", data.get("channel", "unknown"))
            timestamp = data.get("timestamp", "")

            if data.get("channel"):
                # Channel event
                channel = data["channel"]
                event_type = data.get("event_type", "")
                event_data = data.get("data", {})

                output = f"{Colors.OKCYAN}[{channel}]{Colors.ENDC} "
                output += f"{Colors.BOLD}{event_type}{Colors.ENDC}\n"

                for key, value in event_data.items():
                    output += f"  {key}: {value}\n"

                return output.rstrip()
            else:
                # System message
                return f"{Colors.OKBLUE}[{msg_type}]{Colors.ENDC} {json.dumps(data, indent=2)}"

    async def connect(self):
        """Connect to WebSocket server."""
        # Build URL with token if provided
        url = self.url
        if self.token:
            url += f"?token={self.token}"

        self.log(f"Connecting to {self.url}...", Colors.HEADER)

        try:
            self.ws = await websockets.connect(url)
            self.log("Connected!", Colors.OKGREEN)

            # Wait for connection confirmation
            message = await self.ws.recv()
            data = json.loads(message)

            if data.get("type") == "connected":
                self.connected = True
                client_id = data.get("client_id")
                user_id = data.get("user_id", "anonymous")
                self.log(f"Client ID: {client_id}", Colors.OKGREEN)
                self.log(f"User: {user_id}", Colors.OKGREEN)

        except Exception as e:
            self.log(f"Connection failed: {e}", Colors.FAIL)
            raise

    async def subscribe(self, channels: List[str]):
        """Subscribe to channels."""
        if not self.ws:
            return

        self.log(f"Subscribing to channels: {', '.join(channels)}", Colors.OKCYAN)

        await self.ws.send(json.dumps({
            "type": "subscribe",
            "channels": channels
        }))

        # Wait for confirmation
        message = await self.ws.recv()
        data = json.loads(message)

        if data.get("type") == "subscribed":
            self.log(f"Subscribed to: {', '.join(data.get('channels', []))}", Colors.OKGREEN)

    async def send_ping(self):
        """Send ping."""
        if not self.ws:
            return

        self.log("Sending ping...", Colors.WARNING)
        await self.ws.send(json.dumps({"type": "ping"}))

    async def get_subscriptions(self):
        """Get current subscriptions."""
        if not self.ws:
            return

        self.log("Getting subscriptions...", Colors.WARNING)
        await self.ws.send(json.dumps({"type": "get_subscriptions"}))

    async def listen(self):
        """Listen for messages."""
        if not self.ws:
            return

        self.log("Listening for messages... (Press Ctrl+C to stop)", Colors.HEADER)
        print()

        try:
            async for message in self.ws:
                data = json.loads(message)
                formatted = self.format_message(data)
                print(formatted)
                print()  # Empty line between messages

        except websockets.exceptions.ConnectionClosed:
            self.log("Connection closed", Colors.WARNING)
        except KeyboardInterrupt:
            self.log("Interrupted by user", Colors.WARNING)

    async def interactive(self):
        """Interactive mode."""
        self.log("Interactive mode - available commands:", Colors.HEADER)
        print("  ping                     - Send ping")
        print("  subscribe <channels>     - Subscribe to channels (comma-separated)")
        print("  unsubscribe <channels>   - Unsubscribe from channels")
        print("  subs                     - Get current subscriptions")
        print("  quit                     - Exit")
        print()

        try:
            while self.connected:
                # Read command
                try:
                    command = await asyncio.wait_for(
                        asyncio.get_event_loop().run_in_executor(None, input, "> "),
                        timeout=0.1
                    )
                except asyncio.TimeoutError:
                    # Check for incoming messages
                    try:
                        message = await asyncio.wait_for(self.ws.recv(), timeout=0.1)
                        data = json.loads(message)
                        formatted = self.format_message(data)
                        print(f"\n{formatted}\n")
                    except asyncio.TimeoutError:
                        pass
                    continue

                # Process command
                command = command.strip().lower()

                if command == "quit":
                    break
                elif command == "ping":
                    await self.send_ping()
                elif command == "subs":
                    await self.get_subscriptions()
                elif command.startswith("subscribe "):
                    channels = command.split(" ", 1)[1].split(",")
                    channels = [ch.strip() for ch in channels]
                    await self.subscribe(channels)
                elif command.startswith("unsubscribe "):
                    channels = command.split(" ", 1)[1].split(",")
                    channels = [ch.strip() for ch in channels]
                    await self.ws.send(json.dumps({
                        "type": "unsubscribe",
                        "channels": channels
                    }))
                else:
                    print(f"Unknown command: {command}")

        except KeyboardInterrupt:
            self.log("Interrupted by user", Colors.WARNING)

    async def run(self):
        """Run the CLI."""
        try:
            # Connect
            await self.connect()

            # Subscribe to channels if specified
            if self.subscribe_channels:
                await self.subscribe(self.subscribe_channels)

            # Listen for messages
            await self.listen()

        except KeyboardInterrupt:
            self.log("Interrupted by user", Colors.WARNING)
        except Exception as e:
            self.log(f"Error: {e}", Colors.FAIL)
            if self.verbose:
                import traceback
                traceback.print_exc()
        finally:
            if self.ws:
                await self.ws.close()
                self.log("Disconnected", Colors.WARNING)


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="NeuroGraph WebSocket CLI Tool",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Connect and listen
  python -m src.api.websocket.cli --url ws://localhost:8000/ws

  # Subscribe to channels
  python -m src.api.websocket.cli --url ws://localhost:8000/ws --subscribe metrics,signals

  # With authentication
  python -m src.api.websocket.cli --url ws://localhost:8000/ws --token YOUR_JWT_TOKEN

  # JSON output
  python -m src.api.websocket.cli --url ws://localhost:8000/ws --format json
        """
    )

    parser.add_argument(
        "--url",
        default="ws://localhost:8000/ws",
        help="WebSocket URL (default: ws://localhost:8000/ws)"
    )

    parser.add_argument(
        "--token",
        help="JWT authentication token"
    )

    parser.add_argument(
        "--subscribe",
        help="Channels to subscribe to (comma-separated)"
    )

    parser.add_argument(
        "--format",
        choices=["pretty", "json", "compact"],
        default="pretty",
        help="Output format (default: pretty)"
    )

    parser.add_argument(
        "--verbose",
        action="store_true",
        help="Enable verbose logging"
    )

    parser.add_argument(
        "--interactive",
        action="store_true",
        help="Interactive mode"
    )

    args = parser.parse_args()

    # Parse subscribe channels
    channels = None
    if args.subscribe:
        channels = [ch.strip() for ch in args.subscribe.split(",")]

    # Create CLI instance
    cli = WebSocketCLI(
        url=args.url,
        token=args.token,
        subscribe=channels,
        format_output=args.format,
        verbose=args.verbose,
    )

    # Run
    try:
        asyncio.run(cli.run())
    except KeyboardInterrupt:
        print("\nGoodbye!")
        sys.exit(0)


if __name__ == "__main__":
    main()
