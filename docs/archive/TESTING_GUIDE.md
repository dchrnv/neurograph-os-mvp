# Руководство по тестированию

## Оглавление
1. [Обзор](#обзор)
2. [Типы тестов](#типы-тестов)
3. [Настройка окружения](#настройка-окружения)
4. [Написание тестов](#написание-тестов)
5. [Запуск тестов](#запуск-тестов)
6. [Покрытие кода](#покрытие-кода)
7. [Моки и стабы](#моки-и-стабы)
8. [Интеграционные тесты](#интеграционные-тесты)
9. [Нагрузочное тестирование](#нагрузочное-тестирование)
10. [Лучшие практики](#лучшие-практики)

## Обзор

Это руководство описывает процесс тестирования в проекте Neurograph OS. Мы используем комбинацию модульного, интеграционного и системного тестирования для обеспечения надежности и стабильности кодовой базы.

## Типы тестов

### 1. Модульные тесты (Unit Tests)
- Тестируют отдельные функции и методы
- Быстрые и изолированные
- Не требуют внешних зависимостей

### 2. Интеграционные тесты
- Тестируют взаимодействие между компонентами
- Проверяют работу с базой данных и внешними сервисами
- Требуют настроенного тестового окружения

### 3. Системные тесты
- Тестируют систему в целом
- Проверяют эндпоинты API
- Имитируют поведение пользователя

### 4. Нагрузочные тесты
- Проверяют производительность под нагрузкой
- Выявляют узкие места
- Помогают оценить масштабируемость

## Настройка окружения

### Установка зависимостей
```bash
pip install -r requirements/test.txt
```

### Конфигурация
Создайте файл `pytest.ini` в корне проекта:
```ini
[pytest]
testpaths = tests
python_files = test_*.py
python_classes = Test*
python_functions = test_*
addopts = -v --cov=src --cov-report=term-missing
```

### Фикстуры
Используйте фикстуры для общих настроек тестов. Пример:
```python
import pytest
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker

@pytest.fixture(scope="session")
def db_engine():
    return create_engine("sqlite:///:memory:")

@pytest.fixture
db_session(db_engine):
    connection = db_engine.connect()
    transaction = connection.begin()
    session = sessionmaker(bind=connection)()
    
    yield session
    
    session.close()
    transaction.rollback()
    connection.close()
```

## Написание тестов

### Структура теста
Каждый тест должен следовать шаблону AAA (Arrange-Act-Assert):

```python
def test_add_token_to_grid():
    # Arrange
    grid = SparseGrid()
    token = Token(coordinates=(0.5, 0.5, 0.5))
    
    # Act
    grid.add(token)
    
    # Assert
    assert token in grid
    assert len(grid) == 1
```

### Параметризованные тесты
Используйте `@pytest.mark.parametrize` для тестирования с разными входными данными:

```python
import pytest

@pytest.mark.parametrize("x,y,z,expected", [
    (0, 0, 0, True),
    (1, 1, 1, True),
    (-1, 0, 0, False),
    (2, 0, 0, False),
])
def test_is_inside_grid(x, y, z, expected):
    grid = SparseGrid(size=1.0)
    assert grid.is_inside((x, y, z)) == expected
```

## Запуск тестов

### Основные команды
```bash
# Все тесты
pytest

# Конкретный тест
pytest tests/test_grid.py::test_add_token

# Тесты с маркером
pytest -m "not slow"

# Параллельный запуск
pytest -n auto
```

### Полезные флаги
- `-v` - подробный вывод
- `-s` - показать вывод print
- `--pdb` - запустить отладчик при ошибке
- `--lf` - запустить только упавшие тесты
- `--sw` - запускать тесты до первой ошибки

## Покрытие кода

### Генерация отчета
```bash
pytest --cov=src --cov-report=html
```

### Игнорирование кода
Используйте комментарий `# pragma: no cover` для исключения кода из отчета о покрытии:

```python
if debug:  # pragma: no cover
    print("Debug info:", debug_info)
```

## Моки и стабы

### unittest.mock
```python
from unittest.mock import Mock, patch

def test_process_data():
    # Создаем мок-объект
    mock_loader = Mock()
    mock_loader.load.return_value = [1, 2, 3]
    
    result = process_data(mock_loader)
    
    assert result == 6
    mock_loader.load.assert_called_once()

# Использование патчей
@patch('mymodule.requests')
def test_fetch_data(mock_requests):
    mock_response = Mock()
    mock_response.json.return_value = {"key": "value"}
    mock_requests.get.return_value = mock_response
    
    result = fetch_data("http://example.com/api")
    
    assert result == {"key": "value"}
    mock_requests.get.assert_called_with("http://example.com/api")
```

## Интеграционные тесты

### Тестирование API
```python
import requests

def test_create_token():
    url = "http://localhost:8000/api/tokens"
    data = {
        "type": "test",
        "coordinates": [0.5, 0.5, 0.5]
    }
    
    response = requests.post(url, json=data)
    
    assert response.status_code == 201
    assert "id" in response.json()
```

### Тестирование с базой данных
```python
def test_save_token(db_session):
    token = Token(type="test", coordinates=(0.5, 0.5, 0.5))
    
    db_session.add(token)
    db_session.commit()
    
    saved_token = db_session.query(Token).first()
    assert saved_token is not None
    assert saved_token.type == "test"
```

## Нагрузочное тестирование

### Locust
Создайте файл `locustfile.py`:

```python
from locust import HttpUser, task, between

class ApiUser(HttpUser):
    wait_time = between(1, 5)
    
    @task
    def create_token(self):
        self.client.post("/api/tokens", json={
            "type": "test",
            "coordinates": [0.5, 0.5, 0.5]
        })
    
    @task(3)
    def get_token(self):
        self.client.get("/api/tokens/1")
```

Запустите тест:
```bash
locust -f locustfile.py
```

## Лучшие практики

### Что тестировать
- Все публичные методы и функции
- Граничные случаи
- Обработку ошибок
- Валидацию входных данных

### Чего избегать
- Тестирования приватных методов напрямую
- Избыточных тестов
- Хрупких тестов, зависящих от реализации

### Поддержка тестов
- Держите тесты изолированными
- Используйте понятные имена тестов
- Комментируйте сложные проверки
- Обновляйте тесты при изменении кода

### Производительность
- Тесты должны выполняться быстро
- Используйте моки для медленных операций
- Запускайте длительные тесты отдельно

## Дополнительные материалы
- [Документация pytest](https://docs.pytest.org/)
- [unittest.mock](https://docs.python.org/3/library/unittest.mock.html)
- [Locust](https://locust.io/)
- [pytest-cov](https://pytest-cov.readthedocs.io/)
