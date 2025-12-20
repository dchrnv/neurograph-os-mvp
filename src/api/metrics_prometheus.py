"""
Prometheus Metrics for NeuroGraph API

Provides comprehensive metrics for monitoring:
- HTTP request latency and counts
- Token operations (create/get/update/delete)
- Grid queries
- CDNA operations
- System resources (from RuntimeStorage)

Version: v0.52.0
"""

from prometheus_client import (
    Counter,
    Histogram,
    Gauge,
    Info,
    generate_latest,
    CONTENT_TYPE_LATEST,
    CollectorRegistry
)
from fastapi import Response
from typing import Optional
import time

# Create custom registry to avoid conflicts
registry = CollectorRegistry()

# ============================================================================
# HTTP Metrics
# ============================================================================

http_requests_total = Counter(
    'neurograph_http_requests_total',
    'Total HTTP requests by method, path, and status',
    ['method', 'path', 'status_code'],
    registry=registry
)

http_request_duration_seconds = Histogram(
    'neurograph_http_request_duration_seconds',
    'HTTP request latency in seconds',
    ['method', 'path'],
    buckets=(0.001, 0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0, 2.5, 5.0, 7.5, 10.0),
    registry=registry
)

http_requests_in_progress = Gauge(
    'neurograph_http_requests_in_progress',
    'Number of HTTP requests currently being processed',
    ['method', 'path'],
    registry=registry
)

# ============================================================================
# Token Operation Metrics
# ============================================================================

token_operations_total = Counter(
    'neurograph_token_operations_total',
    'Total token operations by type',
    ['operation'],  # create, get, update, delete, list
    registry=registry
)

token_operation_duration_seconds = Histogram(
    'neurograph_token_operation_duration_seconds',
    'Token operation duration in seconds',
    ['operation'],
    buckets=(0.00001, 0.00005, 0.0001, 0.0005, 0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0),
    registry=registry
)

tokens_created_total = Counter(
    'neurograph_tokens_created_total',
    'Total number of tokens created',
    registry=registry
)

tokens_deleted_total = Counter(
    'neurograph_tokens_deleted_total',
    'Total number of tokens deleted',
    registry=registry
)

tokens_active_count = Gauge(
    'neurograph_tokens_active_count',
    'Current number of active tokens in RuntimeStorage',
    registry=registry
)

# ============================================================================
# Grid Operation Metrics
# ============================================================================

grid_queries_total = Counter(
    'neurograph_grid_queries_total',
    'Total grid queries by type',
    ['query_type'],  # range, nearest, field_influence
    registry=registry
)

grid_query_duration_seconds = Histogram(
    'neurograph_grid_query_duration_seconds',
    'Grid query duration in seconds',
    ['query_type'],
    buckets=(0.00001, 0.00005, 0.0001, 0.0005, 0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0),
    registry=registry
)

grid_query_results_count = Histogram(
    'neurograph_grid_query_results_count',
    'Number of results returned by grid queries',
    ['query_type'],
    buckets=(1, 5, 10, 25, 50, 100, 250, 500, 1000, 5000, 10000),
    registry=registry
)

# ============================================================================
# CDNA Operation Metrics
# ============================================================================

cdna_operations_total = Counter(
    'neurograph_cdna_operations_total',
    'Total CDNA operations by type',
    ['operation'],  # get_config, update_scales, get_profile, etc.
    registry=registry
)

cdna_operation_duration_seconds = Histogram(
    'neurograph_cdna_operation_duration_seconds',
    'CDNA operation duration in seconds',
    ['operation'],
    buckets=(0.00001, 0.00005, 0.0001, 0.0005, 0.001, 0.005, 0.01, 0.05, 0.1),
    registry=registry
)

# ============================================================================
# FFI Operation Metrics
# ============================================================================

ffi_calls_total = Counter(
    'neurograph_ffi_calls_total',
    'Total FFI calls to Rust RuntimeStorage',
    ['method'],
    registry=registry
)

ffi_call_duration_seconds = Histogram(
    'neurograph_ffi_call_duration_seconds',
    'FFI call duration in seconds',
    ['method'],
    buckets=(0.000001, 0.000005, 0.00001, 0.00005, 0.0001, 0.0005, 0.001, 0.005, 0.01),
    registry=registry
)

ffi_errors_total = Counter(
    'neurograph_ffi_errors_total',
    'Total FFI errors',
    ['method', 'error_type'],
    registry=registry
)

# ============================================================================
# System Metrics
# ============================================================================

runtime_memory_bytes = Gauge(
    'neurograph_runtime_memory_bytes',
    'RuntimeStorage memory usage in bytes',
    registry=registry
)

connections_active_count = Gauge(
    'neurograph_connections_active_count',
    'Current number of active connections',
    registry=registry
)

# ============================================================================
# Application Info
# ============================================================================

app_info = Info(
    'neurograph_app',
    'NeuroGraph application information',
    registry=registry
)

app_info.info({
    'version': '0.52.0',
    'rust_core': '0.50.0',
    'python_library': '0.50.0',
    'rest_api': '0.52.0',
})

# ============================================================================
# Helper Functions
# ============================================================================

def track_http_request(method: str, path: str, status_code: int, duration: float):
    """
    Track HTTP request metrics.

    Args:
        method: HTTP method (GET, POST, etc.)
        path: Request path
        status_code: Response status code
        duration: Request duration in seconds
    """
    http_requests_total.labels(method=method, path=path, status_code=status_code).inc()
    http_request_duration_seconds.labels(method=method, path=path).observe(duration)


def track_token_operation(operation: str, duration: Optional[float] = None):
    """
    Track token operation metrics.

    Args:
        operation: Operation type (create, get, update, delete, list)
        duration: Operation duration in seconds (optional)
    """
    token_operations_total.labels(operation=operation).inc()

    if duration is not None:
        token_operation_duration_seconds.labels(operation=operation).observe(duration)

    # Update specific counters
    if operation == 'create':
        tokens_created_total.inc()
    elif operation == 'delete':
        tokens_deleted_total.inc()


def track_grid_query(query_type: str, duration: float, result_count: int):
    """
    Track grid query metrics.

    Args:
        query_type: Query type (range, nearest, field_influence)
        duration: Query duration in seconds
        result_count: Number of results returned
    """
    grid_queries_total.labels(query_type=query_type).inc()
    grid_query_duration_seconds.labels(query_type=query_type).observe(duration)
    grid_query_results_count.labels(query_type=query_type).observe(result_count)


def track_cdna_operation(operation: str, duration: float):
    """
    Track CDNA operation metrics.

    Args:
        operation: Operation type
        duration: Operation duration in seconds
    """
    cdna_operations_total.labels(operation=operation).inc()
    cdna_operation_duration_seconds.labels(operation=operation).observe(duration)


def track_ffi_call(method: str, duration: float, error: Optional[str] = None):
    """
    Track FFI call metrics.

    Args:
        method: FFI method name
        duration: Call duration in seconds
        error: Error type if call failed (optional)
    """
    ffi_calls_total.labels(method=method).inc()
    ffi_call_duration_seconds.labels(method=method).observe(duration)

    if error:
        ffi_errors_total.labels(method=method, error_type=error).inc()


def update_system_metrics(token_count: int, connection_count: int, memory_bytes: int):
    """
    Update system gauge metrics.

    Args:
        token_count: Current number of tokens
        connection_count: Current number of connections
        memory_bytes: Memory usage in bytes
    """
    tokens_active_count.set(token_count)
    connections_active_count.set(connection_count)
    runtime_memory_bytes.set(memory_bytes)


def get_metrics_response() -> Response:
    """
    Generate Prometheus metrics response.

    Returns:
        FastAPI Response with Prometheus metrics in text format
    """
    metrics_output = generate_latest(registry)
    return Response(
        content=metrics_output,
        media_type=CONTENT_TYPE_LATEST
    )


# ============================================================================
# Context Managers for Timing
# ============================================================================

class TimedOperation:
    """
    Context manager for timing operations and tracking metrics.

    Usage:
        with TimedOperation('token', 'create'):
            # ... perform operation
            pass
    """

    def __init__(self, category: str, operation: str):
        self.category = category
        self.operation = operation
        self.start_time = None

    def __enter__(self):
        self.start_time = time.perf_counter()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        duration = time.perf_counter() - self.start_time

        if self.category == 'token':
            track_token_operation(self.operation, duration)
        elif self.category == 'grid':
            # For grid, we need result count - should be passed differently
            pass
        elif self.category == 'cdna':
            track_cdna_operation(self.operation, duration)
        elif self.category == 'ffi':
            error = type(exc_val).__name__ if exc_val else None
            track_ffi_call(self.operation, duration, error)


# Example usage
if __name__ == "__main__":
    # Simulate some metrics
    track_http_request("GET", "/api/v1/tokens", 200, 0.015)
    track_http_request("POST", "/api/v1/tokens", 201, 0.023)
    track_token_operation("create", 0.00005)
    track_token_operation("get", 0.000001)
    track_grid_query("range", 0.0001, 15)
    track_cdna_operation("get_config", 0.0000005)
    track_ffi_call("get_token", 0.0000008)
    update_system_metrics(token_count=1250, connection_count=45, memory_bytes=22_500_000)

    # Generate metrics output
    print(generate_latest(registry).decode('utf-8'))
