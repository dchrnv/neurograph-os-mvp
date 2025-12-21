"""Semantic Core - 8D vector representation"""

from pydantic import BaseModel, Field
from typing import List, Optional


class LayerDecomposition(BaseModel):
    """
    Разложение семантики по 8 когнитивным слоям.
    Значения показывают вклад каждого слоя в общий смысл сигнала.
    """

    physical: float = Field(
        default=0.0,
        ge=0.0,
        le=1.0,
        description="Слой 0: физические свойства"
    )
    spatial: float = Field(
        default=0.0,
        ge=0.0,
        le=1.0,
        description="Слой 1: пространственные отношения"
    )
    temporal: float = Field(
        default=0.0,
        ge=0.0,
        le=1.0,
        description="Слой 2: временные отношения"
    )
    causal: float = Field(
        default=0.0,
        ge=0.0,
        le=1.0,
        description="Слой 3: причинно-следственные связи"
    )
    emotional: float = Field(
        default=0.0,
        ge=0.0,
        le=1.0,
        description="Слой 4: эмоциональная окраска"
    )
    social: float = Field(
        default=0.0,
        ge=0.0,
        le=1.0,
        description="Слой 5: социальный контекст"
    )
    abstract: float = Field(
        default=0.0,
        ge=0.0,
        le=1.0,
        description="Слой 6: абстрактные концепции"
    )
    meta: float = Field(
        default=0.0,
        ge=0.0,
        le=1.0,
        description="Слой 7: метакогнитивный уровень"
    )


class SemanticCore(BaseModel):
    """Семантическое ядро сигнала в 8D пространстве."""

    # Основной вектор
    vector: List[float] = Field(
        ...,
        min_length=8,
        max_length=8,
        description="8D координаты"
    )

    # Разложение по семантическим слоям
    layer_decomposition: Optional[LayerDecomposition] = Field(
        default=None,
        description="Вклад каждого из 8 слоёв"
    )

    # Неопределённость
    uncertainty: Optional[List[float]] = Field(
        default=None,
        min_length=8,
        max_length=8,
        description="Погрешность по каждой оси"
    )

    # Метод кодирования
    encoding_method: str = Field(
        default="pca_projection",
        description="pca_projection | transformer | direct | learned | passthrough"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "vector": [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
                "encoding_method": "pca_projection",
                "layer_decomposition": {
                    "physical": 0.2,
                    "emotional": 0.5,
                    "social": 0.3
                }
            }
        }
