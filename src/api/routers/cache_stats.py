"""
Cache statistics endpoint.

Provides cache performance metrics and management.
"""

from fastapi import APIRouter, Depends, status

from ..models.response import SuccessResponse
from ..auth.dependencies import get_current_active_user
from ..cache import get_all_cache_stats, cleanup_all_caches
from ..models.auth import User


router = APIRouter(
    prefix="/cache",
    tags=["cache"],
)


@router.get(
    "/stats",
    response_model=SuccessResponse,
    status_code=status.HTTP_200_OK,
    summary="Get cache statistics",
    description="Get performance statistics for all caches (requires authentication)",
)
async def get_cache_stats_endpoint(
    user: User = Depends(get_current_active_user)
) -> SuccessResponse:
    """
    Get cache statistics.

    Returns statistics for all cache instances:
    - Size and max_size
    - Hit/miss counts
    - Hit rate percentage
    - Eviction count

    **Requires:** admin:config permission
    """
    stats = get_all_cache_stats()

    return SuccessResponse(
        message="Cache statistics retrieved",
        data=stats
    )


@router.post(
    "/cleanup",
    response_model=SuccessResponse,
    status_code=status.HTTP_200_OK,
    summary="Cleanup expired cache entries",
    description="Remove expired entries from all caches (requires authentication)",
)
async def cleanup_caches_endpoint(
    user: User = Depends(get_current_active_user)
) -> SuccessResponse:
    """
    Cleanup expired cache entries.

    Removes all expired entries from all cache instances
    to free up memory.

    **Requires:** admin:config permission
    """
    cleanup_all_caches()

    return SuccessResponse(
        message="Cache cleanup completed",
        data=get_all_cache_stats()
    )
