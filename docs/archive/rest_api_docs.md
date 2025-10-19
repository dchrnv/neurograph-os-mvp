# NeuroGraph OS - Документация REST API

Полный справочник REST API для NeuroGraph OS.

## 📋 Оглавление

- [Обзор](#обзор)
- [Аутентификация](#аутентификация)
- [Endpoints](#endpoints)
- [Обработка ошибок](#обработка-ошибок)
- [Примеры](#примеры)

---

## 🌐 Обзор

### Базовый URL

```
http://localhost:8000/api/v1
```

### Формат ответов

Все ответы возвращаются в формате JSON.

### Коды состояния

- `200` - Успех
- `201` - Создано
- `204` - Нет содержимого (Удаление)
- `400` - Неверный запрос
- `404` - Не найдено
- `500` - Внутренняя ошибка сервера

---

## 🔐 Аутентификация

В данный момент API не требует аутентификации. Для production окружения рекомендуется реализовать JWT токены.

---

## 📡 Endpoints

### Системные Endpoints

#### `GET /api/v1/system/health`

Проверка состояния системы.

**Ответ:**
```json
{
  "status": "healthy",
  "version": "0.3.0",
  "timestamp": "2025-01-15T10:30:00",
  "services": {
    "postgres": "up",
    "websocket": "up"
  }
}
```

#### `GET /api/v1/system/stats`

Получить статистику системы.

**Ответ:**
```json
{
  "tokens": 150,
  "connections": 230,
  "experience_events": 450,
  "websocket_connections": 5,
  "uptime_seconds": 3600.0
}
```

#### `GET /api/v1/system/info`

Получить информацию о системе.

---

### Endpoints токенов

#### `POST /api/v1/tokens/`

Создать новый токен.

**Тело запроса:**
```json
{
  "type": "demo",
  "coordinates": {
    "x": [1.0, 0.5, 0.25, 0.125, 0.0625, 0.03125, 0.015625, 0.0078125],
    "y": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    "z": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
  },
  "weight": 1.0,
  "flags": 0,
  "metadata": {
    "description": "Демонстрационный токен"
  }
}
```

**Ответ: `201 Created`**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "type": "demo",
  "coordinates": {...},
  "weight": 1.0,
  "flags": 0,
  "timestamp": 1705315800000,
  "metadata": {...},
  "created_at": "2025-01-15T10:30:00",
  "updated_at": null
}
```

#### `GET /api/v1/tokens/`

Получить список токенов с пагинацией.

**Параметры запроса:**
- `limit` (int, 1-100): Максимальное количество токенов (по умолчанию: 10)
- `offset` (int): Количество пропускаемых токенов (по умолчанию: 0)
- `token_type` (string): Фильтр по типу

**Ответ:**
```json
{
  "tokens": [...],
  "total": 150,
  "limit": 10,
  "offset": 0
}
```

#### `GET /api/v1/tokens/{token_id}`

Получить токен по ID.

**Ответ:**
```json
{
  "id": "550e8400-...",
  "type": "demo",
  "coordinates": {...},
  "weight": 1.0,
  ...
}
```

#### `PUT /api/v1/tokens/{token_id}`

Обновить токен.

**Тело запроса:**
```json
{
  "type": "updated",
  "weight": 2.0,
  "metadata": {"updated": true}
}
```

#### `DELETE /api/v1/tokens/{token_id}`

Удалить токен.

**Ответ: `204 No Content`**

#### `POST /api/v1/tokens/search/spatial`

Поиск токенов в пространственном регионе.

**Тело запроса:**
```json
{
  "min_x": 0.0,
  "min_y": 0.0,
  "min_z": 0.0,
  "max_x": 10.0,
  "max_y": 10.0,
  "max_z": 10.0,
  "level": 0
}
```

**Ответ:**
```json
{
  "tokens": [...],
  "total": 15,
  "limit": 15,
  "offset": 0
}
```

#### `POST /api/v1/tokens/batch`

Создать несколько токенов пакетом.

**Тело запроса:**
```json
{
  "tokens": [
    {
      "type": "batch1",
      "coordinates": {...},
      "weight": 1.0
    },
    {
      "type": "batch2",
      "coordinates": {...},
      "weight": 1.5
    }
  ]
}
```

**Ответ:**
```json
{
  "created": [...],
  "failed": [],
  "total_created": 2,
  "total_failed": 0
}
```

#### `GET /api/v1/tokens/count/total`

Подсчитать количество токенов.

**Параметры запроса:**
- `token_type` (string): Фильтр по типу

**Ответ:**
```json
{
  "count": 150,
  "type": "demo"
}
```

---

### Endpoints графа

#### `POST /api/v1/graph/connections`

Создать связь между токенами.

**Тело запроса:**
```json
{
  "source_id": "550e8400-...",
  "target_id": "660e9500-...",
  "connection_type": "spatial",
  "weight": 0.8,
  "decay_rate": 0.0,
  "bidirectional": false,
  "metadata": {}
}
```

**Ответ: `201 Created`**
```json
{
  "id": "770f1600-...",
  "source_id": "550e8400-...",
  "target_id": "660e9500-...",
  "connection_type": "spatial",
  "weight": 0.8,
  "decay_rate": 0.0,
  "bidirectional": false,
  "metadata": {},
  "generation": 0,
  "fitness": 0.0,
  "created_at": "2025-01-15T10:30:00",
  "updated_at": null
}
```

#### `GET /api/v1/graph/connections`

Получить список связей.

**Параметры запроса:**
- `limit`, `offset`, `connection_type`

#### `GET /api/v1/graph/connections/{connection_id}`

Получить связь по ID.

#### `PUT /api/v1/graph/connections/{connection_id}`

Обновить связь.

**Тело запроса:**
```json
{
  "weight": 0.9,
  "metadata": {"updated": true}
}
```

#### `DELETE /api/v1/graph/connections/{connection_id}`

Удалить связь.

#### `GET /api/v1/graph/tokens/{token_id}/neighbors`

Получить соседей токена.

**Параметры запроса:**
- `direction` (string): "incoming", "outgoing", "both"
- `connection_type` (string): Фильтр по типу связи

**Ответ:**
```json
{
  "token_id": "550e8400-...",
  "neighbors": [...],
  "count": 3,
  "direction": "both"
}
```

#### `GET /api/v1/graph/tokens/{token_id}/degree`

Получить степень токена (количество связей).

**Ответ:**
```json
{
  "token_id": "550e8400-...",
  "in_degree": 2,
  "out_degree": 3,
  "total_degree": 5
}
```

#### `GET /api/v1/graph/path`

Найти пути между токенами.

**Параметры запроса:**
- `source_id` (UUID): Исходный токен
- `target_id` (UUID): Целевой токен
- `max_depth` (int, 1-10): Максимальная глубина поиска

**Ответ:**
```json
{
  "source_id": "550e8400-...",
  "target_id": "660e9500-...",
  "paths": [
    ["550e8400-...", "770f1600-...", "660e9500-..."],
    ["550e8400-...", "880g2700-...", "660e9500-..."]
  ],
  "count": 2
}
```

#### `GET /api/v1/graph/stats`

Получить статистику графа.

**Ответ:**
```json
{
  "total_nodes": 150,
  "total_edges": 230,
  "avg_degree": 3.07,
  "density": 0.0103,
  "connected_components": null
}
```

#### `POST /api/v1/graph/connections/batch`

Создать несколько связей пакетом.

---

### Endpoints опыта

#### `POST /api/v1/experience/events`

Создать событие опыта.

**Тело запроса:**
```json
{
  "event_type": "action_taken",
  "token_id": "550e8400-...",
  "state_before": {"value": 0},
  "state_after": {"value": 1},
  "action": {"type": "increment"},
  "reward": 1.0,
  "metadata": {},
  "priority": 1.0
}
```

**Ответ: `201 Created`**
```json
{
  "id": "990h3800-...",
  "event_type": "action_taken",
  "timestamp": 1705315800000,
  "token_id": "550e8400-...",
  "state_before": {"value": 0},
  "state_after": {"value": 1},
  "action": {"type": "increment"},
  "reward": 1.0,
  "metadata": {},
  "priority": 1.0,
  "trajectory_id": null,
  "sequence_number": 0,
  "created_at": "2025-01-15T10:30:00"
}
```

#### `GET /api/v1/experience/events`

Получить список событий опыта.

**Параметры запроса:**
- `limit`, `offset`, `event_type`

#### `GET /api/v1/experience/events/{event_id}`

Получить событие по ID.

#### `GET /api/v1/experience/events/token/{token_id}`

Получить события для конкретного токена.

**Параметры запроса:**
- `limit` (int): Максимальное количество событий

#### `DELETE /api/v1/experience/events/cleanup`

Очистить старые события.

**Параметры запроса:**
- `retention_days` (int, 1-365): Хранить события за последние N дней

**Ответ:**
```json
{
  "deleted": 45,
  "retention_days": 30
}
```

---

## ⚠️ Обработка ошибок

### Формат ответа с ошибкой

```json
{
  "detail": "Сообщение об ошибке",
  "status_code": 404
}
```

### Типичные ошибки

**400 Неверный запрос**
```json
{
  "detail": "Ошибка валидации: поле 'coordinates' обязательно"
}
```

**404 Не найдено**
```json
{
  "detail": "Токен 550e8400-... не найден"
}
```

**500 Внутренняя ошибка сервера**
```json
{
  "detail": "Ошибка подключения к базе данных"
}
```

---

## 💻 Примеры

### Примеры с cURL

**Создать токен:**
```bash
curl -X POST http://localhost:8000/api/v1/tokens/ \
  -H "Content-Type: application/json" \
  -d '{
    "type": "demo",
    "coordinates": {
      "x": [1.0,0,0,0,0,0,0,0],
      "y": [0,0,0,0,0,0,0,0],
      "z": [0,0,0,0,0,0,0,0]
    }
  }'
```

**Получить список токенов:**
```bash
curl http://localhost:8000/api/v1/tokens/?limit=5
```

**Получить токен:**
```bash
curl http://localhost:8000/api/v1/tokens/550e8400-e29b-41d4-a716-446655440000
```

**Создать связь:**
```bash
curl -X POST http://localhost:8000/api/v1/graph/connections \
  -H "Content-Type: application/json" \
  -d '{
    "source_id": "550e8400-...",
    "target_id": "660e9500-...",
    "connection_type": "spatial",
    "weight": 0.8
  }'
```

### Примеры на Python

```python
import httpx

async with httpx.AsyncClient() as client:
    # Создать токен
    response = await client.post(
        "http://localhost:8000/api/v1/tokens/",
        json={
            "type": "demo",
            "coordinates": {
                "x": [1.0] + [0]*7,
                "y": [0]*8,
                "z": [0]*8
            }
        }
    )
    token = response.json()
    
    # Получить токен
    response = await client.get(
        f"http://localhost:8000/api/v1/tokens/{token['id']}"
    )
    print(response.json())
```

### Примеры на JavaScript

```javascript
// Создать токен
const response = await fetch('http://localhost:8000/api/v1/tokens/', {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({
    type: 'demo',
    coordinates: {
      x: [1.0, 0, 0, 0, 0, 0, 0, 0],
      y: [0, 0, 0, 0, 0, 0, 0, 0],
      z: [0, 0, 0, 0, 0, 0, 0, 0]
    }
  })
});

const token = await response.json();
console.log(token);
```

---

## 🔗 Интерактивная документация

FastAPI предоставляет интерактивную документацию API:

- **Swagger UI**: http://localhost:8000/docs
- **ReDoc**: http://localhost:8000/redoc
- **OpenAPI JSON**: http://localhost:8000/openapi.json

---

## 📊 Ограничение частоты запросов

В данный момент не реализовано. Для production окружения рекомендуется:
- Rate limiting на базе Redis
- Ограничения по IP
- Квоты для API ключей

---

**Версия**: 0.3.0  
**Последнее обновление**: 2025-10-15

## 📋 Оглавление

- [Overview](#overview)
- [Authentication](#authentication)
- [Endpoints](#endpoints)
- [Error Handling](#error-handling)
- [Examples](#examples)

---

## 🌐 Overview

### Base URL

```
http://localhost:8000/api/v1
```

### Response Format

All responses are in JSON format.

### Status Codes

- `200` - Success
- `201` - Created
- `204` - No Content (Delete)
- `400` - Bad Request
- `404` - Not Found
- `500` - Internal Server Error

---

## 🔐 Authentication

Currently, the API does not require authentication. For production, implement JWT tokens.

---

## 📡 Endpoints

### System Endpoints

#### `GET /api/v1/system/health`

Health check endpoint.

**Response:**
```json
{
  "status": "healthy",
  "version": "0.3.0",
  "timestamp": "2025-01-15T10:30:00",
  "services": {
    "postgres": "up",
    "websocket": "up"
  }
}
```

#### `GET /api/v1/system/stats`

Get system statistics.

**Response:**
```json
{
  "tokens": 150,
  "connections": 230,
  "experience_events": 450,
  "websocket_connections": 5,
  "uptime_seconds": 3600.0
}
```

#### `GET /api/v1/system/info`

Get system information.

---

### Token Endpoints

#### `POST /api/v1/tokens/`

Create a new token.

**Request Body:**
```json
{
  "type": "demo",
  "coordinates": {
    "x": [1.0, 0.5, 0.25, 0.125, 0.0625, 0.03125, 0.015625, 0.0078125],
    "y": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    "z": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
  },
  "weight": 1.0,
  "flags": 0,
  "metadata": {
    "description": "Demo token"
  }
}
```

**Response: `201 Created`**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "type": "demo",
  "coordinates": {...},
  "weight": 1.0,
  "flags": 0,
  "timestamp": 1705315800000,
  "metadata": {...},
  "created_at": "2025-01-15T10:30:00",
  "updated_at": null
}
```

#### `GET /api/v1/tokens/`

List tokens with pagination.

**Query Parameters:**
- `limit` (int, 1-100): Max tokens to return (default: 10)
- `offset` (int): Number to skip (default: 0)
- `token_type` (string): Filter by type

**Response:**
```json
{
  "tokens": [...],
  "total": 150,
  "limit": 10,
  "offset": 0
}
```

#### `GET /api/v1/tokens/{token_id}`

Get token by ID.

**Response:**
```json
{
  "id": "550e8400-...",
  "type": "demo",
  "coordinates": {...},
  "weight": 1.0,
  ...
}
```

#### `PUT /api/v1/tokens/{token_id}`

Update a token.

**Request Body:**
```json
{
  "type": "updated",
  "weight": 2.0,
  "metadata": {"updated": true}
}
```

#### `DELETE /api/v1/tokens/{token_id}`

Delete a token.

**Response: `204 No Content`**

#### `POST /api/v1/tokens/search/spatial`

Search tokens in spatial region.

**Request Body:**
```json
{
  "min_x": 0.0,
  "min_y": 0.0,
  "min_z": 0.0,
  "max_x": 10.0,
  "max_y": 10.0,
  "max_z": 10.0,
  "level": 0
}
```

**Response:**
```json
{
  "tokens": [...],
  "total": 15,
  "limit": 15,
  "offset": 0
}
```

#### `POST /api/v1/tokens/batch`

Create multiple tokens in batch.

**Request Body:**
```json
{
  "tokens": [
    {
      "type": "batch1",
      "coordinates": {...},
      "weight": 1.0
    },
    {
      "type": "batch2",
      "coordinates": {...},
      "weight": 1.5
    }
  ]
}
```

**Response:**
```json
{
  "created": [...],
  "failed": [],
  "total_created": 2,
  "total_failed": 0
}
```

#### `GET /api/v1/tokens/count/total`

Count tokens.

**Query Parameters:**
- `token_type` (string): Filter by type

**Response:**
```json
{
  "count": 150,
  "type": "demo"
}
```

---

### Graph Endpoints

#### `POST /api/v1/graph/connections`

Create a connection between tokens.

**Request Body:**
```json
{
  "source_id": "550e8400-...",
  "target_id": "660e9500-...",
  "connection_type": "spatial",
  "weight": 0.8,
  "decay_rate": 0.0,
  "bidirectional": false,
  "metadata": {}
}
```

**Response: `201 Created`**
```json
{
  "id": "770f1600-...",
  "source_id": "550e8400-...",
  "target_id": "660e9500-...",
  "connection_type": "spatial",
  "weight": 0.8,
  "decay_rate": 0.0,
  "bidirectional": false,
  "metadata": {},
  "generation": 0,
  "fitness": 0.0,
  "created_at": "2025-01-15T10:30:00",
  "updated_at": null
}
```

#### `GET /api/v1/graph/connections`

List connections.

**Query Parameters:**
- `limit`, `offset`, `connection_type`

#### `GET /api/v1/graph/connections/{connection_id}`

Get connection by ID.

#### `PUT /api/v1/graph/connections/{connection_id}`

Update a connection.

**Request Body:**
```json
{
  "weight": 0.9,
  "metadata": {"updated": true}
}
```

#### `DELETE /api/v1/graph/connections/{connection_id}`

Delete a connection.

#### `GET /api/v1/graph/tokens/{token_id}/neighbors`

Get neighbors of a token.

**Query Parameters:**
- `direction` (string): "incoming", "outgoing", "both"
- `connection_type` (string): Filter by type

**Response:**
```json
{
  "token_id": "550e8400-...",
  "neighbors": [...],
  "count": 3,
  "direction": "both"
}
```

#### `GET /api/v1/graph/tokens/{token_id}/degree`

Get token degree (connectivity).

**Response:**
```json
{
  "token_id": "550e8400-...",
  "in_degree": 2,
  "out_degree": 3,
  "total_degree": 5
}
```

#### `GET /api/v1/graph/path`

Find paths between tokens.

**Query Parameters:**
- `source_id` (UUID): Source token
- `target_id` (UUID): Target token
- `max_depth` (int, 1-10): Maximum search depth

**Response:**
```json
{
  "source_id": "550e8400-...",
  "target_id": "660e9500-...",
  "paths": [
    ["550e8400-...", "770f1600-...", "660e9500-..."],
    ["550e8400-...", "880g2700-...", "660e9500-..."]
  ],
  "count": 2
}
```

#### `GET /api/v1/graph/stats`

Get graph statistics.

**Response:**
```json
{
  "total_nodes": 150,
  "total_edges": 230,
  "avg_degree": 3.07,
  "density": 0.0103,
  "connected_components": null
}
```

#### `POST /api/v1/graph/connections/batch`

Create multiple connections in batch.

---

### Experience Endpoints

#### `POST /api/v1/experience/events`

Create an experience event.

**Request Body:**
```json
{
  "event_type": "action_taken",
  "token_id": "550e8400-...",
  "state_before": {"value": 0},
  "state_after": {"value": 1},
  "action": {"type": "increment"},
  "reward": 1.0,
  "metadata": {},
  "priority": 1.0
}
```

**Response: `201 Created`**
```json
{
  "id": "990h3800-...",
  "event_type": "action_taken",
  "timestamp": 1705315800000,
  "token_id": "550e8400-...",
  "state_before": {"value": 0},
  "state_after": {"value": 1},
  "action": {"type": "increment"},
  "reward": 1.0,
  "metadata": {},
  "priority": 1.0,
  "trajectory_id": null,
  "sequence_number": 0,
  "created_at": "2025-01-15T10:30:00"
}
```

#### `GET /api/v1/experience/events`

List experience events.

**Query Parameters:**
- `limit`, `offset`, `event_type`

#### `GET /api/v1/experience/events/{event_id}`

Get event by ID.

#### `GET /api/v1/experience/events/token/{token_id}`

Get events for a specific token.

**Query Parameters:**
- `limit` (int): Max events to return

#### `DELETE /api/v1/experience/events/cleanup`

Clean up old events.

**Query Parameters:**
- `retention_days` (int, 1-365): Keep events from last N days

**Response:**
```json
{
  "deleted": 45,
  "retention_days": 30
}
```

---

## ⚠️ Error Handling

### Error Response Format

```json
{
  "detail": "Error message",
  "status_code": 404
}
```

### Common Errors

**400 Bad Request**
```json
{
  "detail": "Validation error: field 'coordinates' is required"
}
```

**404 Not Found**
```json
{
  "detail": "Token 550e8400-... not found"
}
```

**500 Internal Server Error**
```json
{
  "detail": "Database connection error"
}
```

---

## 💻 Examples

### cURL Examples

**Create Token:**
```bash
curl -X POST http://localhost:8000/api/v1/tokens/ \
  -H "Content-Type: application/json" \
  -d '{
    "type": "demo",
    "coordinates": {
      "x": [1.0,0,0,0,0,0,0,0],
      "y": [0,0,0,0,0,0,0,0],
      "z": [0,0,0,0,0,0,0,0]
    }
  }'
```

**List Tokens:**
```bash
curl http://localhost:8000/api/v1/tokens/?limit=5
```

**Get Token:**
```bash
curl http://localhost:8000/api/v1/tokens/550e8400-e29b-41d4-a716-446655440000
```

**Create Connection:**
```bash
curl -X POST http://localhost:8000/api/v1/graph/connections \
  -H "Content-Type: application/json" \
  -d '{
    "source_id": "550e8400-...",
    "target_id": "660e9500-...",
    "connection_type": "spatial",
    "weight": 0.8
  }'
```

### Python Examples

```python
import httpx

async with httpx.AsyncClient() as client:
    # Create token
    response = await client.post(
        "http://localhost:8000/api/v1/tokens/",
        json={
            "type": "demo",
            "coordinates": {
                "x": [1.0] + [0]*7,
                "y": [0]*8,
                "z": [0]*8
            }
        }
    )
    token = response.json()
    
    # Get token
    response = await client.get(
        f"http://localhost:8000/api/v1/tokens/{token['id']}"
    )
    print(response.json())
```

### JavaScript Examples

```javascript
// Create token
const response = await fetch('http://localhost:8000/api/v1/tokens/', {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({
    type: 'demo',
    coordinates: {
      x: [1.0, 0, 0, 0, 0, 0, 0, 0],
      y: [0, 0, 0, 0, 0, 0, 0, 0],
      z: [0, 0, 0, 0, 0, 0, 0, 0]
    }
  })
});

const token = await response.json();
console.log(token);
```

---

## 🔗 Interactive Documentation

FastAPI provides interactive API documentation:

- **Swagger UI**: http://localhost:8000/docs
- **ReDoc**: http://localhost:8000/redoc
- **OpenAPI JSON**: http://localhost:8000/openapi.json

---

## 📊 Rate Limiting

Currently not implemented. For production, consider:
- Redis-based rate limiting
- Per-IP limits
- API key quotas

---

**Version**: 0.3.0  
**Last Updated**: 2025-10-15