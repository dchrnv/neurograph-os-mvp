# NeuroGraph OS - Project History

**Полная история проекта с момента создания**

**Создан:** 2025-08 (начало проекта)
**Последнее обновление:** 2025-11-04
**Текущая версия:** v0.26.0

---

## Философия истории

- **Хронологический порядок:** От старых версий к новым
- **Краткое описание:** Что добавлено, зачем, результаты
- **Ключевые метрики:** Тесты, производительность, размер кодовой базы
- **Технические детали:** Ссылки на спецификации

---

## Хронология релизов

### v0.3 - Initial Release (2025, Q1)

**Первый коммит проекта:**
- Базовая структура Token
- 8 coordinate spaces концепция
- Graph и DNA начальная реализация
- Events система

**Технологии:** Python
**Статус:** Proof of concept

---

### v0.7.0 - CLI Implementation

**CLI интерфейс:**
- Командная строка для управления токенами
- Базовые операции (create, read, update, delete)
- Интерактивный режим

**Технологии:** Python CLI
**Статус:** Functional prototype

---

### v0.8

**Промежуточная версия:**
- Улучшения архитектуры
- Рефакторинг кода

**Статус:** Development

---

### v0.9 - WebSocket Server

**Real-time коммуникация:**
- WebSocket-сервер для real-time updates
- Event streaming
- Client-server архитектура

**Технологии:** Python WebSocket
**Статус:** Functional prototype

---

### v0.10.0 - HTTP API (FastAPI)

**RESTful API:**
- FastAPI HTTP сервер
- OpenAPI документация
- Health checks
- Token CRUD операции

**Технологии:** Python FastAPI
**API:** `http://localhost:8000`
**Docs:** `http://localhost:8000/docs`
**Статус:** MVP Release

---

### v0.11 - Token v2.0 (Python)

**Переработка Token структуры:**
- 64-byte структура
- 8 семантических пространств (L1-L8)
- Типобезопасные entity types
- Field properties (radius, strength)

**Технологии:** Python
**Статус:** MVP Complete

---

### v0.12.0 - Token V2.0 Rust

**Первая Rust реализация:**
- Полная Rust реализация Token V2.0
- В 100× быстрее Python версии
- Zero external dependencies
- 12+ unit тестов

**Технологии:** Rust 2021
**Performance:** 100× faster than Python
**Статус:** Production-ready core

**Ключевые файлы:**
- `src/core_rust/src/token.rs`
- `docs/specs/TOKEN_V2_RUST.md`

---

### v0.13.0 - Connection V1.0

**Система связей:**
- 40+ типов связей (11 категорий)
- 32-byte структура
- Модель физических сил (притяжение/отталкивание)
- 8-уровневая селективная активация
- 10+ unit тестов

**Технологии:** Rust
**Спецификация:** `docs/specs/CONNECTION_V1_RUST.md`

---

### v0.14.0 - FFI Integration

**Python биндинги:**
- PyO3 Rust-Python биндинги
- 10-100× ускорение для Python API
- Полная Python API обертка
- Zero-copy data transfer

**Технологии:** Rust + PyO3
**Документация:** `docs/FFI_INTEGRATION.md`

---

### v0.15.0 - Grid V2.0

**8D пространственная индексация:**
- Bucket-based быстрый поиск
- KNN поиск (K ближайших соседей)
- Range-запросы с влиянием поля
- Python FFI биндинги
- 6+ unit тестов

**Технологии:** Rust
**Спецификация:** `docs/specs/GRID_V2_RUST.md`

---

### v0.16.0 - Graph V2.0

**Топологическая навигация:**
- Adjacency lists для O(1) neighbor access
- BFS/DFS traversal с итераторами
- Shortest path (BFS) и weighted paths (Dijkstra)
- Subgraph extraction
- 10+ unit тестов

**Технологии:** Rust
**Спецификация:** `docs/specs/GRAPH_V2_RUST.md`

---

### v0.17.0 - Guardian & CDNA V2.1

**Конституционный слой:**
- Guardian V1.0 координатор
- CDNA V2.1 (384 bytes) - конституционный фреймворк
- Event system (3.5M events/sec)
- Profile система (Default, Explorer, Analyst, Creative)
- Evolution с rollback
- 70+ unit тестов

**Технологии:** Rust
**Performance:** 3.5M events/sec
**Спецификация:** `docs/specs/GUARDIAN_CDNA_RUST.md`

---

### v0.18.0 - CDNA Dashboard UI

**Web интерфейс:**
- React dashboard с glassmorphism дизайном
- CDNA configuration panel
- Real-time monitoring
- Token visualization

**Технологии:** React, TypeScript
**Статус:** Deprecated в v0.19 (редизайн)

---

### v0.19 - Hielo (Total Clean)

**Крупная очистка кодовой базы:**

**Удалено:**
- Все устаревшие Python модули (DNA, Events, Graph, Spatial)
- Старая persistence инфраструктура
- Устаревшие конфиги и спецификации
- UI v0.18 implementation

**Результат:**
- Чистая минималистичная кодовая база (832KB)
- 13 Python файлов
- Только актуальные Rust спецификации

**Фокус:** Активное Rust ядро + минимальный Python API
**Статус:** Clean slate for v2.0 UI

---

### v0.20.0 - Desktop UI Foundation

**Переход на native UI:**
- Начало работы над Desktop UI
- Core Rust fixes
- Подготовка к Iced framework

**Технологии:** Rust
**Статус:** Foundation

---

### v0.20.1 - Project Cleanup

**Документация и рефакторинг:**
- Обновлена структура проекта
- Чистая архитектура Rust core
- Подготовка к Desktop UI v2.0

**Статус:** Pre-release

---

### v0.21.0 - Desktop UI v2.0 (Cyberpunk Edition)

**Native Desktop UI на Iced 0.12:**

**Features:**
- 🎨 Киберпанк эстетика (неоновые цвета #00ffcc, #3399ff, #9966ff)
- 📐 Unity-style layout: левый Dock (80px) с ASCII иконками
- 🔐 Dual-mode система: User/Root режимы
- 🚀 6 Workspaces: Welcome, Chat, Settings, Status, Modules, Admin
- 📊 Система метрик (CPU, Memory, Temperature, Disk I/O, Network)
- ⚙️ Module Manager
- ⚡ Direct FFI интеграция с Rust core
- 🔒 Argon2id аутентификация

**Технологии:**
- Iced 0.12 (Rust native GUI)
- Elm Architecture (Model-View-Update)
- Direct Rust-to-Rust FFI (zero overhead)

**Спецификация:** `docs/specs/DESKTOP_UI_SPEC_V2.md`

---

### v0.22.0 - ExperienceStream v2.0

**Фундамент KEY архитектуры:**

**ExperienceStream v2.0:**
- 128-байтная структура `ExperienceEvent` (state, action, reward)
- Circular buffer (1M событий = 128 MB RAM)
- Real-time pub-sub система (tokio::broadcast)
- 4 стратегии семплирования (Uniform, PrioritizedReward, Recent, FilteredByType)
- Reward accumulation для Appraisers
- 11 unit тестов (100% coverage)

**Технологии:** Rust + Tokio (async)
**Dependencies:** tokio, uuid
**Спецификация:** `docs/specs/ExperienceStream_v2.0.md`

**Roadmap:** Phase 1-5 (13 planned releases)

---

### v0.23.0 - ADNA v1.0 MVP

**Static Policy Engine:**

**ADNA v1.0 MVP:**
- 256-byte структура статических политик
- 4 предустановленных профиля:
  - **Balanced** - равновесие всех параметров
  - **Cautious** - высокий homeostasis, низкий curiosity
  - **Curious** - высокий curiosity, низкий homeostasis
  - **Adaptive** - средние значения, высокий exploration
- Веса для Appraisers (homeostasis, curiosity, efficiency, goal)
- Параметры поведения (exploration_rate, decision_timeout, max_actions)
- Version tracking с FNV-1a hash
- Валидация параметров
- 10 unit тестов (100% coverage)

**Фундамент для:** ADNA v2.0+ (ML-assisted policies)

**Технологии:** Rust
**Спецификация:** `docs/specs/ADNA_v1.0_MVP.md`

---

### v0.24.0 - Guardian v1.1 (ADNA Integration)

**ADNA + Guardian Integration:**

**Guardian v1.1:**
- `load_adna()` - загрузка ADNA с валидацией через CDNA
- `update_adna_parameter()` - обновление параметров с версионированием
- `validate_adna_against_cdna()` - конституционная валидация
- `rollback_adna()` - откат к предыдущей версии
- ADNA history management (VecDeque)
- Generation tracking для ADNA эволюции

**Новые Event Types:**
- `ADNALoaded` (0x0011)
- `ADNAUpdated` (0x0012)
- `ADNARolledBack` (0x0013)

**Тесты:** 9 интеграционных тестов (100% coverage), 89 total tests passing

**Технологии:** Rust
**Спецификация:** `docs/specs/ADNA_v1.0_MVP.md` (Section: Guardian v1.1)

---

### v0.25.0 - 4 Appraisers (Текущая версия)

**Reward System для KEY Architecture:**

**Реализовано:**

1. **AppraisersManager** - координирует все оценщики
   - Trait-based architecture
   - Weighted sum calculation: `reward = Σ(component_i * weight_i)`

2. **HomeostasisAppraiser** (242 lines, 10 tests)
   - Квадратичный штраф за отклонение от целевых параметров
   - Cognitive Load target: [0.3, 0.7] (L4)
   - Certainty target: [0.5, 0.9] (L6)
   - Formula: `penalty = -k * deviation²`

3. **CuriosityAppraiser** (170 lines, 8 tests)
   - Линейная награда за новизну (L2 - novelty)
   - Formula: `reward = weight * novelty`

4. **EfficiencyAppraiser** (173 lines, 8 tests)
   - Линейный штраф за затраты энергии (L7 - energy)
   - Formula: `penalty = -weight * energy`

5. **GoalDirectedAppraiser** (186 lines, 9 tests)
   - Линейная награда за прогресс к цели (L8 - goal_progress)
   - Formula: `reward = weight * goal_progress`

**Архитектура:**
```rust
pub trait Appraiser: Send + Sync {
    fn calculate_reward(&self, event: &ExperienceEvent, adna: &ADNA) -> f32;
    fn name(&self) -> &str;
    fn weight(&self, adna: &ADNA) -> f32;
}
```

**Веса:** Все веса берутся из ADNA parameters
**Тесты:** 37 unit tests для Appraisers, 163 total tests passing
**Технологии:** Rust
**Спецификация:** `docs/specs/ADNA_v1.0_MVP.md` (Section: v0.25.0)

---

### v0.26.0 - Learner Module (Текущая версия)

**Hebbian Learning для KEY Architecture:**

**Реализовано:**

1. **Learner Module v1.0** (660 lines, 14 tests)
   - External weight storage: `HashMap<EdgeId, f32>`
   - Eligibility traces для temporal credit assignment
   - BCM thresholds для адаптивного обучения
   - Batch updates buffer

2. **3 Hebbian Rules:**
   - **Classic Hebbian**: `Δw = η * x * y` - базовое правило
   - **BCM (Bienenstock-Cooper-Munro)**: `Δw = η * x * y * (y - θ)` - стабильное обучение
   - **Oja's Rule**: `Δw = η * y * (x - w * y)` - нормализующее правило

3. **Learning Configuration:**
   - Base learning rate из ADNA parameters (0.001-0.1)
   - Per-connection type learning rates (например, Semantic = 1.2×, Motor = 0.8×)
   - Online + Batch dual learning modes
   - Trace decay для eligibility traces

4. **LearnerMetrics:**
   - Total updates count
   - Dead connections tracking (weight < 0.01)
   - Saturated connections tracking (weight > 0.99)
   - Weight variance monitoring

5. **ADNA Integration:**
   - Добавлен `learning_rate: f32` в ADNAParameters
   - Обновлены все 4 профиля (Balanced: 0.01, Cautious: 0.005, Curious: 0.02, Adaptive: 0.015)
   - Валидация range [0.001, 0.1]

6. **Connection v2.0 Planning:**
   - Задокументированы требования в CONNECTION_V1_RUST.md
   - Planned: 40-byte structure с embedded `hebbian_weight` и `learning_rate`
   - External storage как временное решение для v1.0

**Архитектура:**
```rust
pub struct Learner {
    weights: HashMap<EdgeId, f32>,
    eligibility_traces: HashMap<EdgeId, f32>,
    bcm_thresholds: HashMap<EdgeId, f32>,
    config: LearningConfig,
    metrics: LearnerMetrics,
    batch_updates: Vec<WeightUpdate>,
}
```

**Производительность:**
- HashMap overhead: ~24 bytes per connection (24MB для 1M connections)
- Per-event learning: O(k) где k = affected connections (~5-20)
- Target: 10K events/sec с <1ms learning overhead

**Тесты:** 14 new unit tests, 177 total tests passing
**Технологии:** Rust
**Спецификация:** `docs/specs/LEARNER_v1.0.md`

**Roadmap для v2.0:** Connection v2.0 integration, adaptive learning rates, meta-learning

---

## Статистика проекта

### По версиям

| Версия | Дата | Ключевая фича | Тесты | Технологии |
|--------|------|---------------|-------|------------|
| v0.3 | 2025 Q1 | Initial release | - | Python |
| v0.7.0 | 2025 Q1 | CLI | - | Python |
| v0.9 | 2025 Q1 | WebSocket | - | Python |
| v0.10.0 | 2025 Q1 | HTTP API | - | Python FastAPI |
| v0.11 | 2025 Q1 | Token v2.0 Python | - | Python |
| v0.12.0 | 2025 Q2 | Token v2.0 Rust | 12+ | Rust |
| v0.13.0 | 2025 Q2 | Connection v1.0 | 10+ | Rust |
| v0.14.0 | 2025 Q2 | FFI Integration | - | Rust + PyO3 |
| v0.15.0 | 2025 Q2 | Grid v2.0 | 6+ | Rust |
| v0.16.0 | 2025 Q2 | Graph v2.0 | 10+ | Rust |
| v0.17.0 | 2025 Q3 | Guardian + CDNA | 70+ | Rust |
| v0.18.0 | 2025 Q3 | Dashboard UI | - | React |
| v0.19 | 2025 Q3 | Hielo (Clean) | - | - |
| v0.20.0 | 2025 Q3 | Desktop UI Foundation | - | Rust |
| v0.20.1 | 2025 Q3 | Cleanup | - | - |
| v0.21.0 | 2025 Q3 | Desktop UI v2.0 | - | Iced 0.12 |
| v0.22.0 | 2025 Q4 | ExperienceStream | 11 | Rust + Tokio |
| v0.23.0 | 2025 Q4 | ADNA v1.0 MVP | 10 | Rust |
| v0.24.0 | 2025 Q4 | Guardian v1.1 | 89 | Rust |
| v0.25.0 | 2025 Q4 | 4 Appraisers | 163 | Rust |
| v0.26.0 | 2025 Q4 | Learner Module | 177 | Rust |

### Ключевые вехи

**Python Era (v0.3 - v0.11)**
- Proof of concept
- API development
- Core concepts

**Rust Migration (v0.12 - v0.17)**
- 100× performance improvement
- Type safety
- Zero dependencies core

**UI Development (v0.18 - v0.21)**
- Web UI (deprecated)
- Native Desktop UI (Iced)
- Cyberpunk design

**KEY Architecture (v0.22 - v0.26)**
- ExperienceStream (memory)
- ADNA (policies)
- Appraisers (rewards)
- Learner (Hebbian learning)
- Guardian integration

### Технологический стек (эволюция)

**Языки:**
- Python (v0.3 - v0.11): Прототипирование
- Rust 2021 (v0.12+): Production core
- TypeScript/React (v0.18): Web UI (deprecated)

**Frameworks:**
- FastAPI (v0.10): HTTP API
- PyO3 (v0.14): FFI bindings
- Iced 0.12 (v0.21): Native GUI
- Tokio (v0.22): Async runtime

**Architecture:**
- Монолит → Модульная
- Python → Rust core
- Web UI → Native UI
- Sync → Async (tokio)

---

## Метрики роста

### Тесты

```
v0.12:  12 tests  (Token)
v0.13:  22 tests  (+Connection)
v0.15:  28 tests  (+Grid)
v0.16:  38 tests  (+Graph)
v0.17: 108 tests  (+Guardian/CDNA)
v0.22: 119 tests  (+ExperienceStream)
v0.23: 129 tests  (+ADNA)
v0.24: 138 tests  (+Guardian v1.1)
v0.25: 163 tests  (+Appraisers)
v0.26: 177 tests  (+Learner Module)  ← Current
```

### Производительность

**Token operations:**
- Python (v0.11): ~10K ops/sec
- Rust (v0.12): ~1M ops/sec (100× faster)

**Event throughput:**
- Guardian (v0.17): 3.5M events/sec

**Memory efficiency:**
- ExperienceStream (v0.22): 128 MB для 1M событий

### Размер кодовой базы

**Hielo cleanup (v0.19):**
- До: ~5MB (много legacy Python)
- После: 832KB (13 Python files)
- Rust core: Стабильный рост

---

## Референсы

### Документация

**Общая:**
- `README.md` - Текущее состояние проекта
- `architecture_blueprint.json` - Архитектура системы
- `project-reference-map.md` - Референс-карта v2.0

**Планирование:**
- `docs/specs/arch/PROJECT_ROADMAP.md` - Стратегический план проекта
- `docs/specs/arch/ROADMAP.md` - Детальный план текущего модуля

**История:**
- `docs/specs/PROJECT_HISTORY.md` - Этот файл
- `docs/specs/ADNA_v1.0_MVP.md` - Детальная история ADNA модуля

**Спецификации:**
- `docs/specs/TOKEN_V2_RUST.md`
- `docs/specs/CONNECTION_V1_RUST.md`
- `docs/specs/GRID_V2_RUST.md`
- `docs/specs/GRAPH_V2_RUST.md`
- `docs/specs/GUARDIAN_CDNA_RUST.md`
- `docs/specs/DESKTOP_UI_SPEC_V2.md`
- `docs/specs/ExperienceStream_v2.0.md`
- `docs/specs/ADNA_v1.0_MVP.md`
- `docs/specs/LEARNER_v1.0.md`

---

## Заключение

**NeuroGraph OS прошел путь от Python прототипа к производительному Rust-based когнитивному фреймворку.**

### Достижения

✅ **Производительность:** 100× ускорение через Rust
✅ **Архитектура:** Модульная, типобезопасная система
✅ **Качество:** 177 unit + integration тестов
✅ **UI:** Native desktop интерфейс (Iced)
✅ **KEY Architecture:** ExperienceStream + ADNA + Appraisers + Learner

### Следующий шаг

**v0.27.0 - Attention Module (Salience)**
- Salience calculation для token selection
- Focus mechanism для prioritization
- Integration с Learner и Appraisers

См. `docs/specs/arch/PROJECT_ROADMAP.md` для долгосрочного плана.

---

**Последнее обновление:** 2025-11-04
**Текущая версия:** v0.26.0 - Learner Module (Hebbian Learning)
**Следующая версия:** v0.27.0 - Attention Module (Salience)
**Статус проекта:** Active Development - KEY Architecture Phase

---

*"От прототипа к production. От Python к Rust. От концепции к когнитивной архитектуре."*
