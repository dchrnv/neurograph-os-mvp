# Руководство по настройке

## Оглавление
1. [Требования](#требования)
2. [Быстрый старт](#быстрый-старт)
3. [Настройка окружения](#настройка-окружения)
4. [Запуск в Docker](#запуск-в-docker)
5. [Ручная установка](#ручная-установка)
6. [Настройка IDE](#настройка-ide)
7. [Проверка установки](#проверка-установки)
8. [Устранение неполадок](#устранение-неполадок)
9. [Дополнительные настройки](#дополнительные-настройки)

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

## Быстрый старт

### Клонирование репозитория
```bash
git clone https://github.com/yourusername/neurograph-os.git
cd neurograph-os
```

### Запуск с Docker (рекомендуемый способ)
```bash
# Создайте и запустите контейнеры
docker-compose up -d

# Примените миграции
docker-compose exec web alembic upgrade head

# Проверьте логи
docker-compose logs -f
```

Приложение будет доступно по адресу: http://localhost:8000

## Настройка окружения

### 1. Создайте файл .env
```bash
cp .env.example .env
```

### 2. Настройте переменные окружения
Отредактируйте файл `.env`:

```env
# Основные настройки
DEBUG=True
ENVIRONMENT=development
SECRET_KEY=your-secret-key

# База данных
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=neurograph
POSTGRES_HOST=db
POSTGRES_PORT=5432
DATABASE_URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}

# Redis
REDIS_HOST=redis
REDIS_PORT=6379
REDIS_URL=redis://${REDIS_HOST}:${REDIS_PORT}/0

# Настройки API
API_V1_STR=/api/v1
PROJECT_NAME=Neurograph OS
```

## Запуск в Docker

### Сборка и запуск
```bash
# Сборка образов
docker-compose build

# Запуск сервисов
docker-compose up -d

# Остановка сервисов
docker-compose down

# Просмотр логов
docker-compose logs -f

# Выполнение команд в контейнере
docker-compose exec web bash
```

### Доступные сервисы
- Веб-приложение: http://localhost:8000
- PGAdmin: http://localhost:5050
- Redis Commander: http://localhost:8081
- API документация: http://localhost:8000/docs

## Ручная установка

### 1. Установите зависимости
```bash
# На системах Debian/Ubuntu может потребоваться установить пакет для создания виртуальных окружений:
# sudo apt update
# sudo apt install python3-venv

# Создайте виртуальное окружение
python -m venv .venv
source .venv/bin/activate  # Linux/Mac
# ИЛИ
.\venv\Scripts\activate  # Windows

# Установите зависимости
pip install -r requirements/dev.txt
```

### 2. Настройте базу данных
```bash
# Создайте базу данных
createdb neurograph

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
