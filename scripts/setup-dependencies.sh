#!/bin/bash
# NeuroGraph Dependencies Setup Script
# Installs all required Python dependencies for the backend

set -e

# Get project root (parent of scripts dir)
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

# Load project configuration
source "$SCRIPT_DIR/.config.sh"

echo "📦 Setting up NeuroGraph Backend Dependencies..."
echo "📁 Working directory: $PROJECT_ROOT"
echo "🐍 Python version: $(get_python_version)"

# Create venv if it doesn't exist
if [ ! -d "$VENV_PATH" ]; then
    echo "🔧 Creating virtual environment..."
    run_cmd "$PYTHON_CMD -m venv .venv"
fi

echo "⬆️  Upgrading pip and installing maturin..."
run_cmd ".venv/bin/pip install --upgrade pip maturin"

echo "📦 Installing core dependencies..."
run_cmd ".venv/bin/pip install --upgrade pydantic pydantic-settings pydantic-core"

echo "📦 Installing FastAPI and web framework dependencies..."
run_cmd ".venv/bin/pip install fastapi 'uvicorn[standard]' python-multipart"

echo "🔐 Installing security and authentication dependencies..."
run_cmd ".venv/bin/pip install 'python-jose[cryptography]' 'passlib[bcrypt]' PyJWT email-validator"

echo "📊 Installing monitoring and logging dependencies..."
run_cmd ".venv/bin/pip install python-json-logger httpx prometheus-client psutil"

echo "🧪 Installing testing dependencies..."
run_cmd ".venv/bin/pip install pytest pytest-asyncio pytest-cov"

echo "🔬 Installing scientific computing dependencies..."
run_cmd ".venv/bin/pip install numpy scipy scikit-learn"

if [ "$RUST_CORE_ENABLED" = true ]; then
    echo "🦀 Building Rust Core with Python bindings..."
    run_cmd "cd src/core_rust && ../../.venv/bin/maturin develop --features python-bindings --release"

    echo ""
    echo "✅ Verifying Rust Core..."
    run_cmd ".venv/bin/python -c 'import _core; print(\"✅ Rust Core (_core) loaded successfully\")'"
fi

echo ""
echo "✅ All dependencies installed successfully!"
echo ""
echo "📋 Installed packages:"
run_cmd ".venv/bin/pip list | grep -E 'fastapi|uvicorn|pydantic|numpy|prometheus|maturin'"

echo ""
echo "🎉 Setup complete! You can now run:"
echo "   ./start-all.sh      - Start both frontend and backend"
echo "   ./start-backend.sh  - Start the backend API only"
echo "   ./start-frontend.sh - Start the frontend UI only"
