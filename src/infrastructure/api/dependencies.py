"""
FastAPI dependencies for dependency injection.
"""

from typing import AsyncGenerator

from sqlalchemy.ext.asyncio import AsyncSession

from src.infrastructure.config import ConfigLoader
from src.infrastructure.persistence.database import DatabaseFactory


# Global database manager instance
_db_manager = None


async def get_db_manager():
    """Get or create database manager."""
    global _db_manager
    
    if _db_manager is None:
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        _db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
    
    return _db_manager


async def get_db_session() -> AsyncGenerator[AsyncSession, None]:
    """
    Dependency for getting a database session.
    
    Usage:
        @router.get("/")
        async def endpoint(session: AsyncSession = Depends(get_db_session)):
            # Use session
    """
    db_manager = await get_db_manager()
    
    async with db_manager.session() as session:
        yield session