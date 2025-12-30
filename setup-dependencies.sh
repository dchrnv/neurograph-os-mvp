#!/bin/bash
# NeuroGraph Dependencies Setup Script
# Installs all required Python dependencies for the backend

set -e

PROJECT_ROOT="/home/chrnv/neurograph-os-mvp"
VENV_PATH="$PROJECT_ROOT/.venv"

echo "📦 Setting up NeuroGraph Backend Dependencies..."
echo "📁 Working directory: $PROJECT_ROOT"

# Check if running in Flatpak
if [ -f "/.flatpak-info" ] || command -v flatpak-spawn &> /dev/null; then
    echo "🐳 Flatpak environment detected"

    flatpak-spawn --host /bin/bash << 'EOF'
cd /home/chrnv/neurograph-os-mvp

# Create venv if it doesn't exist
if [ ! -d ".venv" ]; then
    echo "🔧 Creating virtual environment..."
    python3 -m venv .venv
fi

source .venv/bin/activate

echo "⬆️  Upgrading pip..."
pip install --upgrade pip

echo "📦 Installing core dependencies (pydantic with Python 3.13 support)..."
pip install --upgrade pydantic pydantic-settings pydantic-core

echo "📦 Installing FastAPI and web framework dependencies..."
pip install fastapi uvicorn[standard] python-multipart

echo "🔐 Installing security and authentication dependencies..."
pip install python-jose[cryptography] passlib[bcrypt] PyJWT email-validator

echo "📊 Installing monitoring and logging dependencies..."
pip install python-json-logger httpx prometheus-client psutil

echo "🧪 Installing testing dependencies..."
pip install pytest pytest-asyncio pytest-cov

echo "🔬 Installing scientific computing dependencies..."
pip install numpy scipy scikit-learn

echo "✅ All dependencies installed successfully!"
pip list | grep -E "fastapi|uvicorn|pydantic|numpy|prometheus"
EOF

else
    echo "💻 Standard environment detected"
    cd "$PROJECT_ROOT"

    # Create venv if it doesn't exist
    if [ ! -d "$VENV_PATH" ]; then
        echo "🔧 Creating virtual environment..."
        python3 -m venv .venv
    fi

    source .venv/bin/activate

    echo "⬆️  Upgrading pip..."
    pip install --upgrade pip

    echo "📦 Installing core dependencies (pydantic with Python 3.13 support)..."
    pip install --upgrade pydantic pydantic-settings pydantic-core

    echo "📦 Installing FastAPI and web framework dependencies..."
    pip install fastapi uvicorn[standard] python-multipart

    echo "🔐 Installing security and authentication dependencies..."
    pip install python-jose[cryptography] passlib[bcrypt] PyJWT email-validator

    echo "📊 Installing monitoring and logging dependencies..."
    pip install python-json-logger httpx prometheus-client psutil

    echo "🧪 Installing testing dependencies..."
    pip install pytest pytest-asyncio pytest-cov

    echo "🔬 Installing scientific computing dependencies..."
    pip install numpy scipy scikit-learn

    echo "✅ All dependencies installed successfully!"
    pip list | grep -E "fastapi|uvicorn|pydantic|numpy|prometheus"
fi

echo ""
echo "🎉 Setup complete! You can now run:"
echo "   ./start-backend.sh  - Start the backend API"
echo "   ./start-frontend.sh - Start the frontend UI"
