# src/core/graph/__init__.py
"""
Графовые компоненты NeuroGraph OS

Система для работы с графом токенов, использующая токены как самодостаточные
единицы графа без дублирования данных.
"""

from .graph_engine import (
    TokenGraph,
    ConnectionType,
    Directionality, 
    PersistenceLevel,
    ConnectionMetadata,
    GraphStats,
    TokenFlags,
    TokenWeightEncoder,
    TokenIDExtractor,
    # Константы флагов
    FLAG_ACTIVE,
    FLAG_ROOT,
    FLAG_LEAF,
    FLAG_HUB,
    FLAG_BRIDGE,
    FLAG_TEMPORARY,
    FLAG_LOCKED,
    FLAG_DIRTY,
    FLAG_PROCESSING,
    FLAG_ERROR,
    FLAG_COMPRESSED,
    FLAG_EXTERNAL_REF,
    FLAG_USER_1,
    FLAG_USER_2,
    FLAG_USER_3,
    FLAG_RESERVED
)

from .graph_manager import (
    GraphManager,
    GraphConfig
)

# Версия модуля
__version__ = "1.0.0"

# Публичный API
__all__ = [
    # Основные классы
    "TokenGraph",
    "GraphManager",
    
    # Конфигурация
    "GraphConfig",
    
    # Типы и перечисления
    "ConnectionType",
    "Directionality",
    "PersistenceLevel",
    
    # Структуры данных
    "ConnectionMetadata",
    "GraphStats",
    
    # Утилиты
    "TokenFlags",
    "TokenWeightEncoder", 
    "TokenIDExtractor",
    
    # Константы флагов
    "FLAG_ACTIVE",
    "FLAG_ROOT",
    "FLAG_LEAF",
    "FLAG_HUB",
    "FLAG_BRIDGE",
    "FLAG_TEMPORARY",
    "FLAG_LOCKED",
    "FLAG_DIRTY",
    "FLAG_PROCESSING",
    "FLAG_ERROR",
    "FLAG_COMPRESSED",
    "FLAG_EXTERNAL_REF",
    "FLAG_USER_1",
    "FLAG_USER_2",
    "FLAG_USER_3",
    "FLAG_RESERVED"
]

