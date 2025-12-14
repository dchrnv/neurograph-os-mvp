"""Query execution and result handling."""

from typing import List, Optional, Dict, Any, TYPE_CHECKING
from dataclasses import dataclass
import logging

from neurograph.types import SimilarityPair, SimilarityList
from neurograph.exceptions import QueryError

if TYPE_CHECKING:
    from neurograph.runtime import Runtime

logger = logging.getLogger(__name__)


@dataclass
class QueryContext:
    """Context for query execution.

    Attributes:
        filters: Optional filters to apply
        boost_terms: Terms to boost in results
        exclude_terms: Terms to exclude from results
    """
    filters: Optional[Dict[str, Any]] = None
    boost_terms: Optional[List[str]] = None
    exclude_terms: Optional[List[str]] = None

    def to_dict(self) -> Dict[str, Any]:
        """Convert context to dictionary for FFI."""
        return {
            "filters": self.filters,
            "boost_terms": self.boost_terms,
            "exclude_terms": self.exclude_terms,
        }


class QueryResult:
    """Result from a semantic query.

    Attributes:
        signal_id: Unique ID for this query signal
        results: List of (term, similarity) tuples
        runtime: Runtime instance for feedback
    """

    def __init__(
        self,
        signal_id: str,
        results: SimilarityList,
        runtime: 'Runtime',
    ):
        """Initialize QueryResult.

        Args:
            signal_id: Unique signal ID
            results: List of (term, similarity) tuples
            runtime: Runtime instance
        """
        self.signal_id = signal_id
        self._results = results
        self._runtime = runtime

        logger.debug(f"QueryResult created: signal_id={signal_id}, count={len(results)}")

    def top(self, k: int = 5) -> SimilarityList:
        """Get top-k results.

        Args:
            k: Number of results to return

        Returns:
            List of (term, similarity) tuples
        """
        return self._results[:k]

    def all(self) -> SimilarityList:
        """Get all results.

        Returns:
            Complete list of (term, similarity) tuples
        """
        return self._results.copy()

    def filter(self, min_similarity: float = 0.0) -> SimilarityList:
        """Filter results by minimum similarity.

        Args:
            min_similarity: Minimum similarity threshold

        Returns:
            Filtered list of (term, similarity) tuples
        """
        return [(term, score) for term, score in self._results if score >= min_similarity]

    def contains(self, term: str) -> bool:
        """Check if term is in results.

        Args:
            term: Term to search for

        Returns:
            True if term found in results
        """
        return any(t == term for t, _ in self._results)

    def get_similarity(self, term: str) -> Optional[float]:
        """Get similarity score for term.

        Args:
            term: Term to look up

        Returns:
            Similarity score or None if not found
        """
        for t, score in self._results:
            if t == term:
                return score
        return None

    def feedback(self, feedback_type: str) -> None:
        """Provide feedback on this result.

        Args:
            feedback_type: "positive", "negative", or "neutral"
        """
        self._runtime.feedback(self.signal_id, feedback_type)
        logger.info(f"Feedback submitted: {feedback_type} for signal {self.signal_id}")

    def __len__(self) -> int:
        """Get number of results."""
        return len(self._results)

    def __iter__(self):
        """Iterate over results."""
        return iter(self._results)

    def __repr__(self) -> str:
        """String representation."""
        preview = self.top(3)
        preview_str = ", ".join(f"{term}={score:.2f}" for term, score in preview)
        return f"QueryResult(signal_id={self.signal_id}, count={len(self)}, top=[{preview_str}])"
