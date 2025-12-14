"""Type definitions for neurograph library."""

from enum import Enum
from typing import List, Tuple, Dict, Any, Optional
from dataclasses import dataclass


class FeedbackType(Enum):
    """User feedback types for query results."""
    POSITIVE = "positive"
    NEGATIVE = "negative"
    NEUTRAL = "neutral"


class EmbeddingFormat(Enum):
    """Supported embedding file formats."""
    GLOVE = "glove"
    WORD2VEC = "word2vec"
    FASTTEXT = "fasttext"


@dataclass
class TokenInfo:
    """Information about a token in the graph."""
    token_id: int
    text: str
    embedding: Optional[List[float]] = None
    connection_count: int = 0
    metadata: Optional[Dict[str, Any]] = None


@dataclass
class ConnectionInfo:
    """Information about a connection between tokens."""
    source_id: int
    target_id: int
    weight: float
    metadata: Optional[Dict[str, Any]] = None


# Type aliases for common structures
SimilarityPair = Tuple[str, float]
SimilarityList = List[SimilarityPair]
