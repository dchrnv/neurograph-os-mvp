# Guardian Specification v1.0 — Official Documentation

**Статус:** Official Specification  
**Версия:** 1.0
**Дата:** 2025-10-21  
**Зависимости:** Token v2.0, Connection v1.0, Grid v2.0, Graph v2.0  
**Целевые языки:** Rust, C++, любой системный язык

---

## Оглавление

1. [Обзор и философия](#1-%D0%BE%D0%B1%D0%B7%D0%BE%D1%80-%D0%B8-%D1%84%D0%B8%D0%BB%D0%BE%D1%81%D0%BE%D1%84%D0%B8%D1%8F)
2. [Архитектура Guardian](#2-%D0%B0%D1%80%D1%85%D0%B8%D1%82%D0%B5%D0%BA%D1%82%D1%83%D1%80%D0%B0-guardian)
3. [CDNA: Конституция системы](#3-cdna-%D0%BA%D0%BE%D0%BD%D1%81%D1%82%D0%B8%D1%82%D1%83%D1%86%D0%B8%D1%8F-%D1%81%D0%B8%D1%81%D1%82%D0%B5%D0%BC%D1%8B)
4. [Валидация и инварианты](#4-%D0%B2%D0%B0%D0%BB%D0%B8%D0%B4%D0%B0%D1%86%D0%B8%D1%8F-%D0%B8-%D0%B8%D0%BD%D0%B2%D0%B0%D1%80%D0%B8%D0%B0%D0%BD%D1%82%D1%8B)
5. [Event System](#5-event-system)
6. [Оркестрация модулей](#6-%D0%BE%D1%80%D0%BA%D0%B5%D1%81%D1%82%D1%80%D0%B0%D1%86%D0%B8%D1%8F-%D0%BC%D0%BE%D0%B4%D1%83%D0%BB%D0%B5%D0%B9)
7. [Производительность](#7-%D0%BF%D1%80%D0%BE%D0%B8%D0%B7%D0%B2%D0%BE%D0%B4%D0%B8%D1%82%D0%B5%D0%BB%D1%8C%D0%BD%D0%BE%D1%81%D1%82%D1%8C)
8. [Lifecycle Management](#8-lifecycle-management)
9. [Интеграция с модулями](#9-%D0%B8%D0%BD%D1%82%D0%B5%D0%B3%D1%80%D0%B0%D1%86%D0%B8%D1%8F-%D1%81-%D0%BC%D0%BE%D0%B4%D1%83%D0%BB%D1%8F%D0%BC%D0%B8)
10. [Сериализация и персистентность](#10-%D1%81%D0%B5%D1%80%D0%B8%D0%B0%D0%BB%D0%B8%D0%B7%D0%B0%D1%86%D0%B8%D1%8F-%D0%B8-%D0%BF%D0%B5%D1%80%D1%81%D0%B8%D1%81%D1%82%D0%B5%D0%BD%D1%82%D0%BD%D0%BE%D1%81%D1%82%D1%8C)
11. [Безопасность и целостность](#11-%D0%B1%D0%B5%D0%B7%D0%BE%D0%BF%D0%B0%D1%81%D0%BD%D0%BE%D1%81%D1%82%D1%8C-%D0%B8-%D1%86%D0%B5%D0%BB%D0%BE%D1%81%D1%82%D0%BD%D0%BE%D1%81%D1%82%D1%8C)
12. [Конфигурация](#12-%D0%BA%D0%BE%D0%BD%D1%84%D0%B8%D0%B3%D1%83%D1%80%D0%B0%D1%86%D0%B8%D1%8F)

---

## 1. Обзор и философия

### 1.1 Определение

**Guardian (Хранитель)** — это центральный модуль управления и оркестрации в NeuroGraph OS, который обеспечивает соблюдение фундаментальных законов системы (CDNA), координирует взаимодействие модулей и гарантирует целостность системы.

Guardian — это не диктатор, а **конституционный суд**: он не управляет модулями напрямую, а обеспечивает, чтобы все операции соответствовали базовым принципам системы.

### 1.2 Природа Guardian

**Guardian — это активный агент с пассивными данными:**

```
Пассивная часть: CDNA (128 bytes) — неизменяемые константы
Активная часть: Механизмы валидации, оркестрации, событий
```

**Аналогия из биологии:**

```
CDNA = ДНК клетки (blueprint, инструкция)
Guardian = Иммунная система + Нервная система
    - Защищает от нарушений (иммунитет)
    - Координирует действия органов (нервы)
    - Обеспечивает гомеостаз (саморегуляция)
```

### 1.3 Философия дизайна

**Принцип неизменности истины:**

CDNA представляет собой **фундаментальную истину** системы — законы физики её когнитивной вселенной. Изменение CDNA — это не рутинная операция, а критическое событие, требующее:

1. **Предложение** — кто-то (обычно система анализа) предлагает изменение
2. **Валидация** — проверка консистентности и безопасности
3. **Карантин** — тестирование предложенных изменений
4. **Подтверждение** — явное или через успешную работу
5. **Применение** — атомарное обновление с версионированием

Аналогия: изменение конституции требует референдума, а не простого голосования.

**Принцип делегирования:**

Guardian **не делает работу модулей** — он их координирует:

```
Guardian НЕ:
    ❌ Не реализует BFS (это Graph)
    ❌ Не симулирует физику полей (это Grid)
    ❌ Не создаёт Token (это создают другие модули)

Guardian ДЕЛАЕТ:
    ✅ Валидирует, что создание Token соответствует CDNA
    ✅ Оповещает модули об изменениях
    ✅ Обеспечивает согласованность состояний
    ✅ Управляет жизненным циклом системы
```

**Принцип минимального вмешательства:**

Guardian не должен стать узким горлышком. Стратегии:

- Модули получают read-only snapshot CDNA при старте
- Валидация делегируется модулям для частых операций
- Guardian проверяет только критичные изменения
- Асинхронная архитектура для событий
- Batch валидация вместо поштучной

**Принцип прозрачности:**

Все действия Guardian логируются и отслеживаются:

- Каждое изменение CDNA версионируется
- Все валидации записываются
- События имеют трассировку
- Решения Guardian объяснимы

### 1.4 Роли Guardian

Guardian выполняет пять ключевых ролей:

**1. Хранитель Конституции (Constitution Keeper)**

- Хранит и защищает CDNA
- Управляет версиями CDNA
- Контролирует процесс изменений CDNA
- Обеспечивает атомарность обновлений

**2. Валидатор (Validator)**

- Проверяет операции на соответствие CDNA
- Обнаруживает нарушения инвариантов
- Реализует "иммунную систему" (отсев некорректных данных)
- Предотвращает деградацию системы

**3. Брокер Событий (Event Broker)**

- Управляет подписками модулей
- Публикует события об изменениях
- Создаёт контекстные срезы данных
- Обеспечивает слабую связанность модулей

**4. Оркестратор (Orchestrator)**

- Координирует взаимодействие модулей
- Управляет жизненным циклом системы
- Синхронизирует состояния
- Обеспечивает graceful degradation

**5. Оптимизатор (Optimizer)**

- Кэширует часто используемые параметры CDNA
- Создаёт look-up tables для предвычислений
- Профилирует обращения к CDNA
- Оптимизирует memory locality

### 1.5 Место Guardian в NeuroGraph OS

```
┌─────────────────────────────────────────────────────────┐
│                    NeuroGraph OS                        │
├─────────────────────────────────────────────────────────┤
│                                                         │
│                   ┌─────────────┐                       │
│                   │  Guardian   │                       │
│                   │  (центр)    │                       │
│                   └──────┬──────┘                       │
│                          │                              │
│          ┌───────────────┼───────────────┐              │
│          │               │               │              │
│     ┌────▼────┐     ┌────▼────┐     ┌───▼────┐         │
│     │  Grid   │     │  Graph  │     │ Conn.  │         │
│     │ (space) │     │ (topo)  │     │(relate)│         │
│     └────┬────┘     └────┬────┘     └───┬────┘         │
│          │               │               │              │
│          └───────────────┼───────────────┘              │
│                          │                              │
│                     ┌────▼────┐                         │
│                     │  Token  │                         │
│                     │  (data) │                         │
│                     └─────────┘                         │
│                                                         │
└─────────────────────────────────────────────────────────┘

Guardian координирует, но не владеет модулями
Модули автономны, Guardian — гарант целостности
```

---

## 2. Архитектура Guardian

### 2.1 Компоненты системы

Guardian состоит из пяти основных компонентов:

```
┌──────────────────────────────────────────────────────────┐
│                      GUARDIAN CORE                       │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  ┌────────────────┐  ┌────────────────┐  ┌───────────┐  │
│  │ Constitution   │  │   Validator    │  │   Event   │  │
│  │    Keeper      │  │                │  │  Broker   │  │
│  │                │  │ • Rule check   │  │           │  │
│  │ • CDNA store   │  │ • Invariants   │  │ • Pub/Sub │  │
│  │ • Versioning   │  │ • Immunity     │  │ • Signals │  │
│  │ • Proposals    │  │ • Safety       │  │ • Slices  │  │
│  └────────────────┘  └────────────────┘  └───────────┘  │
│                                                          │
│  ┌────────────────┐  ┌────────────────┐                 │
│  │  Orchestrator  │  │   Optimizer    │                 │
│  │                │  │                │                 │
│  │ • Module sync  │  │ • Caching      │                 │
│  │ • Lifecycle    │  │ • LUTs         │                 │
│  │ • Coordination │  │ • Profiling    │                 │
│  │ • Degradation  │  │ • Memory opt   │                 │
│  └────────────────┘  └────────────────┘                 │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

### 2.2 Constitution Keeper (Хранитель Конституции)

**Назначение:** Управление CDNA и контроль изменений

**Компоненты:**

```
ConstitutionKeeper {
    cdna: CDNA,                              // Текущая активная CDNA
    cdna_history: Vec<CDNAVersion>,          // История версий
    pending_proposals: Vec<CDNAProposal>,    // Предложения изменений
    quarantine: Option<CDNAQuarantine>,      // Карантин для тестирования
    snapshot_cache: HashMap<String, CDNASnapshot>, // Кэшированные snapshots
}

CDNAVersion {
    version_id: uint32,
    cdna_data: [uint8; 128],
    timestamp: uint64,
    applied_by: String,                      // Кто применил
    reason: String,                          // Причина изменения
    previous_version: Option<uint32>,
    validation_hash: [uint8; 32],            // SHA-256 для integrity
}

CDNAProposal {
    proposal_id: String,
    proposed_cdna: [uint8; 128],
    proposer: String,                        // Модуль, предложивший изменение
    timestamp: uint64,
    justification: String,
    status: ProposalStatus,                  // PENDING | TESTING | APPROVED | REJECTED
    test_results: Option<TestResults>,
}

CDNAQuarantine {
    test_cdna: [uint8; 128],
    start_time: uint64,
    duration: uint64,
    success_metrics: QuarantineMetrics,
    passed: bool,
}
```

**Операции:**

- `GetCDNA() -> &CDNA` — получить текущую CDNA (O(1))
- `ProposeCDNAChange(proposal: CDNAProposal)` — предложить изменение
- `TestProposal(proposal_id: String)` — запустить тестирование в карантине
- `ApplyProposal(proposal_id: String)` — применить после подтверждения
- `RevertToVersion(version_id: uint32)` — откат к предыдущей версии
- `GetCDNAHistory() -> Vec<CDNAVersion>` — получить историю изменений

### 2.3 Validator (Валидатор)

**Назначение:** Проверка операций на соответствие CDNA и обнаружение аномалий

**Компоненты:**

```
Validator {
    cdna_ref: &CDNA,                         // Ссылка на текущую CDNA
    rule_cache: HashMap<RuleId, Rule>,       // Кэш правил для быстрой проверки
    violation_log: Vec<Violation>,           // Лог нарушений
    immunity_system: ImmunitySystem,         // "Иммунная система"
    stats: ValidationStats,
}

Rule {
    rule_id: String,
    rule_type: RuleType,                     // HARD | SOFT | WARNING
    check_fn: Box<dyn Fn(&Operation) -> ValidationResult>,
    source_section: CDNASection,             // Какая секция CDNA определяет правило
}

Violation {
    violation_id: String,
    timestamp: uint64,
    operation: Operation,
    rule_violated: String,
    severity: Severity,                      // CRITICAL | WARNING | INFO
    action_taken: Action,                    // REJECTED | ALLOWED_WITH_WARNING
}

ImmunitySystem {
    known_patterns: Vec<Pattern>,            // Известные безопасные паттерны
    threat_patterns: Vec<Pattern>,           // Известные опасные паттерны
    anomaly_detector: AnomalyDetector,
}
```

**Операции:**

- `ValidateOperation(operation: &Operation) -> ValidationResult`
- `CheckInvariant(invariant: Invariant) -> bool`
- `DetectAnomaly(data: &[u8]) -> Option<Anomaly>`
- `GetViolations(filter: ViolationFilter) -> Vec<Violation>`
- `UpdateRules(cdna: &CDNA)` — обновить правила при изменении CDNA

### 2.4 Event Broker (Брокер событий)

**Назначение:** Pub/Sub система для слабой связанности модулей

**Компоненты:**

```
EventBroker {
    subscriptions: HashMap<EventType, Vec<Subscriber>>,
    event_queue: LockFreeQueue<Event>,       // Асинхронная очередь
    event_history: RingBuffer<Event>,        // Кольцевой буфер последних событий
    stats: EventStats,
}

Subscriber {
    subscriber_id: String,
    module_name: String,
    event_types: Vec<EventType>,             // На какие события подписан
    callback: Box<dyn Fn(Event)>,            // Или channel для async
    filter: Option<EventFilter>,             // Опциональная фильтрация
}

Event {
    event_id: String,
    event_type: EventType,
    timestamp: uint64,
    source: String,                          // Откуда событие
    data: EventData,                         // Payload (контекстный срез)
    priority: Priority,                      // CRITICAL | HIGH | NORMAL | LOW
}

EventData {
    // Контекстный срез — только релевантные данные
    changed_fields: Vec<(String, Value)>,
    affected_modules: Vec<String>,
    metadata: HashMap<String, String>,
}
```

**Операции:**

- `Subscribe(subscriber: Subscriber)`
- `Unsubscribe(subscriber_id: String)`
- `Publish(event: Event)` — асинхронная публикация
- `PublishSync(event: Event)` — синхронная (блокирующая)
- `GetEventHistory(filter: EventFilter) -> Vec<Event>`

### 2.5 Orchestrator (Оркестратор)

**Назначение:** Координация модулей и управление жизненным циклом

**Компоненты:**

```
Orchestrator {
    modules: HashMap<String, ModuleHandle>,
    module_states: HashMap<String, ModuleState>,
    dependency_graph: DirectedGraph,         // Граф зависимостей модулей
    lifecycle_stage: LifecycleStage,
    health_monitor: HealthMonitor,
}

ModuleHandle {
    module_id: String,
    module_name: String,
    module_type: ModuleType,                 // CORE | EXTENSION | PLUGIN
    interface: Box<dyn ModuleInterface>,
    status: ModuleStatus,                    // INITIALIZING | RUNNING | PAUSED | FAILED
}

ModuleState {
    last_heartbeat: uint64,
    operations_count: uint64,
    errors_count: uint64,
    current_load: float,                     // 0.0 - 1.0
    health_score: float,                     // 0.0 - 1.0
}

HealthMonitor {
    health_checks: Vec<HealthCheck>,
    degradation_levels: Vec<DegradationLevel>,
    auto_recovery: bool,
}
```

**Операции:**

- `RegisterModule(module: ModuleHandle)`
- `InitializeModule(module_id: String) -> Result<()>`
- `StartModule(module_id: String) -> Result<()>`
- `PauseModule(module_id: String)`
- `ShutdownModule(module_id: String)`
- `CheckHealth() -> SystemHealth`
- `HandleDegradation(level: DegradationLevel)`

### 2.6 Optimizer (Оптимизатор)

**Назначение:** Повышение производительности через кэширование и предвычисления

**Компоненты:**

```
Optimizer {
    hot_cache: HotCache,                     // Кэш часто используемых параметров
    lut_manager: LUTManager,                 // Look-up tables
    profiler: AccessProfiler,                // Профилирование обращений
    memory_allocator: CustomAllocator,       // Оптимизация memory locality
}

HotCache {
    // "Горячие" параметры из CDNA
    cached_params: HashMap<String, CachedValue>,
    ttl: uint32,                             // Time to live
    hit_count: HashMap<String, uint64>,
    miss_count: HashMap<String, uint64>,
}

LUTManager {
    tables: HashMap<String, LookupTable>,
    // Предвычисленные таблицы для дорогих операций
}

AccessProfiler {
    access_log: RingBuffer<Access>,
    hot_params: Vec<String>,                 // Топ часто запрашиваемых
    cold_params: Vec<String>,                // Редко запрашиваемые
}
```

**Операции:**

- `CacheParameter(param_name: String, value: Value)`
- `GetCached(param_name: String) -> Option<Value>`
- `BuildLUT(function: String, range: Range) -> LookupTable`
- `ProfileAccess(access: Access)`
- `OptimizeMemoryLayout()`

---

## 3. CDNA: Конституция системы

### 3.1 Структура CDNA (128 bytes)

CDNA (Constitutional DNA) — это фундаментальные законы системы, определяющие физику когнитивной вселенной NeuroGraph OS.

**Общая структура:**

```
╔══════════════════════════════════════════════════════════╗
║               CDNA STRUCTURE (128 bytes)                 ║
╠══════════════════════════════════════════════════════════╣
║  Block 1: GRID PHYSICS CONSTANTS         (32 bytes)     ║
║  Block 2: GRAPH TOPOLOGY RULES           (32 bytes)     ║
║  Block 3: TOKEN BASE PROPERTIES          (32 bytes)     ║
║  Block 4: EVOLUTION CONSTRAINTS          (32 bytes)     ║
╚══════════════════════════════════════════════════════════╝

Offset: 0    32    64    96    128
```

### 3.2 Block 1: GRID PHYSICS CONSTANTS (32 bytes)

**Offset: 0-31**  
**Назначение:** Определяет "физику" 8-мерного пространственного индекса

```
Offset  Size  Type      Field                   Description
------  ----  --------  ----------------------  ---------------------------
0-15    16    u16[8]    dimension_semantic_ids  Семантические ID измерений
                                                L0_PHYSICAL (0x0001)
                                                L1_SENSORY (0x0002)
                                                L2_MOTOR (0x0003)
                                                L3_EMOTIONAL (0x0004)
                                                L4_COGNITIVE (0x0005)
                                                L5_SOCIAL (0x0006)
                                                L6_TEMPORAL (0x0007)
                                                L7_ABSTRACT (0x0008)

16-23   8     u8[8]     dimension_flags         Флаги свойств измерений
                                                Bit 0: CYCLIC (циклическое)
                                                Bit 1: BOUNDED (ограниченное)
                                                Bit 2: DISCRETE (дискретное)
                                                Bit 3: INVERTIBLE (обратимое)
                                                Bit 4-7: Reserved

24-31   8     f32[8]    dimension_scale         Масштабные коэффициенты
                                                Для нормализации расстояний
                                                в каждом измерении
------  ----  --------  ----------------------  ---------------------------
TOTAL:  32 bytes
```

**Семантика:**

- **dimension_semantic_ids:** Связывают каждое из 8 измерений с онтологией системы. Например, 0x0001 может означать "пространство физических объектов", 0x0005 — "пространство абстрактных концепций".
    
- **dimension_flags:** Определяют математические свойства измерений. Например, временная ось может быть CYCLIC (циклической) для моделирования суточных ритмов.
    
- **dimension_scale:** Коэффициенты для баланса влияния разных измерений. Если абстрактное измерение имеет scale=10.0, а физическое scale=1.0, то абстрактные различия будут весить больше при вычислении расстояний.
    

### 3.3 Block 2: GRAPH TOPOLOGY RULES (32 bytes)

**Offset: 32-63**  
**Назначение:** Фундаментальные правила строения и динамики графа

```
Offset  Size  Type      Field                   Description
------  ----  --------  ----------------------  ---------------------------
32-33   2     u16       allowed_connection_types Битовая маска типов связей
                                                Bit 0: ASSOCIATION (0x0001)
                                                Bit 1: HIERARCHY (0x0002)
                                                Bit 2: SEQUENCE (0x0004)
                                                Bit 3: CAUSALITY (0x0008)
                                                Bit 4: SIMILARITY (0x0010)
                                                Bit 5: OPPOSITION (0x0020)
                                                Bit 6: DEPENDENCY (0x0040)
                                                Bit 7: COMPOSITION (0x0080)
                                                Bit 8: REFERENCE (0x0100)
                                                Bit 9: MUTATION (0x0200)
                                                Bit 10: CROSSOVER (0x0400)
                                                Bit 11: INHERITANCE (0x0800)
                                                Bit 12: PROXIMITY (0x1000)
                                                Bit 13: CONTAINMENT (0x2000)
                                                Bit 14: CUSTOM_1 (0x4000)
                                                Bit 15: CUSTOM_2 (0x8000)

34-37   4     u32       max_node_degree         Максимальная степень узла
                                                (суммарная in + out)

38-41   4     f32       min_connection_weight   Минимальный вес связи (0.0-1.0)

42-45   4     f32       max_connection_weight   Максимальный вес связи (обычно 1.0)

46-49   4     f32       connection_distance_min Минимальное расстояние для связи

50-53   4     f32       connection_distance_max Максимальное расстояние для связи

54      1     u8        topology_flags          Флаги топологии
                                                Bit 0: ALLOW_SELF_LOOPS
                                                Bit 1: ALLOW_MULTI_EDGES
                                                Bit 2: DIRECTED
                                                Bit 3: WEIGHTED
                                                Bit 4-7: Reserved

55-63   9     bytes     reserved                Зарезервировано
------  ----  --------  ----------------------  ---------------------------
TOTAL:  32 bytes
```

**Семантика:**

- **allowed_connection_types:** Битовая маска определяет, какие типы связей разрешены в системе. Это позволяет создавать специализированные "породы" систем (например, только иерархические связи для таксономий).
    
- **max_node_degree:** Ограничивает количество связей узла. Предотвращает появление "сверххабов" и контролирует сложность графа.
    
- **connection_distance_min/max:** Ограничения на пространственное расстояние между связанными токенами. Например, можно запретить связи между слишком удалёнными концептами.
    
- **topology_flags:** Определяют базовые свойства графа (направленность, взвешенность, допустимость петель).
    

### 3.4 Block 3: TOKEN BASE PROPERTIES (32 bytes)

**Offset: 64-95**  
**Назначение:** Базовые, неизменяемые свойства атомарных единиц-токенов

```
Offset  Size  Type      Field                   Description
------  ----  --------  ----------------------  ---------------------------
64-67   4     f32       min_token_weight        Минимальный вес токена

68-71   4     f32       max_token_weight        Максимальный вес токена

72-75   4     f32       min_field_radius        Минимальный радиус поля

76-79   4     f32       max_field_radius        Максимальный радиус поля

80-83   4     f32       min_field_strength      Минимальная сила поля

84-87   4     f32       max_field_strength      Максимальная сила поля

88      1     u8        token_flags             Флаги токенов
                                                Bit 0: ALLOW_ZERO_WEIGHT
                                                Bit 1: REQUIRE_FSC
                                                Bit 2: ALLOW_MIGRATION
                                                Bit 3: ALLOW_SPLITTING
                                                Bit 4-7: Reserved

89-95   7     bytes     reserved                Зарезервировано
------  ----  --------  ----------------------  ---------------------------
TOTAL:  32 bytes
```

**Семантика:**

- **min/max_token_weight:** Определяют диапазон значимости токенов. Например, система может запретить токены с нулевым весом, если ALLOW_ZERO_WEIGHT=false.
    
- **min/max_field_radius:** Ограничения на радиус влияния поля токена. Контролирует дальность воздействия в пространстве.
    
- **min/max_field_strength:** Диапазон силы поля. Определяет, насколько сильно токен может притягивать/отталкивать другие токены.
    
- **token_flags:** Базовые разрешения для токенов (миграция, расщепление и т.д.).
    

### 3.5 Block 4: EVOLUTION CONSTRAINTS (32 bytes)

**Offset: 96-127**  
**Назначение:** Ограничения и константы для эволюционных/генетических алгоритмов

```
Offset  Size  Type      Field                   Description
------  ----  --------  ----------------------  ---------------------------
96-99   4     f32       min_mutation_rate       Минимальная частота мутаций

100-103 4     f32       max_mutation_rate       Максимальная частота мутаций

104-107 4     f32       min_crossover_rate      Минимальная частота кроссовера

108-111 4     f32       max_crossover_rate      Максимальная частота кроссовера

112-115 4     f32       selection_pressure_min  Минимальное давление отбора

116-119 4     f32       selection_pressure_max  Максимальное давление отбора

120     1     u8        evolution_flags         Флаги эволюции
                                                Bit 0: ALLOW_MUTATIONS
                                                Bit 1: ALLOW_CROSSOVER
                                                Bit 2: ALLOW_SELECTION
                                                Bit 3: ELITISM_ENABLED
                                                Bit 4-7: Reserved

121-127 7     bytes     reserved                Зарезервировано
------  ----  --------  ----------------------  ---------------------------
TOTAL:  32 bytes
```

**Семантика:**

- **min/max_mutation_rate:** Ограничивают, насколько агрессивно система может мутировать свою структуру. Предотвращают деградацию от слишком частых изменений.
    
- **min/max_crossover_rate:** Контролируют частоту рекомбинации подграфов.
    
- **selection_pressure_min/max:** Определяют, насколько строгим должен быть отбор слабых связей.
    
- **evolution_flags:** Глобальные разрешения для эволюционных операторов. Можно отключить мутации/кроссовер полностью, если система достигла стабильного состояния.
    

### 3.6 CDNA как целостность

**CDNA представляет собой когерентную систему правил:**

```
Grid определяет пространство
    ↓
Token существуют в этом пространстве
    ↓
Connection связывают Token согласно Graph rules
    ↓
Evolution модифицирует структуру в рамках constraints
    ↓
Всё это валидируется Guardian через CDNA
```

**Изменение одного параметра CDNA влияет на всю систему:**

```
Пример: Увеличение max_node_degree в GRAPH_TOPOLOGY_RULES
    
Прямые эффекты:
    • Graph может создавать больше связей на узел
    • Возрастает плотность графа
    
Косвенные эффекты:
    • Grid должен пересчитать поля (больше связей = сильнее поля)
    • Token может испытывать большее влияние от соседей
    • Evolution алгоритмы получают больше материала для мутаций
    • Производительность Graph может снизиться (больше рёбер)
    
Guardian отслеживает эти эффекты через валидацию и мониторинг
```

---

## 4. Валидация и инварианты

### 4.1 Типы валидации

Guardian реализует три уровня валидации:

**Level 1: CDNA Compliance (Соответствие CDNA)**

```
Проверка: Соответствует ли операция фундаментальным правилам?
Когда: Перед выполнением операции
Стоимость: O(1) - быстрая проверка по кэшированным правилам
Действие при нарушении: REJECT operation
```

**Level 2: System Invariants (Системные инварианты)**

```
Проверка: Сохраняются ли базовые свойства системы?
Когда: Периодически + после критичных операций
Стоимость: O(V + E) - полный проход по структурам
Действие при нарушении: LOG warning + attempt repair
```

**Level 3: Anomaly Detection (Обнаружение аномалий)**

```
Проверка: Не происходит ли что-то подозрительное?
Когда: Постоянно (через profiling и статистику)
Стоимость: O(1) - инкрементальная статистика
Действие при нарушении: ALERT + quarantine if critical
```

### 4.2 CDNA Compliance Rules

**Правила, извлекаемые из CDNA:**

**Grid Physics Rules:**

```
Rule: ValidDimensionUsage
    For each Token.coordinates[i]:
        assert 0 <= i < 8
        if dimension_flags[i].BOUNDED:
            assert coordinates[i] within valid range

Rule: ValidScale
    For distance calculations:
        assert dimension_scale[i] > 0.0
        apply scale when computing distances
```

**Graph Topology Rules:**

```
Rule: AllowedConnectionType
    For each Connection:
        connection_type_bit = 1 << connection.connection_type
        assert (allowed_connection_types & connection_type_bit) != 0

Rule: MaxDegree
    For each Node:
        degree = in_degree + out_degree
        assert degree <= max_node_degree

Rule: ConnectionWeightRange
    For each Connection:
        assert min_connection_weight <= connection.weight <= max_connection_weight

Rule: ConnectionDistanceRange
    For each Connection:
        distance = Grid.Distance(connection.from_id, connection.to_id)
        assert connection_distance_min <= distance <= connection_distance_max

Rule: SelfLoops
    For each Connection:
        if not topology_flags.ALLOW_SELF_LOOPS:
            assert connection.from_id != connection.to_id

Rule: MultiEdges
    For each pair (from_id, to_id):
        if not topology_flags.ALLOW_MULTI_EDGES:
            assert count(edges from from_id to to_id) <= 1
```

**Token Base Rules:**

```
Rule: TokenWeightRange
    For each Token:
        assert min_token_weight <= token.weight <= max_token_weight

Rule: FieldRadiusRange
    For each Token:
        assert min_field_radius <= token.field_radius <= max_field_radius

Rule: FieldStrengthRange
    For each Token:
        assert min_field_strength <= token.field_strength <= max_field_strength

Rule: ZeroWeight
    For each Token:
        if not token_flags.ALLOW_ZERO_WEIGHT:
            assert token.weight > 0.0

Rule: FSCRequired
    For each Token:
        if token_flags.REQUIRE_FSC:
            assert token.fsc_code is not empty
```

**Evolution Constraints Rules:**

```
Rule: MutationRateRange
    For evolution operations:
        assert min_mutation_rate <= current_mutation_rate <= max_mutation_rate

Rule: CrossoverRateRange
    For evolution operations:
        assert min_crossover_rate <= current_crossover_rate <= max_crossover_rate

Rule: SelectionPressureRange
    For selection operations:
        assert selection_pressure_min <= current_pressure <= selection_pressure_max

Rule: EvolutionEnabled
    For mutation:
        assert evolution_flags.ALLOW_MUTATIONS == true
    For crossover:
        assert evolution_flags.ALLOW_CROSSOVER == true
    For selection:
        assert evolution_flags.ALLOW_SELECTION == true
```

### 4.3 Validation Algorithm

**ValidateOperation(operation: &Operation) -> ValidationResult:**

```
Algorithm:
    1. Identify operation type:
        • ADD_TOKEN
        • REMOVE_TOKEN
        • ADD_CONNECTION
        • REMOVE_CONNECTION
        • UPDATE_CONNECTION
        • EVOLVE_GRAPH
        • MIGRATE_TOKEN
    
    2. Retrieve applicable rules from rule_cache:
        rules = GetRulesFor(operation.type)
    
    3. For each rule in rules:
        result = rule.check_fn(operation)
        
        if result == VIOLATION:
            violation = Violation {
                violation_id: generate_id(),
                timestamp: now(),
                operation: operation.clone(),
                rule_violated: rule.rule_id,
                severity: rule.severity,
                action_taken: REJECTED,
            }
            
            LogViolation(violation)
            
            if rule.rule_type == HARD:
                return ValidationResult::Rejected(violation)
            else if rule.rule_type == SOFT:
                // Soft rule - log warning but allow
                LogWarning(violation)
            else:
                // WARNING level - just log
                LogInfo(violation)
    
    4. return ValidationResult::Approved

Complexity: O(R) where R = number of applicable rules (typically < 10)
```

**Пример валидации:**

```
Operation: ADD_CONNECTION
    connection_id: "conn_42_100"
    from_id: 42
    to_id: 100
    connection_type: CAUSALITY (0x0008)
    weight: 0.85

Validation steps:
    1. Check AllowedConnectionType:
        allowed_connection_types = 0xFFFF (all types allowed)
        connection_type_bit = 1 << 3 = 0x0008
        0xFFFF & 0x0008 = 0x0008 ≠ 0  ✓ PASS
    
    2. Check MaxDegree:
        current_degree(42) = 15
        max_node_degree = 1000
        15 <= 1000  ✓ PASS
    
    3. Check ConnectionWeightRange:
        min_connection_weight = 0.01
        max_connection_weight = 1.0
        0.01 <= 0.85 <= 1.0  ✓ PASS
    
    4. Check ConnectionDistanceRange:
        distance = Grid.Distance(42, 100) = 5.2
        connection_distance_min = 0.0
        connection_distance_max = 1000.0
        0.0 <= 5.2 <= 1000.0  ✓ PASS
    
    5. Check SelfLoops:
        topology_flags.ALLOW_SELF_LOOPS = false
        42 != 100  ✓ PASS

Result: ValidationResult::Approved
```

### 4.4 System Invariants

**Инварианты, которые должны всегда соблюдаться:**

**Invariant 1: Node-Edge Consistency**

```
For all edges:
    connection = ConnectionStore.Get(edge_id)
    assert Grid.ContainsNode(connection.from_token_id)
    assert Grid.ContainsNode(connection.to_token_id)
    assert Graph.adjacency_out[from_id] contains edge_id
    assert Graph.adjacency_in[to_id] contains edge_id
```

**Invariant 2: Index Symmetry**

```
For all nodes:
    for edge_id in Graph.adjacency_out[node_id]:
        connection = ConnectionStore.Get(edge_id)
        assert edge_id in Graph.adjacency_in[connection.to_token_id]
```

**Invariant 3: Weight Conservation**

```
For all tokens:
    sum_of_outgoing_weights = Σ connection.weight for outgoing edges
    sum_of_incoming_weights = Σ connection.weight for incoming edges
    
    // Опционально: можно требовать баланс
    if enforce_balance:
        assert abs(sum_outgoing - sum_incoming) < threshold
```

**Invariant 4: Field Consistency**

```
For all tokens:
    if token.field_radius > 0:
        assert token.field_strength > 0
    
    For all connections where distance < token.field_radius:
        connection should exist and be affected by field
```

**Invariant 5: CDNA Compliance**

```
For all system state:
    assert all operations comply with current CDNA
    assert no values outside CDNA ranges
    assert no forbidden connection types exist
```

### 4.5 Periodic Validation

**CheckSystemInvariants() -> InvariantReport:**

```
Algorithm:
    report = InvariantReport::new()
    
    // Check Node-Edge Consistency
    violations = CheckNodeEdgeConsistency()
    report.add_section("Node-Edge", violations)
    
    // Check Index Symmetry
    violations = CheckIndexSymmetry()
    report.add_section("Index Symmetry", violations)
    
    // Check Weight Conservation
    violations = CheckWeightConservation()
    report.add_section("Weight Conservation", violations)
    
    // Check Field Consistency
    violations = CheckFieldConsistency()
    report.add_section("Field Consistency", violations)
    
    // Check CDNA Compliance
    violations = CheckCDNACompliance()
    report.add_section("CDNA Compliance", violations)
    
    // Compute overall health
    report.health_score = ComputeHealthScore(report)
    
    if report.has_critical_violations():
        report.status = SystemStatus::DEGRADED
        AttemptRepair(report)
    else:
        report.status = SystemStatus::HEALTHY
    
    return report

Рекомендуемая частота:
    - После каждых N операций (N = 10,000 - 100,000)
    - После критичных операций (CDNA change, mass mutation)
    - По расписанию (например, раз в минуту)
    - По запросу администратора
```

### 4.6 Immunity System (Иммунная система)

**Назначение:** Обнаружение и нейтрализация аномальных паттернов

**Концепция:**

Immunity System работает по принципу биологической иммунной системы:

- Знает "свои" паттерны (safe patterns)
- Обнаруживает "чужие" паттерны (threat patterns)
- Обучается на опыте (adaptive immunity)

**Компоненты:**

```
ImmunitySystem {
    known_safe_patterns: Vec<Pattern>,
    known_threat_patterns: Vec<Pattern>,
    anomaly_detector: AnomalyDetector,
    quarantine: QuarantineZone,
    memory_cells: Vec<MemoryCell>,          // "Иммунная память"
}

Pattern {
    pattern_id: String,
    pattern_type: PatternType,               // STRUCTURAL | BEHAVIORAL | STATISTICAL
    signature: Vec<u8>,                      // Отпечаток паттерна
    confidence: f32,                         // 0.0 - 1.0
    last_seen: uint64,
    occurrence_count: uint64,
}

AnomalyDetector {
    baseline_stats: Statistics,              // Baseline для нормального поведения
    current_stats: Statistics,
    deviation_threshold: f32,
    alert_threshold: f32,
}

QuarantineZone {
    quarantined_items: Vec<QuarantinedItem>,
    max_size: usize,
    ttl: uint64,
}

QuarantinedItem {
    item_id: String,
    item_type: ItemType,                     // TOKEN | CONNECTION | OPERATION
    reason: String,
    quarantined_at: uint64,
    release_condition: ReleaseCondition,
}
```

**Работа Immunity System:**

**1. Pattern Recognition:**

```
DetectPattern(data: &[u8]) -> Option<Pattern>:
    1. Compute signature of data
    2. Check against known_safe_patterns:
        if match found:
            return None  // Known safe, no action needed
    
    3. Check against known_threat_patterns:
        if match found:
            return Some(pattern)  // Threat detected
    
    4. Check with anomaly_detector:
        if is_anomalous(data):
            pattern = CreateNewThreatPattern(data)
            known_threat_patterns.push(pattern)
            return Some(pattern)
    
    5. return None  // Not recognized, assume safe (for now)
```

**2. Threat Response:**

```
HandleThreat(threat: Pattern, context: Context):
    severity = AssessSeverity(threat)
    
    match severity:
        CRITICAL:
            // Немедленная блокировка
            BlockOperation(context.operation)
            QuarantineRelatedItems(context)
            AlertAdministrator(threat)
        
        HIGH:
            // Блокировка + логирование
            BlockOperation(context.operation)
            LogThreat(threat, context)
        
        MEDIUM:
            // Разрешить, но мониторить
            AllowWithMonitoring(context.operation)
            LogThreat(threat, context)
        
        LOW:
            // Просто логировать
            LogInfo(threat, context)
```

**3. Adaptive Learning:**

```
LearnFromExperience(outcome: Outcome, pattern: Pattern):
    if outcome == SUCCESS:
        // Операция прошла успешно, паттерн безопасен
        if pattern in known_threat_patterns:
            // Ложная тревога, переместить в safe
            RemoveFromThreats(pattern)
            AddToSafe(pattern)
        else:
            // Укрепить confidence в safe patterns
            UpdateConfidence(pattern, +0.1)
    
    else if outcome == FAILURE:
        // Операция привела к проблеме
        if pattern in known_safe_patterns:
            // Паттерн больше не безопасен
            RemoveFromSafe(pattern)
            AddToThreats(pattern)
        else:
            // Укрепить confidence в threat patterns
            UpdateConfidence(pattern, +0.1)
    
    // Сохранить в иммунную память
    memory_cells.push(MemoryCell {
        pattern: pattern,
        outcome: outcome,
        timestamp: now(),
    })
```

**Примеры угроз, обнаруживаемых Immunity System:**

**Structural Threats:**

```
• Sudden massive increase in node degree (potential attack)
• Creation of disconnected components (system fragmentation)
• Появление циклов в DAG (если граф должен быть ациклическим)
• Нарушение иерархий (child становится parent своего parent)
```

**Behavioral Threats:**

```
• Abnormally high mutation rate (система деградирует)
• Rapid weight decay (система "забывает" всё)
• Oscillating parameters (нестабильность)
• Excessive connection creation/deletion (thrashing)
```

**Statistical Threats:**

```
• Distribution shift (граф перестал быть scale-free)
• Sudden change in clustering coefficient
• Anomalous centrality values (один узел доминирует)
• Unexpected degree distribution
```

---

## 5. Event System

### 5.1 Архитектура Pub/Sub

Event System обеспечивает слабую связанность модулей через асинхронную публикацию событий.

**Принцип работы:**

```
Модуль A                Guardian (Event Broker)            Модуль B
   │                              │                            │
   │  1. Subscribe(EventType)     │                            │
   ├─────────────────────────────►│                            │
   │                              │   2. Subscribe(EventType)  │
   │                              │◄───────────────────────────┤
   │                              │                            │
   │  3. Publish(Event)           │                            │
   ├─────────────────────────────►│                            │
   │                              │  4. Notify(Event)          │
   │                              ├───────────────────────────►│
   │                              │                            │
   │                              │  5. Process(Event)         │
   │                              │                            ├──►
```

**Преимущества:**

- Модули не знают друг о друге напрямую
- Легко добавлять новые модули без изменения существующих
- Асинхронность — публикация не блокирует издателя
- Фильтрация — подписчики получают только релевантные события

### 5.2 Типы событий

**EventType enum:**

```rust
enum EventType {
    // CDNA Events
    CDNA_CHANGED,                    // CDNA была изменена
    CDNA_PROPOSAL_SUBMITTED,         // Предложено изменение CDNA
    CDNA_PROPOSAL_APPROVED,          // Предложение одобрено
    CDNA_PROPOSAL_REJECTED,          // Предложение отклонено
    
    // Module Events
    MODULE_INITIALIZED,              // Модуль инициализирован
    MODULE_STARTED,                  // Модуль запущен
    MODULE_PAUSED,                   // Модуль приостановлен
    MODULE_FAILED,                   // Модуль упал
    MODULE_RECOVERED,                // Модуль восстановился
    
    // Topology Events
    NODE_ADDED,                      // Узел добавлен в Graph
    NODE_REMOVED,                    // Узел удалён из Graph
    EDGE_ADDED,                      // Ребро добавлено
    EDGE_REMOVED,                    // Ребро удалено
    EDGE_WEIGHT_CHANGED,             // Вес ребра изменился
    
    // Evolution Events
    MUTATION_APPLIED,                // Применена мутация
    CROSSOVER_APPLIED,               // Применён кроссовер
    SELECTION_APPLIED,               // Применён отбор
    GENERATION_ADVANCED,             // Новое поколение
    
    // Validation Events
    VALIDATION_FAILED,               // Валидация операции не прошла
    INVARIANT_VIOLATED,              // Нарушен инвариант
    ANOMALY_DETECTED,                // Обнаружена аномалия
    THREAT_QUARANTINED,              // Угроза помещена в карантин
    
    // System Events
    SYSTEM_HEALTH_CHANGED,           // Изменился health score
    DEGRADATION_DETECTED,            // Обнаружена деградация
    RECOVERY_INITIATED,              // Начато восстановление
    SHUTDOWN_REQUESTED,              // Запрошено завершение работы
    
    // Performance Events
    PERFORMANCE_WARNING,             // Проблемы с производительностью
    CACHE_INVALIDATED,               // Кэш инвалидирован
    MEMORY_THRESHOLD_EXCEEDED,       // Превышен порог памяти
}
```

### 5.3 Структура Event

```rust
struct Event {
    event_id: String,                        // Уникальный ID события
    event_type: EventType,
    timestamp: uint64,                       // Unix timestamp (microseconds)
    source: String,                          // Модуль-источник
    priority: Priority,                      // CRITICAL | HIGH | NORMAL | LOW
    data: EventData,                         // Payload
    metadata: HashMap<String, String>,       // Дополнительные метаданные
}

struct EventData {
    // Контекстный срез — только релевантные данные
    changed_fields: Vec<(String, Value)>,    // Что изменилось
    affected_modules: Vec<String>,           // Какие модули затронуты
    old_values: Option<HashMap<String, Value>>, // Старые значения (для rollback)
    new_values: Option<HashMap<String, Value>>, // Новые значения
    additional_context: HashMap<String, Value>, // Доп. контекст
}

enum Priority {
    CRITICAL = 0,  // Требует немедленной обработки
    HIGH = 1,      // Высокий приоритет
    NORMAL = 2,    // Обычный приоритет
    LOW = 3,       // Низкий приоритет
}
```

### 5.4 Subscription Mechanism

**Subscribe:**

```rust
Subscribe(subscriber: Subscriber):
    1. Validate subscriber:
        assert subscriber.module_name is valid
        assert subscriber.callback is valid
    
    2. For each event_type in subscriber.event_types:
        if event_type not in subscriptions:
            subscriptions[event_type] = Vec::new()
        
        subscriptions[event_type].push(subscriber.clone())
    
    3. Log subscription:
        LogInfo(f"{subscriber.module_name} subscribed to {event_types}")
    
    4. Send confirmation event:
        Publish(Event {
            event_type: SUBSCRIPTION_CONFIRMED,
            source: "Guardian",
            data: subscriber_info,
        })
```

**Unsubscribe:**

```rust
Unsubscribe(subscriber_id: String):
    1. For each (event_type, subscribers) in subscriptions:
        subscribers.retain(|s| s.subscriber_id != subscriber_id)
    
    2. Log unsubscription:
        LogInfo(f"Subscriber {subscriber_id} unsubscribed")
```

**Subscriber structure:**

```rust
struct Subscriber {
    subscriber_id: String,
    module_name: String,
    event_types: Vec<EventType>,
    
    // Callback для синхронной обработки
    callback: Option<Box<dyn Fn(Event)>>,
    
    // Или channel для асинхронной
    channel: Option<Sender<Event>>,
    
    // Фильтр для тонкой настройки
    filter: Option<EventFilter>,
    
    // Метаданные
    subscribed_at: uint64,
    events_received: uint64,
}

struct EventFilter {
    // Фильтр по источнику
    allowed_sources: Option<Vec<String>>,
    
    // Фильтр по приоритету
    min_priority: Priority,
    
    // Пользовательский предикат
    custom_predicate: Option<Box<dyn Fn(&Event) -> bool>>,
}
```

### 5.5 Event Publishing

**Асинхронная публикация (по умолчанию):**

```rust
Publish(event: Event):
    1. Enqueue event в event_queue:
        event_queue.push(event)
    
    2. Return immediately (non-blocking)
    
    // Background thread обрабатывает очередь:
    Background loop:
        while event_queue is not empty:
            event = event_queue.pop()
            ProcessEvent(event)

ProcessEvent(event: Event):
    1. Add to event_history (ring buffer)
    
    2. Get subscribers:
        subscribers = subscriptions.get(event.event_type)
    
    3. For each subscriber in subscribers:
        // Применить фильтр
        if subscriber.filter is Some:
            if not subscriber.filter.matches(&event):
                continue
        
        // Отправить событие
        if subscriber.callback is Some:
            subscriber.callback(event.clone())
        else if subscriber.channel is Some:
            subscriber.channel.send(event.clone())
        
        // Обновить статистику
        subscriber.events_received += 1
    
    4. Update event stats:
        stats.total_events += 1
        stats.events_by_type[event.event_type] += 1
```

**Синхронная публикация (блокирующая):**

```rust
PublishSync(event: Event):
    // Немедленная обработка, блокирует вызывающий поток
    ProcessEvent(event)
    
    // Дождаться обработки всеми подписчиками
    WaitForAllSubscribers()

Использование:
    Только для критичных событий, требующих гарантии обработки
    Например: CDNA_CHANGED, SYSTEM_SHUTDOWN
```

### 5.6 Контекстные срезы (Contextual Slices)

**Проблема:** Отправка всей CDNA (128 bytes) или большого состояния в каждом событии неэффективна.

**Решение:** Контекстные срезы — отправка только изменённых или релевантных данных.

**Пример:**

```rust
// Изменился mutation_rate в Evolution Constraints

// НЕправильно (отправка всей CDNA):
Event {
    event_type: CDNA_CHANGED,
    data: EventData {
        new_values: entire_cdna_128_bytes,  // ❌ Избыточно
    }
}

// Правильно (контекстный срез):
Event {
    event_type: CDNA_CHANGED,
    data: EventData {
        changed_fields: vec![
            ("mutation_rate", 0.05),
        ],
        affected_modules: vec!["Graph", "Evolution"],
        old_values: Some({
            "mutation_rate": 0.01,
        }),
        new_values: Some({
            "mutation_rate": 0.05,
        }),
        additional_context: {
            "section": "EVOLUTION_CONSTRAINTS",
            "offset": 96,
            "size": 4,
        }
    }
}

// Подписчик получает только релевантную информацию:
Graph.HandleEvent(event):
    if "mutation_rate" in event.data.changed_fields:
        new_rate = event.data.new_values["mutation_rate"]
        self.update_mutation_rate(new_rate)
```

### 5.7 Event Priority and Ordering

**Priority handling:**

```
CRITICAL events:
    - Обрабатываются немедленно
    - Могут прерывать обработку NORMAL events
    - Примеры: SYSTEM_SHUTDOWN, CRITICAL_INVARIANT_VIOLATED

HIGH events:
    - Обрабатываются с приоритетом
    - Не прерывают CRITICAL
    - Примеры: CDNA_CHANGED, MODULE_FAILED

NORMAL events:
    - Обычная обработка в порядке поступления
    - Примеры: NODE_ADDED, EDGE_ADDED

LOW events:
    - Могут быть отложены при высокой нагрузке
    - Примеры: PERFORMANCE_WARNING, CACHE_INVALIDATED
```

**Ordering guarantees:**

```
В пределах одного event_type:
    События обрабатываются в порядке публикации (FIFO)

Между разными event_types:
    Порядок не гарантируется (зависит от priority)

Для каждого подписчика:
    События одного типа доставляются в порядке публикации
```

### 5.8 Event History

**RingBuffer для хранения последних событий:**

```rust
struct EventHistory {
    buffer: RingBuffer<Event>,
    capacity: usize,                     // По умолчанию 10,000
    index: HashMap<EventType, Vec<usize>>, // Индекс для быстрого поиска
}

GetEventHistory(filter: EventFilter) -> Vec<Event>:
    1. If filter.event_type is specified:
        indices = index.get(filter.event_type)
        events = indices.map(|i| buffer[i])
    else:
        events = buffer.iter()
    
    2. Apply additional filters:
        events = events.filter(|e| filter.matches(e))
    
    3. Sort by timestamp (if needed)
    
    4. Return events
```

**Использование Event History:**

- Debugging — анализ последовательности событий перед ошибкой
- Replay — воспроизведение событий для тестирования
- Audit — проверка, что происходило в системе
- Metrics — статистика по типам событий

---

## 6. Оркестрация модулей

### 6.1 Module Registration

**Модули регистрируются в Guardian при старте:**

```rust
RegisterModule(module: ModuleHandle):
    1. Validate module:
        assert module.module_id is unique
        assert module implements ModuleInterface
    
    2. Check dependencies:
        for dep in module.dependencies:
            if dep not in modules:
                return Error("Missing dependency: {dep}")
    
    3. Add to modules registry:
        modules[module.module_id] = module
        module_states[module.module_id] = ModuleState::new()
    
    4. Update dependency graph:
        dependency_graph.add_node(module.module_id)
        for dep in module.dependencies:
            dependency_graph.add_edge(dep, module.module_id)
    
    5. Publish event:
        Publish(Event {
            event_type: MODULE_REGISTERED,
            source: "Guardian",
            data: module_info,
        })
```

**ModuleInterface:**

```rust
trait ModuleInterface {
    fn initialize(&mut self, cdna: &CDNA) -> Result<()>;
    fn start(&mut self) -> Result<()>;
    fn pause(&mut self);
    fn resume(&mut self);
    fn shutdown(&mut self);
    fn health_check(&self) -> HealthStatus;
    fn handle_event(&mut self, event: Event);
}
```

### 6.2 Module Lifecycle

**Стадии жизненного цикла модуля:**

```
UNINITIALIZED → INITIALIZING → READY → RUNNING → PAUSED → SHUTTING_DOWN → SHUTDOWN
                     ↓                      ↓         ↓
                   FAILED ←────────────────┴─────────┘
                     ↓
                  RECOVERING
                     ↓
                   READY
```

**InitializeModule(module_id: String):**

```rust
InitializeModule(module_id: String) -> Result<()>:
    1. Get module:
        module = modules.get_mut(module_id)
    
    2. Check dependencies initialized:
        for dep in module.dependencies:
            if module_states[dep].status != READY and != RUNNING:
```return Error("Dependency {dep} not ready")

```
3. Update status:
    module_states[module_id].status = INITIALIZING

4. Initialize with CDNA:
    result = module.interface.initialize(&self.cdna)
    
    if result.is_err():
        module_states[module_id].status = FAILED
        Publish(Event {
            event_type: MODULE_FAILED,
            source: module_id,
            data: error_info,
        })
        return result

5. Update status:
    module_states[module_id].status = READY

6. Publish event:
    Publish(Event {
        event_type: MODULE_INITIALIZED,
        source: module_id,
    })

7. return Ok()
```

````

**StartModule(module_id: String):**

```rust
StartModule(module_id: String) -> Result<()>:
    1. Get module:
        module = modules.get_mut(module_id)
    
    2. Check status:
        if module_states[module_id].status != READY:
            return Error("Module not ready")
    
    3. Start dependencies first (topological order):
        deps = dependency_graph.get_dependencies(module_id)
        for dep in deps:
            if module_states[dep].status != RUNNING:
                StartModule(dep)?
    
    4. Start module:
        result = module.interface.start()
        
        if result.is_err():
            module_states[module_id].status = FAILED
            return result
    
    5. Update status:
        module_states[module_id].status = RUNNING
        module_states[module_id].last_heartbeat = now()
    
    6. Publish event:
        Publish(Event {
            event_type: MODULE_STARTED,
            source: module_id,
        })
    
    7. return Ok()
````

**PauseModule(module_id: String):**

```rust
PauseModule(module_id: String):
    1. Pause dependents first (reverse topological order):
        dependents = dependency_graph.get_dependents(module_id)
        for dependent in dependents:
            if module_states[dependent].status == RUNNING:
                PauseModule(dependent)
    
    2. Pause module:
        module.interface.pause()
    
    3. Update status:
        module_states[module_id].status = PAUSED
    
    4. Publish event:
        Publish(Event {
            event_type: MODULE_PAUSED,
            source: module_id,
        })
```

**ShutdownModule(module_id: String):**

```rust
ShutdownModule(module_id: String):
    1. Shutdown dependents first:
        dependents = dependency_graph.get_dependents(module_id)
        for dependent in dependents:
            ShutdownModule(dependent)
    
    2. Update status:
        module_states[module_id].status = SHUTTING_DOWN
    
    3. Graceful shutdown:
        module.interface.shutdown()
    
    4. Update status:
        module_states[module_id].status = SHUTDOWN
    
    5. Publish event:
        Publish(Event {
            event_type: MODULE_SHUTDOWN,
            source: module_id,
        })
```

### 6.3 Dependency Management

**Граф зависимостей:**

```
Пример:
    Grid зависит от: Token
    Graph зависит от: Token, Connection, Grid
    Evolution зависит от: Graph, Connection

Dependency Graph:
    Token
      ↓
    Grid
      ↓
    Connection
      ↓
    Graph
      ↓
    Evolution

При инициализации:
    1. Token (нет зависимостей)
    2. Grid (зависит от Token)
    3. Connection (зависит от Token)
    4. Graph (зависит от Token, Connection, Grid)
    5. Evolution (зависит от Graph, Connection)

При shutdown (обратный порядок):
    1. Evolution
    2. Graph
    3. Connection, Grid
    4. Token
```

**CheckDependencies(module_id: String) -> Result<()>:**

```rust
CheckDependencies(module_id: String) -> Result<()>:
    1. Get dependencies:
        deps = dependency_graph.get_dependencies(module_id)
    
    2. Check each dependency:
        for dep in deps:
            dep_status = module_states[dep].status
            
            if dep_status == FAILED:
                return Error("Dependency {dep} has failed")
            
            if dep_status != READY and dep_status != RUNNING:
                return Error("Dependency {dep} not ready (status: {dep_status})")
    
    3. return Ok()


**DetectCircularDependencies() -> Option<Vec<String>>:**


DetectCircularDependencies() -> Option<Vec<String>>:
    // Используем DFS для поиска циклов
    
    visited = Set()
    rec_stack = Set()
    
    for module_id in modules:
        if module_id not in visited:
            cycle = DFS_FindCycle(module_id, visited, rec_stack)
            if cycle is not None:
                return Some(cycle)
    
    return None
```

### 6.4 Health Monitoring

**Continuous health monitoring каждого модуля:**

```rust
HealthMonitor {
    health_checks: Vec<HealthCheck>,
    check_interval: uint64,              // Интервал проверки (ms)
    degradation_threshold: f32,          // Порог для degradation
    failure_threshold: f32,              // Порог для failure
}

HealthCheck {
    check_id: String,
    check_type: HealthCheckType,
    check_fn: Box<dyn Fn(&ModuleState) -> f32>, // Возвращает score 0.0-1.0
    weight: f32,                         // Вес этой проверки
}

enum HealthCheckType {
    HEARTBEAT,                           // Регулярные heartbeats от модуля
    ERROR_RATE,                          // Частота ошибок
    RESPONSE_TIME,                       // Время отклика
    MEMORY_USAGE,                        // Использование памяти
    CPU_USAGE,                           // Использование CPU
    THROUGHPUT,                          // Производительность
    CUSTOM,                              // Пользовательская проверка
}
```

**CheckHealth() -> SystemHealth:**

```rust
CheckHealth() -> SystemHealth:
    system_health = SystemHealth::new()
    
    for (module_id, module_state) in module_states:
        module_health = CheckModuleHealth(module_id, module_state)
        system_health.add_module(module_id, module_health)
    
    // Compute overall health
    system_health.overall_score = ComputeOverallScore(system_health)
    system_health.status = DetermineStatus(system_health.overall_score)
    
    return system_health

CheckModuleHealth(module_id: String, state: ModuleState) -> ModuleHealth:
    scores = Vec::new()
    
    for health_check in health_monitor.health_checks:
        score = health_check.check_fn(state)
        weighted_score = score * health_check.weight
        scores.push(weighted_score)
    
    total_score = scores.sum() / health_monitor.health_checks.len()
    
    return ModuleHealth {
        module_id: module_id,
        health_score: total_score,
        last_check: now(),
        issues: DetectIssues(state, scores),
    }
```

**Health Check implementations:**

```rust
// Heartbeat check
HeartbeatCheck(state: ModuleState) -> f32:
    time_since_heartbeat = now() - state.last_heartbeat
    max_interval = 1000  // 1 second
    
    if time_since_heartbeat > max_interval * 10:
        return 0.0  // Module likely dead
    else if time_since_heartbeat > max_interval * 2:
        return 0.3  // Concerning
    else if time_since_heartbeat > max_interval:
        return 0.7  // Slightly delayed
    else:
        return 1.0  // Healthy

// Error rate check
ErrorRateCheck(state: ModuleState) -> f32:
    if state.operations_count == 0:
        return 1.0
    
    error_rate = state.errors_count / state.operations_count
    
    if error_rate > 0.1:  // >10% errors
        return 0.0
    else if error_rate > 0.05:  // >5% errors
        return 0.5
    else if error_rate > 0.01:  // >1% errors
        return 0.8
    else:
        return 1.0

// Load check
LoadCheck(state: ModuleState) -> f32:
    load = state.current_load
    
    if load > 0.95:  // >95% load
        return 0.1
    else if load > 0.8:  // >80% load
        return 0.5
    else if load > 0.6:  // >60% load
        return 0.8
    else:
        return 1.0
```

### 6.5 Graceful Degradation

**Когда система обнаруживает проблемы, она не падает сразу, а деградирует постепенно:**

```rust
enum DegradationLevel {
    NONE,                                // 1.0 - 0.9: Всё отлично
    MINOR,                               // 0.9 - 0.7: Небольшие проблемы
    MODERATE,                            // 0.7 - 0.5: Средние проблемы
    SEVERE,                              // 0.5 - 0.3: Серьёзные проблемы
    CRITICAL,                            // 0.3 - 0.0: Критические проблемы
}

HandleDegradation(level: DegradationLevel):
    match level:
        NONE:
            // Всё хорошо, ничего не делать
            return
        
        MINOR:
            // Логировать warning
            LogWarning("System experiencing minor degradation")
            // Увеличить частоту health checks
            health_monitor.check_interval /= 2
        
        MODERATE:
            // Отключить некритичные функции
            DisableNonCriticalFeatures()
            // Снизить нагрузку
            ReduceWorkload()
            // Попытаться восстановиться
            AttemptAutoRecovery()
        
        SEVERE:
            // Приостановить проблемные модули
            PauseFailingModules()
            // Переключиться на минимальный режим
            EnterMinimalMode()
            // Алерт администратору
            AlertAdministrator("Severe degradation detected")
        
        CRITICAL:
            // Сохранить состояние
            SaveSystemState()
            // Graceful shutdown некритичных модулей
            ShutdownNonCriticalModules()
            // Алерт
            AlertAdministrator("CRITICAL: System near failure")
            // Подготовиться к полному shutdown
            PrepareForShutdown()
```

**Стратегии degradation:**

**1. Feature Reduction:**

```
Отключить некритичные функции:
    • Снизить частоту эволюционных операций
    • Отключить автоматическую компакцию
    • Снизить точность некоторых вычислений
    • Отключить детальное логирование
```

**2. Load Shedding:**

```
Снизить нагрузку:
    • Отклонять новые запросы (backpressure)
    • Увеличить batch size для операций
    • Отложить некритичные задачи
    • Использовать sampling вместо полной обработки
```

**3. Resource Reallocation:**

```
Перераспределить ресурсы:
    • Выделить больше памяти критичным модулям
    • Приоритезировать критичные потоки
    • Освободить кэши некритичных модулей
    • Снизить параллелизм для стабильности
```

**4. Isolation:**

```
Изолировать проблемные компоненты:
    • Приостановить failing модуль
    • Перенаправить запросы к здоровым инстансам
    • Запустить модуль в "read-only" режиме
    • Создать новый инстанс модуля
```

### 6.6 Auto Recovery

**Автоматическое восстановление при сбоях:**

```rust
AttemptAutoRecovery(module_id: String):
    1. Identify failure type:
        failure_type = AnalyzeFailure(module_id)
    
    2. Apply recovery strategy:
        match failure_type:
            TRANSIENT_ERROR:
                // Временная ошибка, просто перезапустить
                RestartModule(module_id)
            
            RESOURCE_EXHAUSTION:
                // Нехватка ресурсов
                FreeResources()
                RestartModule(module_id)
            
            CORRUPTED_STATE:
                // Повреждённое состояние
                RestoreFromCheckpoint(module_id)
                RestartModule(module_id)
            
            DEPENDENCY_FAILURE:
                // Проблема с зависимостью
                RecoverDependency(dependency_id)
                RestartModule(module_id)
            
            CONFIGURATION_ERROR:
                // Ошибка конфигурации
                ReloadConfiguration(module_id)
                RestartModule(module_id)
            
            UNKNOWN:
                // Неизвестная причина
                LogError("Unknown failure type for {module_id}")
                // Попробовать полный сброс
                ResetModule(module_id)
    
    3. Verify recovery:
        if CheckModuleHealth(module_id).health_score > 0.8:
            Publish(Event {
                event_type: MODULE_RECOVERED,
                source: module_id,
            })
            return Success
        else:
            // Recovery failed
            return Failure

RestartModule(module_id: String):
    1. Shutdown module gracefully
    2. Wait for cleanup (timeout: 5 seconds)
    3. Re-initialize module with current CDNA
    4. Start module
    5. Verify health
```

**Backoff strategy для repeated failures:**

```rust
struct RecoveryState {
    attempt_count: u32,
    last_attempt: uint64,
    backoff_duration: uint64,           // Exponential backoff
}

ShouldAttemptRecovery(module_id: String) -> bool:
    state = recovery_states[module_id]
    
    // Exponential backoff: 1s, 2s, 4s, 8s, 16s, 32s, ...
    backoff = 1000 * (2 ^ state.attempt_count)
    max_backoff = 60000  // Max 1 minute
    backoff = min(backoff, max_backoff)
    
    time_since_last = now() - state.last_attempt
    
    if time_since_last < backoff:
        return false  // Too soon
    
    if state.attempt_count > MAX_RECOVERY_ATTEMPTS:
        // Too many attempts, give up
        LogError("Max recovery attempts exceeded for {module_id}")
        MarkModuleAsFailed(module_id)
        return false
    
    return true
```

---

## 7. Производительность

### 7.1 Performance Bottleneck Analysis

**Potential bottlenecks в Guardian:**

**1. CDNA Access:**

```
Проблема: Каждая валидация может требовать чтения CDNA
Решение: Кэширование "горячих" параметров в HotCache
```

**2. Event Publishing:**

```
Проблема: Синхронная публикация событий блокирует издателя
Решение: Асинхронная очередь с background обработкой
```

**3. Validation:**

```
Проблема: Валидация каждой операции может быть дорогой
Решение: 
    • Делегирование валидации модулям для простых операций
    • Batch валидация
    • Кэширование результатов валидации
```

**4. Module Coordination:**

```
Проблема: Синхронизация модулей может создавать contention
Решение:
    • Lock-free структуры данных где возможно
    • Минимизация shared state
    • Copy-on-write для CDNA snapshots
```

### 7.2 Hot Cache

**Кэширование часто используемых параметров CDNA:**

```rust
HotCache {
    cache: HashMap<String, CachedValue>,
    hit_count: HashMap<String, u64>,
    miss_count: HashMap<String, u64>,
    ttl: uint64,                         // Time to live (ms)
    max_size: usize,
}

struct CachedValue {
    value: Value,
    cached_at: uint64,
    access_count: u64,
}

GetCached(param_name: String) -> Option<Value>:
    if param_name in cache:
        cached = cache[param_name]
        
        // Check TTL
        age = now() - cached.cached_at
        if age > ttl:
            cache.remove(param_name)
            miss_count[param_name] += 1
            return None
        
        // Hit
        cached.access_count += 1
        hit_count[param_name] += 1
        return Some(cached.value.clone())
    else:
        // Miss
        miss_count[param_name] += 1
        return None

CacheParameter(param_name: String, value: Value):
    // Evict if cache full
    if cache.len() >= max_size:
        EvictLeastUsed()
    
    cache[param_name] = CachedValue {
        value: value,
        cached_at: now(),
        access_count: 0,
    }

EvictLeastUsed():
    // Find entry with lowest access_count
    min_entry = cache.iter().min_by_key(|(_, v)| v.access_count)
    cache.remove(min_entry.key)
```

**Какие параметры кэшировать:**

```
"Горячие" (частый доступ):
    • max_node_degree
    • allowed_connection_types
    • min/max_connection_weight
    • topology_flags

"Холодные" (редкий доступ):
    • dimension_semantic_ids
    • evolution_flags
    • reserved fields
```

### 7.3 Look-Up Tables (LUT)

**Предвычисленные таблицы для дорогих операций:**

```rust
LUTManager {
    tables: HashMap<String, LookupTable>,
}

struct LookupTable {
    table_id: String,
    function_name: String,               // Какая функция
    input_range: Range,
    output_type: Type,
    entries: Vec<(Input, Output)>,
    interpolation: InterpolationType,    // LINEAR | CUBIC | NEAREST
}

BuildLUT(function: String, range: Range, resolution: usize) -> LookupTable:
    1. Create table:
        table = LookupTable::new(function, range)
    
    2. Precompute values:
        step = (range.max - range.min) / resolution
        
        for i in 0..resolution:
            input = range.min + i * step
            output = EvaluateFunction(function, input)
            table.entries.push((input, output))
    
    3. Return table

QueryLUT(table_id: String, input: f32) -> f32:
    table = tables[table_id]
    
    // Binary search для ближайших значений
    (low, high) = BinarySearch(table.entries, input)
    
    // Интерполяция
    match table.interpolation:
        NEAREST:
            return ClosestValue(low, high, input)
        LINEAR:
            return LinearInterpolate(low, high, input)
        CUBIC:
            return CubicInterpolate(low, high, input)
```

**Примеры использования LUT:**

```
Weight decay function:
    decay(t) = base_weight * decay_rate ^ t
    
    Вместо вычисления каждый раз:
        LUT для t ∈ [0, 10000] с шагом 100
        Lookup: O(log n) вместо O(1) для pow, но избегаем float ops

Distance normalization:
    normalized_distance(d, scale) = d / scale
    
    LUT для часто используемых scale values

Sigmoid activation:
    sigmoid(x) = 1 / (1 + e^(-x))
    
    LUT для x ∈ [-10, 10]
```

### 7.4 Memory Optimization

**Оптимизация memory layout:**

**1. Cache-friendly CDNA layout:**

```
Разместить "горячие" поля в начале структуры:
    Первые 64 bytes: часто используемые параметры
    Следующие 64 bytes: редко используемые

Это гарантирует, что "горячие" данные окажутся в одной cache line
```

**2. CDNA Snapshots:**

```
Вместо копирования всей CDNA для каждого модуля:
    • Использовать Copy-on-Write (CoW)
    • Shared reference пока CDNA не изменена
    • При изменении создать новую копию только для изменённых модулей

struct CDNASnapshot {
    version: uint32,
    data: Arc<[u8; 128]>,               // Shared ownership
}

Clone is cheap (только increment ref count)
```

**3. Event Data Optimization:**

```
Использовать arena allocator для EventData:
    • Все события из одного batch в одной memory region
    • Освобождение всей arena за раз
    • Меньше fragmentation
```

### 7.5 Profiling and Metrics

**Непрерывный профилинг работы Guardian:**

```rust
struct GuardianMetrics {
    // CDNA metrics
    cdna_access_count: u64,
    cdna_cache_hit_rate: f32,
    cdna_version: uint32,
    cdna_change_count: u64,
    
    // Validation metrics
    validations_performed: u64,
    validations_passed: u64,
    validations_failed: u64,
    validation_avg_time_us: f32,
    
    // Event metrics
    events_published: u64,
    events_delivered: u64,
    event_queue_size: usize,
    event_delivery_avg_time_us: f32,
    
    // Module metrics
    modules_registered: usize,
    modules_running: usize,
    modules_failed: usize,
    
    // Health metrics
    system_health_score: f32,
    degradation_level: DegradationLevel,
    
    // Performance metrics
    cpu_usage: f32,
    memory_usage: u64,
    avg_response_time_us: f32,
}

CollectMetrics() -> GuardianMetrics:
    // Собрать метрики со всех компонентов
    metrics = GuardianMetrics::new()
    
    metrics.cdna_access_count = hot_cache.hit_count.sum() + hot_cache.miss_count.sum()
    metrics.cdna_cache_hit_rate = hot_cache.hit_count.sum() / metrics.cdna_access_count
    
    metrics.validations_performed = validator.stats.total_validations
    metrics.validations_passed = validator.stats.passed
    metrics.validations_failed = validator.stats.failed
    
    metrics.events_published = event_broker.stats.total_events
    metrics.event_queue_size = event_broker.event_queue.len()
    
    metrics.system_health_score = CheckHealth().overall_score
    
    return metrics
```

**Periodic metrics reporting:**

```rust
// Каждые N секунд публиковать метрики
MetricsReporter:
    loop:
        sleep(METRICS_INTERVAL)
        
        metrics = CollectMetrics()
        
        // Publish as event
        Publish(Event {
            event_type: METRICS_REPORTED,
            source: "Guardian",
            data: metrics,
        })
        
        // Log если есть проблемы
        if metrics.system_health_score < 0.7:
            LogWarning(f"System health low: {metrics.system_health_score}")
        
        if metrics.event_queue_size > MAX_QUEUE_SIZE:
            LogWarning(f"Event queue overflow: {metrics.event_queue_size}")
```

### 7.6 Scalability

**Strategies для масштабирования Guardian:**

**1. Horizontal Scaling (будущее):**

```
Для очень больших систем можно распределить Guardian:
    • Guardian Primary — управляет CDNA и координацией
    • Guardian Workers — обрабатывают events и validation
    • Sharding по module_id или event_type
```

**2. Vertical Scaling:**

```
Оптимизация для single-instance performance:
    • Lock-free data structures
    • Thread-per-core architecture
    • Zero-copy где возможно
    • SIMD для batch validation
```

**3. Lazy Evaluation:**

```
Не делать работу пока не нужно:
    • Metrics собираются только при запросе
    • Health checks только для активных модулей
    • History cleaning только при необходимости
```

---

## 8. Lifecycle Management

### 8.1 System Initialization

**Полная инициализация NeuroGraph OS:**

```rust
InitializeSystem(config: SystemConfig) -> Result<Guardian>:
    1. Create Guardian:
        guardian = Guardian::new()
    
    2. Load CDNA:
        cdna = LoadCDNA(config.cdna_path)?
        guardian.constitution_keeper.set_cdna(cdna)
    
    3. Initialize components:
        guardian.validator.update_rules(&cdna)
        guardian.event_broker.start()
        guardian.orchestrator.initialize()
        guardian.optimizer.build_luts(&cdna)
    
    4. Register core modules:
        guardian.register_module(TokenModule)?
        guardian.register_module(GridModule)?
        guardian.register_module(ConnectionModule)?
        guardian.register_module(GraphModule)?
    
    5. Check dependency graph:
        if guardian.orchestrator.detect_circular_dependencies().is_some():
            return Error("Circular dependencies detected")
    
    6. Initialize modules in topological order:
        for module_id in guardian.orchestrator.topological_sort():
            guardian.initialize_module(module_id)?
    
    7. Start health monitoring:
        guardian.start_health_monitor()
    
    8. Publish system ready event:
        guardian.publish(Event {
            event_type: SYSTEM_INITIALIZED,
            source: "Guardian",
        })
    
    9. return Ok(guardian)
```

### 8.2 Runtime Operation

**Нормальная работа системы:**

```rust
Guardian main loop:
    while running:
        // 1. Process event queue
        ProcessEvents()
        
        // 2. Monitor module health
        if time_for_health_check():
            health = CheckHealth()
            
            if health.overall_score < degradation_threshold:
                HandleDegradation(health.degradation_level)
        
        // 3. Validate pending operations (if any)
        ProcessValidationQueue()
        
        // 4. Handle CDNA proposals
        ProcessCDNAProposals()
        
        // 5. Collect metrics
        if time_for_metrics():
            metrics = CollectMetrics()
            PublishMetrics(metrics)
        
        // 6. Cleanup
        if time_for_cleanup():
            CleanupEventHistory()
            CleanupQuarantine()
            EvictExpiredCache()
        
        // 7. Sleep (если нет работы)
        if event_queue.is_empty():
            sleep(IDLE_SLEEP_MS)
```

### 8.3 Graceful Shutdown

**Корректное завершение работы:**

```rust
Shutdown():
    1. Publish shutdown event:
        PublishSync(Event {
            event_type: SHUTDOWN_REQUESTED,
            source: "Guardian",
            priority: CRITICAL,
        })
    
    2. Stop accepting new operations:
        validator.stop_accepting()
        event_broker.stop_accepting()
    
    3. Wait for pending operations:
        timeout = 30_000  // 30 seconds
        start_time = now()
        
        while has_pending_operations() and (now() - start_time) < timeout:
            sleep(100)
        
        if has_pending_operations():
            LogWarning("Timeout waiting for pending operations")
    
    4. Shutdown modules in reverse topological order:
        modules = orchestrator.reverse_topological_sort()
        for module_id in modules:
            ShutdownModule(module_id)
    
    5. Save system state:
        SaveCDNAHistory()
        SaveEventHistory()
        SaveMetrics()
    
    6. Cleanup resources:
        event_broker.shutdown()
        health_monitor.shutdown()
        optimizer.cleanup()
    
    7. Publish final event:
        PublishSync(Event {
            event_type: SYSTEM_SHUTDOWN,
            source: "Guardian",
        })
    
    8. Log shutdown:
        LogInfo("Guardian shutdown complete")
```

### 8.4 Sleep and Wake

**Система поддерживает "сон" для снижения потребления ресурсов:**

```rust
Sleep():
    1. Pause non-critical modules:
        for module_id in orchestrator.get_non_critical_modules():
            PauseModule(module_id)
    
    2. Reduce monitoring frequency:
        health_monitor.set_interval(SLEEP_CHECK_INTERVAL)
        metrics_reporter.set_interval(SLEEP_METRICS_INTERVAL)
    
    3. Flush caches (опционально):
        if config.flush_caches_on_sleep:
            hot_cache.clear()
            optimizer.clear_luts()
    
    4. Save state:
        SaveSystemState()
    
    5. Publish event:
        Publish(Event {
            event_type: SYSTEM_SLEEP,
            source: "Guardian",
        })
    
    6. Enter low-power mode:
        lifecycle_stage = LifecycleStage::SLEEPING

Wake():
    1. Restore monitoring frequency:
        health_monitor.set_interval(NORMAL_CHECK_INTERVAL)
        metrics_reporter.set_interval(NORMAL_METRICS_INTERVAL)
    
    2. Rebuild caches:
        if hot_cache.is_empty():
            PreloadHotCache()
        
        if optimizer.luts.is_empty():
            optimizer.build_luts(&cdna)
    
    3. Resume modules:
        for module_id in orchestrator.get_paused_modules():
        ResumeModule(module_id)
        
	4. Check system health:
	    health = CheckHealth()
	    if health.overall_score < 0.5:
	        LogWarning("Low health after wake, attempting recovery")
	        AttemptAutoRecovery()
	
	5. Apply pending CDNA changes (если были во время сна):
	    if constitution_keeper.has_pending_proposals():
	        ProcessPendingProposals()
	
	6. Publish event:
	    Publish(Event {
	        event_type: SYSTEM_WAKE,
	        source: "Guardian",
	    })
	
	7. Enter normal mode:
	    lifecycle_stage = LifecycleStage::RUNNING
```


### 8.5 Update and Migration

**Обновление системы без полной перезагрузки:**

```rust
ApplyUpdate(update: SystemUpdate) -> Result<()>:
    1. Validate update:
        if not ValidateUpdate(update):
            return Error("Invalid update")
    
    2. Create backup:
        backup = CreateSystemBackup()
    
    3. Enter maintenance mode:
        lifecycle_stage = LifecycleStage::MAINTENANCE
        Publish(Event {
            event_type: MAINTENANCE_MODE_ENTERED,
        })
    
    4. Apply update components:
        for component in update.components:
            match component.type:
                CDNA_UPDATE:
                    ApplyCDNAUpdate(component)?
                
                MODULE_UPDATE:
                    UpdateModule(component)?
                
                CONFIGURATION_UPDATE:
                    UpdateConfiguration(component)?
                
                CODE_UPDATE:
                    // Hot-reload если возможно
                    HotReloadModule(component)?
    
    5. Verify update:
        if not VerifySystemIntegrity():
            LogError("Update verification failed, rolling back")
            RestoreFromBackup(backup)
            return Error("Update failed")
    
    6. Exit maintenance mode:
        lifecycle_stage = LifecycleStage::RUNNING
        Publish(Event {
            event_type: MAINTENANCE_MODE_EXITED,
        })
    
    7. return Ok()
````

---

## 9. Интеграция с модулями

### 9.1 Token v2.0 Integration

**Guardian использует Token через Grid:**

```rust
// Guardian не создаёт Token напрямую
// Он валидирует операции с Token

ValidateTokenOperation(operation: TokenOperation) -> ValidationResult:
    match operation.type:
        CREATE_TOKEN:
            token = operation.token
            
            // Validate weight range
            if not (min_token_weight <= token.weight <= max_token_weight):
                return Rejected("Token weight out of range")
            
            // Validate field properties
            if not (min_field_radius <= token.field_radius <= max_field_radius):
                return Rejected("Field radius out of range")
            
            if not (min_field_strength <= token.field_strength <= max_field_strength):
                return Rejected("Field strength out of range")
            
            // Check flags
            if token.weight == 0.0 and not token_flags.ALLOW_ZERO_WEIGHT:
                return Rejected("Zero weight tokens not allowed")
            
            if token.fsc_code.is_empty() and token_flags.REQUIRE_FSC:
                return Rejected("FSC code required")
            
            return Approved
        
        MIGRATE_TOKEN:
            if not token_flags.ALLOW_MIGRATION:
                return Rejected("Token migration not allowed")
            
            return Approved
```

**Event flow:**

```
Grid создаёт Token:
    1. Grid::CreateToken(token_data)
    2. Grid → Guardian: ValidateOperation(CREATE_TOKEN)
    3. Guardian: проверяет CDNA.token_base_properties
    4. Guardian → Grid: Approved/Rejected
    5. Если Approved:
        Grid.InsertNode(token)
        Grid → Guardian: Publish(NODE_ADDED event)
    6. Guardian распространяет событие подписчикам
```

### 9.2 Connection v1.0 Integration

**Guardian валидирует Connection и использует его для эволюции:**

```rust
ValidateConnectionOperation(operation: ConnectionOperation) -> ValidationResult:
    match operation.type:
        CREATE_CONNECTION:
            connection = operation.connection
            
            // Validate type is allowed
            type_bit = 1 << connection.connection_type
            if (allowed_connection_types & type_bit) == 0:
                return Rejected("Connection type not allowed")
            
            // Validate weight range
            if not (min_connection_weight <= connection.weight <= max_connection_weight):
                return Rejected("Connection weight out of range")
            
            // Check distance (if enforced)
            if enforce_distance_limit:
                from_token = Grid::GetNode(connection.from_token_id)
                to_token = Grid::GetNode(connection.to_token_id)
                distance = Grid::Distance(from_token, to_token, level=L8_ABSTRACT)
                
                if not (connection_distance_min <= distance <= connection_distance_max):
                    return Rejected("Connection distance out of range")
            
            // Check self-loops
            if connection.from_token_id == connection.to_token_id:
                if not topology_flags.ALLOW_SELF_LOOPS:
                    return Rejected("Self-loops not allowed")
            
            // Check multi-edges
            if not topology_flags.ALLOW_MULTI_EDGES:
                if Graph::ContainsEdge(connection.from_token_id, connection.to_token_id):
                    return Rejected("Multi-edges not allowed")
            
            return Approved
        
        UPDATE_CONNECTION:
            // Validate new weight is in range
            new_weight = operation.new_weight
            if not (min_connection_weight <= new_weight <= max_connection_weight):
                return Rejected("New weight out of range")
            
            return Approved
```

**Connection используется как "адаптивная ДНК":**

```
Connection.metadata содержит эволюционные параметры:
    {
        "mutation_rate": 0.05,
        "crossover_rate": 0.1,
        "selection_pressure": 0.7,
        ...
    }

Guardian валидирует изменения:
    Если Intuition предлагает mutation_rate = 0.15:
        1. Guardian проверяет:
            min_mutation_rate (0.01) <= 0.15 <= max_mutation_rate (0.2)
        2. Если OK, разрешает изменение
        3. Публикует событие CONNECTION_UPDATED
        4. Graph получает событие и обновляет поведение
```

### 9.3 Grid v2.0 Integration

**Guardian валидирует пространственные операции:**

```rust
ValidateGridOperation(operation: GridOperation) -> ValidationResult:
    match operation.type:
        INSERT_NODE:
            token = operation.token
            level = operation.level
            
            // Validate coordinates
            for i in 0..8:
                coord = token.coordinates[i]
                
                // Check dimension flags
                if dimension_flags[i].BOUNDED:
                    if coord < DIMENSION_MIN or coord > DIMENSION_MAX:
                        return Rejected("Coordinate out of bounds")
                
                if dimension_flags[i].DISCRETE:
                    if coord != floor(coord):
                        return Rejected("Coordinate must be discrete")
            
            return Approved
        
        UPDATE_COORDINATES:
            // Similar validation
            ...
```

**Пространственные эвристики для Guardian:**

```
Guardian может использовать Grid для:
    1. Проверки distance при создании Connection
    2. Spatial anomaly detection (токены в неожиданных местах)
    3. Визуализации health metrics в пространстве
```

### 9.4 Graph v2.0 Integration

**Guardian координирует топологические операции:**

```rust
ValidateGraphOperation(operation: GraphOperation) -> ValidationResult:
    match operation.type:
        ADD_EDGE:
            edge = operation.edge
            
            // Validate degree limit
            from_degree = Graph::GetDegree(edge.from_id, BOTH)
            if from_degree >= max_node_degree:
                return Rejected("Max degree exceeded for source node")
            
            to_degree = Graph::GetDegree(edge.to_id, BOTH)
            if to_degree >= max_node_degree:
                return Rejected("Max degree exceeded for target node")
            
            return Approved
        
        MUTATE_CONNECTIONS:
            mutation_rate = operation.mutation_rate
            
            // Validate mutation rate in allowed range
            if not (min_mutation_rate <= mutation_rate <= max_mutation_rate):
                return Rejected("Mutation rate out of range")
            
            // Check if mutations are allowed
            if not evolution_flags.ALLOW_MUTATIONS:
                return Rejected("Mutations not allowed by CDNA")
            
            return Approved
        
        APPLY_CROSSOVER:
            // Similar checks for crossover
            ...
        
        APPLY_SELECTION:
            // Similar checks for selection
            ...
```

**Топологические инварианты через Guardian:**

```
Guardian периодически проверяет Graph:
    1. CheckSystemInvariants() вызывает Graph.ValidateIntegrity()
    2. Graph проверяет:
        • Симметрию индексов
        • Консистентность рёбер
        • Соблюдение max_degree
    3. Если нарушения найдены:
        Guardian → Graph: RepairTopology()
        Guardian логирует нарушения
```

### 9.5 Future Module Integration

**Шаблон для интеграции новых модулей:**

```rust
// 1. Модуль реализует ModuleInterface
struct MyModule {
    guardian_ref: Arc<Guardian>,
    cdna_snapshot: CDNASnapshot,
}

impl ModuleInterface for MyModule {
    fn initialize(&mut self, cdna: &CDNA) -> Result<()> {
        // Получить snapshot CDNA
        self.cdna_snapshot = cdna.snapshot()
        
        // Подписаться на релевантные события
        self.guardian_ref.subscribe(Subscriber {
            subscriber_id: "my_module",
            module_name: "MyModule",
            event_types: vec![CDNA_CHANGED, NODE_ADDED],
            callback: Box::new(|event| self.handle_event(event)),
        })
        
        Ok(())
    }
    
    fn handle_event(&mut self, event: Event) {
        match event.event_type:
            CDNA_CHANGED:
                // Обновить локальный snapshot
                if "relevant_param" in event.data.changed_fields:
                    self.update_behavior(event.data.new_values)
            
            NODE_ADDED:
                // Обработать новый узел
                self.process_new_node(event.data)
    }
}

// 2. Модуль валидирует операции через Guardian
MyModule::PerformOperation(operation):
    // Сначала валидация
    result = self.guardian_ref.validate_operation(operation)
    
    if result == Approved:
        // Выполнить операцию
        self.execute(operation)
        
        // Опубликовать событие
        self.guardian_ref.publish(Event {
            event_type: MY_MODULE_OPERATION,
            source: "MyModule",
            data: operation_data,
        })
    else:
        // Логировать отказ
        LogWarning("Operation rejected: {result.reason}")
```

---

## 10. Сериализация и персистентность

### 10.1 Формат файла Guardian State

**Guardian сохраняет своё состояние для восстановления:**

```
╔═══════════════════════════════════════════════════════════╗
║              GUARDIAN STATE FILE FORMAT                   ║
╠═══════════════════════════════════════════════════════════╣
║                  HEADER (256 bytes)                       ║
╠═══════════════════════════════════════════════════════════╣
║             CURRENT CDNA (128 bytes)                      ║
╠═══════════════════════════════════════════════════════════╣
║             CDNA HISTORY (variable)                       ║
╠═══════════════════════════════════════════════════════════╣
║          MODULE REGISTRY (variable)                       ║
╠═══════════════════════════════════════════════════════════╣
║           EVENT HISTORY (variable)                        ║
╠═══════════════════════════════════════════════════════════╣
║            METRICS (variable)                             ║
╠═══════════════════════════════════════════════════════════╣
║           FOOTER (128 bytes)                              ║
╚═══════════════════════════════════════════════════════════╝
```

### 10.2 Guardian State Header

```
Offset  Size  Type      Field                   Description
------  ----  --------  ----------------------  ---------------------------
0-7     8     char[8]   magic                   "NGGUARD1" (ASCII)
8-9     2     u16       version_major           Major version (1)
10-11   2     u16       version_minor           Minor version (0)
12-13   2     u16       version_patch           Patch version (0)
14-15   2     u16       endianness              0x0102 = little-endian
16-23   8     u64       timestamp               Save timestamp
24-31   8     u64       uptime_seconds          System uptime
32-35   4     u32       cdna_version            Current CDNA version
36-39   4     u32       cdna_history_count      Number of CDNA versions
40-43   4     u32       module_count            Number of registered modules
44-51   8     u64       event_history_count     Events in history
52-59   8     u64       cdna_history_offset     Offset to CDNA history
60-67   8     u64       module_registry_offset  Offset to module registry
68-75   8     u64       event_history_offset    Offset to event history
76-83   8     u64       metrics_offset          Offset to metrics
84-91   8     u64       total_file_size         Total file size
92-95   4     u32       checksum_type           0=none, 1=CRC32, 2=SHA256
96-127  32    bytes     checksum                File checksum
128-255 128   bytes     reserved                Reserved
------  ----  --------  ----------------------  ---------------------------
TOTAL:  256 bytes
```

### 10.3 Операции сериализации

**SaveSystemState(filepath: String):**

```rust
SaveSystemState(filepath: String) -> Result<()>:
    1. Create file and write header:
        file = File::create(filepath)?
        header = CreateHeader()
        file.write(&header)?
    
    2. Write current CDNA (128 bytes):
        cdna_bytes = constitution_keeper.cdna.as_bytes()
        file.write(&cdna_bytes)?
    
    3. Write CDNA history:
        history = constitution_keeper.cdna_history
        for version in history:
            SerializeCDNAVersion(&mut file, version)?
    
    4. Write module registry:
        for (module_id, module) in orchestrator.modules:
            SerializeModuleInfo(&mut file, module_id, module)?
    
    5. Write event history:
        for event in event_broker.event_history:
            SerializeEvent(&mut file, event)?
    
    6. Write metrics:
        metrics = CollectMetrics()
        SerializeMetrics(&mut file, metrics)?
    
    7. Compute and write checksum:
        checksum = ComputeChecksum(&file)
        file.seek_to_footer()
        file.write(&checksum)?
    
    8. Flush and close:
        file.flush()?
        file.close()
    
    9. return Ok()

Рекомендуемая частота:
    • Каждые N минут (N = 5-60)
    • После критичных изменений (CDNA change)
    • Перед shutdown
    • По запросу администратора
```

**LoadSystemState(filepath: String):**

```rust
LoadSystemState(filepath: String) -> Result<Guardian>:
    1. Open and validate file:
        file = File::open(filepath)?
        header = ReadHeader(&file)?
        ValidateHeader(&header)?
    
    2. Verify checksum:
        computed = ComputeChecksum(&file)
        stored = header.checksum
        if computed != stored:
            return Error("Checksum mismatch")
    
    3. Load CDNA:
        cdna_bytes = file.read_at(256, 128)?
        cdna = CDNA::from_bytes(cdna_bytes)?
    
    4. Load CDNA history:
        file.seek(header.cdna_history_offset)
        for i in 0..header.cdna_history_count:
            version = DeserializeCDNAVersion(&file)?
            cdna_history.push(version)
    
    5. Load module registry:
        file.seek(header.module_registry_offset)
        for i in 0..header.module_count:
            module_info = DeserializeModuleInfo(&file)?
            // Note: не восстанавливаем сами модули,
            // только метаданные для регистрации
    
    6. Load event history:
        file.seek(header.event_history_offset)
        for i in 0..header.event_history_count:
            event = DeserializeEvent(&file)?
            event_history.push(event)
    
    7. Load metrics:
        file.seek(header.metrics_offset)
        metrics = DeserializeMetrics(&file)?
    
    8. Reconstruct Guardian:
        guardian = Guardian::new()
        guardian.constitution_keeper.set_cdna(cdna)
        guardian.constitution_keeper.cdna_history = cdna_history
        guardian.event_broker.event_history = event_history
        // Модули нужно будет зарегистрировать заново
    
    9. return Ok(guardian)
```

### 10.4 Incremental Persistence

**Для больших систем — инкрементальное сохранение:**

```
Base snapshot:
    guardian_base_20251021_120000.gs

Change logs:
    guardian_changes_001.log
    guardian_changes_002.log
    ...

Change log format:
    [Timestamp: 8 bytes][OpType: 1 byte][Data: variable]

OpTypes:
    0x01: CDNA_CHANGED
    0x02: MODULE_REGISTERED
    0x03: MODULE_STATUS_CHANGED
    0x04: EVENT_LOGGED
    0x05: METRIC_UPDATED

Восстановление:
    1. Load base snapshot
    2. Apply all change logs in order
    3. Consolidate в новый base snapshot периодически
```

### 10.5 Backup and Restore

**Создание backup перед критичными операциями:**

```rust
CreateSystemBackup() -> Backup:
    1. Save current state to temporary file:
        temp_path = f"/tmp/guardian_backup_{timestamp}.gs"
        SaveSystemState(temp_path)?
    
    2. Create backup metadata:
        backup = Backup {
            backup_id: generate_uuid(),
            created_at: now(),
            filepath: temp_path,
            cdna_version: constitution_keeper.cdna_version,
            system_health: CheckHealth().overall_score,
        }
    
    3. return backup

RestoreFromBackup(backup: Backup) -> Result<()>:
    1. Load state from backup file:
        guardian = LoadSystemState(backup.filepath)?
    
    2. Verify integrity:
        health = guardian.CheckHealth()
        if health.overall_score < 0.5:
            return Error("Backup appears corrupted")
    
    3. Replace current Guardian state:
        self.constitution_keeper = guardian.constitution_keeper
        self.event_broker = guardian.event_broker
        // ... и т.д.
    
    4. Reinitialize modules with restored CDNA:
        for module_id in self.orchestrator.modules.keys():
            self.initialize_module(module_id)?
    
    5. Publish event:
        Publish(Event {
            event_type: SYSTEM_RESTORED_FROM_BACKUP,
            source: "Guardian",
            data: backup_info,
        })
    
    6. return Ok()
```

---

## 11. Безопасность и целостность

### 11.1 CDNA Integrity

**Защита CDNA от случайного/злонамеренного повреждения:**

**Checksum:**

```
При каждом изменении CDNA:
    1. Вычислить SHA-256 хеш всех 128 байт
    2. Сохранить в CDNAVersion.validation_hash
    3. При загрузке проверить:
        computed = SHA256(cdna_bytes)
        if computed != stored_hash:
            Panic("CDNA corrupted!")
```

**Write-once semantics:**

```
CDNA может быть изменена только через Guardian:
    • Прямая запись в память запрещена (immutable reference)
    • Изменение только через ConstitutionKeeper::ApplyProposal
    • Все изменения логируются и версионируются
```

**Quarantine для предложений:**

```
Перед применением CDNA proposal:
    1. Создать test instance системы с предложенной CDNA
    2. Запустить validation suite
    3. Мониторить health метрики в течение quarantine_duration
    4. Если всё OK — применить к production
    5. Иначе — отклонить proposal
```

### 11.2 Access Control

**Кто может что делать с Guardian:**

```rust
enum Permission {
    READ_CDNA,                          // Чтение CDNA
    PROPOSE_CDNA_CHANGE,                // Предложить изменение CDNA
    APPROVE_CDNA_CHANGE,                // Одобрить изменение
    REGISTER_MODULE,                    // Зарегистрировать модуль
    CONTROL_MODULE,                     // Управлять модулями (start/stop)
    SUBSCRIBE_EVENTS,                   // Подписаться на события
    PUBLISH_EVENTS,                     // Публиковать события
    VIEW_METRICS,                       // Смотреть метрики
    ADMIN,                              // Полный доступ
}

struct AccessControlList {
    rules: HashMap<Identity, Vec<Permission>>,
}

CheckPermission(identity: Identity, permission: Permission) -> bool:
    if identity.permissions.contains(ADMIN):
        return true
    
    return identity.permissions.contains(permission)
```

**Примеры:**

```
Module "Graph":
    • READ_CDNA: да
    • PROPOSE_CDNA_CHANGE: нет
    • SUBSCRIBE_EVENTS: да
    • PUBLISH_EVENTS: да
    • CONTROL_MODULE: нет (только себя)

Module "Intuition":
    • READ_CDNA: да
    • PROPOSE_CDNA_CHANGE: да (может предлагать изменения)
    • APPROVE_CDNA_CHANGE: нет
    • SUBSCRIBE_EVENTS: да
    • PUBLISH_EVENTS: да

Administrator:
    • ADMIN: да (все права)
```

### 11.3 Audit Log

**Полное логирование всех критичных операций:**

```rust
struct AuditLog {
    entries: Vec<AuditEntry>,
    storage: AuditStorage,              // File | Database | Remote
}

struct AuditEntry {
    entry_id: String,
    timestamp: uint64,
    actor: Identity,                    // Кто выполнил операцию
    operation: Operation,               // Что было сделано
    result: OperationResult,            // Успех/неудача
    context: HashMap<String, String>,   // Дополнительный контекст
    signature: Option<Signature>,       // Цифровая подпись (опционально)
}

LogOperation(actor: Identity, operation: Operation, result: OperationResult):
    entry = AuditEntry {
        entry_id: generate_uuid(),
        timestamp: now(),
        actor: actor,
        operation: operation,
        result: result,
        context: gather_context(),
        signature: None,
    }
    
    // Опционально: подписать запись
    if config.sign_audit_entries:
        entry.signature = Sign(entry, private_key)
    
    audit_log.append(entry)
    
    // Периодически flush на диск
    if audit_log.entries.len() >= FLUSH_THRESHOLD:
        audit_log.flush()
```

**Что логируется:**

```
Критичные операции:
    • Изменения CDNA
    • Регистрация/удаление модулей
    • Смена статуса модулей (особенно FAILED)
    • Нарушения валидации
    • Изменения ACL
    • System shutdown/restart

Для каждой операции:
    • Кто (actor)
    • Что (operation type + parameters)
    • Когда (timestamp)
    • Результат (success/failure + reason)
    • Контекст (system state, related events)
```

### 11.4 Tamper Detection

**Обнаружение несанкционированного изменения состояния:**

**Merkle Tree для state:**

```
Периодически вычислять Merkle Root всего состояния:
    1. Hash каждого компонента (CDNA, modules, events)
    2. Построить Merkle Tree
    3. Сохранить root hash

При следующей проверке:
    1. Пересчитать Merkle Root
    2. Сравнить с сохранённым
    3. Если не совпадает — state был изменён
    4. Использовать Merkle Proof для локализации изменения
```

**Integrity check:**

```
PeriodicIntegrityCheck():
    1. Compute current state hash:
        cdna_hash = SHA256(cdna)
        modules_hash = SHA256(serialize(modules))
        events_hash = SHA256(serialize(events))
        
        combined = cdna_hash || modules_hash || events_hash
        current_hash = SHA256(combined)
    
    2. Compare with stored hash:
        if current_hash != stored_integrity_hash:
            // State was tampered with
            LogCritical("Integrity violation detected!")
            
            // Identify what changed
            if cdna_hash != stored_cdna_hash:
                HandleCDNATampering()
            else if modules_hash != stored_modules_hash:
                HandleModuleTampering()
            else:
                HandleEventTampering()
            
            // Alert administrator
            AlertAdmin("CRITICAL: System state tampering detected")
            
            // Optionally: enter safe mode
            EnterSafeMode()
    
    3. Update stored hashes:
        stored_integrity_hash = current_hash
        stored_cdna_hash = cdna_hash
        stored_modules_hash = modules_hash
        stored_events_hash = events_hash
```

### 11.5 Safe Mode

**Защитный режим при обнаружении критичных проблем:**

```rust
EnterSafeMode():
    1. Stop accepting new operations:
        validator.reject_all_operations()
    
    2. Pause all non-critical modules:
        for module_id in orchestrator.modules:
            if not module.is_critical():
                PauseModule(module_id)
    
    3. Create emergency backup:
        backup = CreateSystemBackup()
        SaveToSafeLocation(backup)
    
    4. Run diagnostic:
        diagnostic = RunFullDiagnostic()
        LogDiagnostic(diagnostic)
    
    5. Wait for admin intervention:
        WaitForAdminCommand()
        // Admin может:
        // • Восстановить из backup
        // • Применить исправления
        // • Вручную исправить состояние
        // • Разрешить продолжение (если false alarm)
    
    6. Publish event:
        Publish(Event {
            event_type: SAFE_MODE_ENTERED,
            source: "Guardian",
            priority: CRITICAL,
        })
```

---

## 12. Конфигурация

### 12.1 Guardian Configuration File

**Пример конфигурации (YAML):**

```yaml
guardian:
  version: "1.0.0"
  
  # Constitution Keeper
  constitution:
    cdna_path: "config/cdna.bin"
    enable_versioning: true
    max_history_versions: 100
    proposal_quarantine_duration_seconds: 3600  # 1 hour
    allow_runtime_changes: true
    require_explicit_confirmation: true
  
  # Validator
  validation:
    mode: "strict"  # strict | permissive | custom
    cache_rules: true
    rule_cache_ttl_seconds: 300
    log_violations: true
    max_violations_before_alert: 100
  
  # Event Broker
  events:
    queue_type: "lock_free"  # lock_free | bounded | unbounded
    queue_capacity: 10000
    history_capacity: 10000
    history_ttl_seconds: 3600
    delivery_mode: "async"  # async | sync
    max_delivery_retries: 3
  
  # Orchestrator
  orchestration:
    module_startup_timeout_seconds: 30
    module_shutdown_timeout_seconds: 30
    health_check_interval_seconds: 10
    enable_auto_recovery: true
    max_recovery_attempts: 3
    recovery_backoff_base_seconds: 1
  
  # Optimizer
  optimization:
    enable_hot_cache: true
    hot_cache_size: 1000
    hot_cache_ttl_seconds: 300
    enable_luts: true
    lut_resolution: 1000
    profile_access: true
  
  # Health Monitoring
  health:
    degradation_threshold: 0.7
    failure_threshold: 0.3
    enable_graceful_degradation: true
    auto_recovery: true
  
  # Persistence
  persistence:
    enable_auto_save: true
    save_interval_seconds: 300
    save_path: "data/guardian_state.gs"
    enable_incremental: true
    incremental_log_path: "data/guardian_changes.log"
    consolidation_interval_hours: 24
  
  # Security
  security:
    enable_access_control: true
    enable_audit_log: true
    audit_log_path: "logs/guardian_audit.log"
    enable_integrity_checks: true
    integrity_check_interval_seconds: 60
    enable_tamper_detection: true
  
  # Performance
  performance:
    max_concurrent_validations: 1000
    max_event_batch_size: 100
    enable_profiling: true
    profiling_sample_rate: 0.01
  
  # Logging
  logging:
    level: "info"  # trace | debug | info | warn | error
    output: "file"  # file | stdout | both
    file_path: "logs/guardian.log"
    max_file_size_mb: 100
    max_files: 10

4. Check system health:
    health = CheckHealth()
    if health.overall_score < 0.5:
        LogWarning("Low health after wake, attempting recovery")
        AttemptAutoRecovery()

5. Apply pending CDNA changes (если были во время сна):
    if constitution_keeper.has_pending_proposals():
        ProcessPendingProposals()

6. Publish event:
    Publish(Event {
        event_type: SYSTEM_WAKE,
        source: "Guardian",
    })

7. Enter normal mode:
    lifecycle_stage = LifecycleStage::RUNNING
```



### 8.5 Update and Migration

**Обновление системы без полной перезагрузки:**

```rust
ApplyUpdate(update: SystemUpdate) -> Result<()>:
    1. Validate update:
        if not ValidateUpdate(update):
            return Error("Invalid update")
    
    2. Create backup:
        backup = CreateSystemBackup()
    
    3. Enter maintenance mode:
        lifecycle_stage = LifecycleStage::MAINTENANCE
        Publish(Event {
            event_type: MAINTENANCE_MODE_ENTERED,
        })
    
    4. Apply update components:
        for component in update.components:
            match component.type:
                CDNA_UPDATE:
                    ApplyCDNAUpdate(component)?
                
                MODULE_UPDATE:
                    UpdateModule(component)?
                
                CONFIGURATION_UPDATE:
                    UpdateConfiguration(component)?
                
                CODE_UPDATE:
                    // Hot-reload если возможно
                    HotReloadModule(component)?
    
    5. Verify update:
        if not VerifySystemIntegrity():
            LogError("Update verification failed, rolling back")
            RestoreFromBackup(backup)
            return Error("Update failed")
    
    6. Exit maintenance mode:
        lifecycle_stage = LifecycleStage::RUNNING
        Publish(Event {
            event_type: MAINTENANCE_MODE_EXITED,
        })
    
    7. return Ok()
````

---

## 9. Интеграция с модулями

### 9.1 Token v2.0 Integration

**Guardian использует Token через Grid:**

```rust
// Guardian не создаёт Token напрямую
// Он валидирует операции с Token

ValidateTokenOperation(operation: TokenOperation) -> ValidationResult:
    match operation.type:
        CREATE_TOKEN:
            token = operation.token
            
            // Validate weight range
            if not (min_token_weight <= token.weight <= max_token_weight):
                return Rejected("Token weight out of range")
            
            // Validate field properties
            if not (min_field_radius <= token.field_radius <= max_field_radius):
                return Rejected("Field radius out of range")
            
            if not (min_field_strength <= token.field_strength <= max_field_strength):
                return Rejected("Field strength out of range")
            
            // Check flags
            if token.weight == 0.0 and not token_flags.ALLOW_ZERO_WEIGHT:
                return Rejected("Zero weight tokens not allowed")
            
            if token.fsc_code.is_empty() and token_flags.REQUIRE_FSC:
                return Rejected("FSC code required")
            
            return Approved
        
        MIGRATE_TOKEN:
            if not token_flags.ALLOW_MIGRATION:
                return Rejected("Token migration not allowed")
            
            return Approved
```

**Event flow:**

```
Grid создаёт Token:
    1. Grid::CreateToken(token_data)
    2. Grid → Guardian: ValidateOperation(CREATE_TOKEN)
    3. Guardian: проверяет CDNA.token_base_properties
    4. Guardian → Grid: Approved/Rejected
    5. Если Approved:
        Grid.InsertNode(token)
        Grid → Guardian: Publish(NODE_ADDED event)
    6. Guardian распространяет событие подписчикам
```

### 9.2 Connection v1.0 Integration

**Guardian валидирует Connection и использует его для эволюции:**

```rust
ValidateConnectionOperation(operation: ConnectionOperation) -> ValidationResult:
    match operation.type:
        CREATE_CONNECTION:
            connection = operation.connection
            
            // Validate type is allowed
            type_bit = 1 << connection.connection_type
            if (allowed_connection_types & type_bit) == 0:
                return Rejected("Connection type not allowed")
            
            // Validate weight range
            if not (min_connection_weight <= connection.weight <= max_connection_weight):
                return Rejected("Connection weight out of range")
            
            // Check distance (if enforced)
            if enforce_distance_limit:
                from_token = Grid::GetNode(connection.from_token_id)
                to_token = Grid::GetNode(connection.to_token_id)
                distance = Grid::Distance(from_token, to_token, level=L8_ABSTRACT)
                
                if not (connection_distance_min <= distance <= connection_distance_max):
                    return Rejected("Connection distance out of range")
            
            // Check self-loops
            if connection.from_token_id == connection.to_token_id:
                if not topology_flags.ALLOW_SELF_LOOPS:
                    return Rejected("Self-loops not allowed")
            
            // Check multi-edges
            if not topology_flags.ALLOW_MULTI_EDGES:
                if Graph::ContainsEdge(connection.from_token_id, connection.to_token_id):
                    return Rejected("Multi-edges not allowed")
            
            return Approved
        
        UPDATE_CONNECTION:
            // Validate new weight is in range
            new_weight = operation.new_weight
            if not (min_connection_weight <= new_weight <= max_connection_weight):
                return Rejected("New weight out of range")
            
            return Approved
```

**Connection используется как "адаптивная ДНК":**

```
Connection.metadata содержит эволюционные параметры:
    {
        "mutation_rate": 0.05,
        "crossover_rate": 0.1,
        "selection_pressure": 0.7,
        ...
    }

Guardian валидирует изменения:
    Если Intuition предлагает mutation_rate = 0.15:
        1. Guardian проверяет:
            min_mutation_rate (0.01) <= 0.15 <= max_mutation_rate (0.2)
        2. Если OK, разрешает изменение
        3. Публикует событие CONNECTION_UPDATED
        4. Graph получает событие и обновляет поведение
```

### 9.3 Grid v2.0 Integration

**Guardian валидирует пространственные операции:**

```rust
ValidateGridOperation(operation: GridOperation) -> ValidationResult:
    match operation.type:
        INSERT_NODE:
            token = operation.token
            level = operation.level
            
            // Validate coordinates
            for i in 0..8:
                coord = token.coordinates[i]
                
                // Check dimension flags
                if dimension_flags[i].BOUNDED:
                    if coord < DIMENSION_MIN or coord > DIMENSION_MAX:
                        return Rejected("Coordinate out of bounds")
                
                if dimension_flags[i].DISCRETE:
                    if coord != floor(coord):
                        return Rejected("Coordinate must be discrete")
            
            return Approved
        
        UPDATE_COORDINATES:
            // Similar validation
            ...
```

**Пространственные эвристики для Guardian:**

```
Guardian может использовать Grid для:
    1. Проверки distance при создании Connection
    2. Spatial anomaly detection (токены в неожиданных местах)
    3. Визуализации health metrics в пространстве
```

### 9.4 Graph v2.0 Integration

**Guardian координирует топологические операции:**

```rust
ValidateGraphOperation(operation: GraphOperation) -> ValidationResult:
    match operation.type:
        ADD_EDGE:
            edge = operation.edge
            
            // Validate degree limit
            from_degree = Graph::GetDegree(edge.from_id, BOTH)
            if from_degree >= max_node_degree:
                return Rejected("Max degree exceeded for source node")
            
            to_degree = Graph::GetDegree(edge.to_id, BOTH)
            if to_degree >= max_node_degree:
                return Rejected("Max degree exceeded for target node")
            
            return Approved
        
        MUTATE_CONNECTIONS:
            mutation_rate = operation.mutation_rate
            
            // Validate mutation rate in allowed range
            if not (min_mutation_rate <= mutation_rate <= max_mutation_rate):
                return Rejected("Mutation rate out of range")
            
            // Check if mutations are allowed
            if not evolution_flags.ALLOW_MUTATIONS:
                return Rejected("Mutations not allowed by CDNA")
            
            return Approved
        
        APPLY_CROSSOVER:
            // Similar checks for crossover
            ...
        
        APPLY_SELECTION:
            // Similar checks for selection
            ...
```

**Топологические инварианты через Guardian:**

```
Guardian периодически проверяет Graph:
    1. CheckSystemInvariants() вызывает Graph.ValidateIntegrity()
    2. Graph проверяет:
        • Симметрию индексов
        • Консистентность рёбер
        • Соблюдение max_degree
    3. Если нарушения найдены:
        Guardian → Graph: RepairTopology()
        Guardian логирует нарушения
```

### 9.5 Future Module Integration

**Шаблон для интеграции новых модулей:**

```rust
// 1. Модуль реализует ModuleInterface
struct MyModule {
    guardian_ref: Arc<Guardian>,
    cdna_snapshot: CDNASnapshot,
}

impl ModuleInterface for MyModule {
    fn initialize(&mut self, cdna: &CDNA) -> Result<()> {
        // Получить snapshot CDNA
        self.cdna_snapshot = cdna.snapshot()
        
        // Подписаться на релевантные события
        self.guardian_ref.subscribe(Subscriber {
            subscriber_id: "my_module",
            module_name: "MyModule",
            event_types: vec![CDNA_CHANGED, NODE_ADDED],
            callback: Box::new(|event| self.handle_event(event)),
        })
        
        Ok(())
    }
    
    fn handle_event(&mut self, event: Event) {
        match event.event_type:
            CDNA_CHANGED:
                // Обновить локальный snapshot
                if "relevant_param" in event.data.changed_fields:
                    self.update_behavior(event.data.new_values)
            
            NODE_ADDED:
                // Обработать новый узел
                self.process_new_node(event.data)
    }
}

// 2. Модуль валидирует операции через Guardian
MyModule::PerformOperation(operation):
    // Сначала валидация
    result = self.guardian_ref.validate_operation(operation)
    
    if result == Approved:
        // Выполнить операцию
        self.execute(operation)
        
        // Опубликовать событие
        self.guardian_ref.publish(Event {
            event_type: MY_MODULE_OPERATION,
            source: "MyModule",
            data: operation_data,
        })
    else:
        // Логировать отказ
        LogWarning("Operation rejected: {result.reason}")
```

---

## 10. Сериализация и персистентность

### 10.1 Формат файла Guardian State

**Guardian сохраняет своё состояние для восстановления:**

```
╔═══════════════════════════════════════════════════════════╗
║              GUARDIAN STATE FILE FORMAT                   ║
╠═══════════════════════════════════════════════════════════╣
║                  HEADER (256 bytes)                       ║
╠═══════════════════════════════════════════════════════════╣
║             CURRENT CDNA (128 bytes)                      ║
╠═══════════════════════════════════════════════════════════╣
║             CDNA HISTORY (variable)                       ║
╠═══════════════════════════════════════════════════════════╣
║          MODULE REGISTRY (variable)                       ║
╠═══════════════════════════════════════════════════════════╣
║           EVENT HISTORY (variable)                        ║
╠═══════════════════════════════════════════════════════════╣
║            METRICS (variable)                             ║
╠═══════════════════════════════════════════════════════════╣
║           FOOTER (128 bytes)                              ║
╚═══════════════════════════════════════════════════════════╝
```

### 10.2 Guardian State Header

```
Offset  Size  Type      Field                   Description
------  ----  --------  ----------------------  ---------------------------
0-7     8     char[8]   magic                   "NGGUARD1" (ASCII)
8-9     2     u16       version_major           Major version (1)
10-11   2     u16       version_minor           Minor version (0)
12-13   2     u16       version_patch           Patch version (0)
14-15   2     u16       endianness              0x0102 = little-endian
16-23   8     u64       timestamp               Save timestamp
24-31   8     u64       uptime_seconds          System uptime
32-35   4     u32       cdna_version            Current CDNA version
36-39   4     u32       cdna_history_count      Number of CDNA versions
40-43   4     u32       module_count            Number of registered modules
44-51   8     u64       event_history_count     Events in history
52-59   8     u64       cdna_history_offset     Offset to CDNA history
60-67   8     u64       module_registry_offset  Offset to module registry
68-75   8     u64       event_history_offset    Offset to event history
76-83   8     u64       metrics_offset          Offset to metrics
84-91   8     u64       total_file_size         Total file size
92-95   4     u32       checksum_type           0=none, 1=CRC32, 2=SHA256
96-127  32    bytes     checksum                File checksum
128-255 128   bytes     reserved                Reserved
------  ----  --------  ----------------------  ---------------------------
TOTAL:  256 bytes
```

### 10.3 Операции сериализации

**SaveSystemState(filepath: String):**

```rust
SaveSystemState(filepath: String) -> Result<()>:
    1. Create file and write header:
        file = File::create(filepath)?
        header = CreateHeader()
        file.write(&header)?
    
    2. Write current CDNA (128 bytes):
        cdna_bytes = constitution_keeper.cdna.as_bytes()
        file.write(&cdna_bytes)?
    
    3. Write CDNA history:
        history = constitution_keeper.cdna_history
        for version in history:
            SerializeCDNAVersion(&mut file, version)?
    
    4. Write module registry:
        for (module_id, module) in orchestrator.modules:
            SerializeModuleInfo(&mut file, module_id, module)?
    
    5. Write event history:
        for event in event_broker.event_history:
            SerializeEvent(&mut file, event)?
    
    6. Write metrics:
        metrics = CollectMetrics()
        SerializeMetrics(&mut file, metrics)?
    
    7. Compute and write checksum:
        checksum = ComputeChecksum(&file)
        file.seek_to_footer()
        file.write(&checksum)?
    
    8. Flush and close:
        file.flush()?
        file.close()
    
    9. return Ok()

Рекомендуемая частота:
    • Каждые N минут (N = 5-60)
    • После критичных изменений (CDNA change)
    • Перед shutdown
    • По запросу администратора
```

**LoadSystemState(filepath: String):**

```rust
LoadSystemState(filepath: String) -> Result<Guardian>:
    1. Open and validate file:
        file = File::open(filepath)?
        header = ReadHeader(&file)?
        ValidateHeader(&header)?
    
    2. Verify checksum:
        computed = ComputeChecksum(&file)
        stored = header.checksum
        if computed != stored:
            return Error("Checksum mismatch")
    
    3. Load CDNA:
        cdna_bytes = file.read_at(256, 128)?
        cdna = CDNA::from_bytes(cdna_bytes)?
    
    4. Load CDNA history:
        file.seek(header.cdna_history_offset)
        for i in 0..header.cdna_history_count:
            version = DeserializeCDNAVersion(&file)?
            cdna_history.push(version)
    
    5. Load module registry:
        file.seek(header.module_registry_offset)
        for i in 0..header.module_count:
            module_info = DeserializeModuleInfo(&file)?
            // Note: не восстанавливаем сами модули,
            // только метаданные для регистрации
    
    6. Load event history:
        file.seek(header.event_history_offset)
        for i in 0..header.event_history_count:
            event = DeserializeEvent(&file)?
            event_history.push(event)
    
    7. Load metrics:
        file.seek(header.metrics_offset)
        metrics = DeserializeMetrics(&file)?
    
    8. Reconstruct Guardian:
        guardian = Guardian::new()
        guardian.constitution_keeper.set_cdna(cdna)
        guardian.constitution_keeper.cdna_history = cdna_history
        guardian.event_broker.event_history = event_history
        // Модули нужно будет зарегистрировать заново
    
    9. return Ok(guardian)
```

### 10.4 Incremental Persistence

**Для больших систем — инкрементальное сохранение:**

```
Base snapshot:
    guardian_base_20251021_120000.gs

Change logs:
    guardian_changes_001.log
    guardian_changes_002.log
    ...

Change log format:
    [Timestamp: 8 bytes][OpType: 1 byte][Data: variable]

OpTypes:
    0x01: CDNA_CHANGED
    0x02: MODULE_REGISTERED
    0x03: MODULE_STATUS_CHANGED
    0x04: EVENT_LOGGED
    0x05: METRIC_UPDATED

Восстановление:
    1. Load base snapshot
    2. Apply all change logs in order
    3. Consolidate в новый base snapshot периодически
```

### 10.5 Backup and Restore

**Создание backup перед критичными операциями:**

```rust
CreateSystemBackup() -> Backup:
    1. Save current state to temporary file:
        temp_path = f"/tmp/guardian_backup_{timestamp}.gs"
        SaveSystemState(temp_path)?
    
    2. Create backup metadata:
        backup = Backup {
            backup_id: generate_uuid(),
            created_at: now(),
            filepath: temp_path,
            cdna_version: constitution_keeper.cdna_version,
            system_health: CheckHealth().overall_score,
        }
    
    3. return backup

RestoreFromBackup(backup: Backup) -> Result<()>:
    1. Load state from backup file:
        guardian = LoadSystemState(backup.filepath)?
    
    2. Verify integrity:
        health = guardian.CheckHealth()
        if health.overall_score < 0.5:
            return Error("Backup appears corrupted")
    
    3. Replace current Guardian state:
        self.constitution_keeper = guardian.constitution_keeper
        self.event_broker = guardian.event_broker
        // ... и т.д.
    
    4. Reinitialize modules with restored CDNA:
        for module_id in self.orchestrator.modules.keys():
            self.initialize_module(module_id)?
    
    5. Publish event:
        Publish(Event {
            event_type: SYSTEM_RESTORED_FROM_BACKUP,
            source: "Guardian",
            data: backup_info,
        })
    
    6. return Ok()
```

---

## 11. Безопасность и целостность

### 11.1 CDNA Integrity

**Защита CDNA от случайного/злонамеренного повреждения:**

**Checksum:**

```
При каждом изменении CDNA:
    1. Вычислить SHA-256 хеш всех 128 байт
    2. Сохранить в CDNAVersion.validation_hash
    3. При загрузке проверить:
        computed = SHA256(cdna_bytes)
        if computed != stored_hash:
            Panic("CDNA corrupted!")
```

**Write-once semantics:**

```
CDNA может быть изменена только через Guardian:
    • Прямая запись в память запрещена (immutable reference)
    • Изменение только через ConstitutionKeeper::ApplyProposal
    • Все изменения логируются и версионируются
```

**Quarantine для предложений:**

```
Перед применением CDNA proposal:
    1. Создать test instance системы с предложенной CDNA
    2. Запустить validation suite
    3. Мониторить health метрики в течение quarantine_duration
    4. Если всё OK — применить к production
    5. Иначе — отклонить proposal
```

### 11.2 Access Control

**Кто может что делать с Guardian:**

```rust
enum Permission {
    READ_CDNA,                          // Чтение CDNA
    PROPOSE_CDNA_CHANGE,                // Предложить изменение CDNA
    APPROVE_CDNA_CHANGE,                // Одобрить изменение
    REGISTER_MODULE,                    // Зарегистрировать модуль
    CONTROL_MODULE,                     // Управлять модулями (start/stop)
    SUBSCRIBE_EVENTS,                   // Подписаться на события
    PUBLISH_EVENTS,                     // Публиковать события
    VIEW_METRICS,                       // Смотреть метрики
    ADMIN,                              // Полный доступ
}

struct AccessControlList {
    rules: HashMap<Identity, Vec<Permission>>,
}

CheckPermission(identity: Identity, permission: Permission) -> bool:
    if identity.permissions.contains(ADMIN):
        return true
    
    return identity.permissions.contains(permission)
```

**Примеры:**

```
Module "Graph":
    • READ_CDNA: да
    • PROPOSE_CDNA_CHANGE: нет
    • SUBSCRIBE_EVENTS: да
    • PUBLISH_EVENTS: да
    • CONTROL_MODULE: нет (только себя)

Module "Intuition":
    • READ_CDNA: да
    • PROPOSE_CDNA_CHANGE: да (может предлагать изменения)
    • APPROVE_CDNA_CHANGE: нет
    • SUBSCRIBE_EVENTS: да
    • PUBLISH_EVENTS: да

Administrator:
    • ADMIN: да (все права)
```

### 11.3 Audit Log

**Полное логирование всех критичных операций:**

```rust
struct AuditLog {
    entries: Vec<AuditEntry>,
    storage: AuditStorage,              // File | Database | Remote
}

struct AuditEntry {
    entry_id: String,
    timestamp: uint64,
    actor: Identity,                    // Кто выполнил операцию
    operation: Operation,               // Что было сделано
    result: OperationResult,            // Успех/неудача
    context: HashMap<String, String>,   // Дополнительный контекст
    signature: Option<Signature>,       // Цифровая подпись (опционально)
}

LogOperation(actor: Identity, operation: Operation, result: OperationResult):
    entry = AuditEntry {
        entry_id: generate_uuid(),
        timestamp: now(),
        actor: actor,
        operation: operation,
        result: result,
        context: gather_context(),
        signature: None,
    }
    
    // Опционально: подписать запись
    if config.sign_audit_entries:
        entry.signature = Sign(entry, private_key)
    
    audit_log.append(entry)
    
    // Периодически flush на диск
    if audit_log.entries.len() >= FLUSH_THRESHOLD:
        audit_log.flush()
```

**Что логируется:**

```
Критичные операции:
    • Изменения CDNA
    • Регистрация/удаление модулей
    • Смена статуса модулей (особенно FAILED)
    • Нарушения валидации
    • Изменения ACL
    • System shutdown/restart

Для каждой операции:
    • Кто (actor)
    • Что (operation type + parameters)
    • Когда (timestamp)
    • Результат (success/failure + reason)
    • Контекст (system state, related events)
```

### 11.4 Tamper Detection

**Обнаружение несанкционированного изменения состояния:**

**Merkle Tree для state:**

```
Периодически вычислять Merkle Root всего состояния:
    1. Hash каждого компонента (CDNA, modules, events)
    2. Построить Merkle Tree
    3. Сохранить root hash

При следующей проверке:
    1. Пересчитать Merkle Root
    2. Сравнить с сохранённым
    3. Если не совпадает — state был изменён
    4. Использовать Merkle Proof для локализации изменения
```

**Integrity check:**

```
PeriodicIntegrityCheck():
    1. Compute current state hash:
        cdna_hash = SHA256(cdna)
        modules_hash = SHA256(serialize(modules))
        events_hash = SHA256(serialize(events))
        
        combined = cdna_hash || modules_hash || events_hash
        current_hash = SHA256(combined)
    
    2. Compare with stored hash:
        if current_hash != stored_integrity_hash:
            // State was tampered with
            LogCritical("Integrity violation detected!")
            
            // Identify what changed
            if cdna_hash != stored_cdna_hash:
                HandleCDNATampering()
            else if modules_hash != stored_modules_hash:
                HandleModuleTampering()
            else:
                HandleEventTampering()
            
            // Alert administrator
            AlertAdmin("CRITICAL: System state tampering detected")
            
            // Optionally: enter safe mode
            EnterSafeMode()
    
    3. Update stored hashes:
        stored_integrity_hash = current_hash
        stored_cdna_hash = cdna_hash
        stored_modules_hash = modules_hash
        stored_events_hash = events_hash
```

### 11.5 Safe Mode

**Защитный режим при обнаружении критичных проблем:**

```rust
EnterSafeMode():
    1. Stop accepting new operations:
        validator.reject_all_operations()
    
    2. Pause all non-critical modules:
        for module_id in orchestrator.modules:
            if not module.is_critical():
                PauseModule(module_id)
    
    3. Create emergency backup:
        backup = CreateSystemBackup()
        SaveToSafeLocation(backup)
    
    4. Run diagnostic:
        diagnostic = RunFullDiagnostic()
        LogDiagnostic(diagnostic)
    
    5. Wait for admin intervention:
        WaitForAdminCommand()
        // Admin может:
        // • Восстановить из backup
        // • Применить исправления
        // • Вручную исправить состояние
        // • Разрешить продолжение (если false alarm)
    
    6. Publish event:
        Publish(Event {
            event_type: SAFE_MODE_ENTERED,
            source: "Guardian",
            priority: CRITICAL,
        })
```

---

## 12. Конфигурация

### 12.1 Guardian Configuration File

**Пример конфигурации (YAML):**

```yaml
guardian:
  version: "1.0.0"
  
  # Constitution Keeper
  constitution:
    cdna_path: "config/cdna.bin"
    enable_versioning: true
    max_history_versions: 100
    proposal_quarantine_duration_seconds: 3600  # 1 hour
    allow_runtime_changes: true
    require_explicit_confirmation: true
  
  # Validator
  validation:
    mode: "strict"  # strict | permissive | custom
    cache_rules: true
    rule_cache_ttl_seconds: 300
    log_violations: true
    max_violations_before_alert: 100
  
  # Event Broker
  events:
    queue_type: "lock_free"  # lock_free | bounded | unbounded
    queue_capacity: 10000
    history_capacity: 10000
    history_ttl_seconds: 3600
    delivery_mode: "async"  # async | sync
    max_delivery_retries: 3
  
  # Orchestrator
  orchestration:
    module_startup_timeout_seconds: 30
    module_shutdown_timeout_seconds: 30
    health_check_interval_seconds: 10
    enable_auto_recovery: true
    max_recovery_attempts: 3
    recovery_backoff_base_seconds: 1
  
  # Optimizer
  optimization:
    enable_hot_cache: true
    hot_cache_size: 1000
    hot_cache_ttl_seconds: 300
    enable_luts: true
    lut_resolution: 1000
    profile_access: true
  
  # Health Monitoring
  health:
    degradation_threshold: 0.7
    failure_threshold: 0.3
    enable_graceful_degradation: true
    auto_recovery: true
  
  # Persistence
  persistence:
    enable_auto_save: true
    save_interval_seconds: 300
    save_path: "data/guardian_state.gs"
    enable_incremental: true
    incremental_log_path: "data/guardian_changes.log"
    consolidation_interval_hours: 24
  
  # Security
  security:
    enable_access_control: true
    enable_audit_log: true
    audit_log_path: "logs/guardian_audit.log"
    enable_integrity_checks: true
    integrity_check_interval_seconds: 60
    enable_tamper_detection: true
  
  # Performance
  performance:
    max_concurrent_validations: 1000
    max_event_batch_size: 100
    enable_profiling: true
    profiling_sample_rate: 0.01
  
  # Logging
  logging:
    level: "info"  # trace | debug | info | warn | error
    output: "file"  # file | stdout | both
    file_path: "logs/guardian.log"
    max_file_size_mb: 100
    max_files: 10

```
### 12.2 Runtime Configuration Updates

**Изменение конфигурации без перезагрузки:**

```rust
UpdateConfiguration(config_changes: ConfigUpdate) -> Result<()>:
    1. Validate changes:
        if not ValidateConfigUpdate(config_changes):
            return Error("Invalid configuration update")
    
    2. Check if hot-reload is possible:
        for (key, value) in config_changes:
            if not IsHotReloadable(key):
                return Error("Parameter {key} requires system restart")
    
    3. Apply changes atomically:
        old_config = config.clone()
        
        for (key, value) in config_changes:
            match key:
                "health.health_check_interval_seconds":
                    health_monitor.set_interval(value)
                    config.health.health_check_interval_seconds = value
                
                "events.queue_capacity":
                    event_broker.resize_queue(value)
                    config.events.queue_capacity = value
                
                "optimization.hot_cache_size":
                    optimizer.resize_cache(value)
                    config.optimization.hot_cache_size = value
                
                // ... и т.д.
    
    4. Verify system stability:
        sleep(5000)  // Wait 5 seconds
        health = CheckHealth()
        
        if health.overall_score < 0.5:
            // Rollback
            RollbackConfiguration(old_config)
            return Error("Configuration caused system instability")
    
    5. Publish event:
        Publish(Event {
            event_type: CONFIGURATION_UPDATED,
            source: "Guardian",
            data: config_changes,
        })
    
    6. Save configuration:
        SaveConfiguration(config_path)
    
    7. return Ok()
```

**Hot-reloadable vs Restart-required параметры:**

```
Hot-reloadable (можно изменить в рантайме):
    • health_check_interval_seconds
    • save_interval_seconds
    • cache sizes и TTL
    • logging level
    • degradation thresholds

Restart-required (требуют перезагрузки):
    • cdna_path (новая CDNA)
    • queue_type (изменение типа очереди)
    • enable_access_control (смена security модели)
    • Структурные изменения
```

### 12.3 Configuration Profiles

**Предопределённые профили для разных режимов:**

```yaml
# profiles/development.yaml
guardian:
  validation:
    mode: "permissive"
    log_violations: true
  
  logging:
    level: "debug"
  
  persistence:
    save_interval_seconds: 60  # Частое сохранение для отладки
  
  performance:
    enable_profiling: true
    profiling_sample_rate: 0.1  # 10% sampling

# profiles/production.yaml
guardian:
  validation:
    mode: "strict"
    log_violations: true
  
  logging:
    level: "info"
  
  persistence:
    save_interval_seconds: 300
    enable_incremental: true
  
  security:
    enable_access_control: true
    enable_audit_log: true
    enable_integrity_checks: true
  
  performance:
    enable_profiling: false

# profiles/high_performance.yaml
guardian:
  optimization:
    enable_hot_cache: true
    hot_cache_size: 10000
    enable_luts: true
  
  events:
    queue_type: "lock_free"
    queue_capacity: 100000
  
  persistence:
    save_interval_seconds: 600  # Реже сохранять для производительности
  
  logging:
    level: "warn"  # Меньше логирования

# profiles/safe_mode.yaml
guardian:
  validation:
    mode: "strict"
  
  orchestration:
    enable_auto_recovery: false  # Требуется ручное вмешательство
  
  health:
    health_check_interval_seconds: 5  # Частая проверка
  
  persistence:
    save_interval_seconds: 30  # Частое сохранение
```

**Использование профилей:**

```rust
LoadProfile(profile_name: String) -> Result<Config>:
    profile_path = f"profiles/{profile_name}.yaml"
    
    if not file_exists(profile_path):
        return Error("Profile not found")
    
    config = LoadYAML(profile_path)?
    
    // Merge с базовой конфигурацией
    base_config = LoadYAML("config/guardian_base.yaml")?
    merged = MergeConfigs(base_config, config)
    
    return Ok(merged)
```

### 12.4 Environment-specific Configuration

**Переопределение через переменные окружения:**

```bash
# CDNA path
export NEUROGRAPH_CDNA_PATH="/custom/path/cdna.bin"

# Logging level
export NEUROGRAPH_LOG_LEVEL="debug"

# Health check interval
export NEUROGRAPH_HEALTH_CHECK_INTERVAL="5"

# Enable safe mode
export NEUROGRAPH_SAFE_MODE="true"
```

**Приоритет конфигурации:**

```
1. Environment variables (highest priority)
2. Command-line arguments
3. Profile-specific config
4. Base config file
5. Hardcoded defaults (lowest priority)
```

---

## Приложения

### A. Константы и параметры по умолчанию

```rust
// CDNA
const CDNA_SIZE: usize = 128;
const MAX_CDNA_HISTORY: usize = 100;
const PROPOSAL_QUARANTINE_DURATION: u64 = 3600;  // 1 hour

// Validation
const MAX_VIOLATIONS_BEFORE_ALERT: usize = 100;
const RULE_CACHE_TTL: u64 = 300;  // 5 minutes

// Events
const DEFAULT_EVENT_QUEUE_CAPACITY: usize = 10000;
const EVENT_HISTORY_CAPACITY: usize = 10000;
const EVENT_HISTORY_TTL: u64 = 3600;  // 1 hour
const MAX_EVENT_DELIVERY_RETRIES: u32 = 3;

// Health
const HEALTH_CHECK_INTERVAL: u64 = 10;  // 10 seconds
const DEGRADATION_THRESHOLD: f32 = 0.7;
const FAILURE_THRESHOLD: f32 = 0.3;
const MAX_RECOVERY_ATTEMPTS: u32 = 5;

// Optimization
const HOT_CACHE_SIZE: usize = 1000;
const HOT_CACHE_TTL: u64 = 300;  // 5 minutes
const LUT_RESOLUTION: usize = 1000;

// Persistence
const SAVE_INTERVAL: u64 = 300;  // 5 minutes
const CONSOLIDATION_INTERVAL: u64 = 86400;  // 24 hours
const BACKUP_RETENTION_DAYS: u32 = 7;

// Security
const INTEGRITY_CHECK_INTERVAL: u64 = 60;  // 1 minute
const AUDIT_LOG_FLUSH_THRESHOLD: usize = 100;

// Performance
const MAX_CONCURRENT_VALIDATIONS: usize = 1000;
const EVENT_BATCH_SIZE: usize = 100;
const PROFILING_SAMPLE_RATE: f32 = 0.01;  // 1%
```

### B. Error Types

```rust
enum GuardianError {
    // CDNA errors
    CDNACorrupted,
    CDNAVersionMismatch,
    CDNAProposalRejected(String),
    CDNALoadFailed(String),
    
    // Validation errors
    ValidationFailed(String),
    InvalidOperation(String),
    RuleViolation(String),
    
    // Module errors
    ModuleNotFound(String),
    ModuleInitializationFailed(String),
    ModuleStartupFailed(String),
    CircularDependency(Vec<String>),
    DependencyMissing(String),
    
    // Event errors
    EventPublishFailed(String),
    SubscriptionFailed(String),
    EventQueueOverflow,
    
    // Health errors
    SystemDegraded,
    ModuleUnhealthy(String),
    RecoveryFailed(String),
    
    // Security errors
    AccessDenied(String),
    IntegrityViolation(String),
    TamperDetected,
    
    // Persistence errors
    SaveFailed(String),
    LoadFailed(String),
    CorruptedState,
    
    // Configuration errors
    InvalidConfiguration(String),
    ConfigurationLoadFailed(String),
    
    // Generic errors
    Timeout(String),
    InternalError(String),
}
```

### C. API Summary

**Основные публичные методы Guardian:**

```rust
// Constitution Management
pub fn get_cdna(&self) -> &CDNA
pub fn propose_cdna_change(&mut self, proposal: CDNAProposal) -> Result<()>
pub fn approve_proposal(&mut self, proposal_id: &str) -> Result<()>
pub fn get_cdna_history(&self) -> &[CDNAVersion]

// Validation
pub fn validate_operation(&self, operation: &Operation) -> ValidationResult
pub fn check_invariants(&self) -> InvariantReport

// Event System
pub fn subscribe(&mut self, subscriber: Subscriber) -> Result<()>
pub fn unsubscribe(&mut self, subscriber_id: &str)
pub fn publish(&mut self, event: Event)
pub fn publish_sync(&mut self, event: Event)
pub fn get_event_history(&self, filter: EventFilter) -> Vec<Event>

// Module Management
pub fn register_module(&mut self, module: ModuleHandle) -> Result<()>
pub fn initialize_module(&mut self, module_id: &str) -> Result<()>
pub fn start_module(&mut self, module_id: &str) -> Result<()>
pub fn pause_module(&mut self, module_id: &str)
pub fn shutdown_module(&mut self, module_id: &str)

// Health Monitoring
pub fn check_health(&self) -> SystemHealth
pub fn check_module_health(&self, module_id: &str) -> ModuleHealth

// Lifecycle
pub fn initialize(config: SystemConfig) -> Result<Guardian>
pub fn shutdown(&mut self)
pub fn sleep(&mut self)
pub fn wake(&mut self)

// Persistence
pub fn save_state(&self, filepath: &str) -> Result<()>
pub fn load_state(filepath: &str) -> Result<Guardian>

// Configuration
pub fn get_config(&self) -> &GuardianConfig
pub fn update_config(&mut self, changes: ConfigUpdate) -> Result<()>

// Metrics
pub fn collect_metrics(&self) -> GuardianMetrics
```

### D. Integration Checklist

**Для интеграции нового модуля с Guardian:**

```
□ 1. Implement ModuleInterface
    □ initialize(&mut self, cdna: &CDNA)
    □ start(&mut self)
    □ pause(&mut self)
    □ shutdown(&mut self)
    □ health_check(&self) -> HealthStatus
    □ handle_event(&mut self, event: Event)

□ 2. Register module with Guardian
    □ Call guardian.register_module()
    □ Specify dependencies

□ 3. Subscribe to relevant events
    □ CDNA_CHANGED (если используются параметры CDNA)
    □ MODULE_STARTED/STOPPED (если зависит от других модулей)
    □ Domain-specific events

□ 4. Validate operations through Guardian
    □ Call guardian.validate_operation() перед операциями
    □ Handle validation failures gracefully

□ 5. Publish events for significant operations
    □ Module-specific events
    □ Include context in EventData

□ 6. Implement health checks
    □ Regular heartbeats
    □ Error rate tracking
    □ Performance metrics

□ 7. Handle degradation gracefully
    □ Respond to DEGRADATION_DETECTED events
    □ Reduce load when system unhealthy

□ 8. Test integration
    □ Unit tests with mock Guardian
    □ Integration tests with real Guardian
    □ Stress tests under load
```

### E. Troubleshooting Guide

**Частые проблемы и решения:**

**Problem: CDNA proposal rejected**

```
Причина: Предложенные изменения нарушают инварианты
Решение:
    1. Проверить logs для деталей нарушения
    2. Убедиться, что новые значения в допустимых диапазонах
    3. Проверить зависимости между параметрами
    4. Использовать quarantine для тестирования
```

**Problem: Event queue overflow**

```
Причина: События публикуются быстрее, чем обрабатываются
Решение:
    1. Увеличить queue_capacity в конфигурации
    2. Оптимизировать обработчики событий
    3. Включить batch processing
    4. Проверить, нет ли медленных подписчиков
```

**Problem: Module fails to start**

```
Причина: Dependency не готова или ошибка инициализации
Решение:
    1. Проверить dependency graph
    2. Убедиться, что зависимости стартовали
    3. Проверить logs модуля для деталей ошибки
    4. Проверить CDNA параметры, используемые модулем
```

**Problem: Low system health score**

```
Причина: Модули испытывают проблемы
Решение:
    1. Проверить health status каждого модуля
    2. Посмотреть error rate и response time
    3. Проверить resource usage (CPU, memory)
    4. Рассмотреть graceful degradation
```

**Problem: Integrity violation detected**

```
Причина: State был изменён без Guardian
Решение:
    1. Проверить audit log для подозрительных операций
    2. Восстановить из последнего backup
    3. Включить tamper detection если не включено
    4. Проверить access control
```

### F. Performance Tuning Guide

**Оптимизация производительности Guardian:**

**1. Reduce validation overhead:**

```
• Включить rule caching
• Увеличить cache TTL для стабильных правил
• Использовать batch validation для массовых операций
• Делегировать простую валидацию модулям
```

**2. Optimize event system:**

```
• Использовать async publishing (по умолчанию)
• Увеличить event_batch_size
• Фильтровать события на стороне publisher
• Использовать lock-free queue
```

**3. Reduce cache misses:**

```
• Предзагрузить часто используемые CDNA параметры
• Увеличить hot_cache_size
• Правильно настроить cache TTL
• Профилировать access patterns
```

**4. Minimize state saves:**

```
• Увеличить save_interval (но с осторожностью)
• Использовать incremental persistence
• Асинхронное сохранение в background thread
• Компрессия state files
```

**5. Optimize health checks:**

```
• Уменьшить health_check_interval для critical modules
• Увеличить для stable modules
• Использовать sampling для expensive checks
• Кэшировать результаты коротко живущих проверок
```

---

## Заключение

**Guardian Specification v1.0** определяет центральный модуль управления и оркестрации для NeuroGraph OS.

### Ключевые достижения

✅ **Хранитель Конституции** — управление CDNA с версионированием и защитой от повреждений  
✅ **Валидатор** — проверка операций на соответствие фундаментальным правилам  
✅ **Брокер Событий** — асинхронная Pub/Sub система для слабой связанности модулей  
✅ **Оркестратор** — координация жизненного цикла модулей с dependency management  
✅ **Оптимизатор** — кэширование, LUT, профилирование для производительности

### Философия Guardian

```
Guardian — это не диктатор, а конституционный суд
Он не управляет, а обеспечивает соблюдение законов
Модули автономны, Guardian — гарант целостности

CDNA — это истина системы, неизменяемая без подтверждения
Изменение CDNA — это референдум, а не простое решение
```

### Интеграция с экосистемой

Guardian связывает все модули NeuroGraph OS:

```
Token v2.0 — валидирует базовые свойства
Connection v1.0 — использует для адаптивных параметров  
Grid v2.0 — проверяет пространственные операции
Graph v2.0 — контролирует топологию

Все модули подписаны на события Guardian
Все операции валидируются через Guardian
Вся система координируется Guardian
```

### Производительность

Guardian спроектирован для минимального overhead:

- O(1) доступ к кэшированным CDNA параметрам
- Асинхронная публикация событий без блокировки
- Делегирование валидации для снижения нагрузки
- Lock-free структуры где возможно
- Batch обработка для массовых операций

### Надёжность

Множественные уровни защиты:

- Версионирование CDNA с возможностью отката
- Периодические integrity checks
- Audit log всех критичных операций
- Graceful degradation при проблемах
- Auto-recovery с exponential backoff
- Safe mode для критичных ситуаций

### Расширяемость

Guardian готов для будущего роста:

- Модульная архитектура с чёткими интерфейсами
- Простая интеграция новых модулей
- Hot-reloadable конфигурация
- Pluggable компоненты (validators, monitors)
- Horizontal scaling potential

---

**Guardian — это сердце NeuroGraph OS**, обеспечивающее, чтобы когнитивная архитектура оставалась **целостной, консистентной и здоровой** на всех этапах эволюции.

---

**Версия документа:** 1.0.0  
**Дата:** 2025-10-21  
**Статус:** Official Specification  
**Автор:** NeuroGraph OS Team  
**Лицензия:** MIT

**Зависимости:**

- Token Specification v2.0
- Connection Specification v1.0
- Grid Specification v2.0
- Graph Specification v2.0

**Совместимость:** Rust, C++, системные языки с поддержкой:

- Multi-threading
- Async/await
- Lock-free structures
- Event-driven architecture

---

🎯 **Guardian v1.0 Specification — Complete**```
