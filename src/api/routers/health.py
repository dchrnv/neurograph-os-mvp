"""
Health Check Endpoints (v0.52.0)

Comprehensive health checks for monitoring and Kubernetes probes:
- /health - Basic health check
- /health/live - Liveness probe (is process running?)
- /health/ready - Readiness probe (can handle traffic?)
- /health/startup - Startup probe (has app started?)

Kubernetes probe support:
- livenessProbe: /health/live
- readinessProbe: /health/ready
- startupProbe: /health/startup

Version: v0.52.0
"""

from fastapi import APIRouter, Depends, Response, status
from ..models.response import ApiResponse
from ..models.status import HealthResponse, ReadinessResponse
from ..dependencies import get_runtime, get_token_storage, get_grid_storage
from ..logging_config import get_logger
import time

router = APIRouter()
logger = get_logger(__name__, component="health")

# Application lifecycle state
_start_time = time.time()
_startup_complete = False
_min_startup_time = 2.0  # Minimum time before ready (allow components to initialize)


@router.get("/health", response_model=ApiResponse)
async def health_check(
    runtime=Depends(get_runtime),
    token_storage=Depends(get_token_storage)
):
    """
    Basic health check endpoint (v0.52.0).

    Returns overall health status and uptime with RuntimeStorage metrics.
    Use /health/live, /health/ready, or /health/startup for specific probes.

    Response:
        - status: "healthy" | "degraded" | "unhealthy"
        - uptime_seconds: time since startup
        - version: API version
        - runtime_metrics: token count and storage backend
    """
    uptime = time.time() - _start_time

    # Get metrics from RuntimeStorage
    try:
        token_count = token_storage.count()
        health_status = "healthy"
    except Exception as e:
        logger.error(
            "Health check failed",
            extra={"error": str(e)},
            exc_info=True
        )
        token_count = 0
        health_status = "degraded"

    data = HealthResponse(
        status=health_status,
        uptime_seconds=uptime,
        version="0.52.0"
    )
    response_data = data.dict()
    response_data["runtime_metrics"] = {
        "tokens_count": token_count,
        "storage_backend": "runtime" if runtime is not None else "memory"
    }

    return ApiResponse.success_response(response_data)


@router.get("/health/live")
async def liveness_check():
    """
    Liveness probe for Kubernetes (v0.52.0).

    Answers: "Is the application process alive?"

    Returns 200 if the process is running.
    Returns 503 only if the application is critically broken.

    This is a lightweight check - if this fails, Kubernetes will restart the pod.
    Use for: livenessProbe in Kubernetes deployment

    Response Codes:
        200: Process is alive and running
        503: Process is critically broken (rare)
    """
    # This is intentionally minimal - just check that Python is executing
    return {"status": "alive", "check": "liveness"}


@router.get("/health/ready", response_model=ApiResponse)
async def readiness_check(
    runtime=Depends(get_runtime),
    token_storage=Depends(get_token_storage),
    grid_storage=Depends(get_grid_storage)
):
    """
    Readiness probe for Kubernetes (v0.52.0).

    Answers: "Can the application handle traffic?"

    Checks:
    - Runtime initialized
    - Token storage operational
    - Grid storage accessible
    - Minimum uptime elapsed (avoid thundering herd)

    Returns 200 if ready to serve traffic, 503 if not ready.
    Use for: readinessProbe in Kubernetes deployment

    Response Codes:
        200: Ready to handle traffic
        503: Not ready (temporarily unavailable)
    """
    global _startup_complete

    checks = {}
    uptime = time.time() - _start_time

    # Check minimum uptime (avoid marking ready too quickly)
    if uptime < _min_startup_time and not _startup_complete:
        logger.debug("Readiness check: still starting up")
        return Response(
            content='{"ready": false, "reason": "startup_in_progress"}',
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            media_type="application/json"
        )

    # Check runtime
    try:
        runtime_ready = runtime is not None
        checks["runtime"] = "ok" if runtime_ready else "not_initialized"
    except Exception as e:
        logger.warning(f"Runtime check failed: {e}")
        checks["runtime"] = "error"

    # Check token storage
    try:
        _ = token_storage.count()  # Quick operation test
        checks["token_storage"] = "ok"
    except Exception as e:
        logger.warning(f"Token storage check failed: {e}")
        checks["token_storage"] = "error"

    # Check grid storage
    try:
        grid_info = grid_storage.get_grid(0)
        checks["grid_storage"] = "ok" if grid_info is not None else "not_ready"
    except Exception as e:
        logger.warning(f"Grid storage check failed: {e}")
        checks["grid_storage"] = "error"

    ready = all(check_status == "ok" for check_status in checks.values())

    if ready:
        _startup_complete = True  # Mark startup as complete
        data = ReadinessResponse(ready=True, checks=checks)
        return ApiResponse.success_response(data.dict())
    else:
        logger.warning(f"Readiness check failed: {checks}")
        return Response(
            content=f'{{"ready": false, "checks": {checks}}}',
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            media_type="application/json"
        )


@router.get("/health/startup")
async def startup_check(
    runtime=Depends(get_runtime),
    token_storage=Depends(get_token_storage)
):
    """
    Startup probe for Kubernetes (v0.52.0).

    Answers: "Has the application completed its startup sequence?"

    This probe indicates when the application has finished initializing
    and is ready for liveness/readiness probes to begin.

    Returns 200 once startup is complete, 503 while starting.
    Use for: startupProbe in Kubernetes deployment (optional)

    Response Codes:
        200: Startup complete
        503: Still starting up

    Startup Criteria:
    - Runtime initialized
    - Token storage accessible
    - Minimum uptime elapsed (allow Rust FFI to initialize)
    """
    global _startup_complete

    uptime = time.time() - _start_time

    # Quick startup checks
    try:
        # Check runtime and basic storage
        runtime_ok = runtime is not None
        storage_ok = token_storage.count() >= 0  # Just check it doesn't error

        # Check minimum uptime
        uptime_ok = uptime >= _min_startup_time

        startup_complete = runtime_ok and storage_ok and uptime_ok

        if startup_complete:
            _startup_complete = True
            return {
                "started": True,
                "uptime_seconds": round(uptime, 2),
                "checks": {
                    "runtime": "ok",
                    "storage": "ok",
                    "uptime": "ok"
                }
            }
        else:
            return Response(
                content='{"started": false, "reason": "initialization_in_progress"}',
                status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
                media_type="application/json"
            )

    except Exception as e:
        logger.error(f"Startup check failed: {e}")
        return Response(
            content=f'{{"started": false, "error": "{str(e)}"}}',
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            media_type="application/json"
        )
