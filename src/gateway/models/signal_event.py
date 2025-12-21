"""SignalEvent - master event model"""

from pydantic import BaseModel, Field
from typing import Optional
import uuid

from .source import SignalSource
from .semantic import SemanticCore
from .energy import EnergyProfile
from .temporal import TemporalBinding
from .payload import RawPayload
from .result import ProcessingResult
from .routing import RoutingInfo


class SignalEvent(BaseModel):
    """
    Единый формат события в NeuroGraph OS.

    Представляет сигнал от момента поступления через Gateway
    до завершения обработки в Core и доставки подписчикам.

    Architecture:
    - SignalSource: откуда пришёл сигнал
    - SemanticCore: 8D вектор + разложение по слоям
    - EnergyProfile: интенсивность, валентность, срочность
    - TemporalBinding: временная привязка
    - RawPayload: исходные данные
    - ProcessingResult: результаты из Core (опционально)
    - RoutingInfo: маршрутизация и трассировка
    """

    # ═══════════════════════════════════════════════════════════════════════════════
    # CORE IDENTITY
    # ═══════════════════════════════════════════════════════════════════════════════

    event_id: str = Field(
        default_factory=lambda: str(uuid.uuid4()),
        description="Уникальный ID события (UUID4)"
    )

    event_type: str = Field(
        ...,
        description="Иерархический тип события: signal.input.external.text.chat"
    )

    event_type_id: int = Field(
        default=0,
        description="Numeric ID типа события (присваивается EventTypeRegistry)"
    )

    # ═══════════════════════════════════════════════════════════════════════════════
    # NESTED STRUCTURES
    # ═══════════════════════════════════════════════════════════════════════════════

    source: SignalSource = Field(
        ...,
        description="Источник сигнала"
    )

    semantic: SemanticCore = Field(
        ...,
        description="Семантическое представление в 8D пространстве"
    )

    energy: EnergyProfile = Field(
        default_factory=EnergyProfile,
        description="Энергетический профиль"
    )

    temporal: TemporalBinding = Field(
        ...,
        description="Временная привязка"
    )

    payload: RawPayload = Field(
        ...,
        description="Исходные данные сигнала"
    )

    result: Optional[ProcessingResult] = Field(
        default=None,
        description="Результат обработки в Core (заполняется после emit)"
    )

    routing: RoutingInfo = Field(
        default_factory=RoutingInfo,
        description="Информация о маршрутизации"
    )

    # ═══════════════════════════════════════════════════════════════════════════════
    # VERSIONING
    # ═══════════════════════════════════════════════════════════════════════════════

    schema_version: str = Field(
        default="2.0.0",
        description="Версия схемы SignalEvent"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "event_id": "evt_550e8400-e29b-41d4-a716-446655440000",
                "event_type": "signal.input.external.text.chat",
                "event_type_id": 42,
                "source": {
                    "domain": "external",
                    "modality": "text",
                    "sensor_id": "telegram_bot_001",
                    "sensor_type": "text_chat",
                    "confidence": 1.0
                },
                "semantic": {
                    "vector": [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
                    "encoding_method": "tfidf_projection"
                },
                "energy": {
                    "magnitude": 0.8,
                    "valence": 0.6,
                    "arousal": 0.7,
                    "urgency": 0.9
                },
                "temporal": {
                    "timestamp_us": 1735000000000000,
                    "neuro_tick": 12345,
                    "sequence_id": "conv_abc123"
                },
                "payload": {
                    "data_type": "text",
                    "data": "Hello, NeuroGraph!",
                    "mime_type": "text/plain",
                    "size_bytes": 18
                },
                "routing": {
                    "priority": 200,
                    "tags": ["user_input", "chat"]
                },
                "schema_version": "2.0.0"
            }
        }
