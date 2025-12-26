"""Tests for neurograph.types module."""

from neurograph.types import (
    FeedbackType,
    EmbeddingFormat,
    TokenInfo,
    ConnectionInfo,
)


class TestEnums:
    """Tests for enum types."""

    def test_feedback_type_values(self):
        """Test FeedbackType enum values."""
        assert FeedbackType.POSITIVE.value == "positive"
        assert FeedbackType.NEGATIVE.value == "negative"
        assert FeedbackType.NEUTRAL.value == "neutral"

    def test_embedding_format_values(self):
        """Test EmbeddingFormat enum values."""
        assert EmbeddingFormat.GLOVE.value == "glove"
        assert EmbeddingFormat.WORD2VEC.value == "word2vec"
        assert EmbeddingFormat.FASTTEXT.value == "fasttext"


class TestDataclasses:
    """Tests for dataclass types."""

    def test_token_info_minimal(self):
        """Test TokenInfo with minimal fields."""
        token = TokenInfo(token_id=42, text="cat")
        assert token.token_id == 42
        assert token.text == "cat"
        assert token.embedding is None
        assert token.connection_count == 0
        assert token.metadata is None

    def test_token_info_full(self):
        """Test TokenInfo with all fields."""
        token = TokenInfo(
            token_id=42,
            text="cat",
            embedding=[0.1, 0.2, 0.3],
            connection_count=5,
            metadata={"category": "animal"},
        )
        assert token.token_id == 42
        assert token.text == "cat"
        assert token.embedding == [0.1, 0.2, 0.3]
        assert token.connection_count == 5
        assert token.metadata == {"category": "animal"}

    def test_connection_info_minimal(self):
        """Test ConnectionInfo with minimal fields."""
        conn = ConnectionInfo(source_id=1, target_id=2, weight=0.9)
        assert conn.source_id == 1
        assert conn.target_id == 2
        assert conn.weight == 0.9
        assert conn.metadata is None

    def test_connection_info_full(self):
        """Test ConnectionInfo with all fields."""
        conn = ConnectionInfo(
            source_id=1,
            target_id=2,
            weight=0.9,
            metadata={"type": "semantic"},
        )
        assert conn.source_id == 1
        assert conn.target_id == 2
        assert conn.weight == 0.9
        assert conn.metadata == {"type": "semantic"}
