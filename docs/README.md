# NeuroGraph OS - Документация

Добро пожаловать в документацию проекта **NeuroGraph OS**.

---

## 📚 Активная документация (MVP v0.10)

### Основные документы

1. **[CONCEPT.md](CONCEPT.md)** ⭐
   - Основная концепция проекта
   - Философия и принципы
   - Примеры использования
   - Долгосрочное видение

2. **[token_extended_spec.md](token_extended_spec.md)** ⭐
   - Полная спецификация Token v2.0
   - 8 семантических пространств
   - Бинарная структура (64 байта)
   - Python API и примеры

3. **[configuration_structure.md](configuration_structure.md)**
   - Структура конфигурации системы
   - Файлы конфигурации
   - Параметры и настройки

### Гайды по разработке

4. **[SETUP_GUIDE.md](SETUP_GUIDE.md)**
   - Установка и настройка
   - Требования к системе
   - Быстрый старт

5. **[DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)**
   - Структура проекта
   - Code style
   - Workflow разработки

6. **[GIT_GUIDE.md](GIT_GUIDE.md)**
   - Работа с git
   - Branching strategy
   - Commit conventions

### Вспомогательные

7. **[DOCS_CLEANUP_PLAN.md](DOCS_CLEANUP_PLAN.md)**
   - План очистки документации
   - Что оставлено, что удалено
   - Причины изменений

---

## 📦 Архивная документация

Старые документы перемещены в [archive/](archive/) и сохранены для истории:

- **фрактально_смысловой_ии_каркас_архитектуры_v_0.md** - старая концепция (фрактальные точки)
- **ARCHITECTURE_OVERVIEW.md** - устаревший обзор архитектуры
- **EXPERIENCE_STREAM.md** - Experience Stream (отложено до v0.12)
- **PERSISTENCE.md** - Persistence layer (отложено до v0.11)
- **cli.md, cli_docs_ru.md** - CLI (удалён из MVP)
- **websocket_docs.md** - WebSocket (отложен до v0.12)
- **rest_api_docs.md** - старая документация API
- **event_readme.md** - Event система (не используется в MVP)
- **spatial_grid.md** - старая реализация сетки
- **token_factory.md** - старая фабрика токенов
- **API_REFERENCE.md** - устаревший API reference
- **TESTING_GUIDE.md** - устаревший гайд по тестированию
- **requirements.md** - старые требования
- **CONCEPT_v1.md** - первая версия CONCEPT.md

---

## 🚀 Быстрые ссылки

### Для начинающих:
1. Прочитай [CONCEPT.md](CONCEPT.md) - пойми идею проекта
2. Прочитай [token_extended_spec.md](token_extended_spec.md) - пойми Token v2.0
3. Следуй [SETUP_GUIDE.md](SETUP_GUIDE.md) - установи и запусти

### Для разработчиков:
1. [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md) - как разрабатывать
2. [GIT_GUIDE.md](GIT_GUIDE.md) - как работать с git
3. [configuration_structure.md](configuration_structure.md) - как настраивать

### Для понимания архитектуры:
1. [CONCEPT.md](CONCEPT.md) - общая архитектура
2. [../architecture_blueprint.json](../architecture_blueprint.json) - техническая архитектура
3. [token_extended_spec.md](token_extended_spec.md) - детали Token v2.0

---

## 📖 Внешние ресурсы

- **[README.md](../README.md)** - Главная страница проекта
- **[CONTRIBUTING.md](../CONTRIBUTING.md)** - Как участвовать в разработке
- **[GITHUB_SETUP.md](../GITHUB_SETUP.md)** - Настройка GitHub репозитория
- **[QUICKSTART.md](../QUICKSTART.md)** - Быстрый старт за 30 секунд

---

## 🔄 История документации

**2025-01-19 - Большая очистка:**
- Создан новый [CONCEPT.md](CONCEPT.md) объединяющий все видения проекта
- Перемещена устаревшая документация в [archive/](archive/)
- Оставлены только актуальные для MVP документы
- Добавлен этот README.md для навигации

**2025-10-13 - Создание спецификации Token v2.0:**
- Создан [token_extended_spec.md](token_extended_spec.md)
- Описаны 8 семантических пространств

**2025-10-15 - Добавлены гайды:**
- [SETUP_GUIDE.md](SETUP_GUIDE.md)
- [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
- [GIT_GUIDE.md](GIT_GUIDE.md)

---

## ❓ Что читать дальше?

### Если ты хочешь:

**Понять концепцию проекта:**
→ [CONCEPT.md](CONCEPT.md)

**Разобраться в токенах:**
→ [token_extended_spec.md](token_extended_spec.md)

**Начать разработку:**
→ [SETUP_GUIDE.md](SETUP_GUIDE.md) → [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

**Настроить систему:**
→ [configuration_structure.md](configuration_structure.md)

**Посмотреть историю проекта:**
→ [archive/](archive/)

---

*Документация актуальна для версии: 0.15.0 mvp_Grid*
*Последнее обновление: 2025-10-25*

---

## 🦀 Rust Implementation

### Token V2.0 (v0.12.0)

8. **[Token V2 Rust Overview](../TOKEN_V2_RUST.md)** ⭐
   - Обзор Rust реализации Token V2.0
   - 64-байтная структура
   - 8-мерное пространство
   - Примеры и API

9. **[V0.12.0 Release Notes](../V0.12.0_RELEASE_NOTES.md)**
   - Что нового в v0.12.0
   - Технические детали
   - Roadmap

### Connection V1.0 (v0.13.0)

10. **[Connection V1 Rust Overview](../CONNECTION_V1_RUST.md)** ⭐
    - Обзор Rust реализации Connection V1.0
    - 32-байтная структура
    - 40+ типов связей
    - Физическая модель сил
    - Примеры и API

11. **[V0.13.0 Release Notes](V0.13.0_RELEASE_NOTES.md)**
    - Что нового в v0.13.0
    - Connection implementation
    - Технические детали

### FFI Integration (v0.14.0)

12. **[FFI Integration Guide](FFI_INTEGRATION.md)** ⭐
    - Python биндинги через PyO3
    - Полный Python API для Token & Connection
    - Установка и использование
    - Примеры и бенчмарки
    - 10-100x ускорение vs чистый Python

13. **[V0.14.0 Release Notes](V0.14.0_RELEASE_NOTES.md)**
    - Что нового в v0.14.0
    - PyO3 FFI реализация
    - Python wrapper и примеры
    - Бенчмарки производительности

### Grid V2.0 (v0.15.0 - NEW)

14. **[Grid V2 Rust Overview](../GRID_V2_RUST.md)** ⭐ NEW
    - Обзор Rust реализации Grid V2.0
    - 8-мерная пространственная индексация
    - Bucket-based поиск
    - KNN и Range запросы
    - Физика полей (влияние, плотность)
    - Python FFI биндинги
    - Примеры и API

15. **[V0.15.0 Release Notes](V0.15.0_RELEASE_NOTES.md)** ⭐ NEW
    - Что нового в v0.15.0
    - Grid V2.0 реализация
    - Пространственная индексация
    - Технические детали
    - Бенчмарки производительности

### Общее

16. **[Rust API README](../src/core_rust/README.md)**
    - Полная API документация
    - Token + Connection + Grid
    - Установка и использование

17. **[Installation Guide](../src/core_rust/INSTALL.md)**
    - Установка Rust
    - Сборка и тестирование
    - Troubleshooting
