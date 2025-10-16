"""
Token API routes.
"""

from typing import List
from uuid import UUID
import time

from fastapi import APIRouter, Depends, HTTPException, Query, status
from sqlalchemy.ext.asyncio import AsyncSession

from src.infrastructure.api.schemas import (
    TokenCreateRequest, TokenUpdateRequest, TokenResponse,
    TokenListResponse, PaginationParams, TokenFilterParams,
    SpatialSearchRequest, TokenBatchCreateRequest, TokenBatchCreateResponse,
    CoordinatesSchema
)
from src.infrastructure.api.dependencies import get_db_session
from src.infrastructure.persistence.repositories import RepositoryFactory
from src.infrastructure.persistence.models import TokenModel
from src.core.utils.logger import get_logger

logger = get_logger(__name__)

router = APIRouter(prefix="/tokens", tags=["Tokens"])


@router.post("/", response_model=TokenResponse, status_code=status.HTTP_201_CREATED)
async def create_token(
    request: TokenCreateRequest,
    session: AsyncSession = Depends(get_db_session)
):
    """
    Create a new token.
    
    - **type**: Token type (default: "default")
    - **coordinates**: 3D coordinates with 8 levels
    - **weight**: Token weight (default: 1.0)
    - **flags**: Binary flags (default: 0)
    - **metadata**: Additional metadata
    """
    try:
        token_repo = RepositoryFactory.create_token_repository(session)
        
        token = TokenModel(
            binary_data=b'\x00' * 64,
            coord_x=request.coordinates.x,
            coord_y=request.coordinates.y,
            coord_z=request.coordinates.z,
            flags=request.flags,
            weight=request.weight,
            timestamp=int(time.time() * 1000),
            token_type=request.token_type,
            metadata=request.metadata
        )
        
        token = await token_repo.create(token)
        
        return TokenResponse(
            id=token.id,
            type=token.token_type,
            coordinates=CoordinatesSchema(
                x=token.coord_x,
                y=token.coord_y,
                z=token.coord_z
            ),
            weight=token.weight,
            flags=token.flags,
            timestamp=token.timestamp,
            metadata=token.metadata,
            created_at=token.created_at,
            updated_at=token.updated_at
        )
    
    except Exception as e:
        logger.error(f"Error creating token: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.get("/", response_model=TokenListResponse)
async def list_tokens(
    limit: int = Query(10, ge=1, le=100),
    offset: int = Query(0, ge=0),
    token_type: str = Query(None),
    session: AsyncSession = Depends(get_db_session)
):
    """
    List tokens with pagination and filtering.
    
    - **limit**: Maximum number of tokens to return (1-100)
    - **offset**: Number of tokens to skip
    - **token_type**: Filter by token type
    """
    try:
        token_repo = RepositoryFactory.create_token_repository(session)
        
        if token_type:
            tokens = await token_repo.find_by_type(token_type)
            tokens = tokens[offset:offset + limit]
            total = len(tokens)
        else:
            tokens = await token_repo.get_all(limit=limit, offset=offset)
            total = await token_repo.count()
        
        token_responses = [
            TokenResponse(
                id=token.id,
                type=token.token_type,
                coordinates=CoordinatesSchema(
                    x=token.coord_x,
                    y=token.coord_y,
                    z=token.coord_z
                ),
                weight=token.weight,
                flags=token.flags,
                timestamp=token.timestamp,
                metadata=token.metadata,
                created_at=token.created_at,
                updated_at=token.updated_at
            )
            for token in tokens
        ]
        
        return TokenListResponse(
            tokens=token_responses,
            total=total,
            limit=limit,
            offset=offset
        )
    
    except Exception as e:
        logger.error(f"Error listing tokens: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.get("/{token_id}", response_model=TokenResponse)
async def get_token(
    token_id: UUID,
    session: AsyncSession = Depends(get_db_session)
):
    """
    Get a token by ID.
    
    - **token_id**: UUID of the token
    """
    try:
        token_repo = RepositoryFactory.create_token_repository(session)
        token = await token_repo.get_by_id(token_id)
        
        if not token:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Token {token_id} not found"
            )
        
        return TokenResponse(
            id=token.id,
            type=token.token_type,
            coordinates=CoordinatesSchema(
                x=token.coord_x,
                y=token.coord_y,
                z=token.coord_z
            ),
            weight=token.weight,
            flags=token.flags,
            timestamp=token.timestamp,
            metadata=token.metadata,
            created_at=token.created_at,
            updated_at=token.updated_at
        )
    
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error getting token: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.put("/{token_id}", response_model=TokenResponse)
async def update_token(
    token_id: UUID,
    request: TokenUpdateRequest,
    session: AsyncSession = Depends(get_db_session)
):
    """
    Update a token.
    
    - **token_id**: UUID of the token
    - **type**: New token type (optional)
    - **weight**: New weight (optional)
    - **flags**: New flags (optional)
    - **metadata**: New metadata (optional)
    """
    try:
        token_repo = RepositoryFactory.create_token_repository(session)
        token = await token_repo.get_by_id(token_id)
        
        if not token:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Token {token_id} not found"
            )
        
        # Update fields
        if request.token_type is not None:
            token.token_type = request.token_type
        if request.weight is not None:
            token.weight = request.weight
        if request.flags is not None:
            token.flags = request.flags
        if request.metadata is not None:
            token.metadata = request.metadata
        
        token = await token_repo.update(token)
        
        return TokenResponse(
            id=token.id,
            type=token.token_type,
            coordinates=CoordinatesSchema(
                x=token.coord_x,
                y=token.coord_y,
                z=token.coord_z
            ),
            weight=token.weight,
            flags=token.flags,
            timestamp=token.timestamp,
            metadata=token.metadata,
            created_at=token.created_at,
            updated_at=token.updated_at
        )
    
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error updating token: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.delete("/{token_id}", status_code=status.HTTP_204_NO_CONTENT)
async def delete_token(
    token_id: UUID,
    session: AsyncSession = Depends(get_db_session)
):
    """
    Delete a token.
    
    - **token_id**: UUID of the token
    """
    try:
        token_repo = RepositoryFactory.create_token_repository(session)
        success = await token_repo.delete(token_id)
        
        if not success:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Token {token_id} not found"
            )
    
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error deleting token: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.post("/search/spatial", response_model=TokenListResponse)
async def spatial_search(
    request: SpatialSearchRequest,
    session: AsyncSession = Depends(get_db_session)
):
    """
    Search tokens in a spatial region.
    
    - **min_x, min_y, min_z**: Minimum coordinates
    - **max_x, max_y, max_z**: Maximum coordinates
    - **level**: Coordinate level to search (0-7)
    """
    try:
        token_repo = RepositoryFactory.create_token_repository(session)
        
        tokens = await token_repo.find_in_region(
            min_coords=(request.min_x, request.min_y, request.min_z),
            max_coords=(request.max_x, request.max_y, request.max_z),
            level=request.level
        )
        
        token_responses = [
            TokenResponse(
                id=token.id,
                type=token.token_type,
                coordinates=CoordinatesSchema(
                    x=token.coord_x,
                    y=token.coord_y,
                    z=token.coord_z
                ),
                weight=token.weight,
                flags=token.flags,
                timestamp=token.timestamp,
                metadata=token.metadata,
                created_at=token.created_at,
                updated_at=token.updated_at
            )
            for token in tokens
        ]
        
        return TokenListResponse(
            tokens=token_responses,
            total=len(token_responses),
            limit=len(token_responses),
            offset=0
        )
    
    except Exception as e:
        logger.error(f"Error in spatial search: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.post("/batch", response_model=TokenBatchCreateResponse, status_code=status.HTTP_201_CREATED)
async def batch_create_tokens(
    request: TokenBatchCreateRequest,
    session: AsyncSession = Depends(get_db_session)
):
    """
    Create multiple tokens in batch.
    
    - **tokens**: List of tokens to create (max 100)
    """
    try:
        token_repo = RepositoryFactory.create_token_repository(session)
        
        created = []
        failed = []
        
        tokens_to_create = []
        for idx, token_req in enumerate(request.tokens):
            try:
                token = TokenModel(
                    binary_data=b'\x00' * 64,
                    coord_x=token_req.coordinates.x,
                    coord_y=token_req.coordinates.y,
                    coord_z=token_req.coordinates.z,
                    flags=token_req.flags,
                    weight=token_req.weight,
                    timestamp=int(time.time() * 1000),
                    token_type=token_req.token_type,
                    metadata=token_req.metadata
                )
                tokens_to_create.append(token)
            except Exception as e:
                failed.append({"index": idx, "error": str(e)})
        
        # Bulk create
        if tokens_to_create:
            created_tokens = await token_repo.bulk_create(tokens_to_create)
            
            for token in created_tokens:
                created.append(TokenResponse(
                    id=token.id,
                    type=token.token_type,
                    coordinates=CoordinatesSchema(
                        x=token.coord_x,
                        y=token.coord_y,
                        z=token.coord_z
                    ),
                    weight=token.weight,
                    flags=token.flags,
                    timestamp=token.timestamp,
                    metadata=token.metadata,
                    created_at=token.created_at,
                    updated_at=token.updated_at
                ))
        
        return TokenBatchCreateResponse(
            created=created,
            failed=failed,
            total_created=len(created),
            total_failed=len(failed)
        )
    
    except Exception as e:
        logger.error(f"Error in batch create: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.get("/count/total", response_model=dict)
async def count_tokens(
    token_type: str = Query(None),
    session: AsyncSession = Depends(get_db_session)
):
    """
    Count total tokens.
    
    - **token_type**: Filter by token type (optional)
    """
    try:
        token_repo = RepositoryFactory.create_token_repository(session)
        
        if token_type:
            tokens = await token_repo.find_by_type(token_type)
            count = len(tokens)
        else:
            count = await token_repo.count()
        
        return {"count": count, "type": token_type}
    
    except Exception as e:
        logger.error(f"Error counting tokens: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )