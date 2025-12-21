# REST API Specification

**Версия:** 1.0.0  
**Дата:** 2025-01-XX  
**Статус:** Спецификация  
**Framework:** FastAPI  
**Base URL:** `http://localhost:8000/api/v1`

---

## Обзор

REST API предоставляет программный доступ к Tiro для внешних приложений, включая Web Dashboard, Telegram Bot и сторонние интеграции.

### Принципы

- **RESTful** — стандартные HTTP методы и статус-коды
- **JSON** — единый формат данных
- **Версионирование** — `/api/v1/`, `/api/v2/`
- **OpenAPI** — автогенерируемая документация

---

## Аутентификация

### Development Mode (по умолчанию)

```
Без аутентификации для localhost
```

### Production Mode

```http
Authorization: Bearer <jwt_token>
```

### Получение токена

```http
POST /api/v1/auth/token
Content-Type: application/json

{
  "username": "admin",
  "password": "secret"
}
```

Response:
```json
{
  "access_token": "eyJ...",
  "token_type": "bearer",
  "expires_in": 3600
}
```

---

## Формат ответов

### Успешный ответ

```json
{
  "success": true,
  "data": { ... },
  "meta": {
    "request_id": "550e8400-e29b-41d4-a716-446655440000",
    "processing_time_ms": 14.2,
    "version": "1.0.0",
    "timestamp": "2025-01-25T12:34:56Z"
  }
}
```

### Ошибка

```json
{
  "success": false,
  "data": null,
  "meta": { ... },
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Query text is required",
    "details": {
      "field": "text",
      "constraint": "non_empty"
    }
  }
}
```

### Коды ошибок

| Code | HTTP Status | Описание |
|------|-------------|----------|
| `VALIDATION_ERROR` | 400 | Некорректные входные данные |
| `NOT_FOUND` | 404 | Ресурс не найден |
| `UNAUTHORIZED` | 401 | Требуется аутентификация |
| `FORBIDDEN` | 403 | Недостаточно прав |
| `INTERNAL_ERROR` | 500 | Внутренняя ошибка |
| `SERVICE_UNAVAILABLE` | 503 | Сервис недоступен |

---

## Endpoints

### System

#### GET /health

Проверка здоровья сервиса.

```http
GET /api/v1/health
```

Response `200 OK`:
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "uptime_seconds": 3600,
    "version": "1.0.0"
  }
}
```

#### GET /health/ready

Готовность к обработке запросов.

```http
GET /api/v1/health/ready
```

Response `200 OK`:
```json
{
  "success": true,
  "data": {
    "ready": true,
    "checks": {
      "runtime": "ok",
      "bootstrap": "ok",
      "gateway": "ok"
    }
  }
}
```

#### GET /status

Полный статус системы.

```http
GET /api/v1/status
```

Response:
```json
{
  "success": true,
  "data": {
    "state": "running",
    "uptime_seconds": 3600.5,
    "tokens": {
      "total": 50000,
      "active": 12847
    },
    "connections": {
      "total": 1247832,
      "active": 523419
    },
    "memory_usage_mb": 847.3,
    "cpu_usage_percent": 23.5,
    "components": {
      "runtime": "running",
      "gateway": "running",
      "intuition_engine": "running",
      "guardian": "running"
    }
  }
}
```

#### GET /stats

Статистика работы.

```http
GET /api/v1/stats
```

Response:
```json
{
  "success": true,
  "data": {
    "queries": {
      "total": 12847,
      "per_second": 14.2,
      "avg_latency_ms": 12.3
    },
    "feedbacks": {
      "total": 523,
      "positive": 412,
      "negative": 89,
      "corrections": 22
    },
    "cache": {
      "hits": 8934,
      "misses": 3913,
      "hit_rate": 0.695
    },
    "intuition": {
      "fast_path_hits": 7234,
      "slow_path_hits": 5613,
      "fast_path_rate": 0.563
    }
  }
}
```

---

### Query

#### POST /query

Семантический запрос.

```http
POST /api/v1/query
Content-Type: application/json

{
  "text": "кошка",
  "limit": 10,
  "threshold": 0.5,
  "spaces": ["L1Physical", "L8Abstract"],
  "include_connections": true
}
```

Response:
```json
{
  "success": true,
  "data": {
    "signal_id": 12847,
    "query_text": "кошка",
    "processing_time_ms": 14.2,
    "confidence": 0.87,
    "interpretation": "semantic_query",
    
    "tokens": [
      {
        "token_id": 4523,
        "label": "кот",
        "score": 0.92,
        "entity_type": "Concept",
        "coordinates": {
          "L1Physical": [0.12, 0.34, 0.56],
          "L8Abstract": [0.78, 0.90, 0.12]
        }
      },
      {
        "token_id": 4524,
        "label": "котёнок",
        "score": 0.87,
        "entity_type": "Concept",
        "coordinates": { ... }
      }
    ],
    
    "connections": [
      {
        "source_id": 4523,
        "target_id": 4524,
        "connection_type": "Related",
        "strength": 0.85
      }
    ],
    
    "total_candidates": 247
  }
}
```

#### POST /query/batch

Пакетный запрос.

```http
POST /api/v1/query/batch
Content-Type: application/json

{
  "queries": [
    {"text": "кошка", "limit": 5},
    {"text": "собака", "limit": 5},
    {"text": "птица", "limit": 5}
  ],
  "parallel": true
}
```

Response:
```json
{
  "success": true,
  "data": {
    "results": [
      { "query_text": "кошка", "tokens": [...], ... },
      { "query_text": "собака", "tokens": [...], ... },
      { "query_text": "птица", "tokens": [...], ... }
    ],
    "total_processing_time_ms": 42.5
  }
}
```

---

### Tokens

#### GET /tokens/{token_id}

Получение токена по ID.

```http
GET /api/v1/tokens/4523
```

Response:
```json
{
  "success": true,
  "data": {
    "token_id": 4523,
    "label": "кот",
    "weight": 1.5,
    "entity_type": "Concept",
    "flags": {
      "active": true,
      "persistent": true,
      "mutable": true,
      "system": false
    },
    "coordinates": {
      "L1Physical": [0.12, 0.34, 0.56],
      "L2Sensory": [0.23, 0.45, 0.67],
      "L3Motor": [0.0, 0.0, 0.0],
      "L4Emotional": [0.78, 0.56, 0.67],
      "L5Cognitive": [0.89, 0.12, 0.34],
      "L6Social": [0.45, 0.67, 0.89],
      "L7Temporal": [0.0, 0.0, 0.0],
      "L8Abstract": [0.78, 0.90, 0.12]
    },
    "field_radius": 10,
    "field_strength": 128,
    "connections_count": 47,
    "created_at": "2025-01-25T10:00:00Z",
    "last_activation": "2025-01-25T12:34:56Z"
  }
}
```

#### GET /tokens/{token_id}/connections

Получение связей токена.

```http
GET /api/v1/tokens/4523/connections?limit=20&type=Synonym
```

Response:
```json
{
  "success": true,
  "data": {
    "token_id": 4523,
    "connections": [
      {
        "connection_id": 78234,
        "target_id": 4524,
        "target_label": "котёнок",
        "connection_type": "Related",
        "direction": "outgoing",
        "strength": 0.85,
        "activation_count": 127
      }
    ],
    "total": 47,
    "returned": 20
  }
}
```

#### GET /tokens/search

Поиск токенов по параметрам.

```http
GET /api/v1/tokens/search?label=кот&entity_type=Concept&limit=10
```

Response:
```json
{
  "success": true,
  "data": {
    "tokens": [
      {"token_id": 4523, "label": "кот", ...},
      {"token_id": 4525, "label": "котик", ...}
    ],
    "total": 2
  }
}
```

---

### Feedback

#### POST /feedback

Обратная связь на результат.

```http
POST /api/v1/feedback
Content-Type: application/json

{
  "signal_id": 12847,
  "feedback_type": "positive",
  "tokens_relevant": [4523, 4524],
  "tokens_irrelevant": [4530],
  "comment": "Хорошие результаты"
}
```

Response:
```json
{
  "success": true,
  "data": {
    "feedback_id": 523,
    "applied": true,
    "adjustments": {
      "weights_updated": 3,
      "connections_strengthened": 2,
      "connections_weakened": 1
    }
  }
}
```

#### POST /feedback/correction

Коррекция результата.

```http
POST /api/v1/feedback/correction
Content-Type: application/json

{
  "signal_id": 12847,
  "expected_tokens": ["кот", "котёнок", "животное"],
  "unexpected_tokens": ["автомобиль"]
}
```

---

### Bootstrap

#### GET /bootstrap/status

Статус bootstrap.

```http
GET /api/v1/bootstrap/status
```

Response:
```json
{
  "success": true,
  "data": {
    "loaded": true,
    "source": "glove.6B.100d.txt",
    "tokens_loaded": 50000,
    "connections_created": 1247832,
    "loaded_at": "2025-01-25T10:00:00Z"
  }
}
```

#### POST /bootstrap/load

Загрузка bootstrap данных.

```http
POST /api/v1/bootstrap/load
Content-Type: application/json

{
  "source": "/data/embeddings/glove.6B.100d.txt",
  "format": "glove",
  "limit": 50000,
  "projection_method": "pca",
  "create_connections": true
}
```

Response:
```json
{
  "success": true,
  "data": {
    "task_id": "bootstrap_001",
    "status": "started",
    "estimated_time_sec": 120
  }
}
```

#### GET /bootstrap/progress/{task_id}

Прогресс загрузки.

```http
GET /api/v1/bootstrap/progress/bootstrap_001
```

Response:
```json
{
  "success": true,
  "data": {
    "task_id": "bootstrap_001",
    "status": "in_progress",
    "progress": 0.45,
    "tokens_processed": 22500,
    "tokens_total": 50000,
    "eta_seconds": 65
  }
}
```

---

### Modules

#### GET /modules

Список модулей.

```http
GET /api/v1/modules
```

Response:
```json
{
  "success": true,
  "data": {
    "modules": [
      {
        "name": "TokenManager",
        "status": "running",
        "version": "2.0.0",
        "stats": {
          "tokens_managed": 50000,
          "operations_per_sec": 12847
        }
      },
      {
        "name": "ConnectionPool",
        "status": "running",
        "version": "1.0.0",
        "stats": { ... }
      },
      {
        "name": "GridIndex",
        "status": "running",
        "version": "2.0.0",
        "stats": { ... }
      },
      {
        "name": "GraphEngine",
        "status": "running",
        "version": "2.0.0",
        "stats": { ... }
      },
      {
        "name": "IntuitionEngine",
        "status": "running",
        "version": "3.0.0",
        "stats": {
          "fast_path_rate": 0.56,
          "avg_latency_us": 69.5
        }
      },
      {
        "name": "Guardian",
        "status": "running",
        "version": "1.0.0",
        "stats": { ... }
      }
    ]
  }
}
```

#### GET /modules/{name}

Детали модуля.

```http
GET /api/v1/modules/IntuitionEngine
```

#### POST /modules/{name}/restart

Перезапуск модуля (требует admin).

```http
POST /api/v1/modules/IntuitionEngine/restart
```

---

### Config

#### GET /config

Текущая конфигурация.

```http
GET /api/v1/config
```

Response:
```json
{
  "success": true,
  "data": {
    "runtime": {
      "max_tokens": 1000000,
      "max_connections": 10000000
    },
    "gateway": {
      "queue_capacity": 10000,
      "processing_timeout_ms": 100
    },
    "intuition": {
      "enable_fast_path": true,
      "fast_path_threshold": 0.8
    }
  }
}
```

#### PATCH /config

Обновление конфигурации (требует admin).

```http
PATCH /api/v1/config
Content-Type: application/json

{
  "gateway": {
    "processing_timeout_ms": 200
  }
}
```

---

### Metrics

#### GET /metrics

Prometheus-совместимые метрики.

```http
GET /api/v1/metrics
Accept: text/plain
```

Response:
```
# HELP tiro_queries_total Total number of queries
# TYPE tiro_queries_total counter
tiro_queries_total 12847

# HELP tiro_query_latency_seconds Query latency
# TYPE tiro_query_latency_seconds histogram
tiro_query_latency_seconds_bucket{le="0.001"} 2345
tiro_query_latency_seconds_bucket{le="0.01"} 10234
tiro_query_latency_seconds_bucket{le="0.1"} 12800
tiro_query_latency_seconds_bucket{le="+Inf"} 12847
tiro_query_latency_seconds_sum 145.67
tiro_query_latency_seconds_count 12847

# HELP tiro_tokens_total Total tokens in system
# TYPE tiro_tokens_total gauge
tiro_tokens_total 50000

# HELP tiro_memory_usage_bytes Memory usage
# TYPE tiro_memory_usage_bytes gauge
tiro_memory_usage_bytes 888741888
```

#### GET /metrics/json

Метрики в JSON формате.

```http
GET /api/v1/metrics/json
```

---

### WebSocket

#### WS /ws/events

Подписка на события в реальном времени.

```javascript
const ws = new WebSocket('ws://localhost:8000/api/v1/ws/events');

ws.onopen = () => {
  ws.send(JSON.stringify({
    type: 'subscribe',
    channels: ['queries', 'metrics', 'system']
  }));
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log(data);
  // { channel: 'metrics', event: 'update', data: {...} }
};
```

Channels:
- `queries` — новые запросы и результаты
- `metrics` — обновления метрик (каждые 1с)
- `system` — системные события (startup, shutdown, errors)
- `feedback` — обратная связь
- `bootstrap` — прогресс загрузки

---

### Admin (требует root)

#### POST /admin/cdna/reload

Перезагрузка CDNA.

```http
POST /api/v1/admin/cdna/reload
```

#### POST /admin/shutdown

Остановка системы.

```http
POST /api/v1/admin/shutdown
Content-Type: application/json

{
  "graceful": true,
  "timeout_sec": 30
}
```

#### POST /admin/reset

Сброс системы.

```http
POST /api/v1/admin/reset
Content-Type: application/json

{
  "preserve_config": true,
  "preserve_bootstrap": false
}
```

---

## Rate Limiting

| Endpoint | Limit | Window |
|----------|-------|--------|
| `/query` | 100 req | 1 min |
| `/query/batch` | 10 req | 1 min |
| `/feedback` | 60 req | 1 min |
| `/bootstrap/load` | 1 req | 5 min |
| Other | 1000 req | 1 min |

Headers:
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 87
X-RateLimit-Reset: 1706180096
```

---

## OpenAPI Schema

Автогенерируемая документация доступна:

- Swagger UI: `http://localhost:8000/docs`
- ReDoc: `http://localhost:8000/redoc`
- OpenAPI JSON: `http://localhost:8000/openapi.json`

---

## Примеры использования

### cURL

```bash
# Status
curl http://localhost:8000/api/v1/status

# Query
curl -X POST http://localhost:8000/api/v1/query \
  -H "Content-Type: application/json" \
  -d '{"text": "кошка", "limit": 5}'

# Feedback
curl -X POST http://localhost:8000/api/v1/feedback \
  -H "Content-Type: application/json" \
  -d '{"signal_id": 12847, "feedback_type": "positive"}'
```

### Python (requests)

```python
import requests

BASE_URL = "http://localhost:8000/api/v1"

# Query
response = requests.post(
    f"{BASE_URL}/query",
    json={"text": "кошка", "limit": 10}
)
result = response.json()

for token in result["data"]["tokens"]:
    print(f"{token['label']}: {token['score']:.2f}")
```

### JavaScript (fetch)

```javascript
const BASE_URL = 'http://localhost:8000/api/v1';

async function query(text) {
  const response = await fetch(`${BASE_URL}/query`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ text, limit: 10 })
  });
  return response.json();
}

const result = await query('кошка');
console.log(result.data.tokens);
```

---

## Следующий документ

→ **WEB_DASHBOARD_SPEC.md** — структура Web UI

---

## Changelog

| Версия | Дата | Изменения |
|--------|------|-----------|
| 1.0.0 | 2025-01-XX | Начальная версия |

---

**REST API Specification v1.0.0**  
*HTTP interface для Tiro*
