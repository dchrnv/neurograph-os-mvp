# Фабрика токенов (TokenFactory)

## Назначение
**TokenFactory** — это централизованный компонент, отвечающий за создание и парсинг токенов. Его основная задача — инкапсулировать логику сборки сложных токенов из различных данных, обеспечивая консистентность и упрощая их использование в других частях системы.

Ключевая специализация фабрики — работа с **"токенами опыта"** (Experience Tokens), которые используются в задачах обучения с подкреплением (Reinforcement Learning).

## Конфигурация

Фабрика настраивается через Pydantic-модель `TokenFactoryConfig`, которая загружается из YAML-файла `config/core/token.yaml`. Это обеспечивает валидацию, автодополнение в IDE и четкую структуру настроек.

**Структура `config/core/token.yaml`:**

```yaml
# config/core/token.yaml
token:
  factory:
    # Значения по умолчанию для всех создаваемых токенов
    defaults:
      weight: 0.0
      flags: 0
      auto_timestamp: true

    # Специализация для "токенов опыта" (Reinforcement Learning)
    experience_format:
      # Уровень для вектора состояния (s_t)
      state_level: 0
      # Уровень для вектора действия (a_t)
      action_level: 1
      # Уровень для вектора следующего состояния (s_t+1)
      next_state_level: 2
      # Номер бита в поле `flags` для обозначения конца эпизода (done)
      done_flag_bit: 0
```

## Основные методы

### `create_empty_token() -> Token`
Создает "чистый" токен с базовыми значениями, взятыми из конфигурации (`defaults`). Генерирует для него уникальный `id` (на основе UUID) и устанавливает временную метку.

```python
token = factory.create_empty_token()
```

### Создание токена опыта
```python
experience = factory.create_experience_token(
    s_t=[0.1, 0.2],      # Текущее состояние
    a_t=[0.5],            # Действие
    r_t=0.8,              # Награда
    s_t_plus_1=[0.2, 0.1], # Следующее состояние
    done=False            # Флаг завершения эпизода
)
```

### Создание токена состояния
```python
state_token = factory.create_state_token(
    observation=[0.1, 0.2, 0.3]  # Наблюдение
)
```

### Создание токена действия
```python
action_token = factory.create_action_token(
    action=[0.7, 0.3]  # Вектор действия
)
```

## Внутренняя структура
- `_counter`: Счётчик для генерации уникальных ID
- `_experience_config`: Конфигурация формата опыта

## Обработка ошибок
- Проверка размерности входных векторов
- Валидация диапазонов значений
- Обработка отсутствующих значений

## Пример использования
```python
config = {
    "default_values": {"weight": 0.5, "flags": 1, "auto_timestamp": True}
}
factory = TokenFactory(config)

# Создание токена опыта
token = factory.create_experience_token(
    s_t=[0.1, 0.2],
    a_t=[0.5],
    r_t=0.8,
    s_t_plus_1=[0.2, 0.1],
    done=False
)
```
