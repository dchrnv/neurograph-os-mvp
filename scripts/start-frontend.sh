#!/bin/bash
# NeuroGraph Frontend Startup Script
# Automatically detects Flatpak environment and runs Vite dev server

set -e

# Get project root (parent of scripts dir)
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

# Load project configuration
source "$SCRIPT_DIR/.config.sh"

WEB_DIR="$PROJECT_ROOT/src/web"

echo "🚀 Starting NeuroGraph Frontend..."
echo "📁 Working directory: $WEB_DIR"

# Check if node_modules exists
if [ ! -d "$WEB_DIR/node_modules" ]; then
    echo "⚠️  node_modules not found. Installing dependencies..."
    if [ -n "$EXEC_PREFIX" ]; then
        $EXEC_PREFIX /bin/bash -c "cd $WEB_DIR && npm install"
    else
        cd "$WEB_DIR"
        npm install
    fi
fi

# Start Vite dev server
if [ -n "$EXEC_PREFIX" ]; then
    echo "🐳 Flatpak environment detected"
    echo "🔧 Using flatpak-spawn --host"
    $EXEC_PREFIX /bin/bash -c "cd $WEB_DIR && ./node_modules/.bin/vite --port $FRONTEND_PORT"
else
    echo "💻 Standard environment detected"
    cd "$WEB_DIR"
    npm run dev
fi
