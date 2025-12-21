"""Temporal Binding - time and sequence tracking"""

from pydantic import BaseModel, Field
from typing import Optional


class TemporalBinding(BaseModel):
    """
    Временная привязка сигнала.
    Связывает сигнал с физическим временем и внутренним NeuroTick.
    """

    timestamp_us: int = Field(
        ...,
        description="Unix timestamp в микросекундах"
    )

    neuro_tick: int = Field(
        default=0,
        ge=0,
        description="Внутренний tick системы (монотонный счётчик)"
    )

    sequence_id: Optional[str] = Field(
        default=None,
        description="ID последовательности, если сигнал часть conversation/session"
    )

    sequence_position: Optional[int] = Field(
        default=None,
        ge=0,
        description="Позиция в последовательности (0-based)"
    )

    duration_us: Optional[int] = Field(
        default=None,
        ge=0,
        description="Длительность сигнала в микросекундах (для аудио/видео)"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "timestamp_us": 1735000000000000,
                "neuro_tick": 12345,
                "sequence_id": "conv_abc123",
                "sequence_position": 5,
                "duration_us": 3500000
            }
        }
