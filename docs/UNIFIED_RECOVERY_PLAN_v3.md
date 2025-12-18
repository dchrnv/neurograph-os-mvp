# NeuroGraph OS — Unified Recovery Plan v3.0

**Версия:** 3.0  
**Дата:** 2024-12-17  
**Базовая версия:** v0.49.0 (после отката)  
**Статус:** ПЛАН ВОССТАНОВЛЕНИЯ

---

## 🚨 Диагноз проблемы

### Что пошло не так

1. **Архитектурные решения принимались "по ходу"** — не было чёткой схемы где что хранится
2. **Путаница между компонентами:**
   - `PyRuntime.graph` (Arc<Mutex<Graph>>) — граф связей (nodes/edges), **НЕ** хранилище токенов
   - `BootstrapLibrary.graph` — тот же Graph, но для semantic концептов
   - Token методы вызывались на Graph, но их там **нет**
3. **Grid дублирование** — непонятно, semantic Grid или runtime Grid использовать
4. **CDNA** — добавлена в BootstrapLibrary, но методы обращались к Graph
5. **Код не компилировался** — методы вызывали несуществующие функции

### Корень проблемы

**Отсутствует единое хранилище данных.** Graph в NeuroGraph — это только топология (nodes, edges), а не storage для Token/Connection.

---

## 🏗️ Архитектурное решение (ADR)

### ADR-001: Где хранить Runtime данные

**Решение:** Создать **RuntimeStorage** как отдельную структуру в Rust

```rust
/// Единое хранилище runtime данных
pub struct RuntimeStorage {
    // === Token Storage ===
    tokens: HashMap<u32, Token>,
    next_token_id: AtomicU32,
    
    // === Connection Storage ===
    connections: HashMap<u64, ConnectionV3>,
    next_connection_id: AtomicU64,
    
    // === Spatial Index ===
    grid: Grid,  // Runtime Grid для токенов
    
    // === Graph Topology ===
    graph: Graph,  // Связи между токенами (nodes/edges)
    
    // === Constitution ===
    cdna: CDNA,
    
    // === Caches ===
    label_to_id: HashMap<String, u32>,
    id_to_label: HashMap<u32, String>,
}
```

**Почему:**
- Чёткое разделение ответственности
- Один источник правды для runtime данных
- Graph остаётся чистым (только топология)
- Grid интегрирован с Token storage

### ADR-002: Semantic vs Runtime слои

**Решение:** Два отдельных слоя

```
┌────────────────────────────────────────────────────┐
│                   PyRuntime                         │
├────────────────────────────────────────────────────┤
│                                                     │
│  ┌─────────────────┐    ┌─────────────────────┐   │
│  │ RuntimeStorage  │    │  BootstrapLibrary   │   │
│  │ (динамические)  │    │  (статические)      │   │
│  ├─────────────────┤    ├─────────────────────┤   │
│  │ tokens          │    │ concepts (embeddings)│   │
│  │ connections     │    │ semantic_grid       │   │
│  │ runtime_grid    │    │ pca_model           │   │
│  │ graph           │    │ word_to_id          │   │
│  │ cdna            │    │                     │   │
│  └─────────────────┘    └─────────────────────┘   │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │              Unified API                     │   │
│  │  create_token() → RuntimeStorage            │   │
│  │  semantic_search() → BootstrapLibrary       │   │
│  │  query() → RuntimeStorage + Bootstrap       │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
└────────────────────────────────────────────────────┘
```

### ADR-003: Grid стратегия

**Решение:** Два Grid с разным назначением

| Grid | Расположение | Назначение | Данные |
|------|--------------|------------|--------|
| `semantic_grid` | BootstrapLibrary | KNN по word embeddings | Статические (GloVe) |
| `runtime_grid` | RuntimeStorage | Spatial queries по токенам | Динамические |

---

## 📋 Новый план реализации

### Фаза 0: Откат и подготовка (1 день)

```bash
# 1. Откат к стабильной версии
git stash  # Сохранить текущие изменения если нужно
git checkout v0.49.0  # или последний стабильный коммит

# 2. Создать новую ветку
git checkout -b feature/v0.50-architecture-fix

# 3. Проверить что всё компилируется
cd src/core_rust
cargo build --release
cargo test
```

**Deliverables:**
- [ ] Стабильная база для работы
- [ ] Новая ветка создана
- [ ] Все тесты проходят

---

### Фаза 1: RuntimeStorage в Rust (2 дня)

#### День 1: Создание RuntimeStorage

**Файл:** `src/core_rust/src/runtime_storage.rs` (НОВЫЙ)

```rust
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use parking_lot::RwLock;

use crate::token::Token;
use crate::connection_v3::ConnectionV3;
use crate::grid::Grid;
use crate::graph::Graph;
use crate::cdna::CDNA;

/// Unified runtime storage for all dynamic data
pub struct RuntimeStorage {
    // Token storage
    tokens: RwLock<HashMap<u32, Token>>,
    next_token_id: AtomicU32,
    
    // Connection storage
    connections: RwLock<HashMap<u64, ConnectionV3>>,
    next_connection_id: AtomicU64,
    
    // Spatial index for tokens
    grid: RwLock<Grid>,
    
    // Graph topology
    graph: RwLock<Graph>,
    
    // Constitution
    cdna: RwLock<CDNA>,
    
    // Label caches
    label_to_id: RwLock<HashMap<String, u32>>,
    id_to_label: RwLock<HashMap<u32, String>>,
}

impl RuntimeStorage {
    pub fn new() -> Self { ... }
    
    // === Token API ===
    pub fn create_token(&self, token: Token) -> u32 { ... }
    pub fn get_token(&self, id: u32) -> Option<Token> { ... }
    pub fn update_token(&self, id: u32, updates: TokenUpdate) -> Result<(), StorageError> { ... }
    pub fn delete_token(&self, id: u32) -> Option<Token> { ... }
    pub fn list_tokens(&self, limit: usize, offset: usize) -> Vec<Token> { ... }
    pub fn count_tokens(&self) -> usize { ... }
    
    // === Connection API ===
    pub fn create_connection(&self, conn: ConnectionV3) -> u64 { ... }
    pub fn get_connection(&self, id: u64) -> Option<ConnectionV3> { ... }
    // ... остальные методы
    
    // === Grid API ===
    pub fn grid_info(&self) -> GridInfo { ... }
    pub fn add_to_grid(&self, token_id: u32) -> Result<(), StorageError> { ... }
    pub fn find_neighbors(&self, token_id: u32, radius: f32) -> Vec<(u32, f32)> { ... }
    pub fn range_query(&self, center: [f32; 3], radius: f32) -> Vec<u32> { ... }
    
    // === CDNA API ===
    pub fn cdna(&self) -> CDNA { ... }
    pub fn update_cdna(&self, updates: CDNAUpdate) -> Result<(), StorageError> { ... }
    pub fn set_cdna_profile(&self, profile_id: u8) -> Result<(), StorageError> { ... }
}
```

**Задачи дня 1:**
- [ ] Создать `runtime_storage.rs`
- [ ] Реализовать Token API (6 методов)
- [ ] Реализовать Connection API (6 методов)
- [ ] Unit тесты для Token/Connection

#### День 2: Grid и CDNA в RuntimeStorage

**Задачи дня 2:**
- [ ] Реализовать Grid API (5 методов)
- [ ] Реализовать CDNA API (6 методов)
- [ ] Интегрировать Grid с Token storage (auto-index on create)
- [ ] Unit тесты для Grid/CDNA
- [ ] Добавить `mod runtime_storage;` в `lib.rs`

**Проверка:**
```bash
cargo build --release
cargo test runtime_storage
```

---

### Фаза 2: Обновление PyRuntime (1.5 дня)

#### Новая структура PyRuntime

**Файл:** `src/core_rust/src/python/runtime.rs`

```rust
#[pyclass]
pub struct PyRuntime {
    // Runtime storage (динамические данные)
    storage: Arc<RuntimeStorage>,
    
    // Bootstrap library (статические embeddings)
    bootstrap: Option<BootstrapLibrary>,
    
    // State
    initialized: bool,
}

#[pymethods]
impl PyRuntime {
    #[new]
    pub fn new(config: &PyDict) -> PyResult<Self> {
        let storage = Arc::new(RuntimeStorage::new());
        Ok(Self {
            storage,
            bootstrap: None,
            initialized: false,
        })
    }
    
    // === Token API (делегирует в storage) ===
    pub fn create_token(&self, ...) -> PyResult<PyDict> {
        let token = Token::new(...);
        let id = self.storage.create_token(token);
        // return as PyDict
    }
    
    pub fn get_token(&self, id: u32) -> PyResult<Option<PyDict>> {
        match self.storage.get_token(id) {
            Some(token) => Ok(Some(token_to_dict(token))),
            None => Ok(None),
        }
    }
    
    // ... остальные Token методы
    
    // === Grid API (делегирует в storage) ===
    pub fn get_grid_info(&self) -> PyResult<PyDict> {
        let info = self.storage.grid_info();
        Ok(grid_info_to_dict(info))
    }
    
    // ... остальные Grid методы
    
    // === CDNA API (делегирует в storage) ===
    pub fn get_cdna_config(&self) -> PyResult<PyDict> {
        let cdna = self.storage.cdna();
        Ok(cdna_to_dict(cdna))
    }
    
    // ... остальные CDNA методы
    
    // === Bootstrap API (semantic layer) ===
    pub fn bootstrap(&mut self, path: &str, ...) -> PyResult<()> {
        self.bootstrap = Some(BootstrapLibrary::load(path, ...)?);
        self.initialized = true;
        Ok(())
    }
    
    pub fn semantic_search(&self, query: &str, limit: usize) -> PyResult<Vec<PyDict>> {
        let bootstrap = self.bootstrap.as_ref()
            .ok_or_else(|| PyErr::new::<PyRuntimeError, _>("Not initialized"))?;
        let results = bootstrap.semantic_search(query, limit, None)?;
        Ok(results_to_dicts(results))
    }
    
    // === Query API (combines both layers) ===
    pub fn query(&self, text: &str, top_k: Option<usize>) -> PyResult<PyDict> {
        // 1. Semantic search in bootstrap
        // 2. Lookup tokens in storage
        // 3. Combine results
    }
}
```

**Задачи:**
- [ ] Обновить структуру PyRuntime
- [ ] Реализовать Token методы (7 шт)
- [ ] Реализовать Grid методы (6 шт)
- [ ] Реализовать CDNA методы (8 шт)
- [ ] Тестирование компиляции

**Проверка:**
```bash
cargo build --release --features python-bindings
maturin develop --release
```

---

### Фаза 3: Python Integration (1 день)

#### RuntimeStorage classes

**Файл:** `src/api/storage/runtime.py`

```python
from neurograph import _core

class RuntimeTokenStorage(TokenStorageInterface):
    """Token storage backed by Rust RuntimeStorage"""
    
    def __init__(self, runtime: _core.PyRuntime):
        self._runtime = runtime
    
    def create(self, data: TokenCreate) -> Token:
        result = self._runtime.create_token(
            entity_type=data.entity_type,
            domain=data.domain,
            weight=data.weight,
            coordinates=data.coordinates,
        )
        return Token(**result)
    
    def get(self, token_id: int) -> Optional[Token]:
        result = self._runtime.get_token(token_id)
        return Token(**result) if result else None
    
    def list(self, limit: int = 100, offset: int = 0) -> List[Token]:
        results = self._runtime.list_tokens(limit, offset)
        return [Token(**r) for r in results]
    
    # ... остальные методы


class RuntimeGridStorage(GridStorageInterface):
    """Grid operations backed by Rust RuntimeStorage"""
    
    def __init__(self, runtime: _core.PyRuntime):
        self._runtime = runtime
    
    def get_info(self) -> GridInfo:
        result = self._runtime.get_grid_info()
        return GridInfo(**result)
    
    def find_neighbors(self, token_id: int, radius: float) -> List[Neighbor]:
        results = self._runtime.find_neighbors(token_id, radius)
        return [Neighbor(**r) for r in results]
    
    # ... остальные методы


class RuntimeCDNAStorage(CDNAStorageInterface):
    """CDNA config backed by Rust RuntimeStorage"""
    
    def __init__(self, runtime: _core.PyRuntime):
        self._runtime = runtime
    
    def get_config(self) -> CDNAConfig:
        result = self._runtime.get_cdna_config()
        return CDNAConfig(**result)
    
    # ... остальные методы
```

**Задачи:**
- [ ] Реализовать RuntimeTokenStorage
- [ ] Реализовать RuntimeGridStorage
- [ ] Реализовать RuntimeCDNAStorage
- [ ] Обновить dependencies.py для переключения storage
- [ ] Integration тесты

---

### Фаза 4: Тестирование и документация (0.5 дня)

**Задачи:**
- [ ] E2E тесты: API → Python → Rust → Storage
- [ ] Performance тесты (latency < 50ms)
- [ ] CHANGELOG_v0.50.0.md
- [ ] Обновить README
- [ ] Git commit

**Тест план:**
```python
def test_full_flow():
    # 1. Create token via API
    response = client.post("/api/v1/tokens", json={...})
    token_id = response.json()["data"]["id"]
    
    # 2. Verify in storage
    response = client.get(f"/api/v1/tokens/{token_id}")
    assert response.status_code == 200
    
    # 3. Grid query
    response = client.get(f"/api/v1/grid/neighbors/{token_id}")
    assert response.status_code == 200
    
    # 4. CDNA check
    response = client.get("/api/v1/cdna/config")
    assert response.status_code == 200
```

---

## 📊 Timeline

| Фаза | Задача | Дни | Статус |
|------|--------|-----|--------|
| 0 | Откат + подготовка | 0.5 | ⏳ |
| 1 | RuntimeStorage в Rust | 2 | ⏳ |
| 2 | Обновление PyRuntime | 1.5 | ⏳ |
| 3 | Python Integration | 1 | ⏳ |
| 4 | Тесты + документация | 0.5 | ⏳ |
| **ИТОГО** | **v0.50.0 завершён** | **5.5 дней** | |

---

## 📁 Файлы для создания/изменения

### Новые файлы:
```
src/core_rust/src/runtime_storage.rs    # НОВЫЙ — главный storage
src/core_rust/src/storage_error.rs      # НОВЫЙ — error types
```

### Изменяемые файлы:
```
src/core_rust/src/lib.rs                # Добавить mod runtime_storage
src/core_rust/src/python/runtime.rs     # Переписать PyRuntime
src/api/storage/runtime.py              # Реализовать Runtime storage classes
src/api/dependencies.py                 # Обновить storage providers
```

### НЕ трогать:
```
src/core_rust/src/graph.rs              # Graph остаётся чистым (topology only)
src/core_rust/src/bootstrap.rs          # Bootstrap остаётся для semantic layer
src/core_rust/src/grid.rs               # Grid используется, но не меняется
src/core_rust/src/cdna.rs               # CDNA struct не меняется
```

---

## ⚠️ Критические правила

### 1. Единый источник правды

```
RuntimeStorage = единственное место для runtime данных
BootstrapLibrary = единственное место для semantic данных
```

### 2. Чёткое разделение API

```rust
// Runtime операции → self.storage
self.storage.create_token(...)
self.storage.find_neighbors(...)
self.storage.cdna()

// Semantic операции → self.bootstrap
self.bootstrap.semantic_search(...)
self.bootstrap.load_embeddings(...)
```

### 3. Никаких "по ходу" решений

Перед написанием кода — проверить:
- [ ] Где хранятся данные? (storage или bootstrap)
- [ ] Какой API использовать?
- [ ] Есть ли этот метод в RuntimeStorage?

---

## 🎯 Success Criteria для v0.50.0

- [ ] `cargo build --release --features python-bindings` — OK
- [ ] `cargo test` — все тесты проходят
- [ ] `maturin develop --release` — OK
- [ ] Python видит 21+ методов в PyRuntime
- [ ] REST API работает с Rust storage
- [ ] Latency < 50ms (p95)
- [ ] E2E тесты проходят

---

## 🔮 После v0.50.0

Когда v0.50.0 стабилен, продолжаем по MASTER_PLAN:

- **v0.51.0** — Auth + Enhanced endpoints
- **v0.52.0** — WebSocket
- **Phase 2** — Python Library packaging
- **Phase 3** — Web Dashboard
- **Phase 4** — Jupyter Integration

---

**Конец плана восстановления.**

*Главное изменение: RuntimeStorage как единый источник правды для runtime данных.*
