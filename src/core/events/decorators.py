"""
NeuroGraph OS - Event System Decorators
Удобные декораторы для подписки на события

Путь: src/core/events/decorators.py
"""

from functools import wraps
from typing import Callable, Optional, List, Union
import logging

from .event import Event, EventType, EventCategory, EventPriority, EventFilter
from .event_bus import EventBus


logger = logging.getLogger(__name__)


class EventHandler:
    """
    Декоратор для регистрации обработчиков событий
    
    Использование:
        @EventHandler.on(EventType.TOKEN_CREATED)
        async def handle_token_created(event: Event):
            print(f"Token created: {event.payload}")
    """
    
    # Глобальная шина событий (устанавливается через set_event_bus)
    _event_bus: Optional[EventBus] = None
    
    # Коллекция зарегистрированных обработчиков
    _handlers: List[tuple] = []
    
    @classmethod
    def set_event_bus(cls, event_bus: EventBus):
        """Установить глобальную шину событий"""
        cls._event_bus = event_bus
        logger.info("EventBus set for EventHandler decorator")
    
    @classmethod
    def on(
        cls,
        event_types: Optional[Union[EventType, List[EventType]]] = None,
        categories: Optional[Union[EventCategory, List[EventCategory]]] = None,
        min_priority: Optional[EventPriority] = None,
        sources: Optional[List[str]] = None,
        subscriber_id: Optional[str] = None,
        subscription_name: Optional[str] = None
    ):
        """
        Декоратор для подписки на события
        
        Args:
            event_types: Тип(ы) событий для подписки
            categories: Категория(и) событий
            min_priority: Минимальный приоритет
            sources: Список источников событий
            subscriber_id: ID подписчика (по умолчанию - имя функции)
            subscription_name: Имя подписки
        
        Example:
            @EventHandler.on(EventType.TOKEN_CREATED, min_priority=EventPriority.HIGH)
            async def handle_important_tokens(event: Event):
                print(f"Important token: {event.payload}")
        """
        def decorator(func: Callable):
            # Нормализуем типы
            types_list = None
            if event_types:
                types_list = event_types if isinstance(event_types, list) else [event_types]
            
            categories_list = None
            if categories:
                categories_list = categories if isinstance(categories, list) else [categories]
            
            # Создаем фильтр
            event_filter = EventFilter(
                types=types_list,
                categories=categories_list,
                min_priority=min_priority,
                sources=sources
            )
            
            # Определяем subscriber_id
            sub_id = subscriber_id or func.__name__
            sub_name = subscription_name or func.__name__
            
            @wraps(func)
            async def wrapper(event: Event):
                return await func(event)
            
            # Сохраняем информацию для последующей регистрации
            cls._handlers.append((wrapper, sub_id, event_filter, sub_name))
            
            # Если шина уже установлена, регистрируем сразу
            if cls._event_bus:
                cls._event_bus.subscribe(
                    handler=wrapper,
                    subscriber_id=sub_id,
                    event_filter=event_filter,
                    subscription_name=sub_name
                )
                logger.info(f"Handler {func.__name__} registered for {event_filter}")
            
            return wrapper
        
        return decorator
    
    @classmethod
    def register_all(cls):
        """Зарегистрировать все отложенные обработчики"""
        if not cls._event_bus:
            logger.warning("EventBus not set, cannot register handlers")
            return
        
        for handler, sub_id, event_filter, sub_name in cls._handlers:
            cls._event_bus.subscribe(
                handler=handler,
                subscriber_id=sub_id,
                event_filter=event_filter,
                subscription_name=sub_name
            )
        
        logger.info(f"Registered {len(cls._handlers)} event handlers")
        cls._handlers.clear()


class EventEmitter:
    """
    Миксин для классов, которые генерируют события
    
    Использование:
        class TokenService(EventEmitter):
            def __init__(self, event_bus: EventBus):
                super().__init__(event_bus, source_id="token_service")
            
            async def create_token(self, data):
                token = Token(data)
                await self.emit(
                    EventType.TOKEN_CREATED,
                    payload={"token_id": token.id}
                )
    """
    
    def __init__(self, event_bus: EventBus, source_id: str):
        """
        Args:
            event_bus: Шина событий
            source_id: ID источника событий (обычно имя модуля)
        """
        self._event_bus = event_bus
        self._source_id = source_id
    
    async def emit(
        self,
        event_type: EventType,
        payload: dict,
        priority: EventPriority = EventPriority.NORMAL,
        target: Optional[List[str]] = None,
        correlation_id: Optional[str] = None,
        parent_event_id: Optional[str] = None,
        metadata: Optional[dict] = None
    ) -> Event:
        """
        Сгенерировать и опубликовать событие
        
        Args:
            event_type: Тип события
            payload: Данные события
            priority: Приоритет события
            target: Целевые получатели
            correlation_id: ID корреляции
            parent_event_id: ID родительского события
            metadata: Дополнительные метаданные
        
        Returns:
            Созданное событие
        """
        # Определяем категорию из типа
        category_str = event_type.value.split('.')[0]
        category = EventCategory(category_str)
        
        # Создаем событие
        event = Event(
            type=event_type,
            category=category,
            source=self._source_id,
            payload=payload,
            priority=priority,
            target=target,
            correlation_id=correlation_id,
            parent_event_id=parent_event_id,
            metadata=metadata or {}
        )
        
        # Публикуем
        await self._event_bus.publish(event)
        
        return event
    
    async def emit_error(
        self,
        error_type: str,
        error_message: str,
        error_details: Optional[dict] = None
    ) -> Event:
        """
        Сгенерировать событие об ошибке
        
        Args:
            error_type: Тип ошибки
            error_message: Сообщение об ошибке
            error_details: Детали ошибки
        
        Returns:
            Созданное событие
        """
        return await self.emit(
            event_type=EventType.ERROR_OCCURRED,
            payload={
                "error_type": error_type,
                "error_message": error_message,
                "error_details": error_details or {}
            },
            priority=EventPriority.CRITICAL
        )


def event_publisher(event_type: EventType, priority: EventPriority = EventPriority.NORMAL):
    """
    Декоратор для автоматической публикации события после выполнения метода
    
    Метод должен возвращать словарь, который будет использован как payload
    Класс должен наследовать EventEmitter
    
    Example:
        class MyService(EventEmitter):
            @event_publisher(EventType.TOKEN_CREATED)
            async def create_token(self, data):
                token = Token(data)
                return {"token_id": token.id, "coordinates": token.coords}
    """
    def decorator(func: Callable):
        @wraps(func)
        async def wrapper(self, *args, **kwargs):
            # Выполняем оригинальную функцию
            result = await func(self, *args, **kwargs)
            
            # Если класс наследует EventEmitter, публикуем событие
            if isinstance(self, EventEmitter):
                if isinstance(result, dict):
                    await self.emit(
                        event_type=event_type,
                        payload=result,
                        priority=priority
                    )
                else:
                    logger.warning(
                        f"Method {func.__name__} did not return dict, "
                        f"cannot publish event {event_type}"
                    )
            
            return result
        
        return wrapper
    
    return decorator


def event_responder(response_event_type: EventType):
    """
    Декоратор для создания обработчика, который отвечает другим событием
    
    Обработчик должен вернуть словарь для payload ответного события
    
    Example:
        @EventHandler.on(EventType.TOKEN_CREATED)
        @event_responder(EventType.GRAPH_CONNECTION_ADDED)
        async def respond_to_token(event: Event) -> dict:
            # Обработка
            return {"connection_id": "conn_123", "source": event.payload["token_id"]}
    """
    def decorator(func: Callable):
        @wraps(func)
        async def wrapper(event: Event):
            # Выполняем обработчик
            result = await func(event)
            
            # Если возвращен словарь, создаем ответное событие
            if isinstance(result, dict) and EventHandler._event_bus:
                # Определяем категорию из типа
                category_str = response_event_type.value.split('.')[0]
                category = EventCategory(category_str)
                
                response_event = Event(
                    type=response_event_type,
                    category=category,
                    source=func.__name__,
                    payload=result,
                    parent_event_id=event.id,
                    correlation_id=event.correlation_id or event.id
                )
                
                await EventHandler._event_bus.publish(response_event)
            
            return result
        
        return wrapper
    
    return decorator