"""
IPython widgets for real-time monitoring and interaction.

Provides live-updating widgets for metrics, logs, and graph exploration.
"""

from typing import Any, Dict, List, Optional
import time
import asyncio


class MetricsWidget:
    """
    Live-updating metrics display widget.

    Shows real-time metrics with auto-refresh.

    Example:
        from neurograph_jupyter.widgets import MetricsWidget

        widget = MetricsWidget()
        widget.subscribe("metrics")
        display(widget)
    """

    def __init__(self, connection_manager=None, refresh_interval: float = 1.0):
        """
        Initialize metrics widget.

        Args:
            connection_manager: ConnectionManager instance
            refresh_interval: Update interval in seconds
        """
        try:
            import ipywidgets as widgets
            from IPython.display import display, clear_output
        except ImportError:
            raise ImportError(
                "ipywidgets required for widgets. "
                "Install with: pip install ipywidgets"
            )

        self.connection_manager = connection_manager
        self.refresh_interval = refresh_interval
        self.metrics_data = []
        self.running = False

        # Create widgets
        self.output = widgets.Output()
        self.button = widgets.Button(
            description='Start',
            button_style='success',
            icon='play'
        )
        self.button.on_click(self._toggle)

        self.container = widgets.VBox([
            self.button,
            self.output
        ])

    def subscribe(self, channel: str):
        """Subscribe to a metrics channel."""
        if self.connection_manager:
            client_id = "jupyter_metrics_widget"
            if client_id not in self.connection_manager._connections:
                self.connection_manager.register_connection(client_id)
            self.connection_manager.subscribe(client_id, [channel])

    def _toggle(self, b):
        """Toggle start/stop."""
        if self.running:
            self.stop()
        else:
            self.start()

    def start(self):
        """Start live updates."""
        self.running = True
        self.button.description = 'Stop'
        self.button.button_style = 'danger'
        self.button.icon = 'stop'
        self._update_loop()

    def stop(self):
        """Stop live updates."""
        self.running = False
        self.button.description = 'Start'
        self.button.button_style = 'success'
        self.button.icon = 'play'

    def _update_loop(self):
        """Update loop for live display."""
        if not self.running:
            return

        with self.output:
            from IPython.display import clear_output
            clear_output(wait=True)

            # Display current metrics
            if self.metrics_data:
                latest = self.metrics_data[-1]
                print("📊 Live Metrics")
                print("=" * 50)
                for key, value in latest.items():
                    print(f"  {key}: {value}")
                print(f"\nUpdated: {time.strftime('%H:%M:%S')}")
                print(f"Total updates: {len(self.metrics_data)}")
            else:
                print("⏳ Waiting for metrics...")

        # Schedule next update
        if self.running:
            import threading
            threading.Timer(self.refresh_interval, self._update_loop).start()

    def add_metric(self, data: Dict[str, Any]):
        """
        Add metric data point.

        Args:
            data: Metric data dictionary
        """
        self.metrics_data.append({
            **data,
            'timestamp': time.time()
        })

        # Keep last 100 data points
        if len(self.metrics_data) > 100:
            self.metrics_data = self.metrics_data[-100:]

    def _ipython_display_(self):
        """Display widget in Jupyter."""
        from IPython.display import display
        display(self.container)


class GraphExplorerWidget:
    """
    Interactive graph exploration widget.

    Allows browsing nodes, filtering, and visualization.

    Example:
        from neurograph_jupyter.widgets import GraphExplorerWidget

        explorer = GraphExplorerWidget(neurograph_db)
        display(explorer)
    """

    def __init__(self, graph_ops):
        """
        Initialize graph explorer.

        Args:
            graph_ops: GraphOperations instance
        """
        try:
            import ipywidgets as widgets
        except ImportError:
            raise ImportError(
                "ipywidgets required. "
                "Install with: pip install ipywidgets"
            )

        self.graph_ops = graph_ops

        # Search controls
        self.search_input = widgets.Text(
            placeholder='Enter node type...',
            description='Type:',
            style={'description_width': 'initial'}
        )

        self.property_input = widgets.Text(
            placeholder='property=value',
            description='Filter:',
            style={'description_width': 'initial'}
        )

        self.search_button = widgets.Button(
            description='Search',
            button_style='primary',
            icon='search'
        )
        self.search_button.on_click(self._search)

        # Results display
        self.results_output = widgets.Output()

        # Layout
        controls = widgets.HBox([
            self.search_input,
            self.property_input,
            self.search_button
        ])

        self.container = widgets.VBox([
            widgets.HTML("<h3>🔍 Graph Explorer</h3>"),
            controls,
            self.results_output
        ])

    def _search(self, b):
        """Execute search."""
        with self.results_output:
            from IPython.display import clear_output
            clear_output(wait=True)

            # Build query
            node_type = self.search_input.value.strip()
            prop_filter = self.property_input.value.strip()

            if not node_type and not prop_filter:
                query = "find all nodes"
            elif node_type and not prop_filter:
                query = f"find all nodes where type='{node_type}'"
            elif not node_type and prop_filter:
                query = f"find all nodes where {prop_filter}"
            else:
                query = f"find all nodes where type='{node_type}' and {prop_filter}"

            try:
                print(f"🔍 Query: {query}\n")
                result = self.graph_ops.query(query)

                print(f"✅ Found {len(result.nodes)} nodes\n")

                # Display first 10
                for i, node in enumerate(result.nodes[:10]):
                    print(f"[{i+1}] ID: {node.id}")
                    print(f"    Type: {node.type}")
                    if node.properties:
                        print(f"    Properties: {node.properties}")
                    print()

                if len(result.nodes) > 10:
                    print(f"... and {len(result.nodes) - 10} more")

            except Exception as e:
                print(f"❌ Error: {e}")

    def _ipython_display_(self):
        """Display widget in Jupyter."""
        from IPython.display import display
        display(self.container)


class LogViewerWidget:
    """
    Live log viewer widget.

    Displays real-time log messages with filtering.

    Example:
        from neurograph_jupyter.widgets import LogViewerWidget

        logs = LogViewerWidget()
        logs.subscribe("logs")
        display(logs)
    """

    def __init__(self, connection_manager=None, max_logs: int = 100):
        """
        Initialize log viewer.

        Args:
            connection_manager: ConnectionManager instance
            max_logs: Maximum number of logs to keep
        """
        try:
            import ipywidgets as widgets
        except ImportError:
            raise ImportError(
                "ipywidgets required. "
                "Install with: pip install ipywidgets"
            )

        self.connection_manager = connection_manager
        self.max_logs = max_logs
        self.logs = []

        # Level filter
        self.level_filter = widgets.Dropdown(
            options=['ALL', 'INFO', 'WARNING', 'ERROR'],
            value='ALL',
            description='Level:',
        )
        self.level_filter.observe(self._refresh_display, names='value')

        # Clear button
        self.clear_button = widgets.Button(
            description='Clear',
            button_style='warning',
            icon='trash'
        )
        self.clear_button.on_click(self._clear)

        # Log display
        self.log_output = widgets.Output(
            layout=widgets.Layout(
                height='400px',
                overflow_y='auto',
                border='1px solid #ccc'
            )
        )

        # Layout
        controls = widgets.HBox([
            self.level_filter,
            self.clear_button
        ])

        self.container = widgets.VBox([
            widgets.HTML("<h3>📋 Log Viewer</h3>"),
            controls,
            self.log_output
        ])

    def subscribe(self, channel: str):
        """Subscribe to log channel."""
        if self.connection_manager:
            client_id = "jupyter_log_widget"
            if client_id not in self.connection_manager._connections:
                self.connection_manager.register_connection(client_id)
            self.connection_manager.subscribe(client_id, [channel])

    def add_log(self, level: str, message: str):
        """
        Add log entry.

        Args:
            level: Log level (INFO, WARNING, ERROR)
            message: Log message
        """
        self.logs.append({
            'level': level,
            'message': message,
            'timestamp': time.strftime('%H:%M:%S')
        })

        # Keep only max_logs
        if len(self.logs) > self.max_logs:
            self.logs = self.logs[-self.max_logs:]

        self._refresh_display()

    def _refresh_display(self, change=None):
        """Refresh log display."""
        with self.log_output:
            from IPython.display import clear_output
            clear_output(wait=True)

            level_filter = self.level_filter.value

            for log in self.logs:
                if level_filter != 'ALL' and log['level'] != level_filter:
                    continue

                # Color by level
                if log['level'] == 'ERROR':
                    icon = '🔴'
                elif log['level'] == 'WARNING':
                    icon = '🟡'
                else:
                    icon = '🟢'

                print(f"{icon} [{log['timestamp']}] {log['level']}: {log['message']}")

    def _clear(self, b):
        """Clear all logs."""
        self.logs = []
        self._refresh_display()

    def _ipython_display_(self):
        """Display widget in Jupyter."""
        from IPython.display import display
        display(self.container)
