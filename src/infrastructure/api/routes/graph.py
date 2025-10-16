"""
Graph API routes.
"""

from typing import List
from uuid import UUID

from fastapi import APIRouter, Depends, HTTPException, Query, status
from sqlalchemy.ext.asyncio import AsyncSession

from src.infrastructure.api.schemas import (
    ConnectionCreateRequest, ConnectionUpdateRequest, ConnectionResponse,
    ConnectionListResponse, NeighborsResponse, PathResponse,
    GraphStatsResponse, DegreeResponse, ConnectionBatchCreateRequest,
    ConnectionBatchCreateResponse
)
from src.infrastructure.api.dependencies import get_db_session
from src.infrastructure.persistence.repositories import RepositoryFactory
from src.core.utils.logger import get_logger

logger = get_logger(__name__)

router = APIRouter(prefix="/graph", tags=["Graph"])


@router.post("/connections", response_model=ConnectionResponse, status_code=status.HTTP_201_CREATED)
async def create_connection(
    request: ConnectionCreateRequest,
    session: AsyncSession = Depends(get_db_session)
):
    """
    Create a connection between two tokens.
    
    - **source_id**: Source token UUID
    - **target_id**: Target token UUID
    - **connection_type**: Type of connection (default: "generic")
    - **weight**: Connection weight (0.0-1.0)
    - **decay_rate**: Weight decay rate
    - **bidirectional**: Create bidirectional connection
    - **metadata**: Additional metadata
    """
    try:
        graph_repo = RepositoryFactory.create_graph_repository(session)
        
        connection = await graph_repo.create_connection(
            source_id=request.source_id,
            target_id=request.target_id,
            connection_type=request.connection_type,
            weight=request.weight,
            bidirectional=request.bidirectional,
            metadata=request.metadata
        )
        
        return ConnectionResponse.model_validate(connection)
    
    except Exception as e:
        logger.error(f"Error creating connection: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.get("/connections", response_model=ConnectionListResponse)
async def list_connections(
    limit: int = Query(10, ge=1, le=100),
    offset: int = Query(0, ge=0),
    connection_type: str = Query(None),
    session: AsyncSession = Depends(get_db_session)
):
    """
    List connections with pagination.
    
    - **limit**: Maximum number to return (1-100)
    - **offset**: Number to skip
    - **connection_type**: Filter by type
    """
    try:
        graph_repo = RepositoryFactory.create_graph_repository(session)
        
        connections = await graph_repo.get_all(limit=limit, offset=offset)
        total = await graph_repo.count()
        
        return ConnectionListResponse(
            connections=[ConnectionResponse.model_validate(c) for c in connections],
            total=total,
            limit=limit,
            offset=offset
        )
    
    except Exception as e:
        logger.error(f"Error listing connections: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.get("/connections/{connection_id}", response_model=ConnectionResponse)
async def get_connection(
    connection_id: UUID,
    session: AsyncSession = Depends(get_db_session)
):
    """
    Get a connection by ID.
    """
    try:
        graph_repo = RepositoryFactory.create_graph_repository(session)
        connection = await graph_repo.get_by_id(connection_id)
        
        if not connection:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Connection {connection_id} not found"
            )
        
        return ConnectionResponse.model_validate(connection)
    
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error getting connection: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.put("/connections/{connection_id}", response_model=ConnectionResponse)
async def update_connection(
    connection_id: UUID,
    request: ConnectionUpdateRequest,
    session: AsyncSession = Depends(get_db_session)
):
    """
    Update a connection.
    """
    try:
        graph_repo = RepositoryFactory.create_graph_repository(session)
        connection = await graph_repo.get_by_id(connection_id)
        
        if not connection:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Connection {connection_id} not found"
            )
        
        if request.weight is not None:
            await graph_repo.update_weight(connection_id, request.weight)
            connection.weight = request.weight
        
        if request.metadata is not None:
            connection.metadata = request.metadata
            await graph_repo.update(connection)
        
        return ConnectionResponse.model_validate(connection)
    
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error updating connection: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.delete("/connections/{connection_id}", status_code=status.HTTP_204_NO_CONTENT)
async def delete_connection(
    connection_id: UUID,
    session: AsyncSession = Depends(get_db_session)
):
    """
    Delete a connection.
    """
    try:
        graph_repo = RepositoryFactory.create_graph_repository(session)
        success = await graph_repo.delete(connection_id)
        
        if not success:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Connection {connection_id} not found"
            )
    
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error deleting connection: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.get("/tokens/{token_id}/neighbors", response_model=NeighborsResponse)
async def get_neighbors(
    token_id: UUID,
    direction: str = Query("both", regex="^(incoming|outgoing|both)$"),
    connection_type: str = Query(None),
    session: AsyncSession = Depends(get_db_session)
):
    """
    Get neighbors of a token.
    
    - **token_id**: Token UUID
    - **direction**: Direction (incoming, outgoing, both)
    - **connection_type**: Filter by connection type
    """
    try:
        graph_repo = RepositoryFactory.create_graph_repository(session)
        
        neighbors = await graph_repo.get_neighbors(
            token_id,
            direction=direction,
            connection_type=connection_type
        )
        
        return NeighborsResponse(
            token_id=token_id,
            neighbors=[ConnectionResponse.model_validate(n) for n in neighbors],
            count=len(neighbors),
            direction=direction
        )
    
    except Exception as e:
        logger.error(f"Error getting neighbors: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.get("/tokens/{token_id}/degree", response_model=DegreeResponse)
async def get_token_degree(
    token_id: UUID,
    session: AsyncSession = Depends(get_db_session)
):
    """
    Get degree (connectivity) of a token.
    
    - **token_id**: Token UUID
    """
    try:
        graph_repo = RepositoryFactory.create_graph_repository(session)
        degree = await graph_repo.get_degree(token_id)
        
        return DegreeResponse(
            token_id=token_id,
            **degree
        )
    
    except Exception as e:
        logger.error(f"Error getting degree: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.get("/path", response_model=PathResponse)
async def find_path(
    source_id: UUID = Query(...),
    target_id: UUID = Query(...),
    max_depth: int = Query(5, ge=1, le=10),
    session: AsyncSession = Depends(get_db_session)
):
    """
    Find paths between two tokens.
    
    - **source_id**: Source token UUID
    - **target_id**: Target token UUID
    - **max_depth**: Maximum search depth (1-10)
    """
    try:
        graph_repo = RepositoryFactory.create_graph_repository(session)
        
        paths = await graph_repo.find_path(source_id, target_id, max_depth)
        
        return PathResponse(
            source_id=source_id,
            target_id=target_id,
            paths=paths,
            count=len(paths)
        )
    
    except Exception as e:
        logger.error(f"Error finding path: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.get("/stats", response_model=GraphStatsResponse)
async def get_graph_stats(
    session: AsyncSession = Depends(get_db_session)
):
    """
    Get graph statistics.
    """
    try:
        graph_repo = RepositoryFactory.create_graph_repository(session)
        token_repo = RepositoryFactory.create_token_repository(session)
        
        total_nodes = await token_repo.count()
        total_edges = await graph_repo.count()
        
        avg_degree = 0.0
        density = 0.0
        
        if total_nodes > 0:
            avg_degree = (total_edges * 2) / total_nodes
            if total_nodes > 1:
                density = total_edges / (total_nodes * (total_nodes - 1))
        
        return GraphStatsResponse(
            total_nodes=total_nodes,
            total_edges=total_edges,
            avg_degree=avg_degree,
            density=density
        )
    
    except Exception as e:
        logger.error(f"Error getting graph stats: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@router.post("/connections/batch", response_model=ConnectionBatchCreateResponse, status_code=status.HTTP_201_CREATED)
async def batch_create_connections(
    request: ConnectionBatchCreateRequest,
    session: AsyncSession = Depends(get_db_session)
):
    """
    Create multiple connections in batch.
    
    - **connections**: List of connections to create (max 100)
    """
    try:
        graph_repo = RepositoryFactory.create_graph_repository(session)
        
        created = []
        failed = []
        
        for idx, conn_req in enumerate(request.connections):
            try:
                connection = await graph_repo.create_connection(
                    source_id=conn_req.source_id,
                    target_id=conn_req.target_id,
                    connection_type=conn_req.connection_type,
                    weight=conn_req.weight,
                    bidirectional=conn_req.bidirectional,
                    metadata=conn_req.metadata
                )
                created.append(ConnectionResponse.model_validate(connection))
            except Exception as e:
                failed.append({"index": idx, "error": str(e)})
        
        return ConnectionBatchCreateResponse(
            created=created,
            failed=failed,
            total_created=len(created),
            total_failed=len(failed)
        )
    
    except Exception as e:
        logger.error(f"Error in batch create connections: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )