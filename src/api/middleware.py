"""
Middleware for NeuroGraph API

Provides:
- Correlation ID tracking
- Request/Response logging with timing
- Error handling and logging

Version: v0.52.0
"""

import time
import uuid
from typing import Callable
from fastapi import Request, Response
from fastapi.responses import JSONResponse
from starlette.middleware.base import BaseHTTPMiddleware
from starlette.types import ASGIApp

from .logging_config import get_logger, set_correlation_id, clear_correlation_id, get_correlation_id
from .metrics_prometheus import track_http_request, http_requests_in_progress

logger = get_logger(__name__, component="middleware")


class CorrelationIDMiddleware(BaseHTTPMiddleware):
    """
    Middleware to track request correlation IDs.

    Features:
    - Generates UUID for each request
    - Reads X-Correlation-ID header if provided
    - Adds correlation ID to response headers
    - Sets correlation ID in async context for logging
    """

    def __init__(self, app: ASGIApp):
        super().__init__(app)

    async def dispatch(self, request: Request, call_next: Callable) -> Response:
        """Process request and add correlation ID."""
        # Get or generate correlation ID
        correlation_id = request.headers.get("X-Correlation-ID")
        if not correlation_id:
            correlation_id = str(uuid.uuid4())

        # Set in async context for logging
        set_correlation_id(correlation_id)

        try:
            # Process request
            response = await call_next(request)

            # Add correlation ID to response headers
            response.headers["X-Correlation-ID"] = correlation_id

            return response
        finally:
            # Clear correlation ID from context
            clear_correlation_id()


class RequestLoggingMiddleware(BaseHTTPMiddleware):
    """
    Middleware to log HTTP requests and responses with timing.

    Features:
    - Logs request method, path, query params
    - Measures and logs request duration
    - Logs response status code
    - Adds X-Process-Time header
    - Skips health check endpoints to reduce noise
    """

    def __init__(
        self,
        app: ASGIApp,
        skip_paths: list[str] | None = None,
        log_request_body: bool = False,
        log_response_body: bool = False
    ):
        super().__init__(app)
        self.skip_paths = skip_paths or ["/health", "/api/v1/health"]
        self.log_request_body = log_request_body
        self.log_response_body = log_response_body

    async def dispatch(self, request: Request, call_next: Callable) -> Response:
        """Process and log request/response."""
        # Skip logging/metrics for health checks and metrics endpoint itself
        skip_metrics = request.url.path in self.skip_paths or request.url.path == "/api/v1/metrics"

        if skip_metrics:
            return await call_next(request)

        # Start timing
        start_time = time.perf_counter()

        # Track in-progress requests (Prometheus gauge)
        http_requests_in_progress.labels(method=request.method, path=request.url.path).inc()

        # Log incoming request
        logger.info(
            f"{request.method} {request.url.path}",
            extra={
                "event": "request_started",
                "method": request.method,
                "path": request.url.path,
                "query_params": dict(request.query_params),
                "client_ip": request.client.host if request.client else None,
                "user_agent": request.headers.get("user-agent"),
            }
        )

        # Process request
        try:
            response = await call_next(request)

            # Calculate duration
            duration_seconds = time.perf_counter() - start_time
            duration_ms = duration_seconds * 1000

            # Add timing header
            response.headers["X-Process-Time"] = f"{duration_ms:.2f}ms"

            # Track metrics (Prometheus)
            track_http_request(
                method=request.method,
                path=request.url.path,
                status_code=response.status_code,
                duration=duration_seconds
            )

            # Log response
            log_level = "warning" if response.status_code >= 400 else "info"
            log_method = getattr(logger, log_level)

            log_method(
                f"{request.method} {request.url.path} - {response.status_code}",
                extra={
                    "event": "request_completed",
                    "method": request.method,
                    "path": request.url.path,
                    "status_code": response.status_code,
                    "duration_ms": round(duration_ms, 2),
                    "correlation_id": get_correlation_id(),
                }
            )

            return response

        except Exception as exc:
            # Log exception
            duration_seconds = time.perf_counter() - start_time
            duration_ms = duration_seconds * 1000

            # Track failed request metrics
            track_http_request(
                method=request.method,
                path=request.url.path,
                status_code=500,
                duration=duration_seconds
            )

            logger.error(
                f"{request.method} {request.url.path} - ERROR",
                extra={
                    "event": "request_failed",
                    "method": request.method,
                    "path": request.url.path,
                    "duration_ms": round(duration_ms, 2),
                    "error_type": type(exc).__name__,
                    "error_message": str(exc),
                },
                exc_info=True
            )

            # Re-raise to let global exception handler deal with it
            raise

        finally:
            # Decrement in-progress counter
            http_requests_in_progress.labels(method=request.method, path=request.url.path).dec()


class ErrorLoggingMiddleware(BaseHTTPMiddleware):
    """
    Middleware to catch and log unhandled exceptions.

    Features:
    - Catches all unhandled exceptions
    - Logs with full stack trace
    - Returns proper JSON error response
    - Adds error details to response headers (dev mode only)
    """

    def __init__(self, app: ASGIApp, debug: bool = False):
        super().__init__(app)
        self.debug = debug

    async def dispatch(self, request: Request, call_next: Callable) -> Response:
        """Catch and log exceptions."""
        try:
            return await call_next(request)
        except Exception as exc:
            # Log the error
            logger.exception(
                "Unhandled exception in request processing",
                extra={
                    "event": "unhandled_exception",
                    "method": request.method,
                    "path": request.url.path,
                    "error_type": type(exc).__name__,
                    "error_message": str(exc),
                }
            )

            # Build error response
            error_response = {
                "success": False,
                "error": {
                    "code": "INTERNAL_ERROR",
                    "message": "Internal server error",
                }
            }

            # Add details in debug mode
            if self.debug:
                error_response["error"]["details"] = {
                    "type": type(exc).__name__,
                    "message": str(exc),
                }

            # Create response
            response = JSONResponse(
                status_code=500,
                content=error_response
            )

            # Add correlation ID
            correlation_id = get_correlation_id()
            if correlation_id:
                response.headers["X-Correlation-ID"] = correlation_id

            return response


# Example usage
if __name__ == "__main__":
    import asyncio
    from fastapi import FastAPI

    app = FastAPI()

    # Add middlewares
    app.add_middleware(ErrorLoggingMiddleware, debug=True)
    app.add_middleware(RequestLoggingMiddleware)
    app.add_middleware(CorrelationIDMiddleware)

    @app.get("/test")
    async def test_endpoint():
        logger.info("Test endpoint called")
        return {"message": "Hello"}

    @app.get("/error")
    async def error_endpoint():
        raise ValueError("Test error")

    # This would be run with uvicorn in production
    print("Middleware configured successfully!")
