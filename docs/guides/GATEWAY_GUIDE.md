# Gateway v2.0 Guide

Полное руководство по работе с сенсорным слоем NeuroGraph OS.

## Содержание

- [Обзор](#обзор)
- [Архитектура](#архитектура)
- [API Reference](#api-reference)
- [Encoders](#encoders)
- [Sensors](#sensors)
- [Advanced Usage](#advanced-usage)

---

## Обзор

**Gateway v2.0** — сенсорный интерфейс NeuroGraph OS, преобразующий внешние сигналы (текст, метрики, события) в унифицированные 8D векторы для обработки в Rust Core.

### Ключевые возможности

- ✅ **8D Semantic Encoding** - Унифицированное представление всех входов
- ✅ **Built-in Sensors** - 4 встроенных сенсора (text, system, user_input, external)
- ✅ **4 Encoders** - PASSTHROUGH, TEXT_TFIDF, NUMERIC_DIRECT, SENTIMENT_SIMPLE
- ✅ **Pydantic Models** - Типобезопасность и валидация
- ✅ **Sequence Tracking** - Поддержка conversation/session ID
- ✅ **NeuroTick** - Монотонный счётчик для темпоральной синхронизации

---

## Архитектура

```
Input
  ↓
SignalGateway.push_*()
  ↓
SensorRegistry.get_sensor() → Sensor config
  ↓
EncoderFactory.get_encoder() → Encoder instance
  ↓
encoder.encode(raw_data) → 8D vector
  ↓
SignalEvent (Pydantic model)
  ├─ event_id: UUID
  ├─ semantic: SemanticData (8D vector)
  ├─ routing: RoutingData (priority, source, destination)
  ├─ temporal: TemporalData (neuro_tick, timestamp)
  └─ context: ContextData (sequence_id, metadata)
  ↓
Output: SignalEvent ready for Core
```

---

## API Reference

### SignalGateway

**Инициализация:**

```python
from src.gateway import SignalGateway

gateway = SignalGateway()
gateway.initialize()  # Регистрирует built-in sensors
```

**Push methods:**

#### push_text()

```python
event = gateway.push_text(
    text: str,
    priority: int = 128,
    sequence_id: Optional[str] = None,
    metadata: Optional[Dict] = None
) -> SignalEvent
```

**Пример:**

```python
event = gateway.push_text(
    text="Hello, NeuroGraph!",
    priority=200,
    sequence_id="conversation_123"
)

print(f"Event ID: {event.event_id}")
print(f"Vector: {event.semantic.vector}")
print(f"Encoder: {event.semantic.encoder_used}")  # TEXT_TFIDF
```

#### push_system()

```python
event = gateway.push_system(
    metric_name: str,
    metric_value: float,
    priority: int = 100,
    metadata: Optional[Dict] = None
) -> SignalEvent
```

**Пример:**

```python
event = gateway.push_system(
    metric_name="cpu_percent",
    metric_value=45.7,
    priority=150
)

print(f"Vector: {event.semantic.vector}")
print(f"Encoder: {event.semantic.encoder_used}")  # NUMERIC_DIRECT
```

#### push_raw()

```python
event = gateway.push_raw(
    sensor_id: str,
    raw_data: Any,
    priority: int = 128,
    sequence_id: Optional[str] = None,
    metadata: Optional[Dict] = None
) -> SignalEvent
```

**Пример:**

```python
# Регистрируем custom sensor
gateway.register_sensor(
    sensor_id="custom.sentiment",
    sensor_type="sentiment_feed",
    domain="external",
    modality="text",
    encoder_type=EncoderType.SENTIMENT_SIMPLE
)

# Используем
event = gateway.push_raw(
    sensor_id="custom.sentiment",
    raw_data="I love this product!",
    priority=180
)

print(f"Encoder: {event.semantic.encoder_used}")  # SENTIMENT_SIMPLE
```

**Statistics:**

```python
stats = gateway.get_stats()

print(f"Total events: {stats['total_events']}")
print(f"NeuroTick: {stats['neuro_tick']}")
print(f"Sensors: {stats['registered_sensors']}")
print(f"By encoder: {stats['by_encoder']}")
```

---

## Encoders

Gateway v2.0 включает 4 встроенных encoder'а.

### 1. PASSTHROUGH

**Назначение:** Прямая передача 8D вектора без преобразований.

**Использование:**

```python
from src.gateway import EncoderType

gateway.register_sensor(
    sensor_id="custom.direct",
    sensor_type="direct_vector",
    domain="external",
    modality="vector",
    encoder_type=EncoderType.PASSTHROUGH
)

event = gateway.push_raw(
    sensor_id="custom.direct",
    raw_data=[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
    priority=200
)

# Vector не изменён
assert event.semantic.vector == [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8]
```

**Validation:**
- Проверяет, что input — это list из 8 float
- Значения могут быть любыми (не обязательно [0, 1])

### 2. TEXT_TFIDF

**Назначение:** TF-IDF based text encoding с hash bucketing.

**Алгоритм:**

1. Токенизация текста (lowercase + split)
2. TF-IDF для каждого слова: `tf * log(1 / df)`
3. Hash bucketing: `hash(word) % 8` → dimension
4. Суммирование по dimensions
5. L2 normalization

**Использование:**

```python
event = gateway.push_text(
    text="machine learning neural networks",
    priority=200
)

# Автоматически использует TEXT_TFIDF
print(f"Vector: {event.semantic.vector}")  # Normalized 8D vector
```

**Особенности:**

- **Stopwords**: Игнорируются (a, the, is, and, ...)
- **DF tracking**: Считается document frequency для каждого слова
- **Normalization**: L2 norm = 1.0
- **Hash collisions**: Возможны, но редки для 8 dimensions

**Пример:**

```python
# Первый текст
event1 = gateway.push_text("cat dog bird")
vec1 = event1.semantic.vector

# Второй текст (похожий)
event2 = gateway.push_text("cat dog fish")
vec2 = event2.semantic.vector

# Similarity (cosine)
import numpy as np
similarity = np.dot(vec1, vec2)
print(f"Similarity: {similarity:.4f}")  # ~0.8-0.9
```

### 3. NUMERIC_DIRECT

**Назначение:** Простое масштабирование числовых метрик.

**Алгоритм:**

1. Нормализация значения: `value / 100.0`
2. Распределение по dimensions (первая dimension получает значение)
3. Остальные dimensions = 0.0

**Использование:**

```python
event = gateway.push_system(
    metric_name="cpu_percent",
    metric_value=75.5
)

# vector[0] = 0.755, остальные = 0
print(f"Vector: {event.semantic.vector}")
# [0.755, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
```

**Примечание:** Encoder очень простой, подходит для базовых метрик.

### 4. SENTIMENT_SIMPLE

**Назначение:** Sentiment analysis для текста.

**Алгоритм:**

1. Подсчёт positive/negative слов
2. Вычисление polarity: `(pos - neg) / total`
3. Mapping на 8D dimensions:
   - `dim[0]` = polarity ([-1, 1] → [0, 1])
   - `dim[1]` = magnitude (0-1, насколько сильный sentiment)
   - `dim[2-7]` = эмоции (joy, anger, sadness, fear, ...)

**Использование:**

```python
gateway.register_sensor(
    sensor_id="sentiment",
    sensor_type="sentiment",
    domain="external",
    modality="text",
    encoder_type=EncoderType.SENTIMENT_SIMPLE
)

event = gateway.push_raw(
    sensor_id="sentiment",
    raw_data="I absolutely love this! Amazing!",
    priority=180
)

vec = event.semantic.vector
print(f"Polarity: {vec[0]:.2f}")    # ~0.8-1.0 (positive)
print(f"Magnitude: {vec[1]:.2f}")   # ~0.8-1.0 (strong)
```

**Словари:**

- Positive: `["good", "love", "amazing", "great", "excellent", ...]`
- Negative: `["bad", "hate", "terrible", "awful", "horrible", ...]`

**Ограничения:** Очень простой словарный подход, не учитывает контекст.

---

## Sensors

### Built-in Sensors

Gateway v2.0 регистрирует 4 встроенных сенсора при `initialize()`:

#### 1. gateway.input.text

```python
{
    "sensor_id": "gateway.input.text",
    "sensor_type": "text_input",
    "domain": "internal",
    "modality": "text",
    "encoder_type": EncoderType.TEXT_TFIDF
}
```

**Использование:** `gateway.push_text()`

#### 2. gateway.system.metrics

```python
{
    "sensor_id": "gateway.system.metrics",
    "sensor_type": "system_metrics",
    "domain": "internal",
    "modality": "numeric",
    "encoder_type": EncoderType.NUMERIC_DIRECT
}
```

**Использование:** `gateway.push_system()`

#### 3. gateway.user.input

```python
{
    "sensor_id": "gateway.user.input",
    "sensor_type": "user_interaction",
    "domain": "external",
    "modality": "text",
    "encoder_type": EncoderType.TEXT_TFIDF
}
```

**Использование:** Для user input с conversation tracking

#### 4. gateway.external.feed

```python
{
    "sensor_id": "gateway.external.feed",
    "sensor_type": "external_feed",
    "domain": "external",
    "modality": "text",
    "encoder_type": EncoderType.PASSTHROUGH
}
```

**Использование:** Для внешних источников данных

### Custom Sensors

**Регистрация:**

```python
from src.gateway import EncoderType

gateway.register_sensor(
    sensor_id="custom.api",
    sensor_type="api_response",
    domain="external",
    modality="json",
    encoder_type=EncoderType.PASSTHROUGH
)
```

**Использование:**

```python
event = gateway.push_raw(
    sensor_id="custom.api",
    raw_data=preprocessed_vector,  # Уже 8D
    priority=150
)
```

---

## Advanced Usage

### Conversation Tracking

```python
# User says something
event1 = gateway.push_text(
    text="What's the weather?",
    sequence_id="conv_123",
    priority=200
)

# Bot responds
event2 = gateway.push_text(
    text="It's sunny today",
    sequence_id="conv_123",
    priority=150
)

# User continues
event3 = gateway.push_text(
    text="How about tomorrow?",
    sequence_id="conv_123",
    priority=200
)

# Все события связаны через sequence_id
assert event1.context.sequence_id == event2.context.sequence_id == event3.context.sequence_id
```

### Metadata

```python
event = gateway.push_text(
    text="Important message",
    priority=250,
    metadata={
        "user_id": "user_123",
        "chat_id": "chat_456",
        "platform": "telegram",
        "language": "en"
    }
)

print(f"Metadata: {event.context.metadata}")
```

### Export to JSON

```python
event = gateway.push_text("Hello!")

# Pydantic model_dump_json()
json_str = event.model_dump_json(indent=2)

print(json_str)
# {
#   "event_id": "...",
#   "semantic": {
#     "vector": [0.1, 0.2, ...],
#     "encoder_used": "TEXT_TFIDF"
#   },
#   ...
# }
```

### Load from JSON

```python
from src.gateway.models import SignalEvent
import json

# Parse
data = json.loads(json_str)
event = SignalEvent(**data)

print(f"Loaded: {event.event_id}")
```

---

## See Also

- [SignalSystem Guide](SIGNAL_SYSTEM_GUIDE.md) - Rust Core integration
- [ActionController Guide](ACTION_CONTROLLER_GUIDE.md) - Response generation
- [Getting Started](GETTING_STARTED.md) - Quick start guide
