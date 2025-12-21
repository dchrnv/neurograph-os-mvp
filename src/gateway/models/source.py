"""Signal Source - Sensor description model"""

from pydantic import BaseModel, Field
from typing import Dict, Any


class SignalSource(BaseModel):
    """Описание источника сигнала (сенсора)."""

    # Классификация
    domain: str = Field(
        ...,
        description="external | internal | system"
    )
    modality: str = Field(
        ...,
        description="text | audio | vision | haptic | proprioception | chemical | environment | state"
    )

    # Идентификация сенсора
    sensor_id: str = Field(
        ...,
        description="Уникальный ID: telegram_bot_01, camera_front, mic_array_left"
    )
    sensor_type: str = Field(
        ...,
        description="Тип: chat_interface | microphone | camera | thermometer | custom"
    )
    sensor_meta: Dict[str, Any] = Field(
        default_factory=dict,
        description="Произвольные метаданные"
    )

    # Качество сигнала
    confidence: float = Field(
        default=1.0,
        ge=0.0,
        le=1.0,
        description="Уверенность сенсора в данных"
    )
    noise_level: float = Field(
        default=0.0,
        ge=0.0,
        le=1.0,
        description="Уровень шума"
    )
    calibration_state: str = Field(
        default="calibrated",
        description="calibrated | uncalibrated | degraded"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "domain": "external",
                "modality": "text",
                "sensor_id": "telegram_bot_01",
                "sensor_type": "chat_interface",
                "sensor_meta": {"user_id": "12345", "chat_id": "67890"},
                "confidence": 1.0,
                "noise_level": 0.0,
                "calibration_state": "calibrated"
            }
        }
