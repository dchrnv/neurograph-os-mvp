# Gateway v2.0 - User Guide & Cheat Sheet

> **Практическое руководство по использованию Gateway v2.0**
>
> Версия: v0.54.0+
> Дата: 2025-12-22

---

## 📚 Содержание

1. [Быстрый старт](#быстрый-старт)
2. [Основные концепции](#основные-концепции)
3. [SignalGateway API](#signalgateway-api)
4. [Encoders (Энкодеры)](#encoders-энкодеры)
5. [Subscription Filters](#subscription-filters)
6. [Примеры использования](#примеры-использования)
7. [Troubleshooting](#troubleshooting)

---

## Быстрый старт

### Установка

Gateway v2.0 - это чистый Python, не требует компиляции:

```bash
cd /path/to/neurograph-os-mvp

# Просто импортируйте
python
>>> from src.gateway import SignalGateway
```

### Hello World

```python
from src.gateway import SignalGateway

# 1. Создать Gateway
gateway = SignalGateway()
gateway.initialize()  # Регистрирует встроенные сенсоры

# 2. Отправить текстовое сообщение
event = gateway.push_text(
    text="Hello, NeuroGraph!",
    priority=200
)

# 3. Проверить результат
print(f"Event ID: {event.event_id}")
print(f"8D Vector: {event.semantic.vector}")
print(f"NeuroTick: {event.temporal.neuro_tick}")
```

**Вывод:**
```
Event ID: evt_550e8400-e29b-41d4-a716-446655440000
8D Vector: [1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5]
NeuroTick: 1
```

---

## Основные концепции

### SignalEvent - Единый формат события

Все события в системе имеют одинаковую структуру:

```
SignalEvent
├── event_id: str              # UUID события
├── event_type: str            # "signal.input.external.text.chat"
├── source: SignalSource       # Откуда пришёл сигнал
├── semantic: SemanticCore     # 8D вектор
├── energy: EnergyProfile      # Интенсивность, эмоции
├── temporal: TemporalBinding  # Время, NeuroTick, sequence
├── payload: RawPayload        # Исходные данные
├── result: ProcessingResult   # Результат из Core (опционально)
└── routing: RoutingInfo       # Приоритет, теги, TTL
```

### Сенсоры (Sensors)

Сенсор = источник данных + энкодер для преобразования в 8D.

**Встроенные сенсоры:**

| Sensor ID              | Тип           | Модальность | Энкодер        | Использование         |
|------------------------|---------------|-------------|----------------|-----------------------|
| `builtin.text_chat`    | text_chat     | text        | TEXT_TFIDF     | Текстовые сообщения   |
| `builtin.system_monitor` | system_monitor | numeric    | NUMERIC_DIRECT | Системные метрики     |
| `builtin.timer`        | timer         | numeric     | PASSTHROUGH    | Периодические события |

### NeuroTick

Монотонный счётчик событий в Gateway. Каждый `push_*()` увеличивает tick на 1.

```python
event1 = gateway.push_text("First")   # neuro_tick = 1
event2 = gateway.push_text("Second")  # neuro_tick = 2
event3 = gateway.push_text("Third")   # neuro_tick = 3
```

### Sequence ID

Для отслеживания диалогов/сессий:

```python
# Все сообщения в одном диалоге
gateway.push_text("Hello", sequence_id="conv_001")
gateway.push_text("How are you?", sequence_id="conv_001")
gateway.push_text("Goodbye", sequence_id="conv_001")

# Фильтр для этого диалога
filter = SubscriptionFilter({
    "temporal.sequence_id": "conv_001"
})
```

---

## SignalGateway API

### Инициализация

```python
from src.gateway import SignalGateway

# Без подключения к Core
gateway = SignalGateway()
gateway.initialize()

# С подключением к Rust Core (будущее)
import _core
core_system = _core.SignalSystem()
gateway = SignalGateway(core_system=core_system)
gateway.initialize()
```

### Push Methods

#### push_text()

Отправка текстового сообщения:

```python
event = gateway.push_text(
    text="User message here",
    sensor_id="builtin.text_chat",  # По умолчанию
    priority=200,                    # 0-255
    metadata={"user_id": "123"},     # Опциональные метаданные
    sequence_id="conv_abc"           # Для диалогов
)
```

**Параметры:**
- `text` (str) - текст сообщения
- `sensor_id` (str) - ID сенсора (default: "builtin.text_chat")
- `priority` (int) - приоритет 0-255 (default: 200)
- `metadata` (dict) - дополнительные данные
- `sequence_id` (str) - ID последовательности/диалога

#### push_system()

Отправка системной метрики:

```python
event = gateway.push_system(
    metric_name="cpu_percent",
    metric_value=45.7,
    sensor_id="builtin.system_monitor",  # По умолчанию
    priority=100,
    metadata={"host": "localhost"}
)
```

#### push_audio() / push_vision()

```python
# Аудио (будущее)
event = gateway.push_audio(
    audio_data=b"...",
    sensor_id="custom.microphone",
    sample_rate=16000,
    priority=180
)

# Изображения (будущее)
event = gateway.push_vision(
    image_data=b"...",
    sensor_id="custom.camera",
    width=640,
    height=480,
    priority=150
)
```

### Sensor Management

#### Регистрация кастомного сенсора

```python
from src.gateway import EncoderType

gateway.register_sensor(
    sensor_id="custom.weather_api",
    sensor_type="weather_feed",
    domain="external",           # external | internal | system
    modality="json",             # text | audio | vision | numeric | json
    encoder_type=EncoderType.NUMERIC_DIRECT,
    description="Weather API data",
    default_priority=120,
    metadata={"api_version": "v2"}
)

# Использование
event = gateway.push_system(
    metric_name="temperature",
    metric_value=22.5,
    sensor_id="custom.weather_api"
)
```

#### Управление сенсорами

```python
# Список сенсоров
sensors = gateway.list_sensors()
for s in sensors:
    print(f"{s.sensor_id}: {s.sensor_type}")

# Отключить сенсор
gateway.registry.disable_sensor("builtin.timer")

# Включить обратно
gateway.registry.enable_sensor("builtin.timer")

# Удалить сенсор
gateway.unregister_sensor("custom.weather_api")
```

### Statistics

```python
stats = gateway.get_stats()
print(stats)
# {
#   "total_events": 42,
#   "neuro_tick": 42,
#   "registered_sensors": 3,
#   "enabled_sensors": 3
# }
```

---

## Encoders (Энкодеры)

### PASSTHROUGH

Прямая передача готового 8D вектора (для отладки):

```python
from src.gateway.encoders import PassthroughEncoder

encoder = PassthroughEncoder()
vector = encoder.encode([0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8])
# → [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] (normalized to [0,1])
```

### NUMERIC_DIRECT

Масштабирование чисел в 8D:

```python
from src.gateway.encoders import NumericDirectEncoder

encoder = NumericDirectEncoder(scale_factor=100.0)

# Одно число
vector = encoder.encode(45.7)
# → [0.457, 0, 0, 0, 0, 0, 0, 0]

# Список чисел
vector = encoder.encode([10.0, 20.0, 30.0])
# → [0.1, 0.2, 0.3, 0, 0, 0, 0, 0]

# Dict
vector = encoder.encode({"cpu": 45.7, "mem": 67.3})
# → [0.457, 0.673, 0, 0, 0, 0, 0, 0]  (sorted by key)
```

### TEXT_TFIDF

TF-IDF с хэш-бакетингом:

```python
from src.gateway.encoders import TextTfidfEncoder

encoder = TextTfidfEncoder()
vector = encoder.encode("Hello, NeuroGraph!")
# → [1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5]
```

**Как работает:**
1. Токенизация (lowercase, alphanumeric)
2. Фильтрация stopwords (a, the, is, etc.)
3. TF (term frequency) для каждого токена
4. Хэш токена → dimension (0-7)
5. Суммирование TF в каждой dimension
6. Нормализация в [0, 1]

### SENTIMENT_SIMPLE

Sentiment analysis с эмоциями:

```python
from src.gateway.encoders import SentimentSimpleEncoder

encoder = SentimentSimpleEncoder()
vector = encoder.encode("I am very happy today!")
# → [1.0, 0.71, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0]
#    ^     ^     ^    ^
#    |     |     |    joy
#    |     |     intensity
#    |     subjectivity
#    polarity (positive)
```

**8D структура:**
- **Dim 0**: Polarity (0=negative, 1=positive, 0.5=neutral)
- **Dim 1**: Subjectivity (0=objective, 1=subjective)
- **Dim 2**: Intensity (0=mild, 1=strong)
- **Dim 3**: Joy emotion
- **Dim 4**: Sadness emotion
- **Dim 5**: Anger emotion
- **Dim 6**: Fear emotion
- **Dim 7**: Surprise emotion

---

## Subscription Filters

> **NEW in v0.55.0** - MongoDB-style фильтры для событий

### Основы

```python
from src.gateway.filters import SubscriptionFilter

# Простое равенство
filter = SubscriptionFilter({
    "source.domain": "external"
})

# Проверка
if filter.matches(event):
    print("Event matched!")
```

### Операторы сравнения

```python
# $eq (равно)
{"routing.priority": {"$eq": 200}}

# $ne (не равно)
{"routing.priority": {"$ne": 100}}

# $gt, $gte, $lt, $lte
{"routing.priority": {"$gte": 150}}
{"energy.urgency": {"$lt": 0.5}}
```

### Операторы коллекций

```python
# $in (в списке)
{"routing.priority": {"$in": [100, 150, 200]}}

# $nin (не в списке)
{"source.domain": {"$nin": ["internal", "system"]}}

# $contains (содержит элемент)
{"routing.tags": {"$contains": "telegram"}}
```

### Операторы паттернов

```python
# $wildcard (Unix-style wildcards)
{"event_type": {"$wildcard": "signal.input.*"}}
{"event_type": {"$wildcard": "signal.input.*.text.*"}}

# $regex (регулярные выражения)
{"event_type": {"$regex": r"^signal\.input\.external\..*$"}}
{"source.sensor_id": {"$regex": r"telegram_\d+"}}
```

### Логические операторы

```python
# $and (все условия)
{
    "$and": [
        {"source.domain": "external"},
        {"routing.priority": {"$gte": 150}}
    ]
}

# $or (хотя бы одно)
{
    "$or": [
        {"source.domain": "external"},
        {"source.domain": "internal"}
    ]
}

# $not (отрицание)
{"$not": {"source.domain": "system"}}

# Комбинация
{
    "$and": [
        {
            "$or": [
                {"source.domain": "external"},
                {"source.domain": "internal"}
            ]
        },
        {"routing.priority": {"$gte": 150}},
        {"routing.tags": {"$contains": "important"}}
    ]
}
```

### Готовые фильтры

```python
from src.gateway.filters.examples import (
    telegram_user_messages_filter,
    telegram_high_priority_filter,
    dashboard_all_events_filter,
    action_selector_novel_signals_filter,
    tag_contains_filter,
)

# Telegram messages
filter1 = telegram_user_messages_filter()
# Matches: external text, text_chat sensor, priority >= 150

# High priority
filter2 = telegram_high_priority_filter()
# Matches: text_chat, priority >= 200, urgency >= 0.7

# Dashboard - все события
filter3 = dashboard_all_events_filter()
# Matches: signal.input.*

# По тегу
filter4 = tag_contains_filter("urgent")
# Matches: routing.tags contains "urgent"
```

---

## Примеры использования

### Пример 1: Простая обработка текста

```python
from src.gateway import SignalGateway

gateway = SignalGateway()
gateway.initialize()

# Отправить сообщение
event = gateway.push_text("User question here", priority=200)

# Извлечь данные
print(f"Vector: {event.semantic.vector}")
print(f"Urgency: {event.energy.urgency}")
print(f"Tick: {event.temporal.neuro_tick}")
```

### Пример 2: Sentiment Analysis

```python
from src.gateway import SignalGateway, EncoderType

gateway = SignalGateway()
gateway.initialize()

# Зарегистрировать sentiment сенсор
gateway.register_sensor(
    sensor_id="sentiment.analyzer",
    sensor_type="sentiment",
    domain="external",
    modality="text",
    encoder_type=EncoderType.SENTIMENT_SIMPLE
)

# Анализировать сообщения
messages = [
    "I love this product!",
    "This is terrible",
    "The system works fine"
]

for msg in messages:
    event = gateway._push_signal(
        data=msg,
        data_type="text",
        sensor_id="sentiment.analyzer",
        priority=180
    )

    polarity = event.semantic.vector[0]
    joy = event.semantic.vector[3]
    sadness = event.semantic.vector[4]

    sentiment = "positive" if polarity > 0.6 else "negative" if polarity < 0.4 else "neutral"
    print(f'"{msg}" → {sentiment} (polarity={polarity:.2f})')
```

### Пример 3: Системный мониторинг

```python
import psutil
from src.gateway import SignalGateway

gateway = SignalGateway()
gateway.initialize()

# Отправлять метрики каждые N секунд
def send_metrics():
    cpu = psutil.cpu_percent()
    mem = psutil.virtual_memory().percent

    gateway.push_system("cpu_percent", cpu, priority=100)
    gateway.push_system("memory_percent", mem, priority=100)

    # Высокий приоритет при перегрузке
    if cpu > 80:
        gateway.push_system("cpu_alert", cpu, priority=250)

# В цикле или по таймеру
send_metrics()
```

### Пример 4: Фильтрация событий

```python
from src.gateway import SignalGateway
from src.gateway.filters import SubscriptionFilter

gateway = SignalGateway()
gateway.initialize()

# Создать фильтр для важных сообщений
important_filter = SubscriptionFilter({
    "$and": [
        {"event_type": {"$wildcard": "signal.input.external.*"}},
        {"routing.priority": {"$gte": 180}},
        {"routing.tags": {"$contains": "urgent"}}
    ]
})

# Обработать события
events = []
events.append(gateway.push_text("Normal message", priority=150))
events.append(gateway.push_text("Urgent!", priority=200, metadata={"tags": ["urgent"]}))

# Фильтрация
for event in events:
    # Add tag to routing.tags for the second event
    if "tags" in event.payload.metadata:
        event.routing.tags.extend(event.payload.metadata["tags"])

    if important_filter.matches(event):
        print(f"IMPORTANT: {event.payload.data}")
    else:
        print(f"Normal: {event.payload.data}")
```

### Пример 5: Диалог с sequence tracking

```python
from src.gateway import SignalGateway
from src.gateway.filters import SubscriptionFilter

gateway = SignalGateway()
gateway.initialize()

conversation_id = "conv_user_123"

# Отправить реплики диалога
gateway.push_text("Hello!", sequence_id=conversation_id, priority=200)
gateway.push_text("How can I help?", sequence_id=conversation_id, priority=200)
gateway.push_text("I need support", sequence_id=conversation_id, priority=200)

# Фильтр для этого диалога
conv_filter = SubscriptionFilter({
    "temporal.sequence_id": conversation_id
})

# Статистика диалога
stats = gateway.get_stats()
print(f"Total events: {stats['total_events']}")
print(f"Conversation events: {conversation_id}")
```

### Пример 6: JSON Export для API

```python
from src.gateway import SignalGateway
import json

gateway = SignalGateway()
gateway.initialize()

event = gateway.push_text("API test message", priority=180)

# Экспорт в JSON
json_str = event.model_dump_json(indent=2)
print(json_str[:500])  # First 500 chars

# Сохранить в файл
with open("event.json", "w") as f:
    f.write(json_str)

# Загрузить обратно
from src.gateway.models import SignalEvent
with open("event.json", "r") as f:
    restored_event = SignalEvent.model_validate_json(f.read())

print(f"Restored: {restored_event.event_id}")
```

---

## Troubleshooting

### Проблема: ModuleNotFoundError

```python
ModuleNotFoundError: No module named 'src'
```

**Решение:**
```bash
# Установить PYTHONPATH
export PYTHONPATH=/path/to/neurograph-os-mvp

# Или запускать из корня проекта
cd /path/to/neurograph-os-mvp
python your_script.py
```

### Проблема: Sensor not found

```python
ValueError: Sensor 'custom.my_sensor' not found in registry
```

**Решение:**
```python
# Проверить зарегистрированные сенсоры
sensors = gateway.list_sensors()
print([s.sensor_id for s in sensors])

# Зарегистрировать сенсор перед использованием
gateway.register_sensor(
    sensor_id="custom.my_sensor",
    sensor_type="custom",
    domain="external",
    modality="text",
    encoder_type=EncoderType.TEXT_TFIDF
)
```

### Проблема: Sensor disabled

```python
ValueError: Sensor 'builtin.timer' is disabled
```

**Решение:**
```python
# Включить сенсор
gateway.registry.enable_sensor("builtin.timer")
```

### Проблема: Invalid vector length

```python
ValidationError: vector must have exactly 8 elements
```

**Решение:**
```python
# При использовании PASSTHROUGH убедиться что вектор 8D
encoder = PassthroughEncoder()
vector = encoder.encode([0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8])
# Не [0.1, 0.2, 0.3] - это вызовет ошибку
```

### Проблема: Filter not matching

```python
# Фильтр не срабатывает
filter = SubscriptionFilter({"routing.priority": 200})
assert filter.matches(event) == True  # Fails
```

**Решение:**
```python
# Проверить фактическое значение
print(f"Actual priority: {event.routing.priority}")

# Использовать правильный оператор
filter = SubscriptionFilter({"routing.priority": {"$eq": 200}})
# Или просто
filter = SubscriptionFilter({"routing.priority": 200})
```

---

## Чек-лист для начинающих

- [ ] Gateway инициализирован: `gateway.initialize()`
- [ ] Понятна структура SignalEvent (8 nested models)
- [ ] Знаю 4 энкодера: PASSTHROUGH, NUMERIC, TEXT_TFIDF, SENTIMENT
- [ ] Умею использовать `push_text()` и `push_system()`
- [ ] Понимаю NeuroTick (монотонный счётчик)
- [ ] Знаю про sequence_id для диалогов
- [ ] Умею создавать фильтры (SubscriptionFilter)
- [ ] Знаю основные операторы: $wildcard, $gte, $and, $contains
- [ ] Умею регистрировать кастомные сенсоры
- [ ] Знаю как экспортировать в JSON

---

## Быстрые шпаргалки

### SignalGateway API

```python
gateway = SignalGateway()
gateway.initialize()
gateway.push_text(text, priority=200, sequence_id=None)
gateway.push_system(metric_name, metric_value, priority=100)
gateway.register_sensor(sensor_id, sensor_type, domain, modality, encoder_type)
gateway.list_sensors()
gateway.get_stats()
```

### SubscriptionFilter

```python
# Равенство
{"field": value}

# Сравнение
{"field": {"$gte": 150}}

# Wildcard
{"event_type": {"$wildcard": "signal.input.*"}}

# Логика
{"$and": [cond1, cond2]}

# Коллекция
{"tags": {"$contains": "urgent"}}
```

### Encoders

```python
PASSTHROUGH      # [0.1, ..., 0.8] → [0.1, ..., 0.8]
NUMERIC_DIRECT   # 45.7 → [0.457, 0, ..., 0]
TEXT_TFIDF       # "Hello" → [1.0, 0, 0.5, ...]
SENTIMENT_SIMPLE # "Happy!" → [1.0, 0.7, 1.0, 1.0, ...]
```

---

**Полезные ссылки:**
- Спецификация: `docs/specs/Gateway_v2_0.md`
- CHANGELOG: `docs/changelogs/CHANGELOG_v0.54.0.md`
- Примеры: `examples/gateway_v2_demo.py`
- Master Plan: `docs/MASTER_PLAN_v2.1.md`
