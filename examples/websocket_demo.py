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
WebSocket Demo

Demonstrates NeuroGraph WebSocket client usage for real-time event streaming.

Usage:
    python examples/websocket_demo.py

Requirements:
    pip install websockets

Make sure the NeuroGraph API server is running:
    python -m src.api.main
"""

import asyncio
import sys
from pathlib import Path

# Add client library to path
sys.path.insert(0, str(Path(__file__).parent.parent / "client-libraries" / "python"))

from neurograph_ws_client import NeurographWSClient, Channel


async def main():
    """Demo WebSocket client usage."""

    print("=" * 60)
    print("NeuroGraph WebSocket Demo")
    print("=" * 60)
    print()

    # Create client
    client = NeurographWSClient(
        url="ws://localhost:8000/ws",
        debug=True
    )

    print("Connecting to WebSocket server...")
    try:
        await client.connect()
    except Exception as e:
        print(f"❌ Failed to connect: {e}")
        print("\nMake sure the API server is running:")
        print("  python -m src.api.main")
        return

    print(f"✅ Connected!")
    print(f"   Client ID: {client.get_connection_info().client_id}")
    print()

    # Define event handlers
    def on_metrics(data):
        """Handle metrics events."""
        event_data = data.get("data", {})
        print(f"📊 METRICS: {event_data}")

    def on_signals(data):
        """Handle signal events."""
        event_data = data.get("data", {})
        print(f"🔔 SIGNAL: {event_data}")

    def on_actions(data):
        """Handle action events."""
        event_data = data.get("data", {})
        print(f"⚡ ACTION: {event_data}")

    def on_logs(data):
        """Handle log events."""
        event_data = data.get("data", {})
        level = event_data.get("level", "INFO")
        message = event_data.get("message", "")
        print(f"📝 LOG [{level}]: {message}")

    def on_status(data):
        """Handle status events."""
        event_data = data.get("data", {})
        status = event_data.get("status", "unknown")
        print(f"🔧 STATUS: {status}")

    def on_connections(data):
        """Handle connection events."""
        event_data = data.get("data", {})
        print(f"🔗 CONNECTION: {event_data}")

    # Subscribe to all channels
    print("Subscribing to channels...")
    client.subscribe(Channel.METRICS, on_metrics)
    client.subscribe(Channel.SIGNALS, on_signals)
    client.subscribe(Channel.ACTIONS, on_actions)
    client.subscribe(Channel.LOGS, on_logs)
    client.subscribe(Channel.STATUS, on_status)
    client.subscribe(Channel.CONNECTIONS, on_connections)

    print("✅ Subscribed to all channels!")
    print()
    print("Listening for events... (Press Ctrl+C to stop)")
    print("-" * 60)
    print()

    # Connection event handlers
    def on_connected(info):
        print(f"🟢 Connected: {info}")

    def on_disconnected(info):
        print(f"🔴 Disconnected: {info}")

    def on_error(error):
        print(f"❌ Error: {error}")

    client.on("connected", on_connected)
    client.on("disconnected", on_disconnected)
    client.on("error", on_error)

    # Run forever
    try:
        # Keep the client running and send periodic pings
        while True:
            await asyncio.sleep(30)
            await client.ping()
            print("💓 Ping sent")

    except KeyboardInterrupt:
        print()
        print("-" * 60)
        print("Shutting down...")

    finally:
        await client.disconnect()
        print("✅ Disconnected cleanly")


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\n👋 Goodbye!")
