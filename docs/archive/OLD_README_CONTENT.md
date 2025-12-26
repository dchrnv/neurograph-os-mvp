# Old README Content (Archive)

Этот документ содержит секции из старого README, которые были удалены для минимализма.

**Дата архивации**: 2024-12-26
**Причина**: README refactoring - перенос детальной документации в guides

---

## Production Performance Table (v0.45.0)

| Component | Overhead | Status | Version |
|-----------|----------|--------|---------|
| **Core Performance** | 0% (baseline) | ✅ 22M tokens/sec | v0.40.0 |
| **WAL writes** | 8% | ✅ Async MPSC | v0.44.2 |
| **Distributed Tracing** | 9% | ✅ Adaptive sampling | v0.44.3 |
| **Prometheus Metrics** | <5% | ✅ Lock-free | v0.42.0 |
| **Guardian Quotas** | <1% | ✅ Minimal | v0.41.0 |
| **Total Production** | **~22%** | ✅ **Production-Ready** | ✅ |

---

## Old Telegram Bot Examples

### Bot without Core (v0.56.0)

```bash
python examples/telegram_bot_actioncontroller.py
```

Features:
- ActionController integration
- Hot/Cold path routing
- No Rust Core (mock processing)

### Simple Bot (v0.55.0)

```bash
python examples/telegram_bot_simple.py
```

Features:
- Gateway only
- Basic text encoding
- No Core, no ActionController

### Advanced Bot (v0.55.0)

```bash
python examples/telegram_bot_advanced.py
```

Features:
- Gateway with subscriptions
- Event filtering
- Custom handlers

---

## Old Quick Start Sections

### REST API Detailed Usage

**Start Server:**

```bash
# Build FFI module
cd src/core_rust
maturin develop --release --features python-bindings

# Start REST API server (with structured logging)
cd ../..
LOG_LEVEL=INFO LOG_JSON_FORMAT=true python -m src.api.main
# Server running at http://localhost:8000
# JSON logs output to stdout with correlation IDs
```

**Usage - REST API:**

```bash
# Health checks (Kubernetes-ready)
curl http://localhost:8000/api/v1/health/live    # Liveness probe
curl http://localhost:8000/api/v1/health/ready   # Readiness probe
curl http://localhost:8000/api/v1/health/startup # Startup probe
curl http://localhost:8000/api/v1/health         # Basic health
# → {"status": "healthy", "runtime_metrics": {"tokens_count": 0, "storage_backend": "runtime"}}

# Prometheus metrics (NEW in v0.52.0)
curl http://localhost:8000/api/v1/metrics        # Prometheus text format
curl http://localhost:8000/api/v1/metrics/json   # JSON format (human-readable)

# Create token (auto-tracked in metrics)
curl -X POST http://localhost:8000/api/v1/tokens \
  -H "Content-Type: application/json" \
  -d '{"weight": 0.75}'
# → {"success": true, "data": {"id": 1, "weight": 0.0, ...}}
# Automatically tracked: neurograph_token_operations_total, neurograph_http_requests_total

# System status (optimized: <10ms)
curl http://localhost:8000/api/v1/status
# → {"state": "running", "memory_usage_mb": 75.38, "tokens": {"total": 1}, ...}
```

**Structured Logging Example:**

```json
{
  "timestamp": "2024-12-20T09:17:14.873680+00:00",
  "level": "INFO",
  "logger": "src.api.main",
  "message": "POST /api/v1/tokens - 201",
  "correlation_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "method": "POST",
  "path": "/api/v1/tokens",
  "status_code": 201,
  "duration_ms": 5.23
}
```

---

## Python Library (Direct FFI) - Old Content

```python
from neurograph import Runtime, Config

# Initialize runtime with storage
config = Config(grid_size=1000, dimensions=50)
runtime = Runtime(config)

# Token operations
token_id = runtime.tokens.create(weight=1.0)
token = runtime.tokens.get(token_id)
runtime.tokens.update(token_id, weight=0.9)

# CDNA operations (NEW in v0.51.0)
scales = runtime.cdna.get_scales()  # Returns [1.0, 1.0, ..., 1.0]
runtime.cdna.update_scales([1.5, 1.5, 2.0, 2.0, 2.5, 2.5, 3.0, 3.0])

# Connection operations
conn_id = runtime.connections.create(token_a=token_id, token_b=another_token)

# Spatial queries
neighbors = runtime.grid.find_neighbors(token_id=token_id, radius=10.0)
for neighbor_id, distance in neighbors:
    print(f"Token {neighbor_id} at distance {distance:.2f}")

# CDNA configuration
runtime.cdna.update_scales([1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5])
runtime.cdna.set_profile(1)  # Explorer profile
```

**Usage - Semantic Search (v0.47.0):**

```python
# Load embeddings (GloVe format)
runtime.bootstrap("glove.6B.50d.txt", limit=50000)

# Semantic query
result = runtime.query("cat", top_k=5)
for word, similarity in result.top(5):
    print(f"{word}: {similarity:.4f}")
# Output: kitten: 0.9980, dog: 0.9950, puppy: 0.9940, ...

# Provide feedback
result.feedback("positive")
```

---

## Production Deployment Guide (Old)

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

---

## Production-Ready Features List (v0.45.0)

**Core Infrastructure:**
- ✅ **High Performance** - 22M tokens/sec throughput
- ✅ **Crash-Safe Persistence** - WAL with CRC32 checksums (v0.41.0)
- ✅ **OOM Prevention** - Guardian resource quotas (v0.41.0)
- ✅ **Panic Recovery** - Process doesn't crash on errors (v0.41.0)
- ✅ **GIL Release** - Non-blocking Python integration (v0.41.0)

**Observability & Monitoring:**
- ✅ **Structured Logging** - JSON logs с correlation ID tracking (v0.52.0)
- ✅ **Prometheus Metrics** - 12 metric types для полного мониторинга (v0.52.0)
- ✅ **Kubernetes Health Checks** - 4 endpoints (live/ready/startup) (v0.52.0)
- ✅ **Performance Optimized** - /status endpoint 11.3x faster (v0.52.0)
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

---

## Old Roadmap (Completed Items)

**Roadmap (Next Steps):**
- ✅ **v0.47.0** - Python Library (Phase 1: Complete semantic search)
- ✅ **v0.49.0** - REST API (Phase 2: FastAPI routers complete)
- ✅ **v0.50.0** - RuntimeStorage (Unified storage with full Python API)
- ✅ **v0.51.0** - REST API + RuntimeStorage Integration
- ✅ **v0.52.0** - Observability & Monitoring (Structured logging + Prometheus)
- ✅ **v0.53.0** - SignalSystem Python Bindings
- ✅ **v0.54.0** - Gateway v2.0 (Pydantic models, encoders, sensors)
- ✅ **v0.55.0** - Subscription Filters & First Sensors
- ✅ **v0.56.0** - ActionController Foundation
- ✅ **v0.57.0** - Gateway-Core Integration ← **LATEST**

---

## Docker Deployment (Old Content)

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

---

## Python Library with RuntimeStorage (v0.50.0) - Old Content

```bash
# Build FFI module
pip install maturin
cd src/core_rust
maturin develop --release --features python-bindings

# Run example
cd ../..
python examples/runtime_storage_example.py
```

**RuntimeStorage API:**

```python
from neurograph import Runtime, Config

# Initialize runtime
config = Config(grid_size=1000, dimensions=50)
runtime = Runtime(config)

# Token operations
token_id = runtime.tokens.create(weight=1.0)
token = runtime.tokens.get(token_id)
runtime.tokens.update(token_id, weight=0.9)
runtime.tokens.delete(token_id)

# Connection operations
conn_id = runtime.connections.create(token_a=1, token_b=2)
conn = runtime.connections.get(conn_id)

# Spatial grid queries
neighbors = runtime.grid.find_neighbors(token_id=1, radius=10.0)
results = runtime.grid.range_query(center=(0, 0, 0), radius=5.0)

# CDNA configuration
runtime.cdna.update_scales([1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5])
runtime.cdna.set_profile(1)  # Explorer profile
```

---

## REPL Interface (Old Content)

```bash
cd src/core_rust
cargo run --bin neurograph-repl
```

Интерактивный консольный интерфейс с поддержкой обратной связи:

- Текстовые запросы к системе
- Команды: `/help`, `/status`, `/stats`, `/quit`
- Обратная связь после каждого ответа (y/n/c)

---

## Version History - Detailed Changelogs

- **v0.52.0** — Observability & Monitoring 📊
  - Structured JSON logging with correlation ID tracking
  - Prometheus metrics (12 types: HTTP, tokens, grid, CDNA, FFI, system)
  - /status endpoint optimized 11.3x (108ms → 9.5ms P95)
  - Enhanced health checks: /health/live, /health/ready, /health/startup
  - Kubernetes-ready probes with proper lifecycle management
  - Zero breaking changes, fully backward compatible
  - Production-ready observability stack for real-time monitoring
  - See: [CHANGELOG v0.52.0](docs/changelogs/CHANGELOG_v0.52.0.md)

- **v0.51.0** — REST API + RuntimeStorage Integration 🌐
  - Full REST API with RuntimeStorage backend (34 endpoints)
  - Enhanced FFI with 26 methods exposing RuntimeStorage to Python
  - Thread-safe Arc<RwLock<T>> for concurrent REST requests
  - Bug fixes: Token CRUD, CDNA scales, format 'X' error
  - Production tested with integration tests
  - See: [CHANGELOG v0.51.0](docs/changelogs/CHANGELOG_v0.51.0.md)

- **v0.50.0** — RuntimeStorage Complete Integration 🗄️
  - Unified RuntimeStorage in Rust with thread-safe Arc<RwLock<T>>
  - 25 FFI methods exposing tokens, connections, grid, and CDNA to Python
  - 4 Python wrapper classes: RuntimeTokenStorage, RuntimeConnectionStorage, RuntimeGridStorage, RuntimeCDNAStorage
  - Complete integration with Runtime class for seamless access
  - Full example demonstrating all RuntimeStorage features
  - Production-ready with comprehensive testing and documentation
  - See: [CHANGELOG v0.50.0](docs/changelogs/CHANGELOG_v0.50.0.md), [PROGRESS v0.50.0](docs/changelogs/PROGRESS_v0.50.0.md)

- **v0.49.0** — REST API Phase 2 Complete 🚀
  - Token, Grid, and CDNA routers with full CRUD operations
  - Pydantic models for request/response validation
  - Storage and models infrastructure
  - Single production API implementation (MVP removed)
  - See: [CHANGELOG v0.49.0](docs/changelogs/CHANGELOG_v0.49.0.md)

- **v0.47.0** — Python Library (Phase 1 Complete) 🐍
  - Complete Python package with PyO3 FFI bindings
  - Real semantic search using Grid KNN in 3D space
  - Bootstrap system for GloVe/Word2Vec embeddings with PCA projection
  - Query engine with exponential decay similarity scoring
  - Full test suite (88% coverage, 26/28 tests)
  - Working examples with visual similarity display
  - Incremental releases: v0.47.1 (setup) → v0.47.5 (final)
  - See: [CHANGELOG v0.47.0](docs/changelogs/CHANGELOG_v0.47.0.md)

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

## Note

Этот документ является архивом. Актуальную документацию см. в:
- [README.md](../../README.md) - Новый минималистичный README
- [docs/guides/](../guides/) - Подробные руководства
- [docs/changelogs/](../changelogs/) - История версий
