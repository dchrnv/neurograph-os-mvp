"""
PASSTHROUGH Encoder - direct vector pass-through
"""

from typing import List, Any
from .base import BaseEncoder


class PassthroughEncoder(BaseEncoder):
    """
    PASSTHROUGH энкодер.

    Принимает уже готовый 8D вектор и возвращает его без изменений.
    Используется для отладки или когда вектор уже вычислен внешней системой.

    Usage:
        encoder = PassthroughEncoder()
        vector = encoder.encode([0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8])
    """

    def encode(self, data: Any) -> List[float]:
        """
        Передать вектор как есть.

        Args:
            data: List[float] длиной 8 или dict с ключом 'vector'

        Returns:
            8D вектор

        Raises:
            ValueError: Если data не подходит
        """
        # Case 1: already a list
        if isinstance(data, (list, tuple)):
            vector = list(data)
        # Case 2: dict with 'vector' key
        elif isinstance(data, dict) and 'vector' in data:
            vector = data['vector']
        else:
            raise ValueError(
                f"PASSTHROUGH encoder requires list or dict with 'vector' key, got {type(data)}"
            )

        # Ensure 8 dimensions
        vector = self.pad_or_truncate(vector, 8)

        # Ensure [0, 1] range
        vector = self.normalize_vector(vector)

        return vector


__all__ = [
    "PassthroughEncoder",
]
