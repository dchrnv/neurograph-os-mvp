# ./Dockerfile
FROM python:3.11-slim-bookworm

# Установка времениzone
ENV TZ=UTC
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

WORKDIR /app

# Установка системных зависимостей
RUN apt-get update && apt-get install -y \
    gcc \
    g++ \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Создание пользователя для безопасности
RUN useradd -m -u 1000 neurograph && \
    mkdir -p /app/data && \
    chown neurograph:neurograph /app /app/data

# Копирование требований и установка зависимостей
COPY requirements/core.txt .
RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir -r core.txt && \
    pip cache purge

# Копирование исходного кода с правильными правами
COPY --chown=neurograph:neurograph src/ ./src/
COPY --chown=neurograph:neurograph config/ ./config/
COPY --chown=neurograph:neurograph pyproject.toml .

# Переключаемся на непривилегированного пользователя
USER neurograph

# Создаем том для данных
VOLUME /app/data

# Экспортируем порт
EXPOSE 8000

# Health check
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# Точка входа
ENTRYPOINT ["python", "-m", "src.main"]

# Установка зависимостей слоями для кэширования
COPY requirements/core.txt .
RUN pip install --no-cache-dir -r core.txt

COPY requirements/api.txt .
RUN pip install --no-cache-dir -r api.txt

# Для production не устанавливаем dev зависимости
# COPY requirements/dev.txt .
# RUN pip install --no-cache-dir -r dev.txt