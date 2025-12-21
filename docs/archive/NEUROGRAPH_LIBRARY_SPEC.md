# neurograph — Python Library Specification

**Версия:** 1.0.0  
**Дата:** 2025-01-XX  
**Статус:** Спецификация  
**Зависимости:** neurograph_core (Rust FFI)

---

## Обзор

`neurograph` — высокоуровневая Python библиотека для работы с семантическими графами знаний Tiro. Построена поверх `neurograph_core` (Rust FFI) и предоставляет удобный API для:

- Управления жизненным циклом системы (Runtime)
- Выполнения семантических запросов (Query)
- Загрузки начальных данных (Bootstrap)
- Интеграции с Jupyter и другими инструментами

### Принципы

```
1. Простота использования — 3 строки для старта
2. Производительность — делегирование тяжёлых операций в Rust
3. Расширяемость — плагины и хуки
4. Документированность — docstrings везде
```

---

## Быстрый старт

```python
import neurograph as ng

# Инициализация
runtime = ng.Runtime()

# Загрузка базовых знаний
runtime.bootstrap("glove.6B.50d.txt", limit=50000)

# Запрос
result = runtime.query("кошка")
print(result.top(5))
# [('кот', 0.92), ('котёнок', 0.87), ('животное', 0.81), ...]

# Обратная связь
runtime.feedback(result.signal_id, "positive")
```

---

## Архитектура модулей

```
neurograph/
├── __init__.py          # Публичный API
├── runtime.py           # Runtime Manager
├── query.py             # Query Engine
├── bootstrap.py         # Bootstrap Loader
├── feedback.py          # Feedback System
├── metrics.py           # Metrics Collector
├── config.py            # Configuration
├── exceptions.py        # Custom Exceptions
├── types.py             # Type Definitions
├── utils.py             # Helpers
│
├── integrations/
│   ├── __init__.py
│   ├── jupyter.py       # Jupyter magic commands
│   └── pandas.py        # DataFrame integration
│
└── _core.py             # neurograph_core wrapper
```

---

## Runtime Manager

### Класс `Runtime`

Центральный класс для управления системой.

```python
class Runtime:
    """
    Менеджер жизненного цикла Tiro.
    
    Управляет инициализацией, конфигурацией и координацией
    всех компонентов системы.
    
    Example:
        >>> runtime = ng.Runtime()
        >>> runtime.bootstrap("embeddings.txt")
        >>> runtime.start()
        >>> result = runtime.query("hello")
    """
    
    def __init__(
        self,
        config: Optional[RuntimeConfig] = None,
        *,
        auto_start: bool = True,
        enable_metrics: bool = True,
        enable_persistence: bool = False
    ) -> None:
        """
        Инициализация Runtime.
        
        Args:
            config: Конфигурация. None = defaults.
            auto_start: Автоматический запуск после bootstrap.
            enable_metrics: Включить сбор метрик.
            enable_persistence: Включить персистентность (WAL).
        """
        ...
    
    # === Жизненный цикл ===
    
    def start(self) -> None:
        """Запуск всех систем."""
        ...
    
    def stop(self) -> None:
        """Остановка всех систем."""
        ...
    
    def restart(self) -> None:
        """Перезапуск системы."""
        ...
    
    @property
    def is_running(self) -> bool:
        """Статус работы системы."""
        ...
    
    # === Основные операции ===
    
    def query(
        self,
        text: str,
        *,
        limit: int = 10,
        threshold: float = 0.5,
        spaces: Optional[List[str]] = None
    ) -> QueryResult:
        """
        Семантический запрос.
        
        Args:
            text: Текст запроса.
            limit: Максимум результатов.
            threshold: Минимальный порог релевантности.
            spaces: Список пространств для поиска (None = все).
            
        Returns:
            QueryResult с найденными токенами и связями.
            
        Example:
            >>> result = runtime.query("машина", limit=5)
            >>> for token, score in result.top(3):
            ...     print(f"{token}: {score:.2f}")
            автомобиль: 0.89
            транспорт: 0.76
            двигатель: 0.71
        """
        ...
    
    def feedback(
        self,
        signal_id: int,
        feedback_type: Literal["positive", "negative", "correction"],
        correction: Optional[str] = None
    ) -> None:
        """
        Обратная связь на результат.
        
        Args:
            signal_id: ID сигнала из QueryResult.
            feedback_type: Тип обратной связи.
            correction: Текст коррекции (для feedback_type="correction").
        """
        ...
    
    # === Bootstrap ===
    
    def bootstrap(
        self,
        source: Union[str, Path, "BootstrapConfig"],
        *,
        limit: Optional[int] = None,
        progress: bool = True
    ) -> BootstrapResult:
        """
        Загрузка начальных данных.
        
        Args:
            source: Путь к файлу эмбеддингов или конфиг.
            limit: Ограничение количества токенов.
            progress: Показывать прогресс-бар.
            
        Returns:
            BootstrapResult со статистикой загрузки.
        """
        ...
    
    # === Системная информация ===
    
    def status(self) -> RuntimeStatus:
        """Текущий статус системы."""
        ...
    
    def stats(self) -> RuntimeStats:
        """Статистика работы."""
        ...
    
    def health(self) -> HealthCheck:
        """Проверка здоровья системы."""
        ...
    
    # === Конфигурация ===
    
    @property
    def config(self) -> RuntimeConfig:
        """Текущая конфигурация."""
        ...
    
    def update_config(self, **kwargs) -> None:
        """Обновление конфигурации в runtime."""
        ...
    
    # === Контекстный менеджер ===
    
    def __enter__(self) -> "Runtime":
        ...
    
    def __exit__(self, *args) -> None:
        self.stop()
```

### RuntimeConfig

```python
@dataclass
class RuntimeConfig:
    """Конфигурация Runtime."""
    
    # Core
    max_tokens: int = 1_000_000
    max_connections: int = 10_000_000
    
    # Gateway
    queue_capacity: int = 10_000
    processing_timeout_ms: int = 100
    
    # IntuitionEngine
    enable_fast_path: bool = True
    fast_path_threshold: float = 0.8
    
    # Persistence
    wal_enabled: bool = False
    wal_path: Optional[Path] = None
    checkpoint_interval_sec: int = 300
    
    # Metrics
    metrics_enabled: bool = True
    metrics_interval_ms: int = 1000
    
    # Logging
    log_level: str = "INFO"
    
    @classmethod
    def from_file(cls, path: Union[str, Path]) -> "RuntimeConfig":
        """Загрузка из YAML/JSON файла."""
        ...
    
    @classmethod
    def from_env(cls) -> "RuntimeConfig":
        """Загрузка из переменных окружения."""
        ...
```

### RuntimeStatus

```python
@dataclass
class RuntimeStatus:
    """Статус системы."""
    
    state: Literal["stopped", "starting", "running", "stopping", "error"]
    uptime_seconds: float
    
    # Counts
    total_tokens: int
    active_tokens: int
    total_connections: int
    active_connections: int
    
    # Health
    memory_usage_mb: float
    cpu_usage_percent: float
    
    # Components
    components: Dict[str, ComponentStatus]
    
    def __str__(self) -> str:
        return (
            f"Runtime[{self.state}] "
            f"tokens={self.total_tokens:,} "
            f"mem={self.memory_usage_mb:.1f}MB"
        )
```

---

## Query Engine

### QueryResult

```python
@dataclass
class QueryResult:
    """Результат семантического запроса."""
    
    signal_id: int
    query_text: str
    processing_time_ms: float
    
    # Results
    tokens: List[TokenMatch]
    connections: List[ConnectionMatch]
    
    # Metadata
    total_candidates: int
    confidence: float
    interpretation: str
    
    def top(self, n: int = 10) -> List[Tuple[str, float]]:
        """
        Топ-N результатов.
        
        Returns:
            Список кортежей (label, score).
        """
        return [(t.label, t.score) for t in self.tokens[:n]]
    
    def to_dict(self) -> Dict[str, Any]:
        """Преобразование в словарь."""
        ...
    
    def to_dataframe(self) -> "pd.DataFrame":
        """Преобразование в pandas DataFrame."""
        import pandas as pd
        return pd.DataFrame([
            {"label": t.label, "score": t.score, "type": t.entity_type}
            for t in self.tokens
        ])
    
    def visualize(self, **kwargs) -> None:
        """Визуализация результатов (для Jupyter)."""
        from .integrations.jupyter import visualize_result
        visualize_result(self, **kwargs)


@dataclass
class TokenMatch:
    """Совпавший токен."""
    
    token_id: int
    label: str
    score: float
    entity_type: str
    coordinates: Dict[str, Tuple[float, float, float]]
    
    # Context
    connected_tokens: List[int]
    activation_path: List[int]


@dataclass  
class ConnectionMatch:
    """Релевантная связь."""
    
    connection_id: int
    source_id: int
    target_id: int
    connection_type: str
    strength: float
```

### Примеры запросов

```python
# Базовый запрос
result = runtime.query("программирование")

# С фильтрацией по пространству
result = runtime.query(
    "радость",
    spaces=["L4Emotional"],
    threshold=0.7
)

# Запрос с контекстом
result = runtime.query(
    "банк",
    context=["финансы", "деньги"]  # disambiguation
)

# Цепочка запросов
chain = (
    runtime.query("кошка")
    .expand("related")  # расширить по связям
    .filter(entity_type="Concept")
    .limit(20)
)
```

---

## Bootstrap Loader

### BootstrapConfig

```python
@dataclass
class BootstrapConfig:
    """Конфигурация загрузки начальных данных."""
    
    # Source
    embeddings_path: Path
    embeddings_format: Literal["glove", "word2vec", "fasttext"] = "glove"
    
    # Limits
    max_tokens: Optional[int] = None
    min_frequency: Optional[int] = None
    
    # Projection
    projection_method: Literal["pca", "umap", "random"] = "pca"
    target_dimensions: int = 8
    
    # Clustering
    enable_clustering: bool = True
    cluster_method: Literal["kmeans", "hdbscan"] = "kmeans"
    n_clusters: int = 100
    
    # Connections
    create_connections: bool = True
    connection_threshold: float = 0.7
    max_connections_per_token: int = 50
    
    # Processing
    batch_size: int = 10_000
    n_workers: int = 4
```

### BootstrapResult

```python
@dataclass
class BootstrapResult:
    """Результат загрузки."""
    
    tokens_loaded: int
    connections_created: int
    clusters_formed: int
    
    processing_time_sec: float
    memory_peak_mb: float
    
    # Details
    skipped_tokens: int
    skipped_reasons: Dict[str, int]
    
    # Quality
    coverage: float  # % покрытия словаря
    avg_connections: float  # среднее связей на токен
    
    def summary(self) -> str:
        """Человекочитаемая сводка."""
        return (
            f"Loaded {self.tokens_loaded:,} tokens, "
            f"{self.connections_created:,} connections "
            f"in {self.processing_time_sec:.1f}s"
        )
```

### Примеры bootstrap

```python
# Простой bootstrap
runtime.bootstrap("glove.6B.50d.txt")

# С ограничением
runtime.bootstrap("glove.6B.300d.txt", limit=100_000)

# Полная конфигурация
config = ng.BootstrapConfig(
    embeddings_path=Path("embeddings/glove.6B.300d.txt"),
    max_tokens=500_000,
    projection_method="umap",
    enable_clustering=True,
    n_clusters=500
)
result = runtime.bootstrap(config)
print(result.summary())
```

---

## Metrics

### MetricsSnapshot

```python
@dataclass
class MetricsSnapshot:
    """Снимок метрик в момент времени."""
    
    timestamp: float
    
    # Performance
    events_per_second: float
    avg_latency_us: float
    p99_latency_us: float
    
    # Resources
    memory_usage_mb: float
    cpu_usage_percent: float
    
    # Counts
    total_queries: int
    total_feedbacks: int
    cache_hit_rate: float
    
    # Internals
    active_tokens: int
    active_connections: int
    intuition_hits: int
    intuition_misses: int


class MetricsCollector:
    """Сборщик метрик."""
    
    def __init__(self, runtime: Runtime) -> None:
        ...
    
    def snapshot(self) -> MetricsSnapshot:
        """Текущий снимок метрик."""
        ...
    
    def history(
        self,
        minutes: int = 60
    ) -> List[MetricsSnapshot]:
        """История метрик за период."""
        ...
    
    def export_prometheus(self) -> str:
        """Экспорт в формате Prometheus."""
        ...
```

---

## Integrations

### Jupyter

```python
# neurograph/integrations/jupyter.py

def load_ipython_extension(ipython):
    """Загрузка расширения IPython."""
    from .magic import NeurographMagics
    ipython.register_magics(NeurographMagics)


class NeurographMagics(Magics):
    """Magic commands для Jupyter."""
    
    @line_magic
    def ng_status(self, line):
        """Статус системы: %ng_status"""
        ...
    
    @line_magic
    def ng_query(self, line):
        """Быстрый запрос: %ng_query кошка"""
        ...
    
    @cell_magic
    def ng_explore(self, line, cell):
        """Интерактивное исследование."""
        ...


def visualize_result(
    result: QueryResult,
    *,
    style: str = "graph",
    max_nodes: int = 50
) -> None:
    """
    Визуализация результата в Jupyter.
    
    Args:
        result: Результат запроса.
        style: "graph", "tree", "table", "heatmap"
        max_nodes: Максимум узлов для отображения.
    """
    from IPython.display import display, HTML
    import matplotlib.pyplot as plt
    import networkx as nx
    
    if style == "graph":
        _visualize_graph(result, max_nodes)
    elif style == "table":
        display(result.to_dataframe())
    ...
```

### Pandas

```python
# neurograph/integrations/pandas.py

def tokens_to_dataframe(tokens: List[Token]) -> "pd.DataFrame":
    """Конвертация токенов в DataFrame."""
    import pandas as pd
    return pd.DataFrame([
        {
            "id": t.id,
            "label": t.label,
            "weight": t.weight,
            "entity_type": t.entity_type,
            "x_physical": t.coordinates.get("L1Physical", (0,0,0))[0],
            "y_physical": t.coordinates.get("L1Physical", (0,0,0))[1],
            "z_physical": t.coordinates.get("L1Physical", (0,0,0))[2],
            # ... остальные координаты
        }
        for t in tokens
    ])


def dataframe_to_tokens(df: "pd.DataFrame") -> List[Token]:
    """Создание токенов из DataFrame."""
    ...
```

---

## Exceptions

```python
# neurograph/exceptions.py

class NeurographError(Exception):
    """Базовое исключение."""
    pass


class RuntimeError(NeurographError):
    """Ошибка Runtime."""
    pass


class NotRunningError(RuntimeError):
    """Система не запущена."""
    pass


class BootstrapError(NeurographError):
    """Ошибка bootstrap."""
    pass


class QueryError(NeurographError):
    """Ошибка запроса."""
    pass


class ConfigurationError(NeurographError):
    """Ошибка конфигурации."""
    pass


class ValidationError(NeurographError):
    """Ошибка валидации."""
    pass
```

---

## Публичный API

```python
# neurograph/__init__.py

"""
neurograph - Semantic Knowledge Graph Library
=============================================

High-level Python interface for Tiro cognitive system.

Quick Start:
    >>> import neurograph as ng
    >>> runtime = ng.Runtime()
    >>> runtime.bootstrap("embeddings.txt")
    >>> result = runtime.query("hello")
    >>> print(result.top(5))
"""

from .runtime import Runtime, RuntimeConfig, RuntimeStatus
from .query import QueryResult, TokenMatch, ConnectionMatch
from .bootstrap import BootstrapConfig, BootstrapResult
from .feedback import FeedbackType
from .metrics import MetricsSnapshot, MetricsCollector
from .config import Config
from .exceptions import (
    NeurographError,
    RuntimeError,
    NotRunningError,
    BootstrapError,
    QueryError,
    ConfigurationError,
    ValidationError
)

# Re-export from core
from neurograph_core import (
    Token,
    Connection,
    CoordinateSpace,
    EntityType,
    ConnectionType
)

__version__ = "1.0.0"
__all__ = [
    # Runtime
    "Runtime",
    "RuntimeConfig", 
    "RuntimeStatus",
    
    # Query
    "QueryResult",
    "TokenMatch",
    "ConnectionMatch",
    
    # Bootstrap
    "BootstrapConfig",
    "BootstrapResult",
    
    # Feedback
    "FeedbackType",
    
    # Metrics
    "MetricsSnapshot",
    "MetricsCollector",
    
    # Config
    "Config",
    
    # Core types
    "Token",
    "Connection",
    "CoordinateSpace",
    "EntityType",
    "ConnectionType",
    
    # Exceptions
    "NeurographError",
    "RuntimeError",
    "NotRunningError",
    "BootstrapError",
    "QueryError",
    "ConfigurationError",
    "ValidationError",
]


def version() -> str:
    """Версия библиотеки."""
    return __version__


def info() -> dict:
    """Информация о библиотеке и системе."""
    import neurograph_core
    return {
        "neurograph_version": __version__,
        "core_version": neurograph_core.__version__,
        "python_version": sys.version,
    }
```

---

## Установка и зависимости

### pyproject.toml

```toml
[project]
name = "neurograph"
version = "1.0.0"
description = "High-level Python library for Tiro semantic knowledge graphs"
readme = "README.md"
requires-python = ">=3.9"
license = {text = "AGPL-3.0"}
authors = [
    {name = "Denis Chernov", email = "dreeftwood@gmail.com"}
]
classifiers = [
    "Development Status :: 4 - Beta",
    "Intended Audience :: Developers",
    "Intended Audience :: Science/Research",
    "License :: OSI Approved :: GNU Affero General Public License v3",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.9",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
    "Topic :: Scientific/Engineering :: Artificial Intelligence",
]

dependencies = [
    "neurograph-core>=0.41.0",  # Rust FFI bindings
    "numpy>=1.24.0",
    "pydantic>=2.0.0",
    "pyyaml>=6.0.0",
    "rich>=13.0.0",  # Pretty output
]

[project.optional-dependencies]
jupyter = [
    "ipython>=8.0.0",
    "matplotlib>=3.7.0",
    "networkx>=3.0.0",
]
pandas = [
    "pandas>=2.0.0",
]
ml = [
    "scikit-learn>=1.3.0",
    "umap-learn>=0.5.0",
]
all = [
    "neurograph[jupyter,pandas,ml]",
]
dev = [
    "pytest>=7.0.0",
    "pytest-asyncio>=0.21.0",
    "pytest-cov>=4.0.0",
    "mypy>=1.0.0",
    "ruff>=0.1.0",
]

[project.urls]
Homepage = "https://github.com/dchrnv/tiro"
Documentation = "https://tiro.readthedocs.io"
Repository = "https://github.com/dchrnv/tiro"
```

---

## Примеры использования

### Пример 1: Базовый workflow

```python
import neurograph as ng

# Создание и инициализация
runtime = ng.Runtime(enable_persistence=True)

# Bootstrap
result = runtime.bootstrap(
    "data/glove.6B.100d.txt",
    limit=50_000,
    progress=True
)
print(result.summary())
# Loaded 50,000 tokens, 1,247,832 connections in 12.3s

# Запросы
for word in ["программист", "музыка", "счастье"]:
    result = runtime.query(word, limit=5)
    print(f"\n{word}:")
    for label, score in result.top(5):
        print(f"  {label}: {score:.2f}")
```

### Пример 2: Jupyter notebook

```python
# Cell 1: Setup
%load_ext neurograph

import neurograph as ng
runtime = ng.Runtime()
runtime.bootstrap("embeddings.txt", limit=10000)

# Cell 2: Quick query
%ng_query кошка

# Cell 3: Visualization
result = runtime.query("любовь")
result.visualize(style="graph", max_nodes=30)

# Cell 4: DataFrame analysis
df = result.to_dataframe()
df.head(10)
```

### Пример 3: Контекстный менеджер

```python
import neurograph as ng

with ng.Runtime() as runtime:
    runtime.bootstrap("embeddings.txt")
    
    # Работа с системой
    result = runtime.query("hello")
    runtime.feedback(result.signal_id, "positive")
    
    # Автоматическое закрытие при выходе
```

---

## Следующий документ

→ **REST_API_SPEC.md** — HTTP endpoints для всех операций

---

## Changelog

| Версия | Дата | Изменения |
|--------|------|-----------|
| 1.0.0 | 2025-01-XX | Начальная версия |

---

**neurograph Library Specification v1.0.0**  
*Высокоуровневый Python API для Tiro*
