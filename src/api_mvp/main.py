"""
NeuroGraph OS - MVP API
Clean implementation without legacy dependencies
"""

import sys
from pathlib import Path

# Add src to path
src_path = Path(__file__).parent.parent
sys.path.insert(0, str(src_path))

from typing import List, Optional, Dict, Any
from fastapi import FastAPI, HTTPException, status, Query
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import RedirectResponse, JSONResponse
from pydantic import BaseModel, Field
import time

# Import Token v2.0
from core.token.token_v2 import (
    Token,
    create_token_id,
    validate_token,
    extract_local_id,
    extract_entity_type,
    extract_domain,
    FLAG_ACTIVE,
    FLAG_PERSISTENT,
)

# ═══════════════════════════════════════════════════════
# IN-MEMORY STORAGE
# ═══════════════════════════════════════════════════════

TOKEN_STORAGE: Dict[int, Token] = {}
NEXT_LOCAL_ID = 1

# ═══════════════════════════════════════════════════════
# SCHEMAS
# ═══════════════════════════════════════════════════════

class CoordinatesRequest(BaseModel):
    x: Optional[float] = None
    y: Optional[float] = None
    z: Optional[float] = None


class TokenCreateRequest(BaseModel):
    entity_type: int = Field(default=0, ge=0, le=15)
    domain: int = Field(default=0, ge=0, le=15)
    weight: float = Field(default=0.5, ge=0.0, le=1.0)
    field_radius: float = Field(default=1.0, ge=0.0, le=2.55)
    field_strength: float = Field(default=1.0, ge=0.0, le=1.0)
    persistent: bool = False

    l1_physical: Optional[CoordinatesRequest] = None
    l2_sensory: Optional[CoordinatesRequest] = None
    l3_motor: Optional[CoordinatesRequest] = None
    l4_emotional: Optional[CoordinatesRequest] = None
    l5_cognitive: Optional[CoordinatesRequest] = None
    l6_social: Optional[CoordinatesRequest] = None
    l7_temporal: Optional[CoordinatesRequest] = None
    l8_abstract: Optional[CoordinatesRequest] = None


class TokenResponse(BaseModel):
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
    tokens: List[TokenResponse]
    total: int


# ═══════════════════════════════════════════════════════
# HELPERS
# ═══════════════════════════════════════════════════════

def token_to_response(token: Token) -> TokenResponse:
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
        },
        coordinates={
            f"L{i+1}": token.get_coordinates(i)
            for i in range(8)
        }
    )


# ═══════════════════════════════════════════════════════
# CREATE APP
# ═══════════════════════════════════════════════════════

app = FastAPI(
    title="NeuroGraph OS - MVP",
    description="Token v2.0 spatial computing (MVP)",
    version="0.10.0",
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


# ═══════════════════════════════════════════════════════
# ROUTES
# ═══════════════════════════════════════════════════════

@app.get("/")
async def root():
    return RedirectResponse(url="/docs")


@app.get("/api")
async def api_info():
    return {
        "name": "NeuroGraph OS MVP",
        "version": "0.10.0",
        "token_version": "2.0",
        "endpoints": {
            "docs": "/docs",
            "tokens": "/api/v1/tokens",
            "health": "/health"
        }
    }


@app.get("/health")
async def health():
    return {"status": "healthy", "tokens": len(TOKEN_STORAGE)}


@app.post("/api/v1/tokens", response_model=TokenResponse, status_code=201)
async def create_token(request: TokenCreateRequest):
    global NEXT_LOCAL_ID

    token_id = create_token_id(NEXT_LOCAL_ID, request.entity_type, request.domain)
    NEXT_LOCAL_ID += 1

    token = Token(id=token_id)
    token.weight = request.weight
    token.field_radius = request.field_radius
    token.field_strength = request.field_strength

    if request.persistent:
        token.set_flag(FLAG_PERSISTENT)

    coord_map = [
        (0, request.l1_physical), (1, request.l2_sensory),
        (2, request.l3_motor), (3, request.l4_emotional),
        (4, request.l5_cognitive), (5, request.l6_social),
        (6, request.l7_temporal), (7, request.l8_abstract)
    ]

    for level, coords in coord_map:
        if coords:
            token.set_coordinates(level, coords.x, coords.y, coords.z)

    TOKEN_STORAGE[token_id] = token
    return token_to_response(token)


@app.get("/api/v1/tokens", response_model=TokenListResponse)
async def list_tokens(
    limit: int = Query(10, ge=1, le=100),
    offset: int = Query(0, ge=0)
):
    tokens_list = list(TOKEN_STORAGE.values())
    paginated = tokens_list[offset:offset + limit]

    return TokenListResponse(
        tokens=[token_to_response(t) for t in paginated],
        total=len(tokens_list)
    )


@app.get("/api/v1/tokens/{token_id}", response_model=TokenResponse)
async def get_token(token_id: int):
    if token_id not in TOKEN_STORAGE:
        raise HTTPException(404, f"Token {token_id:08X} not found")
    return token_to_response(TOKEN_STORAGE[token_id])


@app.delete("/api/v1/tokens/{token_id}", status_code=204)
async def delete_token(token_id: int):
    if token_id not in TOKEN_STORAGE:
        raise HTTPException(404, f"Token not found")
    del TOKEN_STORAGE[token_id]


@app.post("/api/v1/tokens/examples/create")
async def create_examples():
    examples = []

    # Physical object
    req1 = TokenCreateRequest(
        entity_type=1, domain=0, weight=0.7, persistent=True,
        l1_physical=CoordinatesRequest(x=10.5, y=20.3, z=1.5)
    )
    token1 = await create_token(req1)
    examples.append(token1)

    # Emotional concept
    req2 = TokenCreateRequest(
        entity_type=5, domain=0, weight=0.8,
        l4_emotional=CoordinatesRequest(x=0.8, y=0.5, z=0.3)
    )
    token2 = await create_token(req2)
    examples.append(token2)

    return {"examples": examples, "count": len(examples)}


@app.delete("/api/v1/tokens/admin/clear", status_code=204)
async def clear_all():
    global NEXT_LOCAL_ID
    TOKEN_STORAGE.clear()
    NEXT_LOCAL_ID = 1


if __name__ == "__main__":
    import uvicorn

    print("=" * 60)
    print("🚀 NeuroGraph OS MVP API")
    print("=" * 60)
    print("📖 Docs:   http://localhost:8000/docs")
    print("💚 Health: http://localhost:8000/health")
    print("=" * 60)

    uvicorn.run(app, host="0.0.0.0", port=8000, log_level="info")
