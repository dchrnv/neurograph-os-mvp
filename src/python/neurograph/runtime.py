"""Runtime management for neurograph library."""

from typing import Optional, Dict, Any
from pathlib import Path
import logging

from neurograph.exceptions import RuntimeError, BootstrapError, ConfigError
from neurograph.query import QueryResult, QueryContext
from neurograph.types import EmbeddingFormat
from neurograph.runtime_storage import (
    RuntimeTokenStorage,
    RuntimeConnectionStorage,
    RuntimeGridStorage,
    RuntimeCDNAStorage,
)

logger = logging.getLogger(__name__)


class Config:
    """Configuration for neurograph Runtime.

    Attributes:
        grid_size: Size of the spatial grid (default: 1000)
        dimensions: Embedding dimensions (default: 50)
        learning_rate: Learning rate for updates (default: 0.01)
        max_connections: Max connections per token (default: 100)
        enable_metrics: Enable Prometheus metrics (default: False)
        metrics_port: Port for metrics endpoint (default: 9090)
    """

    def __init__(
        self,
        grid_size: int = 1000,
        dimensions: int = 50,
        learning_rate: float = 0.01,
        max_connections: int = 100,
        enable_metrics: bool = False,
        metrics_port: int = 9090,
    ):
        if grid_size <= 0:
            raise ConfigError(f"grid_size must be positive, got {grid_size}")
        if dimensions <= 0:
            raise ConfigError(f"dimensions must be positive, got {dimensions}")
        if learning_rate <= 0 or learning_rate >= 1:
            raise ConfigError(f"learning_rate must be in (0, 1), got {learning_rate}")
        if max_connections <= 0:
            raise ConfigError(f"max_connections must be positive, got {max_connections}")

        self.grid_size = grid_size
        self.dimensions = dimensions
        self.learning_rate = learning_rate
        self.max_connections = max_connections
        self.enable_metrics = enable_metrics
        self.metrics_port = metrics_port

    def to_dict(self) -> Dict[str, Any]:
        """Convert config to dictionary for FFI."""
        return {
            "grid_size": self.grid_size,
            "dimensions": self.dimensions,
            "learning_rate": self.learning_rate,
            "max_connections": self.max_connections,
            "enable_metrics": self.enable_metrics,
            "metrics_port": self.metrics_port,
        }


class Runtime:
    """Main runtime interface for neurograph.

    Provides access to semantic queries, runtime storage, and system configuration.

    Attributes:
        tokens: Token storage interface (RuntimeTokenStorage)
        connections: Connection storage interface (RuntimeConnectionStorage)
        grid: Spatial grid interface (RuntimeGridStorage)
        cdna: CDNA configuration interface (RuntimeCDNAStorage)

    Example:
        >>> runtime = Runtime()
        >>> runtime.bootstrap("glove.6B.50d.txt", limit=50000)
        >>> result = runtime.query("cat")
        >>> print(result.top(5))
        [('dog', 0.92), ('kitten', 0.87), ('pet', 0.81), ...]

        >>> # Use runtime storage
        >>> token_id = runtime.tokens.create(weight=1.0)
        >>> neighbors = runtime.grid.find_neighbors(token_id, radius=5.0)
    """

    def __init__(self, config: Optional[Config] = None):
        """Initialize runtime.

        Args:
            config: Optional configuration. Uses defaults if None.

        Raises:
            RuntimeError: If initialization fails.
        """
        self._config = config or Config()
        self._initialized = False

        # Initialize PyO3 FFI wrapper
        try:
            from neurograph import _core
            self._core = _core.PyRuntime(self._config.to_dict())
        except ImportError:
            # FFI not built yet - use stub mode
            self._core = None
            logger.warning("FFI module not available - running in stub mode")

        # Initialize storage interfaces
        if self._core is not None:
            self.tokens = RuntimeTokenStorage(self._core)
            self.connections = RuntimeConnectionStorage(self._core)
            self.grid = RuntimeGridStorage(self._core)
            self.cdna = RuntimeCDNAStorage(self._core)
        else:
            self.tokens = None
            self.connections = None
            self.grid = None
            self.cdna = None

        logger.info(f"Runtime initialized with config: {self._config.to_dict()}")

    def bootstrap(
        self,
        path: str,
        format: EmbeddingFormat = EmbeddingFormat.GLOVE,
        limit: Optional[int] = None,
        progress: bool = True,
    ) -> None:
        """Load embeddings from file.

        Args:
            path: Path to embeddings file
            format: Embedding format (default: GLOVE)
            limit: Optional limit on number of embeddings to load
            progress: Show progress bar (default: True)

        Raises:
            BootstrapError: If loading fails
        """
        if not Path(path).exists():
            raise BootstrapError(f"Embeddings file not found: {path}")

        logger.info(f"Loading embeddings from {path} (format={format.value}, limit={limit})")

        if self._core is not None:
            # Call FFI bootstrap
            self._core.bootstrap(path, format.value, limit, progress)
            self._initialized = True
        else:
            raise RuntimeError("FFI module not available. Please rebuild with: maturin develop")

        logger.info("Bootstrap completed successfully")

    def query(
        self,
        text: str,
        top_k: int = 10,
        context: Optional[QueryContext] = None,
    ) -> QueryResult:
        """Execute semantic query.

        Args:
            text: Query text
            top_k: Number of results to return (default: 10)
            context: Optional query context for filtering

        Returns:
            QueryResult with similar tokens

        Raises:
            RuntimeError: If runtime not initialized
            QueryError: If query execution fails
        """
        if not self._initialized:
            raise RuntimeError("Runtime not initialized. Call bootstrap() first.")

        logger.debug(f"Executing query: '{text}' (top_k={top_k})")

        if self._core is not None:
            # Call FFI query
            context_dict = context.to_dict() if context else None
            signal_id, results = self._core.query(text, top_k, context_dict)
            return QueryResult(signal_id, results, self)
        else:
            raise RuntimeError("FFI module not available. Please rebuild with: maturin develop")

    def feedback(self, signal_id: str, feedback_type: str) -> None:
        """Provide feedback on query result.

        Args:
            signal_id: Signal ID from QueryResult
            feedback_type: "positive", "negative", or "neutral"

        Raises:
            RuntimeError: If runtime not initialized
        """
        if not self._initialized:
            raise RuntimeError("Runtime not initialized. Call bootstrap() first.")

        logger.debug(f"Feedback: signal_id={signal_id}, type={feedback_type}")

        if self._core is not None:
            # Call FFI feedback
            self._core.feedback(signal_id, feedback_type)
        else:
            raise RuntimeError("FFI module not available. Please rebuild with: maturin develop")

    def export_metrics(self) -> str:
        """Export Prometheus metrics.

        Returns:
            Metrics in Prometheus text format

        Raises:
            RuntimeError: If metrics not enabled
        """
        if not self._config.enable_metrics:
            raise RuntimeError("Metrics not enabled in config")

        if self._core is not None:
            # Call FFI export_metrics
            return self._core.export_metrics()
        else:
            raise RuntimeError("FFI module not available. Please rebuild with: maturin develop")

    def __enter__(self):
        """Context manager entry."""
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit."""
        # TODO: Cleanup FFI resources
        pass
