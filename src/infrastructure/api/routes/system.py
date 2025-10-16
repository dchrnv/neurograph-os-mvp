"""
System and Experience API routes.
"""

from datetime import datetime
from uuid import UUID

from fastapi import APIRouter, Depends, HTTPException, Query, status
from sqlalchemy.ext.asyncio import AsyncSession

from src.infrastructure.api.schemas import (
    HealthResponse, SystemStatsResponse,
    ExperienceEventCreateRequest, ExperienceEventResponse,
    ExperienceListResponse
)
from src.infrastructure.api.dependencies import get_db_session, get_db_manager
from src.infrastructure.persistence.repositories import RepositoryFactory
from src.infrastructure.websocket.server import connection_manager
from src.core.utils.logger import get_logger

logger = get_logger(__name__)

system_router = APIRouter(prefix="/system", tags=["System"])
experience_router = APIRouter(prefix="/experience", tags=["Experience"])


# ===== System Routes =====

@system_router.get("/health", response_model=HealthResponse)
async def health_check():
    """
    Health check endpoint.
    
    Returns system health status and service availability.
    """
    try:
        db_manager = await get_db_manager()
        pg_healthy = await db_manager.health_check()
        
        return HealthResponse(
            status="healthy" if pg_healthy else "unhealthy",
            version="0.3.0",
            timestamp=datetime.utcnow(),
            services={
                "postgres": "up" if pg_healthy else "down",
                "websocket": "up" if connection_manager else "down"
            }
        )
    
    except Exception as e:
        logger.error(f"Health check error: {e}")
        return HealthResponse(
            status="unhealthy",
            version="0.3.0",
            timestamp=datetime.utcnow(),
            services={
                "postgres": "unknown",
                "websocket": "unknown"
            }
        )


@system_router.get("/stats", response_model=SystemStatsResponse)
async def get_system_stats(
    session: AsyncSession = Depends(get_db_session)
):
    """
    Get system statistics.
    
    Returns counts of tokens, connections, events, and active WebSocket connections.
    """
    try:
        token_repo = RepositoryFactory.create_token_repository(session)
        graph_repo = RepositoryFactory.create_graph_repository(session)
        exp_repo = RepositoryFactory.create_experience_repository(session)
        
        tokens = await token_repo.count()
        connections = await graph_repo.count()
        events = await exp_repo.count()
        ws_connections = connection_manager.get_connection_count()
        
        return SystemStatsResponse(
            tokens=tokens,
            connections=connections,
            experience_events=events,
            websocket_connections=ws_connections,
            uptime_seconds=0.0  # TODO: Track actual uptime
        )
    
    except Exception as e:
        logger.error(f"Error getting system stats: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@system_router.get("/info")
async def get_system_info():
    """
    Get system information.
    """
    return {
        "name": "NeuroGraph OS",
        "version": "0.3.0",
        "architecture": "Clean Architecture with Hexagonal Pattern",
        "components": {
            "token_system": "64-byte binary format with 8-level coordinates",
            "spatial_grid": "Multi-scale sparse grid indexing",
            "graph_engine": "Connection management with CDNA validation",
            "experience_stream": "Event collection and replay",
            "websocket": "Real-time bidirectional communication"
        },
        "status": "development"
    }


# ===== Experience Routes =====

@experience_router.post("/events", response_model=ExperienceEventResponse, status_code=status.HTTP_201_CREATED)
async def create_experience_event(
    request: ExperienceEventCreateRequest,
    session: AsyncSession = Depends(get_db_session)
):
    """
    Create an experience event.
    
    - **event_type**: Type of event
    - **token_id**: Associated token (optional)
    - **state_before**: State before action
    - **state_after**: State after action
    - **action**: Action taken
    - **reward**: Reward value
    - **metadata**: Additional metadata
    """
    try:
        exp_repo = RepositoryFactory.create_experience_repository(session)
        
        import time
        timestamp = int(time.time() * 1000)
        
        event = await exp_repo.create_event(
            event_type=request.event_type,
            timestamp=timestamp,
            token_id=request.token_id,
            state_before=request.state_before,
            state_after=request.state_after,
            action=request.action,
            reward=request.reward,
            metadata=request.metadata
        )
        
        return ExperienceEventResponse.model_validate(event)
    
    except Exception as e:
        logger.error(f"Error creating experience event: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@experience_router.get("/events", response_model=ExperienceListResponse)
async def list_experience_events(
    limit: int = Query(10, ge=1, le=100),
    offset: int = Query(0, ge=0),
    event_type: str = Query(None),
    session: AsyncSession = Depends(get_db_session)
):
    """
    List experience events.
    
    - **limit**: Maximum number to return (1-100)
    - **offset**: Number to skip
    - **event_type**: Filter by event type
    """
    try:
        exp_repo = RepositoryFactory.create_experience_repository(session)
        
        events = await exp_repo.get_recent_events(
            count=limit,
            event_type=event_type
        )
        
        # Apply offset
        events = events[offset:]
        
        total = await exp_repo.count()
        
        return ExperienceListResponse(
            events=[ExperienceEventResponse.model_validate(e) for e in events],
            total=total,
            limit=limit,
            offset=offset
        )
    
    except Exception as e:
        logger.error(f"Error listing experience events: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@experience_router.get("/events/{event_id}", response_model=ExperienceEventResponse)
async def get_experience_event(
    event_id: UUID,
    session: AsyncSession = Depends(get_db_session)
):
    """
    Get an experience event by ID.
    """
    try:
        exp_repo = RepositoryFactory.create_experience_repository(session)
        event = await exp_repo.get_by_id(event_id)
        
        if not event:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Event {event_id} not found"
            )
        
        return ExperienceEventResponse.model_validate(event)
    
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error getting experience event: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@experience_router.get("/events/token/{token_id}", response_model=ExperienceListResponse)
async def get_token_events(
    token_id: UUID,
    limit: int = Query(10, ge=1, le=100),
    session: AsyncSession = Depends(get_db_session)
):
    """
    Get experience events for a specific token.
    """
    try:
        exp_repo = RepositoryFactory.create_experience_repository(session)
        
        events = await exp_repo.get_events_by_token(token_id, limit=limit)
        
        return ExperienceListResponse(
            events=[ExperienceEventResponse.model_validate(e) for e in events],
            total=len(events),
            limit=limit,
            offset=0
        )
    
    except Exception as e:
        logger.error(f"Error getting token events: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )


@experience_router.delete("/events/cleanup")
async def cleanup_old_events(
    retention_days: int = Query(30, ge=1, le=365),
    session: AsyncSession = Depends(get_db_session)
):
    """
    Clean up old experience events.
    
    - **retention_days**: Keep events from last N days
    """
    try:
        exp_repo = RepositoryFactory.create_experience_repository(session)
        
        deleted_count = await exp_repo.cleanup_old_events(retention_days)
        
        return {
            "deleted": deleted_count,
            "retention_days": retention_days
        }
    
    except Exception as e:
        logger.error(f"Error cleaning up events: {e}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )