"""
Database connection manager for PostgreSQL and Redis.
Handles connection pooling, session management, and health checks.
"""

import redis
from typing import Optional, AsyncGenerator
from contextlib import asynccontextmanager
from sqlalchemy import create_engine, event, pool
from sqlalchemy.ext.asyncio import (
    create_async_engine, 
    AsyncSession, 
    async_sessionmaker,
    AsyncEngine
)
from sqlalchemy.orm import Session, sessionmaker
from sqlalchemy.pool import NullPool, QueuePool

from .models import Base


class DatabaseManager:
    """Manages PostgreSQL connections and sessions."""
    
    def __init__(self, config: dict):
        """
        Initialize database manager.
        
        Args:
            config: Database configuration dict from YAML
        """
        self.config = config
        self.engine: Optional[AsyncEngine] = None
        self.session_factory: Optional[async_sessionmaker] = None
        self._sync_engine = None
        
    def build_connection_string(self, async_driver: bool = True) -> str:
        """Build PostgreSQL connection string."""
        pg_config = self.config['postgres']
        driver = "postgresql+asyncpg" if async_driver else "postgresql+psycopg2"
        
        return (
            f"{driver}://{pg_config['user']}:{pg_config['password']}"
            f"@{pg_config['host']}:{pg_config['port']}/{pg_config['database']}"
        )
    
    async def initialize(self):
        """Initialize database connection pool."""
        pool_config = self.config['postgres']['pool']
        
        # Async engine for application use
        self.engine = create_async_engine(
            self.build_connection_string(async_driver=True),
            pool_size=pool_config['min_size'],
            max_overflow=pool_config['max_overflow'],
            pool_timeout=pool_config['pool_timeout'],
            pool_recycle=pool_config['pool_recycle'],
            echo=pool_config['echo'],
            echo_pool=pool_config['echo_pool'],
            poolclass=QueuePool
        )
        
        # Sync engine for migrations
        self._sync_engine = create_engine(
            self.build_connection_string(async_driver=False),
            pool_size=pool_config['min_size'],
            max_overflow=pool_config['max_overflow'],
            pool_timeout=pool_config['pool_timeout'],
            pool_recycle=pool_config['pool_recycle'],
            echo=pool_config['echo']
        )
        
        # Session factory
        self.session_factory = async_sessionmaker(
            self.engine,
            class_=AsyncSession,
            expire_on_commit=False,
            autocommit=False,
            autoflush=False
        )
        
        # Register event listeners
        self._register_events()
        
    def _register_events(self):
        """Register SQLAlchemy event listeners."""
        
        @event.listens_for(self.engine.sync_engine, "connect")
        def receive_connect(dbapi_conn, connection_record):
            """Set up connection parameters on connect."""
            # Set timezone to UTC
            cursor = dbapi_conn.cursor()
            cursor.execute("SET TIME ZONE 'UTC'")
            cursor.close()
    
    async def create_schemas(self):
        """Create database schemas if they don't exist."""
        schemas = self.config['postgres']['schema']
        
        async with self.engine.begin() as conn:
            for schema_name in schemas.values():
                await conn.execute(f"CREATE SCHEMA IF NOT EXISTS {schema_name}")
    
    async def create_tables(self):
        """Create all tables (use for development only)."""
        async with self.engine.begin() as conn:
            await conn.run_sync(Base.metadata.create_all)
    
    async def drop_tables(self):
        """Drop all tables (use with caution!)."""
        async with self.engine.begin() as conn:
            await conn.run_sync(Base.metadata.drop_all)
    
    @asynccontextmanager
    async def session(self) -> AsyncGenerator[AsyncSession, None]:
        """
        Context manager for database sessions.
        
        Usage:
            async with db_manager.session() as session:
                # Use session
                pass
        """
        if not self.session_factory:
            raise RuntimeError("Database not initialized. Call initialize() first.")
        
        session = self.session_factory()
        try:
            yield session
            await session.commit()
        except Exception:
            await session.rollback()
            raise
        finally:
            await session.close()
    
    async def health_check(self) -> bool:
        """Check database connectivity."""
        try:
            async with self.session() as session:
                await session.execute("SELECT 1")
            return True
        except Exception:
            return False
    
    async def close(self):
        """Close all database connections."""
        if self.engine:
            await self.engine.dispose()
        if self._sync_engine:
            self._sync_engine.dispose()
    
    def get_sync_engine(self):
        """Get synchronous engine (for Alembic migrations)."""
        return self._sync_engine


class RedisManager:
    """Manages Redis connections and caching."""
    
    def __init__(self, config: dict):
        """
        Initialize Redis manager.
        
        Args:
            config: Redis configuration dict from YAML
        """
        self.config = config
        self.client: Optional[redis.Redis] = None
        self.pool: Optional[redis.ConnectionPool] = None
        
    def initialize(self):
        """Initialize Redis connection pool."""
        redis_config = self.config['redis']
        conn_config = redis_config['connection']
        
        # Create connection pool
        self.pool = redis.ConnectionPool(
            host=redis_config['host'],
            port=redis_config['port'],
            db=redis_config['db'],
            password=redis_config['password'] or None,
            max_connections=conn_config['max_connections'],
            socket_timeout=conn_config['socket_timeout'],
            socket_connect_timeout=conn_config['socket_connect_timeout'],
            socket_keepalive=conn_config['socket_keepalive'],
            health_check_interval=conn_config['health_check_interval']
        )
        
        # Create client
        self.client = redis.Redis(
            connection_pool=self.pool,
            decode_responses=True  # Auto-decode bytes to strings
        )
    
    def get_client(self) -> redis.Redis:
        """Get Redis client instance."""
        if not self.client:
            raise RuntimeError("Redis not initialized. Call initialize() first.")
        return self.client
    
    def build_key(self, pattern: str, **kwargs) -> str:
        """
        Build cache key from pattern.
        
        Args:
            pattern: Key pattern from config (e.g., 'token_by_id')
            **kwargs: Values to format into pattern
        
        Returns:
            Formatted cache key
        """
        key_template = self.config['redis']['keys'][pattern]
        return key_template.format(**kwargs)
    
    def get_ttl(self, category: str = 'default') -> int:
        """Get TTL for cache category."""
        cache_config = self.config['redis']['cache']
        return cache_config.get(f'{category}_ttl', cache_config['default_ttl'])
    
    def health_check(self) -> bool:
        """Check Redis connectivity."""
        try:
            return self.client.ping()
        except Exception:
            return False
    
    def close(self):
        """Close Redis connections."""
        if self.client:
            self.client.close()
        if self.pool:
            self.pool.disconnect()


class DatabaseFactory:
    """Factory for creating database managers."""
    
    _db_manager: Optional[DatabaseManager] = None
    _redis_manager: Optional[RedisManager] = None
    
    @classmethod
    async def create_database_manager(cls, config: dict) -> DatabaseManager:
        """Create and initialize database manager (singleton)."""
        if cls._db_manager is None:
            cls._db_manager = DatabaseManager(config)
            await cls._db_manager.initialize()
            await cls._db_manager.create_schemas()
        return cls._db_manager
    
    @classmethod
    def create_redis_manager(cls, config: dict) -> RedisManager:
        """Create and initialize Redis manager (singleton)."""
        if cls._redis_manager is None:
            cls._redis_manager = RedisManager(config)
            cls._redis_manager.initialize()
        return cls._redis_manager
    
    @classmethod
    async def close_all(cls):
        """Close all database connections."""
        if cls._db_manager:
            await cls._db_manager.close()
        if cls._redis_manager:
            cls._redis_manager.close()