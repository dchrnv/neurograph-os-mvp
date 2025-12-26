# NeuroGraph

> **Экспериментальная когнитивная архитектура для эмерджентного формирования структур знаний**

[![Version](https://img.shields.io/badge/version-v0.57.0-blue.svg)](https://github.com/dchrnv/neurograph-os)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3.8+-green.svg)](https://www.python.org/)
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

## 🚀 Текущая версия: v0.57.0

**Gateway-Core Integration** — полная интеграция сенсорного слоя с Rust Core

### Архитектура v0.57.0

```
Input → Gateway (8D encoding) → Rust Core (pattern matching) → ActionController → Response
```

### Ключевые возможности

- ⚙️ **Rust Core Integration** - Реальная обработка сигналов через SignalSystem
- 🎯 **Pattern Matching** - Детекция новизны, поиск соседей
- ⚡ **High Performance** - 5,601 msg/sec end-to-end, 0.39μs Core latency
- 🔄 **Complete Pipeline** - Полный цикл обработки
- 🤖 **Production Ready** - Готовые примеры (Telegram бот)

### Performance

| Metric | Value |
|--------|-------|
| **Core throughput** | 304,553 events/sec |
| **Core latency** | 0.39μs average |
| **Full pipeline** | 5,601 messages/sec |
| **End-to-end latency** | 0.18ms total |

---

## Быстрый старт

### 1. Telegram Bot (рекомендуется)

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

### 2. Python API

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

## Документация

### Руководства

- **[Getting Started](docs/guides/GETTING_STARTED.md)** - Подробное руководство для начинающих
- **[Gateway v2.0 Guide](docs/guides/GATEWAY_GUIDE.md)** - Работа с сенсорным слоем
- **[SignalSystem Guide](docs/guides/SIGNAL_SYSTEM_GUIDE.md)** - Rust Core API
- **[REST API Guide](docs/guides/REST_API_GUIDE.md)** - HTTP API documentation
- **[Python Library Guide](docs/guides/PYTHON_LIBRARY_GUIDE.md)** - FFI bindings

### Changelogs

- **[CHANGELOG v0.57.0](docs/changelogs/CHANGELOG_v0.57.0.md)** - Gateway-Core Integration ← **LATEST**
- **[CHANGELOG v0.56.0](docs/changelogs/CHANGELOG_v0.56.0.md)** - ActionController Foundation
- **[CHANGELOG v0.55.0](docs/changelogs/CHANGELOG_v0.55.0.md)** - Subscription Filters & Sensors
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
- ✅ v0.57.0 - Gateway-Core Integration (Dec 2024)
- ✅ v0.56.0 - ActionController Foundation
- ✅ v0.55.0 - Subscription Filters & Sensors
- ✅ v0.54.0 - Gateway v2.0 (Pydantic models)
- ✅ v0.53.0 - SignalSystem Python Bindings
- ✅ v0.52.0 - Observability & Monitoring
- ✅ v0.51.0 - REST API + RuntimeStorage

**Next:**
- 🎯 v0.58.0 - Authentication & Security (JWT, RBAC)
- ⏳ v0.59.0 - Web Dashboard (React)
- ⏳ v0.60.0 - Jupyter Integration

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
