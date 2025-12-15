"""
Query Models

Request and response models for query endpoints.
"""

from pydantic import BaseModel, Field
from typing import List, Optional, Dict
from enum import Enum


class CoordinateSpace(str, Enum):
    """Available coordinate spaces for queries."""
    L1_PHYSICAL = "L1Physical"
    L2_SENSORY = "L2Sensory"
    L3_MOTOR = "L3Motor"
    L4_EMOTIONAL = "L4Emotional"
    L5_COGNITIVE = "L5Cognitive"
    L6_SOCIAL = "L6Social"
    L7_TEMPORAL = "L7Temporal"
    L8_ABSTRACT = "L8Abstract"


class QueryRequest(BaseModel):
    """Query request model."""

    text: str = Field(..., description="Query text", min_length=1, max_length=1000)
    limit: int = Field(10, description="Maximum number of results", ge=1, le=100)
    threshold: float = Field(0.0, description="Minimum similarity threshold", ge=0.0, le=1.0)
    spaces: Optional[List[CoordinateSpace]] = Field(
        None,
        description="Coordinate spaces to search in (default: L1Physical)"
    )
    include_connections: bool = Field(
        False,
        description="Include connection information in results"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "text": "cat",
                "limit": 5,
                "threshold": 0.5,
                "spaces": ["L1Physical"],
                "include_connections": False
            }
        }


class TokenResult(BaseModel):
    """Token result in query response."""

    token_id: Optional[int] = Field(None, description="Token ID (if available)")
    label: str = Field(..., description="Token label/word")
    score: float = Field(..., description="Similarity score", ge=0.0, le=1.0)
    entity_type: str = Field("Concept", description="Entity type")
    coordinates: Optional[Dict[str, List[float]]] = Field(
        None,
        description="Token coordinates in different spaces"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "token_id": 4523,
                "label": "dog",
                "score": 0.92,
                "entity_type": "Concept",
                "coordinates": {
                    "L1Physical": [0.12, 0.34, 0.56]
                }
            }
        }


class ConnectionInfo(BaseModel):
    """Connection information."""

    source_id: int = Field(..., description="Source token ID")
    target_id: int = Field(..., description="Target token ID")
    connection_type: str = Field(..., description="Connection type")
    strength: float = Field(..., description="Connection strength", ge=0.0, le=1.0)


class QueryResponse(BaseModel):
    """Query response data."""

    signal_id: str = Field(..., description="Unique signal ID for feedback")
    query_text: str = Field(..., description="Original query text")
    processing_time_ms: float = Field(..., description="Processing time in milliseconds")
    confidence: float = Field(..., description="Overall confidence", ge=0.0, le=1.0)
    interpretation: str = Field("semantic_query", description="Query interpretation")
    tokens: List[TokenResult] = Field(..., description="Matching tokens")
    connections: Optional[List[ConnectionInfo]] = Field(
        None,
        description="Connections between tokens"
    )
    total_candidates: int = Field(..., description="Total candidates considered")

    class Config:
        json_schema_extra = {
            "example": {
                "signal_id": "550e8400-e29b-41d4-a716-446655440000",
                "query_text": "cat",
                "processing_time_ms": 14.2,
                "confidence": 0.87,
                "interpretation": "semantic_query",
                "tokens": [
                    {
                        "token_id": 4523,
                        "label": "dog",
                        "score": 0.92,
                        "entity_type": "Concept"
                    }
                ],
                "connections": [],
                "total_candidates": 247
            }
        }
