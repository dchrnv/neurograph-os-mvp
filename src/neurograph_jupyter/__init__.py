"""
NeuroGraph Jupyter Integration

IPython extension for interactive NeuroGraph operations in Jupyter notebooks.

Usage:
    %load_ext neurograph_jupyter
    %neurograph init --path ./my_graph.db
    %neurograph status
    %neurograph query "find all nodes"

Cell magic for signal definitions:
    %%signal process_data
    def handler(data):
        return data * 2
"""

from .magic import NeuroGraphMagics
from .display import install_display_formatters


def load_ipython_extension(ipython):
    """
    Load the IPython extension.

    This is called when `%load_ext neurograph_jupyter` is executed.
    """
    # Register magic commands
    ipython.register_magics(NeuroGraphMagics)

    # Install rich display formatters
    install_display_formatters(ipython)

    print("✅ NeuroGraph Jupyter extension loaded")
    print("Try: %neurograph init --path ./my_graph.db")


def unload_ipython_extension(ipython):
    """
    Unload the IPython extension.

    This is called when `%unload_ext neurograph_jupyter` is executed.
    """
    print("NeuroGraph Jupyter extension unloaded")


__version__ = "0.61.0"
__all__ = ["NeuroGraphMagics", "load_ipython_extension", "unload_ipython_extension"]
