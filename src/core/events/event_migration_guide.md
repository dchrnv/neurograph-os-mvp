# NeuroGraph OS - Event System Migration Guide

## 🎯 Цель

Это руководство поможет интегрировать **Event System** в существующие компоненты NeuroGraph OS.

---

## 📋 Общий план интеграции

### Этап 1: Подготовка (1-2 часа)
1. ✅ Добавить Event System в проект
2. ✅ Обновить конфигурацию
3. ✅ Создать глобальный EventBus

### Этап 2: Интеграция компонентов (3-5 часов)
1. ✅ TokenService → Events
2. ✅ GraphManager → Events  
3. ✅ DNAGuardian → Events
4. ✅ ExperienceStream → Events
5. ✅ SpatialIndex → Events (опционально)

### Этап 3: Тестирование (2-3 часа)
1. ✅ Unit tests
2. ✅ Integration tests
3. ✅ Performance tests

---

## 🚀 Шаг 1: Установка Event System

### 1.1. Добавить файлы

Скопировать в проект:
```
src/core/events/
├── __init__.py
├── event.py
├── event_bus.py
└── decorators.py

config/core/
└── event_bus.yaml

examples/
└── event_integration_example.py
```

### 1.2. Обновить зависимости

В `requirements.txt` добавить (если нужно):
```
pydantic>=2.0.0
```

### 1.3. Создать глобальный EventBus

Создать файл `src/core/events/global_bus.py`:

```python
"""
Global Event Bus instance for NeuroGraph OS
"""
from typing import Optional
from .event_bus import EventBus
from infrastructure.config import ConfigLoader

_global_event_bus: Optional[EventBus] = None

def get_event_bus() -> EventBus:
    """Get global event bus instance"""
    global _global_event_bus
    
    if _global_event_bus is None:
        # Load config
        config = ConfigLoader.load("config/core/event_bus.yaml")
        
        # Create bus
        _global_event_bus = EventBus(
            max_queue_size=config.get("event_bus.max_queue_size", 10000),
            enable_metrics=config.get("event_bus.enable_metrics", True),
            log_events=config.get("event_bus.log_events", False)
        )
    
    return _global_event_bus

async def start_event_bus():
    """Start global event bus"""
    bus = get_event_bus()
    await bus.start()
    return bus

async def stop_event_bus():
    """Stop global event bus"""
    if _global_event_bus:
        await _global_event_bus.stop()
```

---

## 🔧 Шаг 2: Интеграция с TokenService

### Было:
```python
class TokenService:
    def __init__(self, factory: TokenFactory):
        self.factory = factory
        self.tokens = {}
    
    def create_token(self, data: dict) -> Token:
        token = self.factory.create(data)
        self.tokens[token.id] = token
        return token
```

### Стало:
```python
from core.events import EventEmitter, EventType, EventPriority

class TokenService(EventEmitter):
    def __init__(self, factory: TokenFactory, event_bus: EventBus):
        super().__init__(event_bus, source_id="token_service")
        self.factory = factory
        self.tokens = {}
    
    async def create_token(self, data: dict) -> Token:
        token = self.factory.create(data)
        self.tokens[token.id] = token
        
        # Публикуем событие
        await self.emit(
            EventType.TOKEN_CREATED,
            payload={
                "token_id": token.id,
                "coordinates": token.coordinates.to_list(),
                "token_type": data.get("type", "unknown")
            }
        )
        
        return token
    
    async def update_token(self, token_id: str, updates: dict):
        if token_id not in self.tokens:
            await self.emit_error(
                error_type="token_not_found",
                error_message=f"Token {token_id} not found"
            )
            return
        
        token = self.tokens[token_id]
        # ... обновление токена ...
        
        await self.emit(
            EventType.TOKEN_UPDATED,
            payload={"token_id": token_id, "updates": updates}
        )
```

### Ключевые изменения:
1. ✅ Наследование от `EventEmitter`
2. ✅ Передача `event_bus` в конструктор
3. ✅ Методы стали `async`
4. ✅ Вызов `self.emit()` после изменений
5. ✅ Использование `self.emit_error()` для ошибок

---

## 🕸️ Шаг 3: Интеграция с GraphManager

### Было:
```python
class GraphManager:
    def __init__(self, graph_engine: TokenGraph):
        self.graph = graph_engine
    
    def add_connection(self, source: str, target: str, weight: float = 1.0):
        self.graph.add_connection(source, target, weight)
```

### Стало:
```python
from core.events import EventEmitter, EventType

class GraphManager(EventEmitter):
    def __init__(self, graph_engine: TokenGraph, event_bus: EventBus):
        super().__init__(event_bus, source_id="graph_manager")
        self.graph = graph_engine
    
    async def add_connection(
        self, 
        source: str, 
        target: str, 
        weight: float = 1.0,
        connection_type: str = "spatial"
    ):
        # Добавляем связь
        self.graph.add_connection(source, target, weight)
        
        # Публикуем событие
        await self.emit(
            EventType.GRAPH_CONNECTION_ADDED,
            payload={
                "source": source,
                "target": target,
                "weight": weight,
                "connection_type": connection_type,
                "total_connections": len(self.graph.get_all_connections())
            }
        )
    
    async def detect_clusters(self):
        clusters = self.graph.find_clusters()
        
        for cluster in clusters:
            await self.emit(
                EventType.GRAPH_CLUSTER_DETECTED,
                payload={
                    "cluster_id": cluster.id,
                    "size": len(cluster.nodes),
                    "density": cluster.density
                },
                priority=EventPriority.HIGH if len(cluster.nodes) > 100 else EventPriority.NORMAL
            )
```

---

## 🧬 Шаг 4: Интеграция с DNAGuardian

### Было:
```python
class DNAGuardian:
    def __init__(self, spec: DNASpec):
        self.spec = spec
    
    def validate(self, operation: str, params: dict) -> bool:
        return self.spec.validate(operation, params)
```

### Стало:
```python
from core.events import EventEmitter, EventHandler, EventType, EventPriority

class DNAGuardian(EventEmitter):
    def __init__(self, spec: DNASpec, event_bus: EventBus):
        super().__init__(event_bus, source_id="dna_guardian")
        self.spec = spec
    
    def validate(self, operation: str, params: dict) -> bool:
        is_valid = self.spec.validate(operation, params)
        
        if not is_valid:
            # Асинхронная публикация нарушения
            asyncio.create_task(self.emit(
                EventType.DNA_CONSTRAINT_VIOLATED,
                payload={
                    "operation": operation,
                    "params": params,
                    "violated_rule": self.spec.get_violated_rule(operation, params)
                },
                priority=EventPriority.CRITICAL
            ))
        
        return is_valid
    
    @EventHandler.on(EventType.TOKEN_CREATED)
    async def validate_token_creation(self, event: Event):
        """Автоматическая валидация при создании токена"""
        token_data = event.payload
        
        if not self.validate("create_token", token_data):
            await self.emit(
                EventType.DNA_CONSTRAINT_VIOLATED,
                payload={
                    "reason": "Token creation violates DNA constraints",
                    "token_data": token_data
                },
                priority=EventPriority.CRITICAL,
                correlation_id=event.id
            )
    
    async def mutate_dna(self, mutation: dict):
        """Изменение DNA спецификации"""
        old_spec = self.spec.to_dict()
        self.spec.apply_mutation(mutation)
        new_spec = self.spec.to_dict()
        
        await self.emit(
            EventType.DNA_MUTATED,
            payload={
                "mutation": mutation,
                "old_spec": old_spec,
                "new_spec": new_spec
            },
            priority=EventPriority.HIGH
        )
```

---

## 💫 Шаг 5: Интеграция с ExperienceStream

### Автоматическая запись событий как Experience

```python
from core.events import EventHandler, EventCategory
from core.experience import ExperienceStream

class ExperienceEventIntegration:
    """Интеграция Event System с Experience Stream"""
    
    def __init__(self, experience_stream: ExperienceStream):
        self.stream = experience_stream
    
    @EventHandler.on(
        categories=[
            EventCategory.TOKEN,
            EventCategory.GRAPH,
            EventCategory.EVOLUTION
        ],
        subscription_name="experience_recorder"
    )
    async def record_event_as_experience(self, event: Event):
        """Записывать значимые события как experience"""
        
        # Конвертируем Event в ExperienceEvent
        experience_event = ExperienceEvent(
            event_type=f"event.{event.type.value}",
            state={
                "event_id": event.id,
                "category": event.category.value,
                "source": event.source
            },
            action=event.payload,
            reward=self._calculate_reward(event),
            metadata={
                "priority": event.priority.name,
                "timestamp": event.timestamp
            }
        )
        
        await self.stream.record(experience_event)
    
    def _calculate_reward(self, event: Event) -> float:
        """Вычислить reward на основе события"""
        # Простая эвристика
        if event.category == EventCategory.EVOLUTION:
            return 1.0
        elif event.priority == EventPriority.CRITICAL:
            return -1.0
        return 0.0
```

---

## 🎯 Шаг 6: Инициализация в main

### Обновить главный файл приложения

```python
# src/main.py или src/app.py

import asyncio
from core.events import EventHandler
from core.events.global_bus import start_event_bus, stop_event_bus, get_event_bus

async def initialize_neurograph_os():
    """Инициализация NeuroGraph OS с Event System"""
    
    print("Starting NeuroGraph OS...")
    
    # 1. Запускаем Event Bus
    event_bus = await start_event_bus()
    print("✅ Event Bus started")
    
    # 2. Устанавливаем глобальную шину для декораторов
    EventHandler.set_event_bus(event_bus)
    
    # 3. Создаем компоненты с event_bus
    token_factory = TokenFactory()
    token_service = TokenService(token_factory, event_bus)
    
    graph_engine = TokenGraph()
    graph_manager = GraphManager(graph_engine, event_bus)
    
    dna_spec = DNASpec.load("config/specs/dna_config.json")
    dna_guardian = DNAGuardian(dna_spec, event_bus)
    
    experience_stream = ExperienceStream()
    experience_integration = ExperienceEventIntegration(experience_stream)
    
    # 4. Регистрируем все обработчики событий
    EventHandler.register_all()
    print("✅ Event handlers registered")
    
    # 5. Возвращаем компоненты
    return {
        "event_bus": event_bus,
        "token_service": token_service,
        "graph_manager": graph_manager,
        "dna_guardian": dna_guardian,
        "experience_stream": experience_stream
    }

async def main():
    # Инициализация
    components = await initialize_neurograph_os()
    
    try:
        # Основная работа приложения
        print("NeuroGraph OS is running...")
        
        # Например, создаем токен
        token = await components["token_service"].create_token({
            "type": "data",
            "value": 42
        })
        print(f"Created token: {token.id}")
        
        # ... остальная логика ...
        
        # Держим приложение активным
        await asyncio.Event().wait()
        
    except KeyboardInterrupt:
        print("\nShutting down...")
    finally:
        # Graceful shutdown
        await stop_event_bus()
        print("✅ NeuroGraph OS stopped")

if __name__ == "__main__":
    asyncio.run(main())
```

---

## 📊 Шаг 7: Мониторинг и метрики

### Добавить endpoint для метрик (если используется FastAPI)

```python
from fastapi import APIRouter
from core.events.global_bus import get_event_bus

router = APIRouter()

@router.get("/metrics/events")
async def get_event_metrics():
    """Получить метрики Event Bus"""
    event_bus = get_event_bus()
    return event_bus.get_metrics()

@router.get("/metrics/subscriptions")
async def get_subscriptions_info():
    """Получить информацию о подписках"""
    event_bus = get_event_bus()
    return event_bus.get_subscriptions_info()
```

---

## ✅ Checklist интеграции

### Для каждого компонента:

- [ ] Наследовать от `EventEmitter`
- [ ] Добавить `event_bus` в конструктор
- [ ] Сделать методы `async` где нужно
- [ ] Добавить `await self.emit()` после изменений
- [ ] Использовать `@EventHandler.on()` для реакции на события
- [ ] Обновить тесты (добавить mock event_bus)
- [ ] Документировать генерируемые события

### Общие задачи:

- [ ] Создать `global_bus.py`
- [ ] Обновить `main.py` / `app.py`
- [ ] Добавить конфигурацию `event_bus.yaml`
- [ ] Написать integration tests
- [ ] Обновить документацию
- [ ] Добавить метрики и мониторинг

---

## 🐛 Типичные проблемы и решения

### Проблема 1: "EventBus not set"

**Причина**: Не установлена глобальная шина для декораторов

**Решение**:
```python
EventHandler.set_event_bus(event_bus)
EventHandler.register_all()  # Зарегистрировать отложенные обработчики
```

### Проблема 2: События не доставляются

**Причины**:
- EventBus не запущен (`await event_bus.start()`)
- Нет подписчиков на событие
- Фильтр блокирует событие

**Отладка**:
```python
# Включить логирование событий
event_bus = EventBus(log_events=True)

# Проверить подписки
info = event_bus.get_subscriptions_info()
print(info)
```

### Проблема 3: Медленная обработка

**Решение**: Проверить обработчики, вынести тяжелые операции в background tasks

```python
@EventHandler.on(EventType.TOKEN_CREATED)
async def handle_token(event: Event):
    # Быстрая обработка
    token_id = event.payload["token_id"]
    
    # Тяжелая работа в фоне
    asyncio.create_task(heavy_processing(token_id))
```

---

## 📚 Дополнительные ресурсы

- [Основная документация](./EVENTS.md)
- [Примеры использования](../examples/event_integration_example.py)
- [Тесты](../tests/core/events/test_event_system.py)
- [Конфигурация](../../config/core/event_bus.yaml)

---

## 🎓 Заключение

После завершения миграции вы получите:

✅ **Реактивную систему** — модули реагируют на изменения автоматически  
✅ **Слабую связность** — модули независимы друг от друга  
✅ **Расширяемость** — легко добавлять новое поведение  
✅ **Наблюдаемость** — полный контроль над потоком событий  

**Время миграции: 6-10 часов** (в зависимости от количества компонентов)

Удачной интеграции! 🚀
