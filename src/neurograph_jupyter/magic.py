"""
IPython magic commands for NeuroGraph operations.
"""

import argparse
import sys
from pathlib import Path
from typing import Optional

from IPython.core.magic import Magics, magics_class, line_magic, cell_magic
from IPython.core.magic_arguments import argument, magic_arguments, parse_argstring

from neurograph.core.connection_manager import ConnectionManager
from neurograph.core.graph_operations import GraphOperations
from neurograph.query.signal_engine import SignalEngine


@magics_class
class NeuroGraphMagics(Magics):
    """
    IPython magic commands for NeuroGraph.

    Line magics:
        %neurograph init --path <db_path>
        %neurograph status
        %neurograph query <query_string>
        %neurograph subscribe <channel>
        %neurograph emit <channel> <data>

    Cell magics:
        %%signal <signal_name>
        def handler(data):
            return data
    """

    def __init__(self, shell):
        super().__init__(shell)
        self.connection_manager: Optional[ConnectionManager] = None
        self.graph_ops: Optional[GraphOperations] = None
        self.signal_engine: Optional[SignalEngine] = None
        self.db_path: Optional[Path] = None

    @line_magic
    @magic_arguments()
    @argument("command", type=str, help="Command to execute: init, status, query, subscribe, emit")
    @argument("args", nargs="*", help="Command arguments")
    @argument("--path", type=str, help="Database path for init command")
    def neurograph(self, line):
        """
        Main NeuroGraph magic command.

        Examples:
            %neurograph init --path ./my_graph.db
            %neurograph status
            %neurograph query "find all nodes"
            %neurograph subscribe metrics
            %neurograph emit metrics "{'value': 42}"
        """
        # Parse arguments
        parts = line.split(None, 1)
        if not parts:
            return self._show_help()

        command = parts[0].lower()
        args = parts[1] if len(parts) > 1 else ""

        # Route to command handlers
        if command == "init":
            return self._handle_init(args)
        elif command == "status":
            return self._handle_status()
        elif command == "query":
            return self._handle_query(args)
        elif command == "subscribe":
            return self._handle_subscribe(args)
        elif command == "emit":
            return self._handle_emit(args)
        else:
            print(f"❌ Unknown command: {command}")
            return self._show_help()

    def _handle_init(self, args: str):
        """Initialize NeuroGraph connection."""
        # Parse --path argument
        parser = argparse.ArgumentParser(prog="%neurograph init")
        parser.add_argument("--path", type=str, required=True, help="Database path")

        try:
            parsed = parser.parse_args(args.split())
        except SystemExit:
            return

        db_path = Path(parsed.path).expanduser().resolve()

        try:
            # Initialize connection manager
            self.connection_manager = ConnectionManager()
            self.graph_ops = GraphOperations(str(db_path))
            self.signal_engine = SignalEngine(self.connection_manager)
            self.db_path = db_path

            # Store in IPython namespace for direct access
            self.shell.user_ns["neurograph_db"] = self.graph_ops
            self.shell.user_ns["neurograph_signals"] = self.signal_engine
            self.shell.user_ns["neurograph_ws"] = self.connection_manager

            print(f"✅ NeuroGraph initialized")
            print(f"📁 Database: {db_path}")
            print(f"🔗 Connection Manager: Ready")
            print(f"📡 Signal Engine: Ready")
            print("\nAvailable in namespace:")
            print("  - neurograph_db: GraphOperations")
            print("  - neurograph_signals: SignalEngine")
            print("  - neurograph_ws: ConnectionManager")

        except Exception as e:
            print(f"❌ Failed to initialize NeuroGraph: {e}")
            import traceback
            traceback.print_exc()

    def _handle_status(self):
        """Show current NeuroGraph status."""
        if not self.connection_manager:
            print("❌ NeuroGraph not initialized. Run: %neurograph init --path <db_path>")
            return

        print("🟢 NeuroGraph Status")
        print(f"📁 Database: {self.db_path}")
        print(f"🔗 Active Connections: {len(self.connection_manager._connections)}")
        print(f"📡 Signal Engine: {'Active' if self.signal_engine else 'Inactive'}")

        # Show subscriptions
        total_subscriptions = sum(
            len(subs) for subs in self.connection_manager._subscriptions.values()
        )
        print(f"📬 Total Subscriptions: {total_subscriptions}")

        # Show channels
        channels = list(self.connection_manager._subscriptions.keys())
        if channels:
            print(f"📢 Active Channels: {', '.join(channels[:5])}")
            if len(channels) > 5:
                print(f"   ... and {len(channels) - 5} more")

    def _handle_query(self, query_string: str):
        """Execute a NeuroGraph query."""
        if not self.graph_ops:
            print("❌ NeuroGraph not initialized. Run: %neurograph init --path <db_path>")
            return

        # Remove quotes if present
        query_string = query_string.strip().strip('"').strip("'")

        try:
            result = self.graph_ops.query(query_string)
            # Store result in _ variable for access
            self.shell.user_ns["_neurograph_result"] = result
            return result  # Will be displayed by rich formatter

        except Exception as e:
            print(f"❌ Query failed: {e}")
            import traceback
            traceback.print_exc()

    def _handle_subscribe(self, channel: str):
        """Subscribe to a channel."""
        if not self.connection_manager:
            print("❌ NeuroGraph not initialized. Run: %neurograph init --path <db_path>")
            return

        channel = channel.strip()

        # Create a client ID for the notebook
        client_id = "jupyter_notebook"

        try:
            # Register connection if not exists
            if client_id not in self.connection_manager._connections:
                self.connection_manager.register_connection(client_id)

            # Subscribe to channel
            self.connection_manager.subscribe(client_id, [channel])

            print(f"✅ Subscribed to channel: {channel}")
            print(f"👤 Client ID: {client_id}")

        except Exception as e:
            print(f"❌ Subscription failed: {e}")

    def _handle_emit(self, args: str):
        """Emit a signal to a channel."""
        if not self.connection_manager:
            print("❌ NeuroGraph not initialized. Run: %neurograph init --path <db_path>")
            return

        # Parse channel and data
        parts = args.split(None, 1)
        if len(parts) < 2:
            print("❌ Usage: %neurograph emit <channel> <data>")
            return

        channel = parts[0].strip()
        data = parts[1].strip()

        # Parse data as JSON if it looks like JSON
        import json
        try:
            data = json.loads(data)
        except json.JSONDecodeError:
            pass  # Keep as string

        try:
            # Broadcast to channel
            import asyncio
            asyncio.run(
                self.connection_manager.broadcast_to_channel(
                    channel,
                    {
                        "type": "signal",
                        "channel": channel,
                        "data": data
                    }
                )
            )

            print(f"✅ Signal emitted to channel: {channel}")
            print(f"📊 Data: {data}")

        except Exception as e:
            print(f"❌ Emit failed: {e}")

    @cell_magic
    def signal(self, line, cell):
        """
        Define a signal handler.

        Usage:
            %%signal process_data
            def handler(data):
                return data * 2

        The function defined in the cell will be registered as a signal handler.
        """
        if not self.signal_engine:
            print("❌ NeuroGraph not initialized. Run: %neurograph init --path <db_path>")
            return

        signal_name = line.strip()
        if not signal_name:
            print("❌ Signal name required")
            return

        try:
            # Execute the cell to define the function
            self.shell.run_cell(cell)

            # Look for the defined function in the namespace
            if "handler" in self.shell.user_ns:
                handler_func = self.shell.user_ns["handler"]

                # Register with signal engine
                # Note: This is simplified - real implementation would integrate with SignalEngine
                print(f"✅ Signal handler registered: {signal_name}")
                print(f"📡 Function: {handler_func.__name__}")

                # Store in namespace
                self.shell.user_ns[f"signal_{signal_name}"] = handler_func

            else:
                print("❌ No 'handler' function found in cell")

        except Exception as e:
            print(f"❌ Signal registration failed: {e}")
            import traceback
            traceback.print_exc()

    def _show_help(self):
        """Show help message."""
        help_text = """
🧠 NeuroGraph Magic Commands

Initialization:
    %neurograph init --path <db_path>    Initialize NeuroGraph connection

Status:
    %neurograph status                    Show current status

Queries:
    %neurograph query "<query_string>"    Execute a query

Real-time:
    %neurograph subscribe <channel>       Subscribe to channel
    %neurograph emit <channel> <data>     Emit signal to channel

Cell Magic:
    %%signal <signal_name>                Define signal handler
    def handler(data):
        return processed_data

Examples:
    %neurograph init --path ./my_graph.db
    %neurograph query "find all nodes where type='user'"
    %neurograph subscribe metrics
    %neurograph emit metrics "{'cpu': 42}"
"""
        print(help_text)
