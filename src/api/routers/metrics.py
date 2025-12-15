"""
Metrics Endpoints

Endpoints for Prometheus-compatible metrics.
"""

from fastapi import APIRouter, Depends, Response
from ..models.response import ApiResponse
from ..dependencies import get_runtime
import time

router = APIRouter()
_start_time = time.time()


@router.get("/metrics", response_class=Response)
async def get_prometheus_metrics(runtime=Depends(get_runtime)):
    """
    Get Prometheus-compatible metrics.

    Returns metrics in Prometheus text format.
    """
    uptime = time.time() - _start_time

    # TODO: Get actual metrics from runtime
    metrics_text = f"""# HELP neurograph_uptime_seconds System uptime
# TYPE neurograph_uptime_seconds gauge
neurograph_uptime_seconds {uptime:.2f}

# HELP neurograph_queries_total Total number of queries
# TYPE neurograph_queries_total counter
neurograph_queries_total 0

# HELP neurograph_tokens_total Total tokens in system
# TYPE neurograph_tokens_total gauge
neurograph_tokens_total 0

# HELP neurograph_memory_usage_bytes Memory usage
# TYPE neurograph_memory_usage_bytes gauge
neurograph_memory_usage_bytes 0
"""

    return Response(content=metrics_text, media_type="text/plain")


@router.get("/metrics/json", response_model=ApiResponse)
async def get_metrics_json(runtime=Depends(get_runtime)):
    """
    Get metrics in JSON format.

    Returns the same metrics as /metrics but in JSON format.
    """
    uptime = time.time() - _start_time

    # TODO: Get actual metrics from runtime
    metrics = {
        "uptime_seconds": uptime,
        "queries_total": 0,
        "tokens_total": 0,
        "memory_usage_bytes": 0
    }

    return ApiResponse.success_response(metrics)
