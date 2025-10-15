"""
Configuration management CLI commands.
"""

import json
import yaml

import click
from rich.console import Console
from rich.syntax import Syntax
from rich.tree import Tree

from src.infrastructure.config import ConfigLoader

console = Console()


@click.group(name='config')
def config_group():
    """Configuration management."""
    pass


@config_group.command(name='show')
@click.argument('config_name', required=False)
@click.option('--format', '-f', type=click.Choice(['yaml', 'json']), default='yaml')
@click.pass_context
def show_config(ctx, config_name, format):
    """Show configuration file contents."""
    
    config_loader = ConfigLoader()
    
    if not config_name:
        # List available configs
        console.print("\n[bold cyan]Available Configurations:[/bold cyan]\n")
        configs = [
            ("infrastructure/database", "Database connection settings"),
            ("core/token_graph", "Graph configuration"),
            ("specs/dna_config", "DNA specifications"),
        ]
        
        for name, desc in configs:
            console.print(f"  [green]{name}[/green]")
            console.print(f"    [dim]{desc}[/dim]\n")
        
        console.print("[dim]Usage: neurograph config show <config_name>[/dim]")
        return
    
    try:
        config_data = config_loader.load(config_name)
        
        if format == 'json':
            output = json.dumps(config_data, indent=2)
            syntax = Syntax(output, "json", theme="monokai", line_numbers=True)
        else:
            output = yaml.dump(config_data, default_flow_style=False, sort_keys=False)
            syntax = Syntax(output, "yaml", theme="monokai", line_numbers=True)
        
        console.print(f"\n[bold cyan]Configuration: {config_name}[/bold cyan]\n")
        console.print(syntax)
    
    except FileNotFoundError:
        console.print(f"[red]Configuration not found: {config_name}[/red]")
    except Exception as e:
        console.print(f"[red]Error loading configuration: {e}[/red]")


@config_group.command(name='validate')
@click.argument('config_name')
@click.pass_context
def validate_config(ctx, config_name):
    """Validate configuration file."""
    
    config_loader = ConfigLoader()
    
    try:
        console.print(f"Validating: [cyan]{config_name}[/cyan]...")
        config_data = config_loader.load(config_name)
        
        # Basic validation
        if not config_data:
            console.print("[yellow]⚠[/yellow] Configuration is empty")
            return
        
        # Check for required keys based on config type
        if 'database' in config_name:
            required = ['database']
            for key in required:
                if key not in config_data:
                    console.print(f"[red]✗[/red] Missing required key: {key}")
                    return
        
        console.print("[green]✓[/green] Configuration is valid")
        console.print(f"[dim]Found {len(config_data)} top-level key(s)[/dim]")
    
    except yaml.YAMLError as e:
        console.print(f"[red]✗[/red] YAML syntax error: {e}")
    except FileNotFoundError:
        console.print(f"[red]✗[/red] Configuration not found: {config_name}")
    except Exception as e:
        console.print(f"[red]✗[/red] Validation error: {e}")


@config_group.command(name='tree')
@click.argument('config_name')
@click.pass_context
def show_config_tree(ctx, config_name):
    """Show configuration as a tree structure."""
    
    config_loader = ConfigLoader()
    
    try:
        config_data = config_loader.load(config_name)
        
        def build_tree(data, tree):
            if isinstance(data, dict):
                for key, value in data.items():
                    if isinstance(value, (dict, list)):
                        branch = tree.add(f"[cyan]{key}[/cyan]")
                        build_tree(value, branch)
                    else:
                        tree.add(f"[cyan]{key}[/cyan]: [yellow]{value}[/yellow]")
            elif isinstance(data, list):
                for i, item in enumerate(data):
                    if isinstance(item, (dict, list)):
                        branch = tree.add(f"[dim]\\[{i}][/dim]")
                        build_tree(item, branch)
                    else:
                        tree.add(f"[dim]\\[{i}][/dim]: [yellow]{item}[/yellow]")
        
        tree = Tree(f"[bold cyan]{config_name}[/bold cyan]")
        build_tree(config_data, tree)
        
        console.print("\n")
        console.print(tree)
        console.print()
    
    except FileNotFoundError:
        console.print(f"[red]Configuration not found: {config_name}[/red]")
    except Exception as e:
        console.print(f"[red]Error: {e}[/red]")


@config_group.command(name='get')
@click.argument('config_name')
@click.argument('key_path')
@click.pass_context
def get_config_value(ctx, config_name, key_path):
    """Get specific configuration value by key path."""
    
    config_loader = ConfigLoader()
    
    try:
        config_data = config