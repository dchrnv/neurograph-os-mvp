"""Raw Payload - original signal data"""

from pydantic import BaseModel, Field
from typing import Any, Dict


class RawPayload(BaseModel):
    """
    Исходные данные сигнала (сырой payload).
    Хранит оригинальные данные до обработки энкодерами.
    """

    data_type: str = Field(
        ...,
        description="Тип данных: text | audio | image | json | binary | multimodal"
    )

    data: Any = Field(
        ...,
        description="Сырые данные (str, bytes, dict, list, etc.)"
    )

    encoding: str = Field(
        default="utf-8",
        description="Кодировка для текстовых/бинарных данных"
    )

    mime_type: str = Field(
        default="text/plain",
        description="MIME-тип данных (text/plain, audio/wav, image/png, etc.)"
    )

    size_bytes: int = Field(
        default=0,
        ge=0,
        description="Размер данных в байтах"
    )

    metadata: Dict[str, Any] = Field(
        default_factory=dict,
        description="Дополнительные метаданные payload"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "data_type": "text",
                "data": "Hello, NeuroGraph!",
                "encoding": "utf-8",
                "mime_type": "text/plain",
                "size_bytes": 18,
                "metadata": {"language": "en", "tokenized": False}
            }
        }
