# NeuroGraph OS 🧠

Когнитивная операционная система с пространственным интеллектом и нейрографическими вычислениями.

## 🚀 Быстрый старт

### Предварительные требования

- Python 3.11+
- Redis (для кэширования)
- PostgreSQL (опционально, для продакшена)

### Установка

```bash
# Клонирование репозитория
git clone https://github.com/dchrnv/neurograph-os.git
cd neurograph-os

# Создание виртуального окружения
python -m venv .venv
source .venv/bin/activate  # Linux/Mac
# или
.venv\Scripts\activate     # Windows

# Установка зависимостей
pip install -r requirements/core.txt
pip install -r requirements/dev.txt  # для разработки
```

## Релиз v0.6 — Персистенс реализован

В версии 0.6 реализован слой персистенса с адаптерами для PostgreSQL и Redis.

Ключевые артефакты:

- Миграции Alembic: `alembic/`, конфигурация `alembic.ini`
- Реализация адаптеров и репозиториев: `src/infrastructure/persistence`
- Конфигурация: `config/infrastructure/persistence.yaml`

Быстрая инструкция по локальному запуску миграций (через docker-compose):

```bash
# Запуск Postgres через docker-compose (пример в deployments/docker/docker-compose.yaml)
docker compose -f deployments/docker/docker-compose.yaml up -d postgres

# Применение миграций alembic
alembic upgrade head
```

Если у вас нет локального `alembic` в PATH, используйте виртуальное окружение или `python -m alembic`.

Тестовые интеграционные тесты для persistence находятся в `tests/integration/`.

---

Для подробной архитектуры смотрите `architecture_blueprint.json` и документацию в `docs/`.
