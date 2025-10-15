"""
NeuroGraph OS - Event System Usage Examples
Практические примеры использования GlobalEventBus

Путь: examples/event_usage_example.py
"""

import asyncio
from typing import Dict, Any

# Импорт Event System
from core.events import (
    # Модели
    Event, EventType, EventCategory, EventPriority,
    
    # Декораторы
    EventHandler, EventEmitter,
    
    # Глобальная шина (разные способы)
    GlobalEventBus,
    get_event_bus,
    start_event_bus,
    stop_event_bus,
    EventBusContext,
    with_event_bus
)


print("""
╔════════════════════════════════════════════════════════════════════╗
║           NeuroGraph OS - Event System Usage Examples             ║
║                     4 Different Approaches                          ║
╚════════════════════════════════════════════════════════════════════╝
""")


# =============================================================================
# Подход 1: Явное управление (Explicit Management)
# =============================================================================

async def approach_1_explicit():
    """
    Явное управление жизненным циклом Event Bus
    Максимальный контроль, но требует ручного управления
    """
    print("\n" + "="*70)
    print("Approach 1: Explicit Management")
    print("="*70 + "\n")
    
    # 1. Инициализация и запуск
    print("🚀 Starting Event Bus...")
    bus = await GlobalEventBus.start(
        max_queue_size=1000,
        enable_metrics=True,
        log_events=False
    )
    print("✅ Event Bus started\n")
    
    # 2. Подписка на события
    received_events = []
    
    async def my_handler(event: Event):
        received_events.append(event)
        print(f"   📨 Received: {event.type.value}")
        print(f"      Source: {event.source}")
        print(f"      Payload: {event.payload}\n")
    
    bus.subscribe(
        handler=my_handler,
        subscriber_id="my_module"
    )
    print("✅ Subscribed to events\n")
    
    # 3. Публикация событий
    print("📤 Publishing events...\n")
    
    event1 = Event(
        type=EventType.SYSTEM_STARTED,
        category=EventCategory.SYSTEM,
        source="system",
        payload={"version": "1.0.0", "approach": "explicit"}
    )
    await bus.publish(event1)
    
    event2 = Event(
        type=EventType.TOKEN_CREATED,
        category=EventCategory.TOKEN,
        source="token_service",
        payload={"token_id": "tok_001", "approach": "explicit"}
    )
    await bus.publish(event2)
    
    # Ждём обработки
    await asyncio.sleep(0.2)
    
    # 4. Метрики
    print("📊 Metrics:")
    metrics = bus.get_metrics()
    print(f"   Total published: {metrics['total_published']}")
    print(f"   Total delivered: {metrics['total_delivered']}")
    print(f"   Received by handler: {len(received_events)}\n")
    
    # 5. Остановка
    print("🛑 Stopping Event Bus...")
    await GlobalEventBus.stop()
    print("✅ Event Bus stopped\n")


# =============================================================================
# Подход 2: Convenience Functions (Удобные функции)
# =============================================================================

async def approach_2_convenience():
    """
    Использование convenience functions
    Простой и понятный код, рекомендуется для большинства случаев
    """
    print("\n" + "="*70)
    print("Approach 2: Convenience Functions")
    print("="*70 + "\n")
    
    # 1. Запуск через convenience function
    print("🚀 Starting Event Bus (convenience)...")
    await start_event_bus()
    print("✅ Event Bus started\n")
    
    # 2. Получение шины через convenience function
    bus = get_event_bus()
    
    # 3. Подписка с использованием декоратора
    EventHandler.set_event_bus(bus)
    
    @EventHandler.on(EventType.TOKEN_CREATED)
    async def handle_token_creation(event: Event):
        print(f"   📨 Token created: {event.payload['token_id']}")
    
    EventHandler.register_all()
    print("✅ Handler registered\n")
    
    # 4. Публикация
    print("📤 Publishing event...\n")
    event = Event(
        type=EventType.TOKEN_CREATED,
        category=EventCategory.TOKEN,
        source="token_service",
        payload={"token_id": "tok_002", "approach": "convenience"}
    )
    await bus.publish(event)
    
    await asyncio.sleep(0.1)
    
    # 5. Остановка через convenience function
    print("\n🛑 Stopping Event Bus...")
    await stop_event_bus()
    print("✅ Event Bus stopped\n")


# =============================================================================
# Подход 3: Context Manager (Контекстный менеджер)
# =============================================================================

async def approach_3_context_manager():
    """
    Использование context manager
    Автоматическое управление жизненным циклом, гарантированная очистка
    """
    print("\n" + "="*70)
    print("Approach 3: Context Manager")
    print("="*70 + "\n")
    
    print("🚀 Starting Event Bus (context manager)...\n")
    
    async with EventBusContext(max_queue_size=500) as bus:
        # Шина автоматически запущена здесь
        print("✅ Event Bus started (automatic)\n")
        
        # Создаём сервис с EventEmitter
        class MyService(EventEmitter):
            def __init__(self, event_bus):
                super().__init__(event_bus, source_id="my_service")
            
            async def do_work(self, data: Dict[str, Any]):
                print(f"   🔧 Processing: {data}")
                
                # Автоматическая публикация через emit
                await self.emit(
                    EventType.TOKEN_CREATED,
                    payload={"token_id": data["id"], "approach": "context_manager"}
                )
        
        service = MyService(bus)
        
        # Подписка
        @EventHandler.on(EventType.TOKEN_CREATED)
        async def handle_event(event: Event):
            print(f"   📨 Event received: {event.payload}\n")
        
        EventHandler.set_event_bus(bus)
        EventHandler.register_all()
        
        # Работа
        print("📤 Service doing work...\n")
        await service.do_work({"id": "tok_003"})
        
        await asyncio.sleep(0.1)
    
    # Шина автоматически остановлена здесь
    print("✅ Event Bus stopped (automatic)\n")


# =============================================================================
# Подход 4: Decorator (Декоратор)
# =============================================================================

@with_event_bus(max_queue_size=500, enable_metrics=True)
async def approach_4_decorator():
    """
    Использование декоратора @with_event_bus
    Самый короткий код, автоматическое управление
    """
    print("\n" + "="*70)
    print("Approach 4: Decorator")
    print("="*70 + "\n")
    
    # Шина уже запущена декоратором!
    print("✅ Event Bus started (by decorator)\n")
    
    bus = get_event_bus()
    
    # Сервис с автоматической публикацией
    class TokenService(EventEmitter):
        def __init__(self):
            super().__init__(get_event_bus(), source_id="token_service")
        
        async def create_token(self, token_id: str):
            print(f"   🎯 Creating token: {token_id}")
            await self.emit(
                EventType.TOKEN_CREATED,
                payload={"token_id": token_id, "approach": "decorator"}
            )
    
    # Обработчики
    @EventHandler.on(EventType.TOKEN_CREATED, min_priority=EventPriority.NORMAL)
    async def log_token(event: Event):
        print(f"   📝 Logged: {event.payload['token_id']}")
    
    @EventHandler.on(EventType.TOKEN_CREATED)
    async def analyze_token(event: Event):
        print(f"   🔍 Analyzed: {event.payload['token_id']}\n")
    
    EventHandler.set_event_bus(bus)
    EventHandler.register_all()
    
    # Работа
    service = TokenService()
    
    print("📤 Creating tokens...\n")
    await service.create_token("tok_004")
    await service.create_token("tok_005")
    
    await asyncio.sleep(0.2)
    
    # Метрики
    metrics = bus.get_metrics()
    print("📊 Final metrics:")
    print(f"   Published: {metrics['total_published']}")
    print(f"   Delivered: {metrics['total_delivered']}")
    print(f"   Events by type: {metrics['events_by_type']}\n")
    
    # Шина автоматически остановится после выхода из функции
    print("✅ Event Bus will stop automatically\n")


# =============================================================================
# Бонус: Полноценный пример с интеграцией компонентов
# =============================================================================

@with_event_bus()
async def bonus_full_integration():
    """
    Полноценный пример с несколькими компонентами
    """
    print("\n" + "="*70)
    print("BONUS: Full Integration Example")
    print("="*70 + "\n")
    
    bus = get_event_bus()
    EventHandler.set_event_bus(bus)
    
    # Компонент 1: TokenService
    class TokenService(EventEmitter):
        def __init__(self):
            super().__init__(bus, source_id="token_service")
            self.tokens = {}
        
        async def create_token(self, token_id: str, data: Dict[str, Any]):
            self.tokens[token_id] = data
            await self.emit(
                EventType.TOKEN_CREATED,
                payload={"token_id": token_id, **data},
                priority=EventPriority.HIGH if data.get("important") else EventPriority.NORMAL
            )
            return token_id
    
    # Компонент 2: GraphManager (реагирует на токены)
    class GraphManager(EventEmitter):
        def __init__(self):
            super().__init__(bus, source_id="graph_manager")
            self.connections = []
        
        @EventHandler.on(EventType.TOKEN_CREATED)
        async def auto_connect_token(self, event: Event):
            token_id = event.payload["token_id"]
            
            if len(self.connections) > 0:
                last_token = self.connections[-1]["target"]
                connection = {"source": last_token, "target": token_id}
                self.connections.append(connection)
                
                print(f"   🔗 Auto-connected: {last_token} -> {token_id}")
                
                await self.emit(
                    EventType.GRAPH_CONNECTION_ADDED,
                    payload=connection
                )
            else:
                self.connections.append({"source": None, "target": token_id})
    
    # Компонент 3: Monitor (следит за всем)
    class SystemMonitor:
        def __init__(self):
            self.stats = {"tokens": 0, "connections": 0}
        
        @EventHandler.on(EventType.TOKEN_CREATED)
        async def count_tokens(self, event: Event):
            self.stats["tokens"] += 1
            print(f"   📊 Stats: {self.stats['tokens']} tokens created")
        
        @EventHandler.on(EventType.GRAPH_CONNECTION_ADDED)
        async def count_connections(self, event: Event):
            self.stats["connections"] += 1
            print(f"   📊 Stats: {self.stats['connections']} connections created")
    
    # Инициализация компонентов
    print("🔧 Initializing components...\n")
    token_service = TokenService()
    graph_manager = GraphManager()
    monitor = SystemMonitor()
    
    # Регистрация всех обработчиков
    EventHandler.register_all()
    print("✅ All handlers registered\n")
    
    # Симуляция работы системы
    print("🚀 Simulating system activity...\n")
    
    await token_service.create_token("tok_001", {"type": "data", "value": 42})
    await asyncio.sleep(0.05)
    
    await token_service.create_token("tok_002", {"type": "neural", "important": True})
    await asyncio.sleep(0.05)
    
    await token_service.create_token("tok_003", {"type": "experience", "value": 100})
    await asyncio.sleep(0.05)
    
    print("\n📈 Final Statistics:")
    print(f"   Tokens created: {monitor.stats['tokens']}")
    print(f"   Connections created: {monitor.stats['connections']}")
    print(f"   Total events: {bus.get_metrics()['total_published']}\n")


# =============================================================================
# Main - Запуск всех примеров
# =============================================================================

async def main():
    """Запуск всех примеров по очереди"""
    
    # Подход 1
    await approach_1_explicit()
    GlobalEventBus.reset()  # Сброс для следующего примера
    await asyncio.sleep(0.5)
    
    # Подход 2
    await approach_2_convenience()
    GlobalEventBus.reset()
    await asyncio.sleep(0.5)
    
    # Подход 3
    await approach_3_context_manager()
    GlobalEventBus.reset()
    await asyncio.sleep(0.5)
    
    # Подход 4
    await approach_4_decorator()
    GlobalEventBus.reset()
    await asyncio.sleep(0.5)
    
    # Bonus
    await bonus_full_integration()
    
    print("\n" + "="*70)
    print("✅ All examples completed successfully!")
    print("="*70 + "\n")
    
    print("""
╔════════════════════════════════════════════════════════════════════╗
║                         Summary                                     ║
╠════════════════════════════════════════════════════════════════════╣
║                                                                     ║
║  Approach 1: Explicit Management                                   ║
║  ✓ Maximum control                                                 ║
║  ✓ Good for complex scenarios                                      ║
║  ⚠ Requires manual lifecycle management                            ║
║                                                                     ║
║  Approach 2: Convenience Functions                                 ║
║  ✓ Simple and readable                                             ║
║  ✓ Recommended for most cases                                      ║
║  ✓ Easy to understand                                              ║
║                                                                     ║
║  Approach 3: Context Manager                                       ║
║  ✓ Automatic lifecycle management                                  ║
║  ✓ Guaranteed cleanup                                              ║
║  ✓ Good for scoped operations                                      ║
║                                                                     ║
║  Approach 4: Decorator                                             ║
║  ✓ Shortest code                                                   ║
║  ✓ Perfect for async main functions                                ║
║  ✓ Clean and elegant                                               ║
║                                                                     ║
║  Bonus: Full Integration                                           ║
║  ✓ Real-world example                                              ║
║  ✓ Multiple components interacting                                 ║
║  ✓ Event chains and reactions                                      ║
║                                                                     ║
╠════════════════════════════════════════════════════════════════════╣
║  Choose the approach that fits your needs!                         ║
╚════════════════════════════════════════════════════════════════════╝
    """)


if __name__ == "__main__":
    asyncio.run(main())