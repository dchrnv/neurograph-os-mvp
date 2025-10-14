#!/usr/bin/env python3
"""
Graph Engine для NeuroGraph OS

Графовая система, использующая токены как самодостаточные единицы графа
без дублирования данных. Граф служит индексной структурой для связей.

Version: 1.0.0
Status: Production Ready

Core module implementing graph-based knowledge representation with:
- Token nodes and typed connections
- Genetic operators (mutation, crossover, selection)
- CDNA validation integration
- Experience stream recording
- Spatial indexing integration
"""

import time
import math
import struct
import random
import hashlib
import uuid
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
    # Structural connections
    ASSOCIATION = "association"      # Ассоциативная связь
    HIERARCHY = "hierarchy"          # Иерархия
    SEQUENCE = "sequence"            # Последовательность
    
    # Logical connections
    CAUSALITY = "causality"          # Причинность
    SIMILARITY = "similarity"        # Сходство
    OPPOSITION = "opposition"        # Противоположность
    
    # Functional connections
    DEPENDENCY = "dependency"        # Зависимость
    COMPOSITION = "composition"      # Композиция
    REFERENCE = "reference"          # Ссылка
    
    # Evolutionary connections
    MUTATION = "mutation"            # Мутация
    CROSSOVER = "crossover"          # Кроссовер
    INHERITANCE = "inheritance"      # Наследование
    
    # Spatial connections
    PROXIMITY = "proximity"          # Близость
    CONTAINMENT = "containment"      # Вложенность
    SPATIAL_PROXIMITY = "spatial"    # Пространственная близость
    TEMPORAL_PROXIMITY = "temporal"  # Временная близость
    
    # Legacy/Additional
    INFLUENCE = "influence"          # Влияние
    
    # Custom
    CUSTOM_1 = "custom_1"
    CUSTOM_2 = "custom_2"

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


# Типы мутаций графа
class MutationType(Enum):
    """Types of graph mutations"""
    ADD_CONNECTION = 1
    REMOVE_CONNECTION = 2
    MODIFY_WEIGHT = 3
    CHANGE_TYPE = 4
    REVERSE_DIRECTION = 5

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
    
    # Evolution metrics
    total_mutations: int = 0
    total_crossovers: int = 0
    generation: int = 0
    
    # Temporal metrics
    avg_connection_age_seconds: float = 0.0
    recent_activity_rate: float = 0.0
    
    # Hub and bridge nodes
    hub_nodes: List[int] = field(default_factory=list)
    bridge_nodes: List[int] = field(default_factory=list)
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary"""
        return {
            'node_count': self.total_nodes,
            'edge_count': self.total_edges,
            'average_degree': self.average_degree,
            'density': self.clustering_coefficient,
            'connected_components': self.connected_components,
            'hub_nodes': self.hub_nodes,
            'bridge_nodes': self.bridge_nodes,
            'total_mutations': self.total_mutations,
            'total_crossovers': self.total_crossovers,
            'generation': self.generation,
            'avg_connection_age_seconds': self.avg_connection_age_seconds,
            'recent_activity_rate': self.recent_activity_rate
        }

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


@dataclass
class Connection:
    """Enhanced connection between two tokens with evolution tracking"""
    
    # Identification
    connection_id: str
    from_token_id: int
    to_token_id: int
    connection_type: ConnectionType
    
    # Connection properties
    weight: float = 1.0
    bidirectional: bool = False
    
    # Metadata
    created_at: float = field(default_factory=time.time)
    last_activated: float = 0.0
    activation_count: int = 0
    
    # Evolution tracking
    generation: int = 0
    parent_connection_id: Optional[str] = None
    mutation_applied: Optional[str] = None
    
    # ADNA snapshot
    adna_generation_hash: Optional[int] = None
    
    # Additional data
    metadata: Dict[str, Any] = field(default_factory=dict)
    
    def activate(self) -> None:
        """Activate connection"""
        self.last_activated = time.time()
        self.activation_count += 1
    
    def decay_weight(self, decay_rate: float = 0.99) -> None:
        """Decrease connection weight (forgetting)"""
        self.weight *= decay_rate
        self.weight = max(0.01, self.weight)
    
    def strengthen(self, amount: float = 0.1) -> None:
        """Strengthen connection"""
        self.weight = min(1.0, self.weight + amount)
    
    def age_seconds(self) -> float:
        """Get connection age in seconds"""
        return time.time() - self.created_at
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary"""
        return {
            'connection_id': self.connection_id,
            'from_token_id': self.from_token_id,
            'to_token_id': self.to_token_id,
            'connection_type': self.connection_type.value,
            'weight': self.weight,
            'bidirectional': self.bidirectional,
            'created_at': self.created_at,
            'last_activated': self.last_activated,
            'activation_count': self.activation_count,
            'generation': self.generation,
            'parent_connection_id': self.parent_connection_id,
            'mutation_applied': self.mutation_applied,
            'metadata': self.metadata
        }


@dataclass
class Subgraph:
    """Subgraph - connected component of the graph"""
    
    subgraph_id: str
    token_ids: Set[int]
    connections: List[Connection]
    
    # Metrics
    density: float = 0.0
    centrality: Dict[int, float] = field(default_factory=dict)
    
    # Semantics
    semantic_label: Optional[str] = None
    quality_score: float = 0.0
    
    # Evolution
    generation: int = 0
    parent_subgraph_ids: List[str] = field(default_factory=list)
    
    def calculate_density(self) -> float:
        """Calculate connection density"""
        n = len(self.token_ids)
        if n < 2:
            return 0.0
        m = len(self.connections)
        max_edges = n * (n - 1) // 2
        self.density = m / max_edges if max_edges > 0 else 0.0
        return self.density
    
    def calculate_centrality(self) -> Dict[int, float]:
        """Calculate node centrality (degree centrality)"""
        degree = defaultdict(int)
        for conn in self.connections:
            degree[conn.from_token_id] += 1
            if conn.bidirectional:
                degree[conn.to_token_id] += 1
            else:
                degree[conn.to_token_id] += 0.5
        
        n = len(self.token_ids)
        if n <= 1:
            return {}
        
        # Normalize by max possible degree
        max_degree = n - 1
        self.centrality = {tid: deg / max_degree for tid, deg in degree.items()}
        return self.centrality
    
    def get_hub_nodes(self, top_k: int = 5) -> List[int]:
        """Get hub nodes (highest centrality)"""
        if not self.centrality:
            self.calculate_centrality()
        return sorted(self.centrality.keys(), 
                     key=lambda x: self.centrality[x], 
                     reverse=True)[:top_k]


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


# === GRAPH CDNA VALIDATOR ===

class GraphCDNAValidator:
    """Validator for graph operations through CDNA"""
    
    def __init__(self, cdna_rules: Optional[Dict[str, Any]] = None):
        """
        Initialize validator
        
        Args:
            cdna_rules: CDNA rules dictionary
        """
        self.cdna_rules = cdna_rules or self._default_rules()
    
    def _default_rules(self) -> Dict[str, Any]:
        """Default CDNA rules"""
        return {
            'allowed_connection_types': [ct.value for ct in ConnectionType],
            'max_degree': 1000,
            'connection_distance_range': [0.0, 1000.0],
            'min_weight': 0.01,
            'max_weight': 1.0,
            'allow_self_loops': False,
            'allow_multi_edges': False
        }
    
    def validate_connection(self, connection: Connection) -> bool:
        """Validate connection"""
        # Check connection type
        if connection.connection_type.value not in self.cdna_rules['allowed_connection_types']:
            return False
        
        # Check weight
        if not (self.cdna_rules['min_weight'] <= connection.weight <= self.cdna_rules['max_weight']):
            return False
        
        # Check self-loop
        if not self.cdna_rules['allow_self_loops']:
            if connection.from_token_id == connection.to_token_id:
                return False
        
        return True
    
    def validate_degree(self, token_id: int, current_degree: int) -> bool:
        """Validate node degree"""
        return current_degree < self.cdna_rules['max_degree']
    
    def validate_distance(self, distance: float) -> bool:
        """Validate distance between tokens"""
        min_dist, max_dist = self.cdna_rules['connection_distance_range']
        return min_dist <= distance <= max_dist


# === ОСНОВНОЙ КЛАСС ГРАФА ===

class TokenGraph:
    """
    Граф токенов - индексная структура для связей между токенами
    без дублирования данных токенов
    """
    
    def __init__(self, sparse_grid: Optional[SparseGrid] = None,
                 config: Optional[Dict[str, Any]] = None,
                 cdna_validator: Optional[GraphCDNAValidator] = None,
                 experience_stream = None):
        """
        Инициализирует граф токенов
        
        Args:
            sparse_grid: Разреженная сетка для пространственного индексирования
            config: Graph configuration
            cdna_validator: CDNA validator instance
            experience_stream: Experience stream for recording events
        """
        self.sparse_grid = sparse_grid or SparseGrid()
        self.config = config or self._default_config()
        self.cdna_validator = cdna_validator or GraphCDNAValidator()
        self.experience_stream = experience_stream
        
        # Основные структуры данных
        self.tokens: Dict[int, Token] = {}                    # ID -> Token
        self.adjacency: Dict[int, Set[int]] = {}             # ID -> Set[connected_IDs]  
        self.edge_metadata: Dict[Tuple[int, int], ConnectionMetadata] = {}  # (ID1,ID2) -> metadata
        self.temporal_index: SortedList = SortedList()       # (timestamp, token_id)

        # Enhanced connection storage for genetic operations
        self._connections: Dict[str, Connection] = {}  # connection_id -> Connection
        self._adjacency_out: Dict[int, List[str]] = defaultdict(list)  # from_id -> [connection_ids]
        self._adjacency_in: Dict[int, List[str]] = defaultdict(list)   # to_id -> [connection_ids]

        # Кэши и статистика
        self._path_cache: Dict[Tuple[int, int], List[int]] = {}
        self._distance_cache: Dict[Tuple[int, int], float] = {}
        self._stats = GraphStats()
        self._lock = threading.RLock()
        
        # Evolution tracking
        self._generation = 0
        self._mutation_history: List[Dict[str, Any]] = []
        self._crossover_history: List[Dict[str, Any]] = []
        
        # Настройки (legacy support)
        self.auto_connect_spatial = True
        self.auto_connect_temporal = True
        self.spatial_connection_radius = 0.1
        self.temporal_connection_window = 3600  # 1 час
        self.max_connections_per_node = self.config.get('topology', {}).get('max_edges_per_node', 100)
    
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
    
    def _default_config(self) -> Dict[str, Any]:
        """Default configuration"""
        return {
            'topology': {
                'max_nodes': 1000000,
                'max_edges': 10000000,
                'max_edges_per_node': 100,
                'allow_self_loops': False,
                'allow_multi_edges': False
            },
            'evolution': {
                'mutation_rate': 0.01,
                'crossover_rate': 0.05,
                'selection_pressure': 0.7,
                'fitness_threshold': 0.3,
                'max_mutations_per_cycle': 1000
            },
            'weights': {
                'initial_weight': 0.5,
                'decay_rate': 0.99,
                'strengthen_amount': 0.1,
                'min_weight': 0.01,
                'max_weight': 1.0
            },
            'cdna_integration': {
                'validate_all_operations': True,
                'enforce_degree_limit': True,
                'enforce_distance_limit': False
            },
            'experience_integration': {
                'record_all_operations': False,
                'calculate_rewards': False,
                'track_fitness': False
            }
        }
    
    # ========================================================================
    # GENETIC OPERATORS
    # ========================================================================
    
    def mutate_connections(self, mutation_rate: Optional[float] = None,
                          allowed_types: Optional[List[ConnectionType]] = None) -> int:
        """
        Mutate graph connections
        
        Types of mutations:
        1. ADD_CONNECTION - add new connection
        2. REMOVE_CONNECTION - remove weak connection
        3. MODIFY_WEIGHT - modify connection weight
        4. CHANGE_TYPE - change connection type
        5. REVERSE_DIRECTION - reverse connection direction
        
        Args:
            mutation_rate: Mutation rate (default from config)
            allowed_types: Allowed connection types
            
        Returns:
            Number of mutations applied
        """
        with self._lock:
            if mutation_rate is None:
                mutation_rate = self.config['evolution']['mutation_rate']
            
            if allowed_types is None:
                allowed_types = list(ConnectionType)
            
            mutations_applied = 0
            max_mutations = self.config['evolution']['max_mutations_per_cycle']
            
            all_connections = list(self._connections.values())
            
            # Mutate existing connections
            for connection in all_connections:
                if mutations_applied >= max_mutations:
                    break
                
                if random.random() < mutation_rate:
                    mutation_type = random.choice(list(MutationType))
                    
                    if mutation_type == MutationType.MODIFY_WEIGHT:
                        delta = random.uniform(-0.2, 0.2)
                        new_weight = max(0.01, min(1.0, connection.weight + delta))
                        connection.weight = new_weight
                        connection.mutation_applied = "weight_modified"
                        connection.generation += 1
                        mutations_applied += 1
                        
                    elif mutation_type == MutationType.CHANGE_TYPE:
                        new_type = random.choice(allowed_types)
                        if new_type != connection.connection_type:
                            old_type = connection.connection_type
                            connection.connection_type = new_type
                            connection.mutation_applied = f"type_changed_{old_type.value}_to_{new_type.value}"
                            connection.generation += 1
                            mutations_applied += 1
                    
                    elif mutation_type == MutationType.REVERSE_DIRECTION:
                        if not connection.bidirectional:
                            new_conn = Connection(
                                connection_id=f"rev_{connection.connection_id}",
                                from_token_id=connection.to_token_id,
                                to_token_id=connection.from_token_id,
                                connection_type=connection.connection_type,
                                weight=connection.weight,
                                generation=connection.generation + 1,
                                parent_connection_id=connection.connection_id,
                                mutation_applied="reversed"
                            )
                            if self._add_enhanced_connection(new_conn):
                                mutations_applied += 1
            
            # Add new random connections
            if random.random() < mutation_rate and mutations_applied < max_mutations:
                if self._add_random_connection(allowed_types):
                    mutations_applied += 1
            
            # Remove weak connections
            weak_threshold = self.config['weights']['min_weight'] * 2
            connections_to_remove = [
                conn.connection_id for conn in all_connections
                if conn.weight < weak_threshold and random.random() < mutation_rate
            ]
            
            for conn_id in connections_to_remove[:max_mutations - mutations_applied]:
                if self._remove_enhanced_connection(conn_id):
                    mutations_applied += 1
            
            # Update statistics
            self._stats.total_mutations += mutations_applied
            self._mutation_history.append({
                'timestamp': time.time(),
                'mutations_applied': mutations_applied,
                'generation': self._generation
            })
            
            self._record_experience('mutations_applied', {
                'count': mutations_applied,
                'generation': self._generation
            })
            
            return mutations_applied
    
    def crossover_subgraphs(self, subgraph1: Subgraph, subgraph2: Subgraph,
                           crossover_point: float = 0.5) -> Subgraph:
        """
        Crossover two subgraphs
        
        Args:
            subgraph1: First subgraph
            subgraph2: Second subgraph
            crossover_point: Crossover point (0.0 - 1.0)
            
        Returns:
            New offspring subgraph
        """
        with self._lock:
            nodes1 = sorted(list(subgraph1.token_ids))
            nodes2 = sorted(list(subgraph2.token_ids))
            
            cut_point1 = int(len(nodes1) * crossover_point)
            cut_point2 = int(len(nodes2) * crossover_point)
            
            new_nodes = set(nodes1[:cut_point1] + nodes2[cut_point2:])
            new_connections = []
            
            for conn in subgraph1.connections:
                if conn.from_token_id in new_nodes and conn.to_token_id in new_nodes:
                    new_conn = Connection(
                        connection_id=f"cross_{conn.connection_id}_{uuid.uuid4().hex[:4]}",
                        from_token_id=conn.from_token_id,
                        to_token_id=conn.to_token_id,
                        connection_type=conn.connection_type,
                        weight=conn.weight,
                        generation=max(subgraph1.generation, subgraph2.generation) + 1,
                        parent_connection_id=conn.connection_id,
                        mutation_applied="crossover"
                    )
                    new_connections.append(new_conn)
            
            for conn in subgraph2.connections:
                if conn.from_token_id in new_nodes and conn.to_token_id in new_nodes:
                    new_conn = Connection(
                        connection_id=f"cross_{conn.connection_id}_{uuid.uuid4().hex[:4]}",
                        from_token_id=conn.from_token_id,
                        to_token_id=conn.to_token_id,
                        connection_type=conn.connection_type,
                        weight=conn.weight,
                        generation=max(subgraph1.generation, subgraph2.generation) + 1,
                        parent_connection_id=conn.connection_id,
                        mutation_applied="crossover"
                    )
                    new_connections.append(new_conn)
            
            offspring = Subgraph(
                subgraph_id=f"offspring_{int(time.time())}_{uuid.uuid4().hex[:4]}",
                token_ids=new_nodes,
                connections=new_connections,
                generation=max(subgraph1.generation, subgraph2.generation) + 1,
                parent_subgraph_ids=[subgraph1.subgraph_id, subgraph2.subgraph_id]
            )
            
            offspring.calculate_density()
            offspring.calculate_centrality()
            
            self._stats.total_crossovers += 1
            self._crossover_history.append({
                'timestamp': time.time(),
                'parent1': subgraph1.subgraph_id,
                'parent2': subgraph2.subgraph_id,
                'offspring': offspring.subgraph_id,
                'generation': self._generation
            })
            
            return offspring
    
    def apply_selection_pressure(self, fitness_threshold: Optional[float] = None) -> int:
        """
        Apply selection pressure - remove weak connections
        
        Args:
            fitness_threshold: Fitness threshold (default from config)
            
        Returns:
            Number of connections removed
        """
        with self._lock:
            if fitness_threshold is None:
                fitness_threshold = self.config['evolution']['fitness_threshold']
            
            removed_count = 0
            connections_to_remove = []
            
            for connection in self._connections.values():
                fitness = self._calculate_connection_fitness(connection)
                if fitness < fitness_threshold:
                    connections_to_remove.append(connection.connection_id)
            
            for conn_id in connections_to_remove:
                if self._remove_enhanced_connection(conn_id):
                    removed_count += 1
            
            self._record_experience('selection_applied', {
                'removed_count': removed_count,
                'fitness_threshold': fitness_threshold
            })
            
            return removed_count
    
    def calculate_fitness(self) -> float:
        """Calculate overall graph fitness"""
        with self._lock:
            if not self._connections:
                return 0.0
            
            total_fitness = sum(
                self._calculate_connection_fitness(conn)
                for conn in self._connections.values()
            )
            avg_fitness = total_fitness / len(self._connections)
            
            stats = self.get_graph_statistics()
            connectivity_score = min(1.0, stats.clustering_coefficient * 2)
            
            fitness = avg_fitness * 0.7 + connectivity_score * 0.3
            return fitness
    
    def get_subgraph(self, token_ids: List[int]) -> Subgraph:
        """Get subgraph for token set"""
        with self._lock:
            token_set = set(token_ids)
            connections = []
            
            for token_id in token_ids:
                for conn_id in self._adjacency_out.get(token_id, []):
                    if conn_id in self._connections:
                        conn = self._connections[conn_id]
                        if conn.to_token_id in token_set:
                            connections.append(conn)
            
            subgraph = Subgraph(
                subgraph_id=f"subgraph_{uuid.uuid4().hex[:8]}",
                token_ids=token_set,
                connections=connections
            )
            
            subgraph.calculate_density()
            subgraph.calculate_centrality()
            
            return subgraph
    
    def get_connected_component(self, token_id: int) -> Set[int]:
        """Get connected component containing token"""
        with self._lock:
            if token_id not in self.tokens:
                return set()
            
            visited = set()
            queue = deque([token_id])
            
            while queue:
                current_id = queue.popleft()
                
                if current_id in visited:
                    continue
                
                visited.add(current_id)
                
                for conn_id in self._adjacency_out.get(current_id, []):
                    if conn_id in self._connections:
                        conn = self._connections[conn_id]
                        if conn.to_token_id not in visited:
                            queue.append(conn.to_token_id)
                
                for conn_id in self._adjacency_in.get(current_id, []):
                    if conn_id in self._connections:
                        conn = self._connections[conn_id]
                        if conn.from_token_id not in visited:
                            queue.append(conn.from_token_id)
            
            return visited
    
    def advance_generation(self) -> int:
        """Advance to next generation"""
        with self._lock:
            self._generation += 1
            self._stats.generation = self._generation
            return self._generation
    
    def get_generation_info(self) -> Dict[str, Any]:
        """Get generation information"""
        with self._lock:
            return {
                'generation': self._generation,
                'total_mutations': self._stats.total_mutations,
                'total_crossovers': self._stats.total_crossovers,
                'graph_fitness': self.calculate_fitness(),
                'node_count': len(self.tokens),
                'edge_count': len(self._connections)
            }
    
    def decay_weights(self, decay_rate: Optional[float] = None) -> int:
        """Decay all connection weights (forgetting)"""
        with self._lock:
            if decay_rate is None:
                decay_rate = self.config['weights']['decay_rate']
            
            count = 0
            for connection in self._connections.values():
                connection.decay_weight(decay_rate)
                count += 1
            
            return count
    
    def prune_weak_connections(self, threshold: Optional[float] = None) -> int:
        """Remove connections below weight threshold"""
        with self._lock:
            if threshold is None:
                threshold = self.config['weights']['min_weight'] * 1.5
            
            weak_connections = [
                conn.connection_id for conn in self._connections.values()
                if conn.weight < threshold
            ]
            
            removed = 0
            for conn_id in weak_connections:
                if self._remove_enhanced_connection(conn_id):
                    removed += 1
            
            return removed
    
    # ========================================================================
    # PRIVATE HELPER METHODS
    # ========================================================================
    
    def _add_enhanced_connection(self, connection: Connection) -> bool:
        """Add enhanced connection to graph"""
        if connection.from_token_id not in self.tokens or connection.to_token_id not in self.tokens:
            return False
        
        if self.config['cdna_integration']['validate_all_operations']:
            if not self.cdna_validator.validate_connection(connection):
                return False
        
        self._connections[connection.connection_id] = connection
        self._adjacency_out[connection.from_token_id].append(connection.connection_id)
        self._adjacency_in[connection.to_token_id].append(connection.connection_id)
        
        return True
    
    def _remove_enhanced_connection(self, connection_id: str) -> bool:
        """Remove enhanced connection from graph"""
        if connection_id not in self._connections:
            return False
        
        connection = self._connections[connection_id]
        
        self._adjacency_out[connection.from_token_id].remove(connection_id)
        self._adjacency_in[connection.to_token_id].remove(connection_id)
        
        del self._connections[connection_id]
        
        return True
    
    def _add_random_connection(self, allowed_types: List[ConnectionType]) -> bool:
        """Add random connection between nodes"""
        if len(self.tokens) < 2:
            return False
        
        nodes = list(self.tokens.keys())
        from_id = random.choice(nodes)
        to_id = random.choice(nodes)
        
        if from_id == to_id:
            return False
        
        connection = Connection(
            connection_id=f"rand_{uuid.uuid4().hex[:8]}",
            from_token_id=from_id,
            to_token_id=to_id,
            connection_type=random.choice(allowed_types),
            weight=self.config['weights']['initial_weight'],
            generation=self._generation,
            mutation_applied="random_add"
        )
        
        return self._add_enhanced_connection(connection)
    
    def _calculate_connection_fitness(self, connection: Connection) -> float:
        """Calculate connection fitness"""
        weight_score = connection.weight
        
        age_seconds = connection.age_seconds()
        if age_seconds > 0:
            activity_score = connection.activation_count / (age_seconds / 3600)
            activity_score = min(1.0, activity_score)
        else:
            activity_score = 0.0
        
        if connection.last_activated > 0:
            time_since_activation = time.time() - connection.last_activated
            recency_score = 1.0 / (1.0 + time_since_activation / 3600)
        else:
            recency_score = 0.0
        
        fitness = (
            weight_score * 0.4 +
            activity_score * 0.3 +
            recency_score * 0.3
        )
        
        return fitness
    
    def _record_experience(self, event_type: str, data: Dict[str, Any]) -> None:
        """Record experience event"""
        if not self.experience_stream:
            return
        
        if not self.config['experience_integration']['record_all_operations']:
            return
        
        try:
            event = {
                'event_id': f"{event_type}_{uuid.uuid4().hex[:8]}",
                'event_type': event_type,
                'timestamp': time.time(),
                'source_component': 'token_graph',
                'data': data
            }
            
            if hasattr(self.experience_stream, 'write_event'):
                self.experience_stream.write_event(event)
        except Exception:
            pass
    
    def __len__(self) -> int:
        """Возвращает количество токенов в графе"""
        return len(self.tokens)
    
    def __contains__(self, token_id: int) -> bool:
        """Проверяет, содержится ли токен в графе"""
        return token_id in self.tokens
    
    def __repr__(self) -> str:
        return f"TokenGraph(nodes={len(self.tokens)}, edges={self._stats.total_edges})"

