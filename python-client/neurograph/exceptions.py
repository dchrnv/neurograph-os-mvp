"""
NeuroGraph Client Exceptions

All exceptions raised by the NeuroGraph client library.
"""

from typing import Optional, Dict, Any


class NeuroGraphError(Exception):
    """Base exception for all NeuroGraph client errors."""

    def __init__(
        self,
        message: str,
        status_code: Optional[int] = None,
        error_code: Optional[str] = None,
        details: Optional[Dict[str, Any]] = None,
    ):
        super().__init__(message)
        self.message = message
        self.status_code = status_code
        self.error_code = error_code
        self.details = details or {}

    def __str__(self) -> str:
        if self.error_code:
            return f"[{self.error_code}] {self.message}"
        return self.message

    def __repr__(self) -> str:
        return (
            f"{self.__class__.__name__}("
            f"message={self.message!r}, "
            f"status_code={self.status_code}, "
            f"error_code={self.error_code!r})"
        )


class AuthenticationError(NeuroGraphError):
    """
    Authentication failed.

    Raised when:
    - Invalid credentials (username/password)
    - Invalid API key
    - Expired or invalid JWT token
    """

    def __init__(
        self,
        message: str = "Authentication failed",
        status_code: int = 401,
        error_code: Optional[str] = None,
        details: Optional[Dict[str, Any]] = None,
    ):
        super().__init__(message, status_code, error_code, details)


class AuthorizationError(NeuroGraphError):
    """
    Authorization failed.

    Raised when:
    - Insufficient permissions for the operation
    - Access denied to resource
    """

    def __init__(
        self,
        message: str = "Authorization failed",
        status_code: int = 403,
        error_code: Optional[str] = None,
        details: Optional[Dict[str, Any]] = None,
    ):
        super().__init__(message, status_code, error_code, details)


class NotFoundError(NeuroGraphError):
    """
    Resource not found.

    Raised when:
    - Requested resource doesn't exist
    - Invalid endpoint
    """

    def __init__(
        self,
        message: str = "Resource not found",
        status_code: int = 404,
        error_code: Optional[str] = None,
        details: Optional[Dict[str, Any]] = None,
    ):
        super().__init__(message, status_code, error_code, details)


class ValidationError(NeuroGraphError):
    """
    Request validation failed.

    Raised when:
    - Invalid request parameters
    - Missing required fields
    - Type mismatches
    """

    def __init__(
        self,
        message: str = "Validation failed",
        status_code: int = 422,
        error_code: Optional[str] = None,
        details: Optional[Dict[str, Any]] = None,
    ):
        super().__init__(message, status_code, error_code, details)


class RateLimitError(NeuroGraphError):
    """
    Rate limit exceeded.

    Raised when:
    - Too many requests in time window
    - Rate limit quota exhausted
    """

    def __init__(
        self,
        message: str = "Rate limit exceeded",
        status_code: int = 429,
        error_code: Optional[str] = None,
        details: Optional[Dict[str, Any]] = None,
        retry_after: Optional[int] = None,
    ):
        super().__init__(message, status_code, error_code, details)
        self.retry_after = retry_after


class ServerError(NeuroGraphError):
    """
    Server error.

    Raised when:
    - Internal server error (500)
    - Service unavailable (503)
    - Other server-side errors
    """

    def __init__(
        self,
        message: str = "Server error",
        status_code: int = 500,
        error_code: Optional[str] = None,
        details: Optional[Dict[str, Any]] = None,
    ):
        super().__init__(message, status_code, error_code, details)


class ConnectionError(NeuroGraphError):
    """
    Connection error.

    Raised when:
    - Cannot connect to server
    - Network timeout
    - DNS resolution failed
    """

    def __init__(
        self,
        message: str = "Connection failed",
        details: Optional[Dict[str, Any]] = None,
    ):
        super().__init__(message, status_code=None, error_code="CONNECTION_ERROR", details=details)


class TimeoutError(NeuroGraphError):
    """
    Request timeout.

    Raised when:
    - Request exceeds timeout limit
    - No response from server
    """

    def __init__(
        self,
        message: str = "Request timeout",
        details: Optional[Dict[str, Any]] = None,
    ):
        super().__init__(message, status_code=None, error_code="TIMEOUT_ERROR", details=details)
