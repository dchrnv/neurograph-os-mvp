# src/infrastructure/config/config_types.py
"""
Базовые типы и схемы для системы конфигураций
"""
from typing import Dict, Any, Optional, List, Union, TypeVar, Generic
from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
import hashlib
import json
from datetime import datetime

class ConfigFormat(Enum):
    """Поддерживаемые форматы конфигураций"""
    YAML = "yaml"
    JSON = "json"
    
class ConfigPurpose(Enum):
    """Назначение конфигурации"""
    HUMAN = "human"  # Для человека (YAML с комментариями)
    MACHINE = "machine"  # Для машины/ИИ (JSON спецификации)
    CACHE = "cache"  # Кэшированные данные
    DEBUG = "debug"  # Отладочные дампы

@dataclass
class ConfigMetadata:
    """Метаданные конфигурации"""
    path: Path
    format: ConfigFormat
    purpose: ConfigPurpose
    checksum: Optional[str] = None
    loaded_at: Optional[datetime] = None
    environment: Optional[str] = None
    version: str = "1.0.0"
    
    def calculate_checksum(self, content: bytes) -> str:
        """Вычисление контрольной суммы"""
        self.checksum = hashlib.sha256(content).hexdigest()
        return self.checksum

@dataclass
class ConfigSchema:
    """Базовая схема конфигурации"""
    name: str
    version: str
    required_fields: List[str] = field(default_factory=list)
    optional_fields: List[str] = field(default_factory=list)
    field_types: Dict[str, type] = field(default_factory=dict)
    validators: Dict[str, callable] = field(default_factory=dict)
    
    def validate_structure(self, data: Dict[str, Any]) -> bool:
        """Валидация структуры данных"""
        # Проверка обязательных полей
        for field in self.required_fields:
            if field not in data:
                raise ValueError(f"Required field '{field}' missing in {self.name}")
        
        # Проверка типов
        for field, expected_type in self.field_types.items():
            if field in data and not isinstance(data[field], expected_type):
                raise TypeError(
                    f"Field '{field}' expected type {expected_type.__name__}, "
                    f"got {type(data[field]).__name__}"
                )
        
        return True

@dataclass
class ConfigEntry:
    """Единица конфигурации с метаданными"""
    data: Dict[str, Any]
    metadata: ConfigMetadata
    schema: Optional[ConfigSchema] = None
    is_validated: bool = False
    is_cached: bool = False
    
    def to_cache_format(self) -> Dict[str, Any]:
        """Преобразование в формат для кэширования"""
        return {
            "data": self.data,
            "metadata": {
                "path": str(self.metadata.path),
                "format": self.metadata.format.value,
                "purpose": self.metadata.purpose.value,
                "checksum": self.metadata.checksum,
                "loaded_at": self.metadata.loaded_at.isoformat() if self.metadata.loaded_at else None,
                "environment": self.metadata.environment,
                "version": self.metadata.version
            },
            "schema_name": self.schema.name if self.schema else None,
            "is_validated": self.is_validated
        }

# Предопределенные схемы для основных конфигов
@dataclass
class TokenConfigSchema(ConfigSchema):
    """Схема конфигурации токенов"""
    def __init__(self):
        super().__init__(
            name="TokenConfig",
            version="1.0.0",
            required_fields=["token_type", "properties"],
            optional_fields=["metadata", "validators", "transformers"],
            field_types={
                "token_type": str,
                "properties": dict,
                "metadata": dict,
                "validators": list,
                "transformers": list
            }
        )

@dataclass
class GridConfigSchema(ConfigSchema):
    """Схема конфигурации сетки"""
    def __init__(self):
        super().__init__(
            name="GridConfig",
            version="1.0.0",
            required_fields=["dimensions", "cell_size"],
            optional_fields=["boundaries", "optimization"],
            field_types={
                "dimensions": dict,
                "cell_size": (int, float),
                "boundaries": dict,
                "optimization": dict
            }
        )

@dataclass
class GraphConfigSchema(ConfigSchema):
    """Схема конфигурации графа"""
    def __init__(self):
        super().__init__(
            name="GraphConfig",
            version="1.0.0",
            required_fields=["node_types", "edge_types"],
            optional_fields=["algorithms", "visualization"],
            field_types={
                "node_types": list,
                "edge_types": list,
                "algorithms": dict,
                "visualization": dict
            }
        )

@dataclass 
class ProcessorConfigSchema(ConfigSchema):
    """Схема конфигурации процессора"""
    def __init__(self):
        super().__init__(
            name="ProcessorConfig", 
            version="1.0.0",
            required_fields=["pipeline", "workers"],
            optional_fields=["batch_size", "timeout", "retry_policy"],
            field_types={
                "pipeline": list,
                "workers": int,
                "batch_size": int,
                "timeout": (int, float),
                "retry_policy": dict
            }
        )

# Registry схем
SCHEMA_REGISTRY = {
    "token": TokenConfigSchema,
    "grid": GridConfigSchema,
    "graph": GraphConfigSchema,
    "processor": ProcessorConfigSchema
}

T = TypeVar('T')

@dataclass
class ConfigResult(Generic[T]):
    """Результат загрузки конфигурации"""
    success: bool
    data: Optional[T] = None
    error: Optional[str] = None
    warnings: List[str] = field(default_factory=list)
    metadata: Optional[ConfigMetadata] = None