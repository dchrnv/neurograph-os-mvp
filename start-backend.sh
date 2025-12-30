#!/bin/bash
# NeuroGraph Backend API Startup Script
# Automatically detects Flatpak environment and runs uvicorn server

set -e

PROJECT_ROOT="/home/chrnv/neurograph-os-mvp"
VENV_PATH="$PROJECT_ROOT/.venv"

echo "🚀 Starting NeuroGraph Backend API..."
echo "📁 Working directory: $PROJECT_ROOT"

# Function to kill existing process on port 8000
kill_port_8000() {
    echo "🔍 Checking for existing processes on port 8000..."

    if command -v flatpak-spawn &> /dev/null; then
        # Flatpak environment
        flatpak-spawn --host pkill -f "uvicorn src.api.main" 2>/dev/null || true
    else
        # Standard environment
        lsof -ti:8000 | xargs kill -9 2>/dev/null || true
    fi

    sleep 1
    echo "✅ Port 8000 cleared"
}

# Kill any existing backend process
kill_port_8000

# Check if running in Flatpak
if [ -f "/.flatpak-info" ] || command -v flatpak-spawn &> /dev/null; then
    echo "🐳 Flatpak environment detected"
    echo "🔧 Using flatpak-spawn --host"

    # Check if venv exists
    if [ ! -d "$VENV_PATH" ]; then
        echo "⚠️  Virtual environment not found. Creating..."
        flatpak-spawn --host /bin/bash -c "cd $PROJECT_ROOT && python3 -m venv .venv"
    fi

    flatpak-spawn --host /bin/bash -c "cd $PROJECT_ROOT && source .venv/bin/activate && uvicorn src.api.main:app --host 0.0.0.0 --port 8000"
else
    echo "💻 Standard environment detected"
    cd "$PROJECT_ROOT"

    # Check if venv exists
    if [ ! -d "$VENV_PATH" ]; then
        echo "⚠️  Virtual environment not found. Creating..."
        python3 -m venv .venv
    fi

    source .venv/bin/activate
    uvicorn src.api.main:app --host 0.0.0.0 --port 8000
fi
