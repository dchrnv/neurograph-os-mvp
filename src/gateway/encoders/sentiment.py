"""
SENTIMENT_SIMPLE Encoder - basic sentiment analysis
"""

from typing import List, Any
import re
from .base import BaseEncoder


class SentimentSimpleEncoder(BaseEncoder):
    """
    SENTIMENT_SIMPLE энкодер.

    Выполняет простой sentiment analysis на основе словаря
    позитивных/негативных слов и проецирует результат в 8D:

    - Dimension 0: Polarity (negative to positive)
    - Dimension 1: Subjectivity (objective to subjective)
    - Dimension 2: Intensity (mild to strong)
    - Dimension 3: Emotion - joy
    - Dimension 4: Emotion - sadness
    - Dimension 5: Emotion - anger
    - Dimension 6: Emotion - fear
    - Dimension 7: Emotion - surprise

    Это MVP версия с ограниченным словарём.
    В production следует использовать ML модели (VADER, BERT-sentiment).

    Usage:
        encoder = SentimentSimpleEncoder()
        vector = encoder.encode("I am very happy today!")
    """

    def __init__(self):
        # Простые словари для MVP
        self.positive_words = {
            'good', 'great', 'awesome', 'excellent', 'love', 'happy', 'wonderful',
            'fantastic', 'amazing', 'best', 'nice', 'perfect', 'beautiful', 'brilliant',
            'joy', 'delighted', 'pleased', 'glad', 'cheerful', 'positive', 'win',
        }

        self.negative_words = {
            'bad', 'terrible', 'awful', 'horrible', 'hate', 'sad', 'angry',
            'worst', 'poor', 'stupid', 'fail', 'failure', 'wrong', 'error',
            'problem', 'issue', 'negative', 'disappointed', 'upset', 'worried',
        }

        self.intensifiers = {
            'very', 'really', 'extremely', 'absolutely', 'totally', 'completely',
            'incredibly', 'super', 'ultra', 'highly', 'too', 'so',
        }

        self.subjective_words = {
            'i', 'think', 'feel', 'believe', 'hope', 'wish', 'want', 'need',
            'seems', 'appears', 'probably', 'maybe', 'perhaps', 'opinion',
        }

        # Emotion-specific words
        self.emotion_words = {
            'joy': {'happy', 'joy', 'excited', 'delighted', 'cheerful', 'glad'},
            'sadness': {'sad', 'unhappy', 'depressed', 'miserable', 'gloomy'},
            'anger': {'angry', 'furious', 'mad', 'annoyed', 'irritated'},
            'fear': {'afraid', 'scared', 'frightened', 'worried', 'anxious'},
            'surprise': {'surprised', 'shocked', 'amazed', 'astonished', 'wow'},
        }

    def encode(self, data: Any) -> List[float]:
        """
        Закодировать текст через sentiment analysis.

        Args:
            data: Текстовая строка

        Returns:
            8D вектор [0, 1]
        """
        # Extract text
        if isinstance(data, str):
            text = data
        elif isinstance(data, dict) and 'text' in data:
            text = data['text']
        else:
            text = str(data)

        # Tokenize
        tokens = self._tokenize(text)

        if not tokens:
            return [0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0]

        # Analyze
        polarity = self._compute_polarity(tokens)
        subjectivity = self._compute_subjectivity(tokens)
        intensity = self._compute_intensity(tokens)
        emotions = self._compute_emotions(tokens)

        # Build 8D vector
        vector = [
            polarity,        # Dim 0: Polarity [-1, +1] → [0, 1]
            subjectivity,    # Dim 1: Subjectivity [0, 1]
            intensity,       # Dim 2: Intensity [0, 1]
            emotions['joy'],       # Dim 3
            emotions['sadness'],   # Dim 4
            emotions['anger'],     # Dim 5
            emotions['fear'],      # Dim 6
            emotions['surprise'],  # Dim 7
        ]

        return vector

    def _tokenize(self, text: str) -> List[str]:
        """Простая токенизация."""
        text = text.lower()
        return re.findall(r'\b\w+\b', text)

    def _compute_polarity(self, tokens: List[str]) -> float:
        """
        Вычислить полярность (positive vs negative).

        Returns:
            Float в [0, 1], где 0 = очень негативный, 1 = очень позитивный, 0.5 = нейтральный
        """
        positive_count = sum(1 for t in tokens if t in self.positive_words)
        negative_count = sum(1 for t in tokens if t in self.negative_words)

        total = positive_count + negative_count
        if total == 0:
            return 0.5  # Neutral

        # Map to [0, 1]
        score = positive_count / total
        return score

    def _compute_subjectivity(self, tokens: List[str]) -> float:
        """
        Вычислить субъективность (objective vs subjective).

        Returns:
            Float в [0, 1], где 0 = объективный, 1 = субъективный
        """
        subjective_count = sum(1 for t in tokens if t in self.subjective_words)
        return min(1.0, subjective_count / max(1, len(tokens)) * 5)

    def _compute_intensity(self, tokens: List[str]) -> float:
        """
        Вычислить интенсивность (mild vs strong).

        Returns:
            Float в [0, 1], где 0 = мягкий, 1 = сильный
        """
        intensifier_count = sum(1 for t in tokens if t in self.intensifiers)
        return min(1.0, intensifier_count / max(1, len(tokens)) * 10)

    def _compute_emotions(self, tokens: List[str]) -> dict:
        """
        Вычислить оценки эмоций.

        Returns:
            Dict {emotion_name: score [0, 1]}
        """
        emotions = {
            'joy': 0.0,
            'sadness': 0.0,
            'anger': 0.0,
            'fear': 0.0,
            'surprise': 0.0,
        }

        token_count = max(1, len(tokens))

        for emotion, words in self.emotion_words.items():
            count = sum(1 for t in tokens if t in words)
            emotions[emotion] = min(1.0, count / token_count * 5)

        return emotions


__all__ = [
    "SentimentSimpleEncoder",
]
