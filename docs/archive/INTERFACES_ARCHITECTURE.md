# Архитектура интерфейсов

**Версия:** 1.0.0
**Дата:** 2025-01-XX
**Статус:** Спецификация
**Автор:** Denis Chernov & Claude (Anthropic)

---

## Обзор

Данный документ описывает многоуровневую архитектуру интерфейсов для Tiro — экспериментальной системы семантических графов знаний. Архитектура построена на принципе разделения ответственности, где каждый слой может развиваться независимо.

### Философия

```
"Ядро должно быть универсальным.
 Интерфейсы — сменяемыми.
 API — стабильным."
```

---

## Архитектура слоёв

```
┌─────────────────────────────────────────────────────────────────────┐
│                         PRESENTATION LAYER                          │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌─────────────────┐  │
│  │  Web App  │  │ Telegram  │  │  Jupyter  │  │  Third-party    │  │
│  │ (Control  │  │   Bot     │  │ Notebook  │  │  Applications   │  │
│  │  Center)  │  │           │  │           │  │                 │  │
│  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘  └───────┬─────────┘  │
└────────┼──────────────┼──────────────┼────────────────┼────────────┘
         │              │              │                │
         └──────────────┴───────┬──────┴────────────────┘
                                │
┌───────────────────────────────┼───────────────────────────────────┐
│                        SERVICE LAYER                               │
│                               │                                    │
│                    ┌──────────▼──────────┐                        │
│                    │      REST API       │                        │
│                    │   FastAPI (Python)  │                        │
│                    │    /api/v1/*        │                        │
│                    └──────────┬──────────┘                        │
│                               │                                    │
│           ┌───────────────────┼───────────────────┐               │
│           │                   │                   │               │
│    ┌──────▼──────┐    ┌───────▼───────┐   ┌──────▼──────┐        │
│    │  WebSocket  │    │    Events     │   │   Metrics   │        │
│    │   Server    │    │    Stream     │   │  Collector  │        │
│    └─────────────┘    └───────────────┘   └─────────────┘        │
└───────────────────────────────┬───────────────────────────────────┘
                                │
┌───────────────────────────────┼───────────────────────────────────┐
│                       LIBRARY LAYER                                │
│                               │                                    │
│                    ┌──────────▼──────────┐                        │
│                    │     neurograph      │                        │
│                    │  Python Package     │                        │
│                    │      (PyPI)         │                        │
│                    └──────────┬──────────┘                        │
│                               │                                    │
│           ┌───────────────────┼───────────────────┐               │
│           │                   │                   │               │
│    ┌──────▼──────┐    ┌───────▼───────┐   ┌──────▼──────┐        │
│    │   Runtime   │    │    Query      │   │  Bootstrap  │        │
│    │   Manager   │    │    Engine     │   │   Loader    │        │
│    └─────────────┘    └───────────────┘   └─────────────┘        │
└───────────────────────────────┬───────────────────────────────────┘
                                │
┌───────────────────────────────┼───────────────────────────────────┐
│                         CORE LAYER                                 │
│                               │                                    │
│                    ┌──────────▼──────────┐                        │
│                    │    neurograph_core  │                        │
│                    │   Rust FFI (PyO3)   │                        │
│                    └──────────┬──────────┘                        │
│                               │                                    │
│    ┌──────────┬───────────────┼───────────────┬──────────┐        │
│    │          │               │               │          │        │
│ ┌──▼───┐ ┌────▼────┐ ┌────────▼────────┐ ┌────▼───┐ ┌────▼────┐  │
│ │Token │ │Connection│ │     Grid       │ │ Graph  │ │Guardian │  │
│ │ V2.0 │ │   V1.0  │ │     V2.0       │ │  V2.0  │ │  CDNA   │  │
│ └──────┘ └─────────┘ └─────────────────┘ └────────┘ └─────────┘  │
│                               │                                    │
│                    ┌──────────▼──────────┐                        │
│                    │   IntuitionEngine   │                        │
│                    │       V3.0          │                        │
│                    └────────────────────-┘                        │
└───────────────────────────────────────────────────────────────────┘
```

---

## Компоненты

### 1. Core Layer (Rust)

**Статус:** ✅ Реализован (v0.41.0)

Низкоуровневое ядро на Rust, обеспечивающее:

- Структуры данных фиксированного размера (Token 64B, Connection 32B)
- Высокопроизводительные операции (14.4 μs на событие)
- FFI через PyO3 для Python интеграции

**Модули:**

| Модуль         | Описание                                                   | Статус |
| -------------------- | ------------------------------------------------------------------ | ------------ |
| Token V2.0           | 8-мерное семантическое пространство | ✅           |
| Connection V1.0      | 40+ типов связей                                        | ✅           |
| Grid V2.0            | Пространственная индексация              | ✅           |
| Graph V2.0           | Обход и анализ графа                              | ✅           |
| Guardian CDNA        | Конституционные правила                      | ✅           |
| IntuitionEngine V3.0 | Быстрый/медленный путь                         | ✅           |

### 2. Library Layer (Python)

**Статус:** 🔄 В разработке

Python-пакет `neurograph`, предоставляющий:

- Высокоуровневый API для работы с графом
- Runtime Manager для управления жизненным циклом
- Bootstrap Loader для начальной загрузки данных
- Query Engine для семантических запросов

**Детали:** См. `NEUROGRAPH_LIBRARY_SPEC.md`

### 3. Service Layer (FastAPI)

**Статус:** 📋 Спецификация

REST API и сервисы:

- HTTP endpoints для всех операций
- WebSocket для real-time обновлений
- Event Stream для асинхронных событий
- Metrics Collector для мониторинга

**Детали:** См. `REST_API_SPEC.md`

### 4. Presentation Layer

**Статус:** 📋 Спецификация

Пользовательские интерфейсы:

- Web App (Control Center) — основной UI
- Telegram Bot — мобильный доступ
- Jupyter Integration — исследования
- Third-party — внешние приложения

**Детали:** См. `WEB_DASHBOARD_SPEC.md`, `TELEGRAM_BOT_SPEC.md`, `JUPYTER_INTEGRATION.md`

---

## Потоки данных

### Query Flow (Запрос)

```
User Input
    │
    ▼
┌─────────────────┐
│ Presentation    │  "что ты знаешь о кошках?"
│ (any interface) │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ REST API        │  POST /api/v1/query
│ /api/v1/query   │  {"text": "что ты знаешь о кошках?"}
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ neurograph      │  ng.query("кошка")
│ QueryEngine     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ neurograph_core │  Gateway → ActionController → Graph
│ (Rust FFI)      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Response        │  ActivationResult {
│                 │    tokens: [...],
│                 │    connections: [...],
│                 │    confidence: 0.87
│                 │  }
└─────────────────┘
```

### Metrics Flow (Мониторинг)

```
┌─────────────────┐
│ Rust Core       │  InternalMetrics {
│ (каждые 100ms)  │    events_per_sec: 14234,
└────────┬────────┘    memory_usage: 847MB,
         │             active_tokens: 12847
         │           }
         ▼
┌─────────────────┐
│ MetricsCollector│  Агрегация, буферизация
│ (Python)        │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
    ▼         ▼
┌───────┐ ┌───────────┐
│WebSock│ │Prometheus │
│Stream │ │ /metrics  │
└───────┘ └───────────┘
```

### Bootstrap Flow (Инициализация)

```
┌─────────────────┐
│ GloVe/Word2Vec  │  300-мерные эмбеддинги
│ embeddings.txt  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ BootstrapLoader │  Проекция 300D → 8D
│ (Python)        │  Кластеризация
└────────┬────────┘  Создание связей
         │
         ▼
┌─────────────────┐
│ neurograph_core │  Batch insert
│ Token.batch_create()
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Ready State     │  50K+ tokens
│                 │  200K+ connections
└─────────────────┘
```

---

## Порядок реализации

### Фаза 1: Фундамент (Неделя 1-2)

```
1. neurograph library     ████████████████████  [4-6ч]
   └─ Runtime, Query, Bootstrap basics

2. REST API              ████████████████████  [3-4ч]
   └─ Core endpoints, health, status

3. Jupyter integration   ████████████████████  [2-3ч]
   └─ Display, visualize helpers
```

### Фаза 2: Интерфейсы (Неделя 3-4)

```
4. Web Dashboard         ████████████████████  [8-12ч]
   └─ React + готовый шаблон

5. Telegram Bot          ████████████████████  [3-4ч]
   └─ python-telegram-bot
```

### Фаза 3: Продакшн (Неделя 5-6)

```
6. Metrics & Monitoring  ████████████████████  [4-6ч]
   └─ Prometheus, Grafana

7. Docker packaging      ████████████████████  [2-3ч]
   └─ docker-compose

8. Documentation         ████████████████████  [2-3ч]
   └─ API docs, user guide
```

---

## Технологический стек

### Backend

| Компонент | Технология | Версия | Обоснование                                         |
| ------------------ | -------------------- | ------------ | -------------------------------------------------------------- |
| Core               | Rust                 | 1.75+        | Производительность, безопасность |
| FFI                | PyO3                 | 0.20+        | Лучшие Python биндинги для Rust               |
| API                | FastAPI              | 0.109+       | Async, автодокументация, типизация    |
| WebSocket          | Starlette            | 0.35+        | Встроен в FastAPI                                      |
| Cache              | Redis                | 7.0+         | Опционально, для масштабирования  |

### Frontend (Web Dashboard)

| Компонент | Технология | Версия | Обоснование                     |
| ------------------ | -------------------- | ------------ | ------------------------------------------ |
| Framework          | React                | 18+          | Экосистема, компоненты |
| UI Kit             | Ant Design Pro       | 6+           | Готовые admin компоненты  |
| State              | Zustand              | 4+           | Простой, TypeScript                 |
| Charts             | Recharts             | 2+           | React-native графики                |
| Terminal           | xterm.js             | 5+           | Полноценный терминал    |

### Infrastructure

| Компонент | Технология | Обоснование                  |
| ------------------ | -------------------- | --------------------------------------- |
| Container          | Docker               | Стандарт индустрии     |
| Orchestration      | docker-compose       | Простота для MVP             |
| Metrics            | Prometheus           | Стандарт мониторинга |
| Dashboards         | Grafana              | Визуализация метрик   |

---

## API Контракты

### Версионирование

```
/api/v1/*  — текущая стабильная версия
/api/v2/*  — будущая версия (breaking changes)
```

### Формат ответов

```json
{
  "success": true,
  "data": { ... },
  "meta": {
    "request_id": "uuid",
    "processing_time_ms": 14.2,
    "version": "1.0.0"
  },
  "error": null
}
```

### Формат ошибок

```json
{
  "success": false,
  "data": null,
  "meta": { ... },
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Query text is required",
    "details": { "field": "text" }
  }
}
```

---

## Безопасность

### Уровни доступа

| Уровень | Права                  | Применение                                            |
| -------------- | --------------------------- | --------------------------------------------------------------- |
| `public`     | Только чтение   | Внешние запросы                                   |
| `user`       | Чтение + запись | Аутентифицированные пользователи |
| `admin`      | Полный доступ   | Системные операции                             |
| `root`       | CDNA модификация | Только локально                                   |

### Аутентификация

```
Authorization: Bearer <jwt_token>
```

Для локальной разработки — опционально.

---

## Мониторинг

### Health Endpoints

```
GET /health          → {"status": "healthy"}
GET /health/ready    → {"ready": true, "checks": {...}}
GET /health/live     → {"alive": true}
```

### Prometheus Metrics

```
GET /metrics

# HELP tiro_events_total Total processed events
# TYPE tiro_events_total counter
tiro_events_total{type="query"} 12847

# HELP tiro_latency_seconds Processing latency
# TYPE tiro_latency_seconds histogram
tiro_latency_seconds_bucket{le="0.001"} 9823
```

---

## Файловая структура

```
tiro/
├── src/
│   ├── core_rust/           # Rust ядро (существует)
│   │   └── ...
│   │
│   ├── neurograph/          # Python библиотека (новое)
│   │   ├── __init__.py
│   │   ├── runtime.py
│   │   ├── query.py
│   │   ├── bootstrap.py
│   │   └── utils.py
│   │
│   ├── api/                 # REST API (новое)
│   │   ├── __init__.py
│   │   ├── main.py
│   │   ├── routers/
│   │   │   ├── query.py
│   │   │   ├── tokens.py
│   │   │   ├── system.py
│   │   │   └── metrics.py
│   │   ├── models/
│   │   ├── services/
│   │   └── middleware/
│   │
│   ├── web/                 # Web Dashboard (новое)
│   │   ├── package.json
│   │   ├── src/
│   │   │   ├── pages/
│   │   │   ├── components/
│   │   │   └── services/
│   │   └── public/
│   │
│   └── telegram/            # Telegram Bot (новое)
│       ├── __init__.py
│       ├── bot.py
│       └── handlers/
│
├── docs/
│   ├── TIRO_INTERFACES_ARCHITECTURE.md  # Этот документ
│   ├── NEUROGRAPH_LIBRARY_SPEC.md
│   ├── REST_API_SPEC.md
│   ├── WEB_DASHBOARD_SPEC.md
│   ├── TELEGRAM_BOT_SPEC.md
│   └── JUPYTER_INTEGRATION.md
│
├── docker/
│   ├── Dockerfile.api
│   ├── Dockerfile.web
│   └── docker-compose.yml
│
└── tests/
    ├── test_library/
    ├── test_api/
    └── test_integration/
```

---

## Следующие документы

1. **NEUROGRAPH_LIBRARY_SPEC.md** — детальная спецификация Python библиотеки
2. **REST_API_SPEC.md** — все endpoints, модели, примеры
3. **WEB_DASHBOARD_SPEC.md** — структура UI, компоненты, экраны
4. **JUPYTER_INTEGRATION.md** — магические команды, визуализация
5. **TELEGRAM_BOT_SPEC.md** — команды, обработчики, сценарии

---

## Changelog

| Версия | Дата   | Изменения              |
| ------------ | ---------- | ------------------------------- |
| 1.0.0        | 2025-01-XX | Начальная версия |

---

**Tiro Interfaces Architecture v1.0.0**
*Многоуровневая архитектура для когнитивной системы*
