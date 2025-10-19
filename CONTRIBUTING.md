# Участие в разработке NeuroGraph OS

Спасибо за интерес к проекту! Мы рады любому вкладу.

## 🎯 Как помочь проекту

### 1. Исследование и обратная связь
- Тестируйте MVP и сообщайте о багах
- Предлагайте улучшения в Issues
- Делитесь идеями по архитектуре

### 2. Разработка
- Добавляйте новые фичи
- Исправляйте баги
- Улучшайте документацию
- Пишите тесты

### 3. Документация
- Улучшайте существующие docs
- Добавляйте примеры использования
- Переводите на другие языки

---

## 🔧 Процесс разработки

### Шаг 1: Fork и клонирование

```bash
# Fork репозитория через GitHub UI
git clone https://github.com/YOUR_USERNAME/neurograph-os-mvp.git
cd neurograph-os-mvp
git remote add upstream https://github.com/dchrnv/neurograph-os-mvp.git
```

### Шаг 2: Создание ветки

```bash
git checkout -b feature/your-feature-name
# или
git checkout -b bugfix/issue-123
```

**Naming conventions:**
- `feature/` - новые функции
- `bugfix/` - исправления багов
- `docs/` - изменения документации
- `refactor/` - рефакторинг без изменения API

### Шаг 3: Разработка

```bash
# Создайте виртуальное окружение
python3 -m venv .venv
source .venv/bin/activate

# Установите зависимости
pip install -r requirements.txt

# Установите pre-commit hooks (если есть)
# pre-commit install
```

### Шаг 4: Тестирование

```bash
# Запустите тесты
python -m pytest src/core/token/tests/ -v

# Проверьте стиль кода (если настроено)
# black src/
# isort src/
# flake8 src/
```

### Шаг 5: Коммит

```bash
git add .
git commit -m "feat: add awesome feature

Detailed description of what was changed and why.

Closes #123"
```

**Commit message format:**
```
<type>: <short summary>

<optional detailed description>

<optional footer>
```

**Types:**
- `feat:` - новая функция
- `fix:` - исправление бага
- `docs:` - изменения в документации
- `refactor:` - рефакторинг
- `test:` - добавление тестов
- `chore:` - изменения инфраструктуры

### Шаг 6: Push и Pull Request

```bash
git push origin feature/your-feature-name
```

Затем создайте Pull Request через GitHub UI:

1. Опишите что изменили
2. Укажите связанные Issues
3. Добавьте скриншоты (если UI)
4. Отметьте чеклист

---

## 📋 Checklist для Pull Request

- [ ] Код работает и протестирован
- [ ] Добавлены тесты для новой функциональности
- [ ] Документация обновлена
- [ ] Commit messages следуют формату
- [ ] Нет конфликтов с main веткой
- [ ] Код соответствует стилю проекта

---

## 🎨 Code Style

### Python
- PEP 8 compliance
- Type hints где возможно
- Docstrings для публичных функций/классов
- Максимальная длина строки: 100 символов

```python
def create_token(
    entity_type: int,
    domain: int = 0,
    weight: float = 0.5
) -> Token:
    """
    Создаёт новый токен с указанными параметрами.

    Args:
        entity_type: Тип сущности (0-15)
        domain: Домен (0-15)
        weight: Вес токена (0.0-1.0)

    Returns:
        Token: Созданный токен
    """
    token_id = create_token_id(get_next_id(), entity_type, domain)
    return Token(id=token_id, weight=weight)
```

### TypeScript/React
- ESLint rules
- Functional components с hooks
- Typed props
- CSS modules или styled-components

```typescript
interface TokenCardProps {
  token: Token;
  onDelete: (id: number) => void;
}

export const TokenCard: React.FC<TokenCardProps> = ({ token, onDelete }) => {
  return (
    <div className="token-card">
      <h3>Token #{token.id}</h3>
      <button onClick={() => onDelete(token.id)}>Delete</button>
    </div>
  );
};
```

---

## 🧪 Тестирование

### Unit Tests
```python
# src/core/token/tests/test_your_feature.py
import pytest
from src.core.token.token_v2 import Token

def test_token_creation():
    """Тест создания токена"""
    token = Token(id=1)
    assert token.id == 1
    assert token.weight == 0.5
```

### Integration Tests
Пишите интеграционные тесты для API endpoints:

```python
# tests/integration/test_api.py
def test_create_token_endpoint(client):
    """Тест создания токена через API"""
    response = client.post("/api/v1/tokens", json={
        "entity_type": 1,
        "domain": 0
    })
    assert response.status_code == 201
    assert "id" in response.json()
```

---

## 🐛 Reporting Bugs

При создании Issue для бага, укажите:

1. **Описание проблемы** - что произошло
2. **Ожидаемое поведение** - что должно было быть
3. **Шаги воспроизведения** - как повторить баг
4. **Окружение**:
   - OS: Linux/macOS/Windows
   - Python версия: 3.10/3.11/3.12
   - Версия NeuroGraph OS: 0.10.0
5. **Логи/Скриншоты** - если есть

**Пример:**
```markdown
### Описание
API возвращает 500 при создании токена с невалидными координатами

### Шаги воспроизведения
1. POST /api/v1/tokens
2. Передать координаты вне диапазона: {"l1_physical": {"x": 999999}}
3. Получить 500 ошибку

### Ожидаемое поведение
Должен вернуться 400 с описанием ошибки валидации

### Окружение
- OS: Ubuntu 22.04
- Python: 3.11
- Version: 0.10.0
```

---

## 💡 Предложение фич

При создании Issue для новой фичи, укажите:

1. **Проблема** - какую задачу решает фича
2. **Решение** - как вы предлагаете её решить
3. **Альтернативы** - другие варианты решения
4. **Приоритет** - насколько это важно

---

## 📚 Архитектура проекта

Перед началом разработки, ознакомьтесь с:

- [README.md](README.md) - общее описание
- [architecture_blueprint.json](architecture_blueprint.json) - архитектура
- [docs/token_extended_spec.md](docs/token_extended_spec.md) - спецификация Token v2.0

---

## 🤝 Code Review Process

После создания PR:

1. **Automated checks** - GitHub Actions запустит тесты
2. **Code review** - мейнтейнеры проверят код
3. **Discussion** - обсуждение изменений в комментариях
4. **Approval** - получение approval от мейнтейнеров
5. **Merge** - слияние в main ветку

**Время ответа:**
- Первая реакция: 1-3 дня
- Полный review: 3-7 дней

---

## 🎖️ Признание вклада

Все контрибьюторы будут добавлены в:
- GitHub Contributors list
- CONTRIBUTORS.md (планируется)
- Release notes

---

## ❓ Вопросы?

- **GitHub Issues** - для багов и фич
- **GitHub Discussions** - для общих вопросов
- **Email**: <dreeftwood@gmail.com> - для приватных вопросов

---

## 📜 Лицензия

Внося вклад в проект, вы соглашаетесь что ваш код будет распространяться под [MIT License](LICENSE).

---

Спасибо за вклад в NeuroGraph OS! ⚡
