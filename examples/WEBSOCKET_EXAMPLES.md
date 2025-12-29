# WebSocket Examples

Примеры использования WebSocket возможностей NeuroGraph (v0.60.0 и v0.60.1).

## Содержание

- [Базовые примеры (v0.60.0)](#базовые-примеры-v0600)
- [Продвинутые возможности (v0.60.1)](#продвинутые-возможности-v0601)
- [CLI Tool](#cli-tool)

---

## Базовые примеры (v0.60.0)

### 1. WebSocket Demo

Полный пример с подпиской на все каналы и обработкой событий:

```bash
# Запустить API сервер
python -m src.api.main

# В другом терминале - запустить demo
python examples/websocket_demo.py
```

**Что делает:**
- Подключается к WebSocket endpoint
- Подписывается на все 6 каналов
- Обрабатывает события в real-time
- Показывает метрики, сигналы, действия, логи, статус

**Файл:** [`websocket_demo.py`](websocket_demo.py)

---

## Продвинутые возможности (v0.60.1)

### 2. Advanced Features Demo

Демонстрация всех новых возможностей v0.60.1:

```bash
python examples/websocket_advanced_demo.py
```

**Демонстрирует:**

#### Demo 1: Reconnection Tokens
- Запрос reconnection token
- Отключение и переподключение
- Автоматическое восстановление подписок
- Проверка сохранения сессии

#### Demo 2: Permissions & RBAC
- Подключение как anonymous user
- Попытка подписки на все каналы
- Получение списка разрешенных/запрещенных каналов
- Демонстрация permission matrix

#### Demo 3: Rate Limiting
- Быстрая отправка множества запросов
- Срабатывание rate limit
- Получение retry_after значения
- Статистика успешных/отклоненных запросов

#### Demo 4: Binary Messages & Compression
- Создание binary image message
- Расчет overhead (header + metadata)
- Сжатие больших JSON объектов
- Измерение коэффициента сжатия

#### Demo 5: Prometheus Metrics
- Эмуляция активности (connections, messages, subscriptions)
- Отслеживание метрик
- Список доступных метрик

**Файл:** [`websocket_advanced_demo.py`](websocket_advanced_demo.py)

---

## CLI Tool

### 3. WebSocket CLI Examples

Готовые команды для тестирования через CLI:

```bash
# Показать все примеры
bash examples/websocket_cli_examples.sh
```

**10 готовых примеров:**

1. **Basic connection** - простое подключение
2. **Subscribe to channels** - подписка на конкретные каналы
3. **JSON output** - вывод в JSON формате
4. **Compact output** - компактный вывод
5. **With authentication** - с JWT токеном
6. **All channels (admin)** - все каналы для админа
7. **Verbose mode** - подробное логирование
8. **Monitor metrics** - мониторинг только метрик
9. **Save to log** - сохранение в файл
10. **Real-time monitoring** - real-time мониторинг

**Файл:** [`websocket_cli_examples.sh`](websocket_cli_examples.sh)

---

## Быстрый старт

### Минимальный пример (Python)

```python
from neurograph_ws_client import NeurographWSClient, Channel

async def main():
    # Подключиться
    client = NeurographWSClient(url="ws://localhost:8000/ws")
    await client.connect()

    # Подписаться
    client.subscribe(Channel.METRICS, lambda data: print(f"Metrics: {data}"))

    # Слушать
    await client.run_forever()

import asyncio
asyncio.run(main())
```

### Минимальный пример (TypeScript)

```typescript
import NeurographWSClient from "./neurograph-ws-client";

const client = new NeurographWSClient({
  url: "ws://localhost:8000/ws"
});

await client.connect();
client.subscribe("metrics", (data) => console.log("Metrics:", data));
```

### Минимальный пример (CLI)

```bash
python -m src.api.websocket.cli --url ws://localhost:8000/ws --subscribe metrics
```

---

## Reconnection Example

```python
# 1. Запросить token перед отключением
await client.send({"type": "get_reconnection_token"})
response = await client.receive()
token = response["token"]

# 2. Переподключиться с token
new_client = NeurographWSClient(
    url=f"ws://localhost:8000/ws?reconnection_token={token}"
)
await new_client.connect()

# Все подписки восстановлены автоматически!
```

---

## Binary Messages Example

```python
from src.api.websocket.binary import binary_handler

# Создать binary image message
image_bytes = open("photo.jpg", "rb").read()
binary_msg = binary_handler.create_image_message(
    image_bytes,
    format="jpeg",
    width=1920,
    height=1080
)

# Отправить через WebSocket
# (binary messages требуют специальной обработки на клиенте)
```

---

## Compression Example

```python
from src.api.websocket.compression import default_compressor

# Сжать большой JSON
large_data = {"items": [...]}  # Много данных
compressed, was_compressed = default_compressor.compress_json(large_data)

if was_compressed:
    print(f"Compressed: {len(compressed)} bytes")
    print(f"Savings: {(1 - len(compressed)/original_size) * 100:.1f}%")
```

---

## Permissions Matrix

| Channel | Admin | Developer | Viewer | Bot | Anonymous |
|---------|-------|-----------|--------|-----|-----------|
| metrics | ✅ Sub+Broadcast | ✅ Subscribe | ✅ Subscribe | ✅ Subscribe | ✅ Subscribe |
| signals | ✅ Sub+Broadcast | ✅ Subscribe | ❌ | ✅ Subscribe | ❌ |
| actions | ✅ Sub+Broadcast | ✅ Subscribe | ❌ | ❌ | ❌ |
| logs | ✅ Sub+Broadcast | ✅ Subscribe | ❌ | ❌ | ❌ |
| status | ✅ Sub+Broadcast | ✅ Subscribe | ✅ Subscribe | ✅ Subscribe | ✅ Subscribe |
| connections | ✅ Sub+Broadcast | ❌ | ❌ | ❌ | ❌ |

---

## Rate Limits

| Message Type | Capacity | Refill Rate |
|-------------|----------|-------------|
| ping | 120 | 2/sec |
| subscribe | 30 | 1/sec |
| unsubscribe | 30 | 1/sec |
| default | 60 | 10/sec |

---

## Prometheus Metrics

Доступны на `/metrics` endpoint:

```
neurograph_ws_connections_total          # Активные соединения
neurograph_ws_connections_opened_total   # Всего открыто
neurograph_ws_connections_closed_total   # Всего закрыто
neurograph_ws_connection_duration_seconds # Длительность
neurograph_ws_messages_sent_total        # Отправлено
neurograph_ws_messages_received_total    # Получено
neurograph_ws_message_size_bytes         # Размер
neurograph_ws_message_latency_seconds    # Latency
neurograph_ws_subscriptions_total        # Подписки
neurograph_ws_channel_subscribers        # Подписчики
neurograph_ws_buffered_events            # Буфер
neurograph_ws_errors_total               # Ошибки
neurograph_ws_broadcast_duration_seconds # Broadcast время
neurograph_ws_reconnections_total        # Переподключения
neurograph_ws_rate_limit_hits_total      # Rate limit срабатывания
```

---

## Troubleshooting

### WebSocket не подключается

```bash
# Проверить что API сервер запущен
curl http://localhost:8000/health

# Проверить WebSocket endpoint
curl -i -N -H "Connection: Upgrade" -H "Upgrade: websocket" \
  http://localhost:8000/ws
```

### Rate limit срабатывает слишком часто

Проверьте настройки в [`rate_limit.py`](../src/api/websocket/rate_limit.py):

```python
_message_limits = {
    "ping": (120, 2.0),      # Capacity, refill/sec
    "subscribe": (30, 1.0),
    "default": (60, 10.0),
}
```

### Reconnection token не работает

- Token валиден только 5 минут
- Token можно использовать только один раз
- Убедитесь что передаете token в query parameter: `?reconnection_token=...`

---

## Дополнительная информация

- **[WebSocket Guide](../docs/guides/WEBSOCKET_GUIDE.md)** - Полное руководство
- **[CHANGELOG v0.60.1](../docs/changelogs/CHANGELOG_v0.60.1.md)** - Детальный changelog
- **[CHANGELOG v0.60.0](../docs/changelogs/CHANGELOG_v0.60.0.md)** - Базовые возможности
- **[Client Libraries](../client-libraries/)** - TypeScript и Python клиенты

---

## Лицензия

Copyright (C) 2024-2025 Chernov Denys

GNU AGPL v3.0 - см. [LICENSE](../LICENSE)
