#!/bin/bash
# NeuroGraph Backend API Startup Script
# Automatically detects Flatpak environment and runs uvicorn server

set -e

# Get project root (parent of scripts dir)
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

# Load project configuration
source "$SCRIPT_DIR/.config.sh"

echo "🚀 Starting NeuroGraph Backend API..."
echo "📁 Working directory: $PROJECT_ROOT"

# Function to kill existing process on port
kill_port() {
    local port=$1
    echo "🔍 Checking for existing processes on port $port..."

    if [ -n "$EXEC_PREFIX" ]; then
        $EXEC_PREFIX pkill -f "uvicorn src.api.main" 2>/dev/null || true
    else
        lsof -ti:$port | xargs kill -9 2>/dev/null || true
    fi

    sleep 1
    echo "✅ Port $port cleared"
}

# Kill any existing backend process
kill_port $BACKEND_PORT

# Check if venv exists
if [ ! -d "$VENV_PATH" ]; then
    echo "⚠️  Virtual environment not found. Run ./setup-dependencies.sh first"
    exit 1
fi

if [ -n "$EXEC_PREFIX" ]; then
    echo "🐳 Flatpak environment detected"
    echo "🔧 Using flatpak-spawn --host"
fi

# Start backend using venv's python
echo "🚀 Starting uvicorn on port $BACKEND_PORT..."
run_cmd ".venv/bin/python -m uvicorn src.api.main:app --host 0.0.0.0 --port $BACKEND_PORT"
