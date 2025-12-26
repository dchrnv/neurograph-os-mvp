# SignalSystem Guide

Полное руководство по работе с Rust Core через Python bindings.

## Содержание

- [Обзор](#обзор)
- [API Reference](#api-reference)
- [Subscription Filters](#subscription-filters)
- [Pattern Matching](#pattern-matching)
- [Performance](#performance)
- [Advanced Usage](#advanced-usage)

---

## Обзор

**SignalSystem** — Rust-based event processing core с pattern matching, novelty detection и subscription system.

### Ключевые возможности

- ⚡ **Ultra-Fast Processing** - 304,553 events/sec, 0.39μs avg latency
- 🎯 **Pattern Matching** - Automatic pattern detection и neighbor finding
- 🆕 **Novelty Detection** - Identifies new vs seen patterns
- 📡 **Event Subscriptions** - Powerful filtering с JSON-based DSL
- 🔗 **Zero-Copy FFI** - PyO3 bindings без Python overhead

---

## API Reference

### SignalSystem

**Import:**

```python
import _core
```

**Creation:**

```python
system = _core.SignalSystem()
```

### emit()

Эмитит сигнал в систему для обработки.

```python
result = system.emit(
    event_type: str,
    vector: list[float],  # Должен быть длины 8
    priority: int = 128,
    **kwargs
) -> dict
```

**Parameters:**

- `event_type` — тип события (например, `"signal.input.text"`)
- `vector` — 8D вектор (обязательно 8 элементов)
- `priority` — приоритет 0-255 (default: 128)
- `**kwargs` — дополнительные поля:
  - `confidence` (int 0-255)
  - `urgency` (int 0-255)
  - `magnitude` (int)
  - `layers` (list[float] длины 8)

**Returns:**

```python
{
    "token_id": int,              # ID токена в Core
    "energy_delta": float,        # Изменение энергии
    "activation_spread": float,   # Распространение активации
    "is_novel": bool,             # True если паттерн новый
    "anomaly_score": float,       # Оценка аномальности
    "processing_time_us": float,  # Время обработки в μs
    "neighbors": [                # Похожие токены
        {
            "token_id": int,
            "distance": float,
            "resonance": float,
            "token_type": int,
            "layer_affinity": float
        },
        ...
    ],
    "triggered_actions": list,    # Triggered action IDs
    "from_core": True             # Маркер что это реальный Core
}
```

**Example:**

```python
result = system.emit(
    event_type="signal.input.text",
    vector=[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
    priority=200,
    confidence=255,
    urgency=180
)

print(f"Token: {result['token_id']}")
print(f"Novel: {result['is_novel']}")
print(f"Time: {result['processing_time_us']:.2f}μs")

if result['neighbors']:
    print(f"Neighbors: {len(result['neighbors'])}")
    for n in result['neighbors']:
        print(f"  - Token {n['token_id']}, distance: {n['distance']:.4f}")
```

### subscribe()

Подписывается на события по фильтру.

```python
subscriber_id = system.subscribe(
    name: str,
    filter_dict: dict,
    callback: callable = None
) -> int
```

**Parameters:**

- `name` — имя подписчика (для debugging)
- `filter_dict` — JSON фильтр (см. [Filters](#subscription-filters))
- `callback` — Python функция `def callback(event: dict):`

**Returns:** `subscriber_id` (int)

**Example:**

```python
def handler(event):
    print(f"Event: {event['event_type_id']}, Priority: {event['priority']}")

sub_id = system.subscribe(
    name="my_handler",
    filter_dict={
        "event_type": {"$wildcard": "signal.input.*"},
        "priority": {"$gte": 150}
    },
    callback=handler
)

# Emit matching event
system.emit(
    event_type="signal.input.text",
    vector=[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
    priority=200
)
# → handler вызывается
```

### unsubscribe()

Отписывается от событий.

```python
system.unsubscribe(subscriber_id: int)
```

**Example:**

```python
system.unsubscribe(sub_id)
```

### get_stats()

Возвращает статистику работы Core.

```python
stats = system.get_stats() -> dict
```

**Returns:**

```python
{
    "total_events": int,                  # Всего событий
    "avg_processing_time_us": float,      # Среднее время обработки
    "subscriber_notifications": int,      # Уведомлений подписчикам
    "filter_matches": int,                # Совпадений фильтров
    "filter_misses": int,                 # Несовпадений фильтров
    "events_by_type": {                   # По типам событий
        1: 100,
        2: 50,
        ...
    }
}
```

**Example:**

```python
stats = system.get_stats()

print(f"Total events: {stats['total_events']}")
print(f"Avg time: {stats['avg_processing_time_us']:.2f}μs")
print(f"Notifications: {stats['subscriber_notifications']}")

# Match ratio
if stats['filter_matches'] + stats['filter_misses'] > 0:
    ratio = stats['filter_matches'] / (stats['filter_matches'] + stats['filter_misses'])
    print(f"Filter match ratio: {ratio:.1%}")
```

### reset_stats()

Сбрасывает статистику.

```python
system.reset_stats()
```

### subscriber_count()

Возвращает количество активных подписчиков.

```python
count = system.subscriber_count() -> int
```

---

## Subscription Filters

Фильтры используют JSON-based DSL для гибкой подписки на события.

### Operators

#### $eq - Equals

```python
{
    "priority": {"$eq": 200}
}
```

Эквивалентно: `event.priority == 200`

#### $ne - Not Equals

```python
{
    "priority": {"$ne": 100}
}
```

Эквивалентно: `event.priority != 100`

#### $gt - Greater Than

```python
{
    "priority": {"$gt": 150}
}
```

Эквивалентно: `event.priority > 150`

#### $gte - Greater Than or Equal

```python
{
    "priority": {"$gte": 150}
}
```

Эквивалентно: `event.priority >= 150`

#### $lt - Less Than

```python
{
    "priority": {"$lt": 100}
}
```

Эквивалентно: `event.priority < 100`

#### $lte - Less Than or Equal

```python
{
    "priority": {"$lte": 100}
}
```

Эквивалентно: `event.priority <= 100`

#### $in - In List

```python
{
    "priority": {"$in": [100, 150, 200]}
}
```

Эквивалентно: `event.priority in [100, 150, 200]`

#### $wildcard - Wildcard Match

```python
{
    "event_type": {"$wildcard": "signal.input.*"}
}
```

Matches: `signal.input.text`, `signal.input.voice`, и т.д.

### Combining Filters

#### AND (implicit)

```python
{
    "priority": {"$gte": 150},
    "confidence": {"$gte": 200}
}
```

Эквивалентно: `priority >= 150 AND confidence >= 200`

#### Multiple Conditions

```python
{
    "event_type": {"$wildcard": "signal.*"},
    "priority": {"$gte": 150, "$lte": 250},
    "confidence": {"$in": [200, 255]}
}
```

### Examples

**High priority events:**

```python
filter = {
    "priority": {"$gte": 200}
}
```

**Specific event types:**

```python
filter = {
    "event_type": {"$wildcard": "signal.input.*"}
}
```

**Priority range:**

```python
filter = {
    "priority": {"$gte": 100, "$lte": 200}
}
```

**Complex filter:**

```python
filter = {
    "event_type": {"$wildcard": "signal.*"},
    "priority": {"$gte": 150},
    "confidence": {"$gte": 128},
    "urgency": {"$in": [200, 255]}
}
```

---

## Pattern Matching

SignalSystem автоматически находит похожие паттерны (neighbors) при обработке событий.

### How It Works

1. Event эмитится с 8D вектором
2. Core ищет ближайших соседей в семантическом пространстве
3. Возвращает список neighbors с расстояниями

### Neighbor Structure

```python
{
    "token_id": 42,           # ID соседнего токена
    "distance": 0.123,        # Euclidean distance
    "resonance": 0.877,       # Similarity score (1 - distance)
    "token_type": 1,          # Тип токена
    "layer_affinity": 0.95    # Affinity к слою
}
```

### Example

```python
# Emit first event
result1 = system.emit(
    event_type="signal.input.text",
    vector=[0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
    priority=200
)

print(f"Novel: {result1['is_novel']}")  # True
print(f"Neighbors: {len(result1['neighbors'])}")  # 0

# Emit similar event
result2 = system.emit(
    event_type="signal.input.text",
    vector=[0.51, 0.49, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
    priority=200
)

print(f"Novel: {result2['is_novel']}")  # False (similar to result1)
print(f"Neighbors: {len(result2['neighbors'])}")  # 1

neighbor = result2['neighbors'][0]
print(f"Neighbor token: {neighbor['token_id']}")  # token_id from result1
print(f"Distance: {neighbor['distance']:.4f}")  # ~0.02
print(f"Resonance: {neighbor['resonance']:.4f}")  # ~0.98
```

### Novelty Detection

```python
def process_event(vector):
    result = system.emit(
        event_type="signal.input",
        vector=vector,
        priority=200
    )

    if result['is_novel']:
        print("🆕 New pattern detected!")
    else:
        print(f"🔗 Similar to {len(result['neighbors'])} patterns")

        # Find closest
        if result['neighbors']:
            closest = min(result['neighbors'], key=lambda n: n['distance'])
            print(f"   Closest: Token {closest['token_id']}, distance: {closest['distance']:.4f}")
```

---

## Performance

### Benchmarks

**Core only:**
- **Throughput**: 304,553 events/sec
- **Latency**: 0.39μs average
- **99th percentile**: <1μs

**Full pipeline (Gateway + Core + ActionController):**
- **Throughput**: 5,601 messages/sec
- **Latency**: 0.18ms total
- **Core overhead**: +0.02ms (+12%)

### Optimization Tips

**1. Batch processing:**

```python
results = []
for vector in vectors:
    result = system.emit(
        event_type="signal.batch",
        vector=vector,
        priority=128
    )
    results.append(result)
```

**2. Use appropriate priorities:**

```python
# High priority (more processing)
system.emit(..., priority=250)

# Normal priority (balanced)
system.emit(..., priority=128)

# Low priority (minimal processing)
system.emit(..., priority=50)
```

**3. Filter subscriptions carefully:**

```python
# Bad: Too broad
filter = {"priority": {"$gte": 0}}  # Matches everything

# Good: Specific
filter = {
    "event_type": {"$wildcard": "signal.input.*"},
    "priority": {"$gte": 200}
}
```

---

## Advanced Usage

### Multi-threaded Processing

```python
import threading

def worker(system, vectors):
    for vec in vectors:
        result = system.emit(
            event_type="signal.worker",
            vector=vec,
            priority=200
        )

# SignalSystem is thread-safe
system = _core.SignalSystem()

threads = []
for i in range(4):
    t = threading.Thread(target=worker, args=(system, vectors_chunk[i]))
    threads.append(t)
    t.start()

for t in threads:
    t.join()

print(f"Total events: {system.get_stats()['total_events']}")
```

### Custom Event Types

```python
# Register custom types
system.emit(event_type="app.user.login", vector=vec, priority=200)
system.emit(event_type="app.user.logout", vector=vec, priority=150)
system.emit(event_type="app.api.request", vector=vec, priority=180)

# Subscribe to custom types
system.subscribe(
    name="user_tracker",
    filter_dict={"event_type": {"$wildcard": "app.user.*"}},
    callback=track_user_event
)

system.subscribe(
    name="api_tracker",
    filter_dict={"event_type": {"$wildcard": "app.api.*"}},
    callback=track_api_event
)
```

### Integration with Gateway

```python
from src.gateway import SignalGateway
import _core

# Create Core
core = _core.SignalSystem()

# Create Gateway
gateway = SignalGateway()
gateway.initialize()

# Process through Gateway → Core
event = gateway.push_text("Hello!")

# Manual Core processing
result = core.emit(
    event_type=event.event_type,
    vector=list(event.semantic.vector),
    priority=event.routing.priority
)

print(f"Gateway event: {event.event_id}")
print(f"Core token: {result['token_id']}")
print(f"Novel: {result['is_novel']}")
```

---

## See Also

- [Gateway Guide](GATEWAY_GUIDE.md) - Sensory interface
- [ActionController Guide](ACTION_CONTROLLER_GUIDE.md) - Response generation
- [Getting Started](GETTING_STARTED.md) - Quick start guide
- [CHANGELOG v0.57.0](../changelogs/CHANGELOG_v0.57.0.md) - Full integration details
