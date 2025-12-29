
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
WebSocket Prometheus Metrics

Metrics for monitoring WebSocket connections, messages, and performance.
"""

from prometheus_client import Counter, Histogram, Gauge
import time
from functools import wraps
from typing import Callable

# Connection metrics
ws_connections_total = Gauge(
    'neurograph_ws_connections_total',
    'Total active WebSocket connections'
)

ws_connections_by_user = Gauge(
    'neurograph_ws_connections_by_user',
    'WebSocket connections by user',
    ['user_id']
)

ws_connection_duration = Histogram(
    'neurograph_ws_connection_duration_seconds',
    'WebSocket connection duration',
    buckets=[1, 5, 10, 30, 60, 300, 600, 1800, 3600]  # 1s to 1h
)

# Channel metrics
ws_channel_subscribers = Gauge(
    'neurograph_ws_channel_subscribers',
    'Number of subscribers per channel',
    ['channel']
)

ws_channel_subscriptions_total = Counter(
    'neurograph_ws_channel_subscriptions_total',
    'Total channel subscriptions',
    ['channel']
)

ws_channel_unsubscriptions_total = Counter(
    'neurograph_ws_channel_unsubscriptions_total',
    'Total channel unsubscriptions',
    ['channel']
)

# Message metrics
ws_messages_sent_total = Counter(
    'neurograph_ws_messages_sent_total',
    'Total messages sent',
    ['channel', 'message_type']
)

ws_messages_received_total = Counter(
    'neurograph_ws_messages_received_total',
    'Total messages received',
    ['message_type']
)

ws_message_size_bytes = Histogram(
    'neurograph_ws_message_size_bytes',
    'WebSocket message size in bytes',
    ['direction'],  # sent/received
    buckets=[100, 500, 1000, 5000, 10000, 50000, 100000, 500000, 1000000]
)

ws_message_latency_seconds = Histogram(
    'neurograph_ws_message_latency_seconds',
    'WebSocket message processing latency',
    ['message_type'],
    buckets=[0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0]  # 1ms to 5s
)

# Event buffering metrics
ws_buffered_events_total = Gauge(
    'neurograph_ws_buffered_events_total',
    'Total buffered events across all clients'
)

ws_buffer_overflow_total = Counter(
    'neurograph_ws_buffer_overflow_total',
    'Total buffer overflows (events dropped)',
    ['client_id']
)

# Error metrics
ws_errors_total = Counter(
    'neurograph_ws_errors_total',
    'Total WebSocket errors',
    ['error_type']
)

ws_disconnects_total = Counter(
    'neurograph_ws_disconnects_total',
    'Total WebSocket disconnections',
    ['reason']
)

# Performance metrics
ws_broadcast_duration_seconds = Histogram(
    'neurograph_ws_broadcast_duration_seconds',
    'Time to broadcast message to all subscribers',
    ['channel'],
    buckets=[0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0]
)

ws_heartbeat_latency_seconds = Histogram(
    'neurograph_ws_heartbeat_latency_seconds',
    'Heartbeat round-trip latency',
    buckets=[0.01, 0.05, 0.1, 0.5, 1.0, 5.0, 10.0]
)


def track_message_latency(message_type: str):
    """
    Decorator to track message processing latency.

    Args:
        message_type: Type of message being processed
    """
    def decorator(func: Callable):
        @wraps(func)
        async def wrapper(*args, **kwargs):
            start_time = time.perf_counter()
            try:
                result = await func(*args, **kwargs)
                return result
            finally:
                duration = time.perf_counter() - start_time
                ws_message_latency_seconds.labels(message_type=message_type).observe(duration)
        return wrapper
    return decorator


def track_broadcast_duration(channel: str):
    """
    Decorator to track broadcast duration.

    Args:
        channel: Channel name
    """
    def decorator(func: Callable):
        @wraps(func)
        async def wrapper(*args, **kwargs):
            start_time = time.perf_counter()
            try:
                result = await func(*args, **kwargs)
                return result
            finally:
                duration = time.perf_counter() - start_time
                ws_broadcast_duration_seconds.labels(channel=channel).observe(duration)
        return wrapper
    return decorator


class WebSocketMetricsCollector:
    """Helper class for collecting WebSocket metrics."""

    @staticmethod
    def track_connection_opened(user_id: str = None):
        """Track new connection."""
        ws_connections_total.inc()
        if user_id:
            ws_connections_by_user.labels(user_id=user_id).inc()

    @staticmethod
    def track_connection_closed(user_id: str = None, reason: str = "unknown"):
        """Track connection closure."""
        ws_connections_total.dec()
        if user_id:
            ws_connections_by_user.labels(user_id=user_id).dec()
        ws_disconnects_total.labels(reason=reason).inc()

    @staticmethod
    def track_connection_duration(duration: float):
        """Track connection duration."""
        ws_connection_duration.observe(duration)

    @staticmethod
    def track_subscription(channel: str):
        """Track channel subscription."""
        ws_channel_subscriptions_total.labels(channel=channel).inc()

    @staticmethod
    def track_unsubscription(channel: str):
        """Track channel unsubscription."""
        ws_channel_unsubscriptions_total.labels(channel=channel).inc()

    @staticmethod
    def update_channel_subscribers(channel: str, count: int):
        """Update channel subscriber count."""
        ws_channel_subscribers.labels(channel=channel).set(count)

    @staticmethod
    def track_message_sent(channel: str, message_type: str, size_bytes: int = 0):
        """Track sent message."""
        ws_messages_sent_total.labels(channel=channel, message_type=message_type).inc()
        if size_bytes > 0:
            ws_message_size_bytes.labels(direction="sent").observe(size_bytes)

    @staticmethod
    def track_message_received(message_type: str, size_bytes: int = 0):
        """Track received message."""
        ws_messages_received_total.labels(message_type=message_type).inc()
        if size_bytes > 0:
            ws_message_size_bytes.labels(direction="received").observe(size_bytes)

    @staticmethod
    def track_error(error_type: str):
        """Track error."""
        ws_errors_total.labels(error_type=error_type).inc()

    @staticmethod
    def update_buffered_events(count: int):
        """Update buffered events count."""
        ws_buffered_events_total.set(count)

    @staticmethod
    def track_buffer_overflow(client_id: str):
        """Track buffer overflow."""
        ws_buffer_overflow_total.labels(client_id=client_id).inc()


# Global metrics collector instance
metrics = WebSocketMetricsCollector()
