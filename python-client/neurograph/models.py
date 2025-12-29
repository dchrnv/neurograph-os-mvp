"""
NeuroGraph Data Models

Pydantic models for request/response objects.
"""

from datetime import datetime
from typing import Optional, List, Dict, Any
from pydantic import BaseModel, Field, ConfigDict


# ============================================================================
# Authentication Models
# ============================================================================

class User(BaseModel):
    """User model."""

    user_id: str
    username: str
    email: Optional[str] = None
    full_name: Optional[str] = None
    role: str
    scopes: List[str] = []
    disabled: bool = False
    created_at: Optional[datetime] = None

    model_config = ConfigDict(from_attributes=True)


class LoginResponse(BaseModel):
    """Login response with JWT tokens."""

    access_token: str
    refresh_token: str
    token_type: str = "bearer"
    expires_in: int
    user: User


# ============================================================================
# API Key Models
# ============================================================================

class APIKey(BaseModel):
    """API Key model."""

    key_id: str
    name: str
    key_prefix: str
    scopes: List[str] = []
    created_at: datetime
    expires_at: Optional[datetime] = None
    last_used_at: Optional[datetime] = None
    is_active: bool = True

    model_config = ConfigDict(from_attributes=True)


class APIKeyCreate(BaseModel):
    """Request to create API key."""

    name: str = Field(..., description="Descriptive name for the key")
    scopes: Optional[List[str]] = Field(None, description="List of scopes")
    expires_in_days: Optional[int] = Field(None, description="Expiration in days")


class APIKeyCreated(BaseModel):
    """Response when API key is created (includes full key)."""

    key_id: str
    name: str
    api_key: str  # Full key, only shown once!
    key_prefix: str
    scopes: List[str]
    created_at: datetime
    expires_at: Optional[datetime] = None


# ============================================================================
# Token Models
# ============================================================================

class Token(BaseModel):
    """Token model."""

    id: int = Field(..., description="Token ID")
    text: Optional[str] = Field(None, description="Token text")
    embedding: List[float] = Field(..., description="Token embedding vector")
    metadata: Optional[Dict[str, Any]] = Field(None, description="Token metadata")
    created_at: Optional[datetime] = None

    model_config = ConfigDict(from_attributes=True)


class TokenCreate(BaseModel):
    """Request to create a token."""

    text: str = Field(..., description="Text to create token from")
    metadata: Optional[Dict[str, Any]] = Field(None, description="Optional metadata")


class TokenUpdate(BaseModel):
    """Request to update a token."""

    text: Optional[str] = Field(None, description="New text")
    embedding: Optional[List[float]] = Field(None, description="New embedding")
    metadata: Optional[Dict[str, Any]] = Field(None, description="New metadata")


class TokenQuery(BaseModel):
    """Query for similar tokens."""

    query_vector: List[float] = Field(..., description="Query embedding vector")
    top_k: int = Field(10, ge=1, le=1000, description="Number of results to return")
    threshold: Optional[float] = Field(None, description="Similarity threshold")


class TokenQueryResult(BaseModel):
    """Token query result with similarity."""

    token: Token
    similarity: float = Field(..., description="Cosine similarity score")
    distance: Optional[float] = Field(None, description="Distance metric")


# ============================================================================
# Grid Models
# ============================================================================

class GridCell(BaseModel):
    """Grid cell model."""

    x: float
    y: float
    z: float
    token_ids: List[int] = []
    density: float = 0.0

    model_config = ConfigDict(from_attributes=True)


class GridQuery(BaseModel):
    """Grid spatial query."""

    center: List[float] = Field(..., description="Center point [x, y, z]")
    radius: float = Field(..., description="Search radius")
    max_results: int = Field(100, ge=1, le=10000)


class GridQueryResult(BaseModel):
    """Grid query result."""

    cells: List[GridCell]
    total_count: int
    query_time_ms: float


# ============================================================================
# CDNA Models
# ============================================================================

class CDNAConfig(BaseModel):
    """CDNA configuration model."""

    profile: str = "explorer"
    curiosity_scale: float = 1.0
    novelty_scale: float = 1.0
    affinity_scale: float = 1.0

    model_config = ConfigDict(from_attributes=True)


class CDNAUpdate(BaseModel):
    """CDNA configuration update."""

    curiosity_scale: Optional[float] = None
    novelty_scale: Optional[float] = None
    affinity_scale: Optional[float] = None


# ============================================================================
# Response Models
# ============================================================================

class SuccessResponse(BaseModel):
    """Generic success response."""

    success: bool = True
    message: str
    data: Optional[Dict[str, Any]] = None


class ErrorResponse(BaseModel):
    """Generic error response."""

    success: bool = False
    error: Dict[str, Any]


class PaginatedResponse(BaseModel):
    """Paginated response wrapper."""

    items: List[Any]
    total: int
    page: int
    page_size: int
    total_pages: int


# ============================================================================
# Health & Status Models
# ============================================================================

class HealthStatus(BaseModel):
    """Health check response."""

    status: str
    version: str
    uptime_seconds: Optional[float] = None
    timestamp: Optional[datetime] = None


class SystemStatus(BaseModel):
    """System status response."""

    api_version: str
    runtime_version: str
    tokens_count: int
    uptime_seconds: float
    memory_usage_mb: float
    requests_total: int
