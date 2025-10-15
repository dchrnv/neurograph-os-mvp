"""
Token management CLI commands.
"""

import asyncio
import time
from uuid import UUID, uuid4

import click
from rich.console import Console
from rich.table import Table
from rich.progress import Progress

from src.infrastructure.config import ConfigLoader
from src.infrastructure.persistence.database import DatabaseFactory
from src.infrastructure.persistence.repositories import RepositoryFactory
from src.infrastructure.persistence.models import TokenModel

console = Console()


@click.group(name='token')
def token_group():
    """Manage tokens."""
    pass


@token_group.command(name='create')
@click.option('--type', '-t', default='default', help='Token type')
@click.option('--x', type=float, multiple=True, help='X coordinates (8 levels)')
@click.option('--y', type=float, multiple=True, help='Y coordinates (8 levels)')
@click.option('--z', type=float, multiple=True, help='Z coordinates (8 levels)')
@click.option('--weight', '-w', type=float, default=1.0, help='Token weight')
@click.option('--flags', type=int, default=0, help='Token flags')
@click.pass_context
def create_token(ctx, type, x, y, z, weight, flags):
    """Create a new token."""
    
    async def _create():
        # Load config
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        
        # Initialize DB
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        try:
            async with db_manager.session() as session:
                token_repo = RepositoryFactory.create_token_repository(session)
                
                # Prepare coordinates (default to zeros if not provided)
                coord_x = list(x) if x else [0.0] * 8
                coord_y = list(y) if y else [0.0] * 8
                coord_z = list(z) if z else [0.0] * 8
                
                # Pad or truncate to 8 levels
                coord_x = (coord_x + [0.0] * 8)[:8]
                coord_y = (coord_y + [0.0] * 8)[:8]
                coord_z = (coord_z + [0.0] * 8)[:8]
                
                # Create token
                token = TokenModel(
                    id=uuid4(),
                    binary_data=b'\x00' * 64,
                    coord_x=coord_x,
                    coord_y=coord_y,
                    coord_z=coord_z,
                    flags=flags,
                    weight=weight,
                    timestamp=int(time.time() * 1000),
                    token_type=type,
                    metadata={}
                )
                
                token = await token_repo.create(token)
                
                console.print(f"\n[green]✓[/green] Token created successfully!")
                console.print(f"[dim]ID:[/dim] {token.id}")
                console.print(f"[dim]Type:[/dim] {token.token_type}")
                console.print(f"[dim]Weight:[/dim] {token.weight}")
                console.print(f"[dim]Coordinates:[/dim] ({coord_x[0]:.2f}, {coord_y[0]:.2f}, {coord_z[0]:.2f})")
        
        finally:
            await DatabaseFactory.close_all()
    
    asyncio.run(_create())


@token_group.command(name='list')
@click.option('--limit', '-l', type=int, default=10, help='Number of tokens to show')
@click.option('--type', '-t', help='Filter by token type')
@click.option('--format', '-f', type=click.Choice(['table', 'json']), default='table', help='Output format')
@click.pass_context
def list_tokens(ctx, limit, type, format):
    """List tokens."""
    
    async def _list():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        try:
            async with db_manager.session() as session:
                token_repo = RepositoryFactory.create_token_repository(session)
                
                # Get tokens
                if type:
                    tokens = await token_repo.find_by_type(type)
                    tokens = tokens[:limit]
                else:
                    tokens = await token_repo.get_all(limit=limit)
                
                if not tokens:
                    console.print("[yellow]No tokens found[/yellow]")
                    return
                
                if format == 'table':
                    table = Table(title=f"Tokens (showing {len(tokens)})")
                    table.add_column("ID", style="cyan")
                    table.add_column("Type", style="green")
                    table.add_column("Coordinates", style="yellow")
                    table.add_column("Weight", justify="right")
                    table.add_column("Flags", justify="right")
                    table.add_column("Created", style="dim")
                    
                    for token in tokens:
                        coords = f"({token.coord_x[0]:.2f}, {token.coord_y[0]:.2f}, {token.coord_z[0]:.2f})"
                        table.add_row(
                            str(token.id)[:8] + "...",
                            token.token_type,
                            coords,
                            f"{token.weight:.2f}",
                            str(token.flags),
                            token.created_at.strftime("%Y-%m-%d %H:%M")
                        )
                    
                    console.print(table)
                
                elif format == 'json':
                    import json
                    data = [
                        {
                            'id': str(token.id),
                            'type': token.token_type,
                            'coordinates': {
                                'x': token.coord_x,
                                'y': token.coord_y,
                                'z': token.coord_z
                            },
                            'weight': token.weight,
                            'flags': token.flags,
                            'created_at': token.created_at.isoformat()
                        }
                        for token in tokens
                    ]
                    console.print(json.dumps(data, indent=2))
        
        finally:
            await DatabaseFactory.close_all()
    
    asyncio.run(_list())


@token_group.command(name='get')
@click.argument('token_id', type=str)
@click.option('--verbose', '-v', is_flag=True, help='Show detailed information')
@click.pass_context
def get_token(ctx, token_id, verbose):
    """Get token by ID."""
    
    async def _get():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        try:
            async with db_manager.session() as session:
                token_repo = RepositoryFactory.create_token_repository(session)
                
                try:
                    token_uuid = UUID(token_id)
                except ValueError:
                    console.print(f"[red]Invalid UUID: {token_id}[/red]")
                    return
                
                token = await token_repo.get_by_id(token_uuid)
                
                if not token:
                    console.print(f"[yellow]Token not found: {token_id}[/yellow]")
                    return
                
                console.print(f"\n[bold cyan]Token: {token.id}[/bold cyan]")
                console.print(f"Type: {token.token_type}")
                console.print(f"Weight: {token.weight}")
                console.print(f"Flags: {token.flags} (0b{token.flags:b})")
                console.print(f"Timestamp: {token.timestamp}")
                console.print(f"Created: {token.created_at}")
                console.print(f"Updated: {token.updated_at}")
                
                if verbose:
                    console.print("\n[bold]Coordinates:[/bold]")
                    console.print(f"  X: {token.coord_x}")
                    console.print(f"  Y: {token.coord_y}")
                    console.print(f"  Z: {token.coord_z}")
                    
                    if token.metadata:
                        console.print("\n[bold]Metadata:[/bold]")
                        import json
                        console.print(json.dumps(token.metadata, indent=2))
        
        finally:
            await DatabaseFactory.close_all()
    
    asyncio.run(_get())


@token_group.command(name='delete')
@click.argument('token_id', type=str)
@click.option('--force', '-f', is_flag=True, help='Skip confirmation')
@click.pass_context
def delete_token(ctx, token_id, force):
    """Delete token by ID."""
    
    if not force:
        if not click.confirm(f"Delete token {token_id}?"):
            console.print("[yellow]Cancelled[/yellow]")
            return
    
    async def _delete():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        try:
            async with db_manager.session() as session:
                token_repo = RepositoryFactory.create_token_repository(session)
                
                try:
                    token_uuid = UUID(token_id)
                except ValueError:
                    console.print(f"[red]Invalid UUID: {token_id}[/red]")
                    return
                
                success = await token_repo.delete(token_uuid)
                
                if success:
                    console.print(f"[green]✓[/green] Token deleted: {token_id}")
                else:
                    console.print(f"[yellow]Token not found: {token_id}[/yellow]")
        
        finally:
            await DatabaseFactory.close_all()
    
    asyncio.run(_delete())


@token_group.command(name='search')
@click.option('--region', '-r', type=float, nargs=6, help='Region: minX minY minZ maxX maxY maxZ')
@click.option('--level', '-l', type=int, default=0, help='Coordinate level (0-7)')
@click.pass_context
def search_tokens(ctx, region, level):
    """Search tokens in spatial region."""
    
    if not region:
        console.print("[red]Error: --region is required[/red]")
        console.print("Example: --region 0 0 0 10 10 10")
        return
    
    async def _search():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        try:
            async with db_manager.session() as session:
                token_repo = RepositoryFactory.create_token_repository(session)
                
                min_coords = tuple(region[:3])
                max_coords = tuple(region[3:])
                
                console.print(f"Searching region: {min_coords} to {max_coords} (level {level})...")
                
                tokens = await token_repo.find_in_region(min_coords, max_coords, level)
                
                console.print(f"\n[green]Found {len(tokens)} token(s)[/green]\n")
                
                if tokens:
                    table = Table()
                    table.add_column("ID", style="cyan")
                    table.add_column("Type")
                    table.add_column("Position")
                    
                    for token in tokens:
                        coords = f"({token.coord_x[level]:.2f}, {token.coord_y[level]:.2f}, {token.coord_z[level]:.2f})"
                        table.add_row(
                            str(token.id)[:8] + "...",
                            token.token_type,
                            coords
                        )
                    
                    console.print(table)
        
        finally:
            await DatabaseFactory.close_all()
    
    asyncio.run(_search())


@token_group.command(name='count')
@click.option('--type', '-t', help='Filter by type')
@click.pass_context
def count_tokens(ctx, type):
    """Count tokens."""
    
    async def _count():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        try:
            async with db_manager.session() as session:
                token_repo = RepositoryFactory.create_token_repository(session)
                
                if type:
                    tokens = await token_repo.find_by_type(type)
                    count = len(tokens)
                    console.print(f"Tokens of type '{type}': [bold]{count}[/bold]")
                else:
                    count = await token_repo.count()
                    console.print(f"Total tokens: [bold]{count}[/bold]")
        
        finally:
            await DatabaseFactory.close_all()
    
    asyncio.run(_count())