#!/usr/bin/env python3
"""
Graph Manager - высокоуровневый интерфейс для работы с графом токенов
"""

import time
import asyncio
from typing import Dict, List, Optional, Set, Tuple, Any, Iterator
from dataclasses import dataclass
from enum import Enum

from .graph_engine import (
    TokenGraph, ConnectionType, Directionality, PersistenceLevel,
    ConnectionMetadata, GraphStats, TokenFlags, FLAG_HUB, FLAG_ROOT, FLAG_LEAF
)
from ..token.token import Token
from ..spatial import SparseGrid, create_demo_sparse_grid
from ..events import Event, EventType, EventCategory
from ..events.global_bus import GlobalEventBus

# === КОНФИГУРАЦИЯ ГРАФА ===

@dataclass
class GraphConfig:
    """Конфигурация графа"""
    auto_connect_spatial: bool = True
    auto_connect_temporal: bool = True
    spatial_connection_radius: float = 0.1
    temporal_connection_window: int = 3600  # 1 час
    max_connections_per_node: int = 100
    enable_auto_cleanup: bool = True
    cleanup_interval: int = 3600  # 1 час
    enable_statistics: bool = True
    enable_caching: bool = True
    cache_size: int = 10000

# === МЕНЕДЖЕР ГРАФА ===

class GraphManager:
    """
    Высокоуровневый менеджер для работы с графом токенов
    """
    
    def __init__(self, config: Optional[GraphConfig] = None, sparse_grid: Optional[SparseGrid] = None):
        """
        Инициализирует менеджер графа
        
        Args:
            config: Конфигурация графа
            sparse_grid: Разреженная сетка для пространственного индексирования
        """
        self.config = config or GraphConfig()
        self.sparse_grid = sparse_grid or create_demo_sparse_grid()
        
        # Создаем граф
        self.graph = TokenGraph(self.sparse_grid)
        self.graph.auto_connect_spatial = self.config.auto_connect_spatial
        self.graph.auto_connect_temporal = self.config.auto_connect_temporal
        self.graph.spatial_connection_radius = self.config.spatial_connection_radius
        self.graph.temporal_connection_window = self.config.temporal_connection_window
        self.graph.max_connections_per_node = self.config.max_connections_per_node
        
        # Кэши и статистика
        self._query_cache: Dict[str, Any] = {}
        self._last_cleanup = time.time()
        
        # Счетчики операций
        self._operation_counts = {
            'add_token': 0,
            'remove_token': 0,
            'connect': 0,
            'disconnect': 0,
            'query': 0
        }
    
    # === ОСНОВНЫЕ ОПЕРАЦИИ ===
    
    def add_token(self, token: Token) -> bool:
        """
        Добавляет токен в граф
        
        Args:
            token: Токен для добавления
            
        Returns:
            bool: True если добавление успешно
        """
        try:
            # Добавляем в граф
            self.graph.add_token(token)
            
            # Добавляем в пространственную сетку
            self.sparse_grid.place_token_simple(token, 0, 0.0, 0.0, 0.0)  # Временное размещение
            
            # Обновляем счетчики
            self._operation_counts['add_token'] += 1

            # Публикуем событие (неинвазивно), если шина активна
            try:
                if GlobalEventBus.is_initialized() and GlobalEventBus.is_running():
                    bus = GlobalEventBus.get()
                    event = Event(
                        type=EventType.GRAPH_STRUCTURE_CHANGED,
                        category=EventCategory.GRAPH,
                        source="graph_manager",
                        payload={
                            "action": "token_added",
                            "token_id": token.id,
                            "total_nodes": len(self.graph.tokens),
                            "total_edges": self.graph._stats.total_edges,
                        },
                    )
                    bus.publish_nowait(event)
            except Exception:
                pass
            
            # Автоматическая очистка
            if self.config.enable_auto_cleanup:
                self._maybe_cleanup()
            
            return True
            
        except Exception as e:
            print(f"Error adding token {token.id}: {e}")
            return False
    
    def remove_token(self, token_id: int) -> bool:
        """
        Удаляет токен из графа
        
        Args:
            token_id: ID токена для удаления
            
        Returns:
            bool: True если удаление успешно
        """
        try:
            # Удаляем из графа
            success = self.graph.remove_token(token_id)
            
            if success:
                # Удаляем из пространственной сетки
                self.sparse_grid.remove_token(token_id)
                
                # Обновляем счетчики
                self._operation_counts['remove_token'] += 1
                
                # Очищаем кэши
                self._clear_caches_for_token(token_id)
            
            return success
            
        except Exception as e:
            print(f"Error removing token {token_id}: {e}")
            return False
    
    def connect_tokens(self, token1_id: int, token2_id: int, 
                      connection_type: ConnectionType = ConnectionType.ASSOCIATION,
                      weight: float = 1.0, confidence: float = 1.0) -> bool:
        """
        Создает связь между токенами
        
        Args:
            token1_id: ID первого токена
            token2_id: ID второго токена
            connection_type: Тип связи
            weight: Вес связи
            confidence: Уверенность в связи
            
        Returns:
            bool: True если связь создана успешно
        """
        try:
            success = self.graph.connect(token1_id, token2_id, connection_type, weight, confidence)
            
            if success:
                self._operation_counts['connect'] += 1
                self._clear_caches_for_edge(token1_id, token2_id)

                # Событие о добавлении связи
                try:
                    if GlobalEventBus.is_initialized() and GlobalEventBus.is_running():
                        bus = GlobalEventBus.get()
                        event = Event(
                            type=EventType.GRAPH_CONNECTION_ADDED,
                            category=EventCategory.GRAPH,
                            source="graph_manager",
                            payload={
                                "source": token1_id,
                                "target": token2_id,
                                "connection_type": connection_type.name if hasattr(connection_type, 'name') else str(connection_type),
                                "weight": weight,
                                "confidence": confidence,
                                "total_edges": self.graph._stats.total_edges,
                            },
                        )
                        bus.publish_nowait(event)
                except Exception:
                    pass
            
            return success
            
        except Exception as e:
            print(f"Error connecting tokens {token1_id}-{token2_id}: {e}")
            return False
    
    def disconnect_tokens(self, token1_id: int, token2_id: int) -> bool:
        """
        Удаляет связь между токенами
        
        Args:
            token1_id: ID первого токена
            token2_id: ID второго токена
            
        Returns:
            bool: True если связь удалена успешно
        """
        try:
            success = self.graph.disconnect(token1_id, token2_id)
            
            if success:
                self._operation_counts['disconnect'] += 1
                self._clear_caches_for_edge(token1_id, token2_id)

                # Событие об удалении связи
                try:
                    if GlobalEventBus.is_initialized() and GlobalEventBus.is_running():
                        bus = GlobalEventBus.get()
                        event = Event(
                            type=EventType.GRAPH_CONNECTION_REMOVED,
                            category=EventCategory.GRAPH,
                            source="graph_manager",
                            payload={
                                "source": token1_id,
                                "target": token2_id,
                                "total_edges": self.graph._stats.total_edges,
                            },
                        )
                        bus.publish_nowait(event)
                except Exception:
                    pass
            
            return success
            
        except Exception as e:
            print(f"Error disconnecting tokens {token1_id}-{token2_id}: {e}")
            return False
    
    # === ПОИСК И НАВИГАЦИЯ ===
    
    def find_token(self, token_id: int) -> Optional[Token]:
        """
        Находит токен по ID
        
        Args:
            token_id: ID токена
            
        Returns:
            Token: Найденный токен или None
        """
        return self.graph.tokens.get(token_id)
    
    def find_neighbors(self, token_id: int) -> List[Token]:
        """
        Находит соседей токена
        
        Args:
            token_id: ID токена
            
        Returns:
            List[Token]: Список соседних токенов
        """
        return self.graph.get_neighbors(token_id)
    
    def find_path(self, start_id: int, end_id: int, max_depth: int = 10) -> List[Token]:
        """
        Находит путь между токенами
        
        Args:
            start_id: ID начального токена
            end_id: ID конечного токена
            max_depth: Максимальная глубина поиска
            
        Returns:
            List[Token]: Список токенов в пути
        """
        path_ids = self.graph.find_path(start_id, end_id, max_depth)
        return [self.graph.tokens[tid] for tid in path_ids if tid in self.graph.tokens]
    
    def find_hub_tokens(self, min_connections: int = 10) -> List[Token]:
        """
        Находит токены-хабы (с большим количеством связей)
        
        Args:
            min_connections: Минимальное количество связей
            
        Returns:
            List[Token]: Список токенов-хабов
        """
        hub_tokens = []
        for token_id, token in self.graph.tokens.items():
            if len(self.graph.adjacency[token_id]) >= min_connections:
                hub_tokens.append(token)
        return hub_tokens
    
    def find_leaf_tokens(self) -> List[Token]:
        """
        Находит листовые токены (без связей)
        
        Returns:
            List[Token]: Список листовых токенов
        """
        leaf_tokens = []
        for token_id, token in self.graph.tokens.items():
            if len(self.graph.adjacency[token_id]) == 0:
                leaf_tokens.append(token)
        return leaf_tokens
    
    def find_root_tokens(self) -> List[Token]:
        """
        Находит корневые токены (с одной связью)
        
        Returns:
            List[Token]: Список корневых токенов
        """
        root_tokens = []
        for token_id, token in self.graph.tokens.items():
            if len(self.graph.adjacency[token_id]) == 1:
                root_tokens.append(token)
        return root_tokens
    
    def find_spatial_clusters(self, level: int = 0, radius: float = 0.5) -> List[List[Token]]:
        """
        Находит пространственные кластеры токенов
        
        Args:
            level: Уровень пространства
            radius: Радиус кластеризации
            
        Returns:
            List[List[Token]]: Список кластеров
        """
        clusters = []
        processed = set()
        
        for token_id, token in self.graph.tokens.items():
            if token_id in processed:
                continue
            
            # Начинаем новый кластер
            cluster = [token]
            processed.add(token_id)
            
            # Находим все токены в радиусе
            to_process = [token_id]
            while to_process:
                current_id = to_process.pop(0)
                spatial_neighbors = self.graph.find_spatial_neighbors(current_id, level, radius)
                
                for neighbor in spatial_neighbors:
                    if neighbor.id not in processed:
                        cluster.append(neighbor)
                        processed.add(neighbor.id)
                        to_process.append(neighbor.id)
            
            if len(cluster) > 1:  # Только кластеры с несколькими токенами
                clusters.append(cluster)
        
        return clusters
    
    def find_temporal_clusters(self, time_window: int = 3600) -> List[List[Token]]:
        """
        Находит временные кластеры токенов
        
        Args:
            time_window: Временное окно в секундах
            
        Returns:
            List[List[Token]]: Список временных кластеров
        """
        clusters = []
        processed = set()
        
        # Сортируем токены по времени
        sorted_tokens = sorted(self.graph.tokens.values(), key=lambda t: t.timestamp)
        
        for token in sorted_tokens:
            if token.id in processed:
                continue
            
            # Начинаем новый кластер
            cluster = [token]
            processed.add(token.id)
            
            # Находим токены в временном окне
            for other_token in sorted_tokens:
                if (other_token.id not in processed and 
                    abs(other_token.timestamp - token.timestamp) <= time_window):
                    cluster.append(other_token)
                    processed.add(other_token.id)
            
            if len(cluster) > 1:  # Только кластеры с несколькими токенами
                clusters.append(cluster)
        
        return clusters
    
    # === АНАЛИЗ ГРАФА ===
    
    def get_graph_statistics(self) -> GraphStats:
        """
        Получает статистику графа
        
        Returns:
            GraphStats: Статистика графа
        """
        return self.graph.get_graph_statistics()
    
    def get_operation_statistics(self) -> Dict[str, int]:
        """
        Получает статистику операций
        
        Returns:
            Dict[str, int]: Статистика операций
        """
        return self._operation_counts.copy()
    
    def analyze_connectivity(self) -> Dict[str, Any]:
        """
        Анализирует связность графа
        
        Returns:
            Dict[str, Any]: Результаты анализа связности
        """
        stats = self.get_graph_statistics()
        
        analysis = {
            'total_nodes': stats.total_nodes,
            'total_edges': stats.total_edges,
            'average_degree': stats.average_degree,
            'clustering_coefficient': stats.clustering_coefficient,
            'connected_components': stats.connected_components,
            'density': stats.total_edges / max(stats.total_nodes * (stats.total_nodes - 1) / 2, 1),
            'hub_count': len(self.find_hub_tokens()),
            'leaf_count': len(self.find_leaf_tokens()),
            'root_count': len(self.find_root_tokens())
        }
        
        return analysis
    
    def find_central_tokens(self, top_k: int = 10) -> List[Tuple[Token, float]]:
        """
        Находит наиболее центральные токены
        
        Args:
            top_k: Количество топ токенов
            
        Returns:
            List[Tuple[Token, float]]: Список (токен, центральность)
        """
        centrality_scores = []
        
        for token_id, token in self.graph.tokens.items():
            # Простая мера центральности - количество связей
            degree = len(self.graph.adjacency[token_id])
            centrality_scores.append((token, degree))
        
        # Сортируем по убыванию центральности
        centrality_scores.sort(key=lambda x: x[1], reverse=True)
        
        return centrality_scores[:top_k]
    
    # === УПРАВЛЕНИЕ КЭШАМИ ===
    
    def clear_caches(self) -> None:
        """Очищает все кэши"""
        self._query_cache.clear()
        self.graph._path_cache.clear()
        self.graph._distance_cache.clear()
    
    def _clear_caches_for_token(self, token_id: int) -> None:
        """Очищает кэши для токена"""
        # Очищаем кэши запросов, содержащие этот токен
        keys_to_remove = []
        for key in self._query_cache:
            if str(token_id) in key:
                keys_to_remove.append(key)
        for key in keys_to_remove:
            del self._query_cache[key]
    
    def _clear_caches_for_edge(self, token1_id: int, token2_id: int) -> None:
        """Очищает кэши для связи"""
        # Очищаем кэши запросов, содержащие эту связь
        keys_to_remove = []
        for key in self._query_cache:
            if str(token1_id) in key and str(token2_id) in key:
                keys_to_remove.append(key)
        for key in keys_to_remove:
            del self._query_cache[key]
    
    def _maybe_cleanup(self) -> None:
        """Выполняет автоматическую очистку при необходимости"""
        current_time = time.time()
        if current_time - self._last_cleanup > self.config.cleanup_interval:
            self.clear_caches()
            self._last_cleanup = current_time
    
    # === СЛУЖЕБНЫЕ МЕТОДЫ ===
    
    def get_token_count(self) -> int:
        """Возвращает количество токенов в графе"""
        return len(self.graph.tokens)
    
    def get_edge_count(self) -> int:
        """Возвращает количество связей в графе"""
        return self.graph._stats.total_edges
    
    def is_connected(self, token1_id: int, token2_id: int) -> bool:
        """Проверяет, связаны ли токены"""
        return token2_id in self.graph.adjacency.get(token1_id, set())
    
    def get_connection_metadata(self, token1_id: int, token2_id: int) -> Optional[ConnectionMetadata]:
        """Получает метаданные связи между токенами"""
        return self.graph.get_connection_metadata(token1_id, token2_id)
    
    def __len__(self) -> int:
        """Возвращает количество токенов в графе"""
        return len(self.graph.tokens)
    
    def __contains__(self, token_id: int) -> bool:
        """Проверяет, содержится ли токен в графе"""
        return token_id in self.graph.tokens
    
    def __repr__(self) -> str:
        return f"GraphManager(nodes={len(self.graph.tokens)}, edges={self.graph._stats.total_edges})"
