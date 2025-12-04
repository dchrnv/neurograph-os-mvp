# NeuroGraph

> **Экспериментальная когнитивная архитектура для эмерджентного формирования структур знаний**

[![Version](https://img.shields.io/badge/version-v0.42.0-blue.svg)](https://github.com/dchrnv/neurograph-os)
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

## 🚀 v0.42.0 Final - Observability & Production Monitoring

**Статус:** Production-Ready (Full Stack) ✅

**Новое в v0.42.0:**

- ✅ **Prometheus Metrics** - /metrics endpoint с 15+ метриками для мониторинга
- ✅ **Black Box Recorder** - flight recorder для post-mortem анализа (последние 1000 событий)
- ✅ **Logging Utilities** - structured logging с контекстом и timing'ом

**Новое в v0.41.0:**

- ✅ **Panic Recovery** - системный crash больше не убивает процесс
- ✅ **GIL Release** - Python не блокируется во время Rust операций
- ✅ **WAL (Write-Ahead Log)** - данные не теряются при крахе (CRC32 checksums, binary format)
- ✅ **Resource Quotas** - защита от OOM с настраиваемыми лимитами (10M токенов, 1GB памяти)

**Готово для:**

- ✅ Локальная разработка и production deployment
- ✅ Proof-of-concept и бенчмарки
- ✅ Python bindings (PyO3) с batch API
- ✅ Crash-safe persistence (WAL replay)
- ✅ OOM prevention (Guardian quotas)
- ✅ Production monitoring (Prometheus, Black Box dumps)

**Требует дополнительно для масштабирования:**

- ⏳ Docker deployment (v0.43.0)
- ⏳ Distributed tracing (v0.44.0)

---

## Быстрый старт

### Python Bindings (v0.40.0 - NEW!)

```bash
# Build Python bindings
pip install maturin
cd src/core_rust
maturin develop --release --features python

# Use in Python
python
>>> import neurograph
>>>
>>> # Batch API (4x faster!)
>>> tokens = neurograph.Token.create_batch(100_000)
>>>
>>> # IntuitionEngine
>>> engine = neurograph.IntuitionEngine.with_defaults()
>>> stats = engine.stats()
>>> print(stats)
```

**Документация:** [python/README.md](python/README.md)

**Примеры:**

- [examples/python/token_batch_performance.py](examples/python/token_batch_performance.py)
- [examples/python/intuition_simple.py](examples/python/intuition_simple.py)

### REPL Interface

```bash
cd src/core_rust
cargo run --bin neurograph-repl
```

Интерактивный консольный интерфейс с поддержкой обратной связи:

- Текстовые запросы к системе
- Команды: `/help`, `/status`, `/stats`, `/quit`
- Обратная связь после каждого ответа (y/n/c)

---

## Документация

Полная документация проекта находится в `docs/`:

- **[docs/specs/](docs/specs/)** — технические спецификации
- **[CONTRIBUTING.md](CONTRIBUTING.md)** — как помочь проекту

### Последние обновления

- **v0.42.0 Final** — Observability & Monitoring (production full-stack) 📊
  - Prometheus Metrics - /metrics endpoint с 15+ метриками
  - Black Box Recorder - flight recorder для crash анализа
  - Logging Utilities - structured logging с контекстом
- **v0.41.0 Final** — WAL + Resource Quotas (production-ready core) 🚀
  - Write-Ahead Log для crash-safe persistence
  - Guardian Resource Quotas для OOM prevention
  - Panic Recovery + GIL Release
- **v0.40.0** — Python Bindings (PyO3) с batch API (4x speedup) ⚡
- **v0.39.2** — 1M tokens stress tests, builder pattern API
- **v0.39.1** — RwLock unification, ActionController-Gateway integration
- **v0.39.0** — REST API + WebSockets
- **v0.38.0** — Curiosity Drive (автономное исследование)

---

## Лицензия

Проект лицензирован на условиях **GNU Affero General Public License v3.0**.
Полный текст лицензии: [LICENSE](LICENSE)

---

## Авторы

**Chernov Denys** — архитектура и разработка
С поддержкой Claude Code (Anthropic)
