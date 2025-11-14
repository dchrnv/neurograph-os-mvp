# NeuroGraph OS - История проекта

> Полная хронология развития проекта от v0.3 до текущей версии

---

## Текущая версия

### v0.21.0 - Desktop UI v2.0 
**Дата:** 2025-11-01
**Статус:** ✅ PRODUCTION

Native Desktop UI на Iced 0.12 с киберпанк эстетикой, dual-mode системой (User/Root), real-time метриками, chat интерфейсом и Module Manager. Direct FFI интеграция с Rust core.

---

## Отмененные эксперименты (wrong way)

### v0.30.0 - Pattern Detector ❌
K-means clustering для обнаружения паттернов в ExperienceStream

### v0.29.0 - Policy Executor ❌
ADNA-driven actions, condition matching, action queue

### v0.28.0 - Attention Module ❌
Salience-based token activation (L2/L6/L7), top-K selection

### v0.27.1 - Bugfix & Cleanup ❌
Desktop UI fixes, warning resolution, compilation errors

### v0.26.0 - Learner Module ❌
Hebbian learning (Classic, BCM, Oja rules), weight storage

### v0.25.0 - 4 Appraisers ❌
Reward system: Homeostasis, Curiosity, Efficiency, GoalDirected

### v0.24.0 - Guardian v1.1 (ADNA Integration) ❌
ADNA load/update/rollback, CDNA validation

### v0.23.0 - ADNA v1.0 MVP ❌
256-byte static policy engine, 4 profiles, Appraiser weights

### v0.22.0 - ExperienceStream v2.0 ❌
128-byte events, circular buffer, pub-sub, sampling, KEY foundation

**Причина отката:** Направление с ADNA и KEY архитектурой оказалось неправильным. Возврат к v0.21.0 для нового подхода.

---

## Стабильные релизы

### v0.20.1 - Project Cleanup
**Дата:** 2025-10-29

Обновлена структура проекта, чистая архитектура Rust core, подготовка к Desktop UI v2.0.

### v0.20.0 - Desktop UI Foundation
**Дата:** 2025-10-28

Auth + Navigation + CoreBridge demo, начало работы над native desktop UI.

### Hielo (v0.19) - Total Clean
**Дата:** 2025-10-27

**Крупная очистка и рефакторинг:**
- Удалены все устаревшие Python модули (DNA, Events, Graph, Spatial)
- Удалена старая инфраструктура и слои персистентности
- Чистая минималистичная кодовая база (832KB, 13 Python файлов)
- Фокус: Активное Rust ядро + минимальный Python API

### v0.18.0 - CDNA Dashboard UI
**Дата:** 2025-10-25

React дашборд с glassmorphism дизайном, панель конфигурации CDNA. *(Удалено в v0.19 для редизайна)*

### v0.17.0 - Guardian & CDNA V2.1
**Дата:** 2025-10-20

Guardian V1.0 координатор, CDNA V2.1 конституционный фреймворк (384 байта), система событий (3.5M событий/сек), профили с эволюцией, 70+ unit тестов.

### v0.16.0 - Graph V2.0
**Дата:** 2025-10-18

Топологическая навигация, BFS/DFS обход, алгоритмы поиска путей, извлечение подграфов, 10+ unit тестов.

### v0.15.0 - Grid V2.0
**Дата:** 2025-10-15

8D пространственная индексация, KNN и range запросы, физика полей, Python FFI биндинги, 6+ unit тестов.

### v0.14.0 - FFI Integration
**Дата:** 2025-10-12

PyO3 Python биндинги, ускорение в 10-100×, полный Python API.

### v0.13.0 - Connection V1.0
**Дата:** 2025-10-10

40+ типов связей в 11 категориях, модель физических сил (притяжение/отталкивание), 8-уровневая селективная активация, 10+ unit тестов.

### v0.12.0 - Token V2.0 Rust
**Дата:** 2025-10-08

Чистая Rust реализация Token V2.0 (64 bytes), 8D семантическое пространство, в 100× быстрее Python, нулевые зависимости, 12+ unit тестов.

---

## Ранние версии (Python-based MVP)

### v0.11.0 - Token v2.0 MVP
**Дата:** 2025-09

Python реализация Token v2.0 с 8D координатной системой, начало работы над Rust портом.

### v0.10.0 - HTTP API (FastAPI)
**Дата:** 2025-08

Clean MVP release с HTTP API на FastAPI, REST endpoints для базовых операций.

### v0.9.0 - WebSocket Server
**Дата:** 2025-07

WebSocket-сервер для real-time коммуникации с клиентами.

### v0.8.0 - WebSocket Implementation
**Дата:** 2025-06

Базовая реализация WebSocket протокола.

### v0.7.0 - CLI Implementation
**Дата:** 2025-05

Реализация CLI для управления системой через командную строку.

### v0.6.0 - Persistence Layer
**Дата:** 2025-04

Слой персистентности для сохранения состояния системы.

### v0.5.0 - Core Implementation
**Дата:** 2025-03

Реализованы базовые компоненты: token, coordinate system, graph, DNA, events.

### v0.4.0 - Architecture Foundation
**Дата:** 2025-02

Базовая архитектура и структура проекта.

### v0.3.0 - Initial Release
**Дата:** 2025-01

Первая публичная версия NeuroGraph OS. Начало проекта, базовые концепции token-based computing и 8D семантического пространства.

---

## Статистика

**Всего версий:** 30+
**Major milestones:**
- v0.3 - Initial release (Python)
- v0.12 - Rust rewrite begins
- v0.17 - Guardian & CDNA
- v0.21 - Desktop UI v2.0 (текущая)

**Отмененные эксперименты:** v0.22-v0.30 (ADNA/KEY архитектура)

**Языки:**
- v0.3 - v0.11: Python
- v0.12+: Rust (core) + Python (FFI wrappers)
- v0.20+: Pure Rust (core + Desktop UI)

---

## Ссылки

- [README.md](../../README.md) - Основная документация

- [Спецификации модулей](.) - Детальные спецификации компонентов
