"""
Database management CLI commands.
"""

import asyncio
import subprocess
from datetime import datetime

import click
from rich.console import Console
from rich.panel import Panel

from src.infrastructure.config import ConfigLoader
from src.infrastructure.persistence.database import DatabaseFactory

console = Console()


@click.group(name='db')
def db_group():
    """Database management commands."""
    pass


@db_group.command(name='init')
@click.option('--drop', is_flag=True, help='Drop existing tables first')
@click.pass_context
def init_database(ctx, drop):
    """Initialize database schema."""
    
    if drop:
        if not click.confirm("⚠️  This will DROP all tables. Continue?"):
            console.print("[yellow]Cancelled[/yellow]")
            return
    
    async def _init():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        try:
            console.print("🏗️  Initializing database...")
            
            # Create schemas
            await db_manager.create_schemas()
            console.print("[green]✓[/green] Schemas created")
            
            # Drop tables if requested
            if drop:
                await db_manager.drop_tables()
                console.print("[yellow]✓[/yellow] Tables dropped")
            
            # Create tables
            await db_manager.create_tables()
            console.print("[green]✓[/green] Tables created")
            
            console.print("\n[green]✨ Database initialized successfully![/green]")
            console.print("\n[dim]Note: For production, use Alembic migrations instead[/dim]")
        
        finally:
            await DatabaseFactory.close_all()
    
    asyncio.run(_init())


@db_group.command(name='migrate')
@click.argument('command', type=click.Choice(['upgrade', 'downgrade', 'current', 'history']))
@click.option('--revision', '-r', default='head', help='Target revision (default: head)')
@click.pass_context
def migrate_database(ctx, command, revision):
    """Run database migrations using Alembic."""
    
    alembic_commands = {
        'upgrade': f'alembic upgrade {revision}',
        'downgrade': f'alembic downgrade {revision}',
        'current': 'alembic current',
        'history': 'alembic history --verbose'
    }
    
    cmd = alembic_commands[command]
    console.print(f"Running: [cyan]{cmd}[/cyan]\n")
    
    try:
        result = subprocess.run(cmd, shell=True, check=True, capture_output=False)
        console.print("\n[green]✓[/green] Migration completed successfully")
    except subprocess.CalledProcessError as e:
        console.print(f"\n[red]✗[/red] Migration failed with exit code {e.returncode}")


@db_group.command(name='revision')
@click.option('--message', '-m', required=True, help='Migration message')
@click.option('--autogenerate', '-a', is_flag=True, help='Auto-generate from models')
@click.pass_context
def create_revision(ctx, message, autogenerate):
    """Create a new migration revision."""
    
    cmd = f'alembic revision {"--autogenerate" if autogenerate else ""} -m "{message}"'
    console.print(f"Creating migration: [cyan]{message}[/cyan]\n")
    
    try:
        subprocess.run(cmd, shell=True, check=True)
        console.print("\n[green]✓[/green] Migration file created")
        console.print("[dim]Review the migration file before applying it[/dim]")
    except subprocess.CalledProcessError as e:
        console.print(f"\n[red]✗[/red] Failed to create migration: {e}")


@db_group.command(name='status')
@click.pass_context
def database_status(ctx):
    """Check database connection status."""
    
    async def _status():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        
        # PostgreSQL
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        pg_healthy = await db_manager.health_check()
        
        # Redis
        redis_manager = DatabaseFactory.create_redis_manager(db_config['database'])
        redis_healthy = redis_manager.health_check()
        
        # Display status
        console.print("\n[bold cyan]Database Status[/bold cyan]\n")
        
        pg_status = "[green]✓ Connected[/green]" if pg_healthy else "[red]✗ Disconnected[/red]"
        redis_status = "[green]✓ Connected[/green]" if redis_healthy else "[red]✗ Disconnected[/red]"
        
        pg_config = db_config['database']['postgres']
        redis_config = db_config['database']['redis']
        
        console.print(f"PostgreSQL: {pg_status}")
        console.print(f"  Host: {pg_config['host']}:{pg_config['port']}")
        console.print(f"  Database: {pg_config['database']}")
        console.print(f"  User: {pg_config['user']}")
        
        console.print(f"\nRedis: {redis_status}")
        console.print(f"  Host: {redis_config['host']}:{redis_config['port']}")
        console.print(f"  DB: {redis_config['db']}")
        
        if redis_healthy:
            client = redis_manager.get_client()
            info = client.info('memory')
            console.print(f"  Memory: {info.get('used_memory_human', 'N/A')}")
            console.print(f"  Keys: {client.dbsize()}")
        
        await DatabaseFactory.close_all()
    
    asyncio.run(_status())


@db_group.command(name='backup')
@click.option('--output', '-o', help='Backup file path')
@click.pass_context
def backup_database(ctx, output):
    """Create database backup."""
    
    config_loader = ConfigLoader()
    db_config = config_loader.load('infrastructure/database.yaml')
    pg_config = db_config['database']['postgres']
    
    if not output:
        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
        output = f"neurograph_backup_{timestamp}.sql"
    
    console.print(f"Creating backup: [cyan]{output}[/cyan]")
    
    cmd = (
        f"PGPASSWORD={pg_config['password']} pg_dump "
        f"-h {pg_config['host']} -p {pg_config['port']} "
        f"-U {pg_config['user']} -d {pg_config['database']} "
        f"-F c -f {output}"
    )
    
    try:
        subprocess.run(cmd, shell=True, check=True, capture_output=True)
        console.print(f"[green]✓[/green] Backup created: {output}")
    except subprocess.CalledProcessError as e:
        console.print(f"[red]✗[/red] Backup failed: {e}")


@db_group.command(name='restore')
@click.argument('backup_file', type=click.Path(exists=True))
@click.option('--force', '-f', is_flag=True, help='Skip confirmation')
@click.pass_context
def restore_database(ctx, backup_file, force):
    """Restore database from backup."""
    
    if not force:
        if not click.confirm(f"⚠️  This will restore database from {backup_file}. Continue?"):
            console.print("[yellow]Cancelled[/yellow]")
            return
    
    config_loader = ConfigLoader()
    db_config = config_loader.load('infrastructure/database.yaml')
    pg_config = db_config['database']['postgres']
    
    console.print(f"Restoring from: [cyan]{backup_file}[/cyan]")
    
    cmd = (
        f"PGPASSWORD={pg_config['password']} pg_restore "
        f"-h {pg_config['host']} -p {pg_config['port']} "
        f"-U {pg_config['user']} -d {pg_config['database']} "
        f"-c {backup_file}"
    )
    
    try:
        subprocess.run(cmd, shell=True, check=True)
        console.print("[green]✓[/green] Database restored successfully")
    except subprocess.CalledProcessError as e:
        console.print(f"[red]✗[/red] Restore failed: {e}")


@db_group.command(name='clean')
@click.option('--force', '-f', is_flag=True, help='Skip confirmation')
@click.pass_context
def clean_database(ctx, force):
    """Clean all data from database (keeps schema)."""
    
    if not force:
        if not click.confirm("⚠️  This will DELETE all data. Continue?"):
            console.print("[yellow]Cancelled[/yellow]")
            return
    
    async def _clean():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        try:
            async with db_manager.session() as session:
                console.print("🧹 Cleaning database...")
                
                # Delete in order to respect foreign keys
                await session.execute("TRUNCATE TABLE experience.experience_events CASCADE")
                await session.execute("TRUNCATE TABLE experience.experience_trajectories CASCADE")
                await session.execute("TRUNCATE TABLE graph.connections CASCADE")
                await session.execute("TRUNCATE TABLE graph.graph_snapshots CASCADE")
                await session.execute("TRUNCATE TABLE tokens.tokens CASCADE")
                await session.execute("TRUNCATE TABLE tokens.spatial_index CASCADE")
                
                console.print("[green]✓[/green] All data deleted")
        
        finally:
            await DatabaseFactory.close_all()
    
    asyncio.run(_clean())


@db_group.command(name='reset')
@click.option('--force', '-f', is_flag=True, help='Skip confirmation')
@click.pass_context
def reset_database(ctx, force):
    """Reset database (drop and recreate)."""
    
    if not force:
        if not click.confirm("⚠️  This will DROP and RECREATE the database. Continue?"):
            console.print("[yellow]Cancelled[/yellow]")
            return
    
    console.print("♻️  Resetting database...")
    
    # Invoke init with drop flag
    ctx.invoke(init_database, drop=True)
    
    console.print("\n[green]✓[/green] Database reset complete")
    console.print("[yellow]⚠️[/yellow]  Don't forget to run migrations: [cyan]neurograph db migrate upgrade[/cyan]")