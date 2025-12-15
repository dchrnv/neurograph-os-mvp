"""
Health Check Endpoints

Endpoints for health and readiness checks.
"""

from fastapi import APIRouter, Depends
from ..models.response import ApiResponse
from ..models.status import HealthResponse, ReadinessResponse
from ..dependencies import get_runtime
import time

router = APIRouter()

# Application start time
_start_time = time.time()


@router.get("/health", response_model=ApiResponse)
async def health_check():
    """
    Health check endpoint.

    Returns basic health status and uptime.
    """
    uptime = time.time() - _start_time
    data = HealthResponse(
        status="healthy",
        uptime_seconds=uptime,
        version="1.0.0"
    )
    return ApiResponse.success_response(data.dict())


@router.get("/health/ready", response_model=ApiResponse)
async def readiness_check(runtime=Depends(get_runtime)):
    """
    Readiness check endpoint.

    Returns whether the system is ready to handle requests.
    """
    # Check if runtime is initialized
    runtime_ready = runtime is not None

    checks = {
        "runtime": "ok" if runtime_ready else "not_initialized",
        "bootstrap": "not_loaded",  # TODO: Check bootstrap status
        "gateway": "not_available"  # TODO: Check gateway status
    }

    ready = all(status == "ok" for status in checks.values())

    data = ReadinessResponse(ready=ready, checks=checks)
    return ApiResponse.success_response(data.dict())
