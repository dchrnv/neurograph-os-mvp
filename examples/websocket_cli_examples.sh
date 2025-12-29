#!/bin/bash

# NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
# Copyright (C) 2024-2025 Chernov Denys

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.

# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

# WebSocket CLI Tool - Usage Examples
#
# Примеры использования инструмента командной строки для тестирования WebSocket

echo "=================================================="
echo "NeuroGraph WebSocket CLI Tool - Examples"
echo "=================================================="
echo ""

# Example 1: Basic connection
echo "Example 1: Basic connection"
echo "----------------------------"
echo "Command:"
echo "  python -m src.api.websocket.cli --url ws://localhost:8000/ws"
echo ""
echo "Description:"
echo "  Connect to WebSocket and listen for all events"
echo "  Output will be in pretty format with colors"
echo ""
echo "=================================================="
echo ""

# Example 2: Subscribe to specific channels
echo "Example 2: Subscribe to specific channels"
echo "-------------------------------------------"
echo "Command:"
echo "  python -m src.api.websocket.cli --url ws://localhost:8000/ws --subscribe metrics,signals"
echo ""
echo "Description:"
echo "  Connect and automatically subscribe to 'metrics' and 'signals' channels"
echo "  You will only receive events from these channels"
echo ""
echo "=================================================="
echo ""

# Example 3: JSON output
echo "Example 3: JSON output"
echo "----------------------"
echo "Command:"
echo "  python -m src.api.websocket.cli --url ws://localhost:8000/ws --format json"
echo ""
echo "Description:"
echo "  Output events in JSON format (useful for parsing)"
echo "  Each event is a complete JSON object"
echo ""
echo "=================================================="
echo ""

# Example 4: Compact output
echo "Example 4: Compact output"
echo "-------------------------"
echo "Command:"
echo "  python -m src.api.websocket.cli --url ws://localhost:8000/ws --format compact"
echo ""
echo "Description:"
echo "  Output events in compact JSON (one line per event)"
echo "  Useful for logging or piping to other tools"
echo ""
echo "=================================================="
echo ""

# Example 5: With authentication
echo "Example 5: With JWT authentication"
echo "-----------------------------------"
echo "Command:"
echo "  python -m src.api.websocket.cli --url ws://localhost:8000/ws --token YOUR_JWT_TOKEN"
echo ""
echo "Description:"
echo "  Connect with JWT authentication"
echo "  Allows access to protected channels based on user role"
echo ""
echo "=================================================="
echo ""

# Example 6: Subscribe to all channels (admin)
echo "Example 6: Subscribe to all channels (requires admin)"
echo "------------------------------------------------------"
echo "Command:"
echo "  python -m src.api.websocket.cli \\"
echo "    --url ws://localhost:8000/ws \\"
echo "    --token YOUR_ADMIN_TOKEN \\"
echo "    --subscribe metrics,signals,actions,logs,status,connections"
echo ""
echo "Description:"
echo "  Connect as admin and subscribe to all 6 channels"
echo "  Only admin role has access to 'connections' channel"
echo ""
echo "=================================================="
echo ""

# Example 7: Verbose mode
echo "Example 7: Verbose logging"
echo "--------------------------"
echo "Command:"
echo "  python -m src.api.websocket.cli --url ws://localhost:8000/ws --verbose"
echo ""
echo "Description:"
echo "  Enable verbose logging to see connection details and errors"
echo "  Useful for debugging connection issues"
echo ""
echo "=================================================="
echo ""

# Example 8: Monitor metrics only
echo "Example 8: Monitor metrics only"
echo "-------------------------------"
echo "Command:"
echo "  python -m src.api.websocket.cli \\"
echo "    --url ws://localhost:8000/ws \\"
echo "    --subscribe metrics \\"
echo "    --format compact | grep -E '\"channel\":\"metrics\"'"
echo ""
echo "Description:"
echo "  Subscribe only to metrics and filter output"
echo "  Useful for monitoring system performance"
echo ""
echo "=================================================="
echo ""

# Example 9: Save to log file
echo "Example 9: Save events to log file"
echo "-----------------------------------"
echo "Command:"
echo "  python -m src.api.websocket.cli \\"
echo "    --url ws://localhost:8000/ws \\"
echo "    --format json > websocket_events.log"
echo ""
echo "Description:"
echo "  Capture all events to a JSON log file"
echo "  Each line is a complete JSON event"
echo ""
echo "=================================================="
echo ""

# Example 10: Real-time monitoring
echo "Example 10: Real-time monitoring with timestamps"
echo "------------------------------------------------"
echo "Command:"
echo "  python -m src.api.websocket.cli \\"
echo "    --url ws://localhost:8000/ws \\"
echo "    --subscribe metrics,status \\"
echo "    --format pretty"
echo ""
echo "Description:"
echo "  Monitor system metrics and status in real-time"
echo "  Pretty format shows event structure clearly"
echo ""
echo "=================================================="
echo ""

echo "To run any example, copy the command and execute it in your terminal."
echo ""
echo "Note: Make sure the API server is running:"
echo "  python -m src.api.main"
echo ""
echo "For more help:"
echo "  python -m src.api.websocket.cli --help"
echo ""
