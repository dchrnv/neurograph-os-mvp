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
WebSocket Advanced Features Demo

Демонстрирует новые возможности v0.60.1:
- Reconnection tokens для бесшовного восстановления сессии
- Permissions checking (RBAC)
- Rate limiting
- Binary messages
- Message compression
- Prometheus metrics

Usage:
    python examples/websocket_advanced_demo.py
"""

import asyncio
import json
from datetime import datetime
from client_libraries.python.neurograph_ws_client import NeurographWSClient, Channel


async def demo_reconnection():
    """Демонстрация reconnection tokens."""
    print("\n" + "="*60)
    print("DEMO 1: Reconnection Tokens")
    print("="*60)

    # Подключаемся к серверу
    client = NeurographWSClient(url="ws://localhost:8000/ws")

    try:
        await client.connect()
        print(f"✅ Connected: {client.client_id}")

        # Подписываемся на каналы
        await client.send({
            "type": "subscribe",
            "channels": ["metrics", "signals"]
        })

        response = await client.receive()
        print(f"✅ Subscribed to: {response.get('channels', [])}")

        # Запрашиваем reconnection token
        print("\n📝 Requesting reconnection token...")
        await client.send({"type": "get_reconnection_token"})

        token_response = await client.receive()
        reconnection_token = token_response.get("token")
        expires_in = token_response.get("expires_in", 300)

        print(f"✅ Received reconnection token (expires in {expires_in}s)")
        print(f"   Token: {reconnection_token[:20]}...")

        # Закрываем соединение
        await client.close()
        print("\n🔌 Connection closed")

        # Ждем немного
        await asyncio.sleep(2)

        # Переподключаемся с reconnection token
        print("\n🔄 Reconnecting with token...")
        new_client = NeurographWSClient(
            url=f"ws://localhost:8000/ws?reconnection_token={reconnection_token}"
        )

        await new_client.connect()

        # Ждем подтверждение подключения
        connect_msg = await new_client.receive()

        if connect_msg.get("reconnected"):
            print("✅ Session restored successfully!")
            print(f"   Client ID: {connect_msg.get('client_id')}")
            print(f"   User ID: {connect_msg.get('user_id')}")

        # Проверяем что подписки восстановлены
        await new_client.send({"type": "get_subscriptions"})
        subs_response = await new_client.receive()

        print(f"✅ Subscriptions restored: {subs_response.get('channels', [])}")

        await new_client.close()

    except Exception as e:
        print(f"❌ Error: {e}")


async def demo_permissions():
    """Демонстрация permissions checking."""
    print("\n" + "="*60)
    print("DEMO 2: Permissions & RBAC")
    print("="*60)

    # Анонимный пользователь
    client = NeurographWSClient(url="ws://localhost:8000/ws")

    try:
        await client.connect()
        print("✅ Connected as anonymous user")

        # Пытаемся подписаться на разные каналы
        print("\n📝 Trying to subscribe to all channels...")
        await client.send({
            "type": "subscribe",
            "channels": ["metrics", "signals", "actions", "logs", "status", "connections"]
        })

        response = await client.receive()

        allowed = response.get("channels", [])
        denied = response.get("denied", [])

        print(f"\n✅ Allowed channels: {allowed}")
        if denied:
            print(f"❌ Denied channels: {denied}")
            print(f"   Reason: {response.get('message', 'No permission')}")

        print("\n📊 Anonymous user permissions:")
        print("   ✅ metrics  - Public channel")
        print("   ✅ status   - Public channel")
        print("   ❌ signals  - Requires authentication")
        print("   ❌ actions  - Requires developer role")
        print("   ❌ logs     - Requires developer role")
        print("   ❌ connections - Admin only")

        await client.close()

    except Exception as e:
        print(f"❌ Error: {e}")


async def demo_rate_limiting():
    """Демонстрация rate limiting."""
    print("\n" + "="*60)
    print("DEMO 3: Rate Limiting")
    print("="*60)

    client = NeurographWSClient(url="ws://localhost:8000/ws")

    try:
        await client.connect()
        print("✅ Connected")

        print("\n📝 Sending rapid ping requests...")

        # Отправляем много ping запросов быстро
        success_count = 0
        rate_limited_count = 0

        for i in range(150):
            await client.send({"type": "ping"})

            response = await client.receive()

            if response.get("type") == "pong":
                success_count += 1
            elif response.get("type") == "error":
                if "rate limit" in response.get("message", "").lower():
                    rate_limited_count += 1
                    retry_after = response.get("retry_after", 0)

                    if rate_limited_count == 1:
                        print(f"\n⚠️  Rate limit exceeded!")
                        print(f"   Retry after: {retry_after:.2f}s")
                        print(f"   Successful requests: {success_count}")
                        break

        print(f"\n📊 Rate Limiting Stats:")
        print(f"   Total requests: {success_count + rate_limited_count}")
        print(f"   Successful: {success_count}")
        print(f"   Rate limited: {rate_limited_count}")
        print(f"\n💡 Rate limits (ping): 120 capacity, 2/sec refill")

        await client.close()

    except Exception as e:
        print(f"❌ Error: {e}")


async def demo_binary_messages():
    """Демонстрация binary messages."""
    print("\n" + "="*60)
    print("DEMO 4: Binary Messages & Compression")
    print("="*60)

    from src.api.websocket.binary import binary_handler, BinaryMessageType
    from src.api.websocket.compression import default_compressor

    # Binary message example
    print("\n📦 Creating binary image message...")

    # Создаем fake image data
    fake_image = b"FAKE_JPEG_DATA" * 100

    binary_msg = binary_handler.create_image_message(
        fake_image,
        format="jpeg",
        width=1920,
        height=1080,
        camera="front",
        timestamp=datetime.utcnow().isoformat()
    )

    print(f"✅ Binary message created:")
    print(f"   Format: Image (JPEG)")
    print(f"   Size: {len(binary_msg)} bytes")
    print(f"   Payload: {len(fake_image)} bytes")
    print(f"   Metadata size: {len(binary_msg) - len(fake_image) - 8} bytes")

    # Compression example
    print("\n🗜️  Testing message compression...")

    large_json = {
        "type": "large_data",
        "items": [{"id": i, "data": "x" * 100} for i in range(100)]
    }

    original_json = json.dumps(large_json)
    original_size = len(original_json.encode('utf-8'))

    compressed, was_compressed = default_compressor.compress_json(large_json)

    if was_compressed:
        compressed_size = len(compressed)
        ratio = compressed_size / original_size
        savings = (1 - ratio) * 100

        print(f"✅ Compression successful:")
        print(f"   Original size: {original_size:,} bytes")
        print(f"   Compressed size: {compressed_size:,} bytes")
        print(f"   Ratio: {ratio:.2%}")
        print(f"   Savings: {savings:.1f}%")

        # Decompress to verify
        decompressed = default_compressor.decompress_json(compressed)
        if decompressed == large_json:
            print(f"   ✅ Decompression verified")
    else:
        print(f"⚠️  Message too small for compression (threshold: 1KB)")


async def demo_metrics():
    """Демонстрация Prometheus metrics."""
    print("\n" + "="*60)
    print("DEMO 5: Prometheus Metrics")
    print("="*60)

    from src.api.websocket.metrics import metrics

    print("\n📊 Tracking WebSocket metrics...")

    # Simulate some activity
    print("\n🔄 Simulating connections...")
    for i in range(5):
        metrics.track_connection_opened(user_id=f"user_{i}")

    print("   ✅ Tracked 5 connection opens")

    print("\n📨 Simulating message activity...")
    for i in range(100):
        metrics.track_message_sent("metrics", "data", 512)
        metrics.track_message_received("ping", 64)

    print("   ✅ Tracked 100 sent messages")
    print("   ✅ Tracked 100 received messages")

    print("\n📡 Simulating subscriptions...")
    for channel in ["metrics", "signals", "actions"]:
        metrics.track_subscription(channel)
        metrics.update_channel_subscribers(channel, 10)

    print("   ✅ Tracked subscriptions to 3 channels")

    print("\n💡 Metrics available at: http://localhost:8000/metrics")
    print("\nKey metrics:")
    print("   - neurograph_ws_connections_total")
    print("   - neurograph_ws_messages_sent_total")
    print("   - neurograph_ws_messages_received_total")
    print("   - neurograph_ws_message_latency_seconds")
    print("   - neurograph_ws_subscriptions_total")
    print("   - neurograph_ws_channel_subscribers")
    print("   - neurograph_ws_errors_total")
    print("   ... and 8 more metrics")


async def main():
    """Run all demos."""
    print("\n" + "="*60)
    print("NeuroGraph WebSocket Advanced Features Demo (v0.60.1)")
    print("="*60)

    try:
        # Demo 1: Reconnection
        await demo_reconnection()
        await asyncio.sleep(1)

        # Demo 2: Permissions
        await demo_permissions()
        await asyncio.sleep(1)

        # Demo 3: Rate Limiting
        await demo_rate_limiting()
        await asyncio.sleep(1)

        # Demo 4: Binary & Compression
        await demo_binary_messages()
        await asyncio.sleep(1)

        # Demo 5: Metrics
        await demo_metrics()

        print("\n" + "="*60)
        print("✅ All demos completed!")
        print("="*60)

    except KeyboardInterrupt:
        print("\n\n⚠️  Demo interrupted by user")
    except Exception as e:
        print(f"\n\n❌ Demo failed: {e}")
        import traceback
        traceback.print_exc()


if __name__ == "__main__":
    asyncio.run(main())
