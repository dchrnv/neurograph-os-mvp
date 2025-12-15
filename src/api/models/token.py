"""
Token Models

Request and response models for token endpoints.
Ported from MVP API with enhancements for production use.
"""

from pydantic import BaseModel, Field
from typing import List, Optional, Dict, Tuple


class CoordinatesRequest(BaseModel):
    """Coordinates for a single dimension."""
    x: Optional[float] = None
    y: Optional[float] = None
    z: Optional[float] = None


class TokenCreateRequest(BaseModel):
    """Request model for creating a token."""

    entity_type: int = Field(default=0, ge=0, le=15, description="Entity type (0-15)")
    domain: int = Field(default=0, ge=0, le=15, description="Domain (0-15)")
    weight: float = Field(default=0.5, ge=0.0, le=1.0, description="Token weight")
    field_radius: float = Field(default=1.0, ge=0.0, le=2.55, description="Field radius")
    field_strength: float = Field(default=1.0, ge=0.0, le=1.0, description="Field strength")
    persistent: bool = Field(default=False, description="Should token persist")

    # 8 levels of coordinates
    l1_physical: Optional[CoordinatesRequest] = Field(None, description="L1: Physical space")
    l2_sensory: Optional[CoordinatesRequest] = Field(None, description="L2: Sensory perception")
    l3_motor: Optional[CoordinatesRequest] = Field(None, description="L3: Motor control")
    l4_emotional: Optional[CoordinatesRequest] = Field(None, description="L4: Emotional state")
    l5_cognitive: Optional[CoordinatesRequest] = Field(None, description="L5: Cognitive processing")
    l6_social: Optional[CoordinatesRequest] = Field(None, description="L6: Social interaction")
    l7_temporal: Optional[CoordinatesRequest] = Field(None, description="L7: Temporal location")
    l8_abstract: Optional[CoordinatesRequest] = Field(None, description="L8: Abstract/semantic")

    class Config:
        json_schema_extra = {
            "example": {
                "entity_type": 1,
                "domain": 0,
                "weight": 0.7,
                "field_radius": 1.5,
                "field_strength": 0.8,
                "persistent": True,
                "l1_physical": {"x": 10.5, "y": 20.3, "z": 1.5}
            }
        }


class TokenUpdateRequest(BaseModel):
    """Request model for updating a token."""

    weight: Optional[float] = Field(None, ge=0.0, le=1.0, description="Token weight")
    field_radius: Optional[float] = Field(None, ge=0.0, le=2.55, description="Field radius")
    field_strength: Optional[float] = Field(None, ge=0.0, le=1.0, description="Field strength")

    # 8 levels of coordinates (all optional for updates)
    l1_physical: Optional[CoordinatesRequest] = None
    l2_sensory: Optional[CoordinatesRequest] = None
    l3_motor: Optional[CoordinatesRequest] = None
    l4_emotional: Optional[CoordinatesRequest] = None
    l5_cognitive: Optional[CoordinatesRequest] = None
    l6_social: Optional[CoordinatesRequest] = None
    l7_temporal: Optional[CoordinatesRequest] = None
    l8_abstract: Optional[CoordinatesRequest] = None


class TokenResponse(BaseModel):
    """Response model for a single token."""

    id: int = Field(..., description="Token ID (32-bit)")
    id_hex: str = Field(..., description="Token ID in hex format")
    local_id: int = Field(..., description="Local ID (24 bits)")
    entity_type: int = Field(..., description="Entity type (4 bits)")
    domain: int = Field(..., description="Domain (4 bits)")
    weight: float = Field(..., description="Token weight")
    field_radius: float = Field(..., description="Field radius")
    field_strength: float = Field(..., description="Field strength")
    timestamp: int = Field(..., description="Creation timestamp")
    age_seconds: int = Field(..., description="Age in seconds")
    flags: Dict[str, bool] = Field(..., description="Token flags")
    coordinates: Dict[str, Optional[Tuple[float, float, float]]] = Field(
        ...,
        description="Coordinates in 8 spaces"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "id": 16777217,
                "id_hex": "0x01000001",
                "local_id": 1,
                "entity_type": 1,
                "domain": 0,
                "weight": 0.7,
                "field_radius": 1.5,
                "field_strength": 0.8,
                "timestamp": 1702915200,
                "age_seconds": 3600,
                "flags": {
                    "active": True,
                    "persistent": True
                },
                "coordinates": {
                    "L1": (10.5, 20.3, 1.5),
                    "L2": None,
                    "L3": None,
                    "L4": None,
                    "L5": None,
                    "L6": None,
                    "L7": None,
                    "L8": None
                }
            }
        }


class TokenListResponse(BaseModel):
    """Response model for token list."""

    tokens: List[TokenResponse] = Field(..., description="List of tokens")
    total: int = Field(..., description="Total number of tokens")
    limit: int = Field(..., description="Pagination limit")
    offset: int = Field(..., description="Pagination offset")

    class Config:
        json_schema_extra = {
            "example": {
                "tokens": [],
                "total": 100,
                "limit": 10,
                "offset": 0
            }
        }


class TokenExamplesResponse(BaseModel):
    """Response for create examples endpoint."""

    examples: List[TokenResponse] = Field(..., description="Created example tokens")
    count: int = Field(..., description="Number of examples created")


class TokenClearResponse(BaseModel):
    """Response for clear all tokens endpoint."""

    cleared: int = Field(..., description="Number of tokens cleared")
    message: str = Field(..., description="Status message")
