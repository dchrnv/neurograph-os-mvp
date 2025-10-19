# План очистки документации

## ✅ ОСТАВЛЯЕМ (актуально для MVP и будущего)

### Ключевые документы:
1. **CONCEPT.md** - основная концепция (ОБНОВЛЁН)
2. **token_extended_spec.md** - спецификация Token v2.0 (АКТУАЛЬНО)
3. **configuration_structure.md** - структура конфигурации

### Технические гайды:
4. **GIT_GUIDE.md** - работа с git
5. **DEVELOPMENT_GUIDE.md** - гайд по разработке
6. **SETUP_GUIDE.md** - установка и настройка

---

## ❌ УДАЛЯЕМ (не актуально для MVP v0.10)

### Устаревшая архитектура:
- **ARCHITECTURE_OVERVIEW.md** - старое описание, заменено на CONCEPT.md
- **фрактально_смысловой_ии_каркас_архитектуры_v_0.md** - старая концепция (фрактальные точки), заменена на Token v2.0

### Модули не в MVP:
- **EXPERIENCE_STREAM.md** - отложено до v0.12
- **PERSISTENCE.md** - отложено до v0.11
- **cli.md, cli_docs_ru.md** - CLI удалён из MVP
- **websocket_docs.md** - WebSocket отложен до v0.12
- **rest_api_docs.md** - устарел, есть актуальный OpenAPI
- **event_readme.md** - Event система не используется в MVP

### Специфичные модули:
- **spatial_grid.md** - старая реализация сетки (не используется)
- **token_factory.md** - старая фабрика токенов (заменена на token_v2.py)
- **API_REFERENCE.md** - устарел (есть /docs от FastAPI)
- **TESTING_GUIDE.md** - пока нет комплексного тестирования
- **requirements.md** - непонятно зачем, есть requirements.txt

---

## 📝 Итоговая структура docs/

```
docs/
├── CONCEPT.md                      # ⭐ Основная концепция проекта
├── token_extended_spec.md          # ⭐ Спецификация Token v2.0
├── configuration_structure.md      # Структура конфигурации
├── GIT_GUIDE.md                    # Git workflow
├── DEVELOPMENT_GUIDE.md            # Гайд по разработке
├── SETUP_GUIDE.md                  # Установка
└── archive/                        # Архив старых документов
    ├── фрактально_смысловой_ии_каркас_архитектуры_v_0.md
    ├── ARCHITECTURE_OVERVIEW.md
    ├── EXPERIENCE_STREAM.md
    ├── PERSISTENCE.md
    ├── cli.md
    ├── websocket_docs.md
    └── ... (всё остальное)
```

---

## Причины удаления/архивирования:

1. **Фрактальная концепция** - была заменена на Token v2.0 с 8 семантическими пространствами
2. **Experience Stream** - хорошая идея, но отложена до v0.12 (сейчас in-memory)
3. **Persistence** - будет в v0.11 (PostgreSQL)
4. **CLI** - полностью удалён из MVP, только API
5. **WebSocket** - отложен до v0.12
6. **Старые API docs** - есть актуальные от FastAPI на /docs

---

## Что делать дальше:

1. Переместить устаревшие документы в `docs/archive/`
2. Оставить только актуальные 6 файлов
3. Обновить README.md с ссылками на актуальные docs
4. Создать MODULES_SPEC.md для v0.11+ (GraphEngine, CDNA и т.д.)

---

*Создано: 2025-01-19*
