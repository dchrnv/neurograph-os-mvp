# PostgreSQL Persistence Setup

Quick guide для настройки PostgreSQL backend для NeuroGraph OS v0.26.0.

---

## Требования

- PostgreSQL 14+ (рекомендуется 15+)
- Rust toolchain с `sqlx-cli` (опционально, для миграций)

---

## Быстрый старт

### 1. Установка PostgreSQL

#### Ubuntu/Debian:
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

#### macOS (Homebrew):
```bash
brew install postgresql@15
brew services start postgresql@15
```

#### Arch Linux:
```bash
sudo pacman -S postgresql
sudo systemctl enable --now postgresql
```

### 2. Создание БД и пользователя

```bash
# Войти как postgres user
sudo -u postgres psql

# В psql:
CREATE DATABASE neurograph_db;
CREATE USER neurograph_user WITH ENCRYPTED PASSWORD 'your_secure_password';
GRANT ALL PRIVILEGES ON DATABASE neurograph_db TO neurograph_user;

# Выйти
\q
```

### 3. Применение схемы

```bash
cd src/core_rust

# Опция 1: Через psql
psql -U neurograph_user -d neurograph_db -f schema.sql

# Опция 2: Через sqlx (если установлен)
sqlx database create
sqlx migrate run
```

### 4. Проверка

```bash
psql -U neurograph_user -d neurograph_db

# В psql:
\dt  # Список таблиц
SELECT COUNT(*) FROM experience_events;  # Должно вернуть 0
SELECT COUNT(*) FROM action_metadata;     # Должно вернуть 0
```

---

## Connection String

Для Rust приложения используй connection string:

```bash
DATABASE_URL=postgres://neurograph_user:your_secure_password@localhost/neurograph_db
```

Сохрани в `.env` файл:

```bash
echo 'DATABASE_URL=postgres://neurograph_user:your_secure_password@localhost/neurograph_db' > .env
```

---

## Cargo.toml Dependencies

Добавь в `Cargo.toml`:

```toml
[dependencies]
# PostgreSQL async driver
tokio-postgres = "0.7"
deadpool-postgres = "0.10"

# Or use sqlx (более высокоуровневый)
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio", "macros", "uuid", "chrono", "json"] }

# For .env support
dotenv = "0.15"
```

---

## Опциональные настройки

### Performance Tuning

Для production отредактируй `postgresql.conf`:

```ini
# Memory
shared_buffers = 256MB          # 25% RAM для dedicated server
effective_cache_size = 1GB      # 50-75% RAM

# Connections
max_connections = 100

# WAL (Write-Ahead Log)
wal_buffers = 16MB
checkpoint_completion_target = 0.9

# Query planner
random_page_cost = 1.1          # SSD
effective_io_concurrency = 200  # SSD
```

### Backup Strategy

```bash
# Daily backup
pg_dump -U neurograph_user neurograph_db > backup_$(date +%Y%m%d).sql

# Restore
psql -U neurograph_user -d neurograph_db < backup_20250114.sql
```

---

## Retention Policy

Автоматическое архивирование старых событий (>7 дней):

```sql
-- Запуск вручную
SELECT archive_old_events(7);

-- Настройка через pg_cron (если установлен)
SELECT cron.schedule('archive-old-events', '0 2 * * *', $$SELECT archive_old_events(7)$$);
```

---

## Мониторинг

### Размер таблиц

```sql
SELECT
    schemaname,
    tablename,
    pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) AS size
FROM pg_tables
WHERE schemaname = 'public'
ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC;
```

### Top queries

```sql
SELECT
    calls,
    total_exec_time,
    mean_exec_time,
    query
FROM pg_stat_statements
ORDER BY total_exec_time DESC
LIMIT 10;
```

---

## Troubleshooting

### Проблема: Connection refused

```bash
# Проверь статус PostgreSQL
sudo systemctl status postgresql

# Проверь pg_hba.conf
sudo nano /etc/postgresql/15/main/pg_hba.conf

# Добавь строку для local connections:
# local   all   neurograph_user   md5
```

### Проблема: Slow queries

```sql
-- Включи логирование медленных запросов
ALTER DATABASE neurograph_db SET log_min_duration_statement = 100;  -- 100ms

-- Analyze query plan
EXPLAIN ANALYZE SELECT * FROM experience_events WHERE timestamp > ...;
```

---

## Docker Setup (Опционально)

```bash
# Запуск PostgreSQL в Docker
docker run --name neurograph-postgres \
    -e POSTGRES_USER=neurograph_user \
    -e POSTGRES_PASSWORD=your_secure_password \
    -e POSTGRES_DB=neurograph_db \
    -p 5432:5432 \
    -v neurograph_data:/var/lib/postgresql/data \
    -d postgres:15

# Применение схемы
docker exec -i neurograph-postgres psql -U neurograph_user -d neurograph_db < schema.sql
```

---

**Готово!** PostgreSQL backend настроен для NeuroGraph OS v0.26.0 🚀