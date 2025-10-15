"""
NeuroGraph OS - Global Event Bus
Singleton для управления глобальной шиной событий

Путь: src/core/events/global_bus.py
"""

import asyncio
import logging
from typing import Optional, Dict, Any
from pathlib import Path

from .event_bus import EventBus
from .decorators import EventHandler

logger = logging.getLogger(__name__)


class GlobalEventBus:
    """
    Singleton для управления глобальной шиной событий
    
    Обеспечивает единую точку доступа к Event Bus во всём приложении
    и упрощает инициализацию.
    """
    
    _instance: Optional[EventBus] = None
    _initialized: bool = False
    _config: Dict[str, Any] = {}
    
    @classmethod
    def initialize(
        cls,
        config: Optional[Dict[str, Any]] = None,
        max_queue_size: Optional[int] = None,
        enable_metrics: Optional[bool] = None,
        log_events: Optional[bool] = None
    ) -> EventBus:
        """
        Инициализировать глобальную шину событий
        
        Args:
            config: Словарь конфигурации (опционально)
            max_queue_size: Максимальный размер очереди
            enable_metrics: Включить метрики
            log_events: Логировать все события
        
        Returns:
            Инициализированная шина событий
        
        Example:
            bus = GlobalEventBus.initialize(max_queue_size=5000)
        """
        if cls._instance is not None:
            logger.warning("EventBus already initialized, returning existing instance")
            return cls._instance
        
        # Загружаем конфигурацию
        if config:
            cls._config = config
        else:
            cls._config = cls._load_default_config()
        
        # Параметры с приоритетом: явные аргументы > config > defaults
        params = {
            "max_queue_size": max_queue_size or cls._config.get("max_queue_size", 10000),
            "enable_metrics": enable_metrics if enable_metrics is not None else cls._config.get("enable_metrics", True),
            "log_events": log_events if log_events is not None else cls._config.get("log_events", False)
        }
        
        # Создаём шину
        cls._instance = EventBus(**params)
        cls._initialized = True
        
        logger.info(f"GlobalEventBus initialized with params: {params}")
        
        return cls._instance
    
    @classmethod
    def get(cls) -> EventBus:
        """
        Получить глобальную шину событий
        
        Returns:
            Шина событий
        
        Raises:
            RuntimeError: Если шина не инициализирована
        
        Example:
            bus = GlobalEventBus.get()
            await bus.publish(event)
        """
        if cls._instance is None:
            raise RuntimeError(
                "GlobalEventBus not initialized. "
                "Call GlobalEventBus.initialize() or GlobalEventBus.start() first."
            )
        return cls._instance
    
    @classmethod
    async def start(
        cls,
        config: Optional[Dict[str, Any]] = None,
        **kwargs
    ) -> EventBus:
        """
        Инициализировать и запустить глобальную шину событий
        
        Это удобный метод, объединяющий initialize() и start()
        
        Args:
            config: Конфигурация (опционально)
            **kwargs: Параметры для initialize()
        
        Returns:
            Запущенная шина событий
        
        Example:
            bus = await GlobalEventBus.start()
        """
        # Инициализируем если ещё не инициализирована
        if cls._instance is None:
            cls.initialize(config=config, **kwargs)
        
        # Запускаем
        await cls._instance.start()
        
        # Устанавливаем для декораторов
        EventHandler.set_event_bus(cls._instance)
        
        logger.info("GlobalEventBus started and ready")
        
        return cls._instance
    
    @classmethod
    async def stop(cls) -> None:
        """
        Остановить глобальную шину событий (graceful shutdown)
        
        Example:
            await GlobalEventBus.stop()
        """
        if cls._instance is None:
            logger.warning("GlobalEventBus not initialized, nothing to stop")
            return
        
        logger.info("Stopping GlobalEventBus...")
        await cls._instance.stop()
        logger.info("GlobalEventBus stopped")
    
    @classmethod
    def reset(cls) -> None:
        """
        Сбросить глобальную шину (для тестов)
        
        Warning:
            Используйте только в тестах!
        """
        cls._instance = None
        cls._initialized = False
        cls._config = {}
        logger.debug("GlobalEventBus reset")
    
    @classmethod
    def is_initialized(cls) -> bool:
        """Проверить, инициализирована ли шина"""
        return cls._initialized and cls._instance is not None
    
    @classmethod
    def is_running(cls) -> bool:
        """Проверить, запущена ли шина"""
        if cls._instance is None:
            return False
        return cls._instance._running
    
    @classmethod
    def get_config(cls) -> Dict[str, Any]:
        """Получить текущую конфигурацию"""
        return cls._config.copy()
    
    @classmethod
    def _load_default_config(cls) -> Dict[str, Any]:
        """
        Загрузить конфигурацию по умолчанию
        
        Пытается загрузить из config/core/event_bus.yaml,
        если не найден - использует встроенные defaults
        """
        default_config = {
            "max_queue_size": 10000,
            "enable_metrics": True,
            "log_events": False
        }
        
        try:
            # Пытаемся загрузить из файла
            from infrastructure.config import ConfigLoader
            
            config_path = Path("config/core/event_bus.yaml")
            if config_path.exists():
                loaded_config = ConfigLoader.load(str(config_path))
                
                # Извлекаем параметры event_bus
                if "event_bus" in loaded_config:
                    event_bus_config = loaded_config["event_bus"]
                    default_config.update({
                        "max_queue_size": event_bus_config.get("max_queue_size", 10000),
                        "enable_metrics": event_bus_config.get("enable_metrics", True),
                        "log_events": event_bus_config.get("log_events", False)
                    })
                    logger.info(f"Loaded config from {config_path}")
                else:
                    logger.warning(f"No 'event_bus' section in {config_path}, using defaults")
        
        except ImportError:
            logger.warning("ConfigLoader not available, using built-in defaults")
        except Exception as e:
            logger.warning(f"Failed to load config: {e}, using built-in defaults")
        
        return default_config


# =============================================================================
# Convenience functions (shortcuts)
# =============================================================================

def get_event_bus() -> EventBus:
    """
    Удобная функция для получения глобальной шины
    
    Returns:
        EventBus instance
    
    Example:
        from core.events import get_event_bus
        
        bus = get_event_bus()
        await bus.publish(event)
    """
    return GlobalEventBus.get()


async def start_event_bus(
    config: Optional[Dict[str, Any]] = None,
    **kwargs
) -> EventBus:
    """
    Удобная функция для запуска глобальной шины
    
    Args:
        config: Конфигурация
        **kwargs: Параметры инициализации
    
    Returns:
        Запущенная EventBus
    
    Example:
        from core.events import start_event_bus
        
        bus = await start_event_bus()
    """
    return await GlobalEventBus.start(config=config, **kwargs)


async def stop_event_bus() -> None:
    """
    Удобная функция для остановки глобальной шины
    
    Example:
        from core.events import stop_event_bus
        
        await stop_event_bus()
    """
    await GlobalEventBus.stop()


def initialize_event_bus(
    config: Optional[Dict[str, Any]] = None,
    **kwargs
) -> EventBus:
    """
    Удобная функция для инициализации (без запуска) глобальной шины
    
    Args:
        config: Конфигурация
        **kwargs: Параметры инициализации
    
    Returns:
        Инициализированная (но не запущенная) EventBus
    
    Example:
        from core.events import initialize_event_bus
        
        bus = initialize_event_bus(max_queue_size=5000)
        # ... настройка обработчиков ...
        await bus.start()
    """
    return GlobalEventBus.initialize(config=config, **kwargs)


# =============================================================================
# Context manager для удобного управления жизненным циклом
# =============================================================================

class EventBusContext:
    """
    Контекстный менеджер для управления жизненным циклом Event Bus
    
    Example:
        async with EventBusContext() as bus:
            # Шина автоматически запущена
            await bus.publish(event)
        # Шина автоматически остановлена
    """
    
    def __init__(
        self,
        config: Optional[Dict[str, Any]] = None,
        **kwargs
    ):
        """
        Args:
            config: Конфигурация
            **kwargs: Параметры инициализации
        """
        self.config = config
        self.kwargs = kwargs
        self.bus: Optional[EventBus] = None
    
    async def __aenter__(self) -> EventBus:
        """Вход в контекст - запуск шины"""
        self.bus = await GlobalEventBus.start(config=self.config, **self.kwargs)
        return self.bus
    
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        """Выход из контекста - остановка шины"""
        await GlobalEventBus.stop()
        return False  # Не подавляем исключения


# =============================================================================
# Декоратор для автоматического управления шиной в async функциях
# =============================================================================

def with_event_bus(
    config: Optional[Dict[str, Any]] = None,
    **kwargs
):
    """
    Декоратор для автоматического запуска/остановки Event Bus
    
    Example:
        @with_event_bus(max_queue_size=5000)
        async def main():
            bus = get_event_bus()
            await bus.publish(event)
        
        # Шина автоматически запустится до main() и остановится после
        asyncio.run(main())
    """
    def decorator(func):
        async def wrapper(*args, **func_kwargs):
            async with EventBusContext(config=config, **kwargs):
                return await func(*args, **func_kwargs)
        return wrapper
    return decorator


# =============================================================================
# Примеры использования
# =============================================================================

if __name__ == "__main__":
    """Демонстрация различных способов использования GlobalEventBus"""
    
    from .event import Event, EventType, EventCategory
    
    # Способ 1: Явное управление
    async def example_explicit():
        print("\n=== Example 1: Explicit Management ===")
        
        # Инициализация и запуск
        bus = await GlobalEventBus.start(max_queue_size=1000)
        
        # Использование
        event = Event(
            type=EventType.SYSTEM_STARTED,
            category=EventCategory.SYSTEM,
            source="example",
            payload={"message": "Hello from example!"}
        )
        await bus.publish(event)
        
        await asyncio.sleep(0.1)
        
        # Метрики
        metrics = bus.get_metrics()
        print(f"Published: {metrics['total_published']}")
        
        # Остановка
        await GlobalEventBus.stop()
    
    # Способ 2: Использование convenience functions
    async def example_convenience():
        print("\n=== Example 2: Convenience Functions ===")
        
        # Запуск
        bus = await start_event_bus()
        
        # Получение через convenience function
        bus2 = get_event_bus()
        assert bus is bus2  # Тот же экземпляр
        
        event = Event(
            type=EventType.SYSTEM_STARTED,
            category=EventCategory.SYSTEM,
            source="example",
            payload={}
        )
        await bus.publish(event)
        
        await asyncio.sleep(0.1)
        
        # Остановка
        await stop_event_bus()
    
    # Способ 3: Context manager
    async def example_context_manager():
        print("\n=== Example 3: Context Manager ===")
        
        async with EventBusContext(max_queue_size=500) as bus:
            # Шина автоматически запущена
            print(f"Bus running: {GlobalEventBus.is_running()}")
            
            event = Event(
                type=EventType.SYSTEM_STARTED,
                category=EventCategory.SYSTEM,
                source="example",
                payload={}
            )
            await bus.publish(event)
            
            await asyncio.sleep(0.1)
        
        # Шина автоматически остановлена
        print(f"Bus running after context: {GlobalEventBus.is_running()}")
    
    # Способ 4: Декоратор
    @with_event_bus(max_queue_size=500)
    async def example_decorator():
        print("\n=== Example 4: Decorator ===")
        
        # Шина уже запущена декоратором
        bus = get_event_bus()
        print(f"Bus running: {GlobalEventBus.is_running()}")
        
        event = Event(
            type=EventType.SYSTEM_STARTED,
            category=EventCategory.SYSTEM,
            source="example",
            payload={}
        )
        await bus.publish(event)
        
        await asyncio.sleep(0.1)
        
        # Шина автоматически остановится после функции
    
    # Запуск примеров
    async def run_all_examples():
        print("=" * 70)
        print("GlobalEventBus Examples")
        print("=" * 70)
        
        await example_explicit()
        GlobalEventBus.reset()  # Сброс для следующего примера
        
        await example_convenience()
        GlobalEventBus.reset()
        
        await example_context_manager()
        GlobalEventBus.reset()
        
        await example_decorator()
        
        print("\n" + "=" * 70)
        print("All examples completed!")
        print("=" * 70)
    
    asyncio.run(run_all_examples())