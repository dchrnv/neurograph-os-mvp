"""
Retry logic with exponential backoff.

Provides automatic retry for transient failures.
"""

import time
from typing import Optional, Callable, TypeVar, Any
from functools import wraps

from .exceptions import (
    RateLimitError,
    ServerError,
    ConnectionError,
    TimeoutError,
)


T = TypeVar("T")


class RetryConfig:
    """Configuration for retry behavior."""

    def __init__(
        self,
        max_retries: int = 3,
        initial_delay: float = 1.0,
        max_delay: float = 60.0,
        exponential_base: float = 2.0,
        jitter: bool = True,
    ):
        """
        Initialize retry configuration.

        Args:
            max_retries: Maximum number of retry attempts
            initial_delay: Initial delay in seconds
            max_delay: Maximum delay in seconds
            exponential_base: Base for exponential backoff
            jitter: Add random jitter to prevent thundering herd
        """
        self.max_retries = max_retries
        self.initial_delay = initial_delay
        self.max_delay = max_delay
        self.exponential_base = exponential_base
        self.jitter = jitter

    def get_delay(self, attempt: int) -> float:
        """
        Calculate delay for given attempt.

        Args:
            attempt: Current attempt number (0-indexed)

        Returns:
            Delay in seconds
        """
        # Exponential backoff
        delay = min(
            self.initial_delay * (self.exponential_base ** attempt),
            self.max_delay
        )

        # Add jitter (0-50% of delay)
        if self.jitter:
            import random
            jitter_range = delay * 0.5
            delay += random.uniform(0, jitter_range)

        return delay


def should_retry(exception: Exception) -> bool:
    """
    Determine if exception should trigger retry.

    Retries on:
    - Rate limit errors (429)
    - Server errors (500+)
    - Connection errors
    - Timeout errors

    Does NOT retry on:
    - Authentication errors (401)
    - Authorization errors (403)
    - Not found errors (404)
    - Validation errors (422)

    Args:
        exception: Exception to check

    Returns:
        True if should retry
    """
    return isinstance(exception, (
        RateLimitError,
        ServerError,
        ConnectionError,
        TimeoutError,
    ))


def retry_with_backoff(
    func: Optional[Callable] = None,
    *,
    config: Optional[RetryConfig] = None,
) -> Callable:
    """
    Decorator for automatic retry with exponential backoff.

    Example:
        >>> @retry_with_backoff
        ... def api_call():
        ...     return client.tokens.create(text="test")

        >>> @retry_with_backoff(config=RetryConfig(max_retries=5))
        ... def custom_call():
        ...     return client.tokens.get(123)

    Args:
        func: Function to wrap
        config: Retry configuration

    Returns:
        Wrapped function with retry logic
    """
    if config is None:
        config = RetryConfig()

    def decorator(f: Callable[..., T]) -> Callable[..., T]:
        @wraps(f)
        def wrapper(*args: Any, **kwargs: Any) -> T:
            last_exception = None

            for attempt in range(config.max_retries + 1):
                try:
                    return f(*args, **kwargs)

                except Exception as e:
                    last_exception = e

                    # Don't retry if not a retryable error
                    if not should_retry(e):
                        raise

                    # Don't retry on last attempt
                    if attempt == config.max_retries:
                        raise

                    # Calculate delay
                    if isinstance(e, RateLimitError) and e.retry_after:
                        # Use server-provided retry delay
                        delay = e.retry_after
                    else:
                        delay = config.get_delay(attempt)

                    # Log retry attempt (optional)
                    # logger.warning(
                    #     f"Attempt {attempt + 1}/{config.max_retries} failed: {e}. "
                    #     f"Retrying in {delay:.2f}s..."
                    # )

                    time.sleep(delay)

            # Should never reach here, but just in case
            if last_exception:
                raise last_exception

        return wrapper

    # Support both @retry_with_backoff and @retry_with_backoff(config=...)
    if func is None:
        return decorator
    else:
        return decorator(func)


# Async version
import asyncio


def async_retry_with_backoff(
    func: Optional[Callable] = None,
    *,
    config: Optional[RetryConfig] = None,
) -> Callable:
    """
    Async decorator for automatic retry with exponential backoff.

    Example:
        >>> @async_retry_with_backoff
        ... async def api_call():
        ...     return await client.tokens.create(text="test")

    Args:
        func: Async function to wrap
        config: Retry configuration

    Returns:
        Wrapped async function with retry logic
    """
    if config is None:
        config = RetryConfig()

    def decorator(f: Callable[..., T]) -> Callable[..., T]:
        @wraps(f)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            last_exception = None

            for attempt in range(config.max_retries + 1):
                try:
                    return await f(*args, **kwargs)

                except Exception as e:
                    last_exception = e

                    if not should_retry(e):
                        raise

                    if attempt == config.max_retries:
                        raise

                    if isinstance(e, RateLimitError) and e.retry_after:
                        delay = e.retry_after
                    else:
                        delay = config.get_delay(attempt)

                    await asyncio.sleep(delay)

            if last_exception:
                raise last_exception

        return wrapper

    if func is None:
        return decorator
    else:
        return decorator(func)


# Example usage
if __name__ == "__main__":
    # Test retry logic
    @retry_with_backoff(config=RetryConfig(max_retries=3))
    def flaky_function():
        import random
        if random.random() < 0.7:  # 70% failure rate
            raise ServerError("Simulated server error")
        return "Success!"

    try:
        result = flaky_function()
        print(f"Result: {result}")
    except ServerError as e:
        print(f"Failed after retries: {e}")
