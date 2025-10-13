#!/usr/bin/env python3
"""
SparseGrid - высокоуровневый интерфейс для работы с пространственными токенами
"""

from typing import Dict, List, Optional, Tuple, Iterator
from .coordinate_system import CoordinateSystem
from .coordinates import (
    Point3D, Region3D, MultiCoordinate, 
    CoordinateConfig, LevelConfig, IndexType
)
from ..token.token import Token

# === МНОГОУРОВНЕВАЯ РАЗРЕЖЕННАЯ СЕТКА ===

class SparseGrid:
    """
    Многоуровневая разреженная сетка - высокоуровневый интерфейс 
    для работы с координатной системой
    """
    
    def __init__(self, coordinate_system: Optional[CoordinateSystem] = None):
        """
        Инициализирует разреженную сетку
        
        Args:
            coordinate_system: Система координат или None для создания дефолтной
        """
        if coordinate_system is None:
            config = CoordinateConfig()
            config.levels = config.get_default_levels()
            self.coord_system = CoordinateSystem(config)
        else:
            self.coord_system = coordinate_system
    
    def add_level(self, level: int, config: LevelConfig) -> None:
        """Добавляет новый уровень координат"""
        self.coord_system.config.levels[level] = config
        
        # Создаем индекс для нового уровня
        if config.index_type == IndexType.SPARSE_GRID:
            from .spatial_index import SparseGridIndex
            self.coord_system._indexes[level] = SparseGridIndex(config)
        elif config.index_type == IndexType.SPATIAL_HASH:
            from .spatial_index import SpatialHashIndex
            self.coord_system._indexes[level] = SpatialHashIndex(config)
    
    def remove_level(self, level: int) -> None:
        """Удаляет уровень координат"""
        if level in self.coord_system._indexes:
            self.coord_system.clear_level(level)
            del self.coord_system._indexes[level]
            if level in self.coord_system.config.levels:
                del self.coord_system.config.levels[level]
    
    def get_active_levels(self) -> List[int]:
        """Возвращает список активных уровней"""
        return self.coord_system.get_active_levels()
    
    def place_token(self, token: Token, coordinates: MultiCoordinate) -> bool:
        """Размещает токен в многомерном пространстве"""
        return self.coord_system.place_token(token, coordinates)
    
    def place_token_simple(self, token: Token, level: int, x: float, y: float, z: float) -> bool:
        """Упрощенное размещение токена на одном уровне"""
        point = Point3D(x, y, z, level)
        coordinates = MultiCoordinate()
        coordinates.set_point(point)
        return self.place_token(token, coordinates)
    
    def get_token_at(self, level: int, x: float, y: float, z: float) -> List[Token]:
        """Получает токены в указанной точке"""
        point = Point3D(x, y, z, level)
        coordinates = MultiCoordinate()
        coordinates.set_point(point)
        return self.coord_system.get_token_at(coordinates)
    
    def find_token_by_id(self, token_id: int) -> Optional[Tuple[Token, MultiCoordinate]]:
        """Находит токен по ID и возвращает его координаты"""
        coordinates = self.coord_system.get_token_coordinates(token_id)
        if not coordinates:
            return None
        
        # Находим сам токен в одном из индексов
        for level, point in coordinates.coordinates.items():
            if level in self.coord_system._indexes:
                tokens = self.coord_system._indexes[level].query_point(point)
                for token in tokens:
                    if token.id == token_id:
                        return token, coordinates
        
        return None
    
    def move_token(self, token_id: int, new_coordinates: MultiCoordinate) -> bool:
        """Перемещает токен в новые координаты"""
        return self.coord_system.move_token(token_id, new_coordinates)
    
    def move_token_simple(self, token_id: int, level: int, x: float, y: float, z: float) -> bool:
        """Упрощенное перемещение токена на одном уровне"""
        point = Point3D(x, y, z, level)
        coordinates = MultiCoordinate()
        coordinates.set_point(point)
        return self.move_token(token_id, coordinates)
    
    def remove_token(self, token_id: int) -> bool:
        """Удаляет токен из сетки"""
        return self.coord_system.remove_token(token_id)
    
    def find_tokens_in_region(self, level: int, min_x: float, min_y: float, min_z: float,
                            max_x: float, max_y: float, max_z: float) -> List[Token]:
        """Находит токены в указанном регионе"""
        min_point = Point3D(min_x, min_y, min_z, level)
        max_point = Point3D(max_x, max_y, max_z, level)
        region = Region3D(min_point, max_point)
        return self.coord_system.find_tokens_in_region(region, level)
    
    def find_tokens_in_radius(self, level: int, center_x: float, center_y: float, center_z: float,
                            radius: float) -> List[Token]:
        """Находит токены в указанном радиусе"""
        center = Point3D(center_x, center_y, center_z, level)
        return self.coord_system.find_tokens_in_radius(center, level, radius)
    
    def find_nearest_neighbors(self, level: int, x: float, y: float, z: float, k: int = 10) -> List[Tuple[Token, float]]:
        """Находит k ближайших соседей"""
        point = Point3D(x, y, z, level)
        return self.coord_system.find_nearest_neighbors(point, level, k)
    
    def get_tokens_at_level(self, level: int) -> Iterator[Tuple[Point3D, Token]]:
        """Итератор по всем токенам на указанном уровне"""
        if level not in self.coord_system._indexes:
            return
        
        for token_id, coordinates in self.coord_system._token_registry.items():
            point = coordinates.get_point(level)
            if point:
                # Находим токен
                tokens = self.coord_system._indexes[level].query_point(point)
                for token in tokens:
                    if token.id == token_id:
                        yield point, token
                        break
    
    def find_aligned_tokens(self, reference_token: Token, 
                           alignment_threshold: float = 0.1) -> List[Token]:
        """
        Находит токены, выровненные с референсным токеном на нескольких уровнях
        
        Args:
            reference_token: Референсный токен
            alignment_threshold: Порог выравнивания
            
        Returns:
            List[Token]: Выровненные токены
        """
        if reference_token.id not in self.coord_system._token_registry:
            return []
        
        ref_coordinates = self.coord_system._token_registry[reference_token.id]
        aligned_tokens = []
        
        # Находим токены на каждом активном уровне референсного токена
        for level, ref_point in ref_coordinates.coordinates.items():
            nearby_tokens = self.coord_system.find_tokens_in_radius(
                ref_point, level, alignment_threshold
            )
            
            for token in nearby_tokens:
                if (token.id != reference_token.id and 
                    token.id in self.coord_system._token_registry):
                    
                    token_coords = self.coord_system._token_registry[token.id]
                    
                    # Проверяем, есть ли токен на других уровнях референсного токена
                    alignment_score = 0
                    total_levels = 0
                    
                    for other_level, other_ref_point in ref_coordinates.coordinates.items():
                        if other_level != level and token_coords.get_point(other_level):
                            token_point = token_coords.get_point(other_level)
                            distance = other_ref_point.distance_to(token_point)
                            
                            if distance <= alignment_threshold:
                                alignment_score += 1
                            total_levels += 1
                    
                    # Если токен выровнен на достаточном количестве уровней
                    if total_levels > 0 and alignment_score / total_levels >= 0.5:
                        aligned_tokens.append(token)
        
        # Удаляем дубликаты
        unique_aligned = {}
        for token in aligned_tokens:
            unique_aligned[token.id] = token
        
        return list(unique_aligned.values())
    
    def compute_inter_level_distance(self, token1: Token, token2: Token) -> Dict[int, float]:
        """
        Вычисляет расстояние между токенами на всех общих уровнях
        
        Args:
            token1: Первый токен
            token2: Второй токен
            
        Returns:
            Dict[int, float]: Расстояния по уровням
        """
        if (token1.id not in self.coord_system._token_registry or
            token2.id not in self.coord_system._token_registry):
            return {}
        
        coords1 = self.coord_system._token_registry[token1.id]
        coords2 = self.coord_system._token_registry[token2.id]
        
        return coords1.distance_to(coords2)
    
    def get_density_map(self, level: int, resolution: float = 0.1) -> Dict[Region3D, int]:
        """Возвращает карту плотности токенов для уровня"""
        return self.coord_system.get_density_map(level, resolution)
    
    def get_level_statistics(self, level: int):
        """Возвращает статистику для уровня"""
        return self.coord_system.get_level_statistics(level)
    
    def get_system_statistics(self):
        """Возвращает общую статистику системы"""
        return self.coord_system.get_system_statistics()
    
    def get_tokens_count(self) -> int:
        """Возвращает общее количество токенов"""
        return self.coord_system.get_tokens_count()
    
    def clear_level(self, level: int) -> bool:
        """Очищает указанный уровень"""
        return self.coord_system.clear_level(level)
    
    def clear_all(self) -> None:
        """Очищает всю сетку"""
        self.coord_system.clear_all()
    
    def __len__(self) -> int:
        """Возвращает количество токенов в сетке"""
        return self.coord_system.get_tokens_count()
    
    def __contains__(self, token_id: int) -> bool:
        """Проверяет, содержится ли токен в сетке"""
        return token_id in self.coord_system._token_registry
    
    def __repr__(self) -> str:
        return f"SparseGrid(levels={len(self.get_active_levels())}, tokens={self.get_tokens_count()})"

# === СТРОИТЕЛЬ КОНФИГУРАЦИИ ===

class SparseGridBuilder:
    """
    Строитель для создания сложных конфигураций разреженной сетки
    """
    
    def __init__(self):
        self.config = CoordinateConfig()
    
    def with_precision(self, precision: int) -> 'SparseGridBuilder':
        """Устанавливает точность координат"""
        self.config.precision = precision
        return self
    
    def with_validation(self, enable: bool = True) -> 'SparseGridBuilder':
        """Включает/выключает валидацию координат"""
        self.config.enable_validation = enable
        return self
    
    def with_statistics(self, enable: bool = True) -> 'SparseGridBuilder':
        """Включает/выключает сбор статистики"""
        self.config.enable_statistics = enable
        return self
    
    def with_cache_size(self, size: int) -> 'SparseGridBuilder':
        """Устанавливает размер кэша"""
        self.config.cache_size = size
        return self
    
    def add_level(self, level: int, name: str, min_val: float, max_val: float,
                  precision: int = 3, index_type: IndexType = IndexType.SPARSE_GRID,
                  semantic_meaning: str = "") -> 'SparseGridBuilder':
        """Добавляет уровень координат"""
        level_config = LevelConfig(
            name=name,
            min_value=min_val,
            max_value=max_val,
            precision=precision,
            semantic_meaning=semantic_meaning,
            index_type=index_type
        )
        self.config.levels[level] = level_config
        return self
    
    def add_physical_level(self, level: int = 0, bounds: float = 327.67) -> 'SparseGridBuilder':
        """Добавляет физический уровень координат"""
        return self.add_level(
            level, "PHYSICAL", -bounds, bounds, 2,
            IndexType.SPARSE_GRID, "Физическое пространство"
        )
    
    def add_sensory_level(self, level: int = 1) -> 'SparseGridBuilder':
        """Добавляет сенсорный уровень координат"""
        return self.add_level(
            level, "SENSORY", -1.0, 1.0, 3,
            IndexType.SPATIAL_HASH, "Сенсорное пространство"
        )
    
    def add_emotional_level(self, level: int = 3) -> 'SparseGridBuilder':
        """Добавляет эмоциональный уровень координат"""
        return self.add_level(
            level, "EMOTIONAL", -1.0, 1.0, 3,
            IndexType.SPARSE_GRID, "Эмоциональное пространство"
        )
    
    def build(self) -> SparseGrid:
        """Создает разреженную сетку с заданной конфигурацией"""
        coord_system = CoordinateSystem(self.config)
        return SparseGrid(coord_system)

# === УТИЛИТЫ ===

def create_demo_sparse_grid() -> SparseGrid:
    """Создает демонстрационную разреженную сетку"""
    
    builder = SparseGridBuilder()
    
    sparse_grid = (builder
                   .with_precision(3)
                   .with_validation(True)
                   .with_statistics(True)
                   .add_physical_level(0, 100.0)
                   .add_sensory_level(1)
                   .add_emotional_level(3)
                   .add_level(4, "COGNITIVE", -1.0, 1.0, 3, IndexType.SPARSE_GRID, "Когнитивное пространство")
                   .build())
    
    return sparse_grid
