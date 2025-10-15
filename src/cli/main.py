"""
NeuroGraph OS CLI - Command Line Interface
Main entry point for all CLI commands.
"""

import asyncio
import sys
from pathlib import Path

import click
from rich.console import Console
from rich.panel import Panel
from rich.text import Text

# Add project root to path
project_root = Path(__file__).parent.parent.parent
sys.path.insert(0, str(project_root))

from src.cli.commands import token, graph, database, system, config

console = Console()


def print_banner():
    """Print CLI banner."""
    banner = Text()
    banner.append("╔══════════════════════════════════════════════╗\n", style="cyan bold")
    banner.append("║        ", style="cyan bold")
    banner.append("NeuroGraph OS CLI", style="green bold")
    banner.append("                ║\n", style="cyan bold")
    banner.append("║        ", style="cyan bold")
    banner.append("v0.3.0", style="yellow")
    banner.append(" - Token-Based Computing        ║\n", style="cyan bold")
    banner.append("╚══════════════════════════════════════════════╝", style="cyan bold")
    
    console.print(banner)


@click.group()
@click.version_option(version="0.3.0", prog_name="NeuroGraph OS")
@click.option('--verbose', '-v', is_flag=True, help='Enable verbose output')
@click.option('--config', '-c', type=click.Path(exists=True), help='Path to config file')
@click.pass_context
def cli(ctx, verbose, config):
    """
    NeuroGraph OS - Token-based spatial computing system.
    
    Use 'neurograph COMMAND --help' for more information on a command.
    """
    # Store context
    ctx.ensure_object(dict)
    ctx.obj['VERBOSE'] = verbose
    ctx.obj['CONFIG_PATH'] = config
    
    if verbose:
        console.print("[dim]Verbose mode enabled[/dim]")


# Register command groups
cli.add_command(token.token_group)
cli.add_command(graph.graph_group)
cli.add_command(database.db_group)
cli.add_command(system.system_group)
cli.add_command(config.config_group)


@cli.command()
def info():
    """Show system information."""
    print_banner()
    console.print()
    
    info_text = """
[bold cyan]Architecture:[/bold cyan] Clean Architecture with Hexagonal Pattern
[bold cyan]Core Components:[/bold cyan]
  • Token System (64-byte binary format)
  • Spatial Grid (8-level coordinates)
  • Graph Engine (connections + CDNA validation)
  • Experience Stream (event collection)
  • DNA Guardian (configuration validation)

[bold cyan]Persistence:[/bold cyan]
  • PostgreSQL (tokens, graph, experience)
  • Redis (caching layer)

[bold cyan]Status:[/bold cyan] Development (v0.3)
    """
    
    console.print(Panel(info_text, title="System Information", border_style="cyan"))


@cli.command()
@click.pass_context
def quickstart(ctx):
    """Interactive quickstart guide."""
    console.print("\n[bold green]🚀 NeuroGraph OS Quickstart[/bold green]\n")
    
    steps = [
        ("1️⃣", "Start database", "docker-compose -f docker-compose.db.yml up -d"),
        ("2️⃣", "Run migrations", "alembic upgrade head"),
        ("3️⃣", "Check system status", "neurograph system status"),
        ("4️⃣", "Create a token", "neurograph token create --type test"),
        ("5️⃣", "View tokens", "neurograph token list"),
    ]
    
    for emoji, desc, cmd in steps:
        console.print(f"{emoji} [bold]{desc}[/bold]")
        console.print(f"   [dim]$ {cmd}[/dim]\n")
    
    console.print("[yellow]💡 Tip:[/yellow] Use --help with any command for more options\n")


def main():
    """Main CLI entry point."""
    try:
        cli(obj={})
    except KeyboardInterrupt:
        console.print("\n[yellow]⚠️  Interrupted by user[/yellow]")
        sys.exit(130)
    except Exception as e:
        console.print(f"\n[red]❌ Error: {e}[/red]")
        if '--verbose' in sys.argv or '-v' in sys.argv:
            import traceback
            console.print(traceback.format_exc())
        sys.exit(1)


if __name__ == '__main__':
    main()