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

- **[Jupyter Integration Guide](JUPYTER_GUIDE.md)** — Работа в Jupyter notebooks ⭐ NEW
  - Magic commands (%neurograph)
  - Rich HTML display
  - Graph visualization
  - Real-time signals
  - DataFrame export

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

### v0.61.0 (Current)

- **Jupyter Integration** — Полная интеграция с Jupyter notebooks
- IPython magic commands (%neurograph)
- Rich HTML display и graph visualization
- Real-time signals в notebook
- DataFrame export для анализа

**Руководства:**
- [Jupyter Integration Guide](JUPYTER_GUIDE.md) — новое ⭐

### v0.60.1

- **WebSocket Advanced Features** — Production-ready WebSocket
- Prometheus metrics, RBAC, rate limiting
- Reconnection tokens, binary messages
- Message compression (GZIP/ZLIB)

### v0.57.0

- **Gateway-Core Integration** — Полный pipeline с Rust Core
- Telegram бот с реальной обработкой
- Pattern matching, novelty detection
- Performance: 5,601 msg/sec, 0.18ms latency

**Руководства:**
- [Getting Started](GETTING_STARTED.md)
- [Gateway Guide](GATEWAY_GUIDE.md)
- [SignalSystem Guide](SIGNAL_SYSTEM_GUIDE.md)

### Старые версии

См. [docs/archive/](../archive/) для документации предыдущих версий.

---

## Быстрые ссылки

### Quick Start

**Jupyter (рекомендуется для исследований):**
```bash
pip install neurograph[jupyter]
jupyter notebook
```

```python
%load_ext neurograph_jupyter
%neurograph init --path ./my_graph.db
%neurograph query "find all nodes"
```

**Telegram Bot:**
```bash
# 1. Build Rust Core
cd src/core_rust
maturin develop --features python-bindings --release
cd ../..

# 2. Run bot
export TELEGRAM_BOT_TOKEN="your_token"
python examples/telegram_bot_with_core.py
```

### Examples

**Jupyter:**
- [notebooks/jupyter_integration_tutorial.ipynb](../../notebooks/jupyter_integration_tutorial.ipynb) — 15 examples
- [examples/jupyter/quick_start.py](../../examples/jupyter/quick_start.py) — Quick start
- [examples/jupyter/real_time_dashboard.py](../../examples/jupyter/real_time_dashboard.py) — Real-time monitoring

**Python:**
- [examples/telegram_bot_with_core.py](../../examples/telegram_bot_with_core.py) — Full pipeline
- [examples/gateway_v2_demo.py](../../examples/gateway_v2_demo.py) — Gateway only
- [examples/signal_system_basic.py](../../examples/signal_system_basic.py) — Core only

### API Reference

- **Jupyter**: [JUPYTER_GUIDE.md](JUPYTER_GUIDE.md) + [Full Docs](../jupyter/JUPYTER_INTEGRATION.md)
- **Gateway**: [GATEWAY_GUIDE.md](GATEWAY_GUIDE.md#api-reference)
- **SignalSystem**: [SIGNAL_SYSTEM_GUIDE.md](SIGNAL_SYSTEM_GUIDE.md#api-reference)

---

## Помощь

- **Issues**: https://github.com/dchrnv/neurograph-os/issues
- **Email**: dreeftwood@gmail.com
- **Contributing**: [CONTRIBUTING.md](../../CONTRIBUTING.md)
