# NeuroGraph OS Guides

Подробные руководства по работе с NeuroGraph OS.

## Содержание

### Для начинающих

- **[Getting Started](GETTING_STARTED.md)** — Полное руководство для новичков
  - Установка и настройка
  - Первые шаги
  - Примеры использования
  - Troubleshooting

### Основные компоненты

- **[Gateway v2.0 Guide](GATEWAY_GUIDE.md)** — Сенсорный интерфейс
  - Архитектура Gateway
  - Encoders (PASSTHROUGH, TEXT_TFIDF, NUMERIC_DIRECT, SENTIMENT_SIMPLE)
  - Sensors (встроенные и custom)
  - API Reference

- **[SignalSystem Guide](SIGNAL_SYSTEM_GUIDE.md)** — Rust Core
  - Python API через PyO3 bindings
  - Pattern matching и novelty detection
  - Subscription filters
  - Performance optimization

### Разработка

- **REST API Guide** (TODO) — HTTP API documentation
- **Python Library Guide** (TODO) — FFI bindings для прямой работы с Rust
- **ActionController Guide** (TODO) — Система генерации ответов
- **Observability Guide** (TODO) — Мониторинг и логирование

### Deployment

- **[DOCKER.md](../../DOCKER.md)** — Docker deployment
- **Kubernetes Guide** (TODO) — K8s deployment

---

## По версиям

### v0.57.0 (Current)

- **Gateway-Core Integration** — Полный pipeline с Rust Core
- Telegram бот с реальной обработкой
- Pattern matching, novelty detection
- Performance: 5,601 msg/sec, 0.18ms latency

**Руководства:**
- [Getting Started](GETTING_STARTED.md) — актуализировано для v0.57.0
- [Gateway Guide](GATEWAY_GUIDE.md) — новое
- [SignalSystem Guide](SIGNAL_SYSTEM_GUIDE.md) — новое

### Старые версии

См. [docs/archive/](../archive/) для документации предыдущих версий.

---

## Быстрые ссылки

### Quick Start

```bash
# 1. Build Rust Core
cd src/core_rust
maturin develop --features python-bindings --release
cd ../..

# 2. Run Telegram bot
export TELEGRAM_BOT_TOKEN="your_token"
python examples/telegram_bot_with_core.py
```

### Examples

- [examples/telegram_bot_with_core.py](../../examples/telegram_bot_with_core.py) — Full pipeline
- [examples/gateway_v2_demo.py](../../examples/gateway_v2_demo.py) — Gateway only
- [examples/signal_system_basic.py](../../examples/signal_system_basic.py) — Core only

### API Reference

- **Gateway**: [GATEWAY_GUIDE.md](GATEWAY_GUIDE.md#api-reference)
- **SignalSystem**: [SIGNAL_SYSTEM_GUIDE.md](SIGNAL_SYSTEM_GUIDE.md#api-reference)

---

## Помощь

- **Issues**: https://github.com/dchrnv/neurograph-os/issues
- **Email**: dreeftwood@gmail.com
- **Contributing**: [CONTRIBUTING.md](../../CONTRIBUTING.md)
