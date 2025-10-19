#!/bin/bash

echo "═══════════════════════════════════════════════════════════"
echo "🚀 NeuroGraph OS MVP - Quick Start"
echo "═══════════════════════════════════════════════════════════"

# Check venv
if [ ! -d ".venv" ]; then
    echo "❌ Virtual environment not found!"
    echo "Creating .venv..."
    python3 -m venv .venv
    source .venv/bin/activate
    echo "Installing dependencies..."
    pip install -r requirements.txt
else
    source .venv/bin/activate
fi

echo ""
echo "Starting NeuroGraph OS API..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

python src/api_mvp/main.py

