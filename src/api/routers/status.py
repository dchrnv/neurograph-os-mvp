"""
Status Endpoints

Endpoints for system status and statistics.
"""

from fastapi import APIRouter, Depends
from ..models.response import ApiResponse
from ..models.status import StatusResponse
from ..dependencies import get_runtime
import time

router = APIRouter()
_start_time = time.time()


@router.get("/status", response_model=ApiResponse)
async def get_status(runtime=Depends(get_runtime)):
    """
    Get full system status.

    Returns detailed information about system state, metrics, and components.
    """
    uptime = time.time() - _start_time

    # TODO: Get actual metrics from runtime
    data = StatusResponse(
        state="running" if runtime is not None else "initializing",
        uptime_seconds=uptime,
        tokens={"total": 0, "active": 0},
        connections={"total": 0, "active": 0},
        memory_usage_mb=0.0,
        cpu_usage_percent=0.0,
        components={
            "runtime": "running" if runtime is not None else "initializing",
            "gateway": "not_available",
            "intuition_engine": "not_available",
            "guardian": "not_available"
        }
    )

    return ApiResponse.success_response(data.dict())


@router.get("/stats", response_model=ApiResponse)
async def get_stats(runtime=Depends(get_runtime)):
    """
    Get system statistics.

    Returns statistics about queries, feedbacks, cache, and intuition engine.
    """
    # TODO: Get actual stats from runtime
    data = {
        "queries": {
            "total": 0,
            "per_second": 0.0,
            "avg_latency_ms": 0.0
        },
        "feedbacks": {
            "total": 0,
            "positive": 0,
            "negative": 0,
            "corrections": 0
        },
        "cache": {
            "hits": 0,
            "misses": 0,
            "hit_rate": 0.0
        },
        "intuition": {
            "fast_path_hits": 0,
            "slow_path_hits": 0,
            "fast_path_rate": 0.0
        }
    }

    return ApiResponse.success_response(data)
