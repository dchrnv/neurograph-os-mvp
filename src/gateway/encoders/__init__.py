"""
Gateway Encoders - transform raw data to 8D vectors

Encoders are responsible for converting raw sensory data
into the 8-dimensional semantic space used by NeuroGraph Core.
"""

from enum import Enum
from .base import BaseEncoder
from .passthrough import PassthroughEncoder
from .numeric import NumericDirectEncoder
from .text_tfidf import TextTfidfEncoder
from .sentiment import SentimentSimpleEncoder


class EncoderType(str, Enum):
    """
    Типы энкодеров для преобразования данных в 8D векторы.

    Каждый энкодер определяет стратегию проекции сырых данных
    в семантическое пространство.
    """

    # ═══════════════════════════════════════════════════════════════════════════════
    # BASIC ENCODERS (v0.54.0 MVP)
    # ═══════════════════════════════════════════════════════════════════════════════

    PASSTHROUGH = "passthrough"
    """Прямая передача 8D вектора без трансформации (для отладки)"""

    TEXT_TFIDF = "text_tfidf"
    """TF-IDF + PCA проекция для текстовых данных"""

    NUMERIC_DIRECT = "numeric_direct"
    """Прямое масштабирование числовых значений в [0, 1]"""

    SENTIMENT_SIMPLE = "sentiment_simple"
    """Простой sentiment analysis (polarity, subjectivity → 8D)"""

    # ═══════════════════════════════════════════════════════════════════════════════
    # ADVANCED ENCODERS (Future versions)
    # ═══════════════════════════════════════════════════════════════════════════════

    TRANSFORMER_BERT = "transformer_bert"
    """BERT embeddings → 8D projection (не реализован в MVP)"""

    TRANSFORMER_OPENAI = "transformer_openai"
    """OpenAI embeddings → 8D projection (не реализован в MVP)"""

    AUDIO_MFCC = "audio_mfcc"
    """MFCC features для аудио (не реализован в MVP)"""

    VISION_CLIP = "vision_clip"
    """CLIP embeddings для изображений (не реализован в MVP)"""

    MULTIMODAL_FUSION = "multimodal_fusion"
    """Fusion нескольких модальностей (не реализован в MVP)"""

    LEARNED_AUTOENCODER = "learned_autoencoder"
    """Обучаемый автоэнкодер (не реализован в MVP)"""

    CUSTOM = "custom"
    """Пользовательский энкодер (определяется через callback)"""


__all__ = [
    "EncoderType",
    "BaseEncoder",
    "PassthroughEncoder",
    "NumericDirectEncoder",
    "TextTfidfEncoder",
    "SentimentSimpleEncoder",
]
