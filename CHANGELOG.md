
# Changelog

Все заметные изменения в этом проекте документируются в этом файле.

Формат версии соответствует SemVer (MAJOR.MINOR.PATCH).

## [0.6.0] - 2025-10-15

### Added (0.6.0)

- Реализован слой персистенса: адаптеры для PostgreSQL и Redis
- Добавлены миграции Alembic (каталог `alembic/` и `alembic.ini`)
- Созданы репозитории и адаптеры в `src/infrastructure/persistence`
- Конфигурация персистенса: `config/infrastructure/persistence.yaml`

### Changed

- Обновлён архитектурный blueprint: `architecture_blueprint.json` (версия 0.6.0)

## [0.3.0] - 2025-10-13

### Added (0.3.0)

- Добавлены подсистемы DNA и Experience; интеграция потоков опыта


> Примечание: детальные действия по миграциям и запуску БД описаны в `README.md`.
