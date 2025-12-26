"""Tests for neurograph.query module."""

import pytest
from neurograph.query import QueryResult, QueryContext
from neurograph.types import SimilarityPair


class TestQueryContext:
    """Tests for QueryContext class."""

    def test_default_context(self):
        """Test default query context."""
        context = QueryContext()
        assert context.filters is None
        assert context.boost_terms is None
        assert context.exclude_terms is None

    def test_custom_context(self):
        """Test custom query context."""
        context = QueryContext(
            filters={"category": "animals"},
            boost_terms=["mammal"],
            exclude_terms=["reptile"],
        )
        assert context.filters == {"category": "animals"}
        assert context.boost_terms == ["mammal"]
        assert context.exclude_terms == ["reptile"]


class TestQueryResult:
    """Tests for QueryResult class."""

    @pytest.fixture
    def mock_runtime(self):
        """Create a mock runtime for testing."""
        class MockRuntime:
            def feedback(self, signal_id, feedback_type):
                pass
        return MockRuntime()

    @pytest.fixture
    def sample_results(self):
        """Sample similarity results."""
        return [
            ("dog", 0.92),
            ("kitten", 0.87),
            ("pet", 0.81),
            ("animal", 0.76),
            ("feline", 0.72),
        ]

    def test_query_result_init(self, mock_runtime, sample_results):
        """Test QueryResult initialization."""
        result = QueryResult("signal-123", sample_results, mock_runtime)
        assert result.signal_id == "signal-123"
        assert len(result) == 5

    def test_top_k(self, mock_runtime, sample_results):
        """Test top-k results retrieval."""
        result = QueryResult("signal-123", sample_results, mock_runtime)
        top3 = result.top(3)
        assert len(top3) == 3
        assert top3[0] == ("dog", 0.92)
        assert top3[1] == ("kitten", 0.87)
        assert top3[2] == ("pet", 0.81)

    def test_all_results(self, mock_runtime, sample_results):
        """Test retrieving all results."""
        result = QueryResult("signal-123", sample_results, mock_runtime)
        all_results = result.all()
        assert len(all_results) == 5
        assert all_results == sample_results
        # Verify it's a copy
        all_results.append(("test", 0.5))
        assert len(result.all()) == 5

    def test_filter_by_similarity(self, mock_runtime, sample_results):
        """Test filtering by minimum similarity."""
        result = QueryResult("signal-123", sample_results, mock_runtime)
        filtered = result.filter(min_similarity=0.8)
        assert len(filtered) == 3
        assert all(score >= 0.8 for _, score in filtered)

    def test_contains(self, mock_runtime, sample_results):
        """Test checking if term is in results."""
        result = QueryResult("signal-123", sample_results, mock_runtime)
        assert result.contains("dog") is True
        assert result.contains("cat") is False

    def test_get_similarity(self, mock_runtime, sample_results):
        """Test getting similarity for term."""
        result = QueryResult("signal-123", sample_results, mock_runtime)
        assert result.get_similarity("dog") == 0.92
        assert result.get_similarity("pet") == 0.81
        assert result.get_similarity("unknown") is None

    def test_iteration(self, mock_runtime, sample_results):
        """Test iterating over results."""
        result = QueryResult("signal-123", sample_results, mock_runtime)
        items = list(result)
        assert items == sample_results

    def test_repr(self, mock_runtime, sample_results):
        """Test string representation."""
        result = QueryResult("signal-123", sample_results, mock_runtime)
        repr_str = repr(result)
        assert "signal-123" in repr_str
        assert "count=5" in repr_str
        assert "dog" in repr_str
