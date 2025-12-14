# NeuroGraph OS - Implementation Roadmap

**Версия:** 1.0
**Дата создания:** 2025-12-14
**Статус:** Active Development Plan
**Базовые спецификации:**
- `/docs/arch/TIRO_INTERFACES_ARCHITECTURE.md`
- `/docs/arch/NEUROGRAPH_LIBRARY_SPEC.md`
- `/docs/arch/REST_API_SPEC.md`
- `/docs/arch/WEB_DASHBOARD_SPEC.md`
- `/docs/arch/JUPYTER_INTEGRATION.md`

---

## 🎯 Общая цель

Построить полноценную систему интерфейсов для NeuroGraph OS, следуя многоуровневой архитектуре:

```
Core (Rust) ✅ → Library (Python) → REST API → Web Dashboard + Jupyter
```

---

## 📊 Текущий статус

### ✅ Готово:
- **Core Layer (Rust)** - neurograph-core v0.45.0
  - Token, Connection, Grid, Graph
  - IntuitionEngine v3.0
  - Guardian with CDNA
  - Prometheus metrics
  - Experience Stream
  - Action Controller
  - Panic Handler

### ⏳ В разработке:
- Ничего (начинаем с чистого листа)

### ❌ Не начато:
- Library Layer (Python)
- Service Layer (REST API)
- Presentation Layer (Web/Jupyter)

---

## 🗺️ Roadmap (4 фазы)

---

## Phase 1: Python Library Foundation 🐍

**Цель:** Создать `neurograph` Python package с FFI bindings к Rust core

**Длительность:** 5-7 дней
**Приоритет:** 🔴 КРИТИЧЕСКИЙ (блокирует всё остальное)

### 1.1 Project Setup (1 день)

**Задачи:**
- [ ] Создать структуру проекта `src/python/neurograph/`
- [ ] Настроить `pyproject.toml` с maturin
- [ ] Настроить PyO3 в `src/core_rust/Cargo.toml`
- [ ] Создать `README.md` для Python package
- [ ] Настроить GitHub Actions для Python package

**Файлы:**
```
src/python/
├── pyproject.toml
├── README.md
├── neurograph/
│   ├── __init__.py
│   ├── runtime.py
│   ├── query.py
│   ├── bootstrap.py
│   ├── config.py
│   ├── types.py
│   ├── exceptions.py
│   └── _core.py  # PyO3 wrapper
└── tests/
    └── test_runtime.py
```

**Критерии готовности:**
- ✅ `pip install -e .` работает
- ✅ `import neurograph as ng` импортируется без ошибок

---

### 1.2 PyO3 FFI Bindings (2 дня)

**Задачи:**
- [ ] Создать Rust модуль с PyO3 bindings в `src/core_rust/src/python_bindings/`
- [ ] Экспортировать базовые функции:
  - `create_runtime()` → Python Runtime
  - `create_token(id, state)`
  - `query(text)` → результаты поиска
  - `get_metrics()` → Prometheus metrics
- [ ] Обработка ошибок Rust → Python exceptions
- [ ] Тесты FFI интерфейса

**Файлы (Rust):**
```
src/core_rust/src/
├── python_bindings/
│   ├── mod.rs
│   ├── runtime.rs
│   ├── token.rs
│   ├── query.rs
│   └── metrics.rs
└── lib.rs  # + #[pymodule]
```

**Критерии готовности:**
- ✅ `neurograph._core.create_runtime()` возвращает объект
- ✅ Rust panics конвертируются в Python exceptions
- ✅ Все тесты PyO3 проходят

---

### 1.3 Python Runtime Manager (1 день)

**Задачи:**
- [ ] Реализовать класс `Runtime` в `runtime.py`
- [ ] Методы:
  - `__init__(config)` - инициализация
  - `start()` - запуск системы
  - `stop()` - остановка
  - `status()` - статус
  - `query(text)` → QueryResult
  - `bootstrap(file)` - загрузка эмбеддингов
- [ ] Lifecycle management (context manager)
- [ ] Документация (docstrings)

**Пример использования:**
```python
import neurograph as ng

runtime = ng.Runtime()
runtime.start()
result = runtime.query("hello")
print(result.top(5))
runtime.stop()
```

**Критерии готовности:**
- ✅ Runtime запускается и останавливается
- ✅ Все методы работают
- ✅ Документация полная

---

### 1.4 Query Engine & Bootstrap (1 день)

**Задачи:**
- [ ] Класс `QueryResult` с результатами
- [ ] Метод `query(text, limit=10)` через FFI
- [ ] Bootstrap loader для эмбеддингов
- [ ] Поддержка форматов: GloVe, Word2Vec
- [ ] Progress bar при загрузке (tqdm)

**Пример:**
```python
runtime.bootstrap("glove.6B.50d.txt", limit=50000)
result = runtime.query("cat", limit=5)

for word, similarity in result:
    print(f"{word}: {similarity:.3f}")
```

**Критерии готовности:**
- ✅ Bootstrap загружает эмбеддинги
- ✅ Query возвращает результаты
- ✅ QueryResult поддерживает итерацию

---

### 1.5 Testing & Documentation (1 день)

**Задачи:**
- [ ] Unit tests для всех модулей (pytest)
- [ ] Integration tests
- [ ] Документация API (Sphinx)
- [ ] Примеры использования
- [ ] README с Quick Start

**Тесты:**
```python
def test_runtime_lifecycle():
    runtime = ng.Runtime()
    runtime.start()
    assert runtime.status() == "running"
    runtime.stop()

def test_query():
    runtime = ng.Runtime()
    runtime.bootstrap("test_embeddings.txt")
    result = runtime.query("test")
    assert len(result) > 0
```

**Критерии готовности:**
- ✅ 80%+ test coverage
- ✅ Все тесты проходят
- ✅ Документация сгенерирована

---

### Phase 1 - Итоги

**Deliverables:**
- ✅ `neurograph` Python package (PyPI ready)
- ✅ FFI bindings к neurograph-core
- ✅ Runtime Manager работает
- ✅ Query + Bootstrap работают
- ✅ Тесты + документация

**Следующий шаг:** Phase 2 - REST API

---

## Phase 2: REST API Service 🌐

**Цель:** Создать FastAPI сервис для HTTP/WebSocket доступа

**Длительность:** 4-5 дней
**Приоритет:** 🟡 ВЫСОКИЙ

### 2.1 FastAPI Project Setup (0.5 дня)

**Задачи:**
- [ ] Создать `src/api/` структуру
- [ ] Настроить FastAPI + uvicorn
- [ ] Docker контейнер для API
- [ ] OpenAPI документация
- [ ] CORS настройка

**Файлы:**
```
src/api/
├── main.py
├── config.py
├── dependencies.py
├── models/
│   ├── __init__.py
│   ├── query.py
│   ├── response.py
│   └── metrics.py
├── routers/
│   ├── __init__.py
│   ├── query.py
│   ├── modules.py
│   ├── metrics.py
│   └── admin.py
├── middleware/
│   ├── __init__.py
│   └── auth.py
└── tests/
    └── test_api.py
```

**Критерии готовности:**
- ✅ `uvicorn src.api.main:app` запускается
- ✅ OpenAPI доступна на `/docs`
- ✅ Health check endpoint работает

---

### 2.2 Core Endpoints (1.5 дня)

**Задачи:**
- [ ] `POST /api/v1/query` - семантический поиск
- [ ] `GET /api/v1/status` - статус системы
- [ ] `GET /api/v1/metrics` - Prometheus metrics
- [ ] `GET /api/v1/modules` - список модулей
- [ ] Response format (success/error wrapper)

**Примеры:**
```bash
# Query
curl -X POST http://localhost:8000/api/v1/query \
  -H "Content-Type: application/json" \
  -d '{"text": "cat", "limit": 5}'

# Response
{
  "success": true,
  "data": {
    "results": [
      {"word": "dog", "similarity": 0.92},
      {"word": "kitten", "similarity": 0.87}
    ]
  },
  "meta": {
    "processing_time_ms": 14.2,
    "timestamp": "2025-01-25T12:34:56Z"
  }
}
```

**Критерии готовности:**
- ✅ Все endpoints отвечают
- ✅ Ошибки обрабатываются корректно
- ✅ OpenAPI схемы валидны

---

### 2.3 WebSocket Support (1 день)

**Задачи:**
- [ ] WebSocket endpoint `/ws`
- [ ] Real-time метрики
- [ ] События системы (stream)
- [ ] Heartbeat/ping-pong
- [ ] Reconnect logic (клиент)

**Пример (клиент):**
```python
import websockets

async with websockets.connect("ws://localhost:8000/ws") as ws:
    await ws.send(json.dumps({"type": "subscribe", "channel": "metrics"}))
    while True:
        msg = await ws.recv()
        print(json.loads(msg))
```

**Критерии готовности:**
- ✅ WebSocket соединение устанавливается
- ✅ Метрики стримятся каждую секунду
- ✅ Graceful disconnect

---

### 2.4 Authentication & Security (1 день)

**Задачи:**
- [ ] JWT authentication
- [ ] `/api/v1/auth/token` endpoint
- [ ] Middleware для проверки токенов
- [ ] Rate limiting (slowapi)
- [ ] HTTPS (production)

**Пример:**
```bash
# Get token
curl -X POST /api/v1/auth/token \
  -d '{"username": "admin", "password": "secret"}'

# Use token
curl -H "Authorization: Bearer <token>" /api/v1/query
```

**Критерии готовности:**
- ✅ JWT токены работают
- ✅ Защищённые endpoints требуют auth
- ✅ Rate limiting активен

---

### 2.5 Testing & Deployment (1 день)

**Задачи:**
- [ ] Integration tests (pytest + httpx)
- [ ] Load testing (locust)
- [ ] Docker Compose для dev
- [ ] Production Dockerfile
- [ ] CI/CD для API

**Tests:**
```python
def test_query_endpoint(client):
    response = client.post("/api/v1/query",
        json={"text": "test", "limit": 5})
    assert response.status_code == 200
    assert response.json()["success"] == True
```

**Критерии готовности:**
- ✅ Все API тесты проходят
- ✅ Docker контейнер собирается
- ✅ Load test: 1000 req/sec

---

### Phase 2 - Итоги

**Deliverables:**
- ✅ FastAPI service на `/api/v1/*`
- ✅ Query, Status, Metrics, Modules endpoints
- ✅ WebSocket для real-time
- ✅ JWT authentication
- ✅ Docker + CI/CD

**Следующий шаг:** Phase 3 - Web Dashboard

---

## Phase 3: Web Dashboard (React) 🎨

**Цель:** Создать Tiro Control Center - веб-панель управления

**Длительность:** 7-10 дней
**Приоритет:** 🟢 СРЕДНИЙ

### 3.1 Project Setup (1 день)

**Задачи:**
- [ ] Create React App + TypeScript
- [ ] Ant Design Pro setup
- [ ] Folder structure
- [ ] Router (React Router v6)
- [ ] State management (Zustand)
- [ ] API client (axios)

**Структура:**
```
src/web/
├── package.json
├── src/
│   ├── components/
│   ├── pages/
│   │   ├── Dashboard/
│   │   ├── Modules/
│   │   ├── Chat/
│   │   ├── Terminal/
│   │   ├── Config/
│   │   └── Admin/
│   ├── services/
│   │   └── api.ts
│   ├── stores/
│   │   └── runtime.ts
│   ├── App.tsx
│   └── index.tsx
└── public/
```

**Критерии готовности:**
- ✅ `npm start` запускается
- ✅ Ant Design Pro layout работает
- ✅ Routing настроен

---

### 3.2 Dashboard Page (2 дня)

**Задачи:**
- [ ] Metrics cards (Status, Tokens, Connections, Queries)
- [ ] System metrics charts (CPU, Memory)
- [ ] Internal metrics (Events/sec, Latency)
- [ ] Recent activity table
- [ ] Auto-refresh (каждые 5 сек)

**Компоненты:**
```tsx
<Dashboard>
  <MetricsRow>
    <MetricCard title="Status" value="Running" />
    <MetricCard title="Tokens" value="50,000" />
    <MetricCard title="Connections" value="1.2M" />
  </MetricsRow>
  <ChartsRow>
    <SystemMetricsChart />
    <InternalMetricsChart />
  </ChartsRow>
  <RecentActivityTable />
</Dashboard>
```

**Критерии готовности:**
- ✅ Dashboard отображается
- ✅ Метрики подтягиваются из API
- ✅ Графики обновляются

---

### 3.3 Modules Management (1.5 дня)

**Задачи:**
- [ ] Modules list (ProTable)
- [ ] Module details modal
- [ ] Start/Stop/Restart actions
- [ ] Module configuration
- [ ] Status indicators (running/stopped/error)

**UI:**
```
┌─ Modules ────────────────────────────────┐
│ Name          | Status   | CPU  | Memory │
│ Gateway       | Running  | 12%  | 145 MB │
│ Intuition     | Running  | 8%   | 98 MB  │
│ Guardian      | Stopped  | -    | -      │
│               [Start] [Stop] [Config]     │
└──────────────────────────────────────────┘
```

**Критерии готовности:**
- ✅ Список модулей отображается
- ✅ Actions (start/stop) работают
- ✅ Config modal открывается

---

### 3.4 Chat & Terminal (2 дня)

**Задачи:**
- [ ] Chat interface (message bubbles)
- [ ] Terminal interface (xterm.js)
- [ ] Mode toggle (Chat ↔ Terminal)
- [ ] Message history
- [ ] WebSocket integration
- [ ] Auto-scroll

**Компоненты:**
```tsx
<ChatTerminal mode={mode}>
  <Header>
    <ModeToggle />
  </Header>
  <MessageArea>
    {mode === 'chat' ? <ChatMessages /> : <Terminal />}
  </MessageArea>
  <InputArea>
    <Input onSubmit={handleSend} />
  </InputArea>
</ChatTerminal>
```

**Критерии готовности:**
- ✅ Chat отправляет сообщения
- ✅ Terminal выполняет команды
- ✅ Переключение режимов работает

---

### 3.5 Config & Admin (1.5 дня)

**Задачи:**
- [ ] Config editor (ProForm)
- [ ] Bootstrap uploader
- [ ] CDNA management
- [ ] System logs viewer
- [ ] Settings persistence

**Критерии готовности:**
- ✅ Конфиг редактируется и сохраняется
- ✅ Bootstrap файлы загружаются
- ✅ Логи отображаются

---

### 3.6 Polish & Deploy (2 дня)

**Задачи:**
- [ ] Dark/Light theme toggle
- [ ] Responsive layout (mobile)
- [ ] Error boundaries
- [ ] Loading states
- [ ] Production build
- [ ] Nginx config
- [ ] Docker для frontend

**Критерии готовности:**
- ✅ Темы переключаются
- ✅ Mobile version работает
- ✅ Production build оптимизирован
- ✅ Docker контейнер собирается

---

### Phase 3 - Итоги

**Deliverables:**
- ✅ Tiro Control Center (React SPA)
- ✅ Dashboard, Modules, Chat, Terminal, Config, Admin
- ✅ Real-time updates via WebSocket
- ✅ Dark/Light themes
- ✅ Production ready

**Следующий шаг:** Phase 4 - Jupyter Integration

---

## Phase 4: Jupyter Integration 📊

**Цель:** Magic commands и rich display для Jupyter Notebook

**Длительность:** 2-3 дня
**Приоритет:** 🟢 НИЗКИЙ (nice-to-have)

### 4.1 IPython Extension (1 день)

**Задачи:**
- [ ] Создать `neurograph/integrations/jupyter.py`
- [ ] IPython magic commands:
  - `%load_ext neurograph`
  - `%ng_status`
  - `%ng_query <text>`
  - `%ng_stats`
- [ ] Cell magic `%%ng_explore`

**Пример:**
```python
%load_ext neurograph
%ng_query cat
```

**Критерии готовности:**
- ✅ Extension загружается
- ✅ Все magic commands работают

---

### 4.2 Rich Display (1 день)

**Задачи:**
- [ ] `_repr_html_()` для QueryResult
- [ ] Красивая таблица результатов
- [ ] Интерактивная визуализация (plotly)
- [ ] Export в DataFrame

**Пример:**
```python
result = runtime.query("cat")
result  # Автоматический rich display в Jupyter
```

**Критерии готовности:**
- ✅ QueryResult отображается красиво
- ✅ Можно экспортировать в DataFrame

---

### 4.3 Visualization (0.5 дня)

**Задачи:**
- [ ] Граф визуализация (networkx + matplotlib)
- [ ] Метод `result.visualize()`
- [ ] Interactive plot (plotly)

**Критерии готовности:**
- ✅ Граф визуализируется
- ✅ Interactive plot работает

---

### 4.4 Documentation & Examples (0.5 дня)

**Задачи:**
- [ ] Jupyter notebook примеры
- [ ] Tutorial для начинающих
- [ ] Advanced examples

**Критерии готовности:**
- ✅ 3+ примера ноутбуков
- ✅ Tutorial полный

---

### Phase 4 - Итоги

**Deliverables:**
- ✅ Jupyter extension с magic commands
- ✅ Rich display для результатов
- ✅ Визуализация графов
- ✅ Примеры ноутбуков

---

## 📦 Deployment & Infrastructure

### Production Stack

```
┌─────────────────────────────────────────┐
│           Load Balancer (nginx)         │
├─────────────────────────────────────────┤
│  Web App (React)  │  API (FastAPI)     │
├─────────────────────┼──────────────────┤
│      neurograph (Python)               │
├────────────────────────────────────────┤
│      neurograph-core (Rust)            │
└────────────────────────────────────────┘
```

**Мониторинг:**
- Prometheus (metrics)
- Grafana (dashboards)
- Jaeger (tracing)

---

## 📋 Общий Timeline

| Phase | Длительность | Дедлайн (ориентир) |
|-------|-------------|-------------------|
| Phase 1: Python Library | 5-7 дней | Week 1-2 |
| Phase 2: REST API | 4-5 дней | Week 2-3 |
| Phase 3: Web Dashboard | 7-10 дней | Week 3-5 |
| Phase 4: Jupyter | 2-3 дня | Week 5 |
| **TOTAL** | **18-25 дней** | **~1 месяц** |

---

## ✅ Success Metrics

### Phase 1 (Python Library):
- [ ] `pip install neurograph` работает
- [ ] Query возвращает результаты < 100ms
- [ ] 80%+ test coverage
- [ ] Документация полная

### Phase 2 (REST API):
- [ ] API обрабатывает 1000 req/sec
- [ ] Latency p95 < 50ms
- [ ] 100% uptime в production
- [ ] OpenAPI docs полные

### Phase 3 (Web Dashboard):
- [ ] Загружается < 2 сек
- [ ] Все экраны работают
- [ ] Mobile responsive
- [ ] Lighthouse score > 90

### Phase 4 (Jupyter):
- [ ] Magic commands работают
- [ ] Rich display красивый
- [ ] 3+ примера ноутбуков

---

## 🚀 Next Steps

**Immediate (сегодня):**
1. ✅ Создать roadmap (этот файл)
2. Начать Phase 1.1 - Python project setup

**This Week:**
- Complete Phase 1.1-1.3 (Project + FFI + Runtime)
- First working `neurograph.query()` call

**This Month:**
- Complete all 4 phases
- Production deployment
- Public demo

---

## 📝 Notes

- Весь код под AGPLv3 + Commercial dual licensing
- Документация на русском (код на английском)
- Все коммиты с Claude Code footer
- Тесты обязательны для каждой фазы

---

**Конец roadmap. Готовы начинать!** 🚀
