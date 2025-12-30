#!/bin/bash
# NeuroGraph Frontend Startup Script
# Automatically detects Flatpak environment and runs Vite dev server

set -e

PROJECT_ROOT="/home/chrnv/neurograph-os-mvp"
WEB_DIR="$PROJECT_ROOT/src/web"

echo "🚀 Starting NeuroGraph Frontend..."
echo "📁 Working directory: $WEB_DIR"

# Check if running in Flatpak
if [ -f "/.flatpak-info" ] || command -v flatpak-spawn &> /dev/null; then
    echo "🐳 Flatpak environment detected"
    echo "🔧 Using flatpak-spawn --host"
    flatpak-spawn --host /bin/bash -c "cd $WEB_DIR && ./node_modules/.bin/vite"
else
    echo "💻 Standard environment detected"
    cd "$WEB_DIR"

    # Check if node_modules exists
    if [ ! -d "node_modules" ]; then
        echo "⚠️  node_modules not found. Installing dependencies..."
        npm install
    fi

    npm run dev
fi
