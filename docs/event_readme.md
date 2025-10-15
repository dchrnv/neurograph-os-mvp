# NeuroGraph OS - Event System (Signals)

## 🎯 Обзор

**Event System** (Система сигналов) — это кровеносная система NeuroGraph OS, обеспечивающая асинхронную, слабосвязанную коммуникацию между всеми модулями системы через механизм публикации и подписки на события.

### Ключевые преимущества

- ✨ **Слабая связность** — модули не вызывают друг друга напрямую
- ⚡ **Реактивность** — система реагирует на изменения в реальном времени
- 🔄 **Масштабируемость** — легко добавлять новые модули и поведение
- 🎭 **Event-Driven Architecture** — современный архитектурный подход
- 📊 **Наблюдаемость** — встроенные метрики и мониторинг

---

## 📦 Структура

```
src/core/events/
├── __init__.py           # Экспорты
├── event.py              # Модели событий
├── event_bus.py          # Шина событий
├── decorators.py         # Декораторы для удобства
└── README.md             # Документация

config/core/
└── event_bus.yaml        # Конфигурация

examples/
└── event_integration_example.py  # Примеры использования

tests/core/events/
└── test_event_system.py  # Тесты
```

---

## 🚀 Быстрый старт

### 1. Базовое использование

```python
import asyncio
from core.events import EventBus, Event, EventType, EventCategory

async def main():
    # Создаем и запускаем шину
    event_bus = EventBus()
    await event_bus.start()
    
    # Подписываемся на события
    async def handler(event: Event):
        print(f"Received: {event.type} - {event.payload}")
    
    event_bus.subscribe(
        handler=handler,
        subscriber_id="my_module"
    )
    
    # Публикуем событие
    event = Event(
        type=EventType.TOKEN_CREATED,
        category=EventCategory.TOKEN,
        source="my_service",
        payload={"token_id": "tok_123"}
    )
    await event_bus.publish(event)
    
    await asyncio.sleep(0.1)
    await event_bus.stop()

asyncio.run(main())
```

### 2. Использование декораторов

```python
from core.events import EventHandler, EventEmitter, EventType

# Обработчик событий
@EventHandler.on(EventType.TOKEN_CREATED)
async def handle_token_created(event: Event):
    print(f"New token: {event.payload['token_id']}")

# Класс, генерирующий события
class TokenService(EventEmitter):
    def __init__(self, event_bus):
        super().__init__(event_bus, source_id="token_service")
    
    async def create_token(self, token_id: str):
        # ... создание токена ...
        
        # Генерация события
        await self.emit(
            EventType.TOKEN_CREATED,
            payload={"token_id": token_id}
        )
```

---

## 📚 Основные концепции

### Event (Событие)

Базовая единица коммуникации в системе:

```python
event = Event(
    type=EventType.TOKEN_CREATED,      # Тип события
    category=EventCategory.TOKEN,      # Категория
    source="token_service",            # Источник
    payload={"token_id": "tok_123"},   # Данные
    priority=EventPriority.NORMAL,     # Приоритет
    target=["module_a", "module_b"]    # Целевые получатели (опционально)
)
```

**Поля события:**
- `id` — уникальный идентификатор
- `version` — версия формата
- `type` — тип события
- `category` — категория для группировки
- `timestamp` — время создания
- `priority` — приоритет (CRITICAL, HIGH, NORMAL, LOW, DEBUG)
- `source` — модуль-источник
- `target` — конкретные получатели (опционально)
- `payload` — полезная нагрузка (dict)
- `correlation_id` — для связи связанных событий
- `parent_event_id` — для цепочек событий
- `metadata` — дополнительные метаданные

### Типы событий

Система определяет следующие категории и типы:

**Token Events:**
- `TOKEN_CREATED` — токен создан
- `TOKEN_UPDATED` — токен обновлен
- `TOKEN_DELETED` — токен удален
- `TOKEN_ACTIVATED` — токен активирован

**Graph Events:**
- `GRAPH_CONNECTION_ADDED` — добавлена связь
- `GRAPH_CONNECTION_REMOVED` — удалена связь
- `GRAPH_STRUCTURE_CHANGED` — изменена структура
- `GRAPH_CLUSTER_DETECTED` — обнаружен кластер

**DNA Events:**
- `DNA_MUTATED` — DNA изменена
- `DNA_VALIDATED` — DNA валидирована
- `DNA_CONSTRAINT_VIOLATED` — нарушено ограничение DNA

**Experience Events:**
- `EXPERIENCE_RECORDED` — записан опыт
- `EXPERIENCE_BATCH_READY` — батч готов
- `EXPERIENCE_TRAJECTORY_COMPLETED` — траектория завершена

**Evolution Events:**
- `EVOLUTION_GENERATION_STARTED` — начато поколение
- `EVOLUTION_GENERATION_COMPLETED` — завершено поколение
- `EVOLUTION_FITNESS_IMPROVED` — улучшен фитнес

**System & Error Events:**
- `SYSTEM_*` — системные события
- `ERROR_*` — ошибки

### EventBus (Шина событий)

Центральный компонент для маршрутизации событий:

```python
event_bus = EventBus(
    max_queue_size=10000,    # Размер очереди
    enable_metrics=True,     # Метрики
    log_events=False         # Логирование всех событий
)

await event_bus.start()      # Запуск
await event_bus.publish(event)  # Публикация
await event_bus.stop()       # Остановка
```

### Подписка на события

**Базовая подписка:**

```python
async def my_handler(event: Event):
    print(f"Event: {event.type}")

event_bus.subscribe(
    handler=my_handler,
    subscriber_id="my_module"
)
```

**С фильтрацией:**

```python
from core.events import EventFilter

event_filter = EventFilter(
    types=[EventType.TOKEN_CREATED, EventType.TOKEN_UPDATED],
    categories=[EventCategory.TOKEN],
    min_priority=EventPriority.HIGH,
    sources=["token_service"]
)

event_bus.subscribe(
    handler=my_handler,
    subscriber_id="my_module",
    event_filter=event_filter
)
```

**С декоратором:**

```python
@EventHandler.on(
    event_types=EventType.TOKEN_CREATED,
    min_priority=EventPriority.HIGH,
    subscription_name="important_tokens"
)
async def handle_important_tokens(event: Event):
    print(f"Important token: {event.payload}")
```

---

## 🔧 Продвинутые возможности

### 1. Цепочки событий (Event Chains)

События могут порождать другие события:

```python
# Родительское событие
parent_event = Event(
    type=EventType.TOKEN_CREATED,
    category=EventCategory.TOKEN,
    source="token_service",
    payload={"token_id": "tok_123"}
)

# Дочернее событие
child_event = parent_event.create_child_event(
    event_type=EventType.GRAPH_CONNECTION_ADDED,
    payload={"connection_id": "conn_456"},
    source="graph_service"
)

# child_event автоматически получит:
# - parent_event_id = parent_event.id
# - correlation_id = parent_event.id
# - trace_id = parent_event.id
```

### 2. Целевая доставка

События могут быть адресованы конкретным модулям:

```python
event = Event(
    type=EventType.TOKEN_UPDATED,
    category=EventCategory.TOKEN,
    source="token_service",
    payload={"token_id": "tok_123"},
    target=["graph_manager", "dna_guardian"]  # Только для этих модулей
)
```

### 3. EventEmitter Mixin

Для классов, которые генерируют события:

```python
class MyService(EventEmitter):
    def __init__(self, event_bus):
        super().__init__(event_bus, source_id="my_service")
    
    async def do_something(self):
        # ... логика ...
        
        # Простая публикация
        await self.emit(
            EventType.TOKEN_CREATED,
            payload={"result": "success"}
        )
        
        # Публикация ошибки
        await self.emit_error(
            error_type="validation_error",
            error_message="Invalid data",
            error_details={"field": "value"}
        )
```

### 4. Автоматическая публикация с декоратором

```python
class TokenService(EventEmitter):
    @event_publisher(EventType.TOKEN_CREATED)
    async def create_token(self, data):
        token = Token(data)
        # Возвращаемый dict станет payload события
        return {
            "token_id": token.id,
            "coordinates": token.coords
        }
```

### 5. Метрики и мониторинг

```python
# Получить метрики
metrics = event_bus.get_metrics()
print(f"Published: {metrics['total_published']}")
print(f"Delivered: {metrics['total_delivered']}")
print(f"Delivery rate: {metrics['delivery_rate']:.2%}")

# Информация о подписках
info = event_bus.get_subscriptions_info()
print(f"Total subscriptions: {info['total_subscriptions']}")
print(f"Queue size: {info['queue_size']}")
```

---

## 🎨 Паттерны использования

### Паттерн 1: Автоматическая реакция

```python
# При создании токена автоматически создается связь в графе
@EventHandler.on(EventType.TOKEN_CREATED)
async def auto_create_graph_connection(event: Event):
    token_id = event.payload['token_id']
    # ... создание связи в графе ...
```

### Паттерн 2: Валидация через события

```python
class DNAGuardian(EventEmitter):
    @EventHandler.on(EventType.TOKEN_CREATED)
    async def validate_token(self, event: Event):
        if not self.is_valid(event.payload):
            await self.emit(
                EventType.DNA_CONSTRAINT_VIOLATED,
                payload={"reason": "Invalid token"},
                priority=EventPriority.CRITICAL
            )
```

### Паттерн 3: Агрегация событий

```python
class EventAggregator:
    def __init__(self):
        self.events = []
    
    @EventHandler.on(categories=[EventCategory.TOKEN])
    async def collect_token_events(self, event: Event):
        self.events.append(event)
        
        if len(self.events) >= 100:
            # Обработать батч
            await self.process_batch(self.events)
            self.events.clear()
```

---

## ⚙️ Конфигурация

Конфигурация находится в `config/core/event_bus.yaml`:

```yaml
event_bus:
  max_queue_size: 10000
  enable_metrics: true
  log_events: false
  
event_types:
  token:
    enabled: true
    default_priority: "NORMAL"
  
  dna:
    enabled: true
    default_priority: "HIGH"

integrations:
  experience_stream:
    enabled: true
    record_categories:
      - "token"
      - "graph"
```

---

## 🧪 Тестирование

Запуск тестов:

```bash
pytest tests/core/events/test_event_system.py -v
```

Пример теста:

```python
@pytest.mark.asyncio
async def test_event_delivery():
    bus = EventBus()
    await bus.start()
    
    received = []
    async def handler(event):
        received.append(event)
    
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

## 🔮 Будущие возможности

### Фаза 2 (Планируется)
- ✅ Приоритетная очередь событий
- ✅ Retry механизм для критичных событий
- ✅ Dead Letter Queue (DLQ)
- ✅ Персистентность событий (Redis/PostgreSQL)

### Фаза 3 (Будущее)
- 🔄 Redis Pub/Sub backend
- 🔄 RabbitMQ интеграция
- 🔄 Распределенный трейсинг (Jaeger)
- 🔄 Горизонтальное масштабирование

---

## 📖 Примеры

Полный пример интеграции см. в `examples/event_integration_example.py`

Запуск примера:

```bash
python examples/event_integration_example.py
```

---

## 🤝 Интеграция с другими компонентами

### TokenService → Events

```python
class TokenService(EventEmitter):
    async def create_token(self, data):
        token = self.token_factory.create(data)
        await self.emit(EventType.TOKEN_CREATED, {"token_id": token.id})
```

### GraphManager → Events

```python
class GraphManager(EventEmitter):
    async def add_connection(self, source, target):
        self.graph.add_edge(source, target)
        await self.emit(EventType.GRAPH_CONNECTION_ADDED, {...})
```

### DNAGuardian → Events

```python
class DNAGuardian(EventEmitter):
    @EventHandler.on(EventType.TOKEN_CREATED)
    async def validate(self, event):
        if not self.check_constraints(event.payload):
            await self.emit(EventType.DNA_CONSTRAINT_VIOLATED, {...})
```

### ExperienceStream → Events

```python
# События автоматически записываются как experience
event_bus.publish(event)  # → ExperienceStream.record(event)
```

---

## 📝 Best Practices

1. **Именование событий**: Используйте глаголы в прошедшем времени (`TOKEN_CREATED`, не `CREATE_TOKEN`)

2. **Размер payload**: Держите payload компактным (< 1MB)

3. **Приоритеты**: Используйте HIGH/CRITICAL только для действительно важных событий

4. **Обработчики**: Делайте обработчики быстрыми, выносите тяжелую работу в фон

5. **Ошибки**: Всегда обрабатывайте ошибки в обработчиках

6. **Тестирование**: Тестируйте цепочки событий

---

## 🎓 Заключение

Event System — это ключевой компонент NeuroGraph OS, который:

- 🔗 Связывает все модули системы
- ⚡ Обеспечивает реактивность
- 📈 Позволяет масштабироваться
- 🎯 Упрощает добавление нового функционала

**Система готова к использованию и активной разработке!** 🚀
