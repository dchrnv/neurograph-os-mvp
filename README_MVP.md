# NeuroGraph OS - MVP

> **Token-based spatial computing system with 8 semantic coordinate spaces**

[![Version](https://img.shields.io/badge/version-0.10.0-blue.svg)](https://github.com/dchrnv/neurograph-os-dev)
[![Python](https://img.shields.io/badge/python-3.10+-blue.svg)](https://www.python.org/downloads/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

---

## 🎯 Что это?

**NeuroGraph OS** — экспериментальная когнитивная архитектура, основанная на:

- **Token v2.0**: Атомарная 64-байтная единица информации
- **8 семантических пространств**: Physical, Sensory, Motor, Emotional, Cognitive, Social, Temporal, Abstract
- **Карта мира**: Токены + Сетка + Граф (без ИИ в MVP)
- **CDNA**: Геном системы - правила валидации

---

## 🚀 Быстрый старт

### 1. Клонируйте репозиторий

```bash
git clone https://github.com/dchrnv/neurograph-os-dev.git
cd neurograph-os-dev
```

### 2. Создайте виртуальное окружение

```bash
python3 -m venv .venv
source .venv/bin/activate  # Linux/macOS
# .venv\Scripts\activate  # Windows
```

### 3. Установите зависимости

```bash
pip install -r requirements.txt
```

### 4. Запустите MVP API

```bash
# Используйте скрипт
./run_mvp.sh

# Или вручную
python src/api_mvp/main.py
```

API будет доступен на `http://localhost:8000`

- 📖 **Документация**: http://localhost:8000/docs
- 💚 **Health check**: http://localhost:8000/health
- 🎯 **API info**: http://localhost:8000/api

---

## 🎨 Дашборд (опционально)

Для запуска красивого React дашборда:

### Предварительные требования

- Node.js 18+ и npm

### Установка

```bash
cd ui/web
npm install
npm run dev
```

Дашборд будет доступен на `http://localhost:3000`

**Особенности дашборда:**

- ⚡ Киберпанк дизайн
- 📊 Мониторинг токенов в реальном времени
- 🎛️ Управление токенами (создание, удаление)
- 📈 Статистика системы

---

## 📖 Основные концепции

### Token v2.0

64-байтная структура с 8 уровнями координат:

```python
from src.core.token.token_v2 import Token, create_token_id

# Создать токен
token_id = create_token_id(local_id=42, entity_type=1, domain=0)
token = Token(id=token_id)

# Установить координаты L1 (Physical)
token.set_coordinates(level=0, x=10.5, y=20.3, z=1.5)

# Установить координаты L4 (Emotional - VAD модель)
token.set_coordinates(level=3, x=0.8, y=0.5, z=0.3)  # Joy

# Сериализация
binary_data = token.pack()  # 64 bytes
token2 = Token.unpack(binary_data)
```

### 8 семантических пространств

| Уровень | Название | Назначение | Примеры осей |
|---------|----------|------------|--------------|
| **L1** | Physical | Физическое 3D пространство | X, Y, Z (метры) |
| **L2** | Sensory | Сенсорное восприятие | Салиентность, Валентность, Новизна |
| **L3** | Motor | Моторика/движение | Скорость, Ускорение, Угловая скорость |
| **L4** | Emotional | Эмоции (VAD модель) | Valence, Arousal, Dominance |
| **L5** | Cognitive | Когнитивная обработка | Нагрузка, Абстракция, Уверенность |
| **L6** | Social | Социальное взаимодействие | Дистанция, Статус, Принадлежность |
| **L7** | Temporal | Временная локализация | Смещение, Длительность, Частота |
| **L8** | Abstract | Семантика и логика | Близость, Каузальность, Модальность |

Подробнее: [docs/token_extended_spec.md](docs/token_extended_spec.md)

---

## 🔧 API Примеры

### Создать токен

```bash
curl -X POST http://localhost:8000/api/v1/tokens \
  -H "Content-Type: application/json" \
  -d '{
    "entity_type": 1,
    "domain": 0,
    "weight": 0.7,
    "persistent": true,
    "l1_physical": {"x": 10.5, "y": 20.3, "z": 1.5},
    "l4_emotional": {"x": 0.8, "y": 0.5, "z": 0.3}
  }'
```

### Получить все токены

```bash
curl http://localhost:8000/api/v1/tokens
```

### Создать примеры токенов

```bash
curl -X POST http://localhost:8000/api/v1/tokens/examples/create
```

### Health check

```bash
curl http://localhost:8000/health
```

---

## 📁 Структура проекта (MVP)

```
neurograph-os-dev/
├── src/
│   ├── core/
│   │   └── token/
│   │       ├── token_v2.py       # Token v2.0 (64 bytes, 8 spaces)
│   │       └── tests/
│   │           └── test_token_v2.py
│   │
│   └── api_mvp/
│       └── main.py               # MVP FastAPI server
│
├── ui/
│   └── web/                      # React Dashboard
│       ├── src/
│       │   ├── App.tsx           # Main component
│       │   └── styles/
│       │       └── index.css     # Cyberpunk styling
│       └── package.json
│
├── docs/
│   ├── token_extended_spec.md    # Token v2.0 specification
│   └── configuration_structure.md
│
├── config/
│   └── specs/
│       └── graph_cdna_rules.json # CDNA validation rules
│
├── requirements.txt              # Python dependencies
├── setup.py                      # Package setup
├── run_mvp.sh                    # Quick start script
└── README_MVP.md                 # This file
```

---

## 🧪 Тестирование

### Запуск тестов Token v2.0

```bash
source .venv/bin/activate
python -m pytest src/core/token/tests/test_token_v2.py -v
```

### Ручное тестирование

```bash
source .venv/bin/activate
python -c "
from src.core.token.token_v2 import Token, create_token_id

token = Token(id=create_token_id(1, 5, 0))
token.set_coordinates(0, x=10.0, y=20.0, z=5.0)
print(f'Token created: {token}')
print(f'Packed size: {len(token.pack())} bytes')
"
```

---

## 🎯 Что НЕ входит в MVP

Эти компоненты будут добавлены позже:

- ❌ Мини-нейронки (модули обработки)
- ❌ Experience Stream (сбор опыта для RL)
- ❌ Интуиция (Intuition Engine)
- ❌ Персистентность в БД (пока in-memory)
- ❌ WebSocket real-time обновления
- ❌ CLI (командная строка)

**MVP фокус:**
- ✅ Token v2.0 (64 bytes, 8 spaces)
- ✅ RESTful API
- ✅ React Dashboard
- ✅ In-memory хранилище
- ✅ CDNA правила валидации

---

## 🛠️ Технологии

| Категория | Технология |
|-----------|------------|
| **Backend** | Python 3.10+, FastAPI, Pydantic v2 |
| **Frontend** | React 18, TypeScript, Vite |
| **Data** | Numpy (координаты), In-memory storage |
| **Dev** | pytest, black, isort |

---

## 📝 Roadmap

### v0.11 (Next)

- [ ] Граф токенов (GraphEngine)
- [ ] CDNA валидатор
- [ ] Базовая визуализация графа

### v0.12

- [ ] Персистентность (PostgreSQL)
- [ ] WebSocket для real-time
- [ ] Experience Stream

### v1.0

- [ ] Мини-нейронки
- [ ] Полная интеграция с геномом
- [ ] Production-ready deployment

---

## 🤝 Участие в разработке

1. Fork репозитория
2. Создайте ветку: `git checkout -b feature/amazing-feature`
3. Commit изменения: `git commit -m 'Add amazing feature'`
4. Push в ветку: `git push origin feature/amazing-feature`
5. Создайте Pull Request

---

## 📄 Лицензия

MIT License - см. [LICENSE](LICENSE)

---

## 🙏 Благодарности

Проект создан как экспериментальная платформа для исследования:

- Token-based computing
- Spatial intelligence
- Multi-dimensional semantic spaces
- Cognitive architectures

---

## 📧 Контакты

**Автор**: Chernov Denys
**Email**: dreeftwood@gmail.com
**GitHub**: [dchrnv/neurograph-os-dev](https://github.com/dchrnv/neurograph-os-dev)

---

Made with ⚡ by NeuroGraph OS Team
