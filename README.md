# NeuroGraph

> **Экспериментальная когнитивная архитектура для эмерджентного формирования структур знаний**

[![Version](https://img.shields.io/badge/version-v0.61.0-blue.svg)](https://github.com/dchrnv/neurograph-os)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3.8+-green.svg)](https://www.python.org/)
[![Jupyter](https://img.shields.io/badge/jupyter-ready-orange.svg)](https://jupyter.org/)
[![License](https://img.shields.io/badge/license-AGPLv3-blue.svg)](LICENSE)

---

## Что это?

**NeuroGraph** — система моделирования знаний как самоорганизующихся семантических структур в 8-мерном пространстве. Знания представлены токенами, которые взаимодействуют через силовые поля и формируют иерархии без явного программирования.

### Основная идея

- Знания существуют в **8D семантическом пространстве** (физическое, сенсорное, моторное, эмоциональное, когнитивное, социальное, темпоральное, абстрактное)
- Токены **самоорганизуются в семантические поля** через силовое взаимодействие
- Иерархии знаний **возникают эмерджентно**
- Система **непрерывно учится** в рамках конституционных ограничений (CDNA)

---

## 🚀 Текущая версия: v0.61.0

**Jupyter Integration** — Полноценная интеграция с Jupyter notebooks через IPython magic commands

### Новое в v0.61.0

- 🪄 **Magic Commands** - `%neurograph` для быстрых операций (init, status, query, subscribe, emit)
- 📊 **Rich Display** - Красивые HTML таблицы для результатов запросов с градиентными заголовками
- 📡 **Real-time Signals** - Подписка на каналы и обработка событий прямо в notebook
- 🎨 **Graph Visualization** - NetworkX визуализация с 3 layout алгоритмами (spring, circular, kamada_kawai)
- ⚡ **Cell Magic** - `%%signal` для определения обработчиков сигналов
- 📈 **DataFrame Export** - Конвертация результатов в pandas для анализа
- 📚 **Tutorial Notebook** - 15 полных примеров использования

### Новое в v0.60.1

- 📊 **Prometheus Metrics** - 15 метрик для мониторинга (connections, messages, latency, errors)
- 🔐 **RBAC Permissions** - Role-based доступ к каналам (admin, developer, viewer, bot, anonymous)
- ⏱️ **Rate Limiting** - Token bucket алгоритм с разными лимитами на тип сообщения
- 🔄 **Reconnection Tokens** - Бесшовное восстановление сессии с сохранением подписок
- 📦 **Binary Messages** - Поддержка бинарных данных (images, audio, video) со структурированным форматом
- 🗜️ **Message Compression** - GZIP/ZLIB/DEFLATE с адаптивным выбором алгоритма (60-80% экономии)
- 🛠️ **CLI Tool** - Полнофункциональный инструмент для тестирования WebSocket

### Новое в v0.60.0

- 🔄 **WebSocket Support** - Endpoint `/ws` для real-time двусторонней связи
- 📡 **6 Event Channels** - metrics, signals, actions, logs, status, connections
- 🔌 **Client Libraries** - TypeScript/JavaScript и Python клиенты с auto-reconnect
- 📊 **Live Metrics** - Автоматический broadcasting метрик каждые 5 секунд
- 💓 **Heartbeat System** - Ping-pong механизм (30s) для отслеживания живых соединений
- 📦 **Event Buffering** - До 1000 событий на клиента для offline режима

### Архитектура v0.60.1

```
WebSocket Client ←→ /ws Endpoint ←→ [ Metrics | Rate Limit | Permissions ] ←→ Channel System
                         ↓                                                            ↓
              Reconnection Manager                        [metrics, signals, actions, logs, status, connections]
                         ↓                                                            ↓
              Binary/Compression                                            Core Integration
```

### Ключевые возможности

- ⚙️ **Rust Core Integration** - Реальная обработка сигналов через SignalSystem
- 🔐 **Enterprise Security** - JWT + API Keys + RBAC + Rate Limiting
- 🔄 **Real-time Events** - WebSocket broadcasting с ~5ms latency
- 🎯 **Pattern Matching** - Детекция новизны, поиск соседей
- ⚡ **High Performance** - 5,601 msg/sec end-to-end, 0.39μs Core latency
- 📚 **Client SDKs** - TypeScript и Python клиенты из коробки
- 🤖 **Production Ready** - Готовые примеры (Telegram бот, WebSocket demo)

### Performance

| Metric | Value |
|--------|-------|
| **Core throughput** | 304,553 events/sec |
| **Core latency** | 0.39μs average |
| **WebSocket latency** | ~5ms event delivery |
| **Full pipeline** | 5,601 messages/sec |
| **End-to-end latency** | 0.18ms total |

---

## Быстрый старт

### 1. Jupyter Notebook (рекомендуется для исследований)

Интерактивная работа с графом прямо в notebook:

```bash
# Установка с Jupyter поддержкой
pip install neurograph[jupyter]

# Запуск Jupyter
jupyter notebook
```

В notebook:

```python
# Загрузка расширения
%load_ext neurograph_jupyter

# Инициализация
%neurograph init --path ./my_graph.db

# Запрос с красивым отображением
%neurograph query "find all nodes where type='user'"

# Визуализация графа
from neurograph_jupyter.display import render_graph_visualization
result = neurograph_db.query("find all nodes")
render_graph_visualization(result, layout="spring")
```

📚 **[Полный туториал](notebooks/jupyter_integration_tutorial.ipynb)** с 15 примерами

### 2. Telegram Bot

Полный пример с реальной обработкой через Rust Core:

```bash
# Сборка Rust Core
cd src/core_rust
maturin develop --features python-bindings --release
cd ../..

# Установка зависимостей
pip install python-telegram-bot

# Настройка токена
export TELEGRAM_BOT_TOKEN="your_token_here"

# Запуск
python examples/telegram_bot_with_core.py
```

**Команды бота:**
- `/start` - Информация об архитектуре
- `/stats` - Статистика Pipeline + Core + ActionController
- `/core` - Информация о Rust Core
- `/test` - Тест полного pipeline с метриками

### 2. WebSocket Real-time Demo (NEW in v0.60.0)

Демонстрация real-time событий через WebSocket:

```bash
# Запустить API сервер
python -m src.api.main

# В другом терминале - запустить WebSocket клиент
python examples/websocket_demo.py
```

**Python WebSocket Client:**
```python
from neurograph_ws_client import NeurographWSClient, Channel

# Создать клиент
client = NeurographWSClient(url="ws://localhost:8000/ws")
await client.connect()

# Подписаться на события
client.subscribe(Channel.METRICS, lambda data: print(f"Metrics: {data}"))
client.subscribe(Channel.SIGNALS, lambda data: print(f"Signal: {data}"))

# Запустить forever
await client.run_forever()
```

**TypeScript WebSocket Client:**
```typescript
import NeurographWSClient from "./neurograph-ws-client";

const client = new NeurographWSClient({
  url: "ws://localhost:8000/ws",
  autoReconnect: true
});

await client.connect();
client.subscribe("metrics", (data) => console.log("Metrics:", data));
```

### 3. WebSocket CLI Tool (NEW in v0.60.1)

Тестирование WebSocket без написания кода:

```bash
# Базовое подключение
python -m src.api.websocket.cli --url ws://localhost:8000/ws

# С подпиской на каналы
python -m src.api.websocket.cli --url ws://localhost:8000/ws --subscribe metrics,signals

# С аутентификацией
python -m src.api.websocket.cli --url ws://localhost:8000/ws --token YOUR_JWT_TOKEN

# JSON output
python -m src.api.websocket.cli --url ws://localhost:8000/ws --format json
```

**Возможности CLI:**
- Цветной вывод событий в real-time
- Автоматическая подписка на каналы
- Поддержка JWT аутентификации
- Форматы вывода: pretty, json, compact
- Показывает типы событий и метаданные

### 4. Python API

```python
from src.integration import SignalPipeline
import _core

# Создаём Rust Core
core = _core.SignalSystem()

# Создаём полный pipeline
pipeline = SignalPipeline(core_system=core)

# Обрабатываем текст
result = await pipeline.process_text(
    text="Hello, NeuroGraph!",
    user_id="user_123",
    chat_id="chat_456",
    priority=200
)

# Результат обработки
print(f"Novel: {result['processing_result']['is_novel']}")
print(f"Neighbors: {len(result['processing_result']['neighbors'])}")
print(f"Core time: {result['stats']['core_time_ms']:.2f}ms")
```

### 3. Прямая работа с Core

```python
import _core

# Создаём систему
system = _core.SignalSystem()

# Эмитим событие
result = system.emit(
    event_type="signal.input.text",
    vector=[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
    priority=200
)

print(f"Token ID: {result['token_id']}")
print(f"Is Novel: {result['is_novel']}")
print(f"Processing: {result['processing_time_us']}μs")
```

---

## WebSocket Advanced Features (v0.60.1)

### Reconnection Tokens

Бесшовное восстановление сессии после разрыва:

```python
# Запросить reconnection token перед отключением
await client.send({"type": "get_reconnection_token"})
# Ответ: {"type": "reconnection_token", "token": "...", "expires_in": 300}

# Переподключение с восстановлением сессии
client = NeurographWSClient(
    url="ws://localhost:8000/ws",
    reconnection_token="your_token"
)
await client.connect()
# Все подписки автоматически восстановлены!
```

### Permissions & RBAC

Контроль доступа к каналам по ролям:

| Channel | Admin | Developer | Viewer | Bot | Anonymous |
|---------|-------|-----------|--------|-----|-----------|
| metrics | ✅ Sub+Broadcast | ✅ Subscribe | ✅ Subscribe | ✅ Subscribe | ✅ Subscribe |
| signals | ✅ Sub+Broadcast | ✅ Subscribe | ❌ | ✅ Subscribe | ❌ |
| actions | ✅ Sub+Broadcast | ✅ Subscribe | ❌ | ❌ | ❌ |
| logs | ✅ Sub+Broadcast | ✅ Subscribe | ❌ | ❌ | ❌ |
| status | ✅ Sub+Broadcast | ✅ Subscribe | ✅ Subscribe | ✅ Subscribe | ✅ Subscribe |
| connections | ✅ Sub+Broadcast | ❌ | ❌ | ❌ | ❌ |

### Rate Limiting

Token bucket алгоритм с разными лимитами:

| Message Type | Capacity | Refill Rate |
|-------------|----------|-------------|
| ping | 120 | 2/sec |
| subscribe | 30 | 1/sec |
| unsubscribe | 30 | 1/sec |
| default | 60 | 10/sec |

### Prometheus Metrics

15 метрик для production мониторинга:

```python
# Доступны на /metrics endpoint
neurograph_ws_connections_total          # Активные соединения
neurograph_ws_connections_opened_total   # Всего открыто
neurograph_ws_connections_closed_total   # Всего закрыто
neurograph_ws_connection_duration_seconds  # Длительность соединений
neurograph_ws_messages_sent_total        # Отправлено сообщений
neurograph_ws_messages_received_total    # Получено сообщений
neurograph_ws_message_size_bytes         # Размер сообщений
neurograph_ws_message_latency_seconds    # Latency
neurograph_ws_subscriptions_total        # Подписки
neurograph_ws_channel_subscribers        # Подписчики по каналам
neurograph_ws_buffered_events            # Буферизованные события
neurograph_ws_errors_total               # Ошибки
```

### Binary Messages & Compression

Эффективная передача больших данных:

```python
from src.api.websocket.binary import binary_handler
from src.api.websocket.compression import default_compressor

# Отправить изображение
image_bytes = open("photo.jpg", "rb").read()
binary_msg = binary_handler.create_image_message(
    image_bytes,
    format="jpeg",
    width=1920,
    height=1080
)

# Сжатие больших JSON (60-80% экономии)
large_data = {"key": "value" * 1000}
compressed, was_compressed = default_compressor.compress_json(large_data)
# compressed size: ~2KB vs ~10KB original
```

---

## Документация

### Руководства

- **[Getting Started](docs/guides/GETTING_STARTED.md)** - Подробное руководство для начинающих
- **[Gateway v2.0 Guide](docs/guides/GATEWAY_GUIDE.md)** - Работа с сенсорным слоем
- **[SignalSystem Guide](docs/guides/SIGNAL_SYSTEM_GUIDE.md)** - Rust Core API
- **[REST API Guide](docs/guides/REST_API_GUIDE.md)** - HTTP API documentation
- **[Python Library Guide](docs/guides/PYTHON_LIBRARY_GUIDE.md)** - FFI bindings

### Changelogs

- **[CHANGELOG v0.60.1](docs/changelogs/CHANGELOG_v0.60.1.md)** - WebSocket Advanced Features ← **LATEST**
- **[CHANGELOG v0.60.0](docs/changelogs/CHANGELOG_v0.60.0.md)** - WebSocket & Real-time Events
- **[CHANGELOG v0.58.0](docs/changelogs/CHANGELOG_v0.58.0.md)** - Authentication & Security
- **[CHANGELOG v0.57.0](docs/changelogs/CHANGELOG_v0.57.0.md)** - Gateway-Core Integration
- **[CHANGELOG v0.56.0](docs/changelogs/CHANGELOG_v0.56.0.md)** - ActionController Foundation
- **[All Changelogs](docs/changelogs/)** - Полная история версий

### Спецификации

- **[docs/specs/](docs/specs/)** - Технические спецификации
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Как помочь проекту

### Архив

- **[docs/archive/](docs/archive/)** - Документация старых версий

---

## Тестирование

```bash
# Unit tests
pytest tests/unit/

# Integration tests
pytest tests/integration/

# Performance tests
pytest tests/performance/

# All tests
pytest tests/
```

---

## Deployment

### Docker (рекомендуется)

```bash
# Запуск с мониторингом
docker-compose up -d

# API: http://localhost:8080
# Metrics: http://localhost:8080/metrics
# Jaeger: http://localhost:16686
```

См. **[DOCKER.md](DOCKER.md)** для деталей.

### Production Features

- ✅ **High Performance** - 22M tokens/sec throughput (Rust Core)
- ✅ **Crash-Safe Persistence** - WAL with CRC32 checksums
- ✅ **OOM Prevention** - Guardian resource quotas
- ✅ **Structured Logging** - JSON logs с correlation ID
- ✅ **Prometheus Metrics** - 12 metric types
- ✅ **Distributed Tracing** - OpenTelemetry + Jaeger
- ✅ **Kubernetes Ready** - Health checks (live/ready/startup)

---

## Roadmap

**Completed:**
- ✅ v0.60.1 - WebSocket Advanced Features (Dec 2024)
- ✅ v0.60.0 - WebSocket & Real-time Events
- ✅ v0.58.0 - Authentication & Security
- ✅ v0.57.0 - Gateway-Core Integration
- ✅ v0.56.0 - ActionController Foundation
- ✅ v0.55.0 - Subscription Filters & Sensors
- ✅ v0.54.0 - Gateway v2.0 (Pydantic models)
- ✅ v0.53.0 - SignalSystem Python Bindings
- ✅ v0.52.0 - Observability & Monitoring
- ✅ v0.51.0 - REST API + RuntimeStorage

**Next (см. [MASTER_PLAN v3.0](docs/MASTER_PLAN_v3.0.md)):**
- 📊 v0.61.0 - Jupyter Integration (Magic commands)
- 🎨 v0.62.0 - Web Dashboard (React SPA)
- 🎥 v0.63.0 - Enhanced Sensors (Audio & Vision)

---

## Лицензия

**Двойное лицензирование** (dual licensing):

- **Open Source**: [GNU AGPL v3.0](LICENSE) (код) + [CC BY-NC-SA 4.0](LICENSE-DATA) (данные)
- **Commercial**: Проприетарная лицензия для коммерческого использования

**Документация:**
- [DUAL_LICENSING.md](docs/legal/DUAL_LICENSING.md)
- [CLA.md](docs/legal/CLA.md)

**Контакт**: <dreeftwood@gmail.com>

---

## Авторы

**Chernov Denys** — архитектура и разработка
С поддержкой Claude Code (Anthropic)
