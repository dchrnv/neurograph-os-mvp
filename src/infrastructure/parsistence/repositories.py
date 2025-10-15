"""
Repository pattern implementation for NeuroGraph OS entities.
Provides CRUD operations with caching support.
"""

from typing import List, Optional, Dict, Any, Tuple
from uuid import UUID
from datetime import datetime
import json

from sqlalchemy import select, update, delete, and_, or_, func
from sqlalchemy.ext.asyncio import AsyncSession

from .models import (
    TokenModel, ConnectionModel, ExperienceEventModel,
    ExperienceTrajectoryModel, SpatialIndexModel
)
from .database import RedisManager


class BaseRepository:
    """Base repository with common CRUD operations."""
    
    def __init__(self, session: AsyncSession, redis: Optional[RedisManager] = None):
        """
        Initialize repository.
        
        Args:
            session: SQLAlchemy async session
            redis: Optional Redis manager for caching
        """
        self.session = session
        self.redis = redis
        self.model = None  # Set in subclass
        
    async def get_by_id(self, id: UUID) -> Optional[Any]:
        """Get entity by ID."""
        result = await self.session.execute(
            select(self.model).where(self.model.id == id)
        )
        return result.scalar_one_or_none()
    
    async def get_all(self, limit: int = 100, offset: int = 0) -> List[Any]:
        """Get all entities with pagination."""
        result = await self.session.execute(
            select(self.model).limit(limit).offset(offset)
        )
        return result.scalars().all()
    
    async def create(self, entity: Any) -> Any:
        """Create new entity."""
        self.session.add(entity)
        await self.session.flush()
        return entity
    
    async def update(self, entity: Any) -> Any:
        """Update existing entity."""
        await self.session.merge(entity)
        await self.session.flush()
        return entity
    
    async def delete(self, id: UUID) -> bool:
        """Delete entity by ID."""
        result = await self.session.execute(
            delete(self.model).where(self.model.id == id)
        )
        return result.rowcount > 0
    
    async def count(self) -> int:
        """Count total entities."""
        result = await self.session.execute(
            select(func.count()).select_from(self.model)
        )
        return result.scalar()


class TokenRepository(BaseRepository):
    """Repository for Token entities with spatial queries."""
    
    def __init__(self, session: AsyncSession, redis: Optional[RedisManager] = None, config: dict = None):
        super().__init__(session, redis)
        self.model = TokenModel
        self.config = config or {}
        self.cache_enabled = self.config.get('cache_enabled', True)
        self.cache_ttl = self.config.get('cache_ttl', 3600)
    
    async def get_by_id(self, id: UUID, use_cache: bool = True) -> Optional[TokenModel]:
        """Get token by ID with caching."""
        if use_cache and self.redis and self.cache_enabled:
            # Try cache first
            cache_key = self.redis.build_key('token_by_id', token_id=str(id))
            cached = self.redis.get_client().get(cache_key)
            
            if cached:
                # Deserialize from cache
                return self._deserialize_token(json.loads(cached))
        
        # Fetch from database
        token = await super().get_by_id(id)
        
        # Cache result
        if token and self.redis and self.cache_enabled:
            cache_key = self.redis.build_key('token_by_id', token_id=str(id))
            self.redis.get_client().setex(
                cache_key,
                self.cache_ttl,
                json.dumps(self._serialize_token(token))
            )
        
        return token
    
    async def find_by_coordinates(
        self, 
        x: List[float], 
        y: List[float], 
        z: List[float],
        tolerance: float = 0.01
    ) -> List[TokenModel]:
        """
        Find tokens by approximate coordinates.
        
        Args:
            x, y, z: Coordinate arrays (8 levels)
            tolerance: Search tolerance
        """
        # Build coordinate search conditions
        conditions = []
        for i in range(min(len(x), 8)):
            conditions.append(
                and_(
                    func.abs(TokenModel.coord_x[i] - x[i]) < tolerance,
                    func.abs(TokenModel.coord_y[i] - y[i]) < tolerance,
                    func.abs(TokenModel.coord_z[i] - z[i]) < tolerance
                )
            )
        
        result = await self.session.execute(
            select(TokenModel).where(or_(*conditions))
        )
        return result.scalars().all()
    
    async def find_in_region(
        self,
        min_coords: Tuple[float, float, float],
        max_coords: Tuple[float, float, float],
        level: int = 0
    ) -> List[TokenModel]:
        """
        Find tokens within a spatial region.
        
        Args:
            min_coords: Minimum (x, y, z) coordinates
            max_coords: Maximum (x, y, z) coordinates
            level: Coordinate level to search (0-7)
        """
        min_x, min_y, min_z = min_coords
        max_x, max_y, max_z = max_coords
        
        result = await self.session.execute(
            select(TokenModel).where(
                and_(
                    TokenModel.coord_x[level] >= min_x,
                    TokenModel.coord_x[level] <= max_x,
                    TokenModel.coord_y[level] >= min_y,
                    TokenModel.coord_y[level] <= max_y,
                    TokenModel.coord_z[level] >= min_z,
                    TokenModel.coord_z[level] <= max_z
                )
            )
        )
        return result.scalars().all()
    
    async def find_by_type(self, token_type: str) -> List[TokenModel]:
        """Find tokens by type."""
        result = await self.session.execute(
            select(TokenModel).where(TokenModel.token_type == token_type)
        )
        return result.scalars().all()
    
    async def find_by_flags(self, flags: int, mask: Optional[int] = None) -> List[TokenModel]:
        """
        Find tokens by flags.
        
        Args:
            flags: Flag values to match