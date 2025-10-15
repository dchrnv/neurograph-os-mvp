"""
Graph management CLI commands.
"""

import asyncio
from uuid import UUID

import click
from rich.console import Console
from rich.table import Table
from rich.tree import Tree

from src.infrastructure.config import ConfigLoader
from src.infrastructure.persistence.database import DatabaseFactory
from src.infrastructure.persistence.repositories import RepositoryFactory

console = Console()


@click.group(name='graph')
def graph_group():
    """Manage graph connections."""
    pass


@graph_group.command(name='connect')
@click.argument('source_id', type=str)
@click.argument('target_id', type=str)
@click.option('--type', '-t', default='generic', help='Connection type')
@click.option('--weight', '-w', type=float, default=1.0, help='Connection weight')
@click.option('--bidirectional', '-b', is_flag=True, help='Create bidirectional connection')
@click.pass_context
def create_connection(ctx, source_id, target_id, type, weight, bidirectional):
    """Create connection between two tokens."""
    
    async def _connect():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        try:
            async with db_manager.session() as session:
                graph_repo = RepositoryFactory.create_graph_repository(session)
                
                try:
                    source_uuid = UUID(source_id)
                    target_uuid = UUID(target_id)
                except ValueError as e:
                    console.print(f"[red]Invalid UUID: {e}[/red]")
                    return
                
                connection = await graph_repo.create_connection(
                    source_id=source_uuid,
                    target_id=target_uuid,
                    connection_type=type,
                    weight=weight,
                    bidirectional=bidirectional
                )
                
                console.print(f"\n[green]✓[/green] Connection created!")
                console.print(f"[dim]ID:[/dim] {connection.id}")
                console.print(f"[dim]Source:[/dim] {source_id}")
                console.print(f"[dim]Target:[/dim] {target_id}")
                console.print(f"[dim]Type:[/dim] {type}")
                console.print(f"[dim]Weight:[/dim] {weight}")
                console.print(f"[dim]Bidirectional:[/dim] {bidirectional}")
        
        finally:
            await DatabaseFactory.close_all()
    
    asyncio.run(_connect())


@graph_group.command(name='neighbors')
@click.argument('token_id', type=str)
@click.option('--direction', '-d', type=click.Choice(['incoming', 'outgoing', 'both']), default='both')
@click.option('--type', '-t', help='Filter by connection type')
@click.pass_context
def get_neighbors(ctx, token_id, direction, type):
    """Get token neighbors."""
    
    async def _neighbors():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        try:
            async with db_manager.session() as session:
                graph_repo = RepositoryFactory.create_graph_repository(session)
                
                try:
                    token_uuid = UUID(token_id)
                except ValueError:
                    console.print(f"[red]Invalid UUID: {token_id}[/red]")
                    return
                
                neighbors = await graph_repo.get_neighbors(
                    token_uuid, 
                    direction=direction,
                    connection_type=type
                )
                
                if not neighbors:
                    console.print(f"[yellow]No neighbors found[/yellow]")
                    return
                
                console.print(f"\n[bold cyan]Neighbors of {token_id[:8]}...[/bold cyan]")
                console.print(f"Direction: {direction}\n")
                
                table = Table()
                table.add_column("Connection ID", style="dim")
                table.add_column("Direction", style="yellow")
                table.add_column("Neighbor", style="cyan")
                table.add_column("Type", style="green")
                table.add_column("Weight", justify="right")
                
                for conn in neighbors:
                    if str(conn.source_id) == token_id:
                        dir_arrow = "→"
                        neighbor_id = str(conn.target_id)
                    else:
                        dir_arrow = "←"
                        neighbor_id = str(conn.source_id)
                    
                    table.add_row(
                        str(conn.id)[:8] + "...",
                        dir_arrow,
                        neighbor_id[:8] + "...",
                        conn.connection_type,
                        f"{conn.weight:.2f}"
                    )
                
                console.print(table)
                console.print(f"\n[dim]Total: {len(neighbors)} connection(s)[/dim]")
        
        finally:
            await DatabaseFactory.close_all()
    
    asyncio.run(_neighbors())


@graph_group.command(name='degree')
@click.argument('token_id', type=str)
@click.pass_context
def get_degree(ctx, token_id):
    """Get token degree (connectivity)."""
    
    async def _degree():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        try:
            async with db_manager.session() as session:
                graph_repo = RepositoryFactory.create_graph_repository(session)
                
                try:
                    token_uuid = UUID(token_id)
                except ValueError:
                    console.print(f"[red]Invalid UUID: {token_id}[/red]")
                    return
                
                degree = await graph_repo.get_degree(token_uuid)
                
                console.print(f"\n[bold cyan]Token: {token_id[:8]}...[/bold cyan]")
                console.print(f"In-degree:  [green]{degree['in_degree']}[/green] (incoming connections)")
                console.print(f"Out-degree: [yellow]{degree['out_degree']}[/yellow] (outgoing connections)")
                console.print(f"Total:      [bold]{degree['total_degree']}[/bold]")
        
        finally:
            await DatabaseFactory.close_all()
    
    asyncio.run(_degree())


@graph_group.command(name='path')
@click.argument('source_id', type=str)
@click.argument('target_id', type=str)
@click.option('--max-depth', '-d', type=int, default=5, help='Maximum path depth')
@click.pass_context
def find_path(ctx, source_id, target_id, max_depth):
    """Find paths between two tokens."""
    
    async def _path():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        try:
            async with db_manager.session() as session:
                graph_repo = RepositoryFactory.create_graph_repository(session)
                
                try:
                    source_uuid = UUID(source_id)
                    target_uuid = UUID(target_id)
                except ValueError as e:
                    console.print(f"[red]Invalid UUID: {e}[/red]")
                    return
                
                console.print(f"Finding paths from {source_id[:8]}... to {target_id[:8]}...")
                console.print(f"Max depth: {max_depth}\n")
                
                paths = await graph_repo.find_path(source_uuid, target_uuid, max_depth)
                
                if not paths:
                    console.print("[yellow]No paths found[/yellow]")
                    return
                
                console.print(f"[green]Found {len(paths)} path(s)[/green]\n")
                
                for i, path in enumerate(paths, 1):
                    console.print(f"[bold]Path {i}[/bold] (length: {len(path) - 1}):")
                    path_str = " → ".join([str(node_id)[:8] + "..." for node_id in path])
                    console.print(f"  {path_str}\n")
        
        finally:
            await DatabaseFactory.close_all()
    
    asyncio.run(_path())


@graph_group.command(name='stats')
@click.pass_context
def graph_stats(ctx):
    """Show graph statistics."""
    
    async def _stats():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        try:
            async with db_manager.session() as session:
                graph_repo = RepositoryFactory.create_graph_repository(session)
                token_repo = RepositoryFactory.create_token_repository(session)
                
                # Get counts
                total_tokens = await token_repo.count()
                total_connections = await graph_repo.count()
                
                console.print("\n[bold cyan]Graph Statistics[/bold cyan]\n")
                console.print(f"Nodes (tokens):     [bold]{total_tokens}[/bold]")
                console.print(f"Edges (connections): [bold]{total_connections}[/bold]")
                
                if total_tokens > 0:
                    avg_degree = (total_connections * 2) / total_tokens
                    console.print(f"Avg degree:         [bold]{avg_degree:.2f}[/bold]")
                    
                    density = (total_connections / (total_tokens * (total_tokens - 1))) if total_tokens > 1 else 0
                    console.print(f"Graph density:      [bold]{density:.4f}[/bold]")
        
        finally:
            await DatabaseFactory.close_all()
    
    asyncio.run(_stats())


@graph_group.command(name='visualize')
@click.argument('token_id', type=str)
@click.option('--depth', '-d', type=int, default=2, help='Visualization depth')
@click.pass_context
def visualize_graph(ctx, token_id, depth):
    """Visualize token neighborhood as a tree."""
    
    async def _visualize():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        try:
            async with db_manager.session() as session:
                graph_repo = RepositoryFactory.create_graph_repository(session)
                
                try:
                    token_uuid = UUID(token_id)
                except ValueError:
                    console.print(f"[red]Invalid UUID: {token_id}[/red]")
                    return
                
                # Build tree
                tree = Tree(f"[bold cyan]Token: {token_id[:8]}...[/bold cyan]")
                visited = set()
                
                async def build_tree(node_id, parent_tree, current_depth):
                    if current_depth >= depth or node_id in visited:
                        return
                    
                    visited.add(node_id)
                    neighbors = await graph_repo.get_neighbors(node_id, direction='outgoing')
                    
                    for conn in neighbors:
                        neighbor_id = conn.target_id
                        label = f"{str(neighbor_id)[:8]}... [{conn.connection_type}] w={conn.weight:.2f}"
                        
                        if neighbor_id not in visited:
                            branch = parent_tree.add(label)
                            await build_tree(neighbor_id, branch, current_depth + 1)
                        else:
                            parent_tree.add(f"[dim]{label} (visited)[/dim]")
                
                await build_tree(token_uuid, tree, 0)
                
                console.print("\n")
                console.print(tree)
                console.print(f"\n[dim]Showing up to depth {depth}[/dim]")
        
        finally:
            await DatabaseFactory.close_all()
    
    asyncio.run(_visualize())