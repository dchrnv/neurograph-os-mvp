# NeuroGraph Quick Index

Быстрая навигация по проекту.

## 🚀 Быстрый старт

```bash
# 1. Установка
./setup-dependencies.sh

# 2. Запуск
./start-all.sh

# 3. Открыть в браузере
# http://localhost:5173
```

## 📋 Основные команды

| Команда | Описание |
|---------|----------|
| `./show-config.sh` | Показать текущую конфигурацию |
| `./setup-dependencies.sh` | Установить все зависимости |
| `./start-all.sh` | Запустить frontend + backend |
| `./start-backend.sh` | Запустить только backend |
| `./start-frontend.sh` | Запустить только frontend |
| `./stop-all.sh` | Остановить все сервисы |

## 📖 Документация

| Файл | Что там |
|------|---------|
| [README.md](README.md) | Основная документация проекта |
| [CONFIGURATION.md](CONFIGURATION.md) | **Как настроить проект** |
| [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) | Структура файлов и компонентов |
| [SCRIPTS.md](SCRIPTS.md) | Описание всех скриптов |
| [QUICKSTART.md](QUICKSTART.md) | Быстрое начало работы |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Как помочь проекту |

## ⚙️ Конфигурация

| Файл | Назначение |
|------|------------|
| [.config.sh](.config.sh) | Главный конфиг (читай первым!) |
| [config/project.env](config/project.env) | Настройки проекта |
| [config/python.env](config/python.env) | Python настройки |
| [config/rust.env](config/rust.env) | Rust настройки |
| [config/versions.env](config/versions.env) | Версии зависимостей |
| `.env.local` | Локальные переопределения |

**Создать локальные настройки:**
```bash
cp .env.local.example .env.local
# Отредактировать .env.local
```

## 🏗️ Структура кода

```
src/
├── core_rust/      # Rust Core (производительность)
├── api/            # FastAPI Backend (REST + WebSocket)
├── web/            # React Frontend (UI)
├── gateway/        # Сенсорный слой
└── integration/    # Python-Rust интеграция
```

Подробнее: [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md)

## 🌐 Доступ

| Сервис | URL |
|--------|-----|
| Frontend | http://localhost:5173 |
| Backend API | http://localhost:8000 |
| API Docs | http://localhost:8000/docs |
| Prometheus | http://localhost:9090 |

## 🔧 Частые задачи

### Изменить порт backend

```bash
# Отредактировать config/project.env
BACKEND_PORT=9000
```

### Пересобрать Rust Core

```bash
cd src/core_rust
../../.venv/bin/maturin develop --features python-bindings --release
```

### Установить новый Python пакет

```bash
source .venv/bin/activate
pip install package-name
```

### Обновить версию проекта

```bash
# Отредактировать config/project.env
PROJECT_VERSION="0.64.0"
```

## 🧪 Тестирование

```bash
# Unit tests
pytest tests/unit/

# Integration tests
pytest tests/integration/

# All tests
pytest tests/
```

## 📊 Мониторинг

```bash
# Prometheus метрики
curl http://localhost:8000/metrics

# Health check
curl http://localhost:8000/api/health
```

## 🐛 Troubleshooting

### Backend не запускается

```bash
# Проверить логи
tail -f backend.log

# Проверить venv
./show-config.sh
```

### Frontend не запускается

```bash
# Проверить логи
tail -f frontend.log

# Переустановить зависимости
cd src/web
npm install
```

### Порт занят

```bash
# Убить процесс на порту 8000
lsof -ti:8000 | xargs kill -9

# Или использовать stop-all.sh
./stop-all.sh
```

## 📚 Расширенная документация

```
docs/
├── specs/          # Технические спецификации
├── guides/         # Руководства
├── changelogs/     # История изменений
└── archive/        # Архив старых версий
```

## 💡 Полезные ссылки

- **GitHub Issues**: Сообщить о проблеме
- **Telegram**: Обсуждение проекта
- **Документация API**: http://localhost:8000/docs
- **Jupyter Notebooks**: `notebooks/`

## 🎯 Следующие шаги

1. ✅ Установили зависимости → Прочитайте [CONFIGURATION.md](CONFIGURATION.md)
2. ✅ Запустили проект → Изучите [API Docs](http://localhost:8000/docs)
3. ✅ Поняли структуру → Смотрите [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md)
4. 🚀 Готовы разрабатывать → Читайте [CONTRIBUTING.md](CONTRIBUTING.md)

---

**Проблемы?** Смотрите [Troubleshooting](#-troubleshooting) или создайте Issue.
