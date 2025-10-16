"""
Pydantic schemas for API request/response models.
"""

from typing import Optional, List, Dict, Any
from datetime import datetime
from uuid import UUID

from pydantic import BaseModel, Field, ConfigDict


# ===== Token Schemas =====

class CoordinatesSchema(BaseModel):
    """3D coordinates with 8 levels."""
    x: List[float] = Field(..., min_length=8, max_length=8)
    y: List[float] = Field(..., min_length=8, max_length=8)
    z: List[float] = Field(..., min_length=8, max_length=8)


class TokenCreateRequest(BaseModel):
    """Request model for creating a token."""
    token_type: str = Field(default="default", alias="type")
    coordinates: CoordinatesSchema
    weight: float = Field(default=1.0, ge=0.0)
    flags: int = Field(default=0, ge=0)
    metadata: Dict[str, Any] = Field(default_factory=dict)
    
    model_config = ConfigDict(populate_by_name=True)


class TokenUpdateRequest(BaseModel):
    """Request model for updating a token."""
    token_type: Optional[str] = Field(None, alias="type")
    weight: Optional[float] = Field(None, ge=0.0)
    flags: Optional[int] = Field(None, ge=0)
    metadata: Optional[Dict[str, Any]] = None
    
    model_config = ConfigDict(populate_by_name=True)


class TokenResponse(BaseModel):
    """Response model for a token."""
    id: UUID
    token_type: str = Field(alias="type")
    coordinates: CoordinatesSchema
    weight: float
    flags: int
    timestamp: int
    metadata: Dict[str, Any]
    created_at: datetime
    updated_at: Optional[datetime] = None
    
    model_config = ConfigDict(from_attributes=True, populate_by_name=True)


class TokenListResponse(BaseModel):
    """Response model for token list."""
    tokens: List[TokenResponse]
    total: int
    limit: int
    offset: int


# ===== Graph Schemas =====

class ConnectionCreateRequest(BaseModel):
    """Request model for creating a connection."""
    source_id: UUID
    target_id: UUID
    connection_type: str = Field(default="generic")
    weight: float = Field(default=1.0, ge=0.0, le=1.0)
    decay_rate: float = Field(default=0.0, ge=0.0)
    bidirectional: bool = Field(default=False)
    metadata: Dict[str, Any] = Field(default_factory=dict)


class ConnectionUpdateRequest(BaseModel):
    """Request model for updating a connection."""
    weight: Optional[float] = Field(None, ge=0.0, le=1.0)
    decay_rate: Optional[float] = Field(None, ge=0.0)
    metadata: Optional[Dict[str, Any]] = None


class ConnectionResponse(BaseModel):
    """Response model for a connection."""
    id: UUID
    source_id: UUID
    target_id: UUID
    connection_type: str
    weight: float
    decay_rate: float
    bidirectional: bool
    metadata: Dict[str, Any]
    generation: int
    fitness: float
    created_at: datetime
    updated_at: Optional[datetime] = None
    
    model_config = ConfigDict(from_attributes=True)


class ConnectionListResponse(BaseModel):
    """Response model for connection list."""
    connections: List[ConnectionResponse]
    total: int
    limit: int
    offset: int


class NeighborsResponse(BaseModel):
    """Response model for token neighbors."""
    token_id: UUID
    neighbors: List[ConnectionResponse]
    count: int
    direction: str


class PathResponse(BaseModel):
    """Response model for path finding."""
    source_id: UUID
    target_id: UUID
    paths: List[List[UUID]]
    count: int


class GraphStatsResponse(BaseModel):
    """Response model for graph statistics."""
    total_nodes: int
    total_edges: int
    avg_degree: float
    density: float
    connected_components: Optional[int] = None


class DegreeResponse(BaseModel):
    """Response model for node degree."""
    token_id: UUID
    in_degree: int
    out_degree: int
    total_degree: int


# ===== Experience Schemas =====

class ExperienceEventCreateRequest(BaseModel):
    """Request model for creating an experience event."""
    event_type: str
    token_id: Optional[UUID] = None
    state_before: Optional[Dict[str, Any]] = None
    state_after: Optional[Dict[str, Any]] = None
    action: Optional[Dict[str, Any]] = None
    reward: float = Field(default=0.0)
    metadata: Dict[str, Any] = Field(default_factory=dict)
    priority: float = Field(default=1.0, ge=0.0)
    trajectory_id: Optional[UUID] = None


class ExperienceEventResponse(BaseModel):
    """Response model for an experience event."""
    id: UUID
    event_type: str
    timestamp: int
    token_id: Optional[UUID] = None
    state_before: Optional[Dict[str, Any]] = None
    state_after: Optional[Dict[str, Any]] = None
    action: Optional[Dict[str, Any]] = None
    reward: float
    metadata: Dict[str, Any]
    priority: float
    trajectory_id: Optional[UUID] = None
    sequence_number: int
    created_at: datetime
    
    model_config = ConfigDict(from_attributes=True)


class ExperienceListResponse(BaseModel):
    """Response model for experience event list."""
    events: List[ExperienceEventResponse]
    total: int
    limit: int
    offset: int


# ===== System Schemas =====

class HealthResponse(BaseModel):
    """Response model for health check."""
    status: str
    version: str
    timestamp: datetime
    services: Dict[str, str]


class SystemStatsResponse(BaseModel):
    """Response model for system statistics."""
    tokens: int
    connections: int
    experience_events: int
    websocket_connections: int
    uptime_seconds: float


class ErrorResponse(BaseModel):
    """Response model for errors."""
    error: str
    message: str
    details: Optional[Dict[str, Any]] = None
    timestamp: datetime = Field(default_factory=datetime.utcnow)


# ===== Query Parameters =====

class PaginationParams(BaseModel):
    """Common pagination parameters."""
    limit: int = Field(default=10, ge=1, le=100)
    offset: int = Field(default=0, ge=0)


class TokenFilterParams(BaseModel):
    """Filter parameters for tokens."""
    token_type: Optional[str] = None
    min_weight: Optional[float] = Field(None, ge=0.0)
    max_weight: Optional[float] = Field(None, ge=0.0)
    flags: Optional[int] = None


class ConnectionFilterParams(BaseModel):
    """Filter parameters for connections."""
    connection_type: Optional[str] = None
    min_weight: Optional[float] = Field(None, ge=0.0)
    max_weight: Optional[float] = Field(None, ge=0.0)


class SpatialSearchRequest(BaseModel):
    """Request model for spatial search."""
    min_x: float
    min_y: float
    min_z: float
    max_x: float
    max_y: float
    max_z: float
    level: int = Field(default=0, ge=0, le=7)


# ===== Batch Operations =====

class TokenBatchCreateRequest(BaseModel):
    """Request model for batch token creation."""
    tokens: List[TokenCreateRequest] = Field(..., min_length=1, max_length=100)


class TokenBatchCreateResponse(BaseModel):
    """Response model for batch token creation."""
    created: List[TokenResponse]
    failed: List[Dict[str, Any]]
    total_created: int
    total_failed: int


class ConnectionBatchCreateRequest(BaseModel):
    """Request model for batch connection creation."""
    connections: List[ConnectionCreateRequest] = Field(..., min_length=1, max_length=100)


class ConnectionBatchCreateResponse(BaseModel):
    """Response model for batch connection creation."""
    created: List[ConnectionResponse]
    failed: List[Dict[str, Any]]
    total_created: int
    total_failed: int