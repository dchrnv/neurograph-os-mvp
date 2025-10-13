#!/usr/bin/env python3
"""
Система координат NeuroGraph OS

Многомерная пространственная структура для размещения и индексации токенов
в 8-мерном континууме состояний.
"""

import math
import time
from typing import Dict, List, Optional, Iterable, Any
from dataclasses import dataclass, field
from collections import defaultdict, deque
from abc import ABC, abstractmethod
from enum import Enum
import heapq
import threading
from functools import lru_cache

from ..token.token import Token

# === БАЗОВЫЕ СТРУКТУРЫ ДАННЫХ ===

@dataclass(frozen=True)
class Point3D:
    """Трехмерная точка в определенном уровне координат"""
    x: float
    y: float
    z: float
    level: int
    
    def distance_to(self, other: 'Point3D') -> float:
        """Евклидово расстояние до другой точки"""
        if self.level != other.level:
            raise ValueError("Нельзя вычислить расстояние между точками разных уровней")
        
        dx = self.x - other.x
        dy = self.y - other.y
        dz = self.z - other.z
        return math.sqrt(dx*dx + dy*dy + dz*dz)
    
    def manhattan_distance_to(self, other: 'Point3D') -> float:
        """Манхэттенское расстояние до другой точки"""
        if self.level != other.level:
            raise ValueError("Нельзя вычислить расстояние между точками разных уровней")
        
        return abs(self.x - other.x) + abs(self.y - other.y) + abs(self.z - other.z)
    
    def normalize_coordinates(self, precision: int = 3) -> 'Point3D':
        """Нормализует координаты с заданной точностью"""
        return Point3D(
            round(self.x, precision),
            round(self.y, precision),
            round(self.z, precision),
            self.level
        )

@dataclass
class Region3D:
    """Трехмерный регион (параллелепипед)"""
    min_point: Point3D
    max_point: Point3D
    
    def __post_init__(self):
        if self.min_point.level != self.max_point.level:
            raise ValueError("Точки региона должны быть на одном уровне")
    
    @property
    def level(self) -> int:
        return self.min_point.level
    
    def contains(self, point: Point3D) -> bool:
        """Проверяет, содержится ли точка в регионе"""
        if point.level != self.level:
            return False
        
        return (self.min_point.x <= point.x <= self.max_point.x and
                self.min_point.y <= point.y <= self.max_point.y and
                self.min_point.z <= point.z <= self.max_point.z)
    
    def intersects(self, other: 'Region3D') -> bool:
        """Проверяет пересечение с другим регионом"""
        if self.level != other.level:
            return False
        
        return not (self.max_point.x < other.min_point.x or
                   other.max_point.x < self.min_point.x or
                   self.max_point.y < other.min_point.y or
                   other.max_point.y < self.min_point.y or
                   self.max_point.z < other.min_point.z or
                   other.max_point.z < self.min_point.z)
    
    def volume(self) -> float:
        """Вычисляет объем региона"""
        dx = self.max_point.x - self.min_point.x
        dy = self.max_point.y - self.min_point.y
        dz = self.max_point.z - self.min_point.z
        return dx * dy * dz
    
    def center(self) -> Point3D:
        """Возвращает центр региона"""
        return Point3D(
            (self.min_point.x + self.max_point.x) / 2,
            (self.min_point.y + self.max_point.y) / 2,
            (self.min_point.z + self.max_point.z) / 2,
            self.level
        )

@dataclass
class MultiCoordinate:
    """Многомерная координата токена"""
    coordinates: Dict[int, Point3D] = field(default_factory=dict)
    primary_level: int = 0
    
    def get_point(self, level: int) -> Optional[Point3D]:
        """Получает точку для определенного уровня"""
        return self.coordinates.get(level)
    
    def set_point(self, point: Point3D) -> None:
        """Устанавливает точку для уровня"""
        self.coordinates[point.level] = point
    
    def get_active_levels(self) -> List[int]:
        """Возвращает список активных уровней"""
        return list(self.coordinates.keys())
    
    def distance_to(self, other: 'MultiCoordinate', 
                   levels: Optional[Iterable[int]] = None) -> Dict[int, float]:
        """Вычисляет расстояние до другой многомерной координаты"""
        if levels is None:
            levels_iter = set(self.coordinates.keys()) & set(other.coordinates.keys())
        else:
            levels_iter = levels

        distances: Dict[int, float] = {}
        for level in levels_iter:
            if level in self.coordinates and level in other.coordinates:
                distances[level] = self.coordinates[level].distance_to(other.coordinates[level])
        
        return distances

# === КОНФИГУРАЦИЯ ===

class IndexType(Enum):
    """Типы пространственных индексов"""
    SPARSE_GRID = "sparse_grid"
    SPATIAL_HASH = "spatial_hash"
    RTREE = "rtree"
    OCTREE = "octree"

@dataclass
class LevelConfig:
    """Конфигурация уровня координат"""
    name: str
    min_value: float
    max_value: float
    precision: int
    semantic_meaning: str
    index_type: IndexType = IndexType.SPARSE_GRID
    enable_spatial_tree: bool = False
    grid_resolution: float = 0.01
    cache_size: int = 1000
    
    def validate_coordinate(self, value: float) -> bool:
        """Проверяет, что координата в допустимых пределах"""
        return self.min_value <= value <= self.max_value
    
    def normalize_coordinate(self, value: float) -> float:
        """Нормализует координату"""
        clamped = max(self.min_value, min(self.max_value, value))
        return round(clamped, self.precision)

@dataclass
class CoordinateConfig:
    """Главная конфигурация системы координат"""
    precision: int = 3
    enable_validation: bool = True
    auto_indexing: bool = True
    cache_size: int = 10000
    enable_statistics: bool = True
    # Если True, CoordinateSystem будет пытаться автоматически
    # переиндексировать уровни при изменении CDNA масштабов.
    enable_auto_reindex: bool = False
    max_search_results: int = 10000
    levels: Dict[int, LevelConfig] = field(default_factory=dict)
    
    def get_default_levels(self) -> Dict[int, LevelConfig]:
        """Возвращает стандартные уровни координат"""
        return {
            0: LevelConfig("PHYSICAL", -327.67, 327.67, 2, "Физическое пространство"),
            1: LevelConfig("SENSORY", -1.0, 1.0, 3, "Сенсорное пространство"),
            2: LevelConfig("MOTOR", -10.0, 10.0, 2, "Моторное пространство"),
            3: LevelConfig("EMOTIONAL", -1.0, 1.0, 3, "Эмоциональное пространство"),
            4: LevelConfig("COGNITIVE", -1.0, 1.0, 3, "Когнитивное пространство"),
            5: LevelConfig("SOCIAL", -1.0, 1.0, 3, "Социальное пространство"),
            6: LevelConfig("TEMPORAL", -100.0, 100.0, 1, "Темпоральное пространство"),
            7: LevelConfig("ABSTRACT", -1.0, 1.0, 3, "Абстрактное пространство")
        }

# === СТАТИСТИКА ===

@dataclass
class LevelStats:
    """Статистика уровня координат"""
    token_count: int = 0
    occupied_cells: int = 0
    min_bounds: Optional[Point3D] = None
    max_bounds: Optional[Point3D] = None
    density: float = 0.0
    last_updated: float = field(default_factory=time.time)
    
    def update_bounds(self, point: Point3D) -> None:
        """Обновляет границы уровня"""
        if self.min_bounds is None:
            self.min_bounds = point
            self.max_bounds = point
        else:
            self.min_bounds = Point3D(
                min(self.min_bounds.x, point.x),
                min(self.min_bounds.y, point.y),
                min(self.min_bounds.z, point.z),
                point.level
            )
            self.max_bounds = Point3D(
                max(self.max_bounds.x, point.x),
                max(self.max_bounds.y, point.y),
                max(self.max_bounds.z, point.z),
                point.level
            )
    
    def calculate_density(self) -> None:
        """Вычисляет плотность токенов"""
        if self.occupied_cells > 0:
            self.density = self.token_count / self.occupied_cells
        else:
            self.density = 0.0

@dataclass 
class SystemStats:
    """Общая статистика системы координат"""
    total_tokens: int = 0
    tokens_per_level: Dict[int, int] = field(default_factory=dict)
    memory_usage_bytes: int = 0
    query_performance: Dict[str, float] = field(default_factory=dict)
    index_efficiency: Dict[int, float] = field(default_factory=dict)
    last_updated: float = field(default_factory=time.time)
    
    def add_query_time(self, operation: str, time_ms: float) -> None:
        """Добавляет время выполнения запроса"""
        if operation in self.query_performance:
            # Экспоненциально взвешенное среднее
            self.query_performance[operation] = (
                0.9 * self.query_performance[operation] + 0.1 * time_ms
            )
        else:
            self.query_performance[operation] = time_ms

    def record_operation(self, operation: str, duration: float) -> None:
        """Записывает длительность операции (в секундах) для профилировщика.

        Использует тот же словарь `query_performance`, приводя секунды к миллисекундам
        и храняя экспоненциально взвешенное среднее.
        """
        try:
            ms = float(duration) * 1000.0
            if operation in self.query_performance:
                self.query_performance[operation] = (
                    0.9 * self.query_performance[operation] + 0.1 * ms
                )
            else:
                self.query_performance[operation] = ms
        except Exception:
            # негромкое поведение — ничего не делаем
            pass
