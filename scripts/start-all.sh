#!/bin/bash
# NeuroGraph Full Stack Startup Script
# Starts both backend and frontend in parallel

set -e

# Get project root (parent of scripts dir)
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

# Load project configuration
source "$SCRIPT_DIR/.config.sh"

echo "🚀 Starting NeuroGraph Full Stack..."
echo ""

# Make scripts executable
chmod +x "$PROJECT_ROOT/scripts/start-backend.sh"
chmod +x "$PROJECT_ROOT/scripts/start-frontend.sh"

# Check if tmux is available for better terminal management
if command -v tmux &> /dev/null; then
    echo "📺 Using tmux for session management"

    # Kill existing session if it exists
    tmux kill-session -t neurograph 2>/dev/null || true

    # Create new session
    tmux new-session -d -s neurograph -n backend
    tmux send-keys -t neurograph:backend "cd $PROJECT_ROOT && ./scripts/start-backend.sh" C-m

    # Create window for frontend
    tmux new-window -t neurograph -n frontend
    tmux send-keys -t neurograph:frontend "cd $PROJECT_ROOT && ./scripts/start-frontend.sh" C-m

    echo ""
    echo "✅ Services started in tmux session 'neurograph'"
    echo ""
    echo "📋 Available commands:"
    echo "   tmux attach -t neurograph        - Attach to session"
    echo "   tmux kill-session -t neurograph  - Stop all services"
    echo ""
    echo "🌐 Access points:"
    echo "   Frontend: http://localhost:$FRONTEND_PORT"
    echo "   Backend:  http://localhost:$BACKEND_PORT"
    echo "   API Docs: http://localhost:$BACKEND_PORT/docs"
    echo ""

    # Attach to the session
    tmux attach -t neurograph

else
    echo "⚠️  tmux not found. Starting services in background..."
    echo "💡 Install tmux for better session management: sudo pacman -S tmux"
    echo ""

    # Start backend in background
    echo "🔧 Starting backend..."
    "$PROJECT_ROOT/scripts/start-backend.sh" > "$PROJECT_ROOT/backend.log" 2>&1 &
    BACKEND_PID=$!
    echo "   Backend PID: $BACKEND_PID"

    # Wait a bit for backend to start
    sleep 3

    # Start frontend in background
    echo "🔧 Starting frontend..."
    "$PROJECT_ROOT/scripts/start-frontend.sh" > "$PROJECT_ROOT/frontend.log" 2>&1 &
    FRONTEND_PID=$!
    echo "   Frontend PID: $FRONTEND_PID"

    echo ""
    echo "✅ Services started in background"
    echo ""
    echo "📋 Process IDs:"
    echo "   Backend:  $BACKEND_PID"
    echo "   Frontend: $FRONTEND_PID"
    echo ""
    echo "📝 Logs:"
    echo "   Backend:  tail -f $PROJECT_ROOT/backend.log"
    echo "   Frontend: tail -f $PROJECT_ROOT/frontend.log"
    echo ""
    echo "🛑 To stop services:"
    echo "   ./stop-all.sh"
    echo ""
    echo "🌐 Access points:"
    echo "   Frontend: http://localhost:$FRONTEND_PORT"
    echo "   Backend:  http://localhost:$BACKEND_PORT"
    echo "   API Docs: http://localhost:$BACKEND_PORT/docs"
    echo ""

    # Save PIDs to file for easy cleanup
    echo "$BACKEND_PID" > "$PROJECT_ROOT/.backend.pid"
    echo "$FRONTEND_PID" > "$PROJECT_ROOT/.frontend.pid"

    # Wait for user interrupt
    echo "Press Ctrl+C to view logs (services will continue running)..."
    trap 'echo ""; echo "Services still running. To stop: ./stop-all.sh"; exit 0' INT

    # Show logs
    tail -f "$PROJECT_ROOT/backend.log" "$PROJECT_ROOT/frontend.log"
fi
