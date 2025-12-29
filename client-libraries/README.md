# NeuroGraph Client Libraries

Official client libraries for connecting to NeuroGraph WebSocket API.

## Available Clients

### TypeScript/JavaScript Client

**File:** `typescript/neurograph-ws-client.ts`

**Features:**
- ✅ Auto-reconnection with exponential backoff
- ✅ Channel subscription management
- ✅ Event handler registration
- ✅ TypeScript type definitions
- ✅ Connection status tracking
- ✅ Heartbeat/ping-pong support

**Installation:**

```bash
# Copy the file to your project
cp typescript/neurograph-ws-client.ts your-project/src/

# Install WebSocket library (Node.js only)
npm install ws @types/ws
```

**Usage:**

```typescript
import NeurographWSClient from "./neurograph-ws-client";

const client = new NeurographWSClient({
  url: "ws://localhost:8000/ws",
  token: "your-jwt-token",  // Optional
  autoReconnect: true,
  debug: true
});

// Connect
await client.connect();

// Subscribe to channels
client.subscribe("metrics", (data) => {
  console.log("Metrics:", data.data);
});

client.subscribe("signals", (data) => {
  console.log("Signal:", data.data);
});

// Handle connection events
client.on("connected", (info) => {
  console.log("Connected:", info.client_id);
});

client.on("disconnected", (info) => {
  console.log("Disconnected:", info.reason);
});

client.on("error", (error) => {
  console.error("Error:", error);
});

// Send ping
client.ping();

// Get current subscriptions
const subs = await client.getSubscriptions();
console.log("Subscriptions:", subs);

// Disconnect
client.disconnect();
```

---

### Python Client

**File:** `python/neurograph_ws_client.py`

**Features:**
- ✅ Asyncio-based WebSocket client
- ✅ Auto-reconnection support
- ✅ Channel subscription API
- ✅ Event handler callbacks
- ✅ Run forever mode
- ✅ Connection info tracking

**Installation:**

```bash
# Install dependencies
pip install websockets

# Copy the file to your project
cp python/neurograph_ws_client.py your-project/
```

**Usage:**

```python
import asyncio
from neurograph_ws_client import NeurographWSClient, Channel

async def main():
    # Create client
    client = NeurographWSClient(
        url="ws://localhost:8000/ws",
        token="your-jwt-token",  # Optional
        auto_reconnect=True,
        debug=True
    )

    # Connect
    await client.connect()

    # Subscribe to channels
    def on_metrics(data):
        print(f"Metrics: {data['data']}")

    client.subscribe(Channel.METRICS, on_metrics)
    client.subscribe(Channel.SIGNALS, lambda d: print(f"Signal: {d}"))

    # Handle connection events
    client.on("connected", lambda info: print(f"Connected: {info.client_id}"))
    client.on("disconnected", lambda info: print("Disconnected"))
    client.on("error", lambda err: print(f"Error: {err}"))

    # Send ping
    await client.ping()

    # Get current subscriptions
    subs = await client.getSubscriptions()
    print(f"Subscriptions: {subs}")

    # Run forever
    await client.run_forever()

    # Or manually manage
    # await asyncio.sleep(60)
    # await client.disconnect()

if __name__ == "__main__":
    asyncio.run(main())
```

---

## Available Channels

| Channel | Description |
|---------|-------------|
| `metrics` | System metrics (CPU, memory, events/sec) |
| `signals` | Signal events from SignalSystem |
| `actions` | Action execution notifications |
| `logs` | System logs and debug information |
| `status` | System status changes |
| `connections` | WebSocket connection events |

---

## Protocol Documentation

### Client → Server Messages

**Subscribe to channels:**
```json
{
  "type": "subscribe",
  "channels": ["metrics", "signals"]
}
```

**Unsubscribe from channels:**
```json
{
  "type": "unsubscribe",
  "channels": ["metrics"]
}
```

**Ping:**
```json
{
  "type": "ping"
}
```

**Get subscriptions:**
```json
{
  "type": "get_subscriptions"
}
```

### Server → Client Messages

**Connection confirmation:**
```json
{
  "type": "connected",
  "client_id": "uuid-here",
  "user_id": "user-id-or-null",
  "timestamp": "2024-12-29T12:00:00Z"
}
```

**Channel event:**
```json
{
  "channel": "metrics",
  "timestamp": "2024-12-29T12:00:00Z",
  "event_type": "system_metrics",
  "data": {
    "cpu_percent": 45.2,
    "memory_mb": 1024,
    "events_per_sec": 5000
  }
}
```

**Pong:**
```json
{
  "type": "pong",
  "timestamp": "2024-12-29T12:00:00Z"
}
```

**Error:**
```json
{
  "type": "error",
  "message": "Error description"
}
```

---

## Examples

### Basic Example (JavaScript)

```javascript
const ws = new WebSocket("ws://localhost:8000/ws");

ws.onopen = () => {
  console.log("Connected");

  // Subscribe to metrics
  ws.send(JSON.stringify({
    type: "subscribe",
    channels: ["metrics"]
  }));
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log("Received:", data);
};
```

### React Example

```typescript
import { useEffect, useState } from 'react';
import NeurographWSClient from './neurograph-ws-client';

function MetricsComponent() {
  const [metrics, setMetrics] = useState(null);
  const [client, setClient] = useState<NeurographWSClient | null>(null);

  useEffect(() => {
    const ws = new NeurographWSClient({
      url: "ws://localhost:8000/ws",
      autoReconnect: true
    });

    ws.connect().then(() => {
      ws.subscribe("metrics", (data) => {
        setMetrics(data.data);
      });
    });

    setClient(ws);

    return () => {
      ws.disconnect();
    };
  }, []);

  return (
    <div>
      <h2>System Metrics</h2>
      {metrics && (
        <pre>{JSON.stringify(metrics, null, 2)}</pre>
      )}
    </div>
  );
}
```

### Python CLI Example

```python
import asyncio
from neurograph_ws_client import NeurographWSClient, Channel

async def main():
    client = NeurographWSClient(url="ws://localhost:8000/ws")
    await client.connect()

    # Print all metrics
    client.subscribe(Channel.METRICS, lambda d: print(d['data']))

    # Run until Ctrl+C
    try:
        await client.run_forever()
    except KeyboardInterrupt:
        await client.disconnect()

asyncio.run(main())
```

---

## Testing

### Test Server Connection

**Using wscat (CLI):**
```bash
npm install -g wscat
wscat -c ws://localhost:8000/ws

# Send messages
> {"type": "subscribe", "channels": ["metrics"]}
> {"type": "ping"}
```

**Using websocat:**
```bash
websocat ws://localhost:8000/ws
```

**Using Python:**
```python
import asyncio
import websockets

async def test():
    async with websockets.connect("ws://localhost:8000/ws") as ws:
        # Wait for connection
        msg = await ws.recv()
        print("Connected:", msg)

        # Subscribe
        await ws.send('{"type": "subscribe", "channels": ["metrics"]}')

        # Listen
        while True:
            msg = await ws.recv()
            print("Received:", msg)

asyncio.run(test())
```

---

## Authentication

### With JWT Token

**TypeScript:**
```typescript
const client = new NeurographWSClient({
  url: "ws://localhost:8000/ws",
  token: "your-jwt-token"
});
```

**Python:**
```python
client = NeurographWSClient(
    url="ws://localhost:8000/ws",
    token="your-jwt-token"
)
```

**Raw WebSocket:**
```javascript
const ws = new WebSocket("ws://localhost:8000/ws?token=your-jwt-token");
```

---

## Troubleshooting

### Connection Failed

**Problem:** Cannot connect to WebSocket server

**Solution:**
1. Make sure the API server is running: `python -m src.api.main`
2. Check the WebSocket URL is correct (default: `ws://localhost:8000/ws`)
3. Check firewall settings

### No Events Received

**Problem:** Connected but not receiving events

**Solution:**
1. Make sure you subscribed to channels: `client.subscribe("metrics", handler)`
2. Check that events are being generated on the server
3. Enable debug mode: `debug: true`

### Disconnects Frequently

**Problem:** WebSocket disconnects after a few seconds

**Solution:**
1. Enable auto-reconnect: `autoReconnect: true`
2. Check network stability
3. Check server logs for errors

---

## Performance

**Recommended Settings:**

- **Maximum concurrent connections:** 1000+ per server
- **Event latency:** < 10ms typical
- **Reconnection delay:** 3s recommended
- **Max reconnect attempts:** 10 recommended

**Optimization Tips:**

1. Use auto-reconnect for production
2. Batch subscribe to multiple channels at once
3. Unsubscribe from unused channels
4. Enable compression for large payloads (future)

---

## Contributing

To add a new client library:

1. Create a new directory: `client-libraries/your-language/`
2. Implement the client following the protocol
3. Add tests
4. Update this README
5. Submit a pull request

---

## License

AGPLv3

Copyright (C) 2024-2025 Chernov Denys

---

## Support

- **Documentation:** [docs/changelogs/CHANGELOG_v0.60.0.md](../docs/changelogs/CHANGELOG_v0.60.0.md)
- **Issues:** https://github.com/anthropics/neurograph-os-mvp/issues
- **Discord:** Coming soon

---

*Last updated: 2024-12-29*
*Version: v0.60.0*
