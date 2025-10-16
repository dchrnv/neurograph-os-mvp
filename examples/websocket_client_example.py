"""
Python WebSocket client example for testing NeuroGraph OS WebSocket API.
"""

import asyncio
import json
from typing import Dict, Any
import websockets
from datetime import datetime


class NeuroGraphWebSocketClient:
    """Simple WebSocket client for NeuroGraph OS."""
    
    def __init__(self, url: str = "ws://localhost:8000/ws"):
        self.url = url
        self.ws = None
        self.connection_id = None
        self.running = False
        
    async def connect(self):
        """Connect to WebSocket server."""
        print(f"Connecting to {self.url}...")
        self.ws = await websockets.connect(self.url)
        self.running = True
        print("✓ Connected!")
        
        # Start receiving messages
        asyncio.create_task(self._receive_loop())
        
    async def disconnect(self):
        """Disconnect from server."""
        self.running = False
        if self.ws:
            await self.ws.close()
            print("✓ Disconnected")
    
    async def send(self, message_type: str, payload: Dict[str, Any] = None):
        """Send a message to the server."""
        if not self.ws:
            raise RuntimeError("Not connected")
        
        message = {
            "id": f"msg_{datetime.now().timestamp()}",
            "type": message_type,
            "payload": payload or {},
            "timestamp": int(datetime.now().timestamp() * 1000)
        }
        
        await self.ws.send(json.dumps(message))
        print(f"→ Sent: {message_type}")
    
    async def _receive_loop(self):
        """Receive and handle messages."""
        try:
            async for message in self.ws:
                data = json.loads(message)
                await self._handle_message(data)
        except websockets.exceptions.ConnectionClosed:
            print("✗ Connection closed")
            self.running = False
    
    async def _handle_message(self, message: Dict[str, Any]):
        """Handle received message."""
        msg_type = message.get("type")
        payload = message.get("payload", {})
        
        print(f"← Received: {msg_type}")
        
        if msg_type == "connection.established":
            self.connection_id = payload.get("connection_id")
            print(f"   Connection ID: {self.connection_id}")
        
        elif msg_type == "pong":
            print("   Pong received")
        
        elif msg_type == "token.created":
            print(f"   Token created: {payload.get('token_id')}")
        
        elif msg_type == "token.data":
            print(f"   Token ID: {payload.get('token_id')}")
            print(f"   Type: {payload.get('type')}")
            print(f"   Weight: {payload.get('weight')}")
        
        elif msg_type == "token.list":
            tokens = payload.get("tokens", [])
            print(f"   Found {len(tokens)} token(s)")
            for token in tokens[:3]:  # Show first 3
                print(f"     - {token.get('token_id')[:8]}... ({token.get('type')})")
        
        elif msg_type == "graph.connected":
            print(f"   Connection: {payload.get('source_id')[:8]}... → {payload.get('target_id')[:8]}...")
        
        elif msg_type == "graph.neighbors":
            neighbors = payload.get("neighbors", [])
            print(f"   Found {len(neighbors)} neighbor(s)")
        
        elif msg_type == "subscribed":
            print(f"   Subscribed to: {payload.get('topic')}")
        
        elif msg_type == "error":
            print(f"   ✗ Error: {payload.get('message')}")
        
        else:
            print(f"   Payload: {json.dumps(payload, indent=2)}")


async def demo():
    """Run demo scenario."""
    client = NeuroGraphWebSocketClient()
    
    try:
        # Connect
        await client.connect()
        await asyncio.sleep(1)
        
        # Test ping
        print("\n1. Testing heartbeat...")
        await client.send("ping")
        await asyncio.sleep(1)
        
        # Subscribe to topics
        print("\n2. Subscribing to topics...")
        await client.send("subscribe", {"topic": "tokens"})
        await client.send("subscribe", {"topic": "graph"})
        await asyncio.sleep(1)
        
        # Create tokens
        print("\n3. Creating tokens...")
        await client.send("token.create", {
            "type": "demo",
            "coord_x": [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            "coord_y": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            "coord_z": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            "weight": 1.0
        })
        await asyncio.sleep(1)
        
        await client.send("token.create", {
            "type": "demo",
            "coord_x": [2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            "coord_y": [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            "coord_z": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            "weight": 1.5
        })
        await asyncio.sleep(1)
        
        # List tokens
        print("\n4. Listing tokens...")
        await client.send("token.list", {"limit": 5})
        await asyncio.sleep(1)
        
        # Keep connection alive
        print("\n5. Keeping connection alive (press Ctrl+C to exit)...")
        while client.running:
            await asyncio.sleep(10)
            await client.send("ping")
    
    except KeyboardInterrupt:
        print("\n\nInterrupted by user")
    
    finally:
        await client.disconnect()


async def interactive_mode():
    """Interactive mode for testing."""
    client = NeuroGraphWebSocketClient()
    
    await client.connect()
    
    print("\n" + "="*60)
    print("Interactive WebSocket Client")
    print("="*60)
    print("\nAvailable commands:")
    print("  ping                          - Send heartbeat")
    print("  subscribe <topic>             - Subscribe to topic")
    print("  token.create                  - Create a token")
    print("  token.list [limit]            - List tokens")
    print("  token.get <id>                - Get token by ID")
    print("  graph.connect <src> <tgt>     - Connect tokens")
    print("  graph.neighbors <id>          - Get neighbors")
    print("  quit                          - Exit\n")
    
    try:
        while True:
            cmd = input(">>> ").strip()
            
            if not cmd:
                continue
            
            if cmd == "quit":
                break
            
            parts = cmd.split()
            command = parts[0]
            
            if command == "ping":
                await client.send("ping")
            
            elif command == "subscribe" and len(parts) > 1:
                await client.send("subscribe", {"topic": parts[1]})
            
            elif command == "token.create":
                await client.send("token.create", {
                    "type": "interactive",
                    "coord_x": [1.0] + [0.0] * 7,
                    "coord_y": [0.0] * 8,
                    "coord_z": [0.0] * 8,
                    "weight": 1.0
                })
            
            elif command == "token.list":
                limit = int(parts[1]) if len(parts) > 1 else 10
                await client.send("token.list", {"limit": limit})
            
            elif command == "token.get" and len(parts) > 1:
                await client.send("token.get", {"token_id": parts[1]})
            
            elif command == "graph.connect" and len(parts) > 2:
                await client.send("graph.connect", {
                    "source_id": parts[1],
                    "target_id": parts[2],
                    "type": "interactive"
                })
            
            elif command == "graph.neighbors" and len(parts) > 1:
                await client.send("graph.neighbors", {"token_id": parts[1]})
            
            else:
                print(f"Unknown command: {cmd}")
            
            await asyncio.sleep(0.5)
    
    except KeyboardInterrupt:
        print("\nExiting...")
    
    finally:
        await client.disconnect()


if __name__ == "__main__":
    import sys
    
    print("""
╔══════════════════════════════════════════════╗
║    NeuroGraph OS - WebSocket Test Client    ║
╚══════════════════════════════════════════════╝
    """)
    
    mode = sys.argv[1] if len(sys.argv) > 1 else "demo"
    
    if mode == "demo":
        print("Running demo mode...\n")
        asyncio.run(demo())
    elif mode == "interactive":
        print("Starting interactive mode...\n")
        asyncio.run(interactive_mode())
    else:
        print(f"Unknown mode: {mode}")
        print("Usage: python websocket_client_example.py [demo|interactive]")