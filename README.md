# NeuroGraph OS

> **Высокопроизводительная система пространственных вычислений на основе токенов на Rust**

[![Version](https://img.shields.io/badge/version-v0.21.0-blue.svg)](https://github.com/dchrnv/neurograph-os)
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
# User password: "user123"
# Root password: "root123"
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
│   │   │   ├── token.rs         # Token V2.0
│   │   │   ├── connection.rs    # Connection V1.0
│   │   │   ├── grid.rs          # Grid V2.0
│   │   │   ├── graph.rs         # Graph V2.0
│   │   │   ├── cdna.rs          # CDNA V2.1
│   │   │   └── guardian.rs      # Guardian V1.0
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
│       └── DESKTOP_UI_SPEC_V2.md  # Desktop UI спецификация
│
├── requirements.txt        # Минимальные зависимости
└── README.md               # Этот файл
```

---

## История версий

### v0.21.0 - Desktop UI v2.0 (Cyberpunk Edition) (Текущая)

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

### v0.20.1 - Project Cleanup

**Документация и рефакторинг:**
- Обновлена структура проекта
- Чистая архитектура Rust core
- Подготовка к Desktop UI v2.0

### Hielo - Total Clean (v0.19)

**Крупная очистка и рефакторинг:**
- Удалены все устаревшие Python модули (DNA, Events, Graph, Spatial)
- Удалена старая инфраструктура и слои персистентности
- Удалены устаревшие конфиги и спецификации
- Очищена реализация UI v0.18
- **Результат**: Чистая, минималистичная кодовая база (832KB, 13 Python файлов)
- Остались только актуальные спецификации Rust модулей
- **Фокус**: Активное Rust ядро + минимальный Python API

### v0.18.0 - CDNA Dashboard UI

- React дашборд с glassmorphism дизайном
- Панель конфигурации CDNA
- *(Удалено в v0.19 для редизайна)*

### v0.17.0 - Guardian & CDNA

- Guardian V1.0 координатор
- CDNA V2.1 конституционный фреймворк (384 байта)
- Система событий (3.5M событий/сек)
- Система профилей с эволюцией
- 70+ unit тестов

### v0.16.0 - Graph V2.0

- Топологическая навигация
- BFS/DFS обход
- Алгоритмы поиска путей
- Извлечение подграфов
- 10+ unit тестов

### v0.15.0 - Grid V2.0

- 8D пространственная индексация
- KNN и range запросы
- Физика полей
- Python FFI биндинги
- 6+ unit тестов

### v0.14.0 - FFI Integration

- PyO3 Python биндинги
- Ускорение в 10-100×
- Полный Python API

### v0.13.0 - Connection V1.0

- 40+ типов связей
- Модель физических сил
- 10+ unit тестов

### v0.12.0 - Token V2.0 Rust

- Чистая Rust реализация
- В 100× быстрее Python
- Нулевые зависимости
- 12+ unit тестов

---

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

Сделано с ⚡ и 🦀
