# NeuroGraph OS - Документация CLI

Полное руководство по использованию интерфейса командной строки для управления NeuroGraph OS.

## 📋 Оглавление

- [Установка](#установка)
- [Быстрый старт](#быстрый-старт)
- [Команды](#команды)
- [Примеры использования](#примеры-использования)

---

## 🚀 Установка

### Вариант 1: Установка в режиме разработки

```bash
# Из корня проекта
pip install -e .
```

### Вариант 2: Установка зависимостей

```bash
pip install -r requirements-cli.txt
pip install -r requirements-persistence.txt
```

### Проверка установки

```bash
neurograph --version
neurograph --help
```

---

## ⚡ Быстрый старт

### 1. Проверка системы

```bash
# Полная информация о системе
neurograph system status

# Проверка здоровья
neurograph system health

# Показать информацию
neurograph info
```

### 2. Инициализация БД

```bash
# Запустить базы данных
docker-compose -f docker-compose.db.yml up -d

# Проверить статус БД
neurograph db status

# Инициализировать схему
neurograph db init

# Или использовать миграции (рекомендуется)
neurograph db migrate upgrade
```

### 3. Работа с токенами

```bash
# Создать токен
neurograph token create --type test --x 1.0 --y 0.0 --z 0.0

# Список токенов
neurograph token list

# Детали токена
neurograph token get <TOKEN_ID>

# Поиск в регионе
neurograph token search --region 0 0 0 10 10 10
```

### 4. Работа с графом

```bash
# Создать связь
neurograph graph connect <SOURCE_ID> <TARGET_ID> --type spatial

# Посмотреть соседей
neurograph graph neighbors <TOKEN_ID>

# Найти путь
neurograph graph path <SOURCE_ID> <TARGET_ID>

# Статистика графа
neurograph graph stats
```

---

## 📚 Команды

### Управление токенами

#### `neurograph token create`

Создание нового токена.

```bash
neurograph token create [ОПЦИИ]

Опции:
  --type, -t TEXT         Тип токена [по умолчанию: default]
  --x FLOAT              X координаты (8 уровней, повторить 8 раз)
  --y FLOAT              Y координаты (8 уровней)
  --z FLOAT              Z координаты (8 уровней)
  --weight, -w FLOAT     Вес токена [по умолчанию: 1.0]
  --flags INTEGER        Флаги токена [по умолчанию: 0]
```

**Примеры:**

```bash
# Простой токен
neurograph token create --type test

# С координатами
neurograph token create --type spatial \
  --x 1.0 --x 0.5 --x 0.25 --x 0.125 \
  --y 0.0 --y 0.0 --y 0.0 --y 0.0 \
  --z 0.0 --z 0.0 --z 0.0 --z 0.0

# С весом и флагами
neurograph token create --type weighted --weight 2.5 --flags 15
```

#### `neurograph token list`

Список токенов.

```bash
neurograph token list [ОПЦИИ]

Опции:
  --limit, -l INTEGER    Количество токенов [по умолчанию: 10]
  --type, -t TEXT        Фильтр по типу токена
  --format, -f [table|json]  Формат вывода [по умолчанию: table]
```

#### `neurograph token get`

Получить токен по ID.

```bash
neurograph token get TOKEN_ID [ОПЦИИ]

Опции:
  --verbose, -v          Показать подробную информацию
```

#### `neurograph token search`

Поиск токенов в пространственном регионе.

```bash
neurograph token search [ОПЦИИ]

Опции:
  --region, -r FLOAT...  minX minY minZ maxX maxY maxZ
  --level, -l INTEGER    Уровень координат [по умолчанию: 0]
```

**Пример:**

```bash
# Поиск в кубе от (0,0,0) до (10,10,10)
neurograph token search --region 0 0 0 10 10 10
```

#### `neurograph token delete`

Удалить токен.

```bash
neurograph token delete TOKEN_ID [ОПЦИИ]

Опции:
  --force, -f            Пропустить подтверждение
```

#### `neurograph token count`

Подсчитать токены.

```bash
neurograph token count [ОПЦИИ]

Опции:
  --type, -t TEXT        Фильтр по типу
```

---

### Управление графом

#### `neurograph graph connect`

Создать связь между токенами.

```bash
neurograph graph connect SOURCE_ID TARGET_ID [ОПЦИИ]

Опции:
  --type, -t TEXT            Тип связи [по умолчанию: generic]
  --weight, -w FLOAT         Вес [по умолчанию: 1.0]
  --bidirectional, -b        Двусторонняя связь
```

**Примеры:**

```bash
# Простая связь
neurograph graph connect <ID1> <ID2>

# Пространственная связь
neurograph graph connect <ID1> <ID2> --type spatial --weight 0.8

# Двусторонняя связь
neurograph graph connect <ID1> <ID2> --bidirectional
```

#### `neurograph graph neighbors`

Получить соседей токена.

```bash
neurograph graph neighbors TOKEN_ID [ОПЦИИ]

Опции:
  --direction, -d [incoming|outgoing|both]  [по умолчанию: both]
  --type, -t TEXT                           Фильтр по типу
```

#### `neurograph graph degree`

Получить степень узла (количество связей).

```bash
neurograph graph degree TOKEN_ID
```

#### `neurograph graph path`

Найти пути между токенами.

```bash
neurograph graph path SOURCE_ID TARGET_ID [ОПЦИИ]

Опции:
  --max-depth, -d INTEGER    Максимальная глубина [по умолчанию: 5]
```

#### `neurograph graph stats`

Показать статистику графа.

```bash
neurograph graph stats
```

#### `neurograph graph visualize`

Визуализировать окрестность токена.

```bash
neurograph graph visualize TOKEN_ID [ОПЦИИ]

Опции:
  --depth, -d INTEGER    Глубина визуализации [по умолчанию: 2]
```

---

### Управление базой данных

#### `neurograph db init`

Инициализировать базу данных.

```bash
neurograph db init [ОПЦИИ]

Опции:
  --drop                 Удалить существующие таблицы сначала
```

#### `neurograph db migrate`

Управление миграциями.

```bash
neurograph db migrate [upgrade|downgrade|current|history] [ОПЦИИ]

Опции:
  --revision, -r TEXT    Целевая ревизия [по умолчанию: head]
```

**Примеры:**

```bash
# Применить все миграции
neurograph db migrate upgrade

# Откатить одну миграцию
neurograph db migrate downgrade -1

# Показать текущую версию
neurograph db migrate current

# История миграций
neurograph db migrate history
```

#### `neurograph db revision`

Создать новую миграцию.

```bash
neurograph db revision [ОПЦИИ]

Опции:
  --message, -m TEXT     Сообщение миграции (обязательно)
  --autogenerate, -a     Автогенерация из моделей
```

**Пример:**

```bash
neurograph db revision -m "Добавить новое поле" --autogenerate
```

#### `neurograph db status`

Проверить статус подключения к БД.

```bash
neurograph db status
```

#### `neurograph db backup`

Создать резервную копию.

```bash
neurograph db backup [ОПЦИИ]

Опции:
  --output, -o PATH      Путь к файлу резервной копии
```

#### `neurograph db restore`

Восстановить из резервной копии.

```bash
neurograph db restore BACKUP_FILE [ОПЦИИ]

Опции:
  --force, -f            Пропустить подтверждение
```

#### `neurograph db clean`

Очистить все данные (сохраняет схему).

```bash
neurograph db clean [ОПЦИИ]

Опции:
  --force, -f            Пропустить подтверждение
```

#### `neurograph db reset`

Полностью сбросить базу данных.

```bash
neurograph db reset [ОПЦИИ]

Опции:
  --force, -f            Пропустить подтверждение
```

---

### Управление системой

#### `neurograph system status`

Показать полный статус системы.

```bash
neurograph system status
```

#### `neurograph system info`

Системная информация.

```bash
neurograph system info
```

#### `neurograph system health`

Проверка здоровья.

```bash
neurograph system health [ОПЦИИ]

Опции:
  --json                 Вывод в формате JSON
```

#### `neurograph system metrics`

Показать метрики системы.

```bash
neurograph system metrics
```

#### `neurograph system logs`

Показать логи.

```bash
neurograph system logs [ОПЦИИ]

Опции:
  --lines, -n INTEGER    Количество строк [по умолчанию: 50]
  --follow, -f           Следить за выводом логов
```

#### `neurograph system version`

Показать версию.

```bash
neurograph system version
```

---

### Управление конфигурацией

#### `neurograph config show`

Показать конфигурацию.

```bash
neurograph config show [CONFIG_NAME] [ОПЦИИ]

Опции:
  --format, -f [yaml|json]   Формат вывода [по умолчанию: yaml]
```

**Примеры:**

```bash
# Список конфигураций
neurograph config show

# Показать конкретную конфигурацию
neurograph config show infrastructure/database

# В формате JSON
neurograph config show infrastructure/database --format json
```

#### `neurograph config validate`

Валидировать конфигурацию.

```bash
neurograph config validate CONFIG_NAME
```

#### `neurograph config tree`

Показать конфигурацию как дерево.

```bash
neurograph config tree CONFIG_NAME
```

#### `neurograph config get`

Получить конкретное значение.

```bash
neurograph config get CONFIG_NAME KEY_PATH
```

**Пример:**

```bash
neurograph config get infrastructure/database database.postgres.host
```

#### `neurograph config list`

Список всех конфигураций.

```bash
neurograph config list
```

#### `neurograph config env`

Показать переменные окружения.

```bash
neurograph config env
```

#### `neurograph config template`

Создать шаблон конфигурации.

```bash
neurograph config template [database|env] [ОПЦИИ]

Опции:
  --output, -o PATH      Путь к выходному файлу
```

---

## 💡 Примеры использования

### Сценарий 1: Первоначальная настройка

```bash
# 1. Запустить Docker-контейнеры
docker-compose -f docker-compose.db.yml up -d

# 2. Проверить статус
neurograph db status

# 3. Применить миграции
neurograph db migrate upgrade

# 4. Проверить систему
neurograph system status
```

### Сценарий 2: Создание и связывание токенов

```bash
# Создать первый токен
TOKEN1=$(neurograph token create --type node1 | grep "ID:" | awk '{print $2}')

# Создать второй токен
TOKEN2=$(neurograph token create --type node2 | grep "ID:" | awk '{print $2}')

# Связать токены
neurograph graph connect $TOKEN1 $TOKEN2 --type spatial --weight 0.9

# Проверить соседей
neurograph graph neighbors $TOKEN1
```

### Сценарий 3: Мониторинг системы

```bash
# Полный статус
neurograph system status

# Метрики
neurograph system metrics

# Статистика графа
neurograph graph stats

# Следить за логами
neurograph system logs --follow
```

### Сценарий 4: Backup и Restore

```bash
# Создать бэкап
neurograph db backup --output backup_$(date +%Y%m%d).sql

# Восстановить из бэкапа
neurograph db restore backup_20250115.sql
```

---

## 🔧 Глобальные опции

Все команды поддерживают:

```bash
--verbose, -v          Подробный вывод
--config, -c PATH      Пользовательский файл конфигурации
--help                 Показать справку
```

---

## 🎨 Форматирование вывода

CLI использует **Rich** для красивого вывода:

- ✅ Цветной текст
- 📊 Таблицы
- 🌳 Деревья
- 📈 Прогресс-бары
- 🎨 Подсветка синтаксиса

---

## 🐛 Устранение неполадок

### Команда не найдена

```bash
# Переустановить CLI
pip install -e .

# Проверить установку
which neurograph
```

### Ошибки подключения к БД

```bash
# Проверить Docker-контейнеры
docker ps

# Проверить логи
docker logs neurograph_postgres

# Проверить статус через CLI
neurograph db status
```

### Конфигурация не найдена

```bash
# Проверить структуру config/
neurograph config list

# Валидировать конфигурацию
neurograph config validate infrastructure/database
```

---

**Версия**: 0.3.0  
**Последнее обновление**: 2025-10-15