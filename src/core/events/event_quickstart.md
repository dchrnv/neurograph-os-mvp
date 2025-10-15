# Event System - Quick Start 🚀

## 10-секундный старт с GlobalEventBus

```python
from core.events import start_event_bus, get_event_bus, Event, EventType

# 1. Запустить шину (один раз в приложении)
await start_event_bus()

# 2. Получить шину где угодно
bus = get_event_bus()

# 3. Опубликовать событие
event = Event(
    type=EventType.TOKEN_CREATED,
    category=EventCategory.TOKEN,
    source="my_service",
    payload={"id": "tok_123"}
)
await bus.publish(event)
```

---

## 4 способа использования

### 1️⃣ Convenience Functions (Рекомендуется)

```python
from core.events import start_event_bus, stop_event_bus, get_event_bus

async def main():
    # Запуск
    await start_event_bus()
    
    # Использование
    bus = get_event_bus()
    await bus.publish(event)
    
    # Остановка
    await stop_event_bus()
```

### 2️⃣ Context Manager (Автоматическое управление)

```python
from core.events import EventBusContext

async def main():
    async with EventBusContext() as bus:
        # Шина автоматически запущена
        await bus.publish(event)
    # Шина автоматически остановлена
```

### 3️⃣ Decorator (Самый короткий)

```python
from core.events import with_event_bus, get_event_bus

@with_event_bus()
async def main():
    # Шина уже запущена!
    bus = get_event_bus()
    await bus.publish(event)

asyncio.run(main())
```

### 4️⃣ Explicit (Максимальный контроль)

```python
from core.events import GlobalEventBus

async def main():
    bus = await GlobalEventBus.start()
    await bus.publish(event)
    await GlobalEventBus.stop()
```

---

## 30-секундный старт

```python
from core.events import EventBus, Event, EventType, EventCategory

# 1. Создать и запустить шину
event_bus = EventBus()
await event_bus.start()

# 2. Подписаться
async def handler(event: Event):
    print(f"Got: {event.type}")

event_bus.subscribe(handler, "my_module")

# 3. Опубликовать
event = Event(
    type=EventType.TOKEN_CREATED,
    category=EventCategory.TOKEN,
    source="my_service",
    payload={"id": "tok_123"}
)
await event_bus.publish(event)
```

---

## Основные паттерны

### 1. Генерация событий (Publisher)

```python
from core.events import EventEmitter

class MyService(EventEmitter):
    def __init__(self, event_bus):
        super().__init__(event_bus, source_id="my_service")
    
    async def do_something(self):
        await self.emit(
            EventType.TOKEN_CREATED,
            payload={"result": "success"}
        )
```

### 2. Обработка событий (Subscriber)

```python
from core.events import EventHandler

@EventHandler.on(EventType.TOKEN_CREATED)
async def handle_token(event: Event):
    print(f"Token: {event.payload}")
```

### 3. Фильтрация

```python
from core.events import EventFilter, EventPriority

# Только важные события токенов
@EventHandler.on(
    categories=[EventCategory.TOKEN],
    min_priority=EventPriority.HIGH
)
async def handle_important_tokens(event: Event):
    pass
```

### 4. Цепочки событий

```python
@EventHandler.on(EventType.TOKEN_CREATED)
async def auto_create_connection(event: Event):
    # Реагируем на создание токена
    # Создаем связь → генерируется GRAPH_CONNECTION_ADDED
    await graph_manager.add_connection(...)
```

---

## Типы событий

### Token
- `TOKEN_CREATED` — создан
- `TOKEN_UPDATED` — обновлен  
- `TOKEN_DELETED` — удален
- `TOKEN_ACTIVATED` — активирован

### Graph
- `GRAPH_CONNECTION_ADDED` — добавлена связь
- `GRAPH_CONNECTION_REMOVED` — удалена связь
- `GRAPH_STRUCTURE_CHANGED` — изменена структура

### DNA
- `DNA_MUTATED` — изменена ДНК
- `DNA_VALIDATED` — валидирована
- `DNA_CONSTRAINT_VIOLATED` — нарушено ограничение

### Experience
- `EXPERIENCE_RECORDED` — записан опыт
- `EXPERIENCE_BATCH_READY` — батч готов

### Evolution
- `EVOLUTION_GENERATION_STARTED` — начато поколение
- `EVOLUTION_FITNESS_IMPROVED` — улучшен фитнес

### System & Errors
- `SYSTEM_STARTED/STOPPED` — система запущена/остановлена
- `ERROR_OCCURRED` — ошибка

---

## Приоритеты

```python
EventPriority.CRITICAL  # 10 - критично (ошибки, безопасность)
EventPriority.HIGH      # 7  - важно (DNA, эволюция)
EventPriority.NORMAL    # 5  - обычно (токены, граф)
EventPriority.LOW       # 3  - неважно (статистика)
EventPriority.DEBUG     # 1  - отладка
```

---

## Полезные методы

```python
# Метрики
metrics = event_bus.get_metrics()
print(f"Published: {metrics['total_published']}")

# Информация о подписках
info = event_bus.get_subscriptions_info()
print(f"Subscriptions: {info['total_subscriptions']}")

# Создать дочернее событие
child = parent_event.create_child_event(
    event_type=EventType.GRAPH_CONNECTION_ADDED,
    payload={...},
    source="child_service"
)

# Проверки
event.is_critical()   # Критичное?
event.is_targeted()   # Адресное?

# Ошибки
await self.emit_error(
    error_type="validation_error",
    error_message="Invalid data"
)
```

---

## Конфигурация

`config/core/event_bus.yaml`:

```yaml
event_bus:
  max_queue_size: 10000
  enable_metrics: true
  log_events: false  # true для отладки
```

---

## Инициализация в приложении

```python
from core.events import EventHandler
from core.events.global_bus import start_event_bus, get_event_bus

async def main():
    # Запускаем шину
    event_bus = await start_event_bus()
    
    # Для декораторов
    EventHandler.set_event_bus(event_bus)
    
    # Создаем компоненты с event_bus
    service = MyService(event_bus)
    
    # Регистрируем обработчики
    EventHandler.register_all()
    
    # Работаем...
    await service.do_something()
```

---

## Тестирование

```python
import pytest

@pytest.mark.asyncio
async def test_events():
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

## Best Practices ✅

✅ Делай обработчики быстрыми  
✅ Используй async/await  
✅ Фильтруй события разумно  
✅ Логируй только критичное  
✅ Тестируй цепочки событий  
✅ Используй correlation_id для связи  
✅ Держи payload компактным (< 1MB)  

❌ Не создавай циклы событий  
❌ Не блокируй обработчики  
❌ Не игнорируй ошибки  

---

## Примеры

**Полный пример**: `examples/event_integration_example.py`

```bash
python examples/event_integration_example.py
```

**Тесты**: `tests/core/events/test_event_system.py`

```bash
pytest tests/core/events/ -v
```

---

## Документация

📖 [Полная документация](./EVENTS.md)  
🔧 [Гид по миграции](./MIGRATION.md)  
🧪 [Примеры](../examples/)  

---

## Поддержка

Если что-то не работает:

1. Проверь, что EventBus запущен (`await bus.start()`)
2. Установи `log_events=True` для отладки
3. Проверь метрики (`bus.get_metrics()`)
4. Проверь подписки (`bus.get_subscriptions_info()`)

---

**Готово! Теперь твоя система живая и реактивная!** ⚡🧠
