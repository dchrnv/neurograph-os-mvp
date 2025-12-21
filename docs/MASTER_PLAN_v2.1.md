# NeuroGraph OS - Мастер-план развития

**Версия:** 2.1
**Дата:** 2025-12-20
**Статус:** Active - Gateway & Signal Processing Focus
**Предыдущая версия:** [MASTER_PLAN_v2.0.md](archive/MASTER_PLAN_v2.0.md)

**Базовые документы:**
- [Gateway_v2_0.md](docs/specs/Gateway_v2_0.md) - Signal Gateway спецификация
- [SignalSystem_v1_1.md](docs/specs/SignalSystem_v1_1.md) - SignalSystem с PyO3 bindings
- [BENCHMARK_RESULTS_v0.52.0.md](docs/BENCHMARK_RESULTS_v0.52.0.md) - производительность системы

---

## 🎯 Общая стратегия

Построить полноценную систему NeuroGraph OS как **когнитивный организм** с сенсорными входами:

```
External World (Sensors)
        ↓
Gateway v2.0 (Python) - Sensory Interface
        ↓ PyO3 (zero-copy, <1ms)
SignalSystem v1.1 (Rust) - Event Processing
        ↓
Core (Rust): Grid + Guardian + Spreading Activation
        ↓
Action Controllers (Response Generation)
        ↓
External World (Actions)
```

**Текущий статус:** v0.53.0 завершён (SignalSystem v1.1 + PyO3), начинаем Gateway v2.0

---

## 📊 Текущее состояние (2025-12-20)

### ✅ Что работает (v0.52.0 - PRODUCTION READY)

#### 1. Rust Core (neurograph-core v0.47.0)
- **Token V2.0** (64 bytes, 8D coordinates)
- **Connection V3.0** (edges with weights)
- **Grid V2.0** (spatial indexing, 8D)
- **Graph** (topology, spreading activation)
- **RuntimeStorage** (persistence, 10M tokens stable)
- **Guardian + CDNA V2.1** (validation, quarantine)
- **Prometheus metrics** (observability)
- **Bootstrap** (semantic embeddings loader)

**Производительность (v0.52.0):**
- Token creation: 13K-21K tokens/sec
- Token read: 600K-940K ops/sec
- Grid queries: 157K queries/sec (small results)
- Memory: ~100MB per 1M tokens (linear growth)
- REST API: ~150 req/sec stable, <10ms latency

#### 2. REST API v0.52.0 (Production-Ready Stack)
**Роутеры (30+ endpoints):**
- `/api/v1/tokens` - Token CRUD + batch operations
- `/api/v1/grid` - Spatial queries (neighbors, range, density)
- `/api/v1/cdna` - CDNA configuration management
- `/api/v1/system` - Health, status, metrics

**Observability:**
- Prometheus metrics export (`/metrics`)
- Grafana dashboards ready
- Structured logging
- Health checks

**Storage:**
- RuntimeStorage (Rust Core via PyO3)
- Persistence через RuntimeStorage
- 10M tokens tested successfully

#### 3. PyRuntime (Full CRUD через FFI)
**Token methods (7):**
- `create_token()`, `get_token()`, `list_tokens()`
- `update_token()`, `delete_token()`
- `count_tokens()`, `clear_tokens()`

**Grid methods (7):**
- `add_token_to_grid()`, `remove_token_from_grid()`
- `find_neighbors()`, `range_query()`
- `calculate_field_influence()`, `calculate_density()`
- `get_grid_info()`

**CDNA methods (8):**
- `get_cdna_config()`, `update_cdna_scales()`
- `get_cdna_profile()`, `set_cdna_profile()`
- `get_cdna_flags()`, `set_cdna_flags()`
- `validate_cdna_scales()`, `reset_cdna()`

**System methods:**
- `bootstrap(path)` - загрузка embeddings
- `query(text)` - семантический поиск
- `export_metrics()` - Prometheus метрики

### ❌ Чего НЕТ (новые приоритеты)

1. **Signal Gateway v2.0** - сенсорный интерфейс для внешних входов
2. **SignalSystem v1.1** - event processing с подписками
3. **Sensor Registry** - динамическая регистрация сенсоров
4. **Subscription Filters** - фильтрация событий для подписчиков
5. **Action Controllers** - формирование ответов на активации

---

## 🗺️ Новый Roadmap - Gateway & Signal Processing

---

## ТРЕК A: Signal Processing Foundation (v0.53 → v0.55)

**Цель:** Создать сенсорный интерфейс и систему обработки сигналов

**Приоритет:** 🔴 КРИТИЧЕСКИЙ

**Обоснование:**
- REST API готов, но это passive интерфейс
- Нужен active интерфейс для Telegram, аудио, камеры
- Философия "организма" требует сенсорных входов

---

### 🎯 v0.53.0 - SignalSystem v1.1 (Rust Core + PyO3)

**Срок:** 5-7 дней
**Статус:** ✅ ЗАВЕРШЕНО (2025-12-21)

**Цель:** Реализовать event processing с подписками в Rust Core

#### Phase 1: Core Structures (2 дня)

**Задачи:**
- [ ] **1.1** SignalEvent struct (256 bytes, repr(C)):
  ```rust
  pub struct SignalEvent {
      // Компактное представление для FFI
      event_id_high: u64,
      event_id_low: u64,
      event_type_id: u32,
      vector: [f32; 8],
      magnitude: i16,
      valence: i8,
      // ... (см. SignalSystem_v1_1.md)
  }
  ```

- [ ] **1.2** EventTypeRegistry:
  ```rust
  pub struct EventTypeRegistry {
      type_to_id: HashMap<String, u32>,
      id_to_type: Vec<String>,
      wildcard_cache: HashMap<String, WildcardMatcher>,
  }
  ```

- [ ] **1.3** ProcessingResult:
  ```rust
  pub struct ProcessingResult {
      token_id: u32,
      neighbors: Vec<NeighborInfo>,
      is_novel: bool,
      triggered_actions: Vec<u32>,
      anomaly_score: f32,
      processing_time_us: u64,
  }
  ```

- [ ] **1.4** Unit tests для структур

**Файлы:**
- `src/core_rust/src/signal_system/mod.rs`
- `src/core_rust/src/signal_system/event.rs`
- `src/core_rust/src/signal_system/registry.rs`
- `src/core_rust/src/signal_system/result.rs`

#### Phase 2: Filter System (2 дня)

**Задачи:**
- [ ] **2.1** SubscriptionFilter с условиями:
  - EventType (wildcard support)
  - Numeric (priority, confidence, layers)
  - Bitmap (tags, flags)
  - Hash (sensor_id, sequence_id)

- [ ] **2.2** Компиляция фильтров из JSON
- [ ] **2.3** Предкомпиляция wildcard паттернов
- [ ] **2.4** Benchmarks для фильтров (target: <1μs per filter)

**Файлы:**
- `src/core_rust/src/signal_system/filter.rs`
- `src/core_rust/src/signal_system/filter_compiler.rs`

#### Phase 3: SignalSystem Core (2-3 дня)

**Задачи:**
- [ ] **3.1** SignalSystem struct:
  ```rust
  pub struct SignalSystem {
      event_registry: EventTypeRegistry,
      subscribers: RwLock<HashMap<SubscriberId, Subscriber>>,
      grid: Arc<RwLock<Grid>>,
      graph: Arc<RwLock<Graph>>,
      guardian: Arc<RwLock<Guardian>>,
  }
  ```

- [ ] **3.2** emit() pipeline:
  1. Guardian.validate_signal()
  2. Grid.add_token_from_signal()
  3. Grid.find_neighbors()
  4. Determine is_novel
  5. Graph.spreading_activation()
  6. Find triggered_actions
  7. notify_subscribers()
  8. Return ProcessingResult

- [ ] **3.3** subscribe()/unsubscribe()
- [ ] **3.4** notify_subscribers() с фильтрацией
- [ ] **3.5** Integration с Grid/Graph

**Файлы:**
- `src/core_rust/src/signal_system/system.rs`
- `src/core_rust/src/signal_system/subscriber.rs`

#### Phase 4: PyO3 Bindings (2 дня)

**Задачи:**
- [ ] **4.1** PySignalSystem class:
  ```python
  class SignalSystem:
      def emit(self, event: dict) -> dict
      def subscribe(self, name: str, filter: dict, callback: Callable) -> int
      def unsubscribe(self, subscriber_id: int) -> bool
      def poll_events(self, subscriber_id: int) -> list
      def get_stats(self) -> dict
  ```

- [ ] **4.2** Dict ↔ SignalEvent conversion
- [ ] **4.3** GIL release для emit()
- [ ] **4.4** Python callback mechanism
- [ ] **4.5** Zero-copy optimization

**Файлы:**
- `src/core_rust/src/python/signal_system.rs`

#### Phase 5: Testing & Integration (1 день)

**Задачи:**
- [ ] **5.1** Rust unit tests
- [ ] **5.2** Python integration tests
- [ ] **5.3** Performance benchmarks:
  - emit() < 100μs (p95)
  - Filter matching (1 sub) < 1μs
  - Filter matching (100 subs) < 50μs
  - Python→Rust conversion < 10μs

- [ ] **5.4** CHANGELOG_v0.53.0.md

**Deliverables:**
- ✅ SignalSystem v1.1 работает в Rust
- ✅ PyO3 bindings готовы
- ✅ Фильтры компилируются и работают
- ✅ Производительность соответствует KPI

**KPI:**
| Метрика | Target | Critical |
|---------|--------|----------|
| emit() полный цикл | < 100μs | < 500μs |
| Filter matching (1) | < 1μs | < 5μs |
| Filter matching (100) | < 50μs | < 200μs |
| Python→Rust conv | < 10μs | < 50μs |

---

### 🎯 v0.54.0 - Gateway v2.0 (Python Sensory Interface)

**Срок:** 4-5 дней
**Статус:** 🔧 IN PROGRESS (начато 2025-12-21)

**Цель:** Создать сенсорный интерфейс для всех внешних входов

#### Phase 1: Data Structures (1 день)

**Задачи:**
- [ ] **1.1** SignalEvent (Pydantic):
  ```python
  class SignalEvent(BaseModel):
      event_id: UUID
      event_type: str
      source: SignalSource
      semantic: SemanticCore  # 8D vector
      energy: EnergyProfile
      temporal: TemporalBinding
      raw: RawPayload
      result: Optional[ProcessingResult]
      routing: RoutingInfo
  ```

- [ ] **1.2** Nested structures:
  - SignalSource (domain, modality, sensor_id)
  - SemanticCore (vector, layer_decomposition)
  - EnergyProfile (magnitude, valence, arousal, urgency)
  - TemporalBinding (timestamp, neuro_tick)
  - RawPayload (data_type, data)
  - RoutingInfo (priority, tags, trace_id)

**Файлы:**
- `src/gateway/models/signal_event.py`
- `src/gateway/models/source.py`
- `src/gateway/models/energy.py`
- `src/gateway/models/temporal.py`

#### Phase 2: Sensor Registry (1-2 дня)

**Задачи:**
- [ ] **2.1** EncoderType enum:
  - TEXT_TRANSFORMER, TEXT_TFIDF
  - AUDIO_MEL, AUDIO_MFCC
  - IMAGE_CNN, IMAGE_CLIP
  - NUMERIC_DIRECT, PASSTHROUGH, CUSTOM

- [ ] **2.2** SensorRegistry:
  ```python
  class SensorRegistry:
      def register(self, config: SensorConfig) -> str
      def unregister(self, sensor_id: str) -> bool
      def get_config(self, sensor_id: str) -> SensorConfig
      def get_encoder(self, sensor_id: str) -> Callable
      def list_sensors(self, domain, modality) -> list
  ```

- [ ] **2.3** Built-in sensors:
  - `builtin_text_chat` - Text chat interface
  - `builtin_system_monitor` - System resources
  - `builtin_timer` - Periodic ticks

**Файлы:**
- `src/gateway/registry/sensor_registry.py`
- `src/gateway/registry/encoders.py`
- `src/gateway/registry/builtin.py`

#### Phase 3: Gateway Core (2 дня)

**Задачи:**
- [ ] **3.1** SignalGateway class:
  ```python
  class SignalGateway:
      def push_text(self, text: str, source: str, **kwargs) -> SignalEvent
      def push_audio(self, audio_vector: list, **kwargs) -> SignalEvent
      def push_vision(self, image_vector: list, **kwargs) -> SignalEvent
      def push_system(self, event_subtype: str, data: dict) -> SignalEvent
      def push(self, sensor_id: str, data: Any, event_type: str) -> SignalEvent
  ```

- [ ] **3.2** Normalization Pipeline:
  1. Get sensor config
  2. Normalize data
  3. Encode to 8D
  4. Compute energy profile
  5. Assemble SignalEvent
  6. Validate via Guardian
  7. Emit to SignalSystem (Rust)
  8. Get ProcessingResult
  9. Return full SignalEvent

- [ ] **3.3** Sensor management:
  - register_sensor()
  - unregister_sensor()
  - list_sensors()

**Файлы:**
- `src/gateway/gateway.py`
- `src/gateway/pipeline.py`

#### Phase 4: Encoders (MVP) (1 день)

**Задачи:**
- [ ] **4.1** PASSTHROUGH encoder (для готовых 8D)
- [ ] **4.2** TEXT_TFIDF encoder (базовый для текста)
- [ ] **4.3** NUMERIC_DIRECT encoder (для системных данных)
- [ ] **4.4** Sentiment analysis (простой для valence)

**Файлы:**
- `src/gateway/encoders/passthrough.py`
- `src/gateway/encoders/text.py`
- `src/gateway/encoders/numeric.py`

#### Phase 5: Testing & Integration (0.5 дня)

**Задачи:**
- [ ] **5.1** Unit tests для Gateway
- [ ] **5.2** Integration tests с SignalSystem
- [ ] **5.3** Performance tests (target: <1ms для push_text)
- [ ] **5.4** CHANGELOG_v0.54.0.md

**Deliverables:**
- ✅ Gateway v2.0 работает
- ✅ Sensor Registry функционален
- ✅ Базовые энкодеры реализованы
- ✅ Integration с SignalSystem v1.1

**KPI:**
| Метрика | Target | Critical |
|---------|--------|----------|
| push_text() latency | < 1ms | < 5ms |
| Encoding (8D) | < 0.5ms | < 2ms |
| Throughput | > 1000 sig/s | > 100 sig/s |

---

### 🎯 v0.55.0 - Subscription Filters & First Sensors

**Срок:** 2-3 дня
**Статус:** ⏳ FUTURE

**Цель:** Добавить систему фильтров и первые реальные сенсоры

#### Phase 1: Subscription Filters (Python) (1 день)

**Задачи:**
- [ ] **1.1** SubscriptionFilter class:
  ```python
  class SubscriptionFilter:
      def __init__(self, filter_dict: dict)
      def matches(self, event: SignalEvent) -> bool
  ```

- [ ] **1.2** Operators support:
  - $eq, $ne, $gt, $gte, $lt, $lte
  - $in, $nin, $contains
  - $regex, $wildcard
  - $and, $or, $not

- [ ] **1.3** Filter examples:
  - Telegram bot filter
  - Dashboard priority filter
  - ActionSelector filter
  - Anomaly detector filter

**Файлы:**
- `src/gateway/filters/subscription_filter.py`
- `src/gateway/filters/examples.py`

#### Phase 2: Input Adapters (1-2 дня)

**Задачи:**
- [ ] **2.1** TextAdapter для Telegram:
  ```python
  async def handle_telegram_message(update, context):
      event = gateway.push_text(
          text=update.message.text,
          source=f"telegram_{user_id}",
          tags=["telegram", "user_message"]
      )
  ```

- [ ] **2.2** SystemAdapter для мониторинга:
  ```python
  def monitor_resources():
      gateway.push_system(
          "resource.memory_low",
          {"used_mb": 450, "total_mb": 512}
      )
  ```

- [ ] **2.3** TimerAdapter для периодических событий

**Файлы:**
- `src/gateway/adapters/text.py`
- `src/gateway/adapters/system.py`
- `src/gateway/adapters/timer.py`

#### Phase 3: First Integration (Telegram Bot) (0.5 дня)

**Задачи:**
- [ ] **3.1** Telegram bot integration
- [ ] **3.2** Subscribe на action_trigger события
- [ ] **3.3** Response generation (simple)

**Deliverables:**
- ✅ Subscription Filters работают
- ✅ Telegram bot подключен к Gateway
- ✅ First end-to-end flow: Text → Gateway → SignalSystem → Action

---

## ТРЕК B: Action Controllers (v0.56+)

**Приоритет:** 🟡 ВЫСОКИЙ (после Gateway)

**Цель:** Формирование ответов на активации

### v0.56.0 - ActionController Foundation

**Задачи:**
- [ ] ActionSelector (выбор действия по активациям)
- [ ] Hot Path / Cold Path routing
- [ ] Response generation framework
- [ ] Telegram response integration

---

## ТРЕК C: Enhanced Sensors (v0.57+)

**Приоритет:** 🟢 СРЕДНИЙ

**Цель:** Расширение сенсорных модальностей

### v0.57.0 - Audio & Vision Inputs

**Задачи:**
- [ ] Audio input adapter
- [ ] Vision input adapter
- [ ] AUDIO_MEL encoder
- [ ] IMAGE_CNN encoder
- [ ] Multi-modal fusion

---

## ТРЕК D: Web Dashboard Evolution (v0.58+)

**Приоритет:** 🟢 НИЗКИЙ

**Цель:** Визуализация сигналов и активаций

### v0.58.0 - Signal Visualization

**Задачи:**
- [ ] Real-time signal stream display
- [ ] Activation visualization
- [ ] Sensor registry management UI
- [ ] WebSocket integration для live updates

---

## 📋 Timeline (обновлённый)

| Версия | Трек | Задача | Срок | Статус |
|--------|------|--------|------|--------|
| **v0.52.0** | A | Observability & Monitoring | Done | ✅ Завершено |
| **v0.53.0** | A | SignalSystem v1.1 (Rust) | 5-7 дней | ✅ Завершено |
| **v0.54.0** | A | Gateway v2.0 (Python) | 4-5 дней | 🔧 В процессе |
| **v0.55.0** | A | Filters & First Sensors | 2-3 дня | ⏳ После Gateway |
| **v0.56.0** | B | ActionController | 3-4 дня | ⏳ Future |
| **v0.57.0** | C | Audio & Vision | 5-7 дней | ⏳ Future |
| **v0.58.0** | D | Web Dashboard v2 | 7-10 дней | ⏳ Future |

**TOTAL до полной системы:** ~1.5-2 месяца

---

## 🎯 Immediate Next Steps (сейчас - 2025-12-20)

### Сегодня (2025-12-21):
1. ✅ v0.53.0 завершён (SignalSystem v1.1 + PyO3 bindings)
2. ✅ Актуализировать MASTER_PLAN v2.1
3. 🔧 Начать v0.54.0 Phase 1: Data Structures (SignalEvent Pydantic models)
4. 🔧 Создать директорию `src/gateway/`

### Эта неделя:
- Завершить v0.54.0 Phase 1-2 (Data Structures + SensorRegistry)
- Начать v0.54.0 Phase 3 (SignalGateway Core)
- Базовые энкодеры (PASSTHROUGH, TEXT_TFIDF)

### Следующая неделя:
- Завершить v0.54.0 (Testing + Integration)
- Начать v0.55.0 (Subscription Filters + First Sensors)
- Telegram bot integration

---

## 📝 Архитектурные решения (ADR)

### ADR-001: Gateway as Sensory Interface
**Дата:** 2025-12-20
**Проблема:** REST API - это passive интерфейс, нужен active
**Решение:** Gateway v2.0 как сенсорный слой с динамической регистрацией сенсоров
**Статус:** ✅ Принято

### ADR-002: PyO3 вместо REST для внутренней коммуникации
**Дата:** 2025-12-20
**Проблема:** REST создаёт overhead ~5ms на сигнал
**Решение:** Gateway → SignalSystem через PyO3 FFI (<0.1ms)
**Статус:** ✅ Принято

### ADR-003: SignalEvent - компактная 256 byte структура
**Дата:** 2025-12-20
**Проблема:** Pydantic модели медленны для FFI
**Решение:** Компактная Rust структура repr(C) с хешами вместо строк
**Статус:** ✅ Принято

### ADR-004: Event таксономия - иерархическая
**Дата:** 2025-12-20
**Проблема:** Как организовать типы событий
**Решение:** `signal.input.external.text.chat` - dot-separated hierarchy
**Статус:** ✅ Принято

---

## ✅ Success Metrics

### v0.53.0 (SignalSystem):
- [ ] emit() < 100μs (p95)
- [ ] Filter matching (100 subs) < 50μs
- [ ] Python FFI overhead < 10μs
- [ ] All unit tests pass

### v0.54.0 (Gateway):
- [ ] push_text() < 1ms
- [ ] Throughput > 1000 signals/sec
- [ ] Sensor registry работает
- [ ] Integration tests pass

### v0.55.0 (Filters & Sensors):
- [ ] Telegram bot integration работает
- [ ] Subscription filters функциональны
- [ ] End-to-end flow: Text → Action

### Production (ultimate):
- [ ] 10K signals/sec sustained
- [ ] Latency p99 < 5ms
- [ ] Multi-modal inputs working
- [ ] Action generation stable

---

## 🚀 References

**Спецификации:**
- [Gateway_v2_0.md](docs/specs/Gateway_v2_0.md) - полная спецификация Gateway
- [SignalSystem_v1_1.md](docs/specs/SignalSystem_v1_1.md) - SignalSystem + PyO3

**Benchmarks:**
- [BENCHMARK_RESULTS_v0.52.0.md](docs/BENCHMARK_RESULTS_v0.52.0.md) - текущая производительность

**История:**
- [MASTER_PLAN_v2.0.md](archive/MASTER_PLAN_v2.0.md) - предыдущий план (REST API фокус)
- [CHANGELOG_v0.52.0.md](docs/changelogs/CHANGELOG_v0.52.0.md) - последний релиз

**Код:**
- `src/core_rust/` - Rust core
- `src/gateway/` - Gateway v2.0 (новое)
- `src/api/` - REST API v0.52.0 (работает)

---

**Философия:** NeuroGraph OS как когнитивный организм с сенсорными входами, обработкой сигналов и формированием действий. Gateway v2.0 - это наши глаза, уши и нервная система.

---

**Конец мастер-плана v2.1. Signal Processing Era begins! 🧠⚡**

---

*Создано: 2025-12-20*
*Автор: Claude Sonnet 4.5*
*Статус: Living Document - обновляется по мере прогресса*
