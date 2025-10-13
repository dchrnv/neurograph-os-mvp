#!/usr/bin/env python3
"""
Пространственные индексы для системы координат NeuroGraph OS
"""

import math
import time
import threading
from typing import Dict, List, Optional, Tuple, Set
from abc import ABC, abstractmethod
from dataclasses import dataclass

from .coordinates import Point3D, Region3D, LevelConfig, LevelStats, MultiCoordinate
from ..token.token import Token

# === АБСТРАКТНЫЙ БАЗОВЫЙ КЛАСС ===

class SpatialIndex(ABC):
    """Абстрактный базовый класс для пространственных индексов"""
    
    @abstractmethod
    def insert(self, point: Point3D, token: Token) -> None:
        pass
    
    @abstractmethod
    def remove(self, point: Point3D, token_id: int) -> bool:
        pass
    
    @abstractmethod
    def query_point(self, point: Point3D) -> List[Token]:
        pass
    
    @abstractmethod
    def query_region(self, region: Region3D) -> List[Token]:
        pass
    
    @abstractmethod
    def query_radius(self, center: Point3D, radius: float) -> List[Token]:
        pass
    
    @abstractmethod
    def get_bounds(self) -> Optional[Region3D]:
        pass
    
    @abstractmethod
    def get_count(self) -> int:
        pass

    # --- Optional APIs that concrete indexes may implement ---
    def get_stats(self) -> LevelStats:
        """Возвращает статистику уровня (необязательный API)."""
        raise NotImplementedError()

    def get_density_map(self, resolution: float) -> Dict[Region3D, int]:
        """Возвращает карту плотности (необязательный API)."""
        raise NotImplementedError()

    def reindex_all(self, token_registry: Dict[int, MultiCoordinate]) -> None:
        """Попытаться переиндексировать все токены из реестра (необязательный API)."""
        raise NotImplementedError()

    def rescale(self, old_scale: float, new_scale: float) -> None:
        """Попытка изменить масштаб измерений в индексе (необязательный API)."""
        raise NotImplementedError()

    def set_scale(self, scale: float) -> None:
        """Установить масштаб для индекса (необязательный API)."""
        raise NotImplementedError()

    def update_scale(self, scale: float) -> None:
        """Обновить масштаб (синаноним для set_scale)."""
        self.set_scale(scale)

# === РАЗРЕЖЕННАЯ СЕТКА ===

class SparseGridIndex(SpatialIndex):
    """Разреженная сетка для быстрого доступа по координатам"""
    
    def __init__(self, level_config: LevelConfig):
        self.config = level_config
        self.level = None  # Будет установлено при первой вставке
        self._grid: Dict[Tuple[float, float, float], List[Token]] = {}
        self._token_positions: Dict[int, Point3D] = {}  # token_id -> position
        self._stats = LevelStats()
        self._lock = threading.RLock()
    
    def _normalize_point(self, point: Point3D) -> Point3D:
        """Нормализует координаты точки"""
        return Point3D(
            self.config.normalize_coordinate(point.x),
            self.config.normalize_coordinate(point.y), 
            self.config.normalize_coordinate(point.z),
            point.level
        )
    
    def _point_to_key(self, point: Point3D) -> Tuple[float, float, float]:
        """Преобразует точку в ключ сетки"""
        normalized = self._normalize_point(point)
        return (normalized.x, normalized.y, normalized.z)
    
    def insert(self, point: Point3D, token: Token) -> None:
        """Вставляет токен в сетку"""
        with self._lock:
            if self.level is None:
                self.level = point.level
            elif self.level != point.level:
                raise ValueError(f"Несоответствие уровня: ожидался {self.level}, получен {point.level}")
            
            key = self._point_to_key(point)
            
            # Удаляем старую позицию если токен уже существует
            if token.id in self._token_positions:
                old_point = self._token_positions[token.id]
                old_key = self._point_to_key(old_point)
                if old_key in self._grid:
                    self._grid[old_key] = [t for t in self._grid[old_key] if t.id != token.id]
                    if not self._grid[old_key]:
                        del self._grid[old_key]
            
            # Вставляем в новую позицию
            if key not in self._grid:
                self._grid[key] = []
                self._stats.occupied_cells += 1
            
            self._grid[key].append(token)
            self._token_positions[token.id] = self._normalize_point(point)
            self._stats.token_count += 1
            self._stats.update_bounds(self._normalize_point(point))
    
    def remove(self, point: Point3D, token_id: int) -> bool:
        """Удаляет токен из сетки"""
        with self._lock:
            if token_id not in self._token_positions:
                return False
            
            stored_point = self._token_positions[token_id]
            key = self._point_to_key(stored_point)
            
            if key in self._grid:
                original_count = len(self._grid[key])
                self._grid[key] = [t for t in self._grid[key] if t.id != token_id]
                
                if len(self._grid[key]) < original_count:
                    del self._token_positions[token_id]
                    self._stats.token_count -= 1
                    
                    if not self._grid[key]:
                        del self._grid[key]
                        self._stats.occupied_cells -= 1
                    
                    return True
            
            return False
    
    def query_point(self, point: Point3D) -> List[Token]:
        """Возвращает токены в точке"""
        with self._lock:
            key = self._point_to_key(point)
            return self._grid.get(key, []).copy()
    
    def query_region(self, region: Region3D) -> List[Token]:
        """Возвращает токены в регионе"""
        # если уровень не установлен в индексе — ничего не найдено
        if self.level is None or region.level != self.level:
            return []
        
        tokens = []
        with self._lock:
            # Итерируемся по всем ключам и проверяем пересечение
            for key, token_list in self._grid.items():
                point = Point3D(key[0], key[1], key[2], self.level)
                if region.contains(point):
                    tokens.extend(token_list)
        
        return tokens
    
    def query_radius(self, center: Point3D, radius: float) -> List[Token]:
        """Возвращает токены в радиусе от центра"""
        if self.level is None or center.level != self.level:
            return []
        
        tokens = []
        radius_squared = radius * radius
        
        with self._lock:
            for key, token_list in self._grid.items():
                point = Point3D(key[0], key[1], key[2], self.level)
                distance_squared = ((point.x - center.x) ** 2 + 
                                  (point.y - center.y) ** 2 + 
                                  (point.z - center.z) ** 2)
                
                if distance_squared <= radius_squared:
                    tokens.extend(token_list)
        
        return tokens
    
    def get_bounds(self) -> Optional[Region3D]:
        """Возвращает границы занятого пространства"""
        with self._lock:
            if self._stats.min_bounds and self._stats.max_bounds:
                return Region3D(self._stats.min_bounds, self._stats.max_bounds)
            return None
    
    def get_count(self) -> int:
        """Возвращает количество токенов"""
        return self._stats.token_count
    
    def get_stats(self) -> LevelStats:
        """Возвращает статистику уровня"""
        with self._lock:
            self._stats.calculate_density()
            return self._stats
    
    def get_density_map(self, resolution: float) -> Dict[Region3D, int]:
        """Возвращает карту плотности"""
        density_map = {}
        
        with self._lock:
            bounds = self.get_bounds()
            if not bounds:
                return density_map
            
            # Разбиваем пространство на регионы с заданным разрешением
            x_steps = int((bounds.max_point.x - bounds.min_point.x) / resolution) + 1
            y_steps = int((bounds.max_point.y - bounds.min_point.y) / resolution) + 1
            z_steps = int((bounds.max_point.z - bounds.min_point.z) / resolution) + 1
            
            for i in range(x_steps):
                for j in range(y_steps):
                    for k in range(z_steps):
                        min_x = bounds.min_point.x + i * resolution
                        min_y = bounds.min_point.y + j * resolution
                        min_z = bounds.min_point.z + k * resolution
                        
                        max_x = min(min_x + resolution, bounds.max_point.x)
                        max_y = min(min_y + resolution, bounds.max_point.y)
                        max_z = min(min_z + resolution, bounds.max_point.z)
                        
                        region = Region3D(
                            Point3D(min_x, min_y, min_z, self.level),
                            Point3D(max_x, max_y, max_z, self.level)
                        )
                        
                        tokens_in_region = self.query_region(region)
                        if tokens_in_region:
                            density_map[region] = len(tokens_in_region)
        
        return density_map

# === ПРОСТРАНСТВЕННЫЙ ХЭШ ===

class SpatialHashIndex(SpatialIndex):
    """Пространственный хэш-индекс для быстрых приближенных запросов"""
    
    def __init__(self, level_config: LevelConfig, cell_size: float = 0.1):
        self.config = level_config
        self.level = None
        self.cell_size = cell_size
        self._hash_grid: Dict[Tuple[int, int, int], List[Token]] = {}
        self._token_positions: Dict[int, Point3D] = {}
        self._stats = LevelStats()
        self._lock = threading.RLock()
    
    def _hash_point(self, point: Point3D) -> Tuple[int, int, int]:
        """Хэширует точку в ячейку сетки"""
        return (
            int(point.x / self.cell_size),
            int(point.y / self.cell_size),
            int(point.z / self.cell_size)
        )
    
    def insert(self, point: Point3D, token: Token) -> None:
        """Вставляет токен в хэш-индекс"""
        with self._lock:
            if self.level is None:
                self.level = point.level
            elif self.level != point.level:
                raise ValueError(f"Несоответствие уровня: ожидался {self.level}, получен {point.level}")
            
            # Удаляем старую позицию
            if token.id in self._token_positions:
                old_point = self._token_positions[token.id]
                old_hash = self._hash_point(old_point)
                if old_hash in self._hash_grid:
                    self._hash_grid[old_hash] = [t for t in self._hash_grid[old_hash] if t.id != token.id]
                    if not self._hash_grid[old_hash]:
                        del self._hash_grid[old_hash]
            
            # Вставляем в новую позицию
            hash_key = self._hash_point(point)
            if hash_key not in self._hash_grid:
                self._hash_grid[hash_key] = []
                self._stats.occupied_cells += 1
            
            self._hash_grid[hash_key].append(token)
            self._token_positions[token.id] = point
            self._stats.token_count += 1
            self._stats.update_bounds(point)
    
    def remove(self, point: Point3D, token_id: int) -> bool:
        """Удаляет токен из хэш-индекса"""
        with self._lock:
            if token_id not in self._token_positions:
                return False
            
            stored_point = self._token_positions[token_id]
            hash_key = self._hash_point(stored_point)
            
            if hash_key in self._hash_grid:
                original_count = len(self._hash_grid[hash_key])
                self._hash_grid[hash_key] = [t for t in self._hash_grid[hash_key] if t.id != token_id]
                
                if len(self._hash_grid[hash_key]) < original_count:
                    del self._token_positions[token_id]
                    self._stats.token_count -= 1
                    
                    if not self._hash_grid[hash_key]:
                        del self._hash_grid[hash_key]
                        self._stats.occupied_cells -= 1
                    
                    return True
            
            return False
    
    def query_point(self, point: Point3D) -> List[Token]:
        """Возвращает токены в ячейке с точкой"""
        with self._lock:
            hash_key = self._hash_point(point)
            return self._hash_grid.get(hash_key, []).copy()
    
    def query_region(self, region: Region3D) -> List[Token]:
        """Возвращает токены в регионе (приближенно)"""
        if region.level != self.level:
            return []
        
        tokens = []
        with self._lock:
            # Находим диапазон хэш-ячеек
            min_hash = self._hash_point(region.min_point)
            max_hash = self._hash_point(region.max_point)
            
            for x in range(min_hash[0], max_hash[0] + 1):
                for y in range(min_hash[1], max_hash[1] + 1):
                    for z in range(min_hash[2], max_hash[2] + 1):
                        hash_key = (x, y, z)
                        if hash_key in self._hash_grid:
                            # Фильтруем токены, которые действительно в регионе
                            for token in self._hash_grid[hash_key]:
                                if token.id in self._token_positions:
                                    token_point = self._token_positions[token.id]
                                    if region.contains(token_point):
                                        tokens.append(token)
        
        return tokens
    
    def query_radius(self, center: Point3D, radius: float) -> List[Token]:
        """Возвращает токены в радиусе (приближенно)"""
        if center.level != self.level:
            return []
        
        # Создаем квадратный регион вокруг центра
        region = Region3D(
            Point3D(center.x - radius, center.y - radius, center.z - radius, center.level),
            Point3D(center.x + radius, center.y + radius, center.z + radius, center.level)
        )
        
        # Получаем кандидатов из региона
        candidates = self.query_region(region)
        
        # Фильтруем по точному расстоянию
        tokens = []
        radius_squared = radius * radius
        
        for token in candidates:
            if token.id in self._token_positions:
                token_point = self._token_positions[token.id]
                distance_squared = ((token_point.x - center.x) ** 2 + 
                                  (token_point.y - center.y) ** 2 + 
                                  (token_point.z - center.z) ** 2)
                
                if distance_squared <= radius_squared:
                    tokens.append(token)
        
        return tokens
    
    def get_bounds(self) -> Optional[Region3D]:
        """Возвращает границы занятого пространства"""
        with self._lock:
            if self._stats.min_bounds and self._stats.max_bounds:
                return Region3D(self._stats.min_bounds, self._stats.max_bounds)
            return None
    
    def get_count(self) -> int:
        """Возвращает количество токенов"""
        return self._stats.token_count
