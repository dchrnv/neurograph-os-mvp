"""
Token Endpoints

CRUD operations for tokens with 8-dimensional coordinate spaces.
Ported from MVP API with production enhancements.
"""

from fastapi import APIRouter, Depends, HTTPException, Query, status
from typing import Dict, Any
import logging
import time
import sys
from pathlib import Path

# Add src/core to path
src_path = Path(__file__).parent.parent.parent.parent
sys.path.insert(0, str(src_path))

from core.token.token_v2 import (
    Token, extract_local_id, extract_entity_type, extract_domain,
    FLAG_ACTIVE, FLAG_PERSISTENT
)

from ..models.response import ApiResponse
from ..models.token import (
    TokenCreateRequest,
    TokenUpdateRequest,
    TokenResponse,
    TokenListResponse,
    TokenExamplesResponse,
    TokenClearResponse,
    CoordinatesRequest,
)
from ..models.auth import User
from ..dependencies import get_token_storage
from ..config import settings
from ..auth.dependencies import get_current_active_user
from ..auth.permissions import Permission, require_permission

logger = logging.getLogger(__name__)
router = APIRouter()


# =============================================================================
# Helper Functions
# =============================================================================

def token_to_response(token: Token) -> TokenResponse:
    """Convert Token to TokenResponse model."""
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


def create_request_to_dict(request: TokenCreateRequest) -> Dict[str, Any]:
    """Convert TokenCreateRequest to storage dict."""
    data = {
        'entity_type': request.entity_type,
        'domain': request.domain,
        'weight': request.weight,
        'field_radius': request.field_radius,
        'field_strength': request.field_strength,
        'persistent': request.persistent,
    }

    # Add coordinates
    coord_attrs = [
        ('l1_physical', request.l1_physical),
        ('l2_sensory', request.l2_sensory),
        ('l3_motor', request.l3_motor),
        ('l4_emotional', request.l4_emotional),
        ('l5_cognitive', request.l5_cognitive),
        ('l6_social', request.l6_social),
        ('l7_temporal', request.l7_temporal),
        ('l8_abstract', request.l8_abstract),
    ]

    for attr_name, coords in coord_attrs:
        if coords:
            data[attr_name] = {'x': coords.x, 'y': coords.y, 'z': coords.z}

    return data


def update_request_to_dict(request: TokenUpdateRequest) -> Dict[str, Any]:
    """Convert TokenUpdateRequest to storage dict."""
    data = {}

    if request.weight is not None:
        data['weight'] = request.weight
    if request.field_radius is not None:
        data['field_radius'] = request.field_radius
    if request.field_strength is not None:
        data['field_strength'] = request.field_strength

    # Add coordinates if provided
    coord_attrs = [
        ('l1_physical', request.l1_physical),
        ('l2_sensory', request.l2_sensory),
        ('l3_motor', request.l3_motor),
        ('l4_emotional', request.l4_emotional),
        ('l5_cognitive', request.l5_cognitive),
        ('l6_social', request.l6_social),
        ('l7_temporal', request.l7_temporal),
        ('l8_abstract', request.l8_abstract),
    ]

    for attr_name, coords in coord_attrs:
        if coords:
            data[attr_name] = {'x': coords.x, 'y': coords.y, 'z': coords.z}

    return data


# =============================================================================
# Endpoints
# =============================================================================

@router.post("/tokens", response_model=ApiResponse, status_code=status.HTTP_201_CREATED)
async def create_token(
    request: TokenCreateRequest,
    storage=Depends(get_token_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Create a new token.

    **Requires:** `tokens:write` permission

    Tokens are 64-byte atomic units with 8-dimensional coordinate spaces:
    - L1: Physical space
    - L2: Sensory perception
    - L3: Motor control
    - L4: Emotional state
    - L5: Cognitive processing
    - L6: Social interaction
    - L7: Temporal location
    - L8: Abstract/semantic
    """
    # Check permission
    if Permission.WRITE_TOKENS.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.WRITE_TOKENS.value} required"
        )

    if not settings.ENABLE_NEW_TOKEN_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Token API not enabled"
        )

    try:
        # Convert request to storage format
        token_data = create_request_to_dict(request)

        # Create token
        token = storage.create(token_data)

        logger.info(f"Token created: ID={token.id:08X}, type={request.entity_type}")

        # Convert to response
        response_data = token_to_response(token)

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0  # Calculated by middleware
        )

    except Exception as e:
        logger.error(f"Token creation failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to create token: {str(e)}"
        )


@router.get("/tokens", response_model=ApiResponse)
async def list_tokens(
    limit: int = Query(10, ge=1, le=100, description="Maximum number of results"),
    offset: int = Query(0, ge=0, description="Offset for pagination"),
    storage=Depends(get_token_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    List tokens with pagination.

    **Requires:** `tokens:read` permission

    Returns paginated list of all tokens in storage.
    """
    # Check permission
    if Permission.READ_TOKENS.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_TOKENS.value} required"
        )

    if not settings.ENABLE_NEW_TOKEN_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Token API not enabled"
        )

    try:
        # Get tokens
        tokens = storage.list(limit=limit, offset=offset)
        total = storage.count()

        # Convert to responses
        token_responses = [token_to_response(t) for t in tokens]

        response_data = TokenListResponse(
            tokens=token_responses,
            total=total,
            limit=limit,
            offset=offset
        )

        logger.debug(f"Listed {len(tokens)} tokens (total: {total})")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except Exception as e:
        logger.error(f"Token listing failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to list tokens: {str(e)}"
        )


@router.get("/tokens/{token_id}", response_model=ApiResponse)
async def get_token(
    token_id: int,
    storage=Depends(get_token_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Get specific token by ID.

    **Requires:** `tokens:read` permission

    Returns detailed information about a single token.
    """
    # Check permission
    if Permission.READ_TOKENS.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_TOKENS.value} required"
        )

    if not settings.ENABLE_NEW_TOKEN_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Token API not enabled"
        )

    try:
        token = storage.get(token_id)

        if not token:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Token {token_id:08X} not found"
            )

        response_data = token_to_response(token)

        logger.debug(f"Retrieved token: ID={token_id:08X}")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Token retrieval failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to get token: {str(e)}"
        )


@router.put("/tokens/{token_id}", response_model=ApiResponse)
async def update_token(
    token_id: int,
    request: TokenUpdateRequest,
    storage=Depends(get_token_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Update existing token.

    **Requires:** `tokens:write` permission

    Allows updating weight, field parameters, and coordinates.
    """
    # Check permission
    if Permission.WRITE_TOKENS.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.WRITE_TOKENS.value} required"
        )

    if not settings.ENABLE_NEW_TOKEN_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Token API not enabled"
        )

    try:
        # Check if exists
        existing = storage.get(token_id)
        if not existing:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Token {token_id:08X} not found"
            )

        # Convert request to storage format
        update_data = update_request_to_dict(request)

        # Update token
        updated_token = storage.update(token_id, update_data)

        if not updated_token:
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail="Update failed"
            )

        logger.info(f"Token updated: ID={token_id:08X}")

        response_data = token_to_response(updated_token)

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Token update failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to update token: {str(e)}"
        )


@router.delete("/tokens/{token_id}", status_code=status.HTTP_204_NO_CONTENT)
async def delete_token(
    token_id: int,
    storage=Depends(get_token_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Delete token.

    **Requires:** `tokens:delete` permission

    Permanently removes token from storage.
    """
    # Check permission
    if Permission.DELETE_TOKENS.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.DELETE_TOKENS.value} required"
        )

    if not settings.ENABLE_NEW_TOKEN_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Token API not enabled"
        )

    try:
        deleted = storage.delete(token_id)

        if not deleted:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Token {token_id:08X} not found"
            )

        logger.info(f"Token deleted: ID={token_id:08X}")

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Token deletion failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to delete token: {str(e)}"
        )


@router.post("/tokens/examples/create", response_model=ApiResponse)
async def create_examples(
    storage=Depends(get_token_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Create example tokens for testing.

    **Requires:** `tokens:write` permission

    Creates two example tokens:
    - Physical object (entity_type=1) with L1 coordinates
    - Emotional concept (entity_type=5) with L4 coordinates
    """
    # Check permission
    if Permission.WRITE_TOKENS.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.WRITE_TOKENS.value} required"
        )

    if not settings.ENABLE_NEW_TOKEN_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Token API not enabled"
        )

    try:
        examples = []

        # Example 1: Physical object
        req1_data = create_request_to_dict(TokenCreateRequest(
            entity_type=1,
            domain=0,
            weight=0.7,
            persistent=True,
            l1_physical=CoordinatesRequest(x=10.5, y=20.3, z=1.5)
        ))
        token1 = storage.create(req1_data)
        examples.append(token_to_response(token1))

        # Example 2: Emotional concept
        req2_data = create_request_to_dict(TokenCreateRequest(
            entity_type=5,
            domain=0,
            weight=0.8,
            l4_emotional=CoordinatesRequest(x=0.8, y=0.5, z=0.3)
        ))
        token2 = storage.create(req2_data)
        examples.append(token_to_response(token2))

        logger.info(f"Created {len(examples)} example tokens")

        response_data = TokenExamplesResponse(
            examples=examples,
            count=len(examples)
        )

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except Exception as e:
        logger.error(f"Example creation failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to create examples: {str(e)}"
        )


@router.delete("/tokens/admin/clear", response_model=ApiResponse)
async def clear_all_tokens(
    storage=Depends(get_token_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Clear all tokens (admin only).

    **Requires:** `config:admin` permission

    Deletes all tokens from storage and resets ID counter.
    Use with caution!
    """
    # Check admin permission
    if Permission.ADMIN_CONFIG.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.ADMIN_CONFIG.value} required (admin only)"
        )

    if not settings.ENABLE_NEW_TOKEN_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Token API not enabled"
        )

    try:
        # TODO: Add admin authentication check
        # For now, anyone can clear (development mode)

        cleared = storage.clear()

        logger.warning(f"Cleared all tokens: {cleared} tokens deleted")

        response_data = TokenClearResponse(
            cleared=cleared,
            message=f"Cleared {cleared} tokens"
        )

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except Exception as e:
        logger.error(f"Token clearing failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to clear tokens: {str(e)}"
        )
