"""
NeuroGraph OS - Event System Core Models
Базовые модели для системы событий (сигналов)

Путь: src/core/events/event.py
"""

from enum import Enum
from typing import Any, Dict, List, Optional
from datetime import datetime
from uuid import uuid4
from pydantic import BaseModel, Field, field_validator


class EventPriority(int, Enum):
    """Приоритеты событий"""
    CRITICAL = 10  # Критические события (ошибки, безопасность)
    HIGH = 7       # Высокий приоритет (изменения DNA, эволюция)
    NORMAL = 5     # Обычный приоритет (создание токенов, обновления)
    LOW = 3        # Низкий приоритет (статистика, логи)
    DEBUG = 1      # Отладочные события


class EventCategory(str, Enum):
    """Категории событий для группировки"""
    TOKEN = "token"
    GRAPH = "graph"
    SPATIAL = "spatial"
    DNA = "dna"
    EXPERIENCE = "experience"
    EVOLUTION = "evolution"
    SYSTEM = "system"
    ERROR = "error"


class EventType(str, Enum):
    """Типы событий в системе"""
    # Token events
    TOKEN_CREATED = "token.created"
    TOKEN_UPDATED = "token.updated"
    TOKEN_DELETED = "token.deleted"
    TOKEN_ACTIVATED = "token.activated"
    
    # Graph events
    GRAPH_CONNECTION_ADDED = "graph.connection.added"
    GRAPH_CONNECTION_REMOVED = "graph.connection.removed"
    GRAPH_STRUCTURE_CHANGED = "graph.structure.changed"
    GRAPH_CLUSTER_DETECTED = "graph.cluster.detected"
    
    # Spatial events
    SPATIAL_REGION_CHANGED = "spatial.region.changed"
    SPATIAL_DENSITY_ALERT = "spatial.density.alert"
    SPATIAL_INDEX_REBUILT = "spatial.index.rebuilt"
    
    # DNA events
    DNA_MUTATED = "dna.mutated"
    DNA_VALIDATED = "dna.validated"
    DNA_CONSTRAINT_VIOLATED = "dna.constraint.violated"
    
    # Experience events
    EXPERIENCE_RECORDED = "experience.recorded"
    EXPERIENCE_BATCH_READY = "experience.batch.ready"
    EXPERIENCE_TRAJECTORY_COMPLETED = "experience.trajectory.completed"
    
    # Evolution events
    EVOLUTION_GENERATION_STARTED = "evolution.generation.started"
    EVOLUTION_GENERATION_COMPLETED = "evolution.generation.completed"
    EVOLUTION_FITNESS_IMPROVED = "evolution.fitness.improved"
    EVOLUTION_TRIGGER = "evolution.trigger"
    
    # System events
    SYSTEM_STARTED = "system.started"
    SYSTEM_STOPPED = "system.stopped"
    SYSTEM_CONFIG_CHANGED = "system.config.changed"
    SYSTEM_HEALTH_CHECK = "system.health.check"
    
    # Error events
    ERROR_OCCURRED = "error.occurred"
    ERROR_ACCESS_DENIED = "error.access.denied"
    ERROR_VALIDATION_FAILED = "error.validation.failed"
    ERROR_RESOURCE_EXHAUSTED = "error.resource.exhausted"


class Event(BaseModel):
    """
    Базовая модель события в системе
    
    Это универсальный контейнер для всех типов событий,
    обеспечивающий единообразную структуру коммуникации
    между модулями.
    """
    
    # Метаданные события
    id: str = Field(default_factory=lambda: str(uuid4()), description="Уникальный ID события")
    version: str = Field(default="1.0", description="Версия формата события")
    type: EventType = Field(..., description="Тип события")
    category: EventCategory = Field(..., description="Категория события")
    
    # Временные метки
    timestamp: float = Field(
        default_factory=lambda: datetime.now().timestamp(),
        description="Время создания события (Unix timestamp)"
    )
    
    # Приоритет и маршрутизация
    priority: EventPriority = Field(default=EventPriority.NORMAL, description="Приоритет события")
    source: str = Field(..., description="ID модуля-источника")
    target: Optional[List[str]] = Field(default=None, description="Конкретные получатели (опционально)")
    
    # Данные события
    payload: Dict[str, Any] = Field(default_factory=dict, description="Полезная нагрузка события")
    
    # Контекст и трейсинг
    correlation_id: Optional[str] = Field(default=None, description="ID для связи связанных событий")
    parent_event_id: Optional[str] = Field(default=None, description="ID родительского события")
    trace_id: Optional[str] = Field(default=None, description="ID для распределенного трейсинга")
    
    # Метаданные
    metadata: Dict[str, Any] = Field(default_factory=dict, description="Дополнительные метаданные")
    
    model_config = {
        "use_enum_values": True,
        "json_encoders": {
            datetime: lambda v: v.timestamp()
        }
    }
    
    @field_validator('payload')
    @classmethod
    def validate_payload(cls, v):
        """Валидация payload - не должен быть слишком большим"""
        if not isinstance(v, dict):
            raise ValueError("Payload must be a dictionary")
        return v
    
    def get_category_from_type(self) -> EventCategory:
        """Определить категорию из типа события"""
        type_str = self.type.value if isinstance(self.type, Enum) else self.type
        category_str = type_str.split('.')[0]
        return EventCategory(category_str)
    
    def is_critical(self) -> bool:
        """Проверка, является ли событие критическим"""
        return self.priority >= EventPriority.HIGH
    
    def is_targeted(self) -> bool:
        """Проверка, адресовано ли событие конкретным получателям"""
        return self.target is not None and len(self.target) > 0
    
    def to_dict(self) -> Dict[str, Any]:
        """Сериализация в словарь"""
        return self.model_dump()
    
    def to_json(self) -> str:
        """Сериализация в JSON"""
        return self.model_dump_json()
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'Event':
        """Десериализация из словаря"""
        return cls(**data)
    
    @classmethod
    def from_json(cls, json_str: str) -> 'Event':
        """Десериализация из JSON"""
        return cls.model_validate_json(json_str)
    
    def create_child_event(
        self,
        event_type: EventType,
        payload: Dict[str, Any],
        source: str,
        priority: Optional[EventPriority] = None
    ) -> 'Event':
        """
        Создать дочернее событие (для цепочек событий)
        
        Args:
            event_type: Тип нового события
            payload: Данные нового события
            source: Источник нового события
            priority: Приоритет (наследуется от родителя если не указан)
        
        Returns:
            Новое событие, связанное с текущим
        """
        return Event(
            type=event_type,
            category=self.get_category_from_type(),
            source=source,
            payload=payload,
            priority=priority or self.priority,
            parent_event_id=self.id,
            correlation_id=self.correlation_id or self.id,
            trace_id=self.trace_id or self.id
        )
    
    def __repr__(self) -> str:
        return f"Event(id={self.id[:8]}, type={self.type}, priority={self.priority.name}, source={self.source})"


class EventFilter(BaseModel):
    """Фильтр для подписки на события"""
    
    types: Optional[List[EventType]] = Field(default=None, description="Типы событий")
    categories: Optional[List[EventCategory]] = Field(default=None, description="Категории событий")
    min_priority: Optional[EventPriority] = Field(default=None, description="Минимальный приоритет")
    sources: Optional[List[str]] = Field(default=None, description="Источники событий")
    
    def matches(self, event: Event) -> bool:
        """
        Проверить, соответствует ли событие фильтру
        
        Args:
            event: Событие для проверки
        
        Returns:
            True если событие проходит фильтр
        """
        # Проверка типов
        if self.types and event.type not in self.types:
            return False
        
        # Проверка категорий
        if self.categories and event.category not in self.categories:
            return False
        
        # Проверка приоритета
        if self.min_priority and event.priority < self.min_priority:
            return False
        
        # Проверка источников
        if self.sources and event.source not in self.sources:
            return False
        
        return True
    
    def __repr__(self) -> str:
        parts = []
        if self.types:
            parts.append(f"types={len(self.types)}")
        if self.categories:
            parts.append(f"categories={self.categories}")
        if self.min_priority:
            parts.append(f"min_priority={self.min_priority.name}")
        if self.sources:
            parts.append(f"sources={len(self.sources)}")
        return f"EventFilter({', '.join(parts)})"