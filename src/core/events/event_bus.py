"""
NeuroGraph OS - Async Event Bus
Асинхронная шина событий с подписками, фильтрацией, метриками и логированием.

Путь: src/core/events/event_bus.py
"""

from __future__ import annotations

import asyncio
import logging
from dataclasses import dataclass
from typing import Awaitable, Callable, Dict, List, Optional

from .event import Event, EventFilter


logger = logging.getLogger(__name__)


AsyncEventHandler = Callable[[Event], Awaitable[None]]


@dataclass
class Subscription:
    handler: AsyncEventHandler
    subscriber_id: str
    event_filter: Optional[EventFilter] = None
    subscription_name: Optional[str] = None


@dataclass
class EventBusMetrics:
    total_published: int = 0
    total_delivered: int = 0
    total_dropped: int = 0

    def to_dict(self) -> Dict[str, int]:
        delivered = self.total_delivered
        published = max(self.total_published, 1)
        return {
            "total_published": self.total_published,
            "total_delivered": self.total_delivered,
            "total_dropped": self.total_dropped,
            "delivery_rate": delivered / published,
        }


class EventBus:
    def __init__(
        self,
        *,
        max_queue_size: int = 10000,
        enable_metrics: bool = True,
        log_events: bool = False,
    ) -> None:
        self._queue: asyncio.Queue[Event] = asyncio.Queue(maxsize=max_queue_size)
        self._subscriptions: Dict[str, Subscription] = {}
        self._running: bool = False
        self._worker_task: Optional[asyncio.Task] = None
        self._enable_metrics: bool = enable_metrics
        self._metrics: EventBusMetrics = EventBusMetrics()
        self._log_events: bool = log_events

    # Lifecycle
    async def start(self) -> None:
        if self._running:
            return
        self._running = True
        self._worker_task = asyncio.create_task(self._worker())
        logger.info("EventBus started")

    async def stop(self) -> None:
        if not self._running:
            return
        self._running = False
        # Put sentinel to unblock queue get()
        try:
            self._queue.put_nowait(Event(
                type=None,  # type: ignore
                category=None,  # type: ignore
                source="event_bus",
                payload={"_sentinel": True},
            ))
        except Exception:
            pass
        if self._worker_task:
            try:
                await self._worker_task
            except Exception:
                pass
        self._worker_task = None
        logger.info("EventBus stopped")

    # Pub/Sub API
    def subscribe(
        self,
        *,
        handler: AsyncEventHandler,
        subscriber_id: str,
        event_filter: Optional[EventFilter] = None,
        subscription_name: Optional[str] = None,
    ) -> None:
        self._subscriptions[subscriber_id] = Subscription(
            handler=handler,
            subscriber_id=subscriber_id,
            event_filter=event_filter,
            subscription_name=subscription_name or subscriber_id,
        )
        logger.debug(f"Subscribed: id={subscriber_id} name={subscription_name}")

    def unsubscribe(self, subscriber_id: str) -> None:
        if subscriber_id in self._subscriptions:
            del self._subscriptions[subscriber_id]
            logger.debug(f"Unsubscribed: id={subscriber_id}")

    async def publish(self, event: Event) -> None:
        if not self._running:
            # Allow publishing before start by dropping silently (or buffer?)
            if self._enable_metrics:
                self._metrics.total_dropped += 1
            return
        if self._enable_metrics:
            self._metrics.total_published += 1
        if self._log_events:
            logger.info(f"EVENT PUBLISHED {event}")
        try:
            await self._queue.put(event)
        except asyncio.CancelledError:
            raise
        except Exception:
            if self._enable_metrics:
                self._metrics.total_dropped += 1

    # Introspection
    def get_metrics(self) -> Dict[str, float]:
        return self._metrics.to_dict() if self._enable_metrics else {
            "total_published": 0,
            "total_delivered": 0,
            "total_dropped": 0,
            "delivery_rate": 0.0,
        }

    def get_subscriptions_info(self) -> Dict[str, int]:
        return {
            "total_subscriptions": len(self._subscriptions),
            "queue_size": self._queue.qsize(),
        }

    # Worker
    async def _worker(self) -> None:
        while self._running:
            try:
                event = await self._queue.get()
            except asyncio.CancelledError:
                break
            except Exception:
                continue

            # Sentinel detection
            if getattr(event, "payload", {}).get("_sentinel"):
                continue

            await self._dispatch(event)

    async def _dispatch(self, event: Event) -> None:
        deliveries: List[asyncio.Task] = []

        for sub_id, sub in list(self._subscriptions.items()):
            # Targeted delivery: if event.target is set, only deliver to specified subscribers
            if event.is_targeted() and sub_id not in (event.target or []):
                continue

            if sub.event_filter and not sub.event_filter.matches(event):
                continue

            try:
                deliveries.append(asyncio.create_task(sub.handler(event)))
            except Exception:
                # best effort
                continue

        if deliveries:
            try:
                await asyncio.gather(*deliveries, return_exceptions=True)
            finally:
                if self._enable_metrics:
                    self._metrics.total_delivered += len(deliveries)
        else:
            if self._enable_metrics:
                self._metrics.total_dropped += 1


