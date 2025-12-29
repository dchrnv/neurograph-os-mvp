
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
WebSocket Core Integrations

Connects SignalSystem, ActionController, and Metrics to WebSocket broadcasting.
"""

import asyncio
from typing import Dict, Any, Optional
from datetime import datetime

from .manager import connection_manager
from .channels import Channel
from ..logging_config import get_logger

logger = get_logger(__name__, service="websocket_integration")


class WebSocketIntegration:
    """
    Integration layer between NeuroGraph Core and WebSocket broadcasting.

    Subscribes to:
    - SignalSystem events (via FFI callbacks)
    - Metrics updates
    - Action executions
    - System status changes
    """

    def __init__(self):
        self._running = False
        self._metrics_task: Optional[asyncio.Task] = None
        logger.info("WebSocket integration initialized")

    async def start(self):
        """Start the integration and begin broadcasting events."""
        if self._running:
            logger.warning("WebSocket integration already running")
            return

        self._running = True

        # Start periodic metrics broadcasting
        self._metrics_task = asyncio.create_task(self._metrics_broadcast_loop())

        logger.info("WebSocket integration started")

    async def stop(self):
        """Stop the integration and cleanup."""
        if not self._running:
            return

        self._running = False

        # Cancel metrics task
        if self._metrics_task:
            self._metrics_task.cancel()
            try:
                await self._metrics_task
            except asyncio.CancelledError:
                pass

        logger.info("WebSocket integration stopped")

    async def broadcast_signal_event(
        self,
        event_id: str,
        event_type: str,
        is_novel: bool,
        priority: int,
        resonance_score: Optional[float] = None,
        neighbors_count: int = 0,
        metadata: Optional[Dict[str, Any]] = None
    ):
        """
        Broadcast a signal event to the signals channel.

        Args:
            event_id: Event identifier
            event_type: Event type (e.g., "signal.input.text")
            is_novel: Whether the signal is novel
            priority: Signal priority (0-255)
            resonance_score: Optional resonance score
            neighbors_count: Number of neighbors found
            metadata: Additional metadata
        """
        event = {
            "channel": Channel.SIGNALS,
            "timestamp": datetime.utcnow().isoformat(),
            "event_type": event_type,
            "data": {
                "event_id": event_id,
                "event_type": event_type,
                "is_novel": is_novel,
                "priority": priority,
                "resonance_score": resonance_score,
                "neighbors_count": neighbors_count,
                **(metadata or {})
            }
        }

        await connection_manager.broadcast_to_channel(Channel.SIGNALS, event)

    async def broadcast_action_event(
        self,
        action_id: str,
        action_type: str,
        status: str,
        result: Optional[Dict[str, Any]] = None,
        execution_time_ms: Optional[float] = None,
        error: Optional[str] = None
    ):
        """
        Broadcast an action execution event to the actions channel.

        Args:
            action_id: Action identifier
            action_type: Action type (e.g., "TextResponseAction")
            status: Action status (e.g., "completed", "failed")
            result: Optional action result
            execution_time_ms: Execution time in milliseconds
            error: Optional error message
        """
        event = {
            "channel": Channel.ACTIONS,
            "timestamp": datetime.utcnow().isoformat(),
            "event_type": "action.executed",
            "data": {
                "action_id": action_id,
                "action_type": action_type,
                "status": status,
                "result": result,
                "execution_time_ms": execution_time_ms,
                "error": error
            }
        }

        await connection_manager.broadcast_to_channel(Channel.ACTIONS, event)

    async def broadcast_metrics(
        self,
        cpu_percent: Optional[float] = None,
        memory_mb: Optional[float] = None,
        events_per_sec: Optional[float] = None,
        total_tokens: Optional[int] = None,
        total_connections: Optional[int] = None,
        custom_metrics: Optional[Dict[str, Any]] = None
    ):
        """
        Broadcast system metrics to the metrics channel.

        Args:
            cpu_percent: CPU usage percentage
            memory_mb: Memory usage in MB
            events_per_sec: Events processed per second
            total_tokens: Total number of tokens
            total_connections: Total number of connections
            custom_metrics: Additional custom metrics
        """
        event = {
            "channel": Channel.METRICS,
            "timestamp": datetime.utcnow().isoformat(),
            "event_type": "system_metrics",
            "data": {
                "cpu_percent": cpu_percent,
                "memory_mb": memory_mb,
                "events_per_sec": events_per_sec,
                "total_tokens": total_tokens,
                "total_connections": total_connections,
                **(custom_metrics or {})
            }
        }

        await connection_manager.broadcast_to_channel(Channel.METRICS, event)

    async def broadcast_log(
        self,
        level: str,
        message: str,
        service: Optional[str] = None,
        correlation_id: Optional[str] = None,
        extra: Optional[Dict[str, Any]] = None
    ):
        """
        Broadcast a log message to the logs channel.

        Args:
            level: Log level (DEBUG, INFO, WARNING, ERROR, CRITICAL)
            message: Log message
            service: Service name
            correlation_id: Request correlation ID
            extra: Additional log data
        """
        event = {
            "channel": Channel.LOGS,
            "timestamp": datetime.utcnow().isoformat(),
            "event_type": f"log.{level.lower()}",
            "data": {
                "level": level,
                "message": message,
                "service": service,
                "correlation_id": correlation_id,
                **(extra or {})
            }
        }

        await connection_manager.broadcast_to_channel(Channel.LOGS, event)

    async def broadcast_status(
        self,
        status: str,
        uptime_seconds: Optional[int] = None,
        version: Optional[str] = None,
        details: Optional[Dict[str, Any]] = None
    ):
        """
        Broadcast a system status change to the status channel.

        Args:
            status: System status (e.g., "running", "stopped", "error")
            uptime_seconds: System uptime in seconds
            version: System version
            details: Additional status details
        """
        event = {
            "channel": Channel.STATUS,
            "timestamp": datetime.utcnow().isoformat(),
            "event_type": f"status.{status}",
            "data": {
                "status": status,
                "uptime_seconds": uptime_seconds,
                "version": version,
                **(details or {})
            }
        }

        await connection_manager.broadcast_to_channel(Channel.STATUS, event)

    async def broadcast_connection_event(
        self,
        event_type: str,
        client_id: str,
        user_id: Optional[str] = None,
        details: Optional[Dict[str, Any]] = None
    ):
        """
        Broadcast a connection event to the connections channel.

        Args:
            event_type: Event type (e.g., "client.connected", "client.disconnected")
            client_id: Client identifier
            user_id: Optional user identifier
            details: Additional event details
        """
        stats = connection_manager.get_connection_stats()

        event = {
            "channel": Channel.CONNECTIONS,
            "timestamp": datetime.utcnow().isoformat(),
            "event_type": event_type,
            "data": {
                "client_id": client_id,
                "user_id": user_id,
                "total_connections": stats["total_connections"],
                **(details or {})
            }
        }

        await connection_manager.broadcast_to_channel(Channel.CONNECTIONS, event)

    async def _metrics_broadcast_loop(self):
        """
        Periodic loop to broadcast system metrics.

        Broadcasts metrics every 5 seconds to the metrics channel.
        """
        interval = 5  # seconds

        try:
            while self._running:
                await asyncio.sleep(interval)

                try:
                    # Get connection stats
                    stats = connection_manager.get_connection_stats()

                    # TODO: Get actual system metrics (CPU, memory, etc.)
                    # For now, send connection stats
                    await self.broadcast_metrics(
                        total_connections=stats["total_connections"],
                        custom_metrics={
                            "total_channels": stats["total_channels"],
                            "total_subscriptions": stats["total_subscriptions"],
                            "buffered_events": stats["buffered_events"]
                        }
                    )

                except Exception as e:
                    logger.error(
                        f"Error broadcasting metrics: {e}",
                        extra={
                            "event": "metrics_broadcast_error",
                            "error": str(e)
                        }
                    )

        except asyncio.CancelledError:
            # Normal shutdown
            pass


# Global integration instance
ws_integration = WebSocketIntegration()
