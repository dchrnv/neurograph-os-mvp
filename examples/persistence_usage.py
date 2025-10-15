"""
Example usage of NeuroGraph OS persistence layer.
Demonstrates database operations with PostgreSQL and Redis.
"""

import asyncio
from uuid import uuid4
from datetime import datetime
import time

from src.infrastructure.config import ConfigLoader
from src.infrastructure.persistence.database import DatabaseFactory
from src.infrastructure.persistence.repositories import RepositoryFactory
from src.infrastructure.persistence.models import TokenModel, ConnectionModel


async def main():
    """Main example function."""
    
    # 1. Load configuration
    print("📋 Loading configuration...")
    config_loader = ConfigLoader()
    db_config = config_loader.load('infrastructure/database.yaml')
    
    # 2. Initialize database managers
    print("🗄️  Initializing database connections...")
    db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
    redis_manager = DatabaseFactory.create_redis_manager(db_config['database'])
    
    # 3. Check health
    print("🏥 Checking database health...")
    pg_health = await db_manager.health_check()
    redis_health = redis_manager.health_check()
    print(f"   PostgreSQL: {'✅' if pg_health else '❌'}")
    print(f"   Redis: {'✅' if redis_health else '❌'}")
    
    if not (pg_health and redis_health):
        print("❌ Database health check failed!")
        return
    
    # 4. Create tables (development only!)
    print("🏗️  Creating database tables...")
    await db_manager.create_tables()
    
    # 5. Work with repositories
    async with db_manager.session() as session:
        
        # Create repositories
        token_repo = RepositoryFactory.create_token_repository(
            session, 
            redis_manager,
            db_config['repositories']['token']
        )
        graph_repo = RepositoryFactory.create_graph_repository(
            session,
            redis_manager,
            db_config['repositories']['graph']
        )
        
        print("\n📝 Creating tokens...")
        
        # Create sample tokens
        token1 = TokenModel(
            id=uuid4(),
            binary_data=b'\x00' * 64,
            coord_x=[1.0, 0.5, 0.25, 0.125, 0.0625, 0.03125, 0.015625, 0.0078125],
            coord_y=[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            coord_z=[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            flags=0b1010,
            weight=1.0,
            timestamp=int(time.time() * 1000),
            token_type='test',
            metadata={'description': 'Test token 1'}
        )
        
        token2 = TokenModel(
            id=uuid4(),
            binary_data=b'\x00' * 64,
            coord_x=[2.0, 1.0, 0.5, 0.25, 0.125, 0.0625, 0.03125, 0.015625],
            coord_y=[1.0, 0.5, 0.25, 0.125, 0.0625, 0.03125, 0.015625, 0.0078125],
            coord_z=[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            flags=0b1100,
            weight=2.0,
            timestamp=int(time.time() * 1000),
            token_type='test',
            metadata={'description': 'Test token 2'}
        )
        
        # Save tokens
        token1 = await token_repo.create(token1)
        token2 = await token_repo.create(token2)
        print(f"   ✅ Created token 1: {token1.id}")
        print(f"   ✅ Created token 2: {token2.id}")
        
        # Create connection
        print("\n🔗 Creating graph connection...")
        connection = await graph_repo.create_connection(
            source_id=token1.id,
            target_id=token2.id,
            connection_type='spatial',
            weight=0.8,
            metadata={'distance': 1.414}
        )
        print(f"   ✅ Created connection: {connection.id}")
        
        # Retrieve token (will use cache on second call)
        print("\n🔍 Retrieving token...")
        retrieved = await token_repo.get_by_id(token1.id, use_cache=True)
        print(f"   ✅ Retrieved: {retrieved.id} (type: {retrieved.token_type})")
        
        # Get neighbors
        print("\n👥 Finding neighbors...")
        neighbors = await graph_repo.get_neighbors(token1.id, direction='outgoing')
        print(f"   ✅ Found {len(neighbors)} neighbor(s)")
        for conn in neighbors:
            print(f"      → {conn.target_id} (type: {conn.connection_type}, weight: {conn.weight})")
        
        # Spatial query
        print("\n🗺️  Spatial query...")
        nearby_tokens = await token_repo.find_in_region(
            min_coords=(0.0, 0.0, 0.0),
            max_coords=(3.0, 3.0, 3.0),
            level=0
        )
        print(f"   ✅ Found {len(nearby_tokens)} token(s) in region")
        
        # Get statistics
        print("\n📊 Graph statistics...")
        degree = await graph_repo.get_degree(token1.id)
        print(f"   In-degree: {degree['in_degree']}")
        print(f"   Out-degree: {degree['out_degree']}")
        print(f"   Total degree: {degree['total_degree']}")
        
        # Count total tokens
        total_tokens = await token_repo.count()
        print(f"\n📈 Total tokens in database: {total_tokens}")
    
    # 6. Test Redis caching
    print("\n💾 Testing Redis cache...")
    cache_key = redis_manager.build_key('token_by_id', token_id=str(token1.id))
    cached_value = redis_manager.get_client().get(cache_key)
    print(f"   Cache key: {cache_key}")
    print(f"   Cached: {'✅ Yes' if cached_value else '❌ No'}")
    
    # 7. Cleanup
    print("\n🧹 Cleaning up...")
    await DatabaseFactory.close_all()
    print("   ✅ Connections closed")
    
    print("\n✨ Example completed successfully!")


async def bulk_insert_example():
    """Example of bulk insert operations."""
    
    print("\n" + "="*60)
    print("BULK INSERT EXAMPLE")
    print("="*60)
    
    # Load config
    config_loader = ConfigLoader()
    db_config = config_loader.load('infrastructure/database.yaml')
    
    # Initialize
    db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
    
    async with db_manager.session() as session:
        token_repo = RepositoryFactory.create_token_repository(
            session, 
            config=db_config['repositories']['token']
        )
        
        print(f"📦 Creating 1000 tokens in bulk...")
        start_time = time.time()
        
        # Generate tokens
        tokens = []
        for i in range(1000):
            token = TokenModel(
                id=uuid4(),
                binary_data=b'\x00' * 64,
                coord_x=[float(i)] * 8,
                coord_y=[0.0] * 8,
                coord_z=[0.0] * 8,
                flags=i % 16,
                weight=1.0,
                timestamp=int(time.time() * 1000),
                token_type='bulk_test',
                metadata={'batch': 'example', 'index': i}
            )
            tokens.append(token)
        
        # Bulk insert
        await token_repo.bulk_create(tokens)
        
        elapsed = time.time() - start_time
        print(f"   ✅ Inserted 1000 tokens in {elapsed:.2f}s")
        print(f"   ⚡ Rate: {1000/elapsed:.0f} tokens/sec")
    
    await DatabaseFactory.close_all()


async def experience_example():
    """Example of working with Experience events."""
    
    print("\n" + "="*60)
    print("EXPERIENCE EVENTS EXAMPLE")
    print("="*60)
    
    # Load config
    config_loader = ConfigLoader()
    db_config = config_loader.load('infrastructure/database.yaml')
    
    # Initialize
    db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
    
    async with db_manager.session() as session:
        exp_repo = RepositoryFactory.create_experience_repository(
            session,
            config=db_config['repositories']['experience']
        )
        
        print("📝 Creating experience events...")
        
        # Create events
        for i in range(5):
            event = await exp_repo.create_event(
                event_type='test_action',
                timestamp=int(time.time() * 1000) + i,
                state_before={'value': i},
                state_after={'value': i + 1},
                action={'type': 'increment'},
                reward=1.0,
                metadata={'step': i}
            )
            print(f"   ✅ Event {i+1}: {event.id}")
        
        # Query recent events
        print("\n🔍 Querying recent events...")
        recent = await exp_repo.get_recent_events(count=3)
        print(f"   Found {len(recent)} recent event(s)")
        for event in recent:
            print(f"      → Type: {event.event_type}, Reward: {event.reward}")
    
    await DatabaseFactory.close_all()


async def migration_example():
    """Example of running database migrations."""
    
    print("\n" + "="*60)
    print("DATABASE MIGRATION EXAMPLE")
    print("="*60)
    
    print("""
To run migrations:

1. Generate migration:
   alembic revision --autogenerate -m "Initial schema"

2. Apply migration:
   alembic upgrade head

3. Rollback migration:
   alembic downgrade -1

4. Show current version:
   alembic current

5. Show migration history:
   alembic history
    """)


def docker_setup_guide():
    """Print Docker setup instructions."""
    
    print("\n" + "="*60)
    print("DOCKER SETUP GUIDE")
    print("="*60)
    
    print("""
1. Start PostgreSQL and Redis:
   docker-compose -f docker-compose.db.yml up -d

2. Check services status:
   docker-compose -f docker-compose.db.yml ps

3. View logs:
   docker-compose -f docker-compose.db.yml logs -f postgres
   docker-compose -f docker-compose.db.yml logs -f redis

4. Stop services:
   docker-compose -f docker-compose.db.yml down

5. Start with management tools (PgAdmin, Redis Commander):
   docker-compose -f docker-compose.db.yml --profile tools up -d

6. Access management UIs:
   - PgAdmin: http://localhost:5050 (admin@neurograph.local / admin)
   - Redis Commander: http://localhost:8081

7. Clean up (removes volumes):
   docker-compose -f docker-compose.db.yml down -v
    """)


def environment_setup_guide():
    """Print environment setup instructions."""
    
    print("\n" + "="*60)
    print("ENVIRONMENT SETUP")
    print("="*60)
    
    print("""
Create .env file in project root:

# PostgreSQL
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_DB=neurograph
POSTGRES_USER=neurograph_user
POSTGRES_PASSWORD=your_secure_password_here

# Redis
REDIS_HOST=localhost
REDIS_PORT=6379
REDIS_DB=0
REDIS_PASSWORD=

# Application
LOG_LEVEL=INFO
ENVIRONMENT=development
    """)


if __name__ == "__main__":
    print("""
╔══════════════════════════════════════════════════════════╗
║        NeuroGraph OS - Persistence Layer Examples        ║
╚══════════════════════════════════════════════════════════╝
    """)
    
    # Print guides
    docker_setup_guide()
    environment_setup_guide()
    
    # Run examples
    print("\n" + "="*60)
    print("RUNNING EXAMPLES")
    print("n"*60)
    
    try:
        # Main CRUD example
        asyncio.run(main())
        
        # Bulk insert example
        asyncio.run(bulk_insert_example())
        
        # Experience events example
        asyncio.run(experience_example())
        
        # Migration info
        asyncio.run(migration_example())
        
    except KeyboardInterrupt:
        print("\n\n⚠️  Interrupted by user")
    except Exception as e:
        print(f"\n\n❌ Error: {e}")
        import traceback
        traceback.print_exc()
    
    print("\n" + "="*60)
    print("Examples completed!")
    print("="*60)