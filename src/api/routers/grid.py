"""
Grid Endpoints

Spatial indexing and operations across 8 coordinate spaces.
Integrates with Rust Grid V2.0 via FFI with graceful fallback.
Ported from MVP API with production enhancements.
"""

from fastapi import APIRouter, Depends, HTTPException, Query, status
from typing import Optional
import logging

from ..models.response import ApiResponse
from ..models.grid import (
    GridConfigRequest,
    GridCreateResponse,
    GridInfoResponse,
    GridStatusResponse,
    GridTokenAddResponse,
    NeighborResult,
    NeighborsResponse,
    RangeQueryResponse,
    FieldInfluenceResponse,
    DensityResponse,
)
from ..models.auth import User
from ..dependencies import get_token_storage, get_grid_storage, check_grid_available
from ..config import settings
from ..auth.dependencies import get_current_active_user
from ..auth.permissions import Permission

logger = logging.getLogger(__name__)
router = APIRouter()


# =============================================================================
# Endpoints
# =============================================================================

@router.get("/grid/status", response_model=ApiResponse)
async def grid_status(
    grid_storage=Depends(get_grid_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Check Grid availability and status.

    **Requires:** `grid:read` permission

    Returns information about whether Rust Grid bindings are available
    and the number of active grid instances.
    """
    # Check permission
    if Permission.READ_GRID.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_GRID.value} required"
        )

    if not settings.GRID_ENABLED:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid functionality is disabled"
        )

    try:
        available = check_grid_available()
        grids_count = len(grid_storage.list_grids()) if available else 0

        message = (
            "Grid V2.0 (Rust) is ready" if available
            else "Grid not available. Install Rust bindings: "
                 "cd src/core_rust && maturin develop --release --features python"
        )

        response_data = GridStatusResponse(
            available=available,
            grids_count=grids_count,
            message=message
        )

        logger.debug(f"Grid status: available={available}, count={grids_count}")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except Exception as e:
        logger.error(f"Grid status check failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to check grid status: {str(e)}"
        )


@router.post("/grid/create", response_model=ApiResponse, status_code=status.HTTP_201_CREATED)
async def create_grid(
    config: Optional[GridConfigRequest] = None,
    grid_storage=Depends(get_grid_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Create a new Grid instance.

    **Requires:** `grid:write` permission

    Grid provides spatial indexing for efficient neighbor searches
    across 8 coordinate spaces.

    Optional configuration:
    - bucket_size: Spatial bucket size (default: 10.0)
    - density_threshold: Threshold for field detection (default: 0.5)
    - min_field_nodes: Minimum nodes to form a field (default: 3)
    """
    # Check permission
    if Permission.WRITE_GRID.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.WRITE_GRID.value} required"
        )

    if not settings.GRID_ENABLED or not settings.ENABLE_NEW_GRID_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid API not enabled"
        )

    if not check_grid_available():
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid not available. Install Rust bindings."
        )

    try:
        # Check max instances
        if len(grid_storage.list_grids()) >= settings.GRID_MAX_INSTANCES:
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail=f"Maximum grid instances ({settings.GRID_MAX_INSTANCES}) reached"
            )

        # Create grid with optional config
        config_dict = None
        if config:
            config_dict = {
                'bucket_size': config.bucket_size,
                'density_threshold': config.density_threshold,
                'min_field_nodes': config.min_field_nodes,
            }
        else:
            config_dict = {
                'bucket_size': settings.GRID_DEFAULT_BUCKET_SIZE,
                'density_threshold': settings.GRID_DEFAULT_DENSITY_THRESHOLD,
                'min_field_nodes': settings.GRID_DEFAULT_MIN_FIELD_NODES,
            }

        grid_id = grid_storage.create_grid(config_dict)

        logger.info(f"Grid created: ID={grid_id}")

        response_data = GridCreateResponse(
            grid_id=grid_id,
            config=config_dict,
            status="created"
        )

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Grid creation failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to create grid: {str(e)}"
        )


@router.get("/grid/{grid_id}", response_model=ApiResponse)
async def get_grid_info(
    grid_id: int,
    grid_storage=Depends(get_grid_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Get information about a Grid instance.

    **Requires:** `grid:read` permission

    Returns grid ID, token count, and configuration.
    """
    # Check permission
    if Permission.READ_GRID.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_GRID.value} required"
        )

    if not settings.GRID_ENABLED or not settings.ENABLE_NEW_GRID_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid API not enabled"
        )

    if not check_grid_available():
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid not available"
        )

    try:
        grid = grid_storage.get_grid(grid_id)
        if not grid:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Grid {grid_id} not found"
            )

        # Get token count
        token_count = len(grid)

        response_data = GridInfoResponse(
            grid_id=grid_id,
            token_count=token_count,
            config={
                'bucket_size': settings.GRID_DEFAULT_BUCKET_SIZE,
                'density_threshold': settings.GRID_DEFAULT_DENSITY_THRESHOLD,
                'min_field_nodes': settings.GRID_DEFAULT_MIN_FIELD_NODES,
            }
        )

        logger.debug(f"Grid info: ID={grid_id}, tokens={token_count}")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Grid info retrieval failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to get grid info: {str(e)}"
        )


@router.post("/grid/{grid_id}/tokens/{token_id}", response_model=ApiResponse)
async def add_token_to_grid(
    grid_id: int,
    token_id: int,
    token_storage=Depends(get_token_storage),
    grid_storage=Depends(get_grid_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Add token to grid.

    **Requires:** `grid:write` permission

    Indexes the token in the grid for spatial queries.
    The token must exist in token storage.
    """
    # Check permission
    if Permission.WRITE_GRID.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.WRITE_GRID.value} required"
        )

    if not settings.GRID_ENABLED or not settings.ENABLE_NEW_GRID_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid API not enabled"
        )

    if not check_grid_available():
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid not available"
        )

    try:
        # Check if grid exists
        grid = grid_storage.get_grid(grid_id)
        if not grid:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Grid {grid_id} not found"
            )

        # Check if token exists
        token = token_storage.get(token_id)
        if not token:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Token {token_id:08X} not found"
            )

        # Add token to grid
        success = grid_storage.add_token(grid_id, token)

        if not success:
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail="Failed to add token to grid"
            )

        grid_size = len(grid)

        logger.info(f"Token {token_id:08X} added to grid {grid_id}")

        response_data = GridTokenAddResponse(
            grid_id=grid_id,
            token_id=token_id,
            status="added",
            grid_size=grid_size
        )

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Add token to grid failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to add token: {str(e)}"
        )


@router.delete("/grid/{grid_id}/tokens/{token_id}", status_code=status.HTTP_204_NO_CONTENT)
async def remove_token_from_grid(
    grid_id: int,
    token_id: int,
    grid_storage=Depends(get_grid_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Remove token from grid.

    **Requires:** `grid:write` permission

    Removes the token from spatial index.
    """
    # Check permission
    if Permission.WRITE_GRID.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.WRITE_GRID.value} required"
        )

    if not settings.GRID_ENABLED or not settings.ENABLE_NEW_GRID_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid API not enabled"
        )

    if not check_grid_available():
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid not available"
        )

    try:
        # Check if grid exists
        grid = grid_storage.get_grid(grid_id)
        if not grid:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Grid {grid_id} not found"
            )

        # Remove token
        removed = grid_storage.remove_token(grid_id, token_id)

        if not removed:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Token {token_id:08X} not found in grid"
            )

        logger.info(f"Token {token_id:08X} removed from grid {grid_id}")

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Remove token from grid failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to remove token: {str(e)}"
        )


@router.get("/grid/{grid_id}/neighbors/{token_id}", response_model=ApiResponse)
async def find_neighbors(
    grid_id: int,
    token_id: int,
    space: int = Query(0, ge=0, le=7, description="Coordinate space (0=L1, ..., 7=L8)"),
    radius: float = Query(10.0, ge=0.1, description="Search radius"),
    max_results: int = Query(10, ge=1, le=1000, description="Maximum results"),
    grid_storage=Depends(get_grid_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Find neighbors of a token.

    **Requires:** `grid:read` permission

    Searches for tokens within radius in specified coordinate space.

    Coordinate spaces:
    - 0: L1 Physical
    - 1: L2 Sensory
    - 2: L3 Motor
    - 3: L4 Emotional
    - 4: L5 Cognitive
    - 5: L6 Social
    - 6: L7 Temporal
    - 7: L8 Abstract
    """
    # Check permission
    if Permission.READ_GRID.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_GRID.value} required"
        )

    if not settings.GRID_ENABLED or not settings.ENABLE_NEW_GRID_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid API not enabled"
        )

    if not check_grid_available():
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid not available"
        )

    try:
        # Check if grid exists
        grid = grid_storage.get_grid(grid_id)
        if not grid:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Grid {grid_id} not found"
            )

        # Check if token exists in grid
        if grid.get(token_id) is None:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Token {token_id:08X} not found in grid"
            )

        # Find neighbors
        neighbors = grid_storage.find_neighbors(grid_id, token_id, space, radius, max_results)

        neighbor_results = [
            NeighborResult(token_id=tid, distance=dist)
            for tid, dist in neighbors
        ]

        response_data = NeighborsResponse(
            grid_id=grid_id,
            center_token_id=token_id,
            space=space,
            radius=radius,
            neighbors=neighbor_results,
            count=len(neighbor_results)
        )

        logger.debug(f"Found {len(neighbor_results)} neighbors for token {token_id:08X}")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Neighbor search failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to find neighbors: {str(e)}"
        )


@router.get("/grid/{grid_id}/range", response_model=ApiResponse)
async def range_query(
    grid_id: int,
    space: int = Query(0, ge=0, le=7, description="Coordinate space"),
    x: float = Query(0.0, description="Center X coordinate"),
    y: float = Query(0.0, description="Center Y coordinate"),
    z: float = Query(0.0, description="Center Z coordinate"),
    radius: float = Query(10.0, ge=0.1, description="Search radius"),
    grid_storage=Depends(get_grid_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Find all tokens within radius of a point.

    **Requires:** `grid:read` permission

    Range query in specified coordinate space.
    """
    # Check permission
    if Permission.READ_GRID.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_GRID.value} required"
        )

    if not settings.GRID_ENABLED or not settings.ENABLE_NEW_GRID_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid API not enabled"
        )

    if not check_grid_available():
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid not available"
        )

    try:
        # Check if grid exists
        grid = grid_storage.get_grid(grid_id)
        if not grid:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Grid {grid_id} not found"
            )

        # Perform range query
        results = grid_storage.range_query(grid_id, space, x, y, z, radius)

        result_items = [
            NeighborResult(token_id=tid, distance=dist)
            for tid, dist in results
        ]

        response_data = RangeQueryResponse(
            grid_id=grid_id,
            space=space,
            center=(x, y, z),
            radius=radius,
            results=result_items,
            count=len(result_items)
        )

        logger.debug(f"Range query found {len(result_items)} tokens")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Range query failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to perform range query: {str(e)}"
        )


@router.get("/grid/{grid_id}/influence", response_model=ApiResponse)
async def calculate_field_influence(
    grid_id: int,
    space: int = Query(0, ge=0, le=7, description="Coordinate space"),
    x: float = Query(0.0, description="Position X"),
    y: float = Query(0.0, description="Position Y"),
    z: float = Query(0.0, description="Position Z"),
    radius: float = Query(10.0, ge=0.1, description="Search radius"),
    grid_storage=Depends(get_grid_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Calculate field influence at a point.

    **Requires:** `grid:read` permission

    Computes the combined field influence of all tokens
    within radius of the specified position.
    """
    # Check permission
    if Permission.READ_GRID.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_GRID.value} required"
        )

    if not settings.GRID_ENABLED or not settings.ENABLE_NEW_GRID_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid API not enabled"
        )

    if not check_grid_available():
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid not available"
        )

    try:
        # Check if grid exists
        grid = grid_storage.get_grid(grid_id)
        if not grid:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Grid {grid_id} not found"
            )

        # Calculate influence
        influence = grid_storage.calculate_field_influence(grid_id, space, x, y, z, radius)

        response_data = FieldInfluenceResponse(
            grid_id=grid_id,
            space=space,
            position=(x, y, z),
            radius=radius,
            influence=influence
        )

        logger.debug(f"Field influence calculated: {influence:.3f}")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Field influence calculation failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to calculate influence: {str(e)}"
        )


@router.get("/grid/{grid_id}/density", response_model=ApiResponse)
async def calculate_density(
    grid_id: int,
    space: int = Query(0, ge=0, le=7, description="Coordinate space"),
    x: float = Query(0.0, description="Position X"),
    y: float = Query(0.0, description="Position Y"),
    z: float = Query(0.0, description="Position Z"),
    radius: float = Query(10.0, ge=0.1, description="Search radius"),
    grid_storage=Depends(get_grid_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Calculate token density in a region.

    **Requires:** `grid:read` permission

    Computes the density of tokens within the specified radius.
    """
    # Check permission
    if Permission.READ_GRID.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_GRID.value} required"
        )

    if not settings.GRID_ENABLED or not settings.ENABLE_NEW_GRID_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid API not enabled"
        )

    if not check_grid_available():
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid not available"
        )

    try:
        # Check if grid exists
        grid = grid_storage.get_grid(grid_id)
        if not grid:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Grid {grid_id} not found"
            )

        # Calculate density
        density = grid_storage.calculate_density(grid_id, space, x, y, z, radius)

        # Count tokens in range
        results = grid_storage.range_query(grid_id, space, x, y, z, radius)
        tokens_in_range = len(results)

        response_data = DensityResponse(
            grid_id=grid_id,
            space=space,
            position=(x, y, z),
            radius=radius,
            density=density,
            tokens_in_range=tokens_in_range
        )

        logger.debug(f"Density calculated: {density:.3f} ({tokens_in_range} tokens)")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Density calculation failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to calculate density: {str(e)}"
        )


@router.delete("/grid/{grid_id}", status_code=status.HTTP_204_NO_CONTENT)
async def delete_grid(
    grid_id: int,
    grid_storage=Depends(get_grid_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Delete a Grid instance.

    **Requires:** `grid:write` permission

    Permanently removes the grid and all its spatial indexes.
    """
    # Check permission
    if Permission.WRITE_GRID.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.WRITE_GRID.value} required"
        )

    if not settings.GRID_ENABLED or not settings.ENABLE_NEW_GRID_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid API not enabled"
        )

    if not check_grid_available():
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Grid not available"
        )

    try:
        deleted = grid_storage.delete_grid(grid_id)

        if not deleted:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Grid {grid_id} not found"
            )

        logger.info(f"Grid {grid_id} deleted")

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Grid deletion failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to delete grid: {str(e)}"
        )
