"""Processing Result - results from Rust Core"""

from pydantic import BaseModel, Field
from typing import List, Optional


class NeighborInfo(BaseModel):
    """
    Информация о соседе в семантическом пространстве.
    Возвращается из Grid после обработки.
    """

    token_id: int = Field(
        ...,
        description="ID токена-соседа"
    )

    distance: float = Field(
        ...,
        ge=0.0,
        description="Евклидово расстояние в 8D пространстве"
    )

    similarity: float = Field(
        ...,
        ge=0.0,
        le=1.0,
        description="Косинусное сходство (0.0 = ортогональны, 1.0 = идентичны)"
    )

    energy_link: float = Field(
        default=0.0,
        description="Энергия связи (передано через активацию)"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "token_id": 42,
                "distance": 0.35,
                "similarity": 0.92,
                "energy_link": 0.15
            }
        }


class ProcessingResult(BaseModel):
    """
    Результат обработки сигнала в Rust Core.
    Заполняется после прохождения через SignalSystem -> Grid -> Graph.
    """

    token_id: int = Field(
        default=0,
        description="ID токена, присвоенный Grid"
    )

    neighbors: List[NeighborInfo] = Field(
        default_factory=list,
        description="Ближайшие соседи в Grid"
    )

    energy_delta: float = Field(
        default=0.0,
        description="Изменение энергии в системе"
    )

    activation_spread: int = Field(
        default=0,
        ge=0,
        description="Количество активированных токенов"
    )

    is_novel: bool = Field(
        default=True,
        description="True если сигнал новый (нет близких соседей)"
    )

    triggered_actions: List[str] = Field(
        default_factory=list,
        description="ID активированных действий/агентов"
    )

    anomaly_score: float = Field(
        default=0.0,
        ge=0.0,
        le=1.0,
        description="Оценка аномальности от Guardian"
    )

    processing_time_us: int = Field(
        default=0,
        ge=0,
        description="Время обработки в микросекундах"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "token_id": 1337,
                "neighbors": [
                    {"token_id": 42, "distance": 0.35, "similarity": 0.92, "energy_link": 0.15}
                ],
                "energy_delta": 0.8,
                "activation_spread": 12,
                "is_novel": False,
                "triggered_actions": ["agent_conversation", "log_input"],
                "anomaly_score": 0.02,
                "processing_time_us": 87
            }
        }
