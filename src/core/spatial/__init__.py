# src/core/spatial/__init__.py
"""
Пространственные компоненты NeuroGraph OS

Многомерная система координат и разреженная сетка для размещения токенов
в 8-мерном континууме состояний.
"""

from .coordinates import (
    Point3D, Region3D, MultiCoordinate,
    LevelConfig, CoordinateConfig, IndexType,
    LevelStats, SystemStats
)

from .spatial_index import (
    SpatialIndex, SparseGridIndex, SpatialHashIndex
)

from .coordinate_system import CoordinateSystem

from .sparse_grid import (
    SparseGrid, SparseGridBuilder, create_demo_sparse_grid
)

# Версия модуля
__version__ = "1.0.0"

# Публичный API
__all__ = [
    # Основные классы
    "SparseGrid",
    "CoordinateSystem",
    
    # Строители
    "SparseGridBuilder",
    "create_demo_sparse_grid",
    
    # Структуры данных
    "Point3D",
    "Region3D", 
    "MultiCoordinate",
    
    # Конфигурация
    "LevelConfig",
    "CoordinateConfig",
    "IndexType",
    
    # Статистика
    "LevelStats",
    "SystemStats",
    
    # Индексы
    "SpatialIndex",
    "SparseGridIndex",
    "SpatialHashIndex"
]
