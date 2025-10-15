# Руководство по настройке

## Оглавление

1. [Требования](#требования)
2. [Быстрый старт с Docker (рекомендуемый способ)](#быстрый-старт-с-docker-рекомендуемый-способ)
3. [Ручная установка (для разработки)](#ручная-установка-для-разработки)
4. [Настройка IDE](#настройка-ide)
5. [Проверка установки](#проверка-установки)
6. [Устранение неполадок](#устранение-неполадок)
7. [Обновление проекта](#обновление-проекта)

## Требования

### Обязательные
- Python 3.10+
- PostgreSQL 13+
- Redis 6+
- Docker 20.10+
- Docker Compose 2.0+
- Git 2.30+

### Рекомендуемые
- 8 ГБ ОЗУ
- 4+ ядра процессора
- 10 ГБ свободного места на диске

## Быстрый старт с Docker (рекомендуемый способ)

Этот метод автоматически настраивает все необходимые сервисы (PostgreSQL, Redis) и само приложение.

1.  **Клонируйте репозиторий:**
    ```bash
    git clone https://github.com/dchrnv/neurograph-os-dev.git
    cd neurograph-os-dev
    ```

2.  **Создайте файл `.env`:**
    Скопируйте файл с примером переменных окружения. Для локального запуска его изменять не нужно.
    ```bash
    cp .env.example .env
    ```

3.  **Соберите и запустите контейнеры:**
    Эта команда соберет образы и запустит все сервисы в фоновом режиме.
    ```bash
    docker-compose up -d --build
    ```

4.  **Примените миграции базы данных:**
    ```bash
    docker-compose exec web alembic upgrade head
    ```

5.  **Проверьте, что все работает:**
    Приложение будет доступно по адресу `http://localhost:8000`.

### Доступные сервисы
- Веб-приложение: http://localhost:8000
- PGAdmin: http://localhost:5050
- Redis Commander: http://localhost:8081
- API документация: http://localhost:8000/docs

## Ручная установка (для разработки)

Этот способ подходит, если вы хотите запускать компоненты системы напрямую на вашей машине.

### 1. Настройте переменные окружения

Скопируйте файл с примером. Для ручной установки убедитесь, что `POSTGRES_HOST` и `REDIS_HOST` установлены в `localhost`.

```bash
cp .env.example .env
# Отредактируйте .env, если необходимо
```

### 1. Создайте и активируйте виртуальное окружение

> **Важно:** Этот шаг обязателен для изоляции зависимостей и предотвращения ошибок `externally-managed-environment` в современных ОС.

```bash
# На системах Debian/Ubuntu может потребоваться установить пакет для создания виртуальных окружений:
# sudo apt update
# sudo apt install python3-venv

# Создайте виртуальное окружение
python -m venv .venv
source .venv/bin/activate  # Linux/Mac
# ИЛИ
.\.venv\Scripts\activate  # Windows

# После активации вы увидите (.venv) в начале командной строки.
```

### 2. Установите зависимости
```bash
pip install -e .[all]
```

### 2. Настройте базу данных
```bash
# Создайте базу данных
createdb neurograph

> **Ошибка подключения?** Если вы видите ошибку `could not connect to server` или `подключиться... не удалось`, это означает, что `createdb` не может найти запущенный сервер PostgreSQL.
> 1.  **Убедитесь, что PostgreSQL запущен.** На Linux: `sudo systemctl status postgresql`. На macOS (Homebrew): `brew services list`.
> 2.  **Попробуйте подключиться через TCP/IP**, указав хост. Это самый надежный способ:
>     ```bash
>     createdb -h localhost -p 5432 neurograph
>     ```

# Примените миграции
alembic upgrade head

# Загрузите начальные данные
python -m scripts.load_fixtures
```

### 3. Запустите сервисы
```bash
# Запустите Redis
redis-server &

# Запустите Celery
celery -A app.worker worker --loglevel=info &

# Запустите сервер разработки
uvicorn app.main:app --reload
```

## Настройка IDE

### VS Code
Установите рекомендуемые расширения:
```bash
code --install-extension ms-python.python
code --install-extension ms-python.vscode-pylance
code --install-extension eamodio.gitlens
code --install-extension dbaeumer.vscode-eslint
```

### PyCharm
1. Откройте настройки (Ctrl+Alt+S)
2. Перейдите в раздел "Project: neurograph-os" > "Python Interpreter"
3. Выберите "Add Interpreter" > "Add Local Interpreter"
4. Выберите "Virtualenv Environment" и укажите путь к `venv`
5. Примените изменения

## Проверка установки

### 1. Проверьте версию Python
```bash
python --version
# Должно быть Python 3.10+
```

### 2. Проверьте зависимости
```bash
pip list
# Убедитесь, что все зависимости из requirements.txt установлены
```

### 3. Запустите тесты
```bash
pytest
# Все тесты должны проходить успешно
```

### 4. Проверьте API
```bash
curl http://localhost:8000/api/v1/health
# Должен вернуть {"status":"ok"}
```

## Устранение неполадок

### Проблемы с базой данных
```bash
# Проверьте статус PostgreSQL
sudo systemctl status postgresql

# Перезапустите сервис
sudo systemctl restart postgresql

# Проверьте подключение
psql -U postgres -d neurograph -c "SELECT 1"
```

### Проблемы с Redis
```bash
# Проверьте статус Redis
redis-cli ping
# Должен ответить PONG

# Очистите кэш
redis-cli FLUSHALL
```

### Проблемы с зависимостями
```bash
# Удалите виртуальное окружение и установите заново
rm -rf .venv
python -m venv .venv
source .venv/bin/activate
pip install -r requirements/dev.txt
```

## Дополнительные настройки

### Настройка HTTPS
1. Получите SSL сертификат (например, с помощью Let's Encrypt)
2. Обновите конфигурацию в `.env`:
   ```
   SSL_CERTFILE=/path/to/cert.pem
   SSL_KEYFILE=/path/to/key.pem
   ```
3. Перезапустите сервер

### Настройка почтового сервера
```env
# В файле .env
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USER=user@example.com
SMTP_PASSWORD=your-password
EMAILS_FROM_EMAIL=noreply@example.com
EMAILS_FROM_NAME="Neurograph OS"
```

### Мониторинг
Для мониторинга приложения можно использовать:
```bash
# Запустите Prometheus и Grafana
docker-compose -f docker-compose.monitoring.yml up -d
```

## Обновление
```bash
# Получите последние изменения
git pull

# Обновите зависимости
pip install -r requirements/dev.txt

# Примените новые миграции
alembic upgrade head

# Перезапустите сервисы
docker-compose up -d --build
```

## Получение помощи
Если у вас возникли проблемы с настройкой, обратитесь:
- К документации: https://docs.neurographos.com
- В чат сообщества: https://slack.neurographos.com
- Создайте issue: https://github.com/yourusername/neurograph-os/issues
