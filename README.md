# NeuroGraph OS - MVP

> **Token-based spatial computing system with 8 semantic coordinate spaces**

[![Version](https://img.shields.io/badge/version-0.16.0_mvp__Graph-blue.svg)](https://github.com/dchrnv/neurograph-os-mvp)
[![Python](https://img.shields.io/badge/python-3.10+-blue.svg)](https://www.python.org/downloads/)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
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
git clone https://github.com/dchrnv/neurograph-os-mvp.git
cd neurograph-os-mvp
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

## 🦀 Rust Core

**High-performance Rust implementation** - Token V2.0 + Connection V1.0, 100× faster, zero dependencies!

### Quick Start (Rust)

```bash
# Install Rust (one-time)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Build and test
cd src/core_rust
./setup_and_test.sh
```

### Features

**Token V2.0 (64 bytes):**

- ✅ 8-dimensional coordinate system
- ✅ Type-safe entity types
- ✅ Field properties (radius, strength)
- ✅ 12+ unit tests

**Connection V1.0 (32 bytes):**

- ✅ 40+ connection types (11 categories)
- ✅ Physical force model (attraction/repulsion)
- ✅ 8-level selective activation
- ✅ Lifecycle tracking
- ✅ 10+ unit tests

**Grid V2.0:**

- ✅ 8-dimensional spatial indexing
- ✅ Bucket-based fast lookups
- ✅ KNN search (K-Nearest Neighbors)
- ✅ Range queries (find all within radius)
- ✅ Field influence calculations
- ✅ Density calculations
- ✅ 6+ unit tests

**Graph V2.0 (NEW in v0.16.0):**

- ✅ Topological navigation and pathfinding
- ✅ Adjacency lists for O(1) neighbor access
- ✅ BFS/DFS traversal with iterators
- ✅ Shortest path (BFS) and weighted paths (Dijkstra)
- ✅ Subgraph extraction (induced + ego-networks)
- ✅ Directed/undirected edge support
- ✅ 10+ unit tests

**Performance:**

- ✅ Zero dependencies - Pure Rust
- ✅ 100× faster than Python
- ✅ Zero-copy serialization
- ✅ Cache-friendly structures

### Usage Examples (Rust)

**Token:**

```rust
use neurograph_core::{Token, CoordinateSpace, EntityType, token_flags};

// Create token
let mut token = Token::new(Token::create_id(12345, 0, 0));

// Set coordinates (precision: x.xx for proper encoding)
token.set_coordinates(CoordinateSpace::L1Physical, 10.50, 20.30, 5.20);
token.set_coordinates(CoordinateSpace::L4Emotional, 0.80, 0.60, 0.50);

// Configure
token.set_entity_type(EntityType::Concept);
token.set_flag(token_flags::PERSISTENT);
token.weight = 0.75;

// Serialize (zero-copy)
let bytes = token.to_bytes();  // [u8; 64]
```

**Connection:**

```rust
use neurograph_core::{Connection, ConnectionType, active_levels, connection_flags};

// Create connection between tokens
let mut conn = Connection::new(token_a_id, token_b_id);

// Set type and parameters
conn.set_connection_type(ConnectionType::Cause);
conn.set_rigidity(0.85);
conn.pull_strength = 0.70;  // Attraction
conn.preferred_distance = 1.50;

// Activate on specific spaces
conn.active_levels = active_levels::COGNITIVE_ABSTRACT;
conn.set_flag(connection_flags::PERSISTENT);

// Use the connection
conn.activate();  // Increments counter, updates timestamp

// Serialize (zero-copy)
let bytes = conn.to_bytes();  // [u8; 32]
```

**Grid:**

```rust
use neurograph_core::{Grid, Token, CoordinateSpace};

// Create grid
let mut grid = Grid::new();

// Add tokens
let mut token = Token::new(42);
token.set_coordinates(CoordinateSpace::L1Physical, 10.50, 20.30, 5.20);
grid.add(token).unwrap();

// Find neighbors
let neighbors = grid.find_neighbors(42, CoordinateSpace::L1Physical, 10.0, 5);
for (id, distance) in neighbors {
    println!("Token {}: distance = {:.2}", id, distance);
}

// Range query
let results = grid.range_query(CoordinateSpace::L1Physical, 0.0, 0.0, 0.0, 15.0);

// Field influence
let influence = grid.calculate_field_influence(
    CoordinateSpace::L1Physical, 10.0, 20.0, 5.0, 10.0
);

// === GRAPH V2.0 - Topological Navigation ===

use neurograph_core::{Graph, Direction};

let mut graph = Graph::new();

// Add nodes
graph.add_node(1);
graph.add_node(2);
graph.add_node(3);

// Add edges
let edge_id = Graph::compute_edge_id(1, 2, 0);
graph.add_edge(edge_id, 1, 2, 0, 1.0, false)?;

// Find neighbors
let neighbors = graph.get_neighbors(1, Direction::Outgoing);

// Find shortest path
let path = graph.find_path(1, 3)?;
println!("Path length: {}", path.length);

// BFS traversal
graph.bfs(1, Some(3), |node_id, depth| {
    println!("Visited node {} at depth {}", node_id, depth);
});

// Extract subgraph
let subgraph = graph.extract_neighborhood(2, 2);
```

**Documentation:**

- [Token V2 Rust Overview](TOKEN_V2_RUST.md) - Token implementation
- [Connection V1 Rust Overview](CONNECTION_V1_RUST.md) - Connection implementation
- [Grid V2 Rust Overview](GRID_V2_RUST.md) - Grid implementation
- [Graph V2 Rust Overview](GRAPH_V2_RUST.md) - Graph implementation (NEW in v0.16.0)
- [FFI Integration Guide](docs/FFI_INTEGRATION.md) - Python bindings
- [Rust API README](src/core_rust/README.md) - Full API docs
- [Installation Guide](src/core_rust/INSTALL.md) - Setup & troubleshooting

---

## 🐍 Python Bindings

**Rust performance with Python convenience!** Use the high-performance Rust core from Python with **10-100x speedup**.

### Quick Start (Python + Rust)

```bash
# Install maturin
pip install maturin

# Build and install Python bindings
cd src/core_rust
maturin develop --release --features python

# Verify installation
python -c "from neurograph import Token, Grid; print(Token(42), Grid())"
```

### Python FFI Features

- ✅ **Zero-copy serialization** - Instant to_bytes/from_bytes
- ✅ **10-100x faster** than pure Python
- ✅ **Complete API** - Token, Connection, Grid
- ✅ **Type-safe** - PyO3 automatic type conversion
- ✅ **Helper functions** - Convenience wrappers

### Usage Examples (Python)

**Token:**

```python
from neurograph import Token, CoordinateSpace, EntityType

# Create token
token = Token(42)
token.set_coordinates(CoordinateSpace.L1Physical(), 10.50, 20.30, 5.20)
token.set_coordinates(CoordinateSpace.L4Emotional(), 0.80, 0.60, 0.50)

# Configure
token.set_entity_type(EntityType.Concept())
token.weight = 2.50
token.set_active(True)

# Get coordinates
x, y, z = token.get_coordinates(CoordinateSpace.L1Physical())

# Serialize (zero-copy, instant!)
data = token.to_bytes()  # Returns 64 bytes
restored = Token.from_bytes(data)
```

**Connection:**

```python
from neurograph import Connection, ConnectionType

# Create connection
conn = Connection(1, 2, ConnectionType.Synonym())

# Configure
conn.pull_strength = 0.70  # Attraction
conn.preferred_distance = 1.50
conn.rigidity = 0.80
conn.set_bidirectional(True)

# Activate
conn.activate()
print(f"Activations: {conn.activation_count}")

# Calculate force (physics model)
force = conn.calculate_force(1.00)  # At distance 1.0m
```

**Grid:**

```python
from neurograph import Grid, GridConfig, Token, CoordinateSpace

# Create grid with custom config
config = GridConfig()
config.bucket_size = 20.0
grid = Grid(config)

# Add tokens
token = Token(42)
token.set_coordinates(CoordinateSpace.L1Physical(), 10.50, 20.30, 5.20)
grid.add(token)

# Find neighbors (space index: 0 = L1Physical)
neighbors = grid.find_neighbors(
    center_token_id=42,
    space=0,
    radius=10.0,
    max_results=5
)

for token_id, distance in neighbors:
    print(f"Token {token_id}: distance = {distance:.2f}")

# Range query
results = grid.range_query(space=0, x=0.0, y=0.0, z=0.0, radius=15.0)

# Field calculations
influence = grid.calculate_field_influence(space=0, x=10.0, y=20.0, z=5.0, radius=10.0)
density = grid.calculate_density(space=0, x=0.0, y=0.0, z=0.0, radius=5.0)
```

**Helper Functions:**

```python
from neurograph import create_emotional_token, create_semantic_connection, create_grid_with_tokens

# Emotional token (VAD model)
happy = create_emotional_token(1, valence=0.80, arousal=0.60, dominance=0.70)

# Semantic connection
conn = create_semantic_connection(
    1, 2,
    ConnectionType.Hypernym(),
    strength=0.90,
    bidirectional=False
)

# Grid with random tokens
grid, tokens = create_grid_with_tokens(num_tokens=100, space=0, spread=50.0)
```

### Performance Benchmarks

Run benchmarks to see the speedup:

```bash
cd src/core_rust
python examples/benchmark.py
```

**Typical results:**

- Token creation: **0.15 μs** (13x faster)
- Serialization: **0.03 μs** (100x faster)
- Distance calc: **0.12 μs** (29x faster)
- Connection ops: **0.07 μs** (14x faster)

### Examples

```bash
# Token & Connection usage
python src/core_rust/examples/python_usage.py

# Grid usage examples
python src/core_rust/examples/python_grid_usage.py

# Performance benchmarks
python src/core_rust/examples/benchmark.py
```

### Integration Example

**Token + Connection + Grid working together:**

```python
from neurograph import Token, Connection, Grid
from neurograph import CoordinateSpace, EntityType, ConnectionType

# Create grid
grid = Grid()

# Create tokens in semantic space
dog = Token(1)
dog.set_coordinates(CoordinateSpace.L8Abstract(), 0.0, 0.0, 0.0)
dog.set_entity_type(EntityType.Concept())
grid.add(dog)

cat = Token(2)
cat.set_coordinates(CoordinateSpace.L8Abstract(), 2.0, 1.0, 0.0)
cat.set_entity_type(EntityType.Concept())
grid.add(cat)

animal = Token(3)
animal.set_coordinates(CoordinateSpace.L8Abstract(), 1.0, 5.0, 0.0)
animal.set_entity_type(EntityType.Concept())
grid.add(animal)

# Create semantic connections
hypernym = Connection(1, 3, ConnectionType.Hypernym())  # dog -> animal
hypernym.pull_strength = 230  # 0.90
hypernym.set_active(True)

similar = Connection(1, 2, ConnectionType.Similar())   # dog <-> cat
similar.pull_strength = 178  # 0.70
similar.set_bidirectional(True)
similar.set_active(True)

# Spatial query: Find semantic neighbors of "dog"
neighbors = grid.find_neighbors(
    center_token_id=1,
    space=7,  # L8Abstract
    radius=3.0,
    max_results=10
)

print(f"Semantic neighbors: {[(id, f'{dist:.2f}') for id, dist in neighbors]}")
# Output: [(1, '0.00'), (2, '2.24')]
```

See [Integration Guide](docs/INTEGRATION_GUIDE.md) for more examples.

### Documentation

- [Integration Guide](docs/INTEGRATION_GUIDE.md) - Token + Connection + Grid + Graph
- [FFI Integration Guide](docs/FFI_INTEGRATION.md) - Complete Python API reference
- [v0.14.0 Release Notes](docs/V0.14.0_RELEASE_NOTES.md) - FFI Integration
- [v0.15.0 Release Notes](docs/V0.15.0_RELEASE_NOTES.md) - Grid V2.0
- [v0.16.0 Release Notes](docs/V0.16.0_RELEASE_NOTES.md) - Graph V2.0 (NEW)

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

```bash
neurograph-os-mvp/
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

### ✅ v0.10.0 - MVP (Completed)

- ✅ Token v2.0 Python (64 bytes, 8 spaces)
- ✅ FastAPI REST API
- ✅ React Dashboard (Cyberpunk UI)
- ✅ In-memory storage
- ✅ Documentation & guides

### ✅ v0.12.0 - Token Rust (Completed)

- ✅ Token V2.0 Rust implementation
- ✅ Zero dependencies (pure Rust)
- ✅ 100× performance vs Python
- ✅ 12+ unit tests
- ✅ Binary-compatible format

### ✅ v0.13.0 - Connection Rust (Completed)

- ✅ Connection V1.0 Rust implementation
- ✅ 40+ connection types (11 categories)
- ✅ Physical force model
- ✅ 8-level selective activation
- ✅ 10+ unit tests

### ✅ v0.14.0 - FFI & Integration (Completed)

- ✅ PyO3 FFI bindings (Rust ↔ Python)
- ✅ Python wrapper module (neurograph.py)
- ✅ Performance benchmarks (10-100x speedup)
- ✅ Complete Python API for Token & Connection
- ✅ Helper functions and examples
- ✅ Comprehensive documentation

### ✅ v0.15.0 - Grid Rust (Completed)

- ✅ Grid V2.0 Rust implementation
- ✅ 8-dimensional spatial indexing (bucket-based)
- ✅ Field physics (influence & density calculations)
- ✅ KNN and range queries
- ✅ Python FFI bindings
- ✅ 6+ unit tests
- ✅ Comprehensive examples and documentation

### ✅ v0.16.0 - Graph Rust (Completed)

- ✅ Graph V2.0 Rust implementation
- ✅ Topological indexing (adjacency lists)
- ✅ Traversal algorithms (BFS, DFS with iterators)
- ✅ Pathfinding (BFS shortest path + Dijkstra)
- ✅ Subgraph extraction (induced subgraphs + ego-networks)
- ✅ Python FFI bindings
- ✅ 10+ comprehensive unit tests
- ✅ Full API documentation

### 📋 v0.17.0 - Guardian & CDNA

- [ ] Guardian V1 Rust implementation
- [ ] CDNA V2 (384 bytes genome)
- [ ] Validation system
- [ ] Event orchestration
- [ ] Python FFI bindings

### 🔮 v1.0.0 - Production (Vision)

- [ ] Complete Rust core (Token + Connection + Grid + Graph + Guardian)
- [ ] Full Python FFI integration
- [ ] TypeScript bindings (NAPI-RS)
- [ ] PostgreSQL persistence
- [ ] WebSocket real-time
- [ ] Production deployment
- [ ] CLI tools
- [ ] Full test coverage (unit + integration)
- [ ] Performance optimization
- [ ] Full documentation

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
**GitHub**: [dchrnv/neurograph-os-mvp](https://github.com/dchrnv/neurograph-os-mvp)

---

Made with ⚡ by NeuroGraph OS Team
