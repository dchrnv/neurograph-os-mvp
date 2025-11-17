
    # NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
    # Copyright (C) 2024-2025 Chernov Denys

    # This program is free software: you can redistribute it and/or modify
    # it under the terms of the GNU Affero General Public License as published by
    # the Free Software Foundation, either version 3 of the License, or
    # (at your option) any later version.

    # This program is distributed in the hope that it will be useful,
    # but WITHOUT ANY WARRANTY; without even the implied warranty of
    # MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    # GNU Affero General Public License for more details.

    # You should have received a copy of the GNU Affero General Public License
    # along with this program. If not, see <https://www.gnu.org/licenses/>.
    

"""
Grid API Routes - Integration with Rust Grid V2.0

Provides spatial indexing, neighbor queries, and field calculations.
"""

from typing import List, Optional, Tuple
from fastapi import APIRouter, HTTPException
from pydantic import BaseModel, Field

# Try to import Rust Grid, fallback if not available
try:
    from neurograph import Grid, GridConfig
    GRID_AVAILABLE = True
except ImportError:
    GRID_AVAILABLE = False
    Grid = None
    GridConfig = None

# ═══════════════════════════════════════════════════════
# GRID STORAGE
# ═══════════════════════════════════════════════════════

GRID_INSTANCES = {}  # grid_id -> Grid
NEXT_GRID_ID = 1

# ═══════════════════════════════════════════════════════
# SCHEMAS
# ═══════════════════════════════════════════════════════

class GridConfigRequest(BaseModel):
    """Grid configuration."""
    bucket_size: float = Field(default=10.0, ge=0.1, le=1000.0)
    density_threshold: float = Field(default=0.5, ge=0.0, le=1.0)
    min_field_nodes: int = Field(default=3, ge=1, le=100)


class GridCreateResponse(BaseModel):
    """Grid creation response."""
    grid_id: int
    config: dict
    status: str


class GridInfoResponse(BaseModel):
    """Grid information response."""
    grid_id: int
    token_count: int
    config: dict


class NeighborResult(BaseModel):
    """Neighbor query result."""
    token_id: int
    distance: float


class NeighborsResponse(BaseModel):
    """Neighbors query response."""
    grid_id: int
    center_token_id: int
    space: int
    radius: float
    neighbors: List[NeighborResult]
    count: int


class RangeQueryResponse(BaseModel):
    """Range query response."""
    grid_id: int
    space: int
    center: Tuple[float, float, float]
    radius: float
    results: List[NeighborResult]
    count: int


class FieldInfluenceResponse(BaseModel):
    """Field influence response."""
    grid_id: int
    space: int
    position: Tuple[float, float, float]
    radius: float
    influence: float


class DensityResponse(BaseModel):
    """Density calculation response."""
    grid_id: int
    space: int
    position: Tuple[float, float, float]
    radius: float
    density: float
    tokens_in_range: int


# ═══════════════════════════════════════════════════════
# ROUTER
# ═══════════════════════════════════════════════════════

router = APIRouter(prefix="/api/v1/grid", tags=["grid"])


def check_grid_available():
    """Check if Grid is available."""
    if not GRID_AVAILABLE:
        raise HTTPException(
            status_code=503,
            detail="Grid not available. Install Rust bindings: "
                   "cd src/core_rust && maturin develop --release --features python"
        )


@router.get("/status")
async def grid_status():
    """Check if Grid is available."""
    return {
        "available": GRID_AVAILABLE,
        "grids_count": len(GRID_INSTANCES),
        "message": "Grid V2.0 (Rust) is ready" if GRID_AVAILABLE
                   else "Install Rust bindings to use Grid"
    }


@router.post("/create", response_model=GridCreateResponse, status_code=201)
async def create_grid(config: Optional[GridConfigRequest] = None):
    """Create a new Grid instance."""
    global NEXT_GRID_ID
    check_grid_available()

    # Create grid with optional config
    if config:
        grid_config = GridConfig()
        grid_config.bucket_size = config.bucket_size
        grid_config.density_threshold = config.density_threshold
        grid_config.min_field_nodes = config.min_field_nodes
        grid = Grid(grid_config)

        config_dict = {
            "bucket_size": config.bucket_size,
            "density_threshold": config.density_threshold,
            "min_field_nodes": config.min_field_nodes,
        }
    else:
        grid = Grid()
        config_dict = {
            "bucket_size": 10.0,
            "density_threshold": 0.5,
            "min_field_nodes": 3,
        }

    grid_id = NEXT_GRID_ID
    NEXT_GRID_ID += 1

    GRID_INSTANCES[grid_id] = grid

    return GridCreateResponse(
        grid_id=grid_id,
        config=config_dict,
        status="created"
    )


@router.get("/{grid_id}", response_model=GridInfoResponse)
async def get_grid_info(grid_id: int):
    """Get information about a Grid instance."""
    check_grid_available()

    if grid_id not in GRID_INSTANCES:
        raise HTTPException(404, f"Grid {grid_id} not found")

    grid = GRID_INSTANCES[grid_id]

    return GridInfoResponse(
        grid_id=grid_id,
        token_count=len(grid),
        config={
            "bucket_size": 10.0,  # Default values (GridConfig not exposed in Python)
            "density_threshold": 0.5,
            "min_field_nodes": 3,
        }
    )


@router.post("/{grid_id}/tokens/{token_id}")
async def add_token_to_grid(grid_id: int, token_id: int):
    """
    Add a token to the grid.

    Note: The token must exist in TOKEN_STORAGE (from main API).
    """
    check_grid_available()

    if grid_id not in GRID_INSTANCES:
        raise HTTPException(404, f"Grid {grid_id} not found")

    # Import TOKEN_STORAGE from main
    from .main import TOKEN_STORAGE
    from neurograph import Token as RustToken, CoordinateSpace

    if token_id not in TOKEN_STORAGE:
        raise HTTPException(404, f"Token {token_id} not found in storage")

    grid = GRID_INSTANCES[grid_id]
    py_token = TOKEN_STORAGE[token_id]

    # Convert Python token to Rust token
    rust_token = RustToken(token_id)

    # Copy coordinates from all spaces
    space_map = [
        (0, CoordinateSpace.L1Physical()),
        (1, CoordinateSpace.L2Sensory()),
        (2, CoordinateSpace.L3Motor()),
        (3, CoordinateSpace.L4Emotional()),
        (4, CoordinateSpace.L5Cognitive()),
        (5, CoordinateSpace.L6Social()),
        (6, CoordinateSpace.L7Temporal()),
        (7, CoordinateSpace.L8Abstract()),
    ]

    for level, space_enum in space_map:
        coords = py_token.get_coordinates(level)
        if coords:
            rust_token.set_coordinates(space_enum, coords[0], coords[1], coords[2])

    # Copy properties
    rust_token.weight = py_token.weight
    rust_token.field_radius = int(py_token.field_radius * 100)  # Encode
    rust_token.field_strength = int(py_token.field_strength * 255)  # Encode
    rust_token.set_active(True)

    # Add to grid
    try:
        grid.add(rust_token)
    except Exception as e:
        raise HTTPException(400, f"Failed to add token to grid: {str(e)}")

    return {
        "grid_id": grid_id,
        "token_id": token_id,
        "status": "added",
        "grid_size": len(grid)
    }


@router.delete("/{grid_id}/tokens/{token_id}", status_code=204)
async def remove_token_from_grid(grid_id: int, token_id: int):
    """Remove a token from the grid."""
    check_grid_available()

    if grid_id not in GRID_INSTANCES:
        raise HTTPException(404, f"Grid {grid_id} not found")

    grid = GRID_INSTANCES[grid_id]

    removed = grid.remove(token_id)
    if removed is None:
        raise HTTPException(404, f"Token {token_id} not found in grid")


@router.get("/{grid_id}/neighbors/{token_id}", response_model=NeighborsResponse)
async def find_neighbors(
    grid_id: int,
    token_id: int,
    space: int = Field(0, ge=0, le=7, description="Coordinate space (0=L1Physical, ..., 7=L8Abstract)"),
    radius: float = Field(10.0, ge=0.1, description="Search radius"),
    max_results: int = Field(10, ge=1, le=1000, description="Maximum number of results")
):
    """Find neighbors of a token in a specific coordinate space."""
    check_grid_available()

    if grid_id not in GRID_INSTANCES:
        raise HTTPException(404, f"Grid {grid_id} not found")

    grid = GRID_INSTANCES[grid_id]

    # Check if token exists in grid
    if grid.get(token_id) is None:
        raise HTTPException(404, f"Token {token_id} not found in grid")

    # Find neighbors
    neighbors = grid.find_neighbors(token_id, space, radius, max_results)

    return NeighborsResponse(
        grid_id=grid_id,
        center_token_id=token_id,
        space=space,
        radius=radius,
        neighbors=[NeighborResult(token_id=tid, distance=dist) for tid, dist in neighbors],
        count=len(neighbors)
    )


@router.get("/{grid_id}/range", response_model=RangeQueryResponse)
async def range_query(
    grid_id: int,
    space: int = Field(0, ge=0, le=7, description="Coordinate space"),
    x: float = Field(0.0, description="Center X coordinate"),
    y: float = Field(0.0, description="Center Y coordinate"),
    z: float = Field(0.0, description="Center Z coordinate"),
    radius: float = Field(10.0, ge=0.1, description="Search radius")
):
    """Find all tokens within radius of a point."""
    check_grid_available()

    if grid_id not in GRID_INSTANCES:
        raise HTTPException(404, f"Grid {grid_id} not found")

    grid = GRID_INSTANCES[grid_id]

    # Range query
    results = grid.range_query(space, x, y, z, radius)

    return RangeQueryResponse(
        grid_id=grid_id,
        space=space,
        center=(x, y, z),
        radius=radius,
        results=[NeighborResult(token_id=tid, distance=dist) for tid, dist in results],
        count=len(results)
    )


@router.get("/{grid_id}/influence", response_model=FieldInfluenceResponse)
async def calculate_field_influence(
    grid_id: int,
    space: int = Field(0, ge=0, le=7, description="Coordinate space"),
    x: float = Field(0.0, description="Position X"),
    y: float = Field(0.0, description="Position Y"),
    z: float = Field(0.0, description="Position Z"),
    radius: float = Field(10.0, ge=0.1, description="Search radius")
):
    """Calculate field influence at a point."""
    check_grid_available()

    if grid_id not in GRID_INSTANCES:
        raise HTTPException(404, f"Grid {grid_id} not found")

    grid = GRID_INSTANCES[grid_id]

    # Calculate influence
    influence = grid.calculate_field_influence(space, x, y, z, radius)

    return FieldInfluenceResponse(
        grid_id=grid_id,
        space=space,
        position=(x, y, z),
        radius=radius,
        influence=influence
    )


@router.get("/{grid_id}/density", response_model=DensityResponse)
async def calculate_density(
    grid_id: int,
    space: int = Field(0, ge=0, le=7, description="Coordinate space"),
    x: float = Field(0.0, description="Position X"),
    y: float = Field(0.0, description="Position Y"),
    z: float = Field(0.0, description="Position Z"),
    radius: float = Field(10.0, ge=0.1, description="Search radius")
):
    """Calculate token density in a region."""
    check_grid_available()

    if grid_id not in GRID_INSTANCES:
        raise HTTPException(404, f"Grid {grid_id} not found")

    grid = GRID_INSTANCES[grid_id]

    # Calculate density
    density = grid.calculate_density(space, x, y, z, radius)

    # Count tokens in range
    results = grid.range_query(space, x, y, z, radius)
    tokens_in_range = len(results)

    return DensityResponse(
        grid_id=grid_id,
        space=space,
        position=(x, y, z),
        radius=radius,
        density=density,
        tokens_in_range=tokens_in_range
    )


@router.delete("/{grid_id}", status_code=204)
async def delete_grid(grid_id: int):
    """Delete a Grid instance."""
    check_grid_available()

    if grid_id not in GRID_INSTANCES:
        raise HTTPException(404, f"Grid {grid_id} not found")

    del GRID_INSTANCES[grid_id]
