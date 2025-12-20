"""
Structured Logging Configuration for NeuroGraph API

Provides JSON-formatted logging with correlation IDs, request/response tracking,
and environment-based log levels.

Version: v0.52.0
"""

import logging
import json
import sys
from datetime import datetime, timezone
from typing import Any, Dict, Optional
from contextvars import ContextVar
import traceback

# Context variable for correlation ID (thread-safe for async)
correlation_id_var: ContextVar[Optional[str]] = ContextVar('correlation_id', default=None)


class JSONFormatter(logging.Formatter):
    """
    Custom JSON formatter for structured logging.

    Outputs log records as JSON with:
    - timestamp (ISO 8601)
    - level
    - logger name
    - message
    - correlation_id (if available)
    - additional fields (exc_info, stack_info, etc.)
    """

    def format(self, record: logging.LogRecord) -> str:
        """Format log record as JSON."""
        log_data: Dict[str, Any] = {
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "level": record.levelname,
            "logger": record.name,
            "message": record.getMessage(),
        }

        # Add correlation ID if available
        correlation_id = correlation_id_var.get()
        if correlation_id:
            log_data["correlation_id"] = correlation_id

        # Add extra fields from record
        if hasattr(record, "extra"):
            log_data.update(record.extra)

        # Add function/line info for debugging
        if record.levelno >= logging.WARNING:
            log_data["function"] = record.funcName
            log_data["line"] = record.lineno
            log_data["file"] = record.pathname

        # Add exception info if present
        if record.exc_info:
            log_data["exception"] = {
                "type": record.exc_info[0].__name__ if record.exc_info[0] else None,
                "message": str(record.exc_info[1]) if record.exc_info[1] else None,
                "traceback": traceback.format_exception(*record.exc_info)
            }

        # Add stack trace for errors
        if record.stack_info:
            log_data["stack_trace"] = record.stack_info

        return json.dumps(log_data, default=str, ensure_ascii=False)


class StructuredLogger(logging.LoggerAdapter):
    """
    Logger adapter that adds structured data to log records.

    Usage:
        logger = StructuredLogger(logging.getLogger(__name__))
        logger.info("User logged in", extra={"user_id": 123, "ip": "1.2.3.4"})
    """

    def process(self, msg, kwargs):
        """Add correlation ID and extra fields to log record."""
        # Get correlation ID from context
        correlation_id = correlation_id_var.get()

        # Add to extra dict
        if "extra" not in kwargs:
            kwargs["extra"] = {}

        if correlation_id:
            kwargs["extra"]["correlation_id"] = correlation_id

        # Merge adapter extra with call extra
        if self.extra:
            kwargs["extra"].update(self.extra)

        return msg, kwargs


def setup_logging(
    level: str = "INFO",
    json_format: bool = True,
    correlation_tracking: bool = True
) -> None:
    """
    Setup application-wide structured logging.

    Args:
        level: Log level (DEBUG, INFO, WARNING, ERROR, CRITICAL)
        json_format: Use JSON formatter (True) or simple text (False)
        correlation_tracking: Enable correlation ID tracking

    Example:
        setup_logging(level="DEBUG", json_format=True)
    """
    # Convert string level to logging constant
    numeric_level = getattr(logging, level.upper(), logging.INFO)

    # Create handler
    handler = logging.StreamHandler(sys.stdout)

    # Set formatter
    if json_format:
        formatter = JSONFormatter()
    else:
        formatter = logging.Formatter(
            '%(asctime)s - %(name)s - %(levelname)s - %(message)s'
        )

    handler.setFormatter(formatter)

    # Configure root logger
    root_logger = logging.getLogger()
    root_logger.setLevel(numeric_level)
    root_logger.handlers.clear()  # Remove existing handlers
    root_logger.addHandler(handler)

    # Reduce noise from uvicorn and other libraries
    logging.getLogger("uvicorn.access").setLevel(logging.WARNING)
    logging.getLogger("uvicorn.error").setLevel(logging.INFO)

    # Log startup message
    if json_format:
        root_logger.info("Structured logging initialized", extra={
            "log_level": level,
            "json_format": json_format,
            "correlation_tracking": correlation_tracking
        })
    else:
        root_logger.info(f"Logging initialized: level={level}, json={json_format}")


def get_logger(name: str, **extra) -> StructuredLogger:
    """
    Get a structured logger with optional extra fields.

    Args:
        name: Logger name (usually __name__)
        **extra: Extra fields to add to all log records

    Returns:
        StructuredLogger instance

    Example:
        logger = get_logger(__name__, service="api", component="tokens")
        logger.info("Token created", extra={"token_id": 123})
    """
    base_logger = logging.getLogger(name)
    return StructuredLogger(base_logger, extra or {})


def set_correlation_id(correlation_id: str) -> None:
    """
    Set correlation ID for current async context.

    Args:
        correlation_id: Unique request identifier (e.g., UUID)

    Example:
        set_correlation_id(str(uuid.uuid4()))
    """
    correlation_id_var.set(correlation_id)


def get_correlation_id() -> Optional[str]:
    """
    Get correlation ID from current async context.

    Returns:
        Correlation ID or None if not set
    """
    return correlation_id_var.get()


def clear_correlation_id() -> None:
    """Clear correlation ID from current async context."""
    correlation_id_var.set(None)


# Example usage and testing
if __name__ == "__main__":
    # Setup logging in JSON mode
    setup_logging(level="DEBUG", json_format=True)

    # Get logger
    logger = get_logger(__name__, service="test")

    # Test different log levels
    logger.debug("Debug message", extra={"debug_data": "test"})
    logger.info("Info message", extra={"request_id": "123"})
    logger.warning("Warning message")
    logger.error("Error message")

    # Test with correlation ID
    set_correlation_id("corr-123-456")
    logger.info("Message with correlation ID", extra={"user": "admin"})
    clear_correlation_id()

    # Test exception logging
    try:
        raise ValueError("Test exception")
    except Exception:
        logger.exception("Caught exception")
