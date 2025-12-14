"""Exception classes for neurograph library."""


class NeurographError(Exception):
    """Base exception for all neurograph errors."""
    pass


class RuntimeError(NeurographError):
    """Raised when runtime initialization or operation fails."""
    pass


class QueryError(NeurographError):
    """Raised when query execution fails."""
    pass


class BootstrapError(NeurographError):
    """Raised when bootstrap loading fails."""
    pass


class ConfigError(NeurographError):
    """Raised when configuration is invalid."""
    pass


class FFIError(NeurographError):
    """Raised when FFI call to Rust core fails."""
    pass


class EmbeddingError(NeurographError):
    """Raised when embedding operations fail."""
    pass
