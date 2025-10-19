# NeuroGraph OS - Документация WebSocket API

Двусторонняя связь в реальном времени для NeuroGraph OS.

## 📋 Оглавление

- [Обзор](#обзор)
- [Подключение](#подключение)
- [Формат сообщений](#формат-сообщений)
- [Методы API](#методы-api)
- [Темы и подписки](#темы-и-подписки)
- [Клиентские библиотеки](#клиентские-библиотеки)
- [Примеры](#примеры)

---

## 🔌 Обзор

WebSocket API предоставляет:
- **Обновления в реальном времени** - мгновенные уведомления о событиях
- **Двусторонняя связь** - клиент ↔ сервер
- **Подписки на темы** - подписка на конкретные события
- **Автоматическое переподключение** - восстановление связи при обрыве
- **Механизм heartbeat** - контроль состояния соединения

### Архитектура

```
Клиент (Браузер/Python)
    ↓
WebSocket Соединение
    ↓
Менеджер Подключений
    ↓
Реестр Обработчиков Сообщений
    ↓
Бизнес-логика (Токены, Граф и т.д.)
    ↓
База данных / Redis
```

---

## 🚀 Подключение

### Endpoint

```
ws://localhost:8000/ws
```

### Параметры запроса

- `client_id` (опционально) - идентификатор клиента

### Заголовки

- `Authorization` (опционально) - токен авторизации

### Пример подключения

**JavaScript:**
```javascript
const ws = new WebSocket('ws://localhost:8000/ws?client_id=my_client');
```

**Python:**
```python
import websockets

async with websockets.connect('ws://localhost:8000/ws') as ws:
    # подключено
```

---

## 📦 Формат сообщений

### Структура сообщения

```json
{
  "id": "msg_123456789",
  "type": "message.type",
  "payload": {
    "key": "value"
  },
  "timestamp": 1234567890000,
  "sender_id": "необязательно"
}
```

### Поля

- `id` (string) - уникальный ID сообщения
- `type` (string) - тип сообщения
- `payload` (object) - данные сообщения
- `timestamp

---

💡 Примеры использования:
```python

from examples.websocket_client_example import NeuroGraphWebSocketClient

client = NeuroGraphWebSocketClient('ws://localhost:8000/ws')
await client.connect()

# Create token
await client.send('token.create', {
    'type': 'demo',
    'coord_x': [1.0] + [0.0] * 7,
    'weight': 1.0
})
```
```typescript
import { createWebSocket } from '@/services/websocket/neurograph-websocket';

const ws = createWebSocket({
  url: 'ws://localhost:8000/ws',
  debug: true,
  reconnect: true
});

await ws.connect();

// Listen for token creation
ws.on('token.created', (msg) => {
  console.log('New token:', msg.payload.token_id);
});

// Create token
ws.createToken({
  type: 'demo',
  coord_x: [1.0, 0, 0, 0, 0, 0, 0, 0],
  coord_y: [0, 0, 0, 0, 0, 0, 0, 0],
  coord_z: [0, 0, 0, 0, 0, 0, 0, 0]
});
```

```tsx
import { useNeuroGraphWebSocket, useWebSocketMessage } from '@/hooks/useNeuroGraphWebSocket';

function TokenMonitor() {
  const [tokens, setTokens] = useState([]);
  
  const { isConnected, subscribe, listTokens, ws } = useNeuroGraphWebSocket(
    'ws://localhost:8000/ws'
  );
  
  // Subscribe to token events
  useEffect(() => {
    if (isConnected) {
      subscribe('tokens');
    }
  }, [isConnected]);
  
  // Listen for new tokens
  useWebSocketMessage(ws, 'token.created', (msg) => {
    console.log('Token created:', msg.payload);
    listTokens(); // Refresh list
  });
  
  // Listen for token list
  useWebSocketMessage(ws, 'token.list', (msg) => {
    setTokens(msg.payload.tokens);
  });
  
  return (
    <div>
      <h2>Token Monitor {isConnected ? '🟢' : '🔴'}</h2>
      <button onClick={() => listTokens()}>Refresh</button>
      <ul>
        {tokens.map(token => (
          <li key={token.token_id}>{token.type} - {token.weight}</li>
        ))}
      </ul>
    </div>
  );
}
```

### 📊 Архитектура WebSocket
```
┌─────────────────────────────────────────────────┐
│                  Clients                        │
│  (Browser, Python, Mobile, etc.)                │
└──────────────────┬──────────────────────────────┘
                   │
                   │ WebSocket Connection
                   │
┌──────────────────▼──────────────────────────────┐
│           FastAPI WebSocket Server              │
│              (/ws endpoint)                     │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│         WebSocket Connection Manager            │
│  • Connection pooling                           │
│  • Heartbeat monitoring                         │
│  • Topic subscriptions                          │
│  • Message broadcasting                         │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│         Message Handler Registry                │
│  • Route messages by type                       │
│  • Validate payloads                            │
│  • Execute handlers                             │
└──────────────────┬──────────────────────────────┘
                   │
         ┌─────────┴─────────┐
         │                   │
┌────────▼────────┐  ┌───────▼──────────┐
│  Token Service  │  │  Graph Service   │
└────────┬────────┘  └───────┬──────────┘
         │                   │
         └─────────┬─────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│         Database Layer                          │
│  • PostgreSQL (persistence)                     │
│  • Redis (caching)                              │
└─────────────────────────────────────────────────┘
```

### 🔄 Message Flow
```
1. Client connects
   → WebSocket handshake
   → Connection registered
   → connection.established sent

2. Client subscribes
   → subscribe message
   → Topic added to subscriptions
   → subscribed confirmation

3. Event occurs (e.g., token created)
   → Database updated
   → Event published to topic
   → All subscribers receive notification

4. Client requests data
   → Message sent to server
   → Handler processes request
   → Response sent back
```


# Subscribe to events
await client.send('subscribe', {'topic': 'tokens'})