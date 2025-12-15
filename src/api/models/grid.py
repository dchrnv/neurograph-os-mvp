"""
Grid Models

Request and response models for grid endpoints.
Ported from MVP API with enhancements for production use.
"""

from pydantic import BaseModel, Field
from typing import List, Tuple, Optional, Dict, Any


class GridConfigRequest(BaseModel):
    """Grid configuration parameters."""

    bucket_size: float = Field(
        default=10.0,
        ge=0.1,
        le=1000.0,
        description="Spatial bucket size"
    )
    density_threshold: float = Field(
        default=0.5,
        ge=0.0,
        le=1.0,
        description="Density threshold for field detection"
    )
    min_field_nodes: int = Field(
        default=3,
        ge=1,
        le=100,
        description="Minimum nodes to form a field"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "bucket_size": 10.0,
                "density_threshold": 0.5,
                "min_field_nodes": 3
            }
        }


class GridCreateResponse(BaseModel):
    """Response for grid creation."""

    grid_id: int = Field(..., description="Created grid ID")
    config: Dict[str, Any] = Field(..., description="Grid configuration")
    status: str = Field(..., description="Creation status")

    class Config:
        json_schema_extra = {
            "example": {
                "grid_id": 1,
                "config": {
                    "bucket_size": 10.0,
                    "density_threshold": 0.5,
                    "min_field_nodes": 3
                },
                "status": "created"
            }
        }


class GridInfoResponse(BaseModel):
    """Response for grid information."""

    grid_id: int = Field(..., description="Grid ID")
    token_count: int = Field(..., description="Number of tokens in grid")
    config: Dict[str, Any] = Field(..., description="Grid configuration")

    class Config:
        json_schema_extra = {
            "example": {
                "grid_id": 1,
                "token_count": 150,
                "config": {
                    "bucket_size": 10.0,
                    "density_threshold": 0.5,
                    "min_field_nodes": 3
                }
            }
        }


class GridStatusResponse(BaseModel):
    """Response for grid status check."""

    available: bool = Field(..., description="Whether Grid is available")
    grids_count: int = Field(..., description="Number of grid instances")
    message: str = Field(..., description="Status message")

    class Config:
        json_schema_extra = {
            "example": {
                "available": True,
                "grids_count": 3,
                "message": "Grid V2.0 (Rust) is ready"
            }
        }


class GridTokenAddResponse(BaseModel):
    """Response for adding token to grid."""

    grid_id: int = Field(..., description="Grid ID")
    token_id: int = Field(..., description="Token ID")
    status: str = Field(..., description="Operation status")
    grid_size: int = Field(..., description="Current grid size")


class NeighborResult(BaseModel):
    """Single neighbor result."""

    token_id: int = Field(..., description="Neighbor token ID")
    distance: float = Field(..., description="Distance to neighbor")

    class Config:
        json_schema_extra = {
            "example": {
                "token_id": 42,
                "distance": 5.23
            }
        }


class NeighborsResponse(BaseModel):
    """Response for neighbor query."""

    grid_id: int = Field(..., description="Grid ID")
    center_token_id: int = Field(..., description="Center token ID")
    space: int = Field(..., description="Coordinate space (0-7)")
    radius: float = Field(..., description="Search radius")
    neighbors: List[NeighborResult] = Field(..., description="Found neighbors")
    count: int = Field(..., description="Number of neighbors found")

    class Config:
        json_schema_extra = {
            "example": {
                "grid_id": 1,
                "center_token_id": 100,
                "space": 0,
                "radius": 10.0,
                "neighbors": [
                    {"token_id": 101, "distance": 3.2},
                    {"token_id": 102, "distance": 7.5}
                ],
                "count": 2
            }
        }


class RangeQueryResponse(BaseModel):
    """Response for range query."""

    grid_id: int = Field(..., description="Grid ID")
    space: int = Field(..., description="Coordinate space (0-7)")
    center: Tuple[float, float, float] = Field(..., description="Query center point")
    radius: float = Field(..., description="Search radius")
    results: List[NeighborResult] = Field(..., description="Found tokens")
    count: int = Field(..., description="Number of tokens found")

    class Config:
        json_schema_extra = {
            "example": {
                "grid_id": 1,
                "space": 0,
                "center": (10.0, 20.0, 5.0),
                "radius": 15.0,
                "results": [
                    {"token_id": 50, "distance": 5.3},
                    {"token_id": 51, "distance": 12.1}
                ],
                "count": 2
            }
        }


class FieldInfluenceResponse(BaseModel):
    """Response for field influence calculation."""

    grid_id: int = Field(..., description="Grid ID")
    space: int = Field(..., description="Coordinate space (0-7)")
    position: Tuple[float, float, float] = Field(..., description="Query position")
    radius: float = Field(..., description="Calculation radius")
    influence: float = Field(..., description="Calculated field influence")

    class Config:
        json_schema_extra = {
            "example": {
                "grid_id": 1,
                "space": 0,
                "position": (10.0, 20.0, 5.0),
                "radius": 10.0,
                "influence": 0.73
            }
        }


class DensityResponse(BaseModel):
    """Response for density calculation."""

    grid_id: int = Field(..., description="Grid ID")
    space: int = Field(..., description="Coordinate space (0-7)")
    position: Tuple[float, float, float] = Field(..., description="Query position")
    radius: float = Field(..., description="Calculation radius")
    density: float = Field(..., description="Calculated density")
    tokens_in_range: int = Field(..., description="Number of tokens in range")

    class Config:
        json_schema_extra = {
            "example": {
                "grid_id": 1,
                "space": 0,
                "position": (10.0, 20.0, 5.0),
                "radius": 10.0,
                "density": 0.42,
                "tokens_in_range": 15
            }
        }
