"""
NeuroGraph OS - Event System
Система событий для асинхронной коммуникации между модулями

Основные компоненты:
    - Event: Базовый класс для событий
    - EventBus: Шина событий для публикации и подписки
    - GlobalEventBus: Глобальная шина событий (синглтон)
    - Декораторы: Упрощают работу с событиями

Пример использования:
    ```python
    from core.events import start_event_bus, get_event_bus, Event, EventType, EventCategory

    async def main():
        # Инициализация шины
        await start_event_bus()
        bus = get_event_bus()
        
        # Подписка на события
        @bus.subscribe(EventType.TOKEN_CREATED)
        async def on_token_created(event):
            print(f"Token created: {event.payload}")
        
        # Публикация события
        await bus.publish(Event(
            type=EventType.TOKEN_CREATED,
            category=EventCategory.TOKEN,
            source="example",
            payload={"token_id": "123"}
        ))
        
        # Остановка шины
        await stop_event_bus()
    ```
"""

from .event import (
    Event,
    EventType,
    EventCategory,
    EventPriority,
    EventFilter
)

from .event_bus import (
    EventBus,
    Subscription,
    EventBusMetrics
)

from .decorators import (
    EventHandler,
    EventEmitter,
    event_publisher,
    event_responder
)

from .global_bus import (
    GlobalEventBus,
    get_event_bus,
    start_event_bus,
    stop_event_bus,
    initialize_event_bus,
    EventBusContext,
    with_event_bus
)


__version__ = "1.0.0"

__all__ = [
    # Event models
    "Event",
    "EventType",
    "EventCategory",
    "EventPriority",
    "EventFilter",
    
    # Event bus
    "EventBus",
    "Subscription",
    "EventBusMetrics",
    
    # Decorators and mixins
    "EventHandler",
    "EventEmitter",
    "event_publisher",
    "event_responder",
    
    # Global bus management
    "GlobalEventBus",
    "get_event_bus",
    "start_event_bus",
    "stop_event_bus",
    "initialize_event_bus",
    "EventBusContext",
    "with_event_bus",
]


# Module metadata
__author__ = "NeuroGraph OS Team"
__description__ = "Event-driven communication system for NeuroGraph OS"
__status__ = "Production"