"""
NUMERIC_DIRECT Encoder - simple numeric scaling
"""

from typing import List, Any, Dict
from .base import BaseEncoder


class NumericDirectEncoder(BaseEncoder):
    """
    NUMERIC_DIRECT энкодер.

    Преобразует числовые значения в 8D вектор путём масштабирования
    и распределения по осям.

    Стратегии:
    1. Одно число → распределяется по первой оси, остальные 0
    2. Несколько чисел → заполняются последовательно до 8 измерений
    3. Dict {key: value} → значения заполняют вектор

    Usage:
        encoder = NumericDirectEncoder()

        # Single number
        vector = encoder.encode(42.5)  # [0.425, 0, 0, 0, 0, 0, 0, 0]

        # Multiple numbers
        vector = encoder.encode([1.0, 2.0, 3.0])  # [0.1, 0.2, 0.3, 0, ...]

        # Dict
        vector = encoder.encode({'cpu': 45.7, 'mem': 67.3})
    """

    def __init__(self, scale_factor: float = 100.0):
        """
        Args:
            scale_factor: Делитель для нормализации (по умолчанию 100.0)
                         Значения делятся на scale_factor для приведения к [0, 1]
        """
        self.scale_factor = scale_factor

    def encode(self, data: Any) -> List[float]:
        """
        Закодировать числовые данные в 8D вектор.

        Args:
            data: число, список чисел или dict

        Returns:
            8D вектор [0, 1]
        """
        # Case 1: Single number
        if isinstance(data, (int, float)):
            values = [float(data)]

        # Case 2: List of numbers
        elif isinstance(data, (list, tuple)):
            values = [float(v) for v in data if isinstance(v, (int, float))]

        # Case 3: Dict with numeric values
        elif isinstance(data, dict):
            # Extract numeric values, sorted by key for consistency
            values = [
                float(v)
                for k, v in sorted(data.items())
                if isinstance(v, (int, float))
            ]

        else:
            # Fallback: zero vector
            values = [0.0]

        # Scale to [0, 1] range
        scaled = [v / self.scale_factor for v in values]

        # Clamp to [0, 1]
        scaled = [max(0.0, min(1.0, v)) for v in scaled]

        # Pad or truncate to 8D
        vector = self.pad_or_truncate(scaled, 8)

        return vector


__all__ = [
    "NumericDirectEncoder",
]
