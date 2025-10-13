# Experience Stream

Короткое руководство по подсистеме ExperienceStream — поток записи и хранения событий опыта для обучения и анализа.

## Концепция

ExperienceStream собирает события из компонентов (TokenFactory, CoordinateSystem, Graph и др.), буферизует их и поставляет в несколько потребителей:

- хранилища (горячий буфер и скользящее окно),
- бекенды вывода (console, file JSONL, HTTP POST),
- механизмы сэмплирования для обучения (uniform, prioritized, recent, diverse),
- интеграция с DNAGuardian для записи событий ADNA/CDNA.

Ключевые цели: минимальное влияние на основной поток, асинхронный flush, простая конфигурация через YAML.

## Файлы и структура

- `src/core/experience/stream.py` — главный класс `ExperienceStream` (буферизация, flush, API чтения).
- `src/core/experience/event.py` — Pydantic-модель `ExperienceEvent` (совместима с Pydantic v2).
- `src/core/experience/events.py` — dataclass-структуры `EventRecord`, `ExperienceTrajectory`, `ExperienceBatch`.
- `src/core/experience/storage.py` — `CircularBuffer`, `SlidingWindow`.
- `src/core/experience/samplers.py` — `ExperienceSampler` и стратегии сэмплинга.
- `config/application/experience.yaml` — runtime конфигурация (defaults).

## Конфигурация

Пример основных опций (в `config/application/experience.yaml`):

```yaml
experience:
  stream:
    batch_size: 256
    flush_interval: 1.0
    backend: console
  storage:
    circular_buffer_size: 100000
    sliding_window_size: 1000000
    sliding_window_duration_hours: 24
  sampling:
    default_strategy: uniform
    prioritized_alpha: 0.6
    prioritized_beta: 0.4
```

`ExperienceStream` может загружать конфиг автоматически (путь по умолчанию `config/application/experience.yaml`) или принимать словарь/путь в конструктор.

## Примеры использования

```python
from src.core.experience.stream import ExperienceStream

# Create with defaults from config file
stream = ExperienceStream(config=None)

# Write events (accepts Pydantic model, dict or EventRecord)
stream.write_event({ 'event_id': 'e1', 'event_type': 'token_created', 'timestamp': time.time(), 'source_component': 'token_factory' })

# Sample a batch
batch = stream.sample_batch(batch_size=32, strategy='uniform')
```

## Тесты

Тесты находятся в `src/core/experience/tests/` и покрывают storage, samplers, events, базовую работу stream.

## Дальнейшие задачи

- Интеграция с DNAGuardian (подписка и запись dna-событий)
- Более продвинутые политики хранения (архивация, перенос в S3/Postgres)
- Визуализация метрик и мониторинг
