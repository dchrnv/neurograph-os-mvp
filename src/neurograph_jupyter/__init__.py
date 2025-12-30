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
from .helpers import install_result_extensions


def load_ipython_extension(ipython):
    """
    Load the IPython extension.

    This is called when `%load_ext neurograph_jupyter` is executed.
    """
    # Register magic commands
    magics = NeuroGraphMagics(ipython)
    ipython.register_magics(magics)

    # Register auto-completion
    try:
        ipython.set_hook('complete_command', magics._neurograph_completions, str_key='%neurograph')
    except:
        pass  # Graceful degradation if completion not supported

    # Install rich display formatters
    install_display_formatters(ipython)

    # Install QueryResult extension methods
    if install_result_extensions():
        print("✅ NeuroGraph Jupyter extension loaded")
        print("💡 Auto-completion enabled (press TAB after %neurograph)")
        print("💡 DataFrame helpers: result.to_dataframe(), result.export_csv(), result.plot_distribution()")
    else:
        print("✅ NeuroGraph Jupyter extension loaded")
        print("💡 Auto-completion enabled (press TAB after %neurograph)")

    print("Try: %neurograph init --path ./my_graph.db")


def unload_ipython_extension(ipython):
    """
    Unload the IPython extension.

    This is called when `%unload_ext neurograph_jupyter` is executed.
    """
    print("NeuroGraph Jupyter extension unloaded")


__version__ = "0.61.1"
__all__ = ["NeuroGraphMagics", "load_ipython_extension", "unload_ipython_extension"]
