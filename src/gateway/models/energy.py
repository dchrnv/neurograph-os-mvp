"""Energy Profile - signal energy characteristics"""

from pydantic import BaseModel, Field


class EnergyProfile(BaseModel):
    """
    Энергетический профиль сигнала.
    Определяет интенсивность, эмоциональную окраску и срочность.
    """

    magnitude: float = Field(
        default=1.0,
        ge=0.0,
        description="Общая интенсивность сигнала (0.0 = слабый, высокие значения = сильный)"
    )

    valence: float = Field(
        default=0.0,
        ge=-1.0,
        le=1.0,
        description="Эмоциональная валентность (-1.0 = негативный, +1.0 = позитивный)"
    )

    arousal: float = Field(
        default=0.5,
        ge=0.0,
        le=1.0,
        description="Уровень активации (0.0 = спокойный, 1.0 = возбуждённый)"
    )

    urgency: float = Field(
        default=0.5,
        ge=0.0,
        le=1.0,
        description="Срочность обработки (0.0 = фоновый, 1.0 = критический)"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "magnitude": 0.8,
                "valence": 0.6,
                "arousal": 0.7,
                "urgency": 0.9
            }
        }
