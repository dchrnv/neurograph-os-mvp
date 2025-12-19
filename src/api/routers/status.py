"""
Status Endpoints (v0.51.0)

Endpoints for system status and statistics with RuntimeStorage metrics.
"""

from fastapi import APIRouter, Depends
from ..models.response import ApiResponse
from ..models.status import StatusResponse
from ..dependencies import get_runtime, get_token_storage, get_cdna_storage
import time
import logging
import psutil
import os

router = APIRouter()
logger = logging.getLogger(__name__)
_start_time = time.time()


@router.get("/status", response_model=ApiResponse)
async def get_status(
    runtime=Depends(get_runtime),
    token_storage=Depends(get_token_storage),
    cdna_storage=Depends(get_cdna_storage)
):
    """
    Get full system status (v0.51.0).

    Returns detailed information about system state, RuntimeStorage metrics, and components.
    """
    uptime = time.time() - _start_time

    # Get metrics from RuntimeStorage
    try:
        token_count = token_storage.count()
        cdna_config = cdna_storage.get_config()

        # Get process memory and CPU
        process = psutil.Process(os.getpid())
        memory_mb = process.memory_info().rss / 1024 / 1024
        cpu_percent = process.cpu_percent(interval=0.1)

        data = StatusResponse(
            state="running",
            uptime_seconds=uptime,
            tokens={"total": token_count, "active": token_count},
            connections={"total": 0, "active": 0},  # TODO: Add connection tracking
            memory_usage_mb=memory_mb,
            cpu_usage_percent=cpu_percent,
            components={
                "runtime": "running",
                "runtime_storage": "running",
                "token_storage": "running",
                "grid_storage": "running",
                "cdna_storage": "running"
            }
        )

        response_data = data.dict()
        response_data["cdna_profile"] = cdna_config.get("profile_id", "unknown")
        response_data["storage_backend"] = "runtime"

    except Exception as e:
        logger.error(f"Status check failed: {e}")
        data = StatusResponse(
            state="degraded",
            uptime_seconds=uptime,
            tokens={"total": 0, "active": 0},
            connections={"total": 0, "active": 0},
            memory_usage_mb=0.0,
            cpu_usage_percent=0.0,
            components={
                "runtime": "error",
                "runtime_storage": "error"
            }
        )
        response_data = data.dict()
        response_data["error"] = str(e)

    return ApiResponse.success_response(response_data)


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
