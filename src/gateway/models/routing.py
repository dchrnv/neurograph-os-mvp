"""Routing Info - delivery and tracing metadata"""

from pydantic import BaseModel, Field
from typing import List, Dict, Any, Optional


class RoutingInfo(BaseModel):
    """
    Метаданные маршрутизации сигнала.
    Определяет приоритет, время жизни, теги и трассировку.
    """

    priority: int = Field(
        default=128,
        ge=0,
        le=255,
        description="Приоритет обработки (0 = минимум, 255 = максимум)"
    )

    ttl: Optional[int] = Field(
        default=None,
        ge=0,
        description="Time-to-live в секундах (None = бесконечно)"
    )

    tags: List[str] = Field(
        default_factory=list,
        description="Теги для фильтрации и маршрутизации"
    )

    trace_id: Optional[str] = Field(
        default=None,
        description="Distributed tracing ID (для мультисистемных сценариев)"
    )

    parent_event_id: Optional[str] = Field(
        default=None,
        description="ID родительского события (для цепочек причинности)"
    )

    correlation_id: Optional[str] = Field(
        default=None,
        description="ID корреляции для группировки связанных событий"
    )

    routing_metadata: Dict[str, Any] = Field(
        default_factory=dict,
        description="Дополнительные метаданные для маршрутизации"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "priority": 200,
                "ttl": 3600,
                "tags": ["user_input", "chat", "high_priority"],
                "trace_id": "trace_abc123xyz",
                "parent_event_id": "evt_parent_456",
                "correlation_id": "corr_session_789",
                "routing_metadata": {
                    "retry_count": 0,
                    "route_path": ["gateway", "signal_system", "grid"]
                }
            }
        }
