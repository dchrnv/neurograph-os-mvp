"""
System monitoring and management CLI commands.
"""

import asyncio
import platform
import psutil
from datetime import datetime

import click
from rich.console import Console
from rich.table import Table
from rich.panel import Panel

from src.infrastructure.config import ConfigLoader
from src.infrastructure.persistence.database import DatabaseFactory
from src.infrastructure.persistence.repositories import RepositoryFactory

console = Console()


@click.group(name='system')
def system_group():
    """System monitoring and management."""
    pass


@system_group.command(name='status')
@click.pass_context
def system_status(ctx):
    """Show complete system status."""
    
    async def _status():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        
        # Database status
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        redis_manager = DatabaseFactory.create_redis_manager(db_config['database'])
        
        pg_healthy = await db_manager.health_check()
        redis_healthy = redis_manager.health_check()
        
        # Get statistics
        async with db_manager.session() as session:
            token_repo = RepositoryFactory.create_token_repository(session)
            graph_repo = RepositoryFactory.create_graph_repository(session)
            exp_repo = RepositoryFactory.create_experience_repository(session)
            
            total_tokens = await token_repo.count()
            total_connections = await graph_repo.count()
            total_events = await exp_repo.count()
        
        # System info
        cpu_percent = psutil.cpu_percent(interval=1)
        memory = psutil.virtual_memory()
        disk = psutil.disk_usage('/')
        
        # Build status panel
        console.print("\n[bold cyan]═══ NeuroGraph OS System Status ═══[/bold cyan]\n")
        
        # Services
        table = Table(title="Services", show_header=True)
        table.add_column("Service", style="cyan")
        table.add_column("Status", style="bold")
        table.add_column("Details", style="dim")
        
        pg_status = "[green]ONLINE[/green]" if pg_healthy else "[red]OFFLINE[/red]"
        redis_status = "[green]ONLINE[/green]" if redis_healthy else "[red]OFFLINE[/red]"
        
        table.add_row("PostgreSQL", pg_status, f"{db_config['database']['postgres']['host']}:{db_config['database']['postgres']['port']}")
        table.add_row("Redis", redis_status, f"{db_config['database']['redis']['host']}:{db_config['database']['redis']['port']}")
        
        console.print(table)
        console.print()
        
        # Data statistics
        stats_table = Table(title="Data Statistics", show_header=True)
        stats_table.add_column("Metric", style="cyan")
        stats_table.add_column("Count", justify="right", style="bold green")
        
        stats_table.add_row("Tokens", str(total_tokens))
        stats_table.add_row("Connections", str(total_connections))
        stats_table.add_row("Experience Events", str(total_events))
        
        if total_tokens > 0:
            avg_degree = (total_connections * 2) / total_tokens
            stats_table.add_row("Avg Degree", f"{avg_degree:.2f}")
        
        console.print(stats_table)
        console.print()
        
        # System resources
        resource_table = Table(title="System Resources", show_header=True)
        resource_table.add_column("Resource", style="cyan")
        resource_table.add_column("Usage", justify="right")
        resource_table.add_column("Details", style="dim")
        
        resource_table.add_row(
            "CPU",
            f"{cpu_percent}%",
            f"{psutil.cpu_count()} cores"
        )
        resource_table.add_row(
            "Memory",
            f"{memory.percent}%",
            f"{memory.used / (1024**3):.1f}GB / {memory.total / (1024**3):.1f}GB"
        )
        resource_table.add_row(
            "Disk",
            f"{disk.percent}%",
            f"{disk.used / (1024**3):.1f}GB / {disk.total / (1024**3):.1f}GB"
        )
        
        console.print(resource_table)
        console.print()
        
        # Overall status
        overall = "[green]HEALTHY[/green]" if (pg_healthy and redis_healthy) else "[red]UNHEALTHY[/red]"
        console.print(f"Overall Status: {overall}")
        console.print(f"[dim]Checked at: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}[/dim]\n")
        
        await DatabaseFactory.close_all()
    
    asyncio.run(_status())


@system_group.command(name='info')
@click.pass_context
def system_info(ctx):
    """Show system information."""
    
    console.print("\n[bold cyan]System Information[/bold cyan]\n")
    
    info_table = Table(show_header=False)
    info_table.add_column("Property", style="cyan")
    info_table.add_column("Value", style="white")
    
    info_table.add_row("Platform", platform.platform())
    info_table.add_row("Python", platform.python_version())
    info_table.add_row("Architecture", platform.machine())
    info_table.add_row("Processor", platform.processor() or "Unknown")
    info_table.add_row("Hostname", platform.node())
    
    console.print(info_table)
    console.print()


@system_group.command(name='health')
@click.option('--json', 'output_json', is_flag=True, help='Output as JSON')
@click.pass_context
def health_check(ctx, output_json):
    """Perform health check."""
    
    async def _health():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        redis_manager = DatabaseFactory.create_redis_manager(db_config['database'])
        
        pg_healthy = await db_manager.health_check()
        redis_healthy = redis_manager.health_check()
        
        health_status = {
            "healthy": pg_healthy and redis_healthy,
            "services": {
                "postgres": {"status": "up" if pg_healthy else "down"},
                "redis": {"status": "up" if redis_healthy else "down"}
            },
            "timestamp": datetime.now().isoformat()
        }
        
        if output_json:
            import json
            console.print(json.dumps(health_status, indent=2))
        else:
            if health_status["healthy"]:
                console.print("[green]✓ System is healthy[/green]")
            else:
                console.print("[red]✗ System is unhealthy[/red]")
                for service, status in health_status["services"].items():
                    if status["status"] == "down":
                        console.print(f"  [red]✗[/red] {service} is down")
        
        await DatabaseFactory.close_all()
        
        # Exit with error code if unhealthy
        if not health_status["healthy"]:
            exit(1)
    
    asyncio.run(_health())


@system_group.command(name='metrics')
@click.pass_context
def show_metrics(ctx):
    """Show system metrics."""
    
    async def _metrics():
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        redis_manager = DatabaseFactory.create_redis_manager(db_config['database'])
        
        console.print("\n[bold cyan]System Metrics[/bold cyan]\n")
        
        # Database metrics
        async with db_manager.session() as session:
            token_repo = RepositoryFactory.create_token_repository(session)
            graph_repo = RepositoryFactory.create_graph_repository(session)
            
            total_tokens = await token_repo.count()
            total_connections = await graph_repo.count()
            
            console.print(f"[cyan]Database:[/cyan]")
            console.print(f"  Tokens: {total_tokens:,}")
            console.print(f"  Connections: {total_connections:,}")
            
            if total_tokens > 0:
                ratio = total_connections / total_tokens
                console.print(f"  Connection ratio: {ratio:.2f}")
        
        # Redis metrics
        if redis_manager.health_check():
            client = redis_manager.get_client()
            info = client.info()
            
            console.print(f"\n[cyan]Redis:[/cyan]")
            console.print(f"  Keys: {client.dbsize():,}")
            console.print(f"  Memory: {info.get('used_memory_human', 'N/A')}")
            console.print(f"  Uptime: {info.get('uptime_in_seconds', 0) // 3600}h")
            console.print(f"  Commands/sec: {info.get('instantaneous_ops_per_sec', 0)}")
        
        # System metrics
        cpu = psutil.cpu_percent(interval=1)
        mem = psutil.virtual_memory()
        
        console.print(f"\n[cyan]System:[/cyan]")
        console.print(f"  CPU: {cpu}%")
        console.print(f"  Memory: {mem.percent}% ({mem.used / (1024**3):.1f}GB / {mem.total / (1024**3):.1f}GB)")
        console.print(f"  Load avg: {', '.join([f'{x:.2f}' for x in psutil.getloadavg()])}")
        
        console.print()
        
        await DatabaseFactory.close_all()
    
    asyncio.run(_metrics())


@system_group.command(name='logs')
@click.option('--lines', '-n', type=int, default=50, help='Number of lines to show')
@click.option('--follow', '-f', is_flag=True, help='Follow log output')
@click.pass_context
def show_logs(ctx, lines, follow):
    """Show system logs."""
    
    import subprocess
    
    log_file = "logs/neurograph.log"
    
    if follow:
        cmd = f"tail -f -n {lines} {log_file}"
    else:
        cmd = f"tail -n {lines} {log_file}"
    
    try:
        subprocess.run(cmd, shell=True)
    except KeyboardInterrupt:
        console.print("\n[yellow]Stopped following logs[/yellow]")
    except Exception as e:
        console.print(f"[red]Error reading logs: {e}[/red]")


@system_group.command(name='version')
@click.pass_context
def show_version(ctx):
    """Show version information."""
    
    console.print("\n[bold cyan]NeuroGraph OS[/bold cyan]")
    console.print("Version: [green]0.3.0[/green]")
    console.print("Architecture: Clean Architecture with Hexagonal Pattern")
    console.print("Status: Development\n")