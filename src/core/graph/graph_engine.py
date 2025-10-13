#!/usr/bin/env python3
"""
Graph Engine для NeuroGraph OS

Графовая система, использующая токены как самодостаточные единицы графа
без дублирования данных. Граф служит индексной структурой для связей.
"""

import time
import math
import struct
from typing import Dict, List, Optional, Set, Tuple, Any, Iterator
from dataclasses import dataclass, field
from enum import Enum
import threading
from collections import defaultdict, deque
import heapq

from sortedcontainers import SortedList
from ..token.token import Token
from ..spatial import SparseGrid, Point3D, MultiCoordinate

# === КОНСТАНТЫ И ФЛАГИ ===

# Битовые флаги для графовых свойств токенов
FLAG_ACTIVE = 0x0001        # Активный узел
FLAG_ROOT = 0x0002          # Корневой узел
FLAG_LEAF = 0x0004          # Листовой узел  
FLAG_HUB = 0x0008           # Узел-хаб (много связей)
FLAG_BRIDGE = 0x0010        # Мостовой узел между кластерами
FLAG_TEMPORARY = 0x0020     # Временный узел
FLAG_LOCKED = 0x0040        # Заблокированный узел
FLAG_DIRTY = 0x0080         # Требует обновления
FLAG_PROCESSING = 0x0100    # В процессе обработки
FLAG_ERROR = 0x0200         # Ошибочное состояние
FLAG_COMPRESSED = 0x0400    # Сжатые данные
FLAG_EXTERNAL_REF = 0x0800  # Ссылка на внешний ресурс
FLAG_USER_1 = 0x1000        # Пользовательский флаг 1
FLAG_USER_2 = 0x2000        # Пользовательский флаг 2  
FLAG_USER_3 = 0x4000        # Пользовательский флаг 3
FLAG_RESERVED = 0x8000      # Резерв

# Типы связей в графе
class ConnectionType(Enum):
    ASSOCIATION = "association"      # Ассоциативная связь
    INFLUENCE = "influence"          # Влияние
    INHERITANCE = "inheritance"      # Наследование
    SIMILARITY = "similarity"        # Сходство
    CAUSALITY = "causality"          # Причинность
    SEQUENCE = "sequence"            # Последовательность
    SPATIAL_PROXIMITY = "spatial"    # Пространственная близость
    TEMPORAL_PROXIMITY = "temporal"  # Временная близость

# Направленность связей
class Directionality(Enum):
    DIRECTED = "directed"           # Направленная
    UNDIRECTED = "undirected"       # Ненаправленная
    BIDIRECTIONAL = "bidirectional" # Двунаправленная

# Уровень персистентности
class PersistenceLevel(Enum):
    TRANSIENT = "transient"         # Временная
    PERSISTENT = "persistent"       # Постоянная
    PERMANENT = "permanent"         # Неизменная

# === СТРУКТУРЫ ДАННЫХ ===

@dataclass
class ConnectionMetadata:
    """Метаданные связи между токенами"""
    connection_type: ConnectionType
    weight: float = 1.0
    confidence: float = 1.0
    created_timestamp: int = field(default_factory=lambda: int(time.time()))
    last_updated: int = field(default_factory=lambda: int(time.time()))
    directionality: Directionality = Directionality.UNDIRECTED
    persistence_level: PersistenceLevel = PersistenceLevel.PERSISTENT
    context_tags: List[str] = field(default_factory=list)
    evolution_history: List[Tuple[int, float]] = field(default_factory=list)  # (timestamp, weight)
    
    def update_weight(self, new_weight: float) -> None:
        """Обновляет вес связи с сохранением истории"""
        self.evolution_history.append((self.last_updated, self.weight))
        self.weight = new_weight
        self.last_updated = int(time.time())

@dataclass
class GraphStats:
    """Статистика графа"""
    total_nodes: int = 0
    total_edges: int = 0
    nodes_per_type: Dict[int, int] = field(default_factory=dict)
    edges_per_type: Dict[ConnectionType, int] = field(default_factory=dict)
    average_degree: float = 0.0
    clustering_coefficient: float = 0.0
    average_path_length: float = 0.0
    connected_components: int = 0
    memory_usage_bytes: int = 0
    last_updated: float = field(default_factory=time.time)

# === УТИЛИТЫ ДЛЯ РАБОТЫ С ТОКЕНАМИ ===

class TokenFlags:
    """Утилиты для работы с флагами токенов"""
    
    @staticmethod
    def set_flag(token: Token, flag: int) -> None:
        """Устанавливает флаг токена"""
        token.flags |= flag
        
    @staticmethod
    def clear_flag(token: Token, flag: int) -> None:
        """Очищает флаг токена"""
        token.flags &= ~flag
        
    @staticmethod
    def has_flag(token: Token, flag: int) -> bool:
        """Проверяет наличие флага"""
        return bool(token.flags & flag)
    
    @staticmethod
    def is_graph_node(token: Token) -> bool:
        """Проверяет, является ли токен узлом графа"""
        return TokenFlags.has_flag(token, FLAG_ACTIVE)
        
    @staticmethod
    def is_hub(token: Token) -> bool:
        """Проверяет, является ли токен хабом"""
        return TokenFlags.has_flag(token, FLAG_HUB)
    
    @staticmethod
    def is_root(token: Token) -> bool:
        """Проверяет, является ли токен корневым"""
        return TokenFlags.has_flag(token, FLAG_ROOT)
    
    @staticmethod
    def is_leaf(token: Token) -> bool:
        """Проверяет, является ли токен листовым"""
        return TokenFlags.has_flag(token, FLAG_LEAF)

class TokenWeightEncoder:
    """Кодировщик веса токена для множественных параметров"""
    
    @staticmethod
    def encode_weight(importance: float, connections: int, activity: float) -> float:
        """
        Кодирует множественные параметры в один float
        importance: 0.0-1.0 (точность до 0.001)
        connections: 0-4095 логарифмически  
        activity: 0.0-1.0 (точность до 0.01)
        """
        # Упаковываем в мантиссу float32
        imp_coded = int(importance * 1000) & 0x3FF     # 10 бит
        con_coded = min(int(math.log2(connections + 1) * 273), 0xFFF)  # 12 бит  
        act_coded = int(activity * 100) & 0x7F         # 7 бит
        
        packed = (imp_coded << 17) | (con_coded << 5) | act_coded
        return struct.unpack('f', struct.pack('I', packed | 0x3F800000))[0]
    
    @staticmethod
    def decode_weight(weight: float) -> Tuple[float, int, float]:
        """Декодирует вес обратно в параметры"""
        packed = struct.unpack('I', struct.pack('f', weight))[0] & 0x7FFFFF
        
        importance = (packed >> 17) & 0x3FF
        connections = int(2 ** ((packed >> 5) & 0xFFF) / 273) - 1
        activity = (packed & 0x7F) / 100.0
        
        return importance / 1000.0, connections, activity

class TokenIDExtractor:
    """Утилиты для извлечения информации из ID токена"""
    
    @staticmethod
    def extract_node_type(token_id: int) -> int:
        """Извлекает тип узла из ID"""
        return (token_id >> 24) & 0xF
    
    @staticmethod
    def extract_cluster(token_id: int) -> int:
        """Извлекает кластер из ID"""
        return (token_id >> 28) & 0xF
    
    @staticmethod
    def get_local_id(token_id: int) -> int:
        """Получает локальный ID"""
        return token_id & 0xFFFFFF

# === ОСНОВНОЙ КЛАСС ГРАФА ===

class TokenGraph:
    """
    Граф токенов - индексная структура для связей между токенами
    без дублирования данных токенов
    """
    
    def __init__(self, sparse_grid: Optional[SparseGrid] = None):
        """
        Инициализирует граф токенов
        
        Args:
            sparse_grid: Разреженная сетка для пространственного индексирования
        """
        self.sparse_grid = sparse_grid or SparseGrid()
        
        # Основные структуры данных
        self.tokens: Dict[int, Token] = {}                    # ID -> Token
        self.adjacency: Dict[int, Set[int]] = {}             # ID -> Set[connected_IDs]  
        self.edge_metadata: Dict[Tuple[int, int], ConnectionMetadata] = {}  # (ID1,ID2) -> metadata
        self.temporal_index: SortedList = SortedList()       # (timestamp, token_id)

        # Кэши и статистика
        self._path_cache: Dict[Tuple[int, int], List[int]] = {}
        self._distance_cache: Dict[Tuple[int, int], float] = {}
        self._stats = GraphStats()
        self._lock = threading.RLock()
        
        # Настройки
        self.auto_connect_spatial = True
        self.auto_connect_temporal = True
        self.spatial_connection_radius = 0.1
        self.temporal_connection_window = 3600  # 1 час
        self.max_connections_per_node = 100
    
    def add_token(self, token: Token) -> None:
        """
        Добавляет токен в граф
        
        Args:
            token: Токен для добавления
        """
        with self._lock:
            self.tokens[token.id] = token
            # Предполагается, что токен уже размещен в sparse_grid извне,
            # но если граф отвечает за его жизненный цикл, то размещение должно быть здесь.
            # self.sparse_grid.place_token(token, coordinates) # `coordinates` нужно будет передать
            self.adjacency[token.id] = set()

            # Добавляем во временной индекс
            self.temporal_index.add((token.timestamp, token.id))
            
            # Обновляем статистику
            self._stats.total_nodes += 1
            node_type = TokenIDExtractor.extract_node_type(token.id)
            self._stats.nodes_per_type[node_type] = self._stats.nodes_per_type.get(node_type, 0) + 1
            
            # Автоматически создаем связи
            if self.auto_connect_spatial:
                self._auto_connect_spatial(token)
            if self.auto_connect_temporal:
                self._auto_connect_temporal(token)
    
    def remove_token(self, token_id: int) -> bool:
        """
        Удаляет токен из графа
        
        Args:
            token_id: ID токена для удаления
            
        Returns:
            bool: True если удаление успешно
        """
        with self._lock:
            if token_id not in self.tokens:
                return False
            
            # Удаляем все связи
            connected_tokens = list(self.adjacency[token_id])
            for connected_id in connected_tokens:
                self.disconnect(token_id, connected_id)
            
            # Удаляем из пространственной сетки
            self.sparse_grid.remove_token(token_id)
            
            # Удаляем из временного индекса
            self.temporal_index.discard((self.tokens[token_id].timestamp, token_id))

            # Удаляем из основных структур
            del self.tokens[token_id]
            del self.adjacency[token_id]
            
            # Очищаем кэши
            self._clear_caches_for_token(token_id)
            
            # Обновляем статистику
            self._stats.total_nodes -= 1
            node_type = TokenIDExtractor.extract_node_type(token_id)
            if node_type in self._stats.nodes_per_type:
                self._stats.nodes_per_type[node_type] -= 1
                if self._stats.nodes_per_type[node_type] <= 0:
                    del self._stats.nodes_per_type[node_type]
            
            return True
    
    def connect(self, token1_id: int, token2_id: int, 
                connection_type: ConnectionType = ConnectionType.ASSOCIATION,
                weight: float = 1.0, confidence: float = 1.0,
                directionality: Directionality = Directionality.UNDIRECTED) -> bool:
        """
        Создает связь между токенами
        
        Args:
            token1_id: ID первого токена
            token2_id: ID второго токена
            connection_type: Тип связи
            weight: Вес связи
            confidence: Уверенность в связи
            directionality: Направленность связи
            
        Returns:
            bool: True если связь создана успешно
        """
        with self._lock:
            if token1_id not in self.tokens or token2_id not in self.tokens:
                return False
            
            if token1_id == token2_id:
                return False  # Нельзя связать токен с самим собой
            
            # Проверяем лимит связей
            if (len(self.adjacency[token1_id]) >= self.max_connections_per_node or
                len(self.adjacency[token2_id]) >= self.max_connections_per_node):
                return False
            
            # Создаем связь
            self.adjacency[token1_id].add(token2_id)
            self.adjacency[token2_id].add(token1_id)
            
            # Создаем метаданные связи
            metadata = ConnectionMetadata(
                connection_type=connection_type,
                weight=weight,
                confidence=confidence,
                directionality=directionality
            )
            
            edge_key = (min(token1_id, token2_id), max(token1_id, token2_id))
            self.edge_metadata[edge_key] = metadata
            
            # Обновляем флаги токенов
            self._update_token_flags(token1_id)
            self._update_token_flags(token2_id)
            
            # Обновляем статистику
            self._stats.total_edges += 1
            self._stats.edges_per_type[connection_type] = self._stats.edges_per_type.get(connection_type, 0) + 1
            
            # Очищаем кэши
            self._clear_caches_for_edge(token1_id, token2_id)
            
            return True
    
    def disconnect(self, token1_id: int, token2_id: int) -> bool:
        """
        Удаляет связь между токенами
        
        Args:
            token1_id: ID первого токена
            token2_id: ID второго токена
            
        Returns:
            bool: True если связь удалена успешно
        """
        with self._lock:
            if token1_id not in self.adjacency or token2_id not in self.adjacency:
                return False
            
            # Удаляем связь
            self.adjacency[token1_id].discard(token2_id)
            self.adjacency[token2_id].discard(token1_id)
            
            # Удаляем метаданные
            edge_key = (min(token1_id, token2_id), max(token1_id, token2_id))
            if edge_key in self.edge_metadata:
                connection_type = self.edge_metadata[edge_key].connection_type
                del self.edge_metadata[edge_key]
                
                # Обновляем статистику
                self._stats.total_edges -= 1
                if connection_type in self._stats.edges_per_type:
                    self._stats.edges_per_type[connection_type] -= 1
                    if self._stats.edges_per_type[connection_type] <= 0:
                        del self._stats.edges_per_type[connection_type]
            
            # Обновляем флаги токенов
            self._update_token_flags(token1_id)
            self._update_token_flags(token2_id)
            
            # Очищаем кэши
            self._clear_caches_for_edge(token1_id, token2_id)
            
            return True
    
    def get_neighbors(self, token_id: int) -> List[Token]:
        """
        Получает соседей токена
        
        Args:
            token_id: ID токена
            
        Returns:
            List[Token]: Список соседних токенов
        """
        with self._lock:
            if token_id not in self.adjacency:
                return []
            
            neighbor_ids = self.adjacency[token_id]
            return [self.tokens[nid] for nid in neighbor_ids if nid in self.tokens]
    
    def get_connection_metadata(self, token1_id: int, token2_id: int) -> Optional[ConnectionMetadata]:
        """
        Получает метаданные связи между токенами
        
        Args:
            token1_id: ID первого токена
            token2_id: ID второго токена
            
        Returns:
            ConnectionMetadata: Метаданные связи или None
        """
        edge_key = (min(token1_id, token2_id), max(token1_id, token2_id))
        return self.edge_metadata.get(edge_key)
    
    def find_path(self, start_id: int, end_id: int, max_depth: int = 10) -> List[int]:
        """
        Находит путь между токенами
        
        Args:
            start_id: ID начального токена
            end_id: ID конечного токена
            max_depth: Максимальная глубина поиска
            
        Returns:
            List[int]: Список ID токенов в пути или пустой список
        """
        with self._lock:
            # Проверяем кэш
            cache_key = (start_id, end_id)
            if cache_key in self._path_cache:
                return self._path_cache[cache_key]
            
            # BFS поиск
            queue = deque([(start_id, [start_id])])
            visited = {start_id}
            
            while queue:
                current_id, path = queue.popleft()
                
                if len(path) > max_depth:
                    continue
                
                if current_id == end_id:
                    self._path_cache[cache_key] = path
                    return path
                
                for neighbor_id in self.adjacency.get(current_id, []):
                    if neighbor_id not in visited:
                        visited.add(neighbor_id)
                        queue.append((neighbor_id, path + [neighbor_id]))
            
            # Путь не найден
            self._path_cache[cache_key] = []
            return []
    
    def calculate_distance(self, token1_id: int, token2_id: int) -> float:
        """
        Вычисляет расстояние между токенами
        
        Args:
            token1_id: ID первого токена
            token2_id: ID второго токена
            
        Returns:
            float: Расстояние между токенами
        """
        with self._lock:
            # Проверяем кэш
            cache_key = (min(token1_id, token2_id), max(token1_id, token2_id))
            if cache_key in self._distance_cache:
                return self._distance_cache[cache_key]
            
            token1 = self.tokens.get(token1_id)
            token2 = self.tokens.get(token2_id)
            
            if not token1 or not token2:
                return float('inf')
            
            # Вычисляем многомерное расстояние
            total_distance = 0.0
            active_levels = 0
            
            for level in range(8):
                coords1 = token1.get_coordinates(level)
                coords2 = token2.get_coordinates(level)
                
                if coords1 and coords2:
                    level_distance = math.sqrt(
                        sum((a - b) ** 2 for a, b in zip(coords1, coords2))
                    )
                    total_distance += level_distance
                    active_levels += 1
            
            if active_levels == 0:
                distance = float('inf')
            else:
                distance = total_distance / active_levels
            
            # Кэшируем результат
            self._distance_cache[cache_key] = distance
            return distance
    
    def find_spatial_neighbors(self, token_id: int, level: int, radius: float) -> List[Token]:
        """
        Находит соседей в пространстве определенного уровня
        
        Args:
            token_id: ID токена
            level: Уровень пространства
            radius: Радиус поиска
            
        Returns:
            List[Token]: Список соседних токенов
        """
        with self._lock:
            token = self.tokens.get(token_id)
            if not token:
                return []
            
            coords = token.get_coordinates(level)
            if not coords:
                return []
            
            # Используем sparse_grid для поиска
            neighbor_tokens = self.sparse_grid.find_tokens_in_radius(level, coords[0], coords[1], coords[2], radius)
            return [t for t in neighbor_tokens if t.id != token_id and t.id in self.tokens]
            
            return []
    
    def find_temporal_neighbors(self, token_id: int, time_window: int) -> List[Token]:
        """
        Находит токены, созданные в близком времени
        
        Args:
            token_id: ID токена
            time_window: Временное окно в секундах
            
        Returns:
            List[Token]: Список временных соседей
        """
        with self._lock:
            token = self.tokens.get(token_id)
            if not token:
                return []
            
            min_ts = token.timestamp - time_window
            max_ts = token.timestamp + time_window
            
            # Находим диапазон во временном индексе
            start_idx = self.temporal_index.bisect_left((min_ts, 0))
            end_idx = self.temporal_index.bisect_right((max_ts, float('inf')))
            
            neighbor_ids = [tid for ts, tid in self.temporal_index[start_idx:end_idx] if tid != token_id]
            
            return [self.tokens[nid] for nid in neighbor_ids if nid in self.tokens]
    
    def get_graph_statistics(self) -> GraphStats:
        """
        Получает статистику графа
        
        Returns:
            GraphStats: Статистика графа
        """
        with self._lock:
            # Обновляем вычисляемые метрики
            self._update_graph_metrics()
            self._stats.last_updated = time.time()
            return self._stats
    
    # === ПРИВАТНЫЕ МЕТОДЫ ===
    
    def _auto_connect_spatial(self, token: Token) -> None:
        """Автоматически создает пространственные связи"""
        for level in range(8):
            coords = token.get_coordinates(level)
            if coords:
                # Находим близкие токены
                close_tokens = self.find_spatial_neighbors(token.id, level, self.spatial_connection_radius)
                
                for close_token in close_tokens:
                    if len(self.adjacency[token.id]) < self.max_connections_per_node:
                        self.connect(token.id, close_token.id, ConnectionType.SPATIAL_PROXIMITY)
    
    def _auto_connect_temporal(self, token: Token) -> None:
        """Автоматически создает временные связи"""
        temporal_neighbors = self.find_temporal_neighbors(token.id, self.temporal_connection_window)
        
        for neighbor in temporal_neighbors:
            if len(self.adjacency[token.id]) < self.max_connections_per_node:
                self.connect(token.id, neighbor.id, ConnectionType.TEMPORAL_PROXIMITY)
    
    def _update_token_flags(self, token_id: int) -> None:
        """Обновляет флаги токена на основе его связей"""
        token = self.tokens.get(token_id)
        if not token:
            return
        
        degree = len(self.adjacency[token_id])
        
        # Очищаем графовые флаги
        token.flags &= ~(FLAG_ACTIVE | FLAG_HUB | FLAG_LEAF | FLAG_ROOT)
        
        # Устанавливаем флаги на основе связей
        if degree > 0:
            TokenFlags.set_flag(token, FLAG_ACTIVE)
        
        if degree > 10:  # Хаб
            TokenFlags.set_flag(token, FLAG_HUB)
        elif degree == 0:  # Лист
            TokenFlags.set_flag(token, FLAG_LEAF)
        elif degree == 1:  # Возможно корень
            TokenFlags.set_flag(token, FLAG_ROOT)
    
    def _clear_caches_for_token(self, token_id: int) -> None:
        """Очищает кэши для токена"""
        keys_to_remove = []
        for key in self._path_cache:
            if token_id in key:
                keys_to_remove.append(key)
        for key in keys_to_remove:
            del self._path_cache[key]
        
        keys_to_remove = []
        for key in self._distance_cache:
            if token_id in key:
                keys_to_remove.append(key)
        for key in keys_to_remove:
            del self._distance_cache[key]
    
    def _clear_caches_for_edge(self, token1_id: int, token2_id: int) -> None:
        """Очищает кэши для связи"""
        keys_to_remove = []
        for key in self._path_cache:
            if token1_id in key or token2_id in key:
                keys_to_remove.append(key)
        for key in keys_to_remove:
            del self._path_cache[key]
        
        cache_key = (min(token1_id, token2_id), max(token1_id, token2_id))
        if cache_key in self._distance_cache:
            del self._distance_cache[cache_key]
    
    def _update_graph_metrics(self) -> None:
        """Обновляет метрики графа"""
        if self._stats.total_nodes == 0:
            return
        
        # Средняя степень
        total_degree = sum(len(neighbors) for neighbors in self.adjacency.values())
        self._stats.average_degree = total_degree / self._stats.total_nodes
        
        # Коэффициент кластеризации (упрощенный)
        clustering_sum = 0.0
        nodes_with_neighbors = 0
        
        for token_id, neighbors in self.adjacency.items():
            if len(neighbors) >= 2:
                # Считаем треугольники
                triangles = 0
                for neighbor1 in neighbors:
                    for neighbor2 in neighbors:
                        if neighbor1 < neighbor2 and neighbor2 in self.adjacency.get(neighbor1, set()):
                            triangles += 1
                
                max_triangles = len(neighbors) * (len(neighbors) - 1) // 2
                if max_triangles > 0:
                    clustering_sum += triangles / max_triangles
                    nodes_with_neighbors += 1
        
        if nodes_with_neighbors > 0:
            self._stats.clustering_coefficient = clustering_sum / nodes_with_neighbors
        
        # Подсчет компонент связности (упрощенный)
        visited = set()
        components = 0
        
        for token_id in self.tokens:
            if token_id not in visited:
                # BFS для компоненты
                queue = deque([token_id])
                while queue:
                    current = queue.popleft()
                    if current not in visited:
                        visited.add(current)
                        queue.extend(self.adjacency.get(current, []))
                components += 1
        
        self._stats.connected_components = components
    
    def __len__(self) -> int:
        """Возвращает количество токенов в графе"""
        return len(self.tokens)
    
    def __contains__(self, token_id: int) -> bool:
        """Проверяет, содержится ли токен в графе"""
        return token_id in self.tokens
    
    def __repr__(self) -> str:
        return f"TokenGraph(nodes={len(self.tokens)}, edges={self._stats.total_edges})"

