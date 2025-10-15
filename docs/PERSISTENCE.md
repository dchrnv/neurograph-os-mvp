# NeuroGraph OS - Persistence Layer

Полная система персистентности для NeuroGraph OS с поддержкой PostgreSQL и Redis.

## 📋 Оглавление

- [Архитектура](#архитектура)
- [Быстрый старт](#быстрый-старт)
- [Конфигурация](#конфигурация)
- [Использование](#использование)
- [Миграции](#миграции)
- [Производительность](#производительность)

---

## 🏗️ Архитектура

### Компоненты

```
src/infrastructure/persistence/
├── models.py           # SQLAlchemy модели
├── database.py         # Менеджеры подключений
├── repositories.py     # Repository pattern
└── migrations/         # Alembic миграции
    ├── env.py
    └── versions/
```

### Схемы БД

- **tokens** - хранение токенов и пространственных индексов
- **graph** - графовые связи и снапшоты
- **experience** - события опыта и траектории

---

## 🚀 Быстрый старт

### 1. Установка зависимостей

```bash
pip install -r requirements-persistence.txt
```

### 2. Запуск БД (Docker)

```bash
# Базовые сервисы (PostgreSQL + Redis)
docker-compose -f docker-compose.db.yml up -d

# С инструментами управления (PgAdmin + Redis Commander)
docker-compose -f docker-compose.db.yml --profile tools up -d
```

### 3. Настройка окружения

Создайте `.env` файл:

```env
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_DB=neurograph
POSTGRES_USER=neurograph_user
POSTGRES_PASSWORD=your_password

REDIS_HOST=localhost
REDIS_PORT=6379
```

### 4. Запуск миграций

```bash
# Создать начальную миграцию
alembic revision --autogenerate -m "Initial schema"

# Применить миграции
alembic upgrade head
```

### 5. Тестирование

```bash
python examples/persistence_usage.py
```

---

## ⚙️ Конфигурация

### Database.yaml

```yaml
database:
  postgres:
    host: ${POSTGRES_HOST:localhost}
    port: ${POSTGRES_PORT:5432}
    database: ${POSTGRES_DB:neurograph}
    user: ${POSTGRES_USER:neurograph_user}
    password: ${POSTGRES_PASSWORD:changeme}
    
    pool:
      min_size: 5
      max_size: 20
      
  redis:
    host: ${REDIS_HOST:localhost}
    port: ${REDIS_PORT:6379}
    cache:
      default_ttl: 3600
```

### Переменные окружения

Поддерживается подстановка через `${VAR:default}` синтаксис.

---

## 💻 Использование

### Инициализация

```python
from src.infrastructure.config import ConfigLoader
from src.infrastructure.persistence.database import DatabaseFactory

# Загрузка конфигурации
config_loader = ConfigLoader()
db_config = config_loader.load('infrastructure/database.yaml')

# Создание менеджеров
db_manager = await DatabaseFactory.create_database_manager(
    db_config['database']
)
redis_manager = DatabaseFactory.create_redis_manager(
    db_config['database']
)
```

### Работа с токенами

```python
from src.infrastructure.persistence.repositories import RepositoryFactory

async with db_manager.session() as session:
    # Создание репозитория
    token_repo = RepositoryFactory.create_token_repository(
        session, redis_manager
    )
    
    # Создание токена
    token = TokenModel(
        id=uuid4(),
        binary_data=b'\x00' * 64,
        coord_x=[1.0] * 8,
        coord_y=[0.0] * 8,
        coord_z=[0.0] * 8,
        token_type='test'
    )
    token = await token_repo.create(token)
    
    # Получение токена (с кэшированием)
    token = await token_repo.get_by_id(token.id, use_cache=True)
    
    # Пространственный поиск
    tokens = await token_repo.find_in_region(
        min_coords=(0.0, 0.0, 0.0),
        max_coords=(10.0, 10.0, 10.0),
        level=0
    )
```

### Работа с графом

```python
# Создание репозитория графа
graph_repo = RepositoryFactory.create_graph_repository(
    session, redis_manager
)

# Создание связи
connection = await graph_repo.create_connection(
    source_id=token1.id,
    target_id=token2.id,
    connection_type='spatial',
    weight=0.8
)

# Получение соседей
neighbors = await graph_repo.get_neighbors(
    token1.id, 
    direction='outgoing'
)

# Статистика узла
degree = await graph_repo.get_degree(token1.id)
```

### Работа с опытом

```python
# Создание репозитория опыта
exp_repo = RepositoryFactory.create_experience_repository(
    session, redis_manager
)

# Создание события
event = await exp_repo.create_event(
    event_type='action_taken',
    timestamp=int(time.time() * 1000),
    state_before={'value': 0},
    state_after={'value': 1},
    reward=1.0
)

# Получение недавних событий
recent = await exp_repo.get_recent_events(count=10)
```

### Массовая вставка

```python
# Создание 1000 токенов
tokens = [create_token() for _ in range(1000)]
await token_repo.bulk_create(tokens)
```

---

## 🔄 Миграции

### Команды Alembic

```bash
# Создать миграцию
alembic revision --autogenerate -m "Add new field"

# Применить миграции
alembic upgrade head

# Откатить миграцию
alembic downgrade -1

# Показать текущую версию
alembic current

# История миграций
alembic history --verbose
```

### Структура миграции

```python
def upgrade():
    op.create_table(
        'my_table',
        sa.Column('id', sa.UUID(), primary_key=True),
        sa.Column('name', sa.String(100)),
        schema='tokens'
    )

def downgrade():
    op.drop_table('my_table', schema='tokens')
```

---

## ⚡ Производительность

### Оптимизации

1. **Connection Pooling** - пул соединений (5-20 коннектов)
2. **Redis Caching** - кэширование часто используемых данных
3. **Bulk Operations** - массовые операции (batch_size: 1000)
4. **Indexes** - индексы на координаты, флаги, временные метки
5. **Query Optimization** - оптимизированные SQL-запросы

### Метрики

- **Bulk Insert**: ~5000 tokens/sec
- **Read (cached)**: <1ms
- **Read (DB)**: 5-10ms
- **Spatial Query**: 10-50ms (зависит от региона)

### Мониторинг

```python
# Проверка здоровья
pg_health = await db_manager.health_check()
redis_health = redis_manager.health_check()

# Статистика пула соединений
pool_stats = db_manager.engine.pool.status()
```

---

## 🐳 Docker

### Управление сервисами

```bash
# Старт
docker-compose -f docker-compose.db.yml up -d

# Статус
docker-compose -f docker-compose.db.yml ps

# Логи
docker-compose -f docker-compose.db.yml logs -f postgres

# Остановка
docker-compose -f docker-compose.db.yml down

# Полная очистка (с данными)
docker-compose -f docker-compose.db.yml down -v
```

### Инструменты управления

- **PgAdmin**: http://localhost:5050
  - Email: admin@neurograph.local
  - Password: admin

- **Redis Commander**: http://localhost:8081

---

## 🔒 Безопасность

### Рекомендации

1. **Изменить пароли** в production
2. **Использовать SSL** для PostgreSQL
3. **Ограничить доступ** через firewall
4. **Регулярные бэкапы** БД
5. **Мониторинг** подозрительной активности

### Настройка SSL (PostgreSQL)

```yaml
postgres:
  ssl_mode: require
  ssl_cert: /path/to/cert.pem
  ssl_key: /path/to/key.pem
```

---

## 📊 Схема БД

### Tokens Schema

- `tokens.tokens` - основная таблица токенов
- `tokens.spatial_index` - пространственный индекс

### Graph Schema

- `graph.connections` - связи между токенами
- `graph.graph_snapshots` - снапшоты состояния графа

### Experience Schema

- `experience.experience_events` - события опыта
- `experience.experience_trajectories` - траектории событий

---

## 🐛 Troubleshooting

### Проблема: Не удается подключиться к БД

```bash
# Проверить, что контейнеры запущены
docker ps

# Проверить логи
docker logs neurograph_postgres

# Проверить сетевое подключение
telnet localhost 5432
```

### Проблема: Миграции не применяются

```bash
# Проверить версию
alembic current

# Пересоздать базу (ОСТОРОЖНО!)
alembic downgrade base
alembic upgrade head
```

### Проблема: Медленные запросы

1. Проверить наличие индексов
2. Использовать EXPLAIN ANALYZE
3. Включить кэширование Redis
4. Увеличить размер пула соединений

---

## 📚 Дополнительные ресурсы

- [SQLAlchemy Documentation](https://docs.sqlalchemy.org/)
- [Alembic Tutorial](https://alembic.sqlalchemy.org/en/latest/tutorial.html)
- [PostgreSQL Best Practices](https://wiki.postgresql.org/wiki/Don%27t_Do_This)
- [Redis Best Practices](https://redis.io/topics/memory-optimization)

---

## 🤝 Contribution

Для добавления новых функций персистентности:

1. Обновить модели в `models.py`
2. Создать миграцию: `alembic revision --autogenerate`
3. Добавить методы в соответствующий репозиторий
4. Написать тесты
5. Обновить документацию

---

**Version**: 0.3.0  
**Last Updated**: 2025-10-15