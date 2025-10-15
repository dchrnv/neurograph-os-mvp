# Event System - Cheatsheet 📋

## Импорты

```python
from core.events import (
    # Модели
    Event, EventType, EventCategory, EventPriority, EventFilter,
    
    # Event Bus
    EventBus,
    
    # Декораторы
    EventHandler, EventEmitter,
    
    # Глобальная шина
    GlobalEventBus,
    get_event_bus,
    start_event_bus,
    stop_event_bus,
    EventBusContext,
    with_event_bus
)
```

---

## Инициализация

### Способ 1: Convenience (Рекомендуется)
```python
await start_event_bus()
bus = get_event_bus()
# ... работа ...
await stop_event_bus()
```

### Способ 2: Context Manager
```python
async with EventBusContext() as bus:
    # работа
```

### Способ 3: Decorator
```python
@with_event_bus()
async def main():
    bus = get_event_bus()
```

### Способ 4: Explicit
```python
bus = await GlobalEventBus.start()
# ... работа ...
await GlobalEventBus.stop()
```

---

## Создание событий

### Базовое
```python
event = Event(
    type=EventType.TOKEN_CREATED,
    category=EventCategory.TOKEN,
    source="my_service",
    payload={"token_id": "tok_123"}
)
```

### С приоритетом
```python
event = Event(
    type=EventType.ERROR_OCCURRED,
    category=EventCategory.ERROR,
    source="my_service",
    payload={"error": "Something went wrong"},
    priority=EventPriority.CRITICAL
)
```

### С целевыми получателями
```python
event = Event(
    type=EventType.TOKEN_UPDATED,
    category=EventCategory.TOKEN,
    source="my_service",
    payload={"token_id": "tok_123"},
    target=["graph_manager", "dna_guardian"]
)
```

### Дочернее событие
```python
child = parent_event.create_child_event(
    event_type=EventType.GRAPH_CONNECTION_ADDED,
    payload={"connection_id": "conn_456"},
    source="graph_service"
)
```

---

## Публикация

### Прямая публикация
```python
await bus.publish(event)
```

### Через EventEmitter
```python
class MyService(EventEmitter):
    def __init__(self, bus):
        super().__init__(bus, source_id="my_service")
    
    async def do_work(self):
        await self.emit(
            EventType.TOKEN_CREATED,
            payload={"result": "success"}
        )
```

### Публикация ошибки
```python
await self.emit_error(
    error_type="validation_error",
    error_message="Invalid data",
    error_details={"field": "value"}
)
```

---

## Подписка

### Базовая подписка
```python
async def handler(event: Event):
    print(event.payload)

bus.subscribe(handler, "my_module")
```

### С фильтром
```python
filter = EventFilter(
    types=[EventType.TOKEN_CREATED],
    min_priority=EventPriority.HIGH
)

bus.subscribe(handler, "my_module", event_filter=filter)
```

### Декоратор (простой)
```python
@EventHandler.on(EventType.TOKEN_CREATED)
async def handle_token(event: Event):
    print(event.payload)
```

### Декоратор (с фильтром)
```python
@EventHandler.on(
    event_types=[EventType.TOKEN_CREATED, EventType.TOKEN_UPDATED],
    min_priority=EventPriority.HIGH,
    subscription_name="important_tokens"
)
async def handle_important(event: Event):
    print(event.payload)
```

### Декоратор (по категории)
```python
@EventHandler.on(
    categories=[EventCategory.TOKEN, EventCategory.GRAPH]
)
async def handle_token_and_graph(event: Event):
    print(event.payload)
```

### Отписка
```python
sub_id = bus.subscribe(handler, "my_module")
bus.unsubscribe(sub_id)

# Или по имени
bus.unsubscribe_by_name("important_tokens")
```

---

## Типы событий

### Token
```python
EventType.TOKEN_CREATED
EventType.TOKEN_UPDATED
EventType.TOKEN_DELETED
EventType.TOKEN_ACTIVATED
```

### Graph
```python
EventType.GRAPH_CONNECTION_ADDED
EventType.GRAPH_CONNECTION_REMOVED
EventType.GRAPH_STRUCTURE_CHANGED
EventType.GRAPH_CLUSTER_DETECTED
```

### DNA
```python
EventType.DNA_MUTATED
EventType.DNA_VALIDATED
EventType.DNA_CONSTRAINT_VIOLATED
```

### Experience
```python
EventType.EXPERIENCE_RECORDED
EventType.EXPERIENCE_BATCH_READY
EventType.EXPERIENCE_TRAJECTORY_COMPLETED
```

### Evolution
```python
EventType.EVOLUTION_GENERATION_STARTED
EventType.EVOLUTION_GENERATION_COMPLETED
EventType.EVOLUTION_FITNESS_IMPROVED
EventType.EVOLUTION_TRIGGER
```

### System
```python
EventType.SYSTEM_STARTED
EventType.SYSTEM_STOPPED
EventType.SYSTEM_CONFIG_CHANGED
EventType.SYSTEM_HEALTH_CHECK
```

### Error
```python
EventType.ERROR_OCCURRED
EventType.ERROR_ACCESS_DENIED
EventType.ERROR_VALIDATION_FAILED
EventType.ERROR_RESOURCE_EXHAUSTED
```

---

## Категории

```python
EventCategory.TOKEN
EventCategory.GRAPH
EventCategory.SPATIAL
EventCategory.DNA
EventCategory.EXPERIENCE
EventCategory.EVOLUTION
EventCategory.SYSTEM
EventCategory.ERROR
```

---

## Приоритеты

```python
EventPriority.CRITICAL  # 10
EventPriority.HIGH      # 7
EventPriority.NORMAL    # 5
EventPriority.LOW       # 3
EventPriority.DEBUG     # 1
```

---

## Метрики

```python
# Получить метрики
metrics = bus.get_metrics()
print(metrics['total_published'])
print(metrics['total_delivered'])
print(metrics['delivery_rate'])
print(metrics['events_by_type'])

# Информация о подписках
info = bus.get_subscriptions_info()
print(info['total_subscriptions'])
print(info['queue_size'])
print(info['is_running'])
```

---

## Проверки

```python
# Событие
event.is_critical()    # Критичное?
event.is_targeted()    # Адресное?

# Шина
GlobalEventBus.is_initialized()  # Инициализирована?
GlobalEventBus.is_running()      # Запущена?
```

---

## Интеграция компонентов

### TokenService
```python
class TokenService(EventEmitter):
    def __init__(self, bus):
        super().__init__(bus, source_id="token_service")
    
    async def create_token(self, data):
        token = Token(data)
        await self.emit(
            EventType.TOKEN_CREATED,
            payload={"token_id": token.id}
        )
```

### GraphManager
```python
class GraphManager(EventEmitter):
    def __init__(self, bus):
        super().__init__(bus, source_id="graph_manager")
    
    async def add_connection(self, source, target):
        self.graph.add_edge(source, target)
        await self.emit(
            EventType.GRAPH_CONNECTION_ADDED,
            payload={"source": source, "target": target}
        )
```

### DNAGuardian (с валидацией)
```python
class DNAGuardian(EventEmitter):
    def __init__(self, bus):
        super().__init__(bus, source_id="dna_guardian")
    
    @EventHandler.on(EventType.TOKEN_CREATED)
    async def validate_token(self, event: Event):
        if not self.is_valid(event.payload):
            await self.emit(
                EventType.DNA_CONSTRAINT_VIOLATED,
                payload={"reason": "Invalid token"},
                priority=EventPriority.CRITICAL
            )
```

---

## Паттерны

### Цепочка событий
```python
@EventHandler.on(EventType.TOKEN_CREATED)
async def auto_create_connection(event: Event):
    # Реагируем на токен
    await graph_manager.add_connection(...)
    # → Генерируется GRAPH_CONNECTION_ADDED
```

### Автоматический publisher
```python
from core.events import event_publisher

class MyService(EventEmitter):
    @event_publisher(EventType.TOKEN_CREATED)
    async def create_token(self, data):
        token = Token(data)
        return {"token_id": token.id}  # Становится payload
```

### Event responder
```python
from core.events import event_responder

@EventHandler.on(EventType.TOKEN_CREATED)
@event_responder(EventType.GRAPH_CONNECTION_ADDED)
async def respond_to_token(event: Event) -> dict:
    # Обработка
    return {"connection_id": "conn_123"}  # Новое событие
```

---

## Конфигурация

### Программно
```python
await start_event_bus(
    max_queue_size=5000,
    enable_metrics=True,
    log_events=False
)
```

### Через config
```python
config = {
    "max_queue_size": 5000,
    "enable_metrics": True,
    "log_events": False
}
await start_event_bus(config=config)
```

### Из файла (config/core/event_bus.yaml)
```yaml
event_bus:
  max_queue_size: 10000
  enable_metrics: true
  log_events: false
```

---

## Тестирование

```python
import pytest

@pytest.mark.asyncio
async def test_event_delivery():
    bus = EventBus()
    await bus.start()
    
    received = []
    async def handler(e):
        received.append(e)
    
    bus.subscribe(handler, "test")
    
    event = Event(
        type=EventType.TOKEN_CREATED,
        category=EventCategory.TOKEN,
        source="test",
        payload={}
    )
    await bus.publish(event)
    await asyncio.sleep(0.1)
    
    assert len(received) == 1
    await bus.stop()
```

---

## Примеры

### Полный пример
```bash
python examples/event_integration_example.py
```

### Примеры использования
```bash
python examples/event_usage_example.py
```

### Тесты
```bash
pytest tests/core/events/test_event_system.py -v
```

---

## Troubleshooting

### "EventBus not initialized"
```python
# Решение:
await start_event_bus()
# или
EventHandler.set_event_bus(bus)
```

### События не доставляются
```python
# Включить логирование
bus = EventBus(log_events=True)

# Проверить подписки
print(bus.get_subscriptions_info())
```

### Медленная обработка
```python
# Вынести в фон
@EventHandler.on(EventType.TOKEN_CREATED)
async def handle(event):
    asyncio.create_task(heavy_work(event))
```

---

## Best Practices

✅ Используй `start_event_bus()` для простоты  
✅ Наследуй `EventEmitter` для генерации событий  
✅ Используй `@EventHandler.on()` для обработки  
✅ Фильтруй события разумно  
✅ Делай обработчики быстрыми  
✅ Логируй только критичное  
✅ Тестируй цепочки событий  

❌ Не создавай циклы событий  
❌ Не блокируй обработчики  
❌ Не игнорируй ошибки  
❌ Не делай огромные payload (< 1MB)  

---

## Ссылки

📖 [Полная документация](./EVENTS.md)  
🚀 [Quick Start](./QUICKSTART.md)  
🔧 [Migration Guide](./MIGRATION.md)  
💻 [Примеры](../examples/)  

---

**Удачи! 🎉**
