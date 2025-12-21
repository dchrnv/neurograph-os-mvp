"""
TEXT_TFIDF Encoder - TF-IDF + dimension reduction
"""

from typing import List, Any
import re
import math
from collections import Counter
from .base import BaseEncoder


class TextTfidfEncoder(BaseEncoder):
    """
    TEXT_TFIDF энкодер.

    Использует упрощённый TF-IDF подход для преобразования текста в 8D вектор:
    1. Токенизация текста
    2. Вычисление TF (term frequency)
    3. Проекция в 8D через хэширование слов

    Это MVP версия без полноценного IDF и обучаемой модели.
    В production следует использовать BERT/OpenAI embeddings.

    Usage:
        encoder = TextTfidfEncoder()
        vector = encoder.encode("Hello, NeuroGraph!")
    """

    def __init__(self):
        # Предопределённые "семантические корзины" для проекции
        # Слова хэшируются в одну из 8 корзин
        self.dimension_count = 8

    def encode(self, data: Any) -> List[float]:
        """
        Закодировать текст в 8D вектор.

        Args:
            data: Текстовая строка или dict с ключом 'text'

        Returns:
            8D вектор [0, 1]
        """
        # Extract text
        if isinstance(data, str):
            text = data
        elif isinstance(data, dict) and 'text' in data:
            text = data['text']
        elif isinstance(data, dict) and 'data' in data:
            text = str(data['data'])
        else:
            text = str(data)

        # Tokenize
        tokens = self._tokenize(text)

        if not tokens:
            return [0.0] * 8

        # Compute TF (term frequency)
        tf = Counter(tokens)
        total_tokens = len(tokens)

        # Project tokens into 8 dimensions using hash-based bucketing
        dimension_scores = [0.0] * 8

        for token, count in tf.items():
            # Hash token to dimension (0-7)
            dim = hash(token) % 8

            # Add TF score to that dimension
            score = count / total_tokens
            dimension_scores[dim] += score

        # Normalize to [0, 1]
        vector = self.normalize_vector(dimension_scores)

        return vector

    def _tokenize(self, text: str) -> List[str]:
        """
        Простая токенизация текста.

        Args:
            text: Входной текст

        Returns:
            Список токенов (lowercase, alphanumeric)
        """
        # Lowercase
        text = text.lower()

        # Extract words (alphanumeric sequences)
        tokens = re.findall(r'\b\w+\b', text)

        # Filter out very short tokens and stopwords
        stopwords = {
            'a', 'an', 'the', 'is', 'are', 'was', 'were', 'be', 'been',
            'i', 'you', 'he', 'she', 'it', 'we', 'they',
            'in', 'on', 'at', 'to', 'for', 'of', 'and', 'or', 'but',
        }

        tokens = [
            t for t in tokens
            if len(t) > 2 and t not in stopwords
        ]

        return tokens


__all__ = [
    "TextTfidfEncoder",
]
