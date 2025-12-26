# NeuroGraph OS - Master Plan v3.0

**Версия:** 3.0
**Дата:** 2024-12-26
**Статус:** Active Development Plan
**Предыдущие версии:**
- [MASTER_PLAN_v2.1.md](archive/MASTER_PLAN_v2.1.md) - Signal Processing Focus
- [IMPLEMENTATION_ROADMAP.md](archive/IMPLEMENTATION_ROADMAP.md) - Full Stack Focus

---

## 🎯 Общая стратегия

Построить полноценную **когнитивную платформу** NeuroGraph OS с тремя ключевыми направлениями:

```
┌─────────────────────────────────────────────────────────────┐
│                    NeuroGraph OS Platform                   │
├─────────────────────────────────────────────────────────────┤
│  ТРЕК A: Core Intelligence (Signal Processing) ✅           │
│  ТРЕК B: Developer Experience (Python Library, Jupyter)     │
│  ТРЕК C: User Interfaces (Web Dashboard, APIs)              │
└─────────────────────────────────────────────────────────────┘
```

---

## 📊 Текущее состояние (2024-12-26)

### ✅ Что работает (v0.57.0 - PRODUCTION READY)

#### ТРЕК A: Core Intelligence ✅ ЗАВЕРШЁН

**Gateway v2.0 (Python Sensory Interface):**
- ✅ Pydantic models (SignalEvent, SemanticData, RoutingData, etc.)
- ✅ SensorRegistry с динамической регистрацией
- ✅ 4 built-in encoders:
  - PASSTHROUGH (прямая передача 8D)
  - TEXT_TFIDF (TF-IDF text encoding)
  - NUMERIC_DIRECT (simple numeric scaling)
  - SENTIMENT_SIMPLE (basic sentiment analysis)
- ✅ Built-in sensors: text, system, user_input, external_feed
- ✅ push_text(), push_system(), push_raw() API

**SignalSystem v1.1 (Rust Core Event Processing):**
- ✅ PyO3 bindings (zero-copy FFI)
- ✅ emit() - event processing pipeline
- ✅ subscribe() - event subscriptions с filters
- ✅ Subscription Filters (JSON DSL):
  - Operators: $eq, $ne, $gt, $gte, $lt, $lte, $in, $wildcard
  - Event type wildcards (signal.input.*)
  - Numeric/bitmap/hash conditions
- ✅ Pattern matching:
  - Novelty detection (is_novel)
  - Neighbor finding (similar patterns)
  - Resonance scoring
- ✅ Performance: 304,553 events/sec, 0.39μs avg latency

**ActionController (Response Generation):**
- ✅ Action registration system
- ✅ Hot/Cold path routing
- ✅ Built-in actions:
  - TextResponseAction
  - LoggingAction
  - MetricsAction
- ✅ Execution prioritization
- ✅ Background task queuing

**SignalPipeline (End-to-end Integration):**
- ✅ Full flow: Input → Gateway → Core → ActionController → Output
- ✅ process_text() orchestration
- ✅ Performance: 5,601 msg/sec, 0.18ms total latency
- ✅ Statistics tracking

**Examples & Integration:**
- ✅ Telegram bot с full Core integration
- ✅ Gateway v2.0 demos
- ✅ Performance benchmarks

**Production Infrastructure (v0.52.0):**
- ✅ REST API (30+ endpoints)
- ✅ Prometheus metrics (12 types)
- ✅ Distributed tracing (OpenTelemetry + Jaeger)
- ✅ Structured logging (JSON with correlation IDs)
- ✅ Health checks (live/ready/startup)
- ✅ Docker deployment
- ✅ Kubernetes ready

**Rust Core (v0.47.0):**
- ✅ Token V2.0 (64 bytes, 8D coordinates)
- ✅ Connection V3.0 (weighted edges)
- ✅ Grid V2.0 (spatial indexing, 8D)
- ✅ Graph (topology, spreading activation)
- ✅ RuntimeStorage (persistence, 10M tokens stable)
- ✅ Guardian + CDNA V2.1 (validation, quarantine)
- ✅ Bootstrap (semantic embeddings loader)

**Performance Metrics (v0.57.0):**
- Core throughput: 304,553 events/sec
- Core latency: 0.39μs average
- Full pipeline: 5,601 messages/sec
- End-to-end: 0.18ms total
- REST API: ~150 req/sec, <10ms latency
- Memory: ~100MB per 1M tokens

---

## ❌ Что нужно реализовать

### ТРЕК B: Developer Experience

**1. Python Library (neurograph package)**
- ❌ PyPI-ready package structure
- ❌ Unified Runtime API (высокоуровневая обёртка)
- ❌ Query Engine для семантического поиска
- ❌ Bootstrap loader (GloVe/Word2Vec через Python)
- ❌ Context managers для lifecycle
- ❌ Comprehensive documentation (Sphinx)
- ❌ Examples & tutorials

**2. Jupyter Integration**
- ❌ IPython extension (magic commands)
- ❌ Rich display для QueryResult
- ❌ Interactive visualizations (plotly)
- ❌ Graph rendering (networkx)
- ❌ Tutorial notebooks
- ❌ Export to DataFrame

### ТРЕК C: User Interfaces

**3. Web Dashboard (Tiro Control Center)**
- ❌ React SPA (TypeScript + Ant Design Pro)
- ❌ Dashboard page (metrics, charts, activity)
- ❌ Modules management (start/stop/config)
- ❌ Chat interface (message bubbles)
- ❌ Terminal interface (xterm.js)
- ❌ Config editor
- ❌ Bootstrap uploader
- ❌ System logs viewer
- ❌ Dark/Light themes
- ❌ Mobile responsive

**4. Enhanced APIs**
- ❌ WebSocket support (real-time events)
- ❌ JWT authentication
- ❌ RBAC (role-based access control)
- ❌ Rate limiting
- ❌ API keys management
- ❌ GraphQL endpoint (опционально)

**5. Enhanced Sensors**
- ❌ Audio input adapter
- ❌ Vision input adapter
- ❌ AUDIO_MEL encoder
- ❌ IMAGE_CNN encoder
- ❌ Multi-modal fusion
- ❌ Custom encoder framework

**6. Visualization & Monitoring**
- ❌ Real-time signal stream display
- ❌ Activation visualization
- ❌ Sensor registry management UI
- ❌ Live metrics dashboard
- ❌ Grafana dashboards
- ❌ Alert system

---

## 🗺️ Roadmap v3.0 (Next 6 Releases)

---

## v0.58.0 - Authentication & Security 🔐

**Цель:** Защитить API и добавить управление доступом

**Приоритет:** 🔴 КРИТИЧЕСКИЙ (блокирует production deployment)

**Длительность:** 5-7 дней

### Phase 1: JWT Authentication (2 дня)

**Задачи:**
- [ ] **1.1** JWT token generation & validation
  ```python
  # src/api/auth/jwt.py
  class JWTManager:
      def create_token(self, user_id: str, scopes: list) -> str
      def verify_token(self, token: str) -> TokenPayload
      def refresh_token(self, refresh_token: str) -> str
  ```

- [ ] **1.2** Authentication endpoints:
  - `POST /api/v1/auth/login` - получение токена
  - `POST /api/v1/auth/refresh` - обновление токена
  - `POST /api/v1/auth/logout` - инвалидация токена
  - `GET /api/v1/auth/me` - информация о текущем пользователе

- [ ] **1.3** Auth middleware для FastAPI
  ```python
  @app.get("/api/v1/protected")
  async def protected_route(user: User = Depends(get_current_user)):
      ...
  ```

**Файлы:**
- `src/api/auth/jwt.py`
- `src/api/auth/dependencies.py`
- `src/api/routers/auth.py`
- `src/api/models/auth.py`

### Phase 2: RBAC System (2 дня)

**Задачи:**
- [ ] **2.1** Role definitions:
  - `admin` - полный доступ
  - `developer` - чтение + запись (кроме системных настроек)
  - `viewer` - только чтение
  - `bot` - ограниченный доступ для ботов/интеграций

- [ ] **2.2** Permission system:
  ```python
  class Permission(Enum):
      READ_TOKENS = "tokens:read"
      WRITE_TOKENS = "tokens:write"
      READ_METRICS = "metrics:read"
      ADMIN_CONFIG = "config:admin"
      # ...
  ```

- [ ] **2.3** Permission decorators:
  ```python
  @app.post("/api/v1/tokens")
  @require_permission(Permission.WRITE_TOKENS)
  async def create_token(...):
      ...
  ```

**Файлы:**
- `src/api/auth/rbac.py`
- `src/api/auth/permissions.py`

### Phase 3: API Keys & Rate Limiting (2 дня)

**Задачи:**
- [ ] **3.1** API keys для M2M communication:
  ```python
  class APIKey(BaseModel):
      key_id: str
      key_secret: str  # hashed
      scopes: list[str]
      rate_limit: int
      expires_at: Optional[datetime]
  ```

- [ ] **3.2** API key management endpoints:
  - `POST /api/v1/keys` - создание ключа
  - `GET /api/v1/keys` - список ключей
  - `DELETE /api/v1/keys/{key_id}` - удаление ключа

- [ ] **3.3** Rate limiting (slowapi):
  - Global: 1000 req/min
  - Per user: 100 req/min
  - Per endpoint: custom limits
  - 429 Too Many Requests response

**Файлы:**
- `src/api/auth/api_keys.py`
- `src/api/middleware/rate_limit.py`

### Phase 4: Security Hardening (1 день)

**Задачи:**
- [ ] **4.1** HTTPS enforcement (production)
- [ ] **4.2** CORS configuration
- [ ] **4.3** Security headers:
  - X-Content-Type-Options
  - X-Frame-Options
  - X-XSS-Protection
  - Strict-Transport-Security

- [ ] **4.4** Input validation & sanitization
- [ ] **4.5** SQL injection prevention (параметризованные запросы)
- [ ] **4.6** Secrets management (environment variables)

**Файлы:**
- `src/api/middleware/security.py`
- `src/api/config/security.py`

### Phase 5: Testing & Documentation (1 день)

**Задачи:**
- [ ] **5.1** Unit tests для auth flow
- [ ] **5.2** Integration tests
- [ ] **5.3** Security tests (OWASP Top 10)
- [ ] **5.4** OpenAPI security schemes
- [ ] **5.5** Authentication guide
- [ ] **5.6** CHANGELOG_v0.58.0.md

**Deliverables:**
- ✅ JWT authentication работает
- ✅ RBAC система функциональна
- ✅ API keys поддерживаются
- ✅ Rate limiting активен
- ✅ Security headers настроены
- ✅ Все тесты проходят

**KPI:**
| Метрика | Target | Critical |
|---------|--------|----------|
| Auth latency | < 5ms | < 20ms |
| Token validation | < 1ms | < 5ms |
| Rate limit overhead | < 0.5ms | < 2ms |
| Security score (OWASP) | 9/10 | 7/10 |

---

## v0.59.0 - Python Library (neurograph package) 🐍

**Цель:** Создать PyPI-ready Python library для разработчиков

**Приоритет:** 🟡 ВЫСОКИЙ

**Длительность:** 5-7 дней

### Phase 1: Package Structure (1 день)

**Задачи:**
- [ ] **1.1** PyPI-ready structure:
  ```
  neurograph/
  ├── pyproject.toml
  ├── README.md
  ├── LICENSE
  ├── neurograph/
  │   ├── __init__.py
  │   ├── runtime.py      # Runtime Manager
  │   ├── query.py        # Query Engine
  │   ├── bootstrap.py    # Embeddings loader
  │   ├── config.py       # Configuration
  │   ├── types.py        # Type definitions
  │   ├── exceptions.py   # Custom exceptions
  │   └── _core.pyi       # Type stubs для PyO3
  └── tests/
      ├── test_runtime.py
      ├── test_query.py
      └── test_bootstrap.py
  ```

- [ ] **1.2** pyproject.toml с dependencies
- [ ] **1.3** setup.py для backward compatibility
- [ ] **1.4** README с Quick Start

**Файлы:**
- `pyproject.toml`
- `neurograph/__init__.py`

### Phase 2: Runtime Manager (2 дня)

**Задачи:**
- [ ] **2.1** Runtime class (высокоуровневый API):
  ```python
  from neurograph import Runtime, Config

  # Initialize
  config = Config(
      grid_size=1000,
      dimensions=50,
      enable_metrics=True
  )
  runtime = Runtime(config)

  # Lifecycle
  runtime.start()
  runtime.stop()

  # Context manager
  with Runtime() as rt:
      result = rt.query("test")
  ```

- [ ] **2.2** Integration с существующими FFI bindings
- [ ] **2.3** Lifecycle management
- [ ] **2.4** Error handling & exceptions
- [ ] **2.5** Logging integration

**Файлы:**
- `neurograph/runtime.py`
- `neurograph/config.py`
- `neurograph/exceptions.py`

### Phase 3: Query Engine (2 дня)

**Задачи:**
- [ ] **3.1** Query API:
  ```python
  result = runtime.query(
      text="machine learning",
      limit=10,
      filters={"category": "AI"}
  )

  # QueryResult
  for word, similarity in result:
      print(f"{word}: {similarity:.3f}")

  # Top results
  top5 = result.top(5)

  # Export to DataFrame
  df = result.to_dataframe()
  ```

- [ ] **3.2** QueryResult class:
  ```python
  class QueryResult:
      def __iter__(self) -> Iterator[Tuple[str, float]]
      def top(self, n: int) -> List[Tuple[str, float]]
      def to_dataframe(self) -> pd.DataFrame
      def to_dict(self) -> dict
      def __repr__(self) -> str
  ```

- [ ] **3.3** Filtering support
- [ ] **3.4** Pagination

**Файлы:**
- `neurograph/query.py`
- `neurograph/types.py`

### Phase 4: Bootstrap Loader (1 день)

**Задачи:**
- [ ] **4.1** Embeddings loader:
  ```python
  # Load GloVe
  runtime.bootstrap(
      "glove.6B.50d.txt",
      format="glove",
      limit=50000
  )

  # Load Word2Vec
  runtime.bootstrap(
      "GoogleNews-vectors-negative300.bin",
      format="word2vec",
      binary=True
  )
  ```

- [ ] **4.2** Progress bar (tqdm)
- [ ] **4.3** Format auto-detection
- [ ] **4.4** Validation & error handling

**Файлы:**
- `neurograph/bootstrap.py`

### Phase 5: Documentation & Testing (1 день)

**Задачи:**
- [ ] **5.1** Sphinx documentation
- [ ] **5.2** API reference (auto-generated)
- [ ] **5.3** Comprehensive docstrings
- [ ] **5.4** Unit tests (80%+ coverage)
- [ ] **5.5** Integration tests
- [ ] **5.6** Examples:
  - Basic usage
  - Advanced querying
  - Bootstrap flow
- [ ] **5.7** CHANGELOG_v0.59.0.md

**Deliverables:**
- ✅ `pip install neurograph` работает
- ✅ Runtime API полнофункционален
- ✅ Query Engine работает
- ✅ Bootstrap loader поддерживает GloVe/Word2Vec
- ✅ Документация полная
- ✅ 80%+ test coverage

**KPI:**
| Метрика | Target | Critical |
|---------|--------|----------|
| Query latency | < 100ms | < 500ms |
| Bootstrap speed | > 10K/sec | > 1K/sec |
| API usability | 9/10 | 7/10 |
| Test coverage | > 80% | > 60% |

---

## v0.60.0 - WebSocket & Real-time Events 🔄

**Цель:** Добавить real-time communication для live updates

**Приоритет:** 🟡 ВЫСОКИЙ

**Длительность:** 3-4 дня

### Phase 1: WebSocket Infrastructure (1 день)

**Задачи:**
- [ ] **1.1** WebSocket endpoint `/ws`:
  ```python
  @app.websocket("/ws")
  async def websocket_endpoint(websocket: WebSocket):
      await websocket.accept()
      # Connection handling
  ```

- [ ] **1.2** Connection manager:
  ```python
  class ConnectionManager:
      def connect(self, websocket: WebSocket, client_id: str)
      def disconnect(self, client_id: str)
      def broadcast(self, message: dict)
      def send_personal(self, message: dict, client_id: str)
  ```

- [ ] **1.3** Authentication для WebSocket (JWT в query params)
- [ ] **1.4** Heartbeat/ping-pong mechanism

**Файлы:**
- `src/api/websocket/connection.py`
- `src/api/websocket/manager.py`

### Phase 2: Event Streaming (1 день)

**Задачи:**
- [ ] **2.1** Subscribe to channels:
  ```json
  // Client → Server
  {
    "type": "subscribe",
    "channels": ["metrics", "signals", "actions"]
  }
  ```

- [ ] **2.2** Stream events:
  ```json
  // Server → Client
  {
    "channel": "signals",
    "event": {
      "event_id": "...",
      "event_type": "signal.input.text",
      "is_novel": true,
      "timestamp": "..."
    }
  }
  ```

- [ ] **2.3** Channels:
  - `metrics` - system metrics stream
  - `signals` - signal events stream
  - `actions` - action execution stream
  - `logs` - system logs stream

**Файлы:**
- `src/api/websocket/channels.py`
- `src/api/websocket/events.py`

### Phase 3: Integration с Core (1 день)

**Задачи:**
- [ ] **3.1** SignalSystem subscription → WebSocket push
- [ ] **3.2** Metrics updates → WebSocket broadcast
- [ ] **3.3** Action execution → WebSocket notification
- [ ] **3.4** Event buffering (если клиент отключён)

**Файлы:**
- `src/api/websocket/integrations.py`

### Phase 4: Testing & Client Library (1 день)

**Задачи:**
- [ ] **4.1** WebSocket tests
- [ ] **4.2** JavaScript/TypeScript client:
  ```typescript
  const client = new NeurographWSClient("ws://localhost:8000/ws");

  await client.connect(token);

  client.subscribe("metrics", (data) => {
    console.log("Metrics:", data);
  });
  ```

- [ ] **4.3** Python client (websockets)
- [ ] **4.4** Reconnection logic
- [ ] **4.5** CHANGELOG_v0.60.0.md

**Deliverables:**
- ✅ WebSocket endpoint работает
- ✅ Real-time event streaming
- ✅ Channel subscription система
- ✅ Client libraries (JS + Python)
- ✅ Integration с Core events

**KPI:**
| Метрика | Target | Critical |
|---------|--------|----------|
| Connection latency | < 50ms | < 200ms |
| Event latency | < 10ms | < 50ms |
| Concurrent connections | > 1000 | > 100 |
| Events throughput | > 10K/sec | > 1K/sec |

---

## v0.61.0 - Jupyter Integration 📊

**Цель:** IPython extension для interactive development

**Приоритет:** 🟢 СРЕДНИЙ

**Длительность:** 3-4 дня

### Phase 1: IPython Extension (1 день)

**Задачи:**
- [ ] **1.1** Extension loader:
  ```python
  %load_ext neurograph
  ```

- [ ] **1.2** Magic commands:
  ```python
  %ng_status              # System status
  %ng_query cat           # Quick query
  %ng_stats               # Statistics
  %ng_config grid_size=2000  # Configuration
  ```

- [ ] **1.3** Cell magic:
  ```python
  %%ng_explore
  query("machine learning")
  visualize_graph()
  ```

**Файлы:**
- `neurograph/integrations/jupyter.py`
- `neurograph/integrations/magic.py`

### Phase 2: Rich Display (1 день)

**Задачи:**
- [ ] **2.1** `_repr_html_()` для QueryResult:
  ```python
  result = runtime.query("cat")
  result  # Beautiful table в Jupyter
  ```

- [ ] **2.2** Interactive tables
- [ ] **2.3** Syntax highlighting для code
- [ ] **2.4** Export to DataFrame

**Файлы:**
- `neurograph/integrations/display.py`

### Phase 3: Visualization (1 день)

**Задачи:**
- [ ] **3.1** Graph visualization:
  ```python
  result = runtime.query("cat")
  result.visualize()  # Interactive plot
  ```

- [ ] **3.2** Plotly integration
- [ ] **3.3** NetworkX graph rendering
- [ ] **3.4** 3D visualization (опционально)

**Файлы:**
- `neurograph/integrations/viz.py`

### Phase 4: Tutorial Notebooks (1 день)

**Задачи:**
- [ ] **4.1** Getting Started.ipynb
- [ ] **4.2** Semantic Search.ipynb
- [ ] **4.3** Advanced Queries.ipynb
- [ ] **4.4** Visualization Examples.ipynb
- [ ] **4.5** CHANGELOG_v0.61.0.md

**Deliverables:**
- ✅ IPython extension работает
- ✅ Magic commands функциональны
- ✅ Rich display красивый
- ✅ Visualization работает
- ✅ 4+ tutorial notebooks

**KPI:**
| Метрика | Target | Critical |
|---------|--------|----------|
| Extension load time | < 1s | < 3s |
| Magic command latency | < 100ms | < 500ms |
| Visualization quality | 9/10 | 7/10 |
| Tutorial completeness | 100% | 80% |

---

## v0.62.0 - Web Dashboard Foundation (React SPA) 🎨

**Цель:** Создать Tiro Control Center - веб-панель управления

**Приоритет:** 🟢 СРЕДНИЙ

**Длительность:** 7-10 дней

### Phase 1: Project Setup (1 день)

**Задачи:**
- [ ] **1.1** Create React App + TypeScript
- [ ] **1.2** Ant Design Pro setup
- [ ] **1.3** Folder structure:
  ```
  src/web/
  ├── package.json
  ├── tsconfig.json
  ├── src/
  │   ├── components/
  │   │   ├── MetricCard/
  │   │   ├── SystemChart/
  │   │   └── ActivityTable/
  │   ├── pages/
  │   │   ├── Dashboard/
  │   │   ├── Signals/
  │   │   ├── Config/
  │   │   └── Admin/
  │   ├── services/
  │   │   ├── api.ts
  │   │   └── websocket.ts
  │   ├── stores/
  │   │   ├── runtime.ts
  │   │   └── auth.ts
  │   ├── App.tsx
  │   └── index.tsx
  └── public/
  ```

- [ ] **1.4** Router (React Router v6)
- [ ] **1.5** State management (Zustand)
- [ ] **1.6** API client (axios)

**Файлы:**
- `src/web/package.json`
- `src/web/src/App.tsx`

### Phase 2: Dashboard Page (2 дня)

**Задачи:**
- [ ] **2.1** Metrics cards:
  - System Status (Running/Stopped)
  - Total Tokens
  - Total Connections
  - Active Signals

- [ ] **2.2** Charts (recharts):
  - System metrics (CPU, Memory) - line chart
  - Signal rate (events/sec) - area chart
  - Action distribution - pie chart

- [ ] **2.3** Recent activity table (ProTable):
  - Recent signals
  - Recent actions
  - Pagination

- [ ] **2.4** Auto-refresh (5 sec interval)

**Компоненты:**
```tsx
<Dashboard>
  <MetricsRow>
    <MetricCard title="Status" value="Running" icon={<CheckCircle />} />
    <MetricCard title="Tokens" value="50,000" trend="+5%" />
  </MetricsRow>
  <ChartsRow>
    <SystemMetricsChart />
    <SignalRateChart />
  </ChartsRow>
  <RecentActivityTable />
</Dashboard>
```

**Файлы:**
- `src/web/src/pages/Dashboard/index.tsx`
- `src/web/src/components/MetricCard.tsx`
- `src/web/src/components/charts/`

### Phase 3: Signals Page (2 дня)

**Задачи:**
- [ ] **3.1** Real-time signal stream (WebSocket)
- [ ] **3.2** Signal list (ProTable):
  - Event ID
  - Event Type
  - Priority
  - Is Novel
  - Timestamp

- [ ] **3.3** Signal details modal:
  - Full event data
  - 8D vector visualization
  - Neighbors list
  - Triggered actions

- [ ] **3.4** Filters:
  - By event type
  - By priority range
  - Only novel
  - Time range

**Файлы:**
- `src/web/src/pages/Signals/index.tsx`
- `src/web/src/components/SignalDetails.tsx`

### Phase 4: Config & Admin (2 дня)

**Задачи:**
- [ ] **4.1** Config editor (ProForm):
  - Grid size
  - Dimensions
  - CDNA scales
  - Guardian settings

- [ ] **4.2** Bootstrap uploader:
  - File upload (drag & drop)
  - Format selection (GloVe/Word2Vec)
  - Progress bar
  - Validation

- [ ] **4.3** System logs viewer:
  - Real-time logs (WebSocket)
  - Log level filter
  - Search
  - Export

- [ ] **4.4** Settings persistence (localStorage)

**Файлы:**
- `src/web/src/pages/Config/index.tsx`
- `src/web/src/pages/Admin/index.tsx`

### Phase 5: Polish & Deploy (2 дня)

**Задачи:**
- [ ] **5.1** Dark/Light theme toggle
- [ ] **5.2** Responsive layout (mobile)
- [ ] **5.3** Error boundaries
- [ ] **5.4** Loading states (Skeleton)
- [ ] **5.5** Production build optimization
- [ ] **5.6** Nginx config
- [ ] **5.7** Docker для frontend
- [ ] **5.8** CHANGELOG_v0.62.0.md

**Deliverables:**
- ✅ Tiro Control Center работает
- ✅ Dashboard page полнофункционален
- ✅ Signals page с real-time updates
- ✅ Config & Admin pages работают
- ✅ Dark/Light themes
- ✅ Mobile responsive
- ✅ Production ready

**KPI:**
| Метрика | Target | Critical |
|---------|--------|----------|
| Load time | < 2s | < 5s |
| Time to interactive | < 3s | < 7s |
| Lighthouse score | > 90 | > 70 |
| Mobile usability | 100% | 80% |

---

## v0.63.0 - Enhanced Sensors (Audio & Vision) 🎥

**Цель:** Расширить сенсорные модальности (аудио, видео)

**Приоритет:** 🟢 НИЗКИЙ

**Длительность:** 5-7 дней

### Phase 1: Audio Input (2-3 дня)

**Задачи:**
- [ ] **1.1** Audio adapter:
  ```python
  gateway.push_audio(
      audio_data=audio_array,
      sample_rate=16000,
      source="microphone"
  )
  ```

- [ ] **1.2** AUDIO_MEL encoder (Mel spectrogram)
- [ ] **1.3** AUDIO_MFCC encoder (MFCC features)
- [ ] **1.4** Integration с speech recognition (Whisper)
- [ ] **1.5** Real-time audio streaming support

**Файлы:**
- `src/gateway/adapters/audio.py`
- `src/gateway/encoders/audio.py`

### Phase 2: Vision Input (2-3 дня)

**Задачи:**
- [ ] **2.1** Vision adapter:
  ```python
  gateway.push_vision(
      image_data=image_array,
      source="camera"
  )
  ```

- [ ] **2.2** IMAGE_CNN encoder (ResNet features)
- [ ] **2.3** IMAGE_CLIP encoder (CLIP embeddings)
- [ ] **2.4** Real-time camera feed support

**Файлы:**
- `src/gateway/adapters/vision.py`
- `src/gateway/encoders/vision.py`

### Phase 3: Multi-modal Fusion (1 день)

**Задачи:**
- [ ] **3.1** Multi-modal event:
  ```python
  event = gateway.push_multimodal(
      text="What is this?",
      image=image_data,
      audio=audio_data
  )
  ```

- [ ] **3.2** Fusion strategies:
  - Early fusion (concatenate features)
  - Late fusion (weighted average)
  - Attention-based fusion

**Файлы:**
- `src/gateway/fusion/multimodal.py`

### Phase 4: Testing & Examples (1 день)

**Задачи:**
- [ ] **4.1** Audio integration tests
- [ ] **4.2** Vision integration tests
- [ ] **4.3** Multi-modal examples
- [ ] **4.4** CHANGELOG_v0.63.0.md

**Deliverables:**
- ✅ Audio input поддерживается
- ✅ Vision input поддерживается
- ✅ Multi-modal fusion работает
- ✅ Real-time streaming

**KPI:**
| Метрика | Target | Critical |
|---------|--------|----------|
| Audio encoding | < 50ms | < 200ms |
| Vision encoding | < 100ms | < 500ms |
| Multi-modal latency | < 200ms | < 1s |
| Accuracy | > 85% | > 70% |

---

## 📋 Overall Timeline

| Version | Track | Feature | Duration | Priority |
|---------|-------|---------|----------|----------|
| **v0.57.0** | A | Gateway-Core Integration | ✅ DONE | 🔴 |
| **v0.58.0** | C | Authentication & Security | 5-7 дней | 🔴 |
| **v0.59.0** | B | Python Library | 5-7 дней | 🟡 |
| **v0.60.0** | C | WebSocket & Real-time | 3-4 дня | 🟡 |
| **v0.61.0** | B | Jupyter Integration | 3-4 дня | 🟢 |
| **v0.62.0** | C | Web Dashboard Foundation | 7-10 дней | 🟢 |
| **v0.63.0** | A | Enhanced Sensors | 5-7 дней | 🟢 |

**TOTAL:** ~35-45 дней (1.5-2 месяца)

---

## 🎯 Immediate Next Steps

### Сегодня (2024-12-26):
1. ✅ Создать MASTER_PLAN v3.0
2. ✅ Архивировать старые планы
3. 🔧 Начать v0.58.0 Phase 1: JWT Authentication

### Эта неделя:
- Завершить v0.58.0 Phase 1-2 (JWT + RBAC)
- Начать v0.58.0 Phase 3 (API Keys & Rate Limiting)

### Следующая неделя:
- Завершить v0.58.0 (Security Hardening + Testing)
- Начать v0.59.0 (Python Library)

---

## 🏗️ Архитектурные решения (ADR)

### ADR-001: Security First для v0.58
**Дата:** 2024-12-26
**Проблема:** REST API не защищён, нельзя деплоить в production
**Решение:** Приоритизировать v0.58.0 (Auth & Security) перед всем остальным
**Статус:** ✅ Принято

### ADR-002: Python Library перед Web Dashboard
**Дата:** 2024-12-26
**Проблема:** Какой трек разрабатывать первым - B или C?
**Решение:** Developer Experience (B) важнее User Interfaces (C) на текущем этапе
**Статус:** ✅ Принято

### ADR-003: WebSocket integration в v0.60
**Дата:** 2024-12-26
**Проблема:** WebSocket нужен для Web Dashboard, но Dashboard в v0.62
**Решение:** Реализовать WebSocket раньше (v0.60), чтобы Dashboard мог его использовать
**Статус:** ✅ Принято

### ADR-004: Jupyter перед Web Dashboard
**Дата:** 2024-12-26
**Проблема:** Jupyter (v0.61) vs Web Dashboard (v0.62) - что первым?
**Решение:** Jupyter проще и полезнее для разработчиков сейчас
**Статус:** ✅ Принято

### ADR-005: Enhanced Sensors - Low Priority
**Дата:** 2024-12-26
**Проблема:** Audio/Vision нужны, но не критичны
**Решение:** Отложить на v0.63+, сфокусироваться на инфраструктуре
**Статус:** ✅ Принято

---

## ✅ Success Metrics

### v0.58.0 (Authentication):
- [ ] JWT auth latency < 5ms
- [ ] RBAC enforcement works
- [ ] Rate limiting handles 1000 req/min
- [ ] OWASP security score > 7/10

### v0.59.0 (Python Library):
- [ ] `pip install neurograph` works
- [ ] Query latency < 100ms
- [ ] Test coverage > 80%
- [ ] Documentation complete

### v0.60.0 (WebSocket):
- [ ] Connection latency < 50ms
- [ ] Event latency < 10ms
- [ ] Supports > 1000 concurrent connections
- [ ] Events throughput > 10K/sec

### v0.61.0 (Jupyter):
- [ ] Extension loads < 1s
- [ ] Magic commands work
- [ ] Rich display beautiful
- [ ] 4+ tutorial notebooks

### v0.62.0 (Web Dashboard):
- [ ] Load time < 2s
- [ ] Lighthouse score > 90
- [ ] Mobile responsive
- [ ] All pages functional

### v0.63.0 (Enhanced Sensors):
- [ ] Audio encoding < 50ms
- [ ] Vision encoding < 100ms
- [ ] Multi-modal latency < 200ms
- [ ] Accuracy > 85%

---

## 📚 References

**Current State:**
- [README.md](../README.md) - Project overview (v0.57.0)
- [CHANGELOG v0.57.0](changelogs/CHANGELOG_v0.57.0.md) - Latest release

**Guides:**
- [Getting Started](guides/GETTING_STARTED.md)
- [Gateway v2.0 Guide](guides/GATEWAY_GUIDE.md)
- [SignalSystem Guide](guides/SIGNAL_SYSTEM_GUIDE.md)

**Specifications:**
- [docs/specs/](specs/) - Technical specs

**Archives:**
- [MASTER_PLAN_v2.1.md](archive/MASTER_PLAN_v2.1.md) - Signal Processing Focus (старый)
- [IMPLEMENTATION_ROADMAP.md](archive/IMPLEMENTATION_ROADMAP.md) - Full Stack Focus (старый)

---

## 📝 Notes

- Весь код под AGPLv3 + Commercial dual licensing
- Документация на русском (код на английском)
- Все коммиты с Claude Code footer
- Тесты обязательны для каждой версии
- Security first - нельзя деплоить без аутентификации

---

**Философия v3.0:** NeuroGraph OS как **платформа** для когнитивных приложений. Три трека (Intelligence, Developer Experience, User Interfaces) развиваются параллельно для создания полноценной экосистемы.

---

**Конец Master Plan v3.0. Let's build! 🚀**

---

*Создано: 2024-12-26*
*Автор: Claude Sonnet 4.5 + Chernov Denys*
*Статус: Living Document - обновляется по мере прогресса*
