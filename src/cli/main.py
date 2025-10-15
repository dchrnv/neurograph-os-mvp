"""
Основная точка входа для CLI NeuroGraph OS.
"""

import click

from src.cli.commands.config import config_group


@click.group()
@click.version_option(version="0.7.0", prog_name="NeuroGraph OS CLI")
def cli():
    """
    NeuroGraph OS Command Line Interface.
    """
    pass


# Регистрация групп команд
cli.add_command(config_group)
# Здесь будут добавляться другие группы команд (token, graph, system и т.д.)