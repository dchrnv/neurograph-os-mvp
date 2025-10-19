"""
Token v2.0 API Routes - Simplified MVP
Работает напрямую с in-memory хранилищем
"""

from typing import List, Optional, Dict, Any
from fastapi import APIRouter, HTTPException, status, Query
from pydantic import BaseModel, Field
import time

from src.core.token.token_v2 import (
    Token,
    create_token_id,
    validate_token,
    FLAG_ACTIVE,
    FLAG_PERSISTENT,
    TYPE_OBJECT,
    TYPE_CONCEPT,
    TYPE_EVENT
)

# ═══════════════════════════════════════════════════════
# IN-MEMORY STORAGE (MVP - заменить на БД позже)
# ═══════════════════════════════════════════════════════

TOKEN_STORAGE: Dict[int, Token] = {}
NEXT_LOCAL_ID = 1

router = APIRouter(prefix="/api/v1/tokens", tags=["Tokens V2"])


# ═══════════════════════════════════════════════════════
# SCHEMAS
# ═══════════════════════════════════════════════════════

class CoordinatesRequest(BaseModel):
    """Coordinates for a specific level"""
    x: Optional[float] = None
    y: Optional[float] = None
    z: Optional[float] = None


class TokenCreateRequest(BaseModel):
    """Request to create a new token"""
    entity_type: int = Field(default=0, ge=0, le=15, description="Entity type (0-15)")
    domain: int = Field(default=0, ge=0, le=15, description="Domain (0-15)")
    weight: float = Field(default=0.5, ge=0.0, le=1.0, description="Token weight")
    field_radius: float = Field(default=1.0, ge=0.0, le=2.55, description="Field radius")
    field_strength: float = Field(default=1.0, ge=0.0, le=1.0, description="Field strength")
    persistent: bool = Field(default=False, description="Should be persisted")

    # Coordinates for 8 levels (optional)
    l1_physical: Optional[CoordinatesRequest] = None
    l2_sensory: Optional[CoordinatesRequest] = None
    l3_motor: Optional[CoordinatesRequest] = None
    l4_emotional: Optional[CoordinatesRequest] = None
    l5_cognitive: Optional[CoordinatesRequest] = None
    l6_social: Optional[CoordinatesRequest] = None
    l7_temporal: Optional[CoordinatesRequest] = None
    l8_abstract: Optional[CoordinatesRequest] = None


class TokenResponse(BaseModel):
    """Token response"""
    id: int
    id_hex: str
    local_id: int
    entity_type: int
    domain: int
    weight: float
    field_radius: float
    field_strength: float
    timestamp: int
    age_seconds: int
    flags: Dict[str, Any]
    coordinates: Dict[str, Optional[tuple]]


class TokenListResponse(BaseModel):
    """List of tokens response"""
    tokens: List[TokenResponse]
    total: int


class SystemStatsResponse(BaseModel):
    """System statistics"""
    total_tokens: int
    next_local_id: int
    memory_bytes: int


# ═══════════════════════════════════════════════════════
# HELPERS
# ═══════════════════════════════════════════════════════

def token_to_response(token: Token) -> TokenResponse:
    """Convert Token to response model"""
    from src.core.token.token_v2 import extract_local_id, extract_entity_type, extract_domain

    return TokenResponse(
        id=token.id,
        id_hex=f"0x{token.id:08X}",
        local_id=extract_local_id(token.id),
        entity_type=extract_entity_type(token.id),
        domain=extract_domain(token.id),
        weight=token.weight,
        field_radius=token.field_radius,
        field_strength=token.field_strength,
        timestamp=token.timestamp,
        age_seconds=int(time.time()) - token.timestamp,
        flags={
            "active": token.has_flag(FLAG_ACTIVE),
            "persistent": token.has_flag(FLAG_PERSISTENT),
            "entity_type": token.get_entity_type()
        },
        coordinates={
            "L1_Physical": token.get_coordinates(0),
            "L2_Sensory": token.get_coordinates(1),
            "L3_Motor": token.get_coordinates(2),
            "L4_Emotional": token.get_coordinates(3),
            "L5_Cognitive": token.get_coordinates(4),
            "L6_Social": token.get_coordinates(5),
            "L7_Temporal": token.get_coordinates(6),
            "L8_Abstract": token.get_coordinates(7)
        }
    )


# ═══════════════════════════════════════════════════════
# ROUTES
# ═══════════════════════════════════════════════════════

@router.post("/", response_model=TokenResponse, status_code=status.HTTP_201_CREATED)
async def create_token(request: TokenCreateRequest):
    """
    Create a new token with v2.0 structure.

    - **entity_type**: Type of entity (0-15)
    - **domain**: Domain (0-15)
    - **weight**: Token weight (0.0-1.0)
    - **persistent**: Should be persisted
    - **lN_***: Coordinates for each level (optional)
    """
    global NEXT_LOCAL_ID

    try:
        # Create token ID
        token_id = create_token_id(NEXT_LOCAL_ID, request.entity_type, request.domain)
        NEXT_LOCAL_ID += 1

        # Create token
        token = Token(id=token_id)
        token.weight = request.weight
        token.field_radius = request.field_radius
        token.field_strength = request.field_strength

        # Set flags
        if request.persistent:
            token.set_flag(FLAG_PERSISTENT)

        # Set coordinates
        coord_map = [
            (0, request.l1_physical),
            (1, request.l2_sensory),
            (2, request.l3_motor),
            (3, request.l4_emotional),
            (4, request.l5_cognitive),
            (5, request.l6_social),
            (6, request.l7_temporal),
            (7, request.l8_abstract)
        ]

        for level, coords in coord_map:
            if coords:
                token.set_coordinates(level, coords.x, coords.y, coords.z)

        # Validate
        if not validate_token(token):
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail="Token validation failed"
            )

        # Store
        TOKEN_STORAGE[token_id] = token

        return token_to_response(token)

    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Error creating token: {str(e)}"
        )


@router.get("/", response_model=TokenListResponse)
async def list_tokens(
    limit: int = Query(10, ge=1, le=100, description="Max tokens to return"),
    offset: int = Query(0, ge=0, description="Number of tokens to skip")
):
    """
    List all tokens with pagination.

    - **limit**: Maximum number of tokens (1-100)
    - **offset**: Number to skip
    """
    tokens_list = list(TOKEN_STORAGE.values())
    total = len(tokens_list)

    # Pagination
    paginated = tokens_list[offset:offset + limit]

    return TokenListResponse(
        tokens=[token_to_response(t) for t in paginated],
        total=total
    )


@router.get("/{token_id}", response_model=TokenResponse)
async def get_token(token_id: int):
    """
    Get a specific token by ID.

    - **token_id**: Token ID (integer)
    """
    if token_id not in TOKEN_STORAGE:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=f"Token {token_id} (0x{token_id:08X}) not found"
        )

    return token_to_response(TOKEN_STORAGE[token_id])


@router.delete("/{token_id}", status_code=status.HTTP_204_NO_CONTENT)
async def delete_token(token_id: int):
    """
    Delete a token.

    - **token_id**: Token ID to delete
    """
    if token_id not in TOKEN_STORAGE:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=f"Token {token_id} not found"
        )

    del TOKEN_STORAGE[token_id]


@router.get("/stats/system", response_model=SystemStatsResponse)
async def system_stats():
    """
    Get system statistics.
    """
    total_bytes = sum(64 for _ in TOKEN_STORAGE)  # Each token is 64 bytes

    return SystemStatsResponse(
        total_tokens=len(TOKEN_STORAGE),
        next_local_id=NEXT_LOCAL_ID,
        memory_bytes=total_bytes
    )


@router.post("/examples/create", response_model=Dict[str, List[TokenResponse]])
async def create_examples():
    """
    Create example tokens for testing.
    Creates tokens with different entity types and coordinate levels.
    """
    global NEXT_LOCAL_ID

    examples = []

    # Example 1: Physical object
    req1 = TokenCreateRequest(
        entity_type=1,  # OBJECT
        domain=0,
        weight=0.7,
        persistent=True,
        l1_physical=CoordinatesRequest(x=10.5, y=20.3, z=1.5)
    )
    token1 = await create_token(req1)
    examples.append(token1)

    # Example 2: Emotional concept
    req2 = TokenCreateRequest(
        entity_type=5,  # CONCEPT
        domain=0,
        weight=0.8,
        l4_emotional=CoordinatesRequest(x=0.8, y=0.5, z=0.3)  # Joy
    )
    token2 = await create_token(req2)
    examples.append(token2)

    # Example 3: Event with temporal coordinates
    req3 = TokenCreateRequest(
        entity_type=2,  # EVENT
        domain=0,
        weight=0.9,
        l7_temporal=CoordinatesRequest(x=60.0, y=30.0, z=0.0)
    )
    token3 = await create_token(req3)
    examples.append(token3)

    return {"examples": examples}


@router.delete("/admin/clear", status_code=status.HTTP_204_NO_CONTENT)
async def clear_all_tokens():
    """
    Clear all tokens (admin endpoint for testing).
    """
    global NEXT_LOCAL_ID
    TOKEN_STORAGE.clear()
    NEXT_LOCAL_ID = 1
