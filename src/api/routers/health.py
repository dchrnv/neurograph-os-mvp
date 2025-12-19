"""
Health Check Endpoints (v0.51.0)

Endpoints for health and readiness checks with RuntimeStorage metrics.
"""

from fastapi import APIRouter, Depends
from ..models.response import ApiResponse
from ..models.status import HealthResponse, ReadinessResponse
from ..dependencies import get_runtime, get_token_storage, get_grid_storage
import time
import logging

router = APIRouter()
logger = logging.getLogger(__name__)

# Application start time
_start_time = time.time()


@router.get("/health", response_model=ApiResponse)
async def health_check(
    runtime=Depends(get_runtime),
    token_storage=Depends(get_token_storage)
):
    """
    Health check endpoint (v0.51.0).

    Returns basic health status and uptime with RuntimeStorage metrics.
    """
    uptime = time.time() - _start_time

    # Get token count from storage
    try:
        token_count = token_storage.count()
        status = "healthy"
    except Exception as e:
        logger.error(f"Health check failed: {e}")
        token_count = 0
        status = "degraded"

    data = HealthResponse(
        status=status,
        uptime_seconds=uptime,
        version="1.0.0 (v0.51.0)"
    )
    response_data = data.dict()
    response_data["runtime_metrics"] = {
        "tokens_count": token_count,
        "storage_backend": "runtime" if runtime is not None else "memory"
    }

    return ApiResponse.success_response(response_data)


@router.get("/health/ready", response_model=ApiResponse)
async def readiness_check(
    runtime=Depends(get_runtime),
    token_storage=Depends(get_token_storage),
    grid_storage=Depends(get_grid_storage)
):
    """
    Readiness check endpoint (v0.51.0).

    Returns whether the system is ready to handle requests.
    Checks RuntimeStorage availability and basic operations.
    """
    checks = {}

    # Check runtime
    try:
        runtime_ready = runtime is not None
        checks["runtime"] = "ok" if runtime_ready else "not_initialized"
    except Exception as e:
        logger.error(f"Runtime check failed: {e}")
        checks["runtime"] = "error"

    # Check token storage
    try:
        token_count = token_storage.count()
        checks["token_storage"] = "ok"
    except Exception as e:
        logger.error(f"Token storage check failed: {e}")
        checks["token_storage"] = "error"

    # Check grid storage
    try:
        grid_info = grid_storage.get_grid(0)
        checks["grid_storage"] = "ok" if grid_info is not None else "not_ready"
    except Exception as e:
        logger.error(f"Grid storage check failed: {e}")
        checks["grid_storage"] = "error"

    ready = all(status == "ok" for status in checks.values())

    data = ReadinessResponse(ready=ready, checks=checks)
    return ApiResponse.success_response(data.dict())
