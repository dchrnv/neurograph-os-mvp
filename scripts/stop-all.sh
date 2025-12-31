#!/bin/bash
# NeuroGraph Stop Script
# Stops all running NeuroGraph services

set -e

PROJECT_ROOT="/home/chrnv/neurograph-os-mvp"

echo "🛑 Stopping NeuroGraph services..."

# Function to kill process
kill_process() {
    local pid=$1
    if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
        echo "   Killing process $pid..."
        kill "$pid" 2>/dev/null || kill -9 "$pid" 2>/dev/null || true
    fi
}

# Kill tmux session if it exists
if command -v tmux &> /dev/null; then
    if tmux has-session -t neurograph 2>/dev/null; then
        echo "🔧 Stopping tmux session 'neurograph'..."
        tmux kill-session -t neurograph
        echo "   ✅ Tmux session stopped"
    fi
fi

# Kill processes from PID files
if [ -f "$PROJECT_ROOT/.backend.pid" ]; then
    BACKEND_PID=$(cat "$PROJECT_ROOT/.backend.pid")
    echo "🔧 Stopping backend (PID: $BACKEND_PID)..."
    kill_process "$BACKEND_PID"
    rm "$PROJECT_ROOT/.backend.pid"
fi

if [ -f "$PROJECT_ROOT/.frontend.pid" ]; then
    FRONTEND_PID=$(cat "$PROJECT_ROOT/.frontend.pid")
    echo "🔧 Stopping frontend (PID: $FRONTEND_PID)..."
    kill_process "$FRONTEND_PID"
    rm "$PROJECT_ROOT/.frontend.pid"
fi

# Kill any process on port 8000 (backend)
echo "🔧 Clearing port 8000..."
if command -v flatpak-spawn &> /dev/null; then
    flatpak-spawn --host pkill -f "uvicorn src.api.main" 2>/dev/null || true
else
    lsof -ti:8000 | xargs kill -9 2>/dev/null || true
fi

# Kill any vite process (frontend)
echo "🔧 Stopping Vite processes..."
if command -v flatpak-spawn &> /dev/null; then
    flatpak-spawn --host pkill -f "vite" 2>/dev/null || true
else
    pkill -f "vite" 2>/dev/null || true
fi

# Clean up log files
if [ -f "$PROJECT_ROOT/backend.log" ]; then
    rm "$PROJECT_ROOT/backend.log"
fi

if [ -f "$PROJECT_ROOT/frontend.log" ]; then
    rm "$PROJECT_ROOT/frontend.log"
fi

echo ""
echo "✅ All NeuroGraph services stopped"
