# NeuroGraph OS - Event System 🎯

> **Кровеносная система NeuroGraph OS**  
> Асинхронная, слабосвязанная коммуникация между всеми модулями через Event-Driven Architecture

[![Status](https://img.shields.io/badge/status-production-green)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue)]()
[![Python](https://img.shields.io/badge/python-3.10+-blue)]()
[![Async](https://img.shields.io/badge/async-asyncio-orange)]()

---

## 🎯 Что это?

**Event System** превращает статические модули NeuroGraph OS в **живой, реактивный организм**, где компоненты общаются через события, реагируют на изменения автоматически и работают слаженно.

### Ключевые возможности

✨ **Слабая связность** — модули независимы, общаются только через события  
⚡ **Реактивность** — автоматическая реакция на изменения в реальном времени  
🔄 **Event Chains** — события порождают другие события  
🎯 **Целевая доставка** — адресация конкретным модулям  
📊 **Метрики** — встроенный мониторинг производительности  
🛡️ **Graceful Shutdown** — корректная остановка без потери событий  

---

## 🚀 Quick Start

### 10 секунд до первого события

```python
from core.events import start_event_bus, get_event_bus, Event, EventType, EventCategory

# 1. Запустить
await start_event_bus()

# 2. Опубликовать
bus = get_event_bus()
await bus.publish(Event(
    type=EventType.TOKEN_CREATED,
    category=EventCategory.TOKEN,
    source="my_service",
    payload={"token_id": "tok_123"}
))
```

### 30 секунд до полной интеграции

```python
from core.events import start_event_bus, EventHandler, EventEmitter, EventType

# Запуск
await start_event_bus()

# Генератор событий
class MyService(EventEmitter):
    async def create_token(self):
        await self.emit(EventType.TOKEN_CREATED, {"id": "tok_123"})

# Обработчик событий
@EventHandler.on(EventType.TOKEN_CREATED)
async def handle_token(event):
    print(f"Token: {event.payload}")

# Готово!
```

---

## 📚 Документация

| Документ | Описание | Для кого |
|----------|----------|----------|
| **[QUICKSTART.md](./QUICKSTART.md)** | Быстрый старт за 30 секунд | Все |
| **[CHEATSHEET.md](./CHEATSHEET.md)** | Шпаргалка по всем возможностям | Разработчики |
| **[EVENTS.md](./EVENTS.md)** | Полная документация | Детальное изучение |
| **[MIGRATION.md](./MIGRATION.md)** | Гид по интеграции в проект | Интеграция |

### 📖 Рекомендуемый порядок изучения

1. 🚀 **QUICKSTART.md** — начни отсюда (5 минут)
2. 📋 **CHEATSHEET.md** — держи под рукой
3. 💻 **examples/event_usage_example.py** — запусти примеры
4. 📚 **EVENTS.md** — углублённое изучение
5. 🔧 **MIGRATION.md** — интеграция в проект

---

## 📦 Структура

```
src/core/events/
├── 📄 __init__.py              # Экспорты модуля
├── 📄 event.py                 # Модели (Event, EventType, EventFilter)
├── 📄 event_bus.py             # Шина событий (EventBus)
├── 📄 decorators.py            # Декораторы (@EventHandler, EventEmitter)
├── 📄 global_bus.py            # Глобальный singleton (GlobalEventBus)
│
├── 📖 README.md                # Этот файл
├── 📖 EVENTS.md                # Полная документация
├── 📖 QUICKSTART.md            # Быстрый старт
├── 📖 CHEATSHEET.md            # Шпаргалка
└── 📖 MIGRATION.md             # Гид по интеграции

config/core/
└── 📄 event_bus.yaml           # Конфигурация

examples/
├── 📄 event_integration_example.py   # Интеграция с компонентами
└── 📄 event_usage_example.py         # Примеры использования

tests/core/events/
└── 📄 test_event_system.py     # Unit & Integration тесты
```

---

## 💡 Основные концепции

### Event (Событие)

Базовая единица коммуникации:

```python
Event(
    type=EventType.TOKEN_CREATED,      # Что произошло
    category=EventCategory.TOKEN,      # Категория
    source="token_service",            # Откуда
    payload={"token_id": "tok_123"},   # Данные
    priority=EventPriority.NORMAL      # Приоритет
)
```

### EventBus (Шина событий)

Центральный маршрутизатор:

```python
bus = EventBus()
await bus.start()
await bus.publish(event)
await bus.stop()
```

### EventEmitter (Генератор)

Mixin для классов, генерирующих события:

```python
class TokenService(EventEmitter):
    async def create_token(self):
        await self.emit(EventType.TOKEN_CREATED, {...})
```

### @EventHandler (Обработчик)

Декоратор для подписки на события:

```python
@EventHandler.on(EventType.TOKEN_CREATED)
async def handle_token(event):
    print(event.payload)
```

### GlobalEventBus (Singleton)

Глобальный доступ к шине:

```python
await start_event_bus()          # Запуск
bus = get_event_bus()            # Получение
await bus.publish(event)         # Использование
await stop_event_bus()           # Остановка
```

---

## 🎨 4 способа использования

### 1️⃣ Convenience Functions (👍 Рекомендуется)

```python
from core.events import start_event_bus, get_event_bus

await start_event_bus()
bus = get_event_bus()
await bus.publish(event)
```

**Когда использовать:** Большинство случаев

### 2️⃣ Context Manager (Автоматика)

```python
from core.events import EventBusContext

async with EventBusContext() as bus:
    await bus.publish(event)
```

**Когда использовать:** Для ограниченного scope

### 3️⃣ Decorator (Элегантность)

```python
from core.events import with_event_bus

@with_event_bus()
async def main():
    bus = get_event_bus()
    await bus.publish(event)
```

**Когда использовать:** Для async main функций

### 4️⃣ Explicit (Контроль)

```python
from core.events import GlobalEventBus

bus = await GlobalEventBus.start()
await bus.publish(event)
await GlobalEventBus.stop()
```

**Когда использовать:** Для максимального контроля

---

## 🔥 Примеры

### Базовое использование

```python
from core.events import start_event_bus, EventHandler, EventEmitter

# Запускаем систему
await start_event_bus()

# Обработчик
@EventHandler.on(EventType.TOKEN_CREATED)
async def handle_token(event):
    print(f"Token created: {event.payload['token_id']}")

# Генератор
class TokenService(EventEmitter):
    async def create_token(self, token_id):
        await self.emit(
            EventType.TOKEN_CREATED,
            payload={"token_id": token_id}
        )

# Использование
service = TokenService(get_event_bus(), "token_service")
await service.create_token("tok_123")
```

### Интеграция компонентов

```python
# TokenService генерирует события
class TokenService(EventEmitter):
    async def create_token(self, data):
        token = Token(data)
        await self.emit(EventType.TOKEN_CREATED, {"token_id": token.id})

# GraphManager реагирует автоматически
class GraphManager(EventEmitter):
    @EventHandler.on(EventType.TOKEN_CREATED)
    async def auto_connect(self, event):
        await self.add_connection(event.payload["token_id"])
        # → генерирует GRAPH_CONNECTION_ADDED автоматически

# DNAGuardian валидирует
class DNAGuardian(EventEmitter):
    @EventHandler.on(EventType.TOKEN_CREATED)
    async def validate(self, event):
        if not self.is_valid(event.payload):
            await self.emit(EventType.DNA_CONSTRAINT_VIOLATED, {...})
```

### Цепочки событий

```python
# Событие A → Событие B → Событие C
@EventHandler.on(EventType.TOKEN_CREATED)
async def on_token_created(event):
    # Создаём связь
    await graph_manager.add_connection(...)
    # → Генерируется GRAPH_CONNECTION_ADDED

@EventHandler.on(EventType.GRAPH_CONNECTION_ADDED)
async def on_connection_added(event):
    # Записываем в experience
    await experience_stream.record(...)
    # → Генерируется EXPERIENCE_RECORDED
```

---

## 🧪 Тестирование

### Запуск тестов

```bash
# Все тесты
pytest tests/core/events/ -v

# Конкретный тест
pytest tests/core/events/test_event_system.py::TestEventBus -v

# С покрытием
pytest tests/core/events/ --cov=src/core/events --cov-report=html
```

### Запуск примеров

```bash
# Интеграция с компонентами
python examples/event_integration_example.py

# Различные способы использования
python examples/event_usage_example.py
```

---

## 📊 Типы событий

### Token Events
- `TOKEN_CREATED` — токен создан
- `TOKEN_UPDATED` — токен обновлён
- `TOKEN_DELETED` — токен удалён
- `TOKEN_ACTIVATED` — токен активирован

### Graph Events
- `GRAPH_CONNECTION_ADDED` — добавлена связь
- `GRAPH_CONNECTION_REMOVED` — удалена связь
- `GRAPH_STRUCTURE_CHANGED` — изменена структура
- `GRAPH_CLUSTER_DETECTED` — обнаружен кластер

### DNA Events
- `DNA_MUTATED` — DNA изменена
- `DNA_VALIDATED` — DNA валидирована
- `DNA_CONSTRAINT_VIOLATED` — нарушено ограничение

### Experience Events
- `EXPERIENCE_RECORDED` — записан опыт
- `EXPERIENCE_BATCH_READY` — батч готов
- `EXPERIENCE_TRAJECTORY_COMPLETED` — траектория завершена

### Evolution Events
- `EVOLUTION_GENERATION_STARTED` — начато поколение
- `EVOLUTION_GENERATION_COMPLETED` — завершено поколение
- `EVOLUTION_FITNESS_IMPROVED` — улучшен фитнес

### System & Error Events
- `SYSTEM_STARTED/STOPPED` — система запущена/остановлена
- `ERROR_OCCURRED` — произошла ошибка
- `ERROR_ACCESS_DENIED` — доступ запрещён

[Полный список в EventType](./event.py)

---

## ⚙️ Конфигурация

### Программная конфигурация

```python
await start_event_bus(
    max_queue_size=5000,
    enable_metrics=True,
    log_events=False
)
```

### Через файл конфигурации

`config/core/event_bus.yaml`:

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
```

---

## 📈 Метрики и мониторинг

```python
# Получить метрики
metrics = bus.get_metrics()
print(f"Published: {metrics['total_published']}")
print(f"Delivered: {metrics['total_delivered']}")
print(f"Delivery rate: {metrics['delivery_rate']:.2%}")

# Информация о подписках
info = bus.get_subscriptions_info()
print(f"Total subscriptions: {info['total_subscriptions']}")
print(f"Queue size: {info['queue_size']}")
```

---

## 🔧 Интеграция в существующий проект

### Шаг 1: Добавить в main.py

```python
from core.events import start_event_bus, EventHandler

async def initialize_neurograph():
    # Запускаем Event Bus
    event_bus = await start_event_bus()
    
    # Устанавливаем для декораторов
    EventHandler.set_event_bus(event_bus)
    
    # Создаём компоненты с event_bus
    token_service = TokenService(event_bus)
    graph_manager = GraphManager(event_bus)
    
    # Регистрируем обработчики
    EventHandler.register_all()
    
    return {"event_bus": event_bus, ...}
```

### Шаг 2: Обновить компоненты

```python
# Было
class TokenService:
    def create_token(self, data):
        token = Token(data)
        return token

# Стало
class TokenService(EventEmitter):
    async def create_token(self, data):
        token = Token(data)
        await self.emit(EventType.TOKEN_CREATED, {"token_id": token.id})
        return token
```

[Подробнее в MIGRATION.md](./MIGRATION.md)

---

## 🎓 Best Practices

### ✅ DO

- Используй `start_event_bus()` для простоты
- Наследуй `EventEmitter` для генерации событий
- Используй `@EventHandler.on()` для обработки
- Фильтруй события по категории/типу/приоритету
- Делай обработчики быстрыми (< 100ms)
- Используй `correlation_id` для связи событий
- Тестируй цепочки событий
- Логируй только критичные события

### ❌ DON'T

- Не создавай циклы событий (A → B → A)
- Не блокируй обработчики тяжёлыми операциями
- Не игнорируй ошибки в обработчиках
- Не делай payload > 1MB
- Не забывай про `await` в async функциях
- Не создавай множество EventBus (используй GlobalEventBus)

---

## 🔮 Roadmap

### ✅ Version 1.0 (Current)
- ✅ Core Event System
- ✅ Async pub/sub
- ✅ Decorators & Mixins
- ✅ GlobalEventBus
- ✅ Metrics & Monitoring
- ✅ Full documentation

### 🔄 Version 1.1 (Planned)
- Priority queue
- Retry mechanism
- Dead Letter Queue (DLQ)
- Event persistence

### 🚀 Version 2.0 (Future)
- Redis Pub/Sub backend
- RabbitMQ integration
- Distributed tracing
- Horizontal scaling

---

## 🤝 Интеграция с компонентами NeuroGraph OS

| Компонент | Интеграция | Статус |
|-----------|------------|--------|
| **TokenService** | Генерация событий о токенах | ✅ Ready |
| **GraphManager** | События о связях и кластерах | ✅ Ready |
| **DNAGuardian** | Валидация через события | ✅ Ready |
| **ExperienceStream** | Запись событий как опыт | ✅ Ready |
| **SpatialIndex** | События об изменениях сетки | 🔄 Planned |
| **EvolutionEngine** | События эволюции | 🔄 Planned |

---

## 📞 FAQ

### Нужно ли создавать EventBus в каждом модуле?

Нет! Используй GlobalEventBus:
```python
from core.events import get_event_bus
bus = get_event_bus()  # Везде один и тот же экземпляр
```

### Как избежать циклов событий?

Используй `correlation_id` и проверяй глубину цепочки:
```python
if event.metadata.get("chain_depth", 0) > 10:
    return  # Останавливаем цепочку
```

### Что делать если событие не доставляется?

1. Проверь что EventBus запущен: `await start_event_bus()`
2. Включи логирование: `EventBus(log_events=True)`
3. Проверь фильтры подписки
4. Проверь метрики: `bus.get_metrics()`

### Как тестировать компоненты с событиями?

```python
@pytest.fixture
async def event_bus():
    bus = EventBus()
    await bus.start()
    yield bus
    await bus.stop()

async def test_my_component(event_bus):
    service = MyService(event_bus)
    # тестируй
```

---

## 🌟 Примеры из практики

### Real-time Dashboard

```python
@EventHandler.on(categories=[EventCategory.TOKEN, EventCategory.GRAPH])
async def update_dashboard(event):
    await websocket.send_json({
        "type": "update",
        "event": event.to_dict()
    })
```

### Automatic Backups

```python
@EventHandler.on(
    EventType.DNA_MUTATED,
    priority=EventPriority.HIGH
)
async def backup_dna(event):
    await backup_service.save_snapshot(event.payload)
```

### Performance Monitoring

```python
@EventHandler.on(categories=[EventCategory.SYSTEM])
async def monitor_performance(event):
    if event.type == EventType.SYSTEM_HEALTH_CHECK:
        metrics = collect_metrics()
        if metrics["cpu"] > 80:
            await emit_alert(EventType.ERROR_RESOURCE_EXHAUSTED)
```

---

## 💬 Поддержка

- 📖 Документация: [./EVENTS.md](./EVENTS.md)
- 🚀 Quick Start: [./QUICKSTART.md](./QUICKSTART.md)
- 📋 Cheatsheet: [./CHEATSHEET.md](./CHEATSHEET.md)
- 🔧 Migration: [./MIGRATION.md](./MIGRATION.md)
- 💻 Examples: [../examples/](../examples/)
- 🧪 Tests: [../tests/core/events/](../tests/core/events/)

---

## 📄 License

Part of NeuroGraph OS  
Copyright © 2025 NeuroGraph OS Team

---

## 🎉 Заключение

Event System — это **сердце NeuroGraph OS**, которое:

- 🔗 **Связывает** все модули системы
- ⚡ **Оживляет** архитектуру через реактивность
- 📈 **Масштабирует** систему без изменения кода
- 🎯 **Упрощает** добавление нового функционала

**Система готова к production использованию!** 🚀

---

<div align="center">

**[Начать использовать →](./QUICKSTART.md)**

Made with ❤️ for NeuroGraph OS

</div>
