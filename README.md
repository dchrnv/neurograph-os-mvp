# NeuroGraph OS

> **Высокопроизводительная система пространственных вычислений на основе токенов на Rust**

[![Version](https://img.shields.io/badge/version-v0.25.0-blue.svg)](https://github.com/dchrnv/neurograph-os)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

---

## Что такое NeuroGraph OS?

**NeuroGraph OS** — экспериментальная когнитивная архитектура, построенная на:

- **Token V2.0**: Атомарная 64-байтная единица информации с 8-мерным семантическим пространством
- **Connection V1.0**: 32-байтные типизированные связи с моделью физических сил
- **Grid V2.0**: 8-мерная пространственная индексация с KNN-поиском
- **Graph V2.0**: Топологическая навигация с BFS/DFS поиском путей
- **Guardian & CDNA V2.1**: Конституционный слой

**Основная философия**: Чистая, минималистичная, производительная Rust-реализация.

---

## Быстрый старт

### Desktop UI v2.0 (Cyberpunk Edition)

```bash
# Установить Rust (один раз)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Запустить Desktop UI
cd src/desktop
cargo run

# По умолчанию:
# User password: "user123"
# Root password: "root123"
```

**Возможности:**

- UI с неоновыми акцентами
- Dual-mode: User/Root аутентификация
- Real-time системные метрики
- Chat интерфейс для управления
- Module Manager для системных компонентов
- Native performance (Iced 0.12 + Direct FFI)

### Rust Core

```bash
# Собрать и протестировать
cd src/core_rust
./setup_and_test.sh
```

---

#### Rust Core модули

#### Token V2.0 (64 байта)

#### Connection V1.0 (32 байта)

#### Grid V2.0

#### Graph V2.0

#### Guardian & CDNA V2.1

#### ADNA v3.0 (256 байт) + ExperienceToken (128 байт)
- Policy Engine для reinforcement learning
- Versioned evolution с lineage tracking
- Gradient-based policy updates
- Приоритезированный Experience Replay
- Machine-friendly cache-aligned структуры

**Производительность:**

- В 100× быстрее чем Python
- Zero-copy сериализация
- Cache-friendly упакованные структуры
- Нулевые внешние зависимости

## История версий

### v0.25.0 - ActionController + E2E Integration (Текущая)

**Замыкание цикла восприятие-действие:**

- **ActionController v1.0**: Центральный диспетчер действий
  - Intent → ADNA Policy → Executor Selection → Action Execution
  - Epsilon-greedy exploration/exploitation (default: 10% exploration)
  - Timeout для выполнения действий (default: 30 секунд)
  - Полное логирование в ExperienceStream (action_started + action_finished events)
- **ActionExecutor trait**: Общий интерфейс для всех исполнителей
  - `execute()`: Асинхронное выполнение действия
  - `validate_params()`: Валидация параметров перед выполнением
  - `id()` и `description()`: Метаданные исполнителя
- **Базовые Executors**:
  - `NoOpExecutor`: Пустое действие (для тестирования)
  - `MessageSenderExecutor`: Отправка лог-сообщений с приоритетами
- **ADNA Integration**: Расширение ADNAReader
  - `get_action_policy()`: Получение политики для текущего состояния
  - State quantization (4 бина на измерение → 65,536 состояний)
  - Default policies для неизвестных состояний
- **ActionController Demo** (`action-controller-demo`):
  - 5 тестовых Intents с различными параметрами
  - Демонстрация exploration/exploitation
  - Parameter validation
  - Error handling
- **Полная документация**:
  - [ActionController_v1.0.md](docs/specs/ActionController_v1.0.md)

**Результат**: Полный E2E цикл: Perception → Appraisal → Learning → Action Selection → Execution → Feedback

### v0.24.0 - Learning Loop Integration

**Полный цикл обучения через опыт:**

- **IntuitionEngine v2.1**: Анализ паттернов и генерация предложений
  - **Статистический анализ (v1.0)**: Корреляция действий и вознаграждений
    - Квантизация 8D пространства состояний (4 бина на измерение = 65,536 состояний)
    - Агрегация action-reward по state bins
    - Статистическая значимость через упрощённый t-test
    - Генерация Proposals для улучшения ADNA политик
  - **SamplingStrategy**: 4 стратегии выборки опыта
    - `Uniform`: Равномерная случайная выборка
    - `PrioritizedByReward`: Приоритет высоким вознаграждениям
    - `RecencyWeighted`: Приоритет недавним событиям
    - `Mixed`: Комбинация reward + recency
  - **IntuitionConfig**: Настраиваемые параметры анализа
    - Интервал анализа, размер батча, минимальные пороги
    - Confidence threshold для proposal acceptance
  - **Pattern Detection**: Идентификация значимых корреляций
    - Минимальная разница вознаграждений, минимум сэмплов
    - Confidence scoring на основе variance и sample size
- **EvolutionManager v1.0**: Безопасная эволюция ADNA
  - **Validation Pipeline**: Многоступенчатая проверка Proposals
    - Confidence threshold: минимальная уверенность в изменении
    - Expected impact: минимальное ожидаемое улучшение
    - CDNA validation: соответствие конституционным правилам
    - Format validation: корректность структуры данных
  - **ADNAState**: In-memory хранилище политик
    - `HashMap<String, ActionPolicy>`: state_bin_id → policy mapping
    - Атомарное применение изменений через RwLock
    - Version tracking для rollback capability
  - **Audit Trail**: Полное логирование решений
    - ProposalAccepted / ProposalRejected events в ExperienceStream
    - Meta-learning feedback loop для самооптимизации
  - **Rate Limiting**: Контроль скорости изменений
    - Максимум proposals в секунду (default: 10/sec)
- **Learning Loop Demo** (`learning-loop-demo`):
  - Полная интеграция всех компонентов
  - 100 событий с 3 чёткими паттернами
  - Автоматическое обнаружение: action 100 > action 200 в state [0.5, ...]
  - Успешное обучение: 1 ADNA политика за 3 цикла анализа
  - Демонстрация: Events → Rewards → Analysis → Proposals → Validation → ADNA Updates
- **Новые структуры ADNA**:
  - `Proposal`: Предложение изменения политики (JSON Patch format)
    - UUID идентификатор, target entity, confidence, expected impact
  - `Intent`: Абстрактное высокоуровневое описание действия
  - `ActionPolicy`: Веса действий для принятия решений
    - `HashMap<u16, f64>`: action_type → weight mapping
  - `ExperienceBatch`: Batch событий для анализа
- **Расширение ExperienceStream**:
  - `sample_batch()`: Выборка событий по стратегии
  - Поддержка prioritized replay для обучения
- **Обновлённые зависимости**:
  - `serde = { version = "1.0", features = ["derive"] }` - сериализация
  - `serde_json = "1.0"` - JSON поддержка для Proposals
  - `uuid = { version = "1.0", features = ["v4"] }` - уникальные ID
  - `rand = "0.8"` - probabilistic sampling
  - `tokio = { version = "1.42", features = ["sync", "macros", "rt", "time"] }` - добавлен "time" feature
- **Полная документация**:
  - [IntuitionEngine_v2.1.md](docs/specs/IntuitionEngine_v2.1.md)

**Результат**: Полный замкнутый цикл обучения от сырого опыта до автоматического улучшения ADNA политик с конституционными гарантиями CDNA.

### v0.23.0 - Intuition Module v2.2

**Система оценки и вознаграждения:**

- **Intuition Module v2.2**: Полная реализация модуля интуиции
  - **L1-L8 Coordinate System**: 8-мерное семантическое пространство
    - `CoordinateIndex` enum: L1 Existence, L2 Novelty, L3 Velocity, L4 Attention, L5 Cognitive Load, L6 Certainty, L7 Valence, L8 Coherence
    - `CoordinateExt` trait: Типизированные геттеры для ExperienceEvent (100% тестовое покрытие)
  - **ADNA v3.0 → v3.1**: Расширение Policy Engine с параметрами апрейзеров
    - `HomeostasisParams`: Целевые диапазоны для L5/L6/L8 (cognitive_load, certainty, coherence)
    - `CuriosityParams`: Порог новизны (novelty_threshold) для L2
    - `EfficiencyParams`: Пороги ресурсов для L3/L5 (motor_threshold, cognitive_threshold)
    - `GoalDirectedParams`: Порог позитивной валентности для L7 (positive_valence_threshold)
    - `ADNAReader` trait: Async интерфейс для чтения параметров
    - `InMemoryADNAReader`: RwLock-based реализация с defaults
  - **4 Reward Appraisers** работающих параллельно (tokio async):
    - `HomeostasisAppraiser`: Штрафует отклонения L5/L6/L8 от целевых диапазонов
    - `CuriosityAppraiser`: Награждает за новизну (L2 > порога)
    - `EfficiencyAppraiser`: Штрафует расход ресурсов (L3 Velocity + L5 Cognitive Load)
    - `GoalDirectedAppraiser`: Награждает достижение целей (L7 Valence > порога)
  - **AppraiserSet**: Координатор для управления всеми апрейзерами
    - Запускает 4 параллельных задачи (tokio::spawn)
    - Graceful shutdown через wait_all()
  - **ExperienceStream v2.1**: Event-based память с pub-sub
    - 128-byte events в circular buffer
    - Lock-free rewards: каждый апрейзер пишет в dedicated slot
    - Broadcast channels для real-time delivery
    - Sequence numbers для отслеживания событий
  - **2 Demos**:
    - `experience-stream-demo`: Базовая функциональность ExperienceStream
    - `intuition-demo`: Полная интеграция (6 тестовых сценариев)
- **Новые зависимости**:
  - `async-trait = "0.1"` - async trait support
  - `thiserror = "1.0"` - error handling
  - `tokio = { version = "1.42", features = ["sync", "macros", "rt"] }` - async runtime
- **Полная документация**:
  - [IntuitionModule_v2.2_Implementation.md](docs/specs/IntuitionModule_v2.2_Implementation.md)
  - [ExperienceStream_v2.1.md](docs/specs/ExperienceStream_v2.1.md)

### v0.22.0 - ADNA v3.0 Policy Engine

**Reinforcement Learning ядро:**

- **ADNA v3.0** (256 байт): Policy Engine с градиентным обучением
  - 4 блока по 64 байта: Header, EvolutionMetrics, PolicyPointer, StateMapping
  - Versioned evolution с SHA256 lineage tracking
  - Policy types: Linear, Neural, TreeBased, Hybrid, Programmatic
  - Fitness score, confidence, exploration rate metrics
- **ExperienceToken** (128 байт): State-action-reward tuples для обучения
  - 4 блока по 32 байта для оптимального кэширования
  - Система флагов для приоритезированного replay (HIGH_VALUE, NOVEL, etc.)
  - Episode tracking с terminal/truncated маркерами
  - ADNA version hash для отслеживания политик
- **Policy Trait**: Универсальный интерфейс для различных типов политик
  - Gradient computation и application
  - Action validation с bounds checking
  - Serialization/deserialization поддержка
- **Полная документация** (русский): ADNA_V3_RUST_RU.md, ExperienceToken_RU.md
- Cache-aligned структуры для CPU оптимизации (32, 64, 128, 256 байт)

### v0.21.0 - Desktop UI v2.0

**Native Desktop UI на Iced 0.12:**

- Киберпанк эстетика (неоновые цвета #00ffcc, #3399ff, #9966ff)
- Unity-style layout: левый Dock (80px) с ASCII иконками `[≈] [◐] [⚙] [◉] [⬡] [!]`
- Dual-mode система: User/Root режимы с визуальным разделением
- 6 Workspaces: Welcome, Chat, Settings, Status, Modules, Admin
- Система метрик (CPU, Memory, Temperature, Disk I/O, Network)
- Module Manager для управления системными модулями
- Direct FFI интеграция с Rust core (низкая латентность)
- Аутентификация Argon2id для User/Root режимов
- Custom StyleSheet для всех компонентов

### Hielo - Total Clean (v0.19)

**Крупная очистка и рефакторинг:**

- Удалены все устаревшие Python модули (DNA, Events, Graph, Spatial)
- Удалена старая инфраструктура и слои персистентности
- Удалены устаревшие конфиги и спецификации
- Очищена реализация UI v0.18
- **Результат**: Чистая, минималистичная кодовая база (832KB, 13 Python файлов)
- Остались только актуальные спецификации Rust модулей
- **Фокус**: Активное Rust ядро + минимальный Python API

## Roadmap к v1.0.0

### Текущий статус (Hielo)

**Завершено:**

- Token V2.0: полная Rust реализация + Python FFI обертки
- Connection V1.0, Grid V2.0, Graph V2.0 - полное Rust ядро
- Guardian + CDNA V2.1 конституционный слой
- Комплексное покрытие тестами (100+ unit tests)
- Чистая архитектура кодовой базы

**Следующие шаги:**

### Следующее - Интеграция и эволюция (Запланировано)

- Python FFI биндинги для всех модулей (PyO3)
- Интеграция и оптимизация системы
- Продвинутые алгоритмы эволюции
- Новая спецификация и реализация UI

### v1.0.0 - Production (Видение)

- TypeScript биндинги (NAPI-RS)
- Слой персистентности PostgreSQL
- WebSocket обновления в реальном времени
- Production deployment
- CLI инструменты
- Полное покрытие тестами (>95% интеграционных)
- Профилирование производительности
- Production hardening
- Полная документация API

---

## Тестирование

```bash
# Rust тесты
cd src/core_rust
cargo test

# Запуск примеров
cargo run --example token_demo
cargo run --example graph_demo
```

---

## Технологии

| Категория                     | Технология                              |
| -------------------------------------- | ------------------------------------------------- |
| **Ядро**                     | Rust 2021 (нулевые зависимости) |
| **Desktop UI**                   | Iced 0.12 (Rust native GUI)                       |
| **Аутентификация** | Argon2id password hashing                         |
| **Архитектура UI**    | Elm Architecture (Model-View-Update)              |
| **FFI**                          | Direct Rust-to-Rust (zero overhead)               |
| **Хранение**             | In-memory (PostgreSQL запланирован)   |
| **Тестирование**     | Rust test framework                               |

---

## Документация

.../docs/specs

---

## Участие в разработке

1. Fork репозитория
2. Создать feature ветку: `git checkout -b feature/amazing-feature`
3. Commit изменений: `git commit -m 'Add amazing feature'`
4. Push в ветку: `git push origin feature/amazing-feature`
5. Открыть Pull Request

См. [CONTRIBUTING.md](CONTRIBUTING.md) для деталей.

---

## Лицензия

MIT License - см. [LICENSE](LICENSE)

---

## Контакты

**Автор**: Chernov Denys
**Email**: dreeftwood@gmail.com
**GitHub**: [dchrnv/neurograph-os](https://github.com/dchrnv/neurograph-os)-mvp

---

**NeuroGraph OS** - Экспериментальная когнитивная архитектура для пространственных вычислений на основе токенов

Сделано с ⚡ и 🦀
