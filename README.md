# NeuroGraph

> **Экспериментальная когнитивная архитектура для эмерджентного формирования структур знаний**

[![Version](https://img.shields.io/badge/version-v0.45.0-blue.svg)](https://github.com/dchrnv/neurograph-os)
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

## 🚀 v0.45.0 - Cross-Service Sampling Propagation

**Статус:** Production-Ready (Complete distributed tracing solution) ✅

**Текущая версия: v0.45.0** - Полная поддержка distributed systems

### Ключевые возможности v0.45.0:

- 🔗 **Cross-Service Sampling** - автоматическая propagation sampling decisions
- 🌐 **W3C TraceContext** - стандарт-совместимая интеграция
- 📊 **Complete Traces** - 10,000x improvement в trace completeness
- ⚡ **Head-Based Sampling** - управление через HTTP headers (v0.44.4)
- 🎯 **Dynamic Rate Adjustment** - автоподстройка под нагрузку (v0.44.4)
- 🔧 **Adaptive Sampling** - 9% overhead вместо 98% (v0.44.3)

### 📊 Production Performance (актуально для v0.45.0):

| Component | Overhead | Status | Version |
|-----------|----------|--------|---------|
| **Core Performance** | 0% (baseline) | ✅ 22M tokens/sec | v0.40.0 |
| **WAL writes** | 8% | ✅ Async MPSC | v0.44.2 |
| **Distributed Tracing** | 9% | ✅ Adaptive sampling | v0.44.3 |
| **Prometheus Metrics** | <5% | ✅ Lock-free | v0.42.0 |
| **Guardian Quotas** | <1% | ✅ Minimal | v0.41.0 |
| **Total Production** | **~22%** | ✅ **Production-Ready** | ✅ |

### 🎯 Distributed Tracing Features (v0.43.0 - v0.45.0):

**Evolution of observability:**

```
v0.44.0: OpenTelemetry + Jaeger (17x overhead) ❌
    ↓
v0.44.3: Adaptive Sampling (9% overhead) ✅
    ↓
v0.44.4: Head-Based + Dynamic Rate ✅
    ↓
v0.45.0: Cross-Service Propagation ✅ ← YOU ARE HERE
```

**Trace Completeness Improvement:**
- **Before v0.45.0**: 0.001% complete traces (broken distributed traces)
- **After v0.45.0**: 10% complete traces (parent sampling inherited)
- **Improvement**: **10,000x more complete traces** 🚀

### Production Deployment Guide:

```bash
# 1. Docker Compose (рекомендуется)
docker-compose up -d

# 2. Доступные endpoints
http://localhost:3000          # REST API
http://localhost:3000/metrics  # Prometheus metrics
http://localhost:16686         # Jaeger UI (tracing)
http://localhost:9090          # Prometheus UI (optional)
http://localhost:3001          # Grafana (optional)
```

**См. также:**
- [CHANGELOG v0.45.0](docs/changelogs/CHANGELOG_v0.45.0.md) - Cross-service sampling
- [CHANGELOG v0.44.4](docs/changelogs/CHANGELOG_v0.44.4.md) - Head-based sampling
- [CHANGELOG v0.44.3](docs/changelogs/CHANGELOG_v0.44.3.md) - Adaptive sampling
- [CHANGELOG v0.44.2](docs/changelogs/CHANGELOG_v0.44.2.md) - Async WAL
- [Performance Tests](docs/performance/STRESS_TEST_v0.44.0.md)

### Production-Ready Features (v0.45.0):

**Core Infrastructure:**
- ✅ **High Performance** - 22M tokens/sec throughput
- ✅ **Crash-Safe Persistence** - WAL with CRC32 checksums (v0.41.0)
- ✅ **OOM Prevention** - Guardian resource quotas (v0.41.0)
- ✅ **Panic Recovery** - Process doesn't crash on errors (v0.41.0)
- ✅ **GIL Release** - Non-blocking Python integration (v0.41.0)

**Observability & Monitoring:**
- ✅ **Prometheus Metrics** - /metrics endpoint с 15+ метриками (v0.42.0)
- ✅ **Black Box Recorder** - Flight recorder для post-mortem анализа (v0.42.0)
- ✅ **Distributed Tracing** - OpenTelemetry + Jaeger (v0.44.0)
- ✅ **Adaptive Sampling** - 9% overhead вместо 98% (v0.44.3)
- ✅ **Head-Based Sampling** - HTTP header control (v0.44.4)
- ✅ **Cross-Service Propagation** - W3C TraceContext (v0.45.0)

**Deployment & DevOps:**
- ✅ **Docker Deployment** - Multi-stage Dockerfile <50MB (v0.43.0)
- ✅ **Docker Compose** - Full stack с мониторингом (v0.43.0)
- ✅ **Python Bindings** - PyO3 с batch API (v0.40.0)
- ✅ **REST API + WebSockets** - Полный API (v0.39.0)

**Использование:**
- ✅ Local development & production deployment
- ✅ Docker/Kubernetes deployment
- ✅ Distributed microservices (complete trace propagation)
- ✅ High-load scenarios (22% total overhead)

**Roadmap (Next Steps):**
- 🎯 **v0.46.0** - Desktop UI (таск-менеджер интерфейс)
- ⏳ **v0.47.0** - Tail-Based Sampling (OpenTelemetry Collector)
- ⏳ **v0.48.0** - ML-Based Sampling (IntuitionEngine integration)

---

## Быстрый старт

### Docker Deployment (Production-Ready)

```bash
# Quick start (single command)
docker-compose up -d

# API доступен на http://localhost:8080
curl http://localhost:8080/health

# Метрики
curl http://localhost:8080/metrics

# С мониторингом (Prometheus + Grafana)
docker-compose --profile monitoring up -d

# Grafana: http://localhost:3000 (admin/admin)
# Prometheus: http://localhost:9090
```

**Документация:** [DOCKER.md](DOCKER.md)

**Особенности:**

- Multi-stage build (<50MB образ)
- Health checks и resource limits
- Non-root user для безопасности
- Persistent volumes для данных
- Optional monitoring stack

### Python Bindings (v0.40.0)

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

- **v0.45.0** — Cross-Service Sampling Propagation 🔗
  - W3C TraceContext integration for parent trace sampling inheritance
  - Automatic sampling decision propagation across distributed services
  - Maintains trace continuity in microservices architecture
  - Zero configuration - works automatically with existing traceparent headers
- **v0.44.4** — Head-Based Sampling & Dynamic Rate Adjustment ⚡
  - Head-based sampling via HTTP headers (X-Force-Trace, X-Sampling-Priority)
  - Dynamic rate adjustment based on system load (auto-tune sampling)
  - Priority levels: High (10x rate), Normal (1x), Low (0.1x)
  - Load-aware adaptation: reduce rate at high RPS, increase at low RPS
- **v0.44.3** — Adaptive Tracing Sampling (Observability Without Overhead) 🎯
  - Reduces tracing overhead from 98% → 9% (10x improvement)
  - Adaptive sampling: 1% baseline, 100% errors, 50% slow requests
  - CDNA integration for configurable sampling rates
  - All production bottlenecks eliminated (total: 22% overhead)
- **v0.44.2** — Async WAL Writer (P0 Critical Performance Fix) 🚀
  - Async WAL с batching (1000 entries/fsync)
  - 10,000x performance improvement (971x → 8% overhead)
  - MPSC channel + graceful shutdown
  - Production-ready с minimal overhead
- **v0.44.1** — Observability Analysis & Documentation 📊
  - Comprehensive stress testing (9.5M tokens, ~7 minutes)
  - Performance bottleneck identification (WAL: 971x, Tracing: 98%)
  - Known issues documentation с production recommendations
  - Roadmap для v0.44.2 (Async WAL), v0.44.3 (Tracing Sampling)
- **v0.44.0 Final** — Distributed Tracing (observability complete) 🔍
  - OpenTelemetry integration с Jaeger backend
  - W3C TraceContext propagation через HTTP headers
  - Automatic span creation для всех HTTP requests
  - Trace visualization в Jaeger UI
- **v0.43.0 Final** — Docker Deployment (container-native) 🐳
  - Multi-stage Dockerfile (<50MB Alpine-based image)
  - Docker Compose с monitoring stack (Prometheus + Grafana)
  - Production-ready: health checks, resource limits, non-root user
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

NeuroGraph использует **модель двойного лицензирования** (dual licensing):

### Open Source (Бесплатно)

- **Код**: [GNU Affero General Public License v3.0](LICENSE) (AGPLv3)
- **Данные/Модели**: [Creative Commons BY-NC-SA 4.0](LICENSE-DATA) (CC BY-NC-SA 4.0)

### Commercial (Платно)

Для коммерческого использования без ограничений AGPL/CC доступны проприетарные лицензии.

**Подробнее:**
- [docs/legal/DUAL_LICENSING.md](docs/legal/DUAL_LICENSING.md) - Объяснение модели
- [docs/legal/CLA.md](docs/legal/CLA.md) - Соглашение для контрибьюторов
- **Контакт**: <dreeftwood@gmail.com>

---

## Авторы

**Chernov Denys** — архитектура и разработка
С поддержкой Claude Code (Anthropic)
