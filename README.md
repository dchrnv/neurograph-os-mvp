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
- **Guardian & CDNA V2.1**: Конституционный слой с валидацией и эволюцией

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
# User password: "demo"
# Root password: "root"
```

**Возможности:**
- 🎨 Киберпанк UI с неоновыми акцентами
- 🔐 Dual-mode: User/Root аутентификация
- 📊 Real-time системные метрики
- 💬 Chat интерфейс для управления
- ⚙️ Module Manager для системных компонентов
- ⚡ Native performance (Iced 0.12 + Direct FFI)

### Rust Core

```bash
# Собрать и протестировать
cd src/core_rust
./setup_and_test.sh
```

---

## Rust Core модули

Вся основная функциональность реализована на чистом Rust **без внешних зависимостей**:

### Token V2.0 (64 байта)
- 8-мерная система семантических координат
- Типобезопасные типы сущностей и флаги
- Свойства поля (радиус, сила)
- Zero-copy сериализация

### Connection V1.0 (32 байта)
- 40+ типов связей (11 категорий)
- Модель физических сил (притяжение/отталкивание)
- 8-уровневая селективная активация
- Отслеживание жизненного цикла

### Grid V2.0
- 8-мерная пространственная индексация
- Bucket-based быстрый поиск
- KNN поиск (K ближайших соседей)
- Range-запросы с влиянием поля

### Graph V2.0
- Списки смежности для O(1) доступа к соседям
- BFS/DFS обход с итераторами
- Кратчайший путь (BFS) и взвешенные пути (Dijkstra)
- Извлечение подграфов

### Guardian & CDNA V2.1
- 384-байтный конституционный фреймворк
- Система событий (3.5M событий/сек)
- Валидация Token и Connection
- Система профилей (Default, Explorer, Analyst, Creative)
- Эволюция CDNA с откатом

### ExperienceStream v2.0
- 128-байтная структура событий (ExperienceEvent)
- Circular buffer (1M событий = 128 MB в памяти)
- Real-time pub-sub система (tokio::broadcast)
- Sampling strategies (Uniform, PrioritizedReward, Recent)
- Reward accumulation для Appraisers
- Фундамент для KEY архитектуры

### ADNA v1.0 MVP
- 256-байтная структура статических политик
- 4 профиля (Balanced, Cautious, Curious, Adaptive)
- Веса для 4 Appraisers (Homeostasis, Curiosity, Efficiency, GoalDirected)
- Параметры поведения системы (exploration rate, learning rate, timeouts)
- Version tracking и валидация
- Фундамент для эволюции до ADNA v2.0/v3.0

### Learner Module v1.0 ✨ NEW
- Hebbian learning для connection weights: "Neurons that fire together, wire together"
- 3 learning rules: Classic, BCM (stable), Oja (normalizing)
- External weight storage (HashMap) - готовность к Connection v2.0
- Online + Batch learning modes
- Learning rate из ADNA parameters (адаптивная политика)
- Metrics tracking (dead/saturated connections, variance)
- 14 unit tests

**Производительность:**
- В 100× быстрее чем Python
- Zero-copy сериализация
- Cache-friendly упакованные структуры
- Нулевые внешние зависимости

---

## 8 семантических пространств

| Уровень | Название | Назначение | Примеры осей |
|---------|----------|------------|--------------|
| **L1** | Physical | 3D физическое пространство | X, Y, Z (метры) |
| **L2** | Sensory | Восприятие | Значимость, Валентность, Новизна |
| **L3** | Motor | Движение | Скорость, Ускорение, Угловая |
| **L4** | Emotional | VAD модель | Валентность, Возбуждение, Доминирование |
| **L5** | Cognitive | Обработка | Нагрузка, Абстракция, Уверенность |
| **L6** | Social | Взаимодействие | Дистанция, Статус, Принадлежность |
| **L7** | Temporal | Время | Смещение, Длительность, Частота |
| **L8** | Abstract | Семантика | Близость, Каузальность, Модальность |

---

## Структура проекта

```
neurograph-os/
├── src/
│   ├── core_rust/          # Полная Rust реализация
│   │   ├── src/
│   │   │   ├── token.rs              # Token V2.0
│   │   │   ├── connection.rs         # Connection V1.0
│   │   │   ├── grid.rs               # Grid V2.0
│   │   │   ├── graph.rs              # Graph V2.0
│   │   │   ├── cdna.rs               # CDNA V2.1
│   │   │   ├── guardian.rs           # Guardian V1.0
│   │   │   └── experience_stream.rs  # ExperienceStream v2.0 ✨ NEW
│   │   ├── tests/               # 100+ unit тестов
│   │   └── examples/            # Примеры использования
│   │
│   ├── desktop/            # Desktop UI v2.0 (Iced 0.12)
│   │   └── src/
│   │       ├── main.rs          # Entry point
│   │       ├── app.rs           # Главное приложение
│   │       ├── auth.rs          # Аутентификация (User/Root)
│   │       ├── core.rs          # FFI bridge
│   │       ├── theme.rs         # Cyberpunk палитра
│   │       ├── metrics.rs       # Визуализация метрик
│   │       └── workspaces/      # UI экраны
│   │
│   └── core/token/         # Минимальный Python Token V2.0
│       └── token_v2.py          # Только для MVP API
│
├── docs/                   # Документация
│   ├── FFI_INTEGRATION.md
│   └── specs/              # Спецификации модулей
│       ├── TOKEN_V2_RUST.md
│       ├── CONNECTION_V1_RUST.md
│       ├── GRID_V2_RUST.md
│       ├── GRAPH_V2_RUST.md
│       ├── GUARDIAN_CDNA_RUST.md
│       ├── DESKTOP_UI_SPEC_V2.md       # Desktop UI спецификация
│       ├── ExperienceStream_v2.0.md    # ExperienceStream спецификация ✨ NEW
│       └── ADNA_v1.0_MVP.md            # ADNA MVP спецификация ✨ NEW
│
├── requirements.txt        # Минимальные зависимости
└── README.md               # Этот файл
```

---

## Текущая версия

### v0.25.0 - 4 Appraisers (Reward System)

**Последний релиз KEY Architecture:**

- **AppraisersManager** - координация всех оценщиков
- **4 Appraisers:**
  - **HomeostasisAppraiser** - квадратичный штраф за отклонение (L4, L6)
  - **CuriosityAppraiser** - линейная награда за новизну (L2)
  - **EfficiencyAppraiser** - линейный штраф за энергозатраты (L7)
  - **GoalDirectedAppraiser** - линейная награда за прогресс (L8)
- Trait-based architecture для расширяемости
- Weighted reward: `reward = Σ(component_i * weight_i)`
- 163 unit + integration тестов

**Следующий шаг:** v0.26.0 - Learner Module (Hebbian Learning)

📜 **Полная история проекта:** [docs/specs/PROJECT_HISTORY.md](docs/specs/PROJECT_HISTORY.md) (v0.3 → v0.25.0)

---

## Roadmap

**Текущая фаза:** KEY Architecture Implementation

**Ближайшие релизы:**
- v0.26.0: Learner Module (Hebbian Learning)
- v0.27.0: Attention Module (Salience)
- v0.28.0: Policy Executor (ADNA → Actions)

**Долгосрочное видение (v1.0.0):**
- TypeScript биндинги (NAPI-RS)
- PostgreSQL persistence
- Production deployment
- >95% test coverage
- Full API documentation

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

| Категория | Технология |
|-----------|------------|
| **Ядро** | Rust 2021 (нулевые зависимости) |
| **Desktop UI** | Iced 0.12 (Rust native GUI) |
| **Аутентификация** | Argon2id password hashing |
| **Архитектура UI** | Elm Architecture (Model-View-Update) |
| **FFI** | Direct Rust-to-Rust (zero overhead) |
| **Хранение** | In-memory (PostgreSQL запланирован) |
| **Тестирование** | Rust test framework |

---

## Документация

**Архитектура:**

- [Architecture Blueprint](architecture_blueprint.json) - Архитектура системы
- [Project Reference Map](project-reference-map.md) - Референс-карта проекта v2.0

**Desktop UI:**

- [Desktop UI Spec v2.0](docs/specs/DESKTOP_UI_SPEC_V2.md) - Cyberpunk Edition спецификация
- [UI Control Panel v2](docs/specs/UI_Control_Panel_V2.md) - Дизайн-система
- [UI Windows System v2](docs/specs/UI_Windows_System_V2.md) - Оконная система

**Core модули:**

- [Token V2 Spec](docs/specs/TOKEN_V2_RUST.md) - Реализация Token
- [Connection V1 Spec](docs/specs/CONNECTION_V1_RUST.md) - Реализация Connection
- [Grid V2 Spec](docs/specs/GRID_V2_RUST.md) - Реализация Grid
- [Graph V2 Spec](docs/specs/GRAPH_V2_RUST.md) - Реализация Graph
- [Guardian & CDNA Spec](docs/specs/GUARDIAN_CDNA_RUST.md) - Конституционный слой

**Интеграция:**

- [FFI Integration](docs/FFI_INTEGRATION.md) - Python биндинги (v0.14)
- [Quick Start](QUICKSTART.md) - Руководство по началу работы
- [Contributing](CONTRIBUTING.md) - Руководство для разработчиков

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

**Автор**: Чернов Денис
**Email**: dreeftwood@gmail.com
**GitHub**: [dchrnv/neurograph-os](https://github.com/dchrnv/neurograph-os)

---

**NeuroGraph OS** - Экспериментальная когнитивная архитектура для пространственных вычислений на основе токенов

**NeuroGraph Team:**
- Denys Chernov - Lead Developer & Architect
- Claude (Anthropic AI) - AI Co-Developer & Design Partner

Сделано с ⚡ и 🦀
