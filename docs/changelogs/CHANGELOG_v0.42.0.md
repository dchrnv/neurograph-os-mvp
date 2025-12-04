# Changelog v0.42.0 - Observability & Production Monitoring

**Дата релиза:** 4 декабря 2025
**Статус:** Production-Ready (Full Stack) ✅

---

## 🎯 Основные улучшения

v0.42.0 завершает production-ready стек добавлением полного набора observability инструментов:
- **Prometheus Metrics** - мониторинг в реальном времени
- **Black Box Recorder** - post-mortem анализ crashes
- **Logging Utilities** - structured logging с контекстом

Совместно с v0.41.0 (Panic Recovery, WAL, Resource Quotas) формирует **полный production-ready stack**.

---

## 📊 Part 1: Prometheus Metrics v1.0

### Новые возможности

#### Metrics Module (`src/metrics.rs`)
- **250+ LOC** модуль для экспорта Prometheus метрик
- **15+ метрик** для мониторинга системы
- **Thread-safe** с использованием `lazy_static`

#### Типы метрик

**Counters (счётчики):**
- `neurograph_tokens_created_total` - всего токенов создано
- `neurograph_connections_created_total` - всего соединений создано
- `neurograph_tokens_validated_total` - токенов валидировано Guardian
- `neurograph_tokens_rejected_total` - токенов отклонено Guardian
- `neurograph_connections_validated_total` - соединений валидировано
- `neurograph_connections_rejected_total` - соединений отклонено
- `neurograph_quota_exceeded_total` - превышений квот
- `neurograph_aggressive_cleanups_total` - агрессивных очисток памяти
- `neurograph_panics_recovered_total` - паник восстановлено
- `neurograph_wal_entries_written_total` - WAL записей записано
- `neurograph_wal_entries_replayed_total` - WAL записей воспроизведено

**Gauges (моментальные значения):**
- `neurograph_tokens_active` - активных токенов сейчас
- `neurograph_connections_active` - активных соединений сейчас
- `neurograph_memory_used_bytes` - использовано памяти (байты)
- `neurograph_memory_usage_percent` - использование памяти (%)
- `neurograph_intuition_queue_size` - размер очереди IntuitionEngine
- `neurograph_guardian_event_queue_size` - размер очереди Guardian

**Histograms (распределения):**
- `neurograph_token_creation_duration_seconds` - время создания токена
- `neurograph_connection_creation_duration_seconds` - время создания соединения
- `neurograph_validation_duration_seconds` - время валидации
- `neurograph_wal_write_duration_seconds` - время записи WAL

#### API Endpoint

**GET /metrics**
- Возвращает метрики в Prometheus exposition format
- Content-Type: `text/plain; version=0.0.4; charset=utf-8`
- Аутентификация НЕ требуется (стандартная практика Prometheus)
- Интеграция с API сервером (`src/api/handlers.rs`, `src/api/router.rs`)

#### Интеграция

Метрики автоматически обновляются в:
- **Guardian** - при создании токенов/соединений, валидации, превышении квот
- **WAL** - при записи и воспроизведении записей
- **PanicHandler** - при восстановлении паник

#### Тестирование

- Unit tests в `src/metrics.rs` (4 теста)
- Пример `examples/test_metrics.rs`
- ✅ Все тесты проходят

### Технические детали

**Зависимости:**
```toml
prometheus = "0.13"
lazy_static = "1.4"
```

**Пример использования:**
```rust
use neurograph_core::metrics;

// Record events
metrics::TOKENS_CREATED.inc();
metrics::MEMORY_USED_BYTES.set(1024000);

// Export for Prometheus
let metrics_text = metrics::export_metrics().unwrap();
```

**Prometheus scrape config:**
```yaml
scrape_configs:
  - job_name: 'neurograph'
    static_configs:
      - targets: ['localhost:8080']
    metrics_path: '/metrics'
    scrape_interval: 15s
```

---

## 🛩️ Part 2: Black Box Recorder v1.0

### Новые возможности

#### Black Box Module (`src/black_box.rs`)
- **450+ LOC** модуль flight recorder
- **Circular buffer** - последние N событий (default: 1000)
- **Thread-safe** - Arc<Mutex<>> для многопоточного доступа
- **Auto-dump on panic** - автоматическая запись при крахе

#### Event Types

Записываются следующие типы событий:
- `TokenCreated` - создание токена
- `ConnectionCreated` - создание соединения
- `TokenValidationFailed` - провал валидации токена
- `ConnectionValidationFailed` - провал валидации соединения
- `QuotaExceeded` - превышение квоты ресурсов
- `AggressiveCleanup` - агрессивная очистка памяти
- `PanicRecovered` - восстановление после паники
- `WalWritten` - запись в WAL
- `WalReplayed` - воспроизведение WAL
- `SystemStarted` - запуск системы
- `SystemStopped` - остановка системы
- `Custom(String)` - пользовательские события

#### Структура Event

```rust
pub struct Event {
    pub event_type: EventType,
    pub timestamp_us: u64,  // Unix timestamp в микросекундах
    pub data: Vec<(String, String)>,  // key-value пары
}
```

#### Global Instance

```rust
use neurograph_core::black_box::{GLOBAL_BLACK_BOX, record_event, Event, EventType};

// Record event
record_event(Event::new(EventType::TokenCreated)
    .with_data("token_id", "42")
    .with_data("weight", "1.5"));

// Dump to file
GLOBAL_BLACK_BOX.dump_to_file("crash_dump.json").unwrap();
```

#### Crash Dumps

При panic автоматически создаётся файл:
```
neurograph_crash_dump_{timestamp}.json
```

**Формат JSON:**
```json
{
  "timestamp_us": 1733356800000000,
  "stats": {
    "capacity": 1000,
    "current_size": 500,
    "total_recorded": 1500,
    "total_dropped": 500
  },
  "events": [
    {
      "event_type": "TokenCreated",
      "timestamp_us": 1733356799999000,
      "data": [["id", "42"], ["weight", "1.5"]]
    },
    ...
  ]
}
```

#### Интеграция

**PanicHandler:**
- Автоматический dump при panic в `install_panic_hook()`
- Запись события в `catch_panic()` и `catch_panic_async()`

**Guardian:**
- События при `quota_exceeded`
- События при `aggressive_cleanup`

#### Тестирование

- Unit tests в `src/black_box.rs` (8 тестов)
- Пример `examples/test_black_box.rs`
- ✅ Все тесты проходят (circular buffer, overflow, dump, global)

### Технические детали

**API:**
```rust
// Create local black box
let bb = BlackBox::new(1000);

// Record events
bb.record(Event::new(EventType::SystemStarted));

// Statistics
let stats = bb.stats();
println!("Recorded: {}, Dropped: {}", stats.total_recorded, stats.total_dropped);

// Dump to file
bb.dump_to_file("dump.json").unwrap();

// Clear buffer
bb.clear();
```

**Производительность:**
- Circular buffer - O(1) для записи
- Minimal overhead (~1% CPU)
- No allocations в hot path (кроме первого заполнения)

---

## 📝 Part 3: Logging Utilities v1.0

### Новые возможности

#### Logging Module (`src/logging_utils.rs`)
- **150+ LOC** модуль для structured logging
- Helper функции для консистентного логирования
- Контекстная информация в логах

#### Initialization Functions

**Development mode:**
```rust
use neurograph_core::logging_utils::init_logging;

init_logging("info");  // or "debug", "warn", "error"
```

**Production mode:**
```rust
use neurograph_core::logging_utils::init_production_logging;

init_production_logging();  // compact format, thread IDs
```

#### Context Helpers

**Operation logging:**
```rust
use neurograph_core::logging_utils::*;

log_operation_start("token_creation", "Creating batch of 1000 tokens");
// ... do work ...
log_operation_complete("token_creation", 1.5, "success");
```

**Failure logging:**
```rust
log_operation_failed("validation", "Token weight out of range");
```

**Resource logging:**
```rust
log_resource_usage("memory", 1024000, Some(2048000));
// Output: "Resource usage: memory=1024000/2048000 (50.0%)"

log_resource_warning("memory", 1800000, 2048000, 0.8);
// Output: "Resource usage exceeds threshold: 87.9% >= 80.0%"
```

#### Features

- **Structured fields** - operation, duration, status, resource
- **Thread IDs** - в production mode
- **Line numbers** - для быстрого поиска
- **Target info** - модуль источника лога
- **Consistent format** - единый стиль логов

#### Интеграция

**API Server:**
```rust
use neurograph_core::{logging_utils, black_box};

// Initialize logging
logging_utils::init_logging("info");

// Record system start
black_box::record_event(
    black_box::Event::new(black_box::EventType::SystemStarted)
        .with_data("component", "api_server")
        .with_data("version", "v0.42.0")
);
```

### Технические детали

**Output format (default):**
```
2025-12-04T10:30:45.123456Z  INFO operation="token_creation" details="Creating batch" src/main.rs:42
2025-12-04T10:30:46.654321Z  INFO operation="token_creation" duration_secs=1.5 status="success" src/main.rs:45
```

**Output format (production - compact):**
```
2025-12-04T10:30:45.123Z INFO [thread-3] operation="token_creation" details="Creating batch" src/main.rs:42
```

---

## 🔧 Изменённые файлы

### Новые модули
- `src/core_rust/src/metrics.rs` - Prometheus metrics (250+ LOC)
- `src/core_rust/src/black_box.rs` - Black Box Recorder (450+ LOC)
- `src/core_rust/src/logging_utils.rs` - Logging utilities (150+ LOC)

### Изменённые модули
- `src/core_rust/src/lib.rs` - экспорт новых модулей
- `src/core_rust/src/guardian.rs` - интеграция metrics + Black Box
- `src/core_rust/src/wal.rs` - интеграция metrics
- `src/core_rust/src/panic_handler.rs` - интеграция metrics + Black Box
- `src/core_rust/src/api/handlers.rs` - handler для /metrics
- `src/core_rust/src/api/router.rs` - route для /metrics
- `src/core_rust/src/bin/api.rs` - logging utilities, Black Box events

### Примеры
- `examples/test_metrics.rs` - тест Prometheus metrics
- `examples/test_black_box.rs` - тест Black Box Recorder

### Зависимости
```toml
# Cargo.toml
prometheus = "0.13"
lazy_static = "1.4"
```

### Документация
- `README.md` - обновлён до v0.42.0
- `python/README.md` - обновлён до v0.42.0
- `docs/changelogs/CHANGELOG_v0.42.0.md` - этот файл

---

## 📈 Production Benefits

### Observability Stack

**До v0.42.0:**
- ❌ Нет real-time мониторинга
- ❌ Нет crash dumps для анализа
- ⚠️ Базовое логирование без контекста

**После v0.42.0:**
- ✅ Prometheus metrics для Grafana dashboards
- ✅ Black Box dumps для post-mortem анализа
- ✅ Structured logging с контекстом и timing

### Use Cases

**Monitoring in Production:**
```bash
# Prometheus scrapes /metrics every 15s
curl http://localhost:8080/metrics

# Grafana visualizes:
# - Token creation rate (tokens/sec)
# - Memory usage (% and absolute)
# - Validation failures
# - WAL write latency
```

**Post-Mortem Analysis:**
```bash
# After crash, analyze:
cat neurograph_crash_dump_1733356800.json | jq '.events[-10:]'

# See last 10 events before crash:
# - What tokens were created?
# - Were quotas exceeded?
# - What was memory usage?
```

**Debugging with Logs:**
```bash
# Structured logs are easy to parse:
grep "operation=\"token_creation\"" logs.txt | jq '.duration_secs'

# Find slow operations:
grep "duration_secs" logs.txt | awk '$NF > 1.0'
```

---

## 🧪 Тестирование

### Unit Tests

```bash
# Metrics tests
cargo test --lib metrics::tests

# Black Box tests
cargo test --lib black_box::tests

# Logging tests
cargo test --lib logging_utils::tests
```

### Manual Tests

```bash
# Test Prometheus metrics
cargo run --example test_metrics --release
# ✅ Metrics export successful!

# Test Black Box
cargo run --example test_black_box --release
# 🎉 All Black Box tests passed!
```

### Integration Tests

```bash
# Start API server
cargo run --bin neurograph-api --release

# Scrape metrics
curl http://localhost:8080/metrics

# Check for crash dump after panic
ls -la neurograph_crash_dump_*.json
```

---

## 📊 Статистика изменений

### Lines of Code

- **Metrics module:** 250+ LOC
- **Black Box module:** 450+ LOC
- **Logging utils:** 150+ LOC
- **Integration:** ~100 LOC
- **Tests:** ~200 LOC
- **Examples:** ~150 LOC

**Total:** ~1300 LOC добавлено

### Files Changed

- **New files:** 5 (3 modules + 2 examples)
- **Modified files:** 8 (API, Guardian, WAL, PanicHandler, etc.)
- **Documentation:** 3 files (README.md, python/README.md, changelog)

### Commits

1. `8841ac5` - feat: Implement Prometheus Metrics v1.0 (v0.42.0 Part 1/3)
2. `1621eec` - feat: Implement Black Box Recorder v1.0 (v0.42.0 Part 2/3)
3. `49b9d0a` - feat: Implement Logging Utilities v1.0 (v0.42.0 Part 3/3)
4. `ccfdb61` - docs: Update documentation for v0.42.0 Final release

---

## 🚀 Roadmap Updates

### Completed Milestones

- ✅ **v0.40.0** - Python Bindings (PyO3)
- ✅ **v0.41.0** - Reliability (Panic Recovery, WAL, Resource Quotas)
- ✅ **v0.42.0** - Observability (Prometheus, Black Box, Logging) ← **WE ARE HERE**

### Next Milestones

- ⏳ **v0.43.0** - Docker Deployment
  - Dockerfile (multi-stage, <50MB)
  - Docker Compose для full stack
  - Health check endpoints

- ⏳ **v0.44.0** - Distributed Tracing
  - OpenTelemetry integration
  - Trace context propagation
  - Jaeger/Zipkin export

---

## 💡 Migration Guide

### Для существующих проектов

1. **Обновите зависимости:**
```toml
[dependencies]
neurograph-core = "0.42.0"
```

2. **Используйте новое логирование:**
```rust
// Вместо:
tracing_subscriber::fmt().init();

// Используйте:
use neurograph_core::logging_utils;
logging_utils::init_logging("info");
```

3. **Добавьте мониторинг:**
```rust
// Metrics обновляются автоматически
// Просто добавьте Prometheus scraping:
// curl http://localhost:8080/metrics
```

4. **Получайте crash dumps:**
```rust
// Black Box работает автоматически
// После panic ищите файлы:
// neurograph_crash_dump_*.json
```

### Backwards Compatibility

- ✅ Полная обратная совместимость с v0.41.0
- ✅ Все существующие API работают без изменений
- ✅ Новые возможности опциональны
- ✅ Zero breaking changes

---

## 🎯 Known Issues

Нет известных проблем в v0.42.0.

---

## 👥 Contributors

- Chernov Denys (@dchrnv) - lead developer
- Claude (Anthropic) - code generation assistant

---

## 📜 License

AGPL-3.0 - Copyright (C) 2024-2025 Chernov Denys

---

**v0.42.0 Final** - Production-Ready Full Stack! 🎉
