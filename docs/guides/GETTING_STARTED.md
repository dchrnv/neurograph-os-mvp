# Getting Started with NeuroGraph

Полное руководство по началу работы с NeuroGraph OS v0.57.0.

## Содержание

- [Системные требования](#системные-требования)
- [Установка](#установка)
- [Первые шаги](#первые-шаги)
- [Telegram Bot](#telegram-bot)
- [Python API](#python-api)
- [REST API](#rest-api)
- [Примеры](#примеры)

---

## Системные требования

- **Python**: 3.8+
- **Rust**: 1.70+ (для сборки Core)
- **maturin**: для Python bindings
- **Docker**: опционально, для production deployment

---

## Установка

### 1. Клонирование репозитория

```bash
git clone https://github.com/dchrnv/neurograph-os-mvp.git
cd neurograph-os-mvp
```

### 2. Сборка Rust Core

```bash
# Установка maturin
pip install maturin

# Сборка Python bindings
cd src/core_rust
maturin develop --features python-bindings --release
cd ../..
```

**Проверка:**

```python
import _core
system = _core.SignalSystem()
print("✅ Rust Core loaded successfully")
```

### 3. Установка Python зависимостей

```bash
# Для Telegram бота
pip install python-telegram-bot

# Для REST API
pip install fastapi uvicorn

# Для примеров
pip install pydantic
```

---

## Первые шаги

### Проверка установки

Запустите базовый пример:

```bash
python examples/signal_system_basic.py
```

Вывод должен показать:
- Создание SignalSystem
- Обработку нескольких событий
- Статистику обработки
- Подписку на события

---

## Telegram Bot

Самый простой способ попробовать полный pipeline.

### 1. Создание бота

1. Найдите [@BotFather](https://t.me/BotFather) в Telegram
2. Отправьте `/newbot`
3. Следуйте инструкциям
4. Скопируйте токен

### 2. Настройка

```bash
export TELEGRAM_BOT_TOKEN="your_token_here"
```

### 3. Запуск

```bash
python examples/telegram_bot_with_core.py
```

### 4. Использование

Откройте вашего бота в Telegram и попробуйте:

**Команды:**
- `/start` - Приветствие и список команд
- `/stats` - Статистика всего pipeline
- `/core` - Информация о Rust Core
- `/test` - Тестовый прогон с метриками

**Обычные сообщения:**

Отправьте любой текст, бот покажет:
- 🆕 Если паттерн новый (novel)
- 🔗 Количество похожих паттернов (neighbors)
- ⏱ Время обработки в Core

### Пример взаимодействия

```
Вы: Hello!
Бот: 🆕 Novel pattern detected!
     ✅ Processed (Core: 0.42μs)

Вы: Hello again!
Бот: 🔗 Found 1 similar patterns
     ✅ Processed (Core: 0.38μs)

Вы: /stats
Бот: 📊 Statistics

     Pipeline:
     • Processed: 2
     • With Core: 2

     Rust Core:
     • Total events: 2
     • Avg time: 0.40μs
     • Subscribers: 0

     ActionController:
     • Executions: 2
     • Hot path: 2
     • Cold path: 2
```

---

## Python API

### Полный Pipeline

```python
import asyncio
from src.integration import SignalPipeline
import _core

async def main():
    # Создаём Rust Core
    core = _core.SignalSystem()

    # Создаём pipeline
    pipeline = SignalPipeline(core_system=core)

    # Регистрируем actions (опционально)
    # ...

    # Обрабатываем текст
    result = await pipeline.process_text(
        text="Hello, NeuroGraph!",
        user_id="user_123",
        chat_id="chat_456",
        priority=200
    )

    # Результаты
    print("=== Signal Event ===")
    print(f"Event ID: {result['signal_event'].event_id}")
    print(f"Vector: {result['signal_event'].semantic.vector}")

    print("\n=== Core Processing ===")
    core_result = result['processing_result']
    print(f"Token ID: {core_result['token_id']}")
    print(f"Novel: {core_result['is_novel']}")
    print(f"Neighbors: {len(core_result['neighbors'])}")
    print(f"Processing: {core_result['processing_time_us']}μs")

    print("\n=== Actions ===")
    print(f"Hot path: {result['action_results']['stats']['hot_path_executed']}")
    print(f"Cold path: {result['action_results']['stats']['cold_path_queued']}")

    print("\n=== Performance ===")
    print(f"Total: {result['stats']['total_time_ms']:.2f}ms")
    print(f"Core: {result['stats']['core_time_ms']:.2f}ms")

asyncio.run(main())
```

### Только Gateway

```python
from src.gateway import SignalGateway, EncoderType

# Инициализация
gateway = SignalGateway()
gateway.initialize()

# Push text
event = gateway.push_text(
    text="Hello!",
    priority=200,
    sequence_id="conv_001"
)

print(f"Event: {event.event_id}")
print(f"Vector: {event.semantic.vector}")

# Push system metric
metric = gateway.push_system(
    metric_name="cpu_percent",
    metric_value=45.7
)

# Custom sensor
gateway.register_sensor(
    sensor_id="custom.sentiment",
    sensor_type="sentiment_feed",
    domain="external",
    modality="text",
    encoder_type=EncoderType.SENTIMENT_SIMPLE
)
```

### Только Core

```python
import _core

# Создание
system = _core.SignalSystem()

# Emit event
result = system.emit(
    event_type="signal.input.text",
    vector=[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
    priority=200
)

print(f"Token: {result['token_id']}")
print(f"Novel: {result['is_novel']}")

# Subscribe
def handler(event):
    print(f"Event: {event}")

sub_id = system.subscribe(
    name="handler",
    filter_dict={
        "event_type": {"$wildcard": "signal.*"},
        "priority": {"$gte": 150}
    },
    callback=handler
)

# Emit more events
system.emit(
    event_type="signal.test",
    vector=[0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9],
    priority=180
)
# → handler будет вызван

# Statistics
stats = system.get_stats()
print(f"Events: {stats['total_events']}")
print(f"Avg time: {stats['avg_processing_time_us']}μs")
```

---

## REST API

### Запуск сервера

```bash
# С Rust Core
cd src/core_rust
maturin develop --release --features python-bindings
cd ../..

# Запуск API
LOG_LEVEL=INFO python -m src.api.main
```

### Использование

**Health checks:**

```bash
curl http://localhost:8000/api/v1/health
curl http://localhost:8000/api/v1/health/live
curl http://localhost:8000/api/v1/health/ready
```

**Metrics:**

```bash
curl http://localhost:8000/api/v1/metrics
curl http://localhost:8000/api/v1/metrics/json
```

**Operations:**

```bash
# Create token
curl -X POST http://localhost:8000/api/v1/tokens \
  -H "Content-Type: application/json" \
  -d '{"weight": 0.75}'

# Get status
curl http://localhost:8000/api/v1/status
```

См. [REST API Guide](REST_API_GUIDE.md) для полной документации.

---

## Примеры

### Базовые примеры

**SignalSystem:**
```bash
python examples/signal_system_basic.py
```

**Gateway v2.0:**
```bash
python examples/gateway_v2_demo.py
```

### Telegram боты

**С Rust Core (рекомендуется):**
```bash
python examples/telegram_bot_with_core.py
```

**С ActionController:**
```bash
python examples/telegram_bot_actioncontroller.py
```

**Простой (Gateway only):**
```bash
python examples/telegram_bot_simple.py
```

**С подписками:**
```bash
python examples/telegram_bot_advanced.py
```

### Runtime Storage

```bash
python examples/runtime_storage_example.py
```

### Performance тесты

```bash
# Unit tests
pytest tests/unit/

# Integration tests
pytest tests/integration/

# Performance benchmarks
pytest tests/performance/
```

---

## Следующие шаги

1. **Изучите архитектуру**: [docs/specs/](../specs/)
2. **Прочитайте Changelogs**: [docs/changelogs/](../changelogs/)
3. **Попробуйте примеры**: [examples/](../../examples/)
4. **Создайте свои Actions**: [ActionController Guide](ACTION_CONTROLLER_GUIDE.md)
5. **Настройте мониторинг**: [Observability Guide](OBSERVABILITY_GUIDE.md)

---

## Troubleshooting

### Rust Core не импортируется

```python
ModuleNotFoundError: No module named '_core'
```

**Решение:**

```bash
cd src/core_rust
maturin develop --features python-bindings --release

# Проверка symlink
ls -la target/release/ | grep _core
# Должен быть _core.so → lib_core.so
```

### Telegram бот не запускается

```
Error: TELEGRAM_BOT_TOKEN not set
```

**Решение:**

```bash
export TELEGRAM_BOT_TOKEN="your_token"
python examples/telegram_bot_with_core.py
```

### Import errors

```
ModuleNotFoundError: No module named 'src'
```

**Решение:**

```bash
# Добавьте project root в PYTHONPATH
export PYTHONPATH=/path/to/neurograph-os-mvp
python examples/...
```

---

## Помощь

- **Issues**: https://github.com/dchrnv/neurograph-os/issues
- **Email**: dreeftwood@gmail.com
- **Contributing**: [CONTRIBUTING.md](../../CONTRIBUTING.md)
