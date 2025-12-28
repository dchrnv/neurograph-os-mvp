"""
Metrics Endpoints

Prometheus-compatible metrics for monitoring NeuroGraph API.

Version: v0.52.0 - Enhanced with prometheus_client
"""

from fastapi import APIRouter, Depends, Response, HTTPException, status
from ..models.response import ApiResponse
from ..models.auth import User
from ..dependencies import get_runtime
from ..auth.dependencies import get_current_active_user
from ..auth.permissions import Permission
from ..metrics_prometheus import (
    get_metrics_response,
    update_system_metrics
)
from ..logging_config import get_logger
import time

router = APIRouter()
_start_time = time.time()
logger = get_logger(__name__, component="metrics")


@router.get("/metrics", response_class=Response)
async def get_prometheus_metrics(
    runtime=Depends(get_runtime),
    current_user: User = Depends(get_current_active_user)
):
    """
    Get Prometheus-compatible metrics.

    **Requires:** `metrics:read` permission

    Returns comprehensive metrics in Prometheus text format including:
    - HTTP request counts and latencies
    - Token operation metrics
    - Grid query performance
    - CDNA operations
    - FFI call metrics
    - System resources

    This endpoint is designed for Prometheus scraping.
    """
    # Check permission
    if Permission.READ_METRICS.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_METRICS.value} required"
        )

    try:
        # Update system metrics from RuntimeStorage
        token_count = runtime.tokens.count()
        connection_count = runtime.connections.count()

        # Estimate memory usage (RSS would be better, but requires psutil)
        # For now, we'll use a simple heuristic based on token count
        estimated_memory = token_count * 64  # 64 bytes per token (rough estimate)

        update_system_metrics(
            token_count=token_count,
            connection_count=connection_count,
            memory_bytes=estimated_memory
        )

        # Generate and return Prometheus metrics
        return get_metrics_response()

    except Exception as e:
        logger.error(
            "Failed to generate metrics",
            extra={"error": str(e)},
            exc_info=True
        )
        # Return empty metrics on error to avoid breaking Prometheus scraping
        return Response(content="", media_type="text/plain; version=0.0.4")


@router.get("/metrics/json", response_model=ApiResponse)
async def get_metrics_json(
    runtime=Depends(get_runtime),
    current_user: User = Depends(get_current_active_user)
):
    """
    Get metrics in JSON format (human-readable alternative to /metrics).

    **Requires:** `metrics:read` permission

    Returns key system metrics in JSON format for easy consumption
    by dashboards or monitoring tools that prefer JSON over Prometheus format.

    Note: For Prometheus scraping, use /metrics endpoint instead.
    """
    # Check permission
    if Permission.READ_METRICS.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_METRICS.value} required"
        )

    try:
        uptime = time.time() - _start_time

        # Get real-time metrics from RuntimeStorage
        token_count = runtime.tokens.count()
        connection_count = runtime.connections.count()

        metrics = {
            "system": {
                "uptime_seconds": round(uptime, 2),
                "version": "0.52.0",
            },
            "tokens": {
                "active_count": token_count,
            },
            "connections": {
                "active_count": connection_count,
            },
            "storage": {
                "backend": "RuntimeStorage",
                "rust_core": "v0.50.0",
            }
        }

        logger.info(
            "Metrics JSON requested",
            extra={"token_count": token_count, "uptime": round(uptime, 2)}
        )

        return ApiResponse.success_response(metrics)

    except Exception as e:
        logger.error(
            "Failed to generate JSON metrics",
            extra={"error": str(e)},
            exc_info=True
        )
        return ApiResponse.error_response(
            code="METRICS_ERROR",
            message="Failed to generate metrics"
        )
