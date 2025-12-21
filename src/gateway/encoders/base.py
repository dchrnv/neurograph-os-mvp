"""
Base Encoder Interface
"""

from abc import ABC, abstractmethod
from typing import List, Any


class BaseEncoder(ABC):
    """
    Базовый интерфейс энкодера.

    Все энкодеры должны наследоваться от этого класса
    и реализовать метод encode().
    """

    @abstractmethod
    def encode(self, data: Any) -> List[float]:
        """
        Закодировать данные в 8D вектор.

        Args:
            data: Входные данные (любой тип)

        Returns:
            List из 8 float значений в диапазоне [0, 1]
        """
        pass

    def normalize_vector(self, vector: List[float]) -> List[float]:
        """
        Нормализовать вектор в диапазон [0, 1].

        Args:
            vector: Входной вектор

        Returns:
            Нормализованный вектор [0, 1]
        """
        if not vector:
            return [0.0] * 8

        # Min-max normalization
        min_val = min(vector)
        max_val = max(vector)

        if max_val == min_val:
            return [0.5] * len(vector)

        return [
            (v - min_val) / (max_val - min_val)
            for v in vector
        ]

    def pad_or_truncate(self, vector: List[float], target_length: int = 8) -> List[float]:
        """
        Привести вектор к нужной длине.

        Args:
            vector: Входной вектор
            target_length: Целевая длина (по умолчанию 8)

        Returns:
            Вектор длины target_length
        """
        if len(vector) == target_length:
            return vector
        elif len(vector) < target_length:
            # Pad with zeros
            return vector + [0.0] * (target_length - len(vector))
        else:
            # Truncate
            return vector[:target_length]


__all__ = [
    "BaseEncoder",
]
