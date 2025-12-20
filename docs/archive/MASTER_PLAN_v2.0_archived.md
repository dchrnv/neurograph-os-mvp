# NeuroGraph OS - Мастер-план развития

**Версия:** 2.0
**Дата:** 2024-12-17
**Статус:** Active - Unified Plan
**Базовые документы:**
- `/docs/IMPLEMENTATION_ROADMAP.md` - общий roadmap (4 фазы)
- `/docs/plan v 0.49.x.md` - план REST API v0.49-0.52
- `/docs/PYRUNTIME_FIX_INSTRUCTIONS 2.md` - архитектурные исправления

---

## 🎯 Общая стратегия

Построить полноценную систему NeuroGraph OS по слоям:

```
Core (Rust) ✅ → Library (Python) → REST API → Web Dashboard + Jupyter
```

**Текущий статус:** v0.49.0 завершён, работаем над v0.50.0 (Runtime Integration)

---

## 📊 Текущее состояние (2024-12-17)

### ✅ Что работает (v0.49.0)

#### 1. Rust Core (neurograph-core v0.45.0+)
- Token V2.0 (64 bytes, 8D coordinates)
- Connection V3.0
- Grid V2.0 (spatial indexing)
- Graph (nodes/edges)
- IntuitionEngine v3.0
- Guardian + CDNA V2.1
- Prometheus metrics
- Bootstrap (semantic embeddings loader)

#### 2. REST API v0.49.0 (30 endpoints)
- **Token Router** - 10 endpoints (CRUD + batch)
- **Grid Router** - 10 endpoints (spatial queries)
- **CDNA Router** - 10 endpoints (config management)
- **Storage:** InMemory (временное решение)

#### 3. PyRuntime (базовые методы)
- `bootstrap(path)` - загрузка embeddings
- `query(text)` - семантический поиск
- `feedback()` - обратная связь
- `export_metrics()` - метрики

### ❌ Чего НЕТ (блокеры для v0.50.0)

1. **Token CRUD в Rust** - методы не реализованы в Graph
2. **Grid runtime** - Grid есть только semantic (в BootstrapLibrary)
3. **CDNA storage** - CDNA не добавлена в BootstrapLibrary
4. **RuntimeStorage** - классы только заглушки
5. **Integration** - REST API не подключен к Rust core

---

## 🗺️ Roadmap (6 треков параллельно)

---

## ТРЕК A: REST API (v0.49 → v0.52)

**Цель:** Довести REST API до production-ready состояния

---

### ✅ v0.49.0 - CRUD API Foundation (DONE)

**Дата:** 2024-12-14
**Статус:** ✅ Завершено

**Что сделано:**
- 30 endpoints (Token/Grid/CDNA)
- InMemory storage
- FastAPI structure
- OpenAPI docs
- Response models

**Файлы:**
```
src/api/
├── routers/
│   ├── token.py (10 endpoints)
│   ├── grid.py (10 endpoints)
│   └── cdna.py (10 endpoints)
└── storage/
    ├── in_memory.py (работает)
    └── runtime.py (заглушки)
```

---

### 🔧 v0.50.0 - Runtime Integration (IN PROGRESS)

**Дата:** 2024-12-17 (сейчас)
**Срок:** 3-4 дня
**Приоритет:** 🔴 КРИТИЧЕСКИЙ

**Цель:** Подключить реальный Rust core вместо InMemory

#### Архитектурные исправления (Phase 0)

**Проблема:** PyRuntime архитектура не готова для CRUD операций

**Решение:**
1. ⏳ Добавить Token storage в Graph (Rust)
2. ⏳ Добавить CDNA в BootstrapLibrary (Rust)
3. ⏳ Определить Grid стратегию (semantic vs runtime)
4. ⏳ Реализовать FFI методы в PyRuntime

**Файлы:**
- `src/core_rust/src/graph.rs` - добавить Token methods
- `src/core_rust/src/bootstrap.rs` - добавить CDNA field
- `src/core_rust/src/python/runtime.rs` - CRUD методы
- `docs/ARCHITECTURE_DECISIONS.md` - документировать решения

#### Phase 1: Rust Core расширение (2 дня)

**Задачи:**
- [ ] **1.1** Добавить Token storage в Graph:
  ```rust
  pub struct Graph {
      // существующие поля
      tokens: HashMap<u32, Token>,  // NEW
  }

  impl Graph {
      pub fn add_token(&mut self, token: Token) -> Result<()>
      pub fn get_token(&self, id: u32) -> Option<&Token>
      pub fn update_token(&mut self, id: u32, updates: TokenUpdate)
      pub fn delete_token(&mut self, id: u32) -> Option<Token>
      pub fn list_tokens(&self, limit: usize, offset: usize) -> Vec<&Token>
      pub fn count_tokens(&self) -> usize
      pub fn clear_tokens(&mut self) -> usize
  }
  ```

- [ ] **1.2** Добавить CDNA в BootstrapLibrary:
  ```rust
  pub struct BootstrapLibrary {
      // существующие поля
      cdna: CDNA,  // NEW
  }

  impl BootstrapLibrary {
      pub fn cdna(&self) -> &CDNA
      pub fn cdna_mut(&mut self) -> &mut CDNA
  }
  ```

- [ ] **1.3** Решить Grid вопрос:
  - **Вариант A:** Добавить runtime Grid в Graph
  - **Вариант B:** Использовать semantic Grid из BootstrapLibrary
  - Документировать решение

- [ ] **1.4** Реализовать PyRuntime CRUD методы (21 метод):

  **Token (7):**
  - `create_token()`, `get_token()`, `list_tokens()`
  - `update_token()`, `delete_token()`
  - `count_tokens()`, `clear_tokens()`

  **Grid (6):**
  - `get_grid_info()`, `add_token_to_grid()`, `remove_token_from_grid()`
  - `find_neighbors()`, `range_query()`, `calculate_field_influence()`, `calculate_density()`

  **CDNA (8):**
  - `get_cdna_config()`, `update_cdna_scales()`
  - `get_cdna_profile()`, `set_cdna_profile()`
  - `get_cdna_flags()`, `set_cdna_flags()`
  - `validate_cdna_scales()`, `reset_cdna()`

- [ ] **1.5** Тестирование Rust:
  ```bash
  cargo build --release --features python-bindings
  cargo test
  ```

#### Phase 2: Python Runtime Storage (1 день)

**Задачи:**
- [ ] **2.1** Реализовать RuntimeTokenStorage:
  ```python
  class RuntimeTokenStorage(TokenStorageInterface):
      def __init__(self, runtime: Runtime):
          self.runtime = runtime._core

      def create(self, token: Token) -> Token:
          result = self.runtime.create_token(...)
          return Token.from_dict(result)

      # + остальные методы
  ```

- [ ] **2.2** Реализовать RuntimeGridStorage:
  ```python
  class RuntimeGridStorage(GridStorageInterface):
      # аналогично
  ```

- [ ] **2.3** Реализовать RuntimeCDNAStorage:
  ```python
  class RuntimeCDNAStorage(CDNAStorageInterface):
      # аналогично
  ```

- [ ] **2.4** Обновить dependencies:
  ```python
  # src/api/dependencies.py

  def get_token_storage() -> TokenStorageInterface:
      if settings.USE_RUNTIME:
          return RuntimeTokenStorage(runtime)
      return InMemoryTokenStorage()
  ```

#### Phase 3: Интеграция и тестирование (0.5 дня)

**Задачи:**
- [ ] **3.1** Пересобрать Python bindings:
  ```bash
  maturin develop --release
  ```

- [ ] **3.2** Integration tests:
  ```python
  def test_token_crud_via_api(client):
      # Create
      response = client.post("/api/v1/tokens", json={...})
      token_id = response.json()["data"]["id"]

      # Read
      response = client.get(f"/api/v1/tokens/{token_id}")
      assert response.status_code == 200

      # Update
      response = client.put(f"/api/v1/tokens/{token_id}", json={...})

      # Delete
      response = client.delete(f"/api/v1/tokens/{token_id}")
  ```

- [ ] **3.3** Performance testing:
  - Token CRUD: < 10ms per operation
  - Grid queries: < 50ms
  - Bulk operations: 1000 tokens/sec

#### Phase 4: Документация и коммит (0.5 дня)

**Задачи:**
- [ ] **4.1** Создать CHANGELOG_v0.50.0.md
- [ ] **4.2** Обновить README
- [ ] **4.3** Git commit:
  ```bash
  git add .
  git commit -m "feat: REST API v0.50.0 - Runtime Integration Complete

  - Add Token storage to Graph
  - Add CDNA to BootstrapLibrary
  - Implement 21 PyRuntime CRUD methods
  - Create Runtime storage classes
  - Full integration with Rust core

  🤖 Generated with Claude Code
  Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
  ```

**Deliverables:**
- ✅ REST API работает с Rust core
- ✅ InMemory storage заменён на Runtime
- ✅ 30 endpoints используют реальные данные
- ✅ Persistence работает

---

### 📋 v0.51.0 - Enhanced System + Auth (NEXT)

**Срок:** 2-3 дня
**Приоритет:** 🟡 ВЫСОКИЙ

**Задачи:**

#### Enhanced System Endpoints
- [ ] `/health` - health check с реальными данными
- [ ] `/status` - детальный статус системы
- [ ] `/metrics` - Prometheus metrics из Rust

#### Authentication & Security
- [ ] JWT authentication (`/api/v1/auth/token`)
- [ ] Middleware для проверки токенов
- [ ] Role-based access control (admin/user/readonly)
- [ ] API keys support
- [ ] Rate limiting (slowapi)

#### Admin Endpoints Protection
- [ ] Protect POST/PUT/DELETE endpoints
- [ ] Admin-only operations
- [ ] Audit logging

**Deliverables:**
- ✅ JWT auth работает
- ✅ RBAC реализован
- ✅ Admin endpoints защищены

---

### 🔌 v0.52.0 - WebSocket Support (FINAL)

**Срок:** 1-2 дня
**Приоритет:** 🟢 СРЕДНИЙ

**Задачи:**
- [ ] WebSocket endpoint `/ws`
- [ ] Event streaming:
  - `token.created`, `token.updated`, `token.deleted`
  - `grid.query`, `cdna.updated`
- [ ] Live metrics broadcasting
- [ ] Heartbeat/reconnect logic

**Пример:**
```python
# Client
async with websockets.connect("ws://localhost:8000/ws") as ws:
    await ws.send(json.dumps({"subscribe": "metrics"}))
    while True:
        msg = await ws.recv()
        print(json.loads(msg))
```

**Deliverables:**
- ✅ WebSocket endpoint работает
- ✅ Events стримятся
- ✅ Frontend integration ready

---

## ТРЕК B: Python Library (Phase 1 Roadmap)

**Цель:** Создать `neurograph` Python package

**Срок:** 5-7 дней
**Приоритет:** 🟡 СРЕДНИЙ (может идти параллельно с REST API)

---

### B.1 Project Setup (1 день)

**Статус:** Частично выполнено

**Что есть:**
- ✅ `src/python/neurograph/` структура
- ✅ PyO3 bindings начаты
- ✅ `_core.so` компилируется

**Что нужно:**
- [ ] Полный `pyproject.toml` с maturin
- [ ] Proper package structure
- [ ] GitHub Actions для Python package
- [ ] PyPI publishing setup

---

### B.2 PyO3 FFI Bindings (2 дня)

**Статус:** В процессе (v0.50.0)

**Что нужно:**
- [ ] Завершить все FFI методы (21+ методов)
- [ ] Error handling (Rust → Python)
- [ ] Type hints для всех методов
- [ ] FFI tests

---

### B.3 Python Runtime Manager (1 день)

**Задачи:**
- [ ] Класс `Runtime` в `runtime.py`
- [ ] Lifecycle management (start/stop/status)
- [ ] Context manager support:
  ```python
  with ng.Runtime() as runtime:
      result = runtime.query("hello")
  ```
- [ ] Configuration management

---

### B.4 Query Engine & Bootstrap (1 день)

**Задачи:**
- [ ] `QueryResult` класс
- [ ] `query(text, limit=10)` метод
- [ ] Bootstrap loader с progress bar
- [ ] Support GloVe, Word2Vec formats

---

### B.5 Testing & Documentation (1 день)

**Задачи:**
- [ ] Unit tests (pytest)
- [ ] Integration tests
- [ ] Sphinx documentation
- [ ] README with Quick Start
- [ ] 80%+ coverage

**Deliverables:**
- ✅ `neurograph` package на PyPI
- ✅ Full documentation
- ✅ Examples

---

## ТРЕК C: Web Dashboard (Phase 3 Roadmap)

**Цель:** Tiro Control Center - React SPA

**Срок:** 7-10 дней
**Приоритет:** 🟢 НИЗКИЙ (после REST API)

---

### C.1 Project Setup (1 день)
- [ ] Create React App + TypeScript
- [ ] Ant Design Pro
- [ ] Router + State management (Zustand)
- [ ] API client (axios)

### C.2 Dashboard Page (2 дня)
- [ ] Metrics cards
- [ ] Charts (CPU, Memory, Events)
- [ ] Recent activity table
- [ ] Auto-refresh

### C.3 Modules Management (1.5 дня)
- [ ] Modules list (ProTable)
- [ ] Start/Stop/Restart actions
- [ ] Module configuration

### C.4 Chat & Terminal (2 дня)
- [ ] Chat interface
- [ ] Terminal (xterm.js)
- [ ] WebSocket integration

### C.5 Config & Admin (1.5 дня)
- [ ] Config editor
- [ ] Bootstrap uploader
- [ ] CDNA management
- [ ] System logs viewer

### C.6 Polish & Deploy (2 дня)
- [ ] Dark/Light themes
- [ ] Responsive layout
- [ ] Production build
- [ ] Docker

**Deliverables:**
- ✅ Tiro Control Center deployed
- ✅ All features working

---

## ТРЕК D: Jupyter Integration (Phase 4 Roadmap)

**Срок:** 2-3 дня
**Приоритет:** 🟢 НИЗКИЙ

### D.1 IPython Extension (1 день)
- [ ] Magic commands (`%ng_query`, `%ng_status`)
- [ ] Cell magic (`%%ng_explore`)

### D.2 Rich Display (1 день)
- [ ] `_repr_html_()` для QueryResult
- [ ] DataFrame export

### D.3 Visualization (0.5 дня)
- [ ] Graph viz (networkx + plotly)
- [ ] Interactive plots

### D.4 Documentation (0.5 дня)
- [ ] Jupyter notebook examples
- [ ] Tutorial

**Deliverables:**
- ✅ Jupyter extension
- ✅ Rich displays
- ✅ Examples

---

## ТРЕК E: Архитектурные улучшения

**Приоритет:** 🟡 ПОСТОЯННЫЙ

### E.1 Grid Refactoring (v0.51.0+)
- [ ] Разделить semantic Grid и runtime Grid
- [ ] Runtime Grid в Graph
- [ ] Update Grid API

### E.2 CDNA System (v0.51.0+)
- [ ] CDNA validation layer
- [ ] Quarantine mode implementation
- [ ] Profile system
- [ ] History tracking

### E.3 Performance (ongoing)
- [ ] Benchmark suite
- [ ] Profiling (Rust + Python)
- [ ] Optimization hotspots

---

## ТРЕК F: Deployment & Infra

**Приоритет:** 🟢 СРЕДНИЙ

### F.1 Docker
- [x] Rust build stage
- [ ] Python package stage
- [ ] API service container
- [ ] Web app container
- [ ] Docker Compose

### F.2 CI/CD
- [ ] GitHub Actions:
  - Rust tests
  - Python tests
  - Build Docker images
  - Deploy to production

### F.3 Monitoring
- [ ] Prometheus setup
- [ ] Grafana dashboards
- [ ] Jaeger tracing

---

## 📋 Timeline (общий)

| Трек | Задача | Срок | Статус |
|------|--------|------|--------|
| **A** | v0.49.0 CRUD API | Week 1 | ✅ Done |
| **A** | v0.50.0 Runtime Integration | Week 2-3 | 🔧 In Progress |
| **B** | Python Library Phase 1 | Week 2-4 | ⏳ Pending |
| **A** | v0.51.0 Enhanced + Auth | Week 4 | ⏳ Pending |
| **A** | v0.52.0 WebSocket | Week 4 | ⏳ Pending |
| **C** | Web Dashboard | Week 5-7 | ⏳ Pending |
| **D** | Jupyter Integration | Week 7 | ⏳ Pending |
| **E/F** | Infra + Deploy | Ongoing | ⏳ Pending |

**TOTAL:** ~2 месяца до полного production

---

## 🎯 Immediate Next Steps (сейчас)

### Сегодня (2024-12-17):
1. ✅ Создать MASTER_PLAN.md (этот файл)
2. ⏳ Принять архитектурные решения для v0.50.0:
   - Где хранить runtime токены?
   - Как разделить semantic/runtime Grid?
   - Куда добавить CDNA?
3. ⏳ Начать Phase 0: Архитектурные исправления

### Эта неделя:
- Завершить v0.50.0 (Runtime Integration)
- Первый working E2E test (API → Rust → Storage)
- CHANGELOG v0.50.0

### Этот месяц:
- v0.51.0 + v0.52.0 (Enhanced + WebSocket)
- Python Library Phase 1 complete
- Начать Web Dashboard

---

## 📝 Важные решения (Architecture Decision Records)

### ADR-001: Token Storage Location
**Проблема:** Где хранить runtime токены?
**Решение:** TBD
**Дата:** 2024-12-17

### ADR-002: Grid Separation
**Проблема:** Semantic vs Runtime Grid
**Решение:** TBD
**Дата:** 2024-12-17

### ADR-003: CDNA Storage
**Проблема:** Где хранить CDNA конфигурацию?
**Решение:** В BootstrapLibrary
**Дата:** 2024-12-17

---

## ✅ Success Metrics

### v0.50.0:
- [ ] REST API работает с Rust core
- [ ] Latency < 50ms (p95)
- [ ] All 30 endpoints functional
- [ ] Integration tests pass

### Python Library:
- [ ] `pip install neurograph` works
- [ ] Query < 100ms
- [ ] 80%+ test coverage

### Web Dashboard:
- [ ] Load < 2 sec
- [ ] Lighthouse > 90
- [ ] Mobile responsive

### Production:
- [ ] 1000 req/sec sustained
- [ ] 99.9% uptime
- [ ] Full monitoring

---

## 🚀 References

**Документы:**
- `/docs/IMPLEMENTATION_ROADMAP.md` - общий план
- `/docs/plan v 0.49.x.md` - REST API план
- `/docs/PYRUNTIME_FIX_INSTRUCTIONS 2.md` - архитектура PyRuntime
- `/docs/arch/` - спецификации архитектуры

**Файлы:**
- `src/api/` - REST API service
- `src/core_rust/` - Rust core
- `src/python/neurograph/` - Python library
- `src/web/` - Web dashboard (future)

---

**Конец мастер-плана. Готовы к исполнению! 🚀**

---

*Создано: 2024-12-17*
*Автор: Claude Sonnet 4.5 + Opus 4.5*
*Статус: Living Document - обновляется по мере прогресса*
